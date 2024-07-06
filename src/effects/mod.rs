#![doc = include_str!("README.md")]

use std::cell::RefCell;
use std::cmp::max;
use std::collections::VecDeque;
use std::marker::PhantomData as Marker;
use std::rc::Weak;
use std::time::{Duration, Instant};

use crate::effects::scheduler::Delay;
pub(crate) use crate::effects::task::Executor;
#[doc(hidden)]
pub use crate::effects::task::Task;
use futures::stream::unfold;
use futures::{future, stream::once, Future, Stream, StreamExt};

mod scheduler;
mod task;

/// `Effects` are used within `Reducer`s to propagate `Action`s as side-effects of performing other `Action`s.
///
/// `Effects` are also [`Scheduler`]s; able to apply modifiers to when (and how often) `Action`s. are sent.
///
/// See [the module level documentation](self) for more.
pub trait Effects: Clone {
    /// The `Action` type sent by this `Effects`.
    type Action;

    /// An effect that immediately sends an [`Action`][`Self::Action`] through
    /// the `Store`’s [`Reducer`][`crate::Reducer`].
    fn send(&self, action: impl Into<Self::Action>);

    /// A [`Task`] represents asynchronous work that will then [`send`][`crate::Store::send`]
    /// zero or more [`Action`][`Self::Action`]s back into the `Store`’s [`Reducer`][`crate::Reducer`]
    /// as it runs.
    ///
    /// Use this method if you need to ability to [`cancel`][Task::cancel] the task
    /// while it is running. Otherwise [`future`][Effects::future] or [`stream`][Effects::stream]
    /// should be preferred.
    fn task<S: Stream<Item = Self::Action> + 'static>(&self, stream: S) -> Task;

    /// An effect that runs a [`Future`][`std::future`] and, if it returns an
    /// [`Action`][`Self::Action`], sends it through the `Store`’s [`Reducer`][`crate::Reducer`].
    #[inline]
    fn future<F: Future<Output = Option<Self::Action>> + 'static>(&self, future: F)
    where
        Self::Action: 'static,
    {
        let stream = once(future).map(future::ready).filter_map(|option| option);
        self.task(stream).detach()
    }

    /// An effect that runs a [`Stream`](https://docs.rs/futures/latest/futures/stream/index.html)
    /// and sends every [`Action`][`Self::Action`] it returns through the `Store`’s
    /// [`Reducer`][`crate::Reducer`].
    #[inline(always)]
    fn stream<S: Stream<Item = Self::Action> + 'static>(&self, stream: S) {
        self.task(stream).detach()
    }

    /// Scopes the `Effects` to one that sends child actions.
    ///
    /// For example, the inner loop of the [`RecursiveReducer`] macro is,
    /// effectively, just calling
    ///
    /// ```rust ignore
    /// if let Ok(action) = action.clone().try_into() {
    ///     reduce(&mut self.child_reducer, action, effects.scope());
    /// }
    /// ```
    /// on each child-reducer.
    ///
    /// [`RecursiveReducer`]: crate::derive_macros
    #[inline(always)]
    fn scope<ChildAction>(&self) -> Scoped<Self, ChildAction>
    where
        Self::Action: From<ChildAction>,
    {
        Scoped(self.clone(), Marker)
    }
}

pub struct Scoped<Parent, Child>(Parent, Marker<Child>);

// Using `#[derive(Clone)]` adds a `Clone` requirement to all `Action`s
impl<Parent: Clone, Child> Clone for Scoped<Parent, Child> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Scoped(self.0.clone(), Marker)
    }
}

impl<Parent, Child> Effects for Scoped<Parent, Child>
where
    Parent: Effects,
    <Parent as Effects>::Action: From<Child>,
{
    type Action = Child;

    #[inline(always)]
    fn send(&self, action: impl Into<Self::Action>) {
        self.0.send(action.into());
    }

    #[inline(always)]
    fn task<S: Stream<Item = Child> + 'static>(&self, stream: S) -> Task {
        self.0.task(stream.map(|action| action.into()))
    }
}

#[doc(hidden)]
// `Parent` for `Effects::scope` tuples
impl<Action: 'static> Effects for Weak<RefCell<VecDeque<Action>>> {
    type Action = Action;

    #[inline(always)]
    fn send(&self, action: impl Into<Self::Action>) {
        if let Some(actions) = self.upgrade() {
            actions.borrow_mut().push_back(action.into())
        }
    }

    fn task<S: Stream<Item = Action> + 'static>(&self, stream: S) -> Task {
        Task::new(stream)
    }
}

/// `Effects` are also [`Scheduler`]s; able to apply modifiers to when (and how often) `Action`s. are sent.
pub trait Scheduler {
    /// The [`Effects::Action`].
    type Action;

    /// Sends the `Action` after this `Instant` of time.
    fn after(&self, instant: Instant, action: impl Into<Self::Action> + 'static) -> Task;

    /// Sends the `Action` every `Interval` of time.
    fn every(&self, interval: Interval, action: impl Into<Self::Action> + Clone + 'static) -> Task;

    /// An effect that sends an [`Action`][`Self::Action`] through the `Store`’s
    /// [`Reducer`][`crate::Reducer`] if at least one `interval` of time has passed
    /// since `previous` was sent. Otherwise, all subsequent actions but the last
    /// are dropped until that time; which resets the countdown until the next
    /// debounced action can be sent.
    ///
    /// The `debounce` function will automatically update the information
    /// stored in `previous` as it runs. The `Task` debounced by this call
    /// will be the _previous_ task for the next call, if any.
    fn debounce(
        &self,
        action: impl Into<Self::Action> + 'static,
        previous: &mut Option<Task>,
        interval: Interval,
    ) where
        Self::Action: 'static;

    /// An effect that coalesces repeated attempts to send [`Action`][`Self::Action`]s
    /// through the`Store`’s [`Reducer`][`crate::Reducer`] into a singe send.
    /// Once `timeout` has elapsed with no further actions being attempted, a single
    /// `Action` will be sent.
    ///
    /// The `coalesce` function will automatically update the information
    /// stored in `previous` as it runs. The `Task` coalesced by this call
    /// will be the _previous_ task for the next call, if any.
    fn coalesce(
        &self,
        action: impl Into<Self::Action> + 'static,
        previous: &mut Option<Task>,
        timeout: Duration,
    ) where
        Self::Action: 'static;
}

impl<T: Effects> Scheduler for T {
    type Action = T::Action;

    fn after(&self, instant: Instant, action: impl Into<Self::Action> + 'static) -> Task {
        self.task(once(async move {
            Delay::new(instant).await;
            action.into()
        }))
    }

    fn every(&self, interval: Interval, action: impl Into<Self::Action> + Clone + 'static) -> Task {
        let (n, delay) = match interval {
            Interval::Leading(delay) => (0, delay), // 0 × delay => no initial delay
            Interval::Trailing(delay) => (1, delay),
        };

        let start = Instant::now(); // FIXME
        self.task(unfold(n, move |n| {
            let action = action.clone();

            async move {
                let instant = start.checked_add(delay.checked_mul(n)?)?;
                Delay::new(instant).await;

                Some((action.into(), n + 1))
            }
        }))
    }

    fn debounce(
        &self,
        action: impl Into<Self::Action> + 'static,
        previous: &mut Option<Task>,
        interval: Interval,
    ) where
        Self::Action: 'static,
    {
        let now = Instant::now(); // FIXME
        let delay = interval.duration();

        let when = match previous.take().and_then(|task| task.when) {
            Some(when) if when > now => when, // previous was not yet sent — replace it
            Some(when) if when + delay > now => when + delay, // previous was sent recently; delay this send
            _ => match interval {
                Interval::Leading(_) => now,
                Interval::Trailing(_) => now + delay,
            },
        };

        let task = self.after(when, action);
        *previous = Some(task);
    }

    fn coalesce(
        &self,
        action: impl Into<Self::Action> + 'static,
        waiting: &mut Option<Task>,
        timeout: Duration,
    ) where
        Self::Action: 'static,
    {
        let now = Instant::now(); // FIXME

        let when = waiting
            .take()
            .and_then(|task| task.when)
            .map(|previous| max(previous + timeout, now))
            .unwrap_or_else(|| now + timeout);

        let task = self.after(when, action);
        *waiting = Some(task);
    }
}

/// When a [`Scheduler`] uses a repeating interval, that interval can begin immediately, a `Leading`
/// interval, or it may begin after the first delay, a `Trailing` interval.
pub enum Interval {
    /// The first `Action` should be sent immediately.
    Leading(Duration),
    /// The first `Action` should not be send until after the first `Interval` has passed.
    Trailing(Duration),
}

impl Interval {
    pub fn duration(&self) -> Duration {
        match self {
            Interval::Leading(duration) => *duration,
            Interval::Trailing(duration) => *duration,
        }
    }
}
