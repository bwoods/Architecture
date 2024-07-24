#![doc = include_str!("README.md")]

use std::cell::RefCell;
use std::collections::VecDeque;
use std::iter::from_fn;
use std::marker::PhantomData as Marker;
use std::rc::Weak;
use std::time::{Duration, Instant};

use futures::stream::{iter, once};
use futures::{Future, Stream, StreamExt};

pub(crate) use delay::Delay;
pub(crate) use task::Executor;
#[doc(hidden)]
pub use task::Task;

mod delay;
pub(crate) mod scheduler;
mod task;

/// `Effects` are used within `Reducer`s to propagate `Action`s as side-effects of performing other `Action`s.
///
/// `Effects` are also [`Scheduler`]s — able to apply modifiers to when (and how often) `Action`s. are sent.
///
/// See [the module level documentation](self) for more.
pub trait Effects: Clone + Scheduler<Action = <Self as Effects>::Action> {
    /// The `Action` type sent by this `Effects`.
    type Action;

    /// An effect that sends an [`Action`][`Self::Action`] through
    /// the `Store`’s [`Reducer`][`crate::Reducer`].
    #[doc(alias = "send")]
    fn action(&self, action: impl Into<<Self as Effects>::Action>);

    /// A [`Task`] represents asynchronous work that will then [`send`][`crate::Store::send`]
    /// zero or more [`Action`][`Self::Action`]s back into the `Store`’s [`Reducer`][`crate::Reducer`]
    /// as it runs.
    ///
    /// Use this method if you need to ability to [`cancel`][Task::cancel] the task
    /// while it is running. Otherwise [`future`][Effects::future] or [`stream`][Effects::stream]
    /// should be preferred.
    fn task<S: Stream<Item = <Self as Effects>::Action> + 'static>(&self, stream: S) -> Task;

    /// An effect that runs a [`Future`][`std::future`] and, if it returns an
    /// [`Action`][`Self::Action`], sends it through the `Store`’s [`Reducer`][`crate::Reducer`].
    #[inline(always)]
    fn future<F: Future<Output = Option<<Self as Effects>::Action>> + 'static>(&self, future: F)
    where
        <Self as Effects>::Action: 'static,
    {
        let stream = once(future).filter_map(|action| async move { action });
        self.task(stream).detach()
    }

    /// An effect that runs a [`Stream`](https://docs.rs/futures/latest/futures/stream/index.html)
    /// and sends every [`Action`][`Self::Action`] it returns through the `Store`’s
    /// [`Reducer`][`crate::Reducer`].
    #[inline(always)]
    fn stream<S: Stream<Item = <Self as Effects>::Action> + 'static>(&self, stream: S) {
        self.task(stream).detach()
    }

    /// Scopes the `Effects` down to one that sends child actions.
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
        <Self as Effects>::Action: From<ChildAction>,
    {
        Scoped(self.clone(), Marker)
    }
}

/// [`Effects`] are also `Scheduler`s — able to apply modifiers to when (and how often) `Action`s. are sent.
pub trait Scheduler {
    /// The `Action` sends scheduled by this `Scheduler`.
    type Action;

    #[doc(hidden)]
    fn now(&self) -> Instant {
        Instant::now()
    }

    #[doc(hidden)]
    fn schedule(
        &self,
        action: Self::Action,
        after: impl IntoIterator<Item = Delay> + 'static,
    ) -> Task
    where
        Self::Action: Clone + 'static;

    /// Sends the `Action` after `duration`.
    fn after(&self, duration: Duration, action: Self::Action) -> Task
    where
        Self::Action: Clone + 'static,
    {
        let instant = self.now() + duration;
        self.at(instant, action)
    }

    /// Sends the `Action` at `instant`.
    fn at(&self, instant: Instant, action: Self::Action) -> Task
    where
        Self::Action: Clone + 'static,
    {
        let mut task = self.schedule(action, [Delay::new(instant)]);
        task.when = Some(instant);
        task
    }

    /// Sends the `Action` every `interval`.
    fn every(&self, interval: Interval, action: Self::Action) -> Task
    where
        Self::Action: Clone + 'static,
    {
        let (mut n, duration) = match interval {
            Interval::Leading(duration) => (0, duration), // 0 × delay => no initial delay
            Interval::Trailing(duration) => (1, duration),
        };

        let start = self.now();
        self.schedule(
            action,
            from_fn(move || {
                let instant = start.checked_add(duration.checked_mul(n)?)?;
                n = n.checked_add(1)?;

                Some(Delay::new(instant))
            }),
        )
    }

    /// An effect that coalesces repeated attempts to send [`Action`][`Effects::Action`]s
    /// through the `Store`’s [`Reducer`][`crate::Reducer`] into a singe send.
    /// Once `timeout` has elapsed with no further `Action`s being attempted,
    /// the last `Action` will be sent.
    ///
    /// The `debounce` function will automatically update the information
    /// stored in `previous` as it runs. The `Task` debounced by this call
    /// will be the _previous_ task for the next call, if any.
    fn debounce(&self, action: Self::Action, previous: &mut Option<Task>, interval: Interval)
    where
        Self::Action: Clone + 'static,
    {
        let task = match interval {
            Interval::Trailing(timeout) => self.after(timeout, action),
            Interval::Leading(timeout) => {
                let now = self.now();
                match previous.as_ref().and_then(|task| task.when).as_ref() {
                    None => self.at(now, action),
                    Some(then) => {
                        if now <= *then + timeout {
                            return; // A leading debounce DROPS subsequent actions within the interval
                        }

                        self.at(now, action)
                    }
                }
            }
        };

        *previous = Some(task);
    }

    /// An effect that sends an [`Action`][`Effects::Action`] through the `Store`’s
    /// [`Reducer`][`crate::Reducer`] if at least one `interval` of time has passed
    /// since `previous` was sent. Otherwise, all subsequent actions but the last
    /// are dropped until that time; which resets the countdown until the next
    /// debounced action can be sent.
    ///
    /// The `throttle` function will automatically update the information
    /// stored in `previous` as it runs. The `Task` throttled by this call
    /// will be the _previous_ task for the next call, if any.
    fn throttle(&self, action: Self::Action, previous: &mut Option<Task>, interval: Interval)
    where
        Self::Action: Clone + 'static,
    {
        let now = self.now();
        let timeout = interval.duration();

        let when = match previous.take().and_then(|task| task.when) {
            Some(when) if when > now => when, // previous was not yet sent — replace it
            Some(when) if when + timeout > now => when + timeout, // previous was sent recently
            _ => match interval {
                Interval::Leading(_) => now,
                Interval::Trailing(_) => now + timeout,
            },
        };

        let task = self.at(when, action);
        *previous = Some(task);
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

impl Interval {
    pub fn duration(&self) -> Duration {
        match self {
            Interval::Leading(duration) => *duration,
            Interval::Trailing(duration) => *duration,
        }
    }
}

/// An `Effects` that scopes its `Action`s to one that sends child actions.
///
/// This `struct` is created by the [`scope`] method on [`Effects`]. See its
/// documentation for more.
///
/// [`scope`]: Effects::scope
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
    <Parent as Effects>::Action: Clone + From<Child> + 'static,
    Child: 'static,
{
    type Action = Child;

    #[inline(always)]
    fn action(&self, action: impl Into<<Self as Effects>::Action>) {
        self.0.action(action.into());
    }

    #[inline(always)]
    fn task<S: Stream<Item = Child> + 'static>(&self, stream: S) -> Task {
        self.0.task(stream.map(|action| action.into()))
    }
}

#[doc(hidden)]
impl<Parent, Child> Scheduler for Scoped<Parent, Child>
where
    Parent: Effects,
    <Parent as Effects>::Action: From<Child> + Clone + 'static,
{
    type Action = Child;

    #[inline(always)]
    fn schedule(
        &self,
        action: Self::Action,
        after: impl IntoIterator<Item = Delay> + 'static,
    ) -> Task
    where
        Self::Action: Clone + 'static,
    {
        self.0.schedule(action.into(), after)
    }
}

#[doc(hidden)]
// `Parent` for `Effects::scope` tuples
impl<Action: 'static> Effects for Weak<RefCell<VecDeque<Action>>> {
    type Action = Action;

    fn action(&self, action: impl Into<Action>) {
        if let Some(actions) = self.upgrade() {
            actions.borrow_mut().push_back(action.into())
        }
    }

    fn task<S: Stream<Item = Action> + 'static>(&self, stream: S) -> Task {
        Task::new(stream)
    }
}

#[doc(hidden)]
impl<Action: 'static> Scheduler for Weak<RefCell<VecDeque<Action>>> {
    type Action = Action;

    fn schedule(&self, action: Action, delays: impl IntoIterator<Item = Delay> + 'static) -> Task
    where
        Action: Clone + 'static,
    {
        self.task(iter(delays).then(move |delay| {
            let action = action.clone();

            async move {
                delay.await;
                action.clone()
            }
        }))
    }
}
