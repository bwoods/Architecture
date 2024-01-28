use std::cell::RefCell;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, MutexGuard, Weak};
use std::task::{Context, Poll, Waker};
use std::{mem::swap, pin::Pin};

use futures::Stream;

struct Shared<T> {
    queue: VecDeque<T>,
    waker: Option<Waker>,
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
    buffer: RefCell<VecDeque<T>>,
}

impl<T> Stream for Receiver<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let internal = &mut *self.buffer.borrow_mut();

        if let Some(value) = internal.pop_front() {
            return Poll::Ready(Some(value));
        }

        let mut shared = self
            .shared
            .lock() //
            .unwrap_or_else(|err| err.into_inner());
        let external = &mut shared.queue;

        match external.pop_front() {
            Some(value) => {
                // move all other pending values (if any) into the (un-Mutex’d) internal buffer
                swap(external, internal);
                Poll::Ready(Some(value))
            }
            None if Arc::strong_count(&self.shared) == 1 => {
                Poll::Ready(None) // no receivers remaining
            }
            None => {
                match shared.waker.as_mut() {
                    None => shared.waker = Some(cx.waker().clone()),
                    Some(waker) => waker.clone_from(cx.waker()),
                };

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
        self.wake_after(|shared| drop(shared)) // redundant, but…
    }
}

impl<T> Sender<T> {
    pub fn send(&self, value: T) {
        self.wake_after(move |mut shared| shared.queue.push_back(value))
    }

    pub fn downgrade(&self) -> WeakSender<T> {
        WeakSender {
            shared: Arc::downgrade(&self.shared),
        }
    }

    /// Perform some work and then, if a `Receiver` was waiting, wake it.
    ///
    /// Note that the [`Waker`] will only ever be callen once for each time it
    /// has entered the [`Poll::Pending`] state. Regardless of how many times
    /// `wake_after` is called.
    fn wake_after<F: FnOnce(MutexGuard<Shared<T>>)>(&self, f: F) {
        let mut shared = self
            .shared
            .lock() //
            .unwrap_or_else(|err| err.into_inner());

        let waker = shared.waker.take(); // there are no “extra” wakes
        f(shared);

        if let Some(waker) = waker {
            waker.wake() // wake _after_ the `MutexGuard` has been dropped
        }
    }
}

pub struct WeakSender<T> {
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
        self.shared
            .upgrade() //
            .map(|shared| Sender { shared })
    }
}

pub struct WeakReceiver<T> {
    shared: Weak<Mutex<Shared<T>>>,
}

impl<T> WeakReceiver<T> {
    pub fn upgrade(self) -> Option<Receiver<T>> {
        self.shared
            .upgrade() //
            .map(|shared| Receiver {
                shared,
                buffer: Default::default(),
            })
    }
}

pub fn channel<T>() -> (Sender<T>, WeakReceiver<T>) {
    let shared = Default::default();

    let recv = WeakReceiver {
        shared: Arc::downgrade(&shared),
    };
    let send = Sender { shared };

    (send, recv)
}
