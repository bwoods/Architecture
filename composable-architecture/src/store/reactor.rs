use crate::Reducer;
use crate::effects::Inner;
use crate::store::wait::{Queue, Wait};
use futures::task::Poll;
use futures::{Stream, pin_mut};
use std::cell::RefCell;
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
    pub wait: Arc<Wait>,

    pub ready: Arc<Queue<Reason>>,
    pub recv: Arc<Queue<T>>,
}

#[derive(Copy, Clone)]
pub(crate) enum Reason {
    Wake(u64),

    ShrinkToFit,
    Shutdown,

    #[cfg(test)]
    Advance(std::time::Duration),
}

impl<State: Reducer, T, R> Reactor<State, T, R> {
    pub fn new<F>(with: F) -> Self
    where
        <State as Reducer>::Action: 'static, // T is required to be Send, so that Action is not
        F: Send + (FnOnce() -> State) + 'static,
        T: Send + Into<<State as Reducer>::Action> + 'static,
        R: Send + From<State> + 'static,
    {
        let wait = Wait::new();
        let recv = Queue::<T>::new(wait.clone());
        let ready = Queue::new(wait.clone());
        let handle = Self::start(with, ready.clone(), recv.clone(), wait.clone());

        Self {
            marker: PhantomData,
            handle,
            wait,
            ready,
            recv,
        }
    }

    fn start<F>(
        initial: F,
        ready: Arc<Queue<Reason>>,
        recv: Arc<Queue<T>>,
        wait: Arc<Wait>,
    ) -> JoinHandle<R>
    where
        <State as Reducer>::Action: 'static,
        F: Send + (FnOnce() -> State) + 'static,
        T: Send + Into<<State as Reducer>::Action> + 'static,
        R: Send + From<State> + 'static,
    {
        Builder::new()
            .name(std::any::type_name::<Self>().into())
            .spawn(move || {
                let mut state = initial();
                let effects = Inner::new();

                let mut tasks = BTreeMap::<
                    u64,
                    Pin<Box<dyn Stream<Item = ControlFlow<Instant, <State as Reducer>::Action>>>>,
                >::new();
                let mut scheduler = VecDeque::<(Instant, u64)>::new();
                let mut shrink_to_fit = false;
                let mut shutdown = false;

                let mut pending = VecDeque::<u64>::new();
                let mut now = Instant::now();

                loop {
                    for value in recv.take() {
                        Self::reduce(&mut state, value.into(), &effects);
                    }

                    // During unit tests, time only advances explicitly
                    if cfg!(not(test)) {
                        now = Instant::now();
                    }

                    let here = scheduler.partition_point(|&(when, _)| when <= now);
                    let due = scheduler.drain(..here).map(|(_, id)| id);

                    for id in
                        pending
                            .drain(..)
                            .chain(due)
                            .chain(ready.take().into_iter().filter_map(|reason| match reason {
                                Reason::Wake(id) => Some(id),
                                Reason::Shutdown => {
                                    shutdown = true;
                                    None
                                }
                                Reason::ShrinkToFit => {
                                    shrink_to_fit = true;
                                    None
                                }
                                #[cfg(test)]
                                Reason::Advance(duration) => {
                                    use std::ops::AddAssign;
                                    now.add_assign(duration);
                                    None
                                }
                            }))
                    {
                        if let Some(stream) = tasks.get_mut(&id) {
                            let waker = ready.waker(Reason::Wake(id)); // mimalloc is faster than a BtreeMap lookup…
                            let mut context = Context::from_waker(&waker);
                            pin_mut!(stream);

                            let completed = loop {
                                match stream.as_mut().poll_next(&mut context) {
                                    Poll::Ready(Some(ControlFlow::Continue(action))) => {
                                        Self::reduce(&mut state, action, &effects);
                                    }
                                    Poll::Ready(Some(ControlFlow::Break(until))) => {
                                        effects.borrow_mut().timers.push_back((until, id));
                                    }
                                    Poll::Ready(None) => break true,
                                    Poll::Pending => break false,
                                }
                            };

                            if completed {
                                tasks.remove(&id);
                            }
                        } // otherwise the task may have been canceled
                    }

                    let mut effects = effects.borrow_mut();
                    debug_assert!(effects.actions.is_empty());

                    #[allow(unused_mut)]
                    let mut is_empty = effects.is_empty();

                    // Wait until all processing is done when unit testing or benchmarking
                    #[cfg(any(test, feature = "testing"))]
                    {
                        is_empty &= tasks.is_empty();
                        is_empty &= recv.is_empty();
                    }

                    match (shutdown, is_empty) {
                        (false, true) => {
                            drop(effects); // drop RefMut (before we block)

                            if shrink_to_fit {
                                pending.shrink_to_fit();
                                scheduler.shrink_to_fit();
                                shrink_to_fit = false;
                            }

                            match scheduler.front() {
                                None => wait.wait(),
                                Some((when, _)) => wait
                                    .wait_timeout(when.saturating_duration_since(Instant::now())),
                            }
                        }
                        (_, false) => {
                            pending.extend(effects.tasks.keys());
                            tasks.append(&mut effects.tasks); // MUST come after keys() → pending; it empties .tasks
                            scheduler.append(&mut effects.timers);
                            scheduler.make_contiguous().sort_by_key(|(when, _)| *when);

                            continue;
                        }
                        (true, _) => {
                            return state.into();
                        }
                    }
                }
            })
            .expect("Reactor thread")
    }

    pub fn stop(&self) {
        self.ready.send(Reason::Shutdown)
    }

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
        // free to push further actions to `effects`
        let next = || effects.borrow_mut().actions.pop_front();

        // side effects MUST be run immediately as to never interleave
        // them with other actions
        while let Some(action) = next() {
            state.reduce(action, Rc::downgrade(effects));
        }
    }
}
