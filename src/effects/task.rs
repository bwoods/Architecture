use std::marker::PhantomData as Marker;

use async_executor::Executor;
use flume::{Sender, WeakSender};
use futures::future::Future;
use futures::future::FutureExt;
use once_cell::sync::OnceCell;

use crate::effects::Effects;

fn executor() -> &'static Executor<'static> {
    static INSTANCE: OnceCell<Executor> = OnceCell::new();

    INSTANCE.get_or_init(Executor::new)
}

/// `Task` based `Effects` are run on an async executor
pub struct Task(async_executor::Task<()>);

impl Task {
    pub fn detach(self) {
        self.0.detach();
    }

    pub fn cancel(self) {
        executor().spawn(self.0.cancel()).detach()
    }
}

/// Async support for [`Effects`]
pub trait TaskEffects: Effects {
    fn task<Fut: Future<Output = Option<Self::Action>> + Send + 'static>(
        &self,
        future: Fut,
    ) -> Task;
}

#[doc(hidden)]
// Nested tuples are used by the [`scope`] function
impl<Action, Parent> TaskEffects for (Parent, Marker<Action>)
where
    Parent: TaskEffects,
    <Parent as Effects>::Action: From<Action>,
{
    #[inline(always)]
    fn task<Fut: Future<Output = Option<Self::Action>> + Send + 'static>(
        &self,
        future: Fut,
    ) -> Task {
        let future = future.map(|option| option.map(|action| action.into()));
        self.0.task(future)
    }
}

#[doc(hidden)]
// `Parent` for `Effect::scope` tuples
//   - holds both the (internal) effect’s and (external) action’s `Sender`s
impl<Action, Parent> TaskEffects for (Parent, WeakSender<Action>)
where
    Parent: TaskEffects,
    <Parent as Effects>::Action: From<Action> + Send,
    Action: Send + 'static,
{
    #[inline]
    fn task<Fut: Future<Output = Option<Self::Action>> + Send + 'static>(
        &self,
        future: Fut,
    ) -> Task {
        if let Some(actions) = self.1.upgrade() {
            actions.task(future)
        } else {
            #[cold]
            fn no_op<T: TaskEffects>(parent: &T) -> Task {
                parent.task(async { None })
            }

            // The `Store` is shutting down, but we need to return a Task…
            no_op(&self.0)
        }
    }
}

#[doc(hidden)]
// `Parent` for `Store::scope` tuples
//   - refers to the (external) action’s `Sender`
impl<Action> TaskEffects for Sender<Action>
where
    Self::Action: Send + 'static,
{
    #[inline]
    fn task<Fut: Future<Output = Option<Self::Action>> + Send + 'static>(
        &self,
        future: Fut,
    ) -> Task {
        let actions = self.clone();
        let future = future.map(move |option| {
            if let Some(action) = option {
                let _ = actions.send(action);
            };
        });

        Task(executor().spawn(future))
    }
}
