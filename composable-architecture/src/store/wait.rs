use std::collections::VecDeque;
use std::mem::take;
use std::sync::{Arc, Condvar, Mutex};
use std::task::{Wake, Waker};
use std::time::Duration;

/// A [counting semaphore][Wikipedia].
///
/// > - [`wait`]: Decrements the value of the semaphore variable by 1. If the new
/// >             value of the semaphore variable is zero, the thread executing
/// >             `wait` is `park`ed.
/// > - [`wake`]: Increments the value of the semaphore variable by 1. After the
/// >             increment, if the pre-increment value was zero, it `unpark`s
/// >             the waiting thread.[^1]
///
/// [^1]: [Wikipedia] (with modifications)
///
/// Using [`park`] and [`unpark`] directly contains a race condition. If `unpark`
/// is called before the thread `park`s, when the thread eventually does `park`
/// it will wait forever as the previous `unpark` call was ignored.
///
/// Using `Wait` solves this.
///
/// [Wikipedia]: https://en.wikipedia.org/wiki/Semaphore_(programming)#Semantics_and_implementation
/// [`park`]: std::thread::park
/// [`unpark`]: std::thread::Thread::unpark
/// [`wake`]: Wait::wake
/// [`wait`]: Wait::wait
///
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

    /// Blocks the current thread until [`wake`] is called. If `wake` has
    /// already been called, the thread is left running.
    ///
    /// [`wake`]: Wait::wake
    pub fn wait(&self) {
        let mut unparked = self.unparked.lock().unwrap_or_else(|err| err.into_inner());
        *unparked = unparked.checked_sub(1).expect("wait underflow");

        let unparked = self
            .condvar
            .wait_while(unparked, |unparked| *unparked == 0)
            .unwrap_or_else(|err| err.into_inner());
        debug_assert_ne!(*unparked, 0);
    }

    /// Blocks the current thread until [`wake`] is called, timing out after a specified duration.
    ///
    /// The semantics of this function are equivalent to [`wait`] except that the thread will be
    /// blocked for roughly no longer than `duration`.
    ///
    /// [`wake`]: Wait::wake
    /// [`wait`]: Wait::wait
    pub fn wait_timeout(&self, duration: Duration) {
        let mut unparked = self.unparked.lock().unwrap_or_else(|err| err.into_inner());
        *unparked = unparked.checked_sub(1).expect("wait_timeout underflow");

        let (mut unparked, reason_was) = self
            .condvar
            .wait_timeout_while(unparked, duration, |unparked| *unparked == 0)
            .unwrap();

        if reason_was.timed_out() {
            debug_assert_eq!(*unparked, 0);
            *unparked = unparked.checked_add(1).expect("wait_timeout overflow"); // mark as unparked
        }
    }

    /// Wakes up the thread blocked on this `Wait`.
    pub fn wake(&self) {
        let mut unparked = self.unparked.lock().unwrap_or_else(|err| err.into_inner());
        *unparked = unparked.checked_add(1).expect("wake overflow");
        drop(unparked); // release the lock

        self.condvar.notify_one();
    }
}

/// A threadsafe queue of values.
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

    /// Appends a value to the back of the queue and notifies [waker][Queue::waker]s.
    pub fn send(&self, value: T) {
        let mut queue = self.queue.lock().unwrap_or_else(|err| err.into_inner());
        queue.push_back(value);
        drop(queue); // release the lock

        self.wait.wake();
    }

    /// Returns true if the queue contains no elements.
    pub fn is_empty(&self) -> bool {
        let queue = self.queue.lock().unwrap_or_else(|err| err.into_inner());
        queue.is_empty()
    }

    /// Returns all the values currently within the `Queue`.
    ///
    /// To amortize the cost of locking, `Queue` only allows this batch removal
    /// of values. There are no `recv` or `pop_front`  methods.
    pub fn take(&self) -> impl IntoIterator<Item = T> {
        let mut queue = self.queue.lock().unwrap_or_else(|err| err.into_inner());
        take(&mut *queue)
    }

    /// Creates a [`Waker`] that wakes when [`send`] is called.
    ///
    /// [`send`]: Queue::send
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

/// Creates a [`Waker`] from a closure.
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
