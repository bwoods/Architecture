use crate::effects::Inner;
use futures::stream::{AbortHandle, PollNext, abortable, select_with_strategy};
use futures::{FutureExt, stream};
use futures::{Stream, future};
use std::cell::RefCell;
use std::iter;
use std::ops::ControlFlow;
use std::rc::Weak;
use std::time::{Duration, Instant};

/// `Effects` are also `Scheduler`s — able to apply modifiers to when (and how often) `Action`s are sent.
pub trait Scheduler {
    type Item;

    #[doc(hidden)]
    fn now(&self) -> Instant;

    #[doc(hidden)]
    fn schedule_stream<
        S: Stream<Item = ControlFlow<Instant, <Self as Scheduler>::Item>> + 'static,
    >(
        &self,
        stream: S,
    );

    #[doc(hidden)]
    fn schedule_task<S: Stream<Item = ControlFlow<Instant, <Self as Scheduler>::Item>> + 'static>(
        &self,
        stream: S,
        when: Option<Instant>,
    ) -> Task;

    /// Sends the `Action` after `duration`.
    fn after(&self, duration: Duration, action: Self::Item) -> Task
    where
        Self::Item: 'static,
    {
        if let Some(instant) = self.now().checked_add(duration) {
            self.at(instant, action)
        } else {
            self.schedule_task(future::pending().into_stream(), None) // pending forever on overflow
        }
    }

    /// Sends the `Action` at `instant`.
    fn at(&self, instant: Instant, action: Self::Item) -> Task
    where
        Self::Item: 'static,
    {
        let stream = stream::iter([ControlFlow::Break(instant), ControlFlow::Continue(action)]);
        self.schedule_task(stream, Some(instant))
    }

    /// Sends the `Action` every `interval`.
    fn every(&self, interval: Interval, action: Self::Item) -> Task
    where
        Self::Item: Clone + 'static,
    {
        let start = self.now();
        let (mut n, duration) = match interval {
            Interval::Leading(duration) => (0, duration), // 0 × delay => no initial delay
            Interval::Trailing(duration) => (1, duration),
        };

        let actions = stream::repeat_with(move || ControlFlow::Continue(action.clone()));
        let delays = stream::iter(iter::from_fn(move || {
            // if Instant or Duration overflow, the Iterator returns None
            let instant = start.checked_add(duration.checked_mul(n)?)?;
            n = n.checked_add(1)?;

            Some(ControlFlow::Break(instant))
        }));

        let alternating = |prev: &mut PollNext| prev.toggle();
        let stream = select_with_strategy(delays, actions, alternating);
        self.schedule_task(stream, Some(start))
    }

    fn debounce(&self, action: Self::Item, previous: &mut Option<Task>, interval: Interval)
    where
        Self::Item: 'static,
    {
        let task = match interval {
            Interval::Trailing(duration) => self.after(duration, action),
            Interval::Leading(duration) => {
                let now = self.now();
                match previous.as_ref().and_then(|task| task.when) {
                    None => self.at(now, action),
                    Some(then) => {
                        if now < then + duration {
                            return; // A leading debounce DROPS subsequent actions within the interval
                        }

                        self.at(now, action)
                    }
                }
            }
        };

        *previous = Some(task);
    }

    fn throttle(&self, action: Self::Item, previous: &mut Option<Task>, interval: Interval)
    where
        Self::Item: 'static,
    {
        let now = self.now();
        let duration = match interval {
            Interval::Leading(duration) => duration,
            Interval::Trailing(duration) => duration,
        };

        let when = match previous.take().and_then(|task| task.when) {
            Some(when) if when >= now => when, // previous was not yet sent — replace it
            Some(when) if when + duration > now => when + duration, // previous was sent within the interval
            _ => match interval {
                Interval::Leading(_) => now,
                Interval::Trailing(_) => now + duration,
            },
        };

        let task = self.at(when, action);
        *previous = Some(task);
    }
}

#[derive(Debug)]
pub struct Task {
    handle: Option<AbortHandle>,
    when: Option<Instant>,
}

impl Task {
    /// Detaches the task; leaving its [`Stream`][`futures::Stream`] running in the background.
    pub fn detach(mut self) {
        self.handle = None
    }

    /// Cancels the task; meaning its [`Stream`][`futures::Stream`] won’t be polled again.
    pub fn cancel(self) {
        drop(self)
    }
}

impl Drop for Task {
    /// Dropping a task, without detaching it first, cancels it.
    fn drop(&mut self) {
        if let Some(handle) = self.handle.as_ref() {
            handle.abort();
        }
    }
}

/// When a [`Scheduler`] uses a repeating interval, that interval can begin immediately, a `Leading`
/// interval, or it may begin after the first delay, a `Trailing` interval.
pub enum Interval {
    /// The first `Action` should be sent immediately.
    Leading(Duration),
    /// The first `Action` should not be send until after the `Duration` has passed.
    Trailing(Duration),
}

#[doc(hidden)]
impl<Action: 'static> Scheduler for Weak<RefCell<Inner<Action>>> {
    type Item = Action;

    fn now(&self) -> Instant {
        self.upgrade()
            .map(|effects| effects.borrow().now.get())
            .unwrap_or_else(Instant::now) // if we’ve been deallocated, the answer no longer matters…?
    }

    fn schedule_stream<S>(&self, stream: S)
    where
        S: Stream<Item = ControlFlow<Instant, <Self as Scheduler>::Item>> + 'static,
    {
        if let Some(effects) = self.upgrade() {
            let mut effects = effects.borrow_mut();
            let id = effects.next_id();

            effects.tasks.insert(id, Box::pin(stream));
        }
    }

    fn schedule_task<S>(&self, stream: S, when: Option<Instant>) -> Task
    where
        S: Stream<Item = ControlFlow<Instant, <Self as Scheduler>::Item>> + 'static,
    {
        let (stream, handle) = abortable(stream);
        self.schedule_stream(stream);

        Task {
            handle: Some(handle),
            when,
        }
    }
}
