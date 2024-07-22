use std::cmp::Reverse;
use std::collections::VecDeque;
use std::mem::replace;
use std::sync::{Arc, Mutex};
use std::thread::{park, park_timeout, Builder, JoinHandle};
use std::time::Instant;

use crate::dependencies::DependencyDefault;
use crate::effects::delay::State;

/// Shared between the `Scheduler` and its polling Thread
#[derive(Default)]
struct Shared {
    pub(crate) queue: Queue<Instant, Arc<Mutex<State>>>,
}

impl Shared {
    pub fn poll(now: Instant, shared: &Mutex<Shared>) -> Option<Instant> {
        let mut shared = shared.lock().unwrap();
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

        next
    }
}

/// A minimal [Reactor] that powers the `Delay` future/stream.
///
/// [Reactor]: https://rust-lang.github.io/async-book/08_ecosystem/00_chapter.html#async-runtimes
pub struct Reactor {
    shared: Arc<Mutex<Shared>>,
    handle: Option<JoinHandle<()>>,
}

impl Default for Reactor {
    #[inline(never)]
    fn default() -> Self {
        let shared = Arc::new(Mutex::<Shared>::default());
        let remote = shared.clone();

        let handle = Builder::new()
            .name(std::any::type_name::<Self>().into())
            .spawn(move || loop {
                let now = Instant::now();
                let next = Shared::poll(now, &remote);

                match next {
                    None => park(),
                    Some(when) => park_timeout(when.saturating_duration_since(now)),
                }
            })
            .expect("scheduler thread");

        Self {
            shared,
            handle: Some(handle),
        }
    }
}

impl DependencyDefault for Reactor {}

impl Reactor {
    pub(crate) fn new() -> Self {
        let shared = Arc::new(Mutex::<Shared>::default());

        Self {
            shared,
            handle: None,
        }
    }

    pub(crate) fn poll(&self, now: Instant) -> Instant {
        Shared::poll(now, &self.shared).unwrap_or(now)
    }

    #[inline(never)]
    pub(crate) fn add(&self, new: Instant, state: Arc<Mutex<State>>) {
        let mut shared = self.shared.lock().unwrap();
        let next = shared.queue.peek_next();
        shared.queue.insert(new, state);
        drop(shared);

        match (&self.handle, next) {
            (Some(handle), None) => handle.thread().unpark(), // no `unpark` is scheduled yet
            (Some(handle), Some(pending)) if new < pending => handle.thread().unpark(),
            _ => {}
        }
    }
}

pub(crate) struct Queue<Key, Value> {
    deque: VecDeque<(Reverse<Key>, Value)>,
}

// Using `#[derive(Default)]` adds a `Default` requirement to Key
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
        let key = Reverse(key);
        let index = self.deque.partition_point(|x| x.0 < key);

        // without the use of `Reverse` for the keys `split_off` would return the wrong half
        self.deque.split_off(index).into_iter().rev().map(|kv| kv.1)
    }
}
