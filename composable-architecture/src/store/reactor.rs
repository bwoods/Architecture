use crate::Reducer;
use crate::effects::Inner;
use crate::store::wait::{Queue, Wait};
use futures::task::Poll;
use futures::{Stream, pin_mut};
use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, VecDeque};
use std::marker::PhantomData;
use std::ops::ControlFlow;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use std::task::Context;
use std::thread::{Builder, JoinHandle};
use std::time::Instant;

pub(crate) struct Reactor<State, T, R> {
    marker: PhantomData<State>,
    pub handle: JoinHandle<R>,

    /// external events…
    pub events: Arc<Queue<T>>,
    /// other (internal) reasons for the Reactor to…
    pub other: Arc<Queue<Reason>>,
}

#[derive(Clone)]
pub(crate) enum Reason {
    Wake(u64),

    ShrinkToFit,
    Shutdown,
    /// When testing, the test controls the passage of time, not `Instant::now()`.
    Sync(Instant, Arc<std::sync::Barrier>),
}

impl<State: Reducer, T, R> Reactor<State, T, R> {
    pub fn new<F>(with: F) -> Self
    where
        <State as Reducer>::Action: 'static,
        F: Send + (FnOnce() -> State) + 'static,
        T: Send + Into<<State as Reducer>::Action> + 'static,
        R: Send + From<State> + 'static,
    {
        let wait = Wait::new();
        let events = Queue::<T>::new(wait.clone());
        let other = Queue::new(wait.clone());

        let handle = Self::start(with, events.clone(), other.clone(), wait);

        Self {
            marker: PhantomData,
            handle,
            other,
            events,
        }
    }

    /// Each [`Reactor`] runs in a dedicated thread with a specified stack size.
    /// Thread stack sizes vary between OSes and toolchains, and are often much
    /// smaller than this.
    pub const STACK_SIZE: usize = 8 * 1024 * 1024;

    fn start<F>(
        initial: F,
        events: Arc<Queue<T>>,
        other: Arc<Queue<Reason>>,
        wait: Arc<Wait>,
    ) -> JoinHandle<R>
    where
        <State as Reducer>::Action: 'static,
        F: Send + (FnOnce() -> State) + 'static,
        T: Send + Into<<State as Reducer>::Action> + 'static,
        R: Send + From<State> + 'static,
    {
        Builder::new()
            .stack_size(Self::STACK_SIZE)
            .name(std::any::type_name::<Self>().into())
            .spawn(move || {
                let mut state = Box::new(initial());

                let now = Cell::new(Instant::now());
                let effects = Inner::new(now.get());

                let mut tasks = BTreeMap::<
                    u64, // tasks ids are moved between collections, rather than the tasks themselves
                    Pin<Box<dyn Stream<Item = ControlFlow<Instant, <State as Reducer>::Action>>>>,
                >::new();
                let mut scheduler = VecDeque::<(Instant, u64)>::new();
                let mut unpolled = VecDeque::<u64>::new();

                let mut shrink_to_fit = false;
                let mut shutdown = false;

                loop {
                    for event in events.take() {
                        Self::reduce(&mut state, event.into(), &effects);
                    }

                    // during testing `now` is explicitly controlled (see Reason::Sync)
                    // otherwise we update it here and now
                    if cfg!(not(any(test, feature = "testing"))) {
                        let actual = Instant::now();
                        now.set(actual);
                        effects.borrow().sync(now.get());
                    }

                    let current = now.get();
                    let index = scheduler.partition_point(move |&(when, _)| when <= current);
                    let due = scheduler.drain(..index).map(|(_, id)| id);

                    #[rustfmt::skip]
                    let ready = other
                        .take()
                        .into_iter()
                        .filter_map(|reason| match reason {
                            Reason::Wake(id) => Some(id),
                            Reason::Shutdown => { shutdown = true; None }
                            Reason::ShrinkToFit => { shrink_to_fit = true; None }
                            Reason::Sync(instant, barrier) => {
                                now.set(instant);
                                effects.borrow().sync(instant);
                                barrier.wait();
                                None
                            }
                        });

                    let ids = due // due timers, then
                        .chain(ready) // ready tasks, then
                        .chain(unpolled.drain(..)); // new (unpolled) tasks

                    for id in ids {
                        let stream = tasks.get_mut(&id).expect("task id");
                        let waker = other.waker(Reason::Wake(id)); // mimalloc is faster than a BtreeMap lookup…
                        let mut context = Context::from_waker(&waker);
                        pin_mut!(stream);

                        let completed = loop {
                            match stream.as_mut().poll_next(&mut context) {
                                Poll::Ready(Some(ControlFlow::Continue(action))) => {
                                    Self::reduce(&mut state, action, &effects);
                                }
                                Poll::Ready(Some(ControlFlow::Break(until))) => {
                                    effects.borrow_mut().timers.push_back((until, id));
                                    break false;
                                }
                                Poll::Ready(None) => break true,
                                Poll::Pending => break false,
                            }
                        };

                        if completed {
                            tasks.remove(&id);
                        }
                    }

                    let mut effects = effects.borrow_mut();
                    let mut is_empty = effects.tasks.is_empty() && effects.timers.is_empty();
                    debug_assert!(effects.actions.is_empty());

                    if cfg!(any(test, feature = "testing")) {
                        // Wait until ALL processing is done when unit testing or benchmarking
                        is_empty &= tasks.is_empty();
                        is_empty &= events.is_empty();
                    }

                    match (shutdown, is_empty) {
                        (false, true) => {
                            if shrink_to_fit {
                                scheduler.shrink_to_fit();
                                unpolled.shrink_to_fit();
                                effects.shrink_to_fit();

                                shrink_to_fit = false;
                            }

                            drop(effects); // holding the RefMut while we block just feels wrong…

                            match scheduler.front() {
                                None => wait.wait(),
                                Some((when, _)) => {
                                    if cfg!(not(any(test, feature = "testing"))) {
                                        wait.wait_timeout(
                                            when.saturating_duration_since(Instant::now()), // a fresh `now`, for precision
                                        );
                                    } else {
                                        wait.wait(); // the test will wake us when it’s time
                                    }
                                }
                            }
                        }
                        (_, false) => {
                            unpolled.extend(effects.tasks.keys());
                            tasks.append(&mut effects.tasks); // MUST come after keys() → unpolled; .append empties .tasks
                            scheduler.append(&mut effects.timers);
                            scheduler.make_contiguous().sort_by_key(|(when, _)| *when);
                        }
                        (true, true) => {
                            return (*state).into();
                        }
                    }
                }
            })
            .expect("Reactor thread")
    }

    pub fn stop(&self) {
        self.other.send(Reason::Shutdown)
    }

    #[inline(never)]
    fn reduce(
        state: &mut State,
        action: <State as Reducer>::Action,
        effects: &Rc<RefCell<Inner<<State as Reducer>::Action>>>,
    ) where
        <State as Reducer>::Action: 'static,
    {
        state.reduce(action, Rc::downgrade(effects));

        // wrapping the `borrow_mut` in a closure to ensure that the
        // `borrow_mut` is dropped immediately so that the action is
        // free to push further actions onto `effects`
        let next = || effects.borrow_mut().actions.pop_front();

        // side effects MUST be run immediately as to never accidentally
        // interleave them with other actions (or external events)
        while let Some(action) = next() {
            state.reduce(action, Rc::downgrade(effects));
        }
    }
}
