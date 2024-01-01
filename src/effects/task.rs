use std::thread::Thread;

use flume::WeakSender;
use futures::executor::LocalSpawner;
use futures::future::RemoteHandle;
use futures::task::LocalSpawnExt;
use futures::{future, Stream, StreamExt};

use crate::dependencies::Dependency;

/// Asynchronous work being performed by a `Store`.
///
/// A [`Store`][`crate::Store`] uses a [Local Async Executor] to run its `Task`s.
///
/// [Local Async Executor]: https://maciej.codes/2022-06-09-local-async.html
#[doc(hidden)]
#[must_use = "dropping a Task cancels the underlying future"]
pub struct Task {
    pub(crate) handle: Option<RemoteHandle<()>>,
    pub(crate) when: Option<std::time::Instant>,
}

impl Task {
    /// Detaches the task; leaving its [`Future`][`std::future`] running in the background.
    pub fn detach(self) {
        if let Some(handle) = self.handle {
            handle.forget()
        }
    }

    /// Cancels the task; meaning its [`Future`][`std::future`] won’t be polled again.
    pub fn cancel(self) {
        drop(self)
    }

    pub(crate) fn new<Action: 'static, S: Stream<Item = Action> + 'static>(stream: S) -> Self {
        // Only called by “root” `Effects`, so it will be the same `Action` as used by the `Store`
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

        Task {
            handle, // may return a `Task { handle: None }` while the `Store` is shutting down
            when: None,
        }
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
