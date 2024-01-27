use std::cell::{OnceCell, RefCell};
use std::collections::VecDeque;
use std::pin::Pin;
use std::sync::{Arc, Mutex, MutexGuard, Weak};
use std::task::{Context, Poll, Waker};

use futures::Stream;

pub struct Shared<T> {
    queue: VecDeque<T>,
    waker: OnceCell<Waker>,
}

impl<T> Default for Shared<T> {
    fn default() -> Self {
        Shared {
            queue: Default::default(),
            waker: Default::default(),
        }
    }
}

#[derive(Default)]
pub struct Receiver<T> {
    shared: Arc<Mutex<Shared<T>>>,
    queue: RefCell<VecDeque<T>>,
}

impl<T> Stream for Receiver<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let internal = &mut *self.queue.borrow_mut();

        if let Some(val) = internal.pop_front() {
            return Poll::Ready(Some(val));
        }

        let mut guard = self.shared.lock().unwrap();
        let external = &mut guard.queue;

        match external.pop_front() {
            Some(val) => {
                // move all other pending values (if any) into the (un-Mutex’d) internal queue
                std::mem::swap(external, internal);
                Poll::Ready(Some(val))
            }
            None if Arc::strong_count(&self.shared) == 1 => {
                Poll::Ready(None) // no receivers remaining
            }
            None => {
                match guard.waker.get_mut() {
                    Some(waker) => waker.clone_from(cx.waker()),
                    None => {
                        let cell = OnceCell::new();
                        cell.set(cx.waker().clone()).ok();
                        guard.waker = cell;
                    }
                }

                Poll::Pending
            }
        }
    }
}

pub struct Sender<T> {
    shared: Arc<Mutex<Shared<T>>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Sender {
            shared: self.shared.clone(),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        self.wake_after(|_| {})
    }
}

impl<T> Sender<T> {
    pub fn wake_after<F: FnOnce(MutexGuard<Shared<T>>)>(&self, f: F) {
        let mut guard = self.shared.lock().unwrap();
        let waker = guard.waker.take();
        f(guard);

        if let Some(waker) = waker {
            waker.wake()
        }
    }

    pub fn send(&self, value: T) {
        self.wake_after(|mut shared| shared.queue.push_back(value))
    }

    pub(crate) fn downgrade(&self) -> WeakSender<T> {
        WeakSender {
            shared: Arc::downgrade(&self.shared),
        }
    }

    pub(crate) fn receiver(&self) -> Receiver<T> {
        Receiver {
            shared: self.shared.clone(),
            queue: Default::default(),
        }
    }
}

pub(crate) struct WeakSender<T> {
    shared: Weak<Mutex<Shared<T>>>,
}

impl<T> Clone for WeakSender<T> {
    fn clone(&self) -> Self {
        WeakSender {
            shared: self.shared.clone(),
        }
    }
}

impl<T> WeakSender<T> {
    pub fn upgrade(&self) -> Option<Sender<T>> {
        self.shared.upgrade().map(|shared| Sender { shared })
    }

    pub(crate) fn receiver(&self) -> Option<Receiver<T>> {
        self.shared.upgrade().map(|shared| Receiver {
            shared,
            queue: Default::default(),
        })
    }
}

pub fn unbounded<T>() -> Sender<T> {
    Sender {
        shared: Default::default(),
    }
}
