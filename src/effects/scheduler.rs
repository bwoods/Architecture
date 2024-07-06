use std::cmp::Reverse;
use std::collections::VecDeque;
use std::future::Future;
use std::mem::{replace, swap};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread::{park, park_timeout, Builder, JoinHandle};
use std::time::Instant;

use crate::dependencies::{Dependency, DependencyDefault};

#[derive(Default)]
enum State {
    #[default]
    None,
    Pending(Waker),
    Ready(Instant),
}

impl State {
    fn is_some(&self) -> bool {
        match self {
            State::None => false,
            _ => true,
        }
    }
}

#[derive(Clone)]
pub struct Delay(Arc<Mutex<State>>);

impl Delay {
    pub fn new(instant: Instant) -> Self {
        let delay = Delay(Default::default());

        let scheduler = Dependency::<Scheduler>::new();
        scheduler.add(instant, delay.0.clone());

        delay
    }
}

/// When `await`ed as a `Future` it returns the `Instant` it scheduled for —
/// which may not necessarily be the same `Instant` it was actually returned.
impl Future for Delay {
    type Output = Instant;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.0.lock().unwrap();
        let state = &mut *state;

        match state {
            State::None => {
                swap(state, &mut State::Pending(cx.waker().clone()));
                Poll::Pending
            }
            State::Pending(waker) => {
                waker.clone_from(cx.waker()); // update the waker if needed
                Poll::Pending
            }
            State::Ready(now) => Poll::Ready(*now),
        }
    }
}

/// Shared between the `Scheduler` and its polling Thread (if any)
struct Shared {
    queue: Queue<Instant, Arc<Mutex<State>>>,
    now: Option<Instant>,
}

impl Default for Shared {
    fn default() -> Self {
        Self {
            queue: Default::default(),
            now: None,
        }
    }
}

struct Scheduler {
    shared: Arc<Mutex<Shared>>,
    handle: Option<JoinHandle<()>>,
}

impl Scheduler {
    pub fn add(&self, new: Instant, state: Arc<Mutex<State>>) {
        let mut shared = self.shared.lock().unwrap();
        let next = shared.queue.peek_next();
        shared.queue.insert(new, state);
        drop(shared);

        self.reschedule_polling_thread_if_needed(new, next);
    }

    pub(crate) fn advance_to(&self, now: Instant) {
        let mut shared = self.shared.lock().unwrap();
        let next = shared.queue.peek_next();
        shared.now = Some(now);
        drop(shared);

        self.reschedule_polling_thread_if_needed(now, next);
    }

    fn reschedule_polling_thread_if_needed(&self, new: Instant, next: Option<Instant>) {
        match (&self.handle, next) {
            (Some(polling), None) => polling.thread().unpark(), // no `unpark` is scheduled yet
            (Some(polling), Some(pending)) if new < pending => polling.thread().unpark(),
            _ => {}
        }
    }
}

impl Default for Scheduler {
    #[inline(never)]
    fn default() -> Self {
        let shared = Arc::new(Mutex::<Shared>::default());
        let remote = shared.clone();

        let handle = Some(
            Builder::new()
                .name(std::any::type_name::<Self>().into())
                .spawn(move || {
                    // thread loop ends if the `Mutex` is poisoned
                    while let Ok(mut shared) = remote.lock() {
                        let now = shared.now.take().unwrap_or_else(|| Instant::now());
                        let delayed = shared.queue.drain_until_while(now, |delay| {
                            delay // stop at un-polled Futures as they do not have a Waker yet
                                .try_lock() // (including being polled right now!)
                                .map(|state| state.is_some())
                                .ok()
                                .unwrap_or(false)
                        });
                        let next = shared.queue.peek_next();
                        drop(shared); // release the `Mutex` in case any of the delayed work wants the `Scheduler`

                        for (when, delay) in delayed {
                            let mut state = delay.lock().unwrap();
                            let waker = replace(&mut *state, State::Ready(when));
                            drop(state); // release the `Mutex` before the waker is called

                            match waker {
                                State::Pending(waker) => waker.wake(),
                                _ => unreachable!(),
                            }
                        }

                        match next {
                            None => park(),
                            Some(when) => park_timeout(when - now),
                        }
                    }
                })
                .expect("scheduler thread"),
        );

        Self { shared, handle }
    }
}

impl DependencyDefault for Scheduler {}

struct Queue<Key, Value> {
    deque: VecDeque<(Reverse<Key>, Value)>,
}

impl<Key, Value> Default for Queue<Key, Value> {
    fn default() -> Self {
        Queue {
            deque: Default::default(),
        }
    }
}

impl<Key: Clone, Value> Queue<Key, Value> {
    pub fn peek_next(&self) -> Option<Key> {
        self.deque.back().map(|kv| kv.0 .0.clone())
    }
}

impl<Key: PartialOrd, Value> Queue<Key, Value> {
    pub fn insert(&mut self, key: Key, value: Value) {
        let key = Reverse(key);
        let index = self.deque.partition_point(|x| x.0 <= key);
        self.deque.insert(index, (key, value));
    }

    pub fn drain_until_while(
        &mut self,
        key: Key,
        mut pred: impl FnMut(&Value) -> bool,
    ) -> impl Iterator<Item = (Key, Value)> {
        let key = Reverse(key);
        // without the use of `Reverse` for the keys `split_off` would return the wrong half!
        // similarly we have to reverse the predicate…
        let index = self.deque.partition_point(|x| x.0 < key || !pred(&x.1));
        self.deque
            .split_off(index)
            .into_iter()
            .rev()
            .map(|kv| (kv.0 .0, kv.1))
    }
}
