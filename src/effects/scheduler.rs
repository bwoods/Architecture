use std::cmp::Reverse;
use std::collections::VecDeque;
use std::future::Future;
use std::mem::replace;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread::{park, park_timeout, Builder, JoinHandle};
use std::time::Instant;

use crate::dependencies::{Dependency, DependencyDefault};

enum State {
    New(Instant),
    Waiting(Waker),
    Ready,
}

#[derive(Clone)]
pub struct Delay(Arc<Mutex<State>>);

impl Delay {
    pub fn new(instant: Instant) -> Self {
        Delay(Arc::new(Mutex::new(State::New(instant))))
    }
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.0.lock().unwrap_or_else(|err| err.into_inner());

        match &mut *state {
            State::New(instant) => {
                let instant = *instant;
                *state = State::Waiting(cx.waker().clone());
                drop(state);

                // Now that it has a Waker…
                let scheduler = Dependency::<Scheduler>::new();
                scheduler.add(instant, self.0.clone());

                Poll::Pending
            }
            State::Waiting(waker) => {
                waker.clone_from(cx.waker()); // update the waker if needed
                Poll::Pending
            }
            State::Ready => Poll::Ready(()),
        }
    }
}

/// Shared between the `Scheduler` and its polling Thread (if any)
#[derive(Default)]
struct Shared {
    queue: Queue<Instant, Arc<Mutex<State>>>,
}

struct Scheduler {
    shared: Arc<Mutex<Shared>>,
    handle: JoinHandle<()>,
}

impl Scheduler {
    #[inline(never)]
    pub fn add(&self, new: Instant, state: Arc<Mutex<State>>) {
        let mut shared = self.shared.lock().unwrap();
        let next = shared.queue.peek_next();
        shared.queue.insert(new, state);
        drop(shared);

        match next {
            None => self.handle.thread().unpark(), // no `unpark` is scheduled yet
            Some(pending) if new < pending => self.handle.thread().unpark(),
            _ => {}
        }
    }
}

impl Default for Scheduler {
    #[inline(never)]
    fn default() -> Self {
        let shared = Arc::new(Mutex::<Shared>::default());
        let remote = shared.clone();

        let handle = Builder::new()
            .name(std::any::type_name::<Self>().into())
            .spawn(move || {
                loop {
                    let now = Instant::now();

                    let mut shared = remote.lock().unwrap();
                    let delays = shared.queue.drain_until(now);
                    let next = shared.queue.peek_next();
                    drop(shared); // release the `Mutex` in case any of the delayed work wants the `Scheduler`

                    for delay in delays {
                        let mut state = delay.lock().unwrap();
                        let waiting = replace(&mut *state, State::Ready);
                        drop(state); // release the `Mutex` before the waker is called

                        match waiting {
                            State::Waiting(waker) => waker.wake(),
                            _ => unreachable!(),
                        }
                    }

                    match next {
                        None => park(),
                        Some(when) => park_timeout(when - now),
                    }
                }
            })
            .expect("scheduler thread");

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

impl<Key: Copy, Value> Queue<Key, Value> {
    pub fn peek_next(&self) -> Option<Key> {
        self.deque.back().map(|kv| kv.0 .0)
    }
}

impl<Key: PartialOrd, Value> Queue<Key, Value> {
    pub fn insert(&mut self, key: Key, value: Value) {
        let key = Reverse(key);
        let index = self.deque.partition_point(|x| x.0 <= key);
        self.deque.insert(index, (key, value));
    }

    pub fn drain_until(&mut self, key: Key) -> impl Iterator<Item = Value> {
        let key = Reverse(key); // without the use of `Reverse` for the keys `split_off` would return the wrong half
        let index = self.deque.partition_point(|x| x.0 < key);
        self.deque.split_off(index).into_iter().rev().map(|kv| kv.1)
    }
}
