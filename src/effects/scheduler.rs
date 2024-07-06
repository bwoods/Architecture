use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::VecDeque;
use std::rc::Weak;
use std::sync::Mutex;
use std::thread::{current, park_timeout, Thread};
use std::time::Instant;

use crate::effects::Effects;
use crate::store::channel::WeakSender;

struct Scheduler<Action> {
    shared: Mutex<Queue<Instant, Action>>,
    thread: Option<Thread>,

    effects: Result<WeakSender<Action>, Weak<RefCell<VecDeque<Action>>>>,
}

impl<Action: 'static> Scheduler<Action> {
    pub fn after(&mut self, instant: Instant, action: Action) {
        let mut shared = self.shared.lock().unwrap();
        let next = shared.peek_next();
        shared.insert(instant, action);
        drop(shared);

        if let (Some(next), Some(thread)) = (next, self.thread.as_ref()) {
            if instant < next {
                debug_assert!(thread.id() != current().id());
                thread.unpark();
            }
        }
    }

    pub(crate) fn advance_to(&self, now: Instant) {
        let mut shared = self.shared.lock().unwrap();
        let events = shared.drain_up_to(now);
        let next = shared.peek_next();
        drop(shared); // release the Mutex before the actions are sent

        for action in events {
            match &self.effects {
                Ok(sender) => sender.send(action),
                Err(sender) => sender.send(action),
            }
        }

        if let (Some(next), Some(thread)) = (next, self.thread.as_ref()) {
            debug_assert!(thread.id() == current().id());
            park_timeout(next - now);
        }
    }
}

struct Queue<Key, Value> {
    deque: VecDeque<(Reverse<Key>, Value)>,
}

impl<Key: Clone, Value> Queue<Key, Value> {
    pub fn peek_next(&self) -> Option<Key> {
        self.deque.back().map(|kv| kv.0.clone().0)
    }
}

impl<Key: PartialOrd, Value> Queue<Key, Value> {
    pub fn insert(&mut self, key: Key, value: Value) {
        let key = Reverse(key);
        let index = self.deque.partition_point(|x| x.0 <= key);
        self.deque.insert(index, (key, value));
    }

    pub fn drain_up_to(&mut self, key: Key) -> impl Iterator<Item = Value> {
        let key = Reverse(key);
        let index = self.deque.partition_point(|x| x.0 < key);
        // without the use of `Reverse` for the keys `split_off` would return the wrong half!
        self.deque.split_off(index).into_iter().rev().map(|kv| kv.1)
    }
}

#[test]
#[ignore]
fn scratch() {
    let mut values = [1, 2, 3, 3, 5, 6, 7].map(|n| Reverse(n));
    values.sort();

    let mut deque: VecDeque<_> = values.into();

    let value = Reverse(10);
    let index = deque.partition_point(|&x| x < value);
    deque.insert(index, value);

    let limit = Reverse(5);
    let index = deque.partition_point(|&x| x < limit);
    let chunk = deque.split_off(index);

    println!("queue: {:?}", deque);
    println!("chunk: {:?}", chunk);
    println!(" next: {:?}", deque.back().unwrap().0);
}
