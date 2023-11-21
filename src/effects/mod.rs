use std::cell::RefCell;
use std::collections::VecDeque;
use std::marker::PhantomData as Marker;
use std::rc::Rc;

use flume::WeakSender;
use futures::executor::{LocalPool, LocalSpawner};
use futures::future::RemoteHandle;
use futures::task::LocalSpawnExt;
use futures::{future, Future, FutureExt, Stream, StreamExt};

pub trait Effects: Clone {
    type Action;

    /// An effect that immediately sends an action through the `Store`.
    fn send(&self, action: Self::Action);

    #[inline(always)]
    fn scope<ChildAction>(&self) -> impl Effects<Action = ChildAction>
    where
        Self::Action: From<ChildAction>,
    {
        (self.clone(), Marker)
    }

    /// A [`Task`] represents asynchronous work that will then [`send`][`Store::send`] zero or more
    /// actions back into the [`Store`] as it runs.
    ///
    /// Use this method if yuu need to ability to [`cancel`][Task::cancel] the [`Task`]
    /// while it is running. Otherwise [`future`][Effects::future] or [`stream`][Effects::stream]
    /// should be preferred.
    fn task<S: Stream<Item = Self::Action> + 'static>(&self, stream: S) -> Task;

    /// An effect that runs a [`future`][`std::future`] and, if it returns an `Action`,
    /// sends it through the `Store`.
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

    /// An effect that runs a [`stream`](https://docs.rs/futures/latest/futures/stream/index.html)
    /// and sends every `Action` it returns through the `Store`.
    fn stream<S: Stream<Item = Self::Action> + 'static>(&self, stream: S) {
        self.task(stream).detach()
    }
}

#[doc(hidden)]
// Nested tuples are used by the [`scope`] function
impl<Action, Parent> Effects for (Parent, Marker<Action>)
where
    Parent: Effects,
    <Parent as Effects>::Action: From<Action>,
{
    type Action = Action;

    #[inline(always)]
    fn send(&self, action: Action) {
        self.0.send(action.into());
    }

    #[inline(always)]
    fn task<S: Stream<Item = Action> + 'static>(&self, stream: S) -> Task {
        self.0.task(stream.map(|action| action.into()))
    }
}

#[doc(hidden)]
// `Parent` for `Effect::scope` tuples
impl<Action: 'static> Effects for Rc<RefCell<VecDeque<Action>>> {
    type Action = Action;

    #[inline(always)]
    fn send(&self, action: Action) {
        self.borrow_mut().push_back(action);
    }

    fn task<S: Stream<Item = Action> + 'static>(&self, stream: S) -> Task {
        let executor = ambience::thread::get::<Executor<Action>>().unwrap();

        match executor.actions.upgrade() {
            None => Task(None),
            Some(sender) => {
                let stream = stream.then(move |action| sender.clone().into_send_async(action));
                let future = stream.for_each(|_| future::ready(())); // discard `send`s return value

                // may return a `Task(None)` while the `Store` is shutting down
                Task(executor.spawner.spawn_local_with_handle(future).ok())
            }
        }
    }
}

/// Asynchronous work being performed by a [`Store`][`crate::Store`].
///
/// A `Store` uses a [Local Async Executor][Why] to run its `Task`s.
///
/// [Why]: https://maciej.codes/2022-06-09-local-async.html
#[must_use = "dropping a Task cancels the underlying future"]
pub struct Task(Option<RemoteHandle<()>>);

impl Task {
    /// Detaches the task; leaving its [`future`][`std::future`] running in the background.
    pub fn detach(self) {
        if let Some(handle) = self.0 {
            handle.forget()
        }
    }

    /// Cancels the task; meaning its [`future`][`std::future`] won’t be polled again.
    pub fn cancel(self) {
        drop(self)
    }
}

pub(crate) struct Executor<Action> {
    pub(crate) spawner: LocalSpawner,
    pub(crate) actions: WeakSender<Action>,
}

impl<Action> Executor<Action> {
    pub(crate) fn new(executor: &LocalPool, actions: WeakSender<Action>) -> Self {
        Self {
            spawner: executor.spawner(),
            actions,
        }
    }
}
