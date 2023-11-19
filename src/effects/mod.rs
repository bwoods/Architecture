use std::cell::RefCell;
use std::collections::VecDeque;
use std::marker::PhantomData as Marker;
use std::rc::Rc;

use async_executor::LocalExecutor;
use flume::Sender;
use futures::{future, Stream, StreamExt};

pub trait Effects: Clone {
    type Action;

    fn send(&self, action: Self::Action);

    #[inline(always)]
    fn scope<ChildAction>(&self) -> impl Effects<Action = ChildAction>
    where
        <Self as Effects>::Action: From<ChildAction> + 'static,
        ChildAction: 'static,
    {
        (self.clone(), Marker)
    }

    fn task<S: Stream<Item = Self::Action> + 'static>(&self, stream: S) -> Task;
}

#[doc(hidden)]
// Nested tuples are used by the [`scope`] function
impl<Action, Parent> Effects for (Parent, Marker<Action>)
where
    Action: 'static,
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
// `Parent` tuple for `Effect::scope` tuples
impl<Action> Effects for Rc<(LocalExecutor<'_>, RefCell<VecDeque<Action>>)>
where
    Action: 'static,
{
    type Action = Action;

    #[inline(always)]
    fn send(&self, action: Action) {
        self.1.borrow_mut().push_back(action);
    }

    #[inline(always)]
    fn task<S: Stream<Item = Action> + 'static>(&self, stream: S) -> Task {
        let actions = self.clone();

        Task(self.0.spawn(stream.for_each(move |action| {
            actions.1.borrow_mut().push_back(action);
            future::ready(()) // https://docs.rs/futures/0.3.29/futures/stream/trait.StreamExt.html#method.for_each
        })))
    }
}

#[doc(hidden)]
// `Parent` for `TestStore` effects
impl<Action> Effects for Sender<Action>
where
    Action: 'static,
{
    type Action = Action;

    #[inline(always)]
    fn send(&self, action: Action) {
        let _ = self.send(action);
    }

    #[inline(always)]
    fn task<S: Stream<Item = Action> + 'static>(&self, stream: S) -> Task {
        let executor = LocalExecutor::default();
        let actions = self.clone();

        let task = executor.spawn(stream.for_each(move |action| {
            let _ = actions.send(action);
            future::ready(())
        }));

        while !executor.is_empty() {
            executor.try_tick(); // TestStore runs it’s executor to completion
        }

        Task(task)
    }
}

/// `Task` based `Effects` are run on an local async executor
#[must_use = "Dropping a Task cancels it, which means its stream won't continue running"]
pub struct Task(async_executor::Task<()>);

impl Task {
    pub fn detach(self) {
        self.0.detach();
    }

    pub fn cancel(self) {
        // dropping an async_executor::Task cancels it
    }
}
