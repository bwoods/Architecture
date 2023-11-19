use std::cell::RefCell;
use std::collections::VecDeque;
use std::marker::PhantomData as Marker;
use std::rc::Rc;

use async_executor::LocalExecutor;
use flume::Sender;
use futures::executor::block_on;
use futures::future::Future;
use futures::future::FutureExt;

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

    fn task<Fut: Future<Output = Option<Self::Action>> + 'static>(&self, future: Fut);
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

    fn send(&self, action: Action) {
        self.0.send(action.into());
    }

    #[inline(always)]
    fn task<Fut: Future<Output = Option<Action>> + 'static>(&self, future: Fut) {
        let future = future.map(|option| option.map(|action| action.into()));
        self.0.task(future)
    }
}

#[doc(hidden)]
// `Parent` tuple for `Effect::scope` tuples
impl<Action> Effects for (Rc<LocalExecutor<'_>>, Rc<RefCell<VecDeque<Action>>>)
where
    Action: 'static,
{
    type Action = Action;

    fn send(&self, action: Action) {
        self.1.borrow_mut().push_back(action);
    }

    #[inline(always)]
    fn task<Fut: Future<Output = Option<Action>> + 'static>(&self, future: Fut) {
        self.0.spawn(future).detach();
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
    fn task<Fut: Future<Output = Option<Action>> + 'static>(&self, future: Fut) {
        let actions = self.clone();
        let future = future.map(move |option| {
            if let Some(action) = option {
                let _ = actions.send(action);
            };
        });

        let executor = LocalExecutor::default();
        block_on(executor.run(future));
    }
}
