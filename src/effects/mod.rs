#![doc = include_str!("README.md")]

use std::cell::RefCell;
use std::collections::VecDeque;
use std::marker::PhantomData as Marker;
use std::rc::Weak;
use std::thread::Thread;
use std::time::{Duration, Instant};

use flume::WeakSender;
use futures::{future, stream::once, Future, FutureExt, Stream, StreamExt};
use futures_timer::Delay;

pub(crate) use crate::effects::task::Executor;
#[doc(hidden)]
pub use crate::effects::task::Task;

mod task;

/// `Effects` are used within `Reducer`s to propagate `Action`s as side-effects of performing other `Action`s.
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

    /// An effect that sends an [`Action`][`Self::Action`] through the `Store`’s
    /// [`Reducer`][`crate::Reducer`] if at least `delay` time has passed since
    /// `previous` was sent. Otherwise, all subsequent actions but the last are
    /// dropped until that time; which resets the countdown until the next
    /// debounced action can be sent.
    ///
    /// The `debounce` function will automatically update the information
    /// stored in `previous` as it runs. The `Task` configured for this call
    /// will be the _previous_ task for the next call, after all.
    fn debounce(
        &self,
        action: impl Into<Self::Action> + 'static,
        previous: &mut Option<Task>,
        delay: Duration,
    ) where
        Self::Action: 'static,
    {
        let now = Instant::now();
        let when = match previous.take().and_then(|task| task.when) {
            Some(when) if when + delay > now => when + delay, // previous was sent recently; delay this send
            Some(when) if when > now => when, // previous was not yet sent — replace it
            _ => now, // goes through the same code path as delayed events for CONSISTENT performance
        };

        let task = Task {
            handle: self
                .task(once(async move {
                    // TODO: this will have to be restructured once we are simulating time for tests
                    // (note that futures_timer::native::timer::Timer has everything that is needed)
                    Delay::new(when - now).await;
                    action.into()
                }))
                .handle,
            when: Some(when),
        };

        *previous = Some(task);
    }

    /// An effect that runs a [`Future`][`std::future`] and, if it returns an
    /// [`Action`][`Self::Action`], sends it through the `Store`’s [`Reducer`][`crate::Reducer`].
    #[inline]
    fn future<F: Future<Output = Option<Self::Action>> + 'static>(&self, future: F)
    where
        Self::Action: 'static,
    {
        let stream = future
            .into_stream()
            .map(future::ready)
            .filter_map(|option| option);
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

#[doc(hidden)]
// `Parent` for `StoreOf::scope` tuples
impl<Action: 'static> Effects for WeakSender<Result<Action, Thread>> {
    type Action = Action;

    #[inline(always)]
    fn send(&self, action: impl Into<Self::Action>) {
        self.upgrade()
            .and_then(|sender| sender.send(Ok(action.into())).ok());
    }

    fn task<S: Stream<Item = Action> + 'static>(&self, stream: S) -> Task {
        Task::new(stream)
    }
}
