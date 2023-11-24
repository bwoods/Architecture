use std::cell::RefCell;
use std::collections::VecDeque;
use std::marker::PhantomData as Marker;
use std::rc::Rc;
use std::thread::Thread;

use flume::WeakSender;
use futures::{future, Future, FutureExt, Stream, StreamExt};
use futures::executor::LocalSpawner;
use futures::future::RemoteHandle;
use futures::task::LocalSpawnExt;

use crate::dependencies::Dependency;

#[doc = include_str!("README.md")]
pub trait Effects: Clone {
    type Action;

    /// An effect that immediately sends an [`Action`][`Self::Action`] through the `Store`’s
    /// [`Reducer`][`crate::Reducer`].
    fn send(&self, action: Self::Action);

    // #[inline(always)]
    // /// Reduces the `Effects` to one that sends child actions.
    // fn scope<ChildAction>(&self) -> impl Effects<Action = ChildAction>
    // where
    //     Self::Action: From<ChildAction>,
    // {
    //     (self.clone(), Marker)
    // }

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
}

#[doc(hidden)]
// Nested tuples are used by `Effects::scope`
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
// `Parent` for `Effects::scope` tuples
impl<Action: 'static> Effects for Rc<RefCell<VecDeque<Action>>> {
    type Action = Action;

    #[inline(always)]
    fn send(&self, action: Action) {
        self.borrow_mut().push_back(action);
    }

    fn task<S: Stream<Item = Action> + 'static>(&self, stream: S) -> Task {
        let handle = Dependency::<Executor<Result<Action, Thread>>>::new() //
            .and_then(|executor| {
                match executor.actions.upgrade() {
                    None => None,
                    Some(sender) => {
                        let stream =
                            stream.then(move |action| sender.clone().into_send_async(Ok(action)));
                        let future = stream.for_each(|_| future::ready(())); // discard `send`s return value

                        executor.spawner.spawn_local_with_handle(future).ok()
                    }
                }
            });

        Task(handle) // may return a `Task(None)` while the `Store` is shutting down
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
    /// Detaches the task; leaving its [`Future`][`std::future`] running in the background.
    pub fn detach(self) {
        if let Some(handle) = self.0 {
            handle.forget()
        }
    }

    /// Cancels the task; meaning its [`Future`][`std::future`] won’t be polled again.
    pub fn cancel(self) {
        drop(self)
    }
}

pub(crate) struct Executor<Action> {
    pub(crate) spawner: LocalSpawner,
    pub(crate) actions: WeakSender<Action>,
}

impl<Action> Executor<Action> {
    pub(crate) fn new(spawner: LocalSpawner, actions: WeakSender<Action>) -> Self {
        Self { spawner, actions }
    }
}
