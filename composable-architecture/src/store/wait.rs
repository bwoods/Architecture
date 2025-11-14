use std::collections::VecDeque;
use std::mem::take;
use std::sync::{Arc, Condvar, Mutex};
use std::task::{Wake, Waker};
use std::time::Duration;

pub(crate) struct Wait {
    condvar: Condvar,
    unparked: Mutex<u32>,
}

impl Wait {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            condvar: Condvar::new(),
            unparked: Mutex::new(1),
        })
    }

    pub fn waker<F: Fn() + Send + Sync + 'static>(f: F) -> Waker {
        Waker::from(Arc::new(WakeFn(f)))
    }

    // only wait if all unparks are accounted for
    #[inline(always)]
    fn while_parked(unparked: &mut u32) -> bool {
        *unparked == 0
    }

    pub fn wait(&self) {
        let mut unparked = self.unparked.lock().unwrap_or_else(|err| err.into_inner());
        *unparked -= 1;

        let unparked = self
            .condvar
            .wait_while(unparked, Self::while_parked)
            .unwrap();
        debug_assert_ne!(*unparked, 0);
    }

    pub fn wait_timeout(&self, duration: Duration) {
        let mut unparked = self.unparked.lock().unwrap_or_else(|err| err.into_inner());
        *unparked -= 1;

        let (mut unparked, reason_was) = self
            .condvar
            .wait_timeout_while(unparked, duration, Self::while_parked)
            .unwrap();

        if reason_was.timed_out() {
            debug_assert_eq!(*unparked, 0);
            *unparked += 1; // mark as unparked
        }
    }

    pub fn wake(&self) {
        let mut unparked = self.unparked.lock().unwrap_or_else(|err| err.into_inner());
        *unparked += 1;
        drop(unparked); // eagerly release the lock

        self.condvar.notify_one();
    }
}

pub(crate) struct Queue<T> {
    queue: Mutex<VecDeque<T>>,
    wait: Arc<Wait>,
}

impl<T> Queue<T> {
    pub fn new(wait: Arc<Wait>) -> Arc<Self> {
        Arc::new(Self {
            queue: Default::default(),
            wait,
        })
    }

    pub fn send(&self, value: T) {
        let mut queue = self.queue.lock().unwrap_or_else(|err| err.into_inner());
        queue.push_back(value);
        drop(queue); // eagerly release the lock

        self.wait.wake();
    }

    pub fn is_empty(&self) -> bool {
        let queue = self.queue.lock().unwrap_or_else(|err| err.into_inner());
        queue.is_empty()
    }

    pub fn take(&self) -> VecDeque<T> {
        let mut queue = self.queue.lock().unwrap_or_else(|err| err.into_inner());
        take(&mut *queue)
    }

    pub fn waker(self: &Arc<Self>, value: T) -> Waker
    where
        T: Send + Sync + Clone + 'static,
    {
        let queue = self.clone();

        Waker::from(Arc::new(WakeFn(move || {
            queue.send(value.clone());
        })))
    }
}

struct WakeFn<F>(F);

impl<F: Fn() + Send + Sync + 'static> Wake for WakeFn<F> {
    #[inline(always)]
    fn wake(self: Arc<Self>) {
        Self::wake_by_ref(&self)
    }

    fn wake_by_ref(self: &Arc<Self>) {
        (self.0)()
    }
}
