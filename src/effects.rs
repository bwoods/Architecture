// Nested tuples are used by the [`scope`] function
use std::marker::PhantomData as Marker;

use flume::Sender;

pub trait Effects: Clone {
    type Action;

    fn send(&self, action: impl Into<Self::Action>);

    #[inline(always)]
    fn scope<ChildAction>(&self) -> impl Effects<Action = ChildAction>
    where
        <Self as Effects>::Action: From<ChildAction>,
    {
        (self.clone(), Default::default())
    }
}

#[doc(hidden)]
impl<Action> Effects for Sender<Action> {
    type Action = Action;

    #[inline(always)]
    fn send(&self, action: impl Into<Action>) {
        let _ = self.send(action.into());
    }
}

#[doc(hidden)]
impl<Action, Parent> Effects for (Parent, Marker<Action>)
where
    Parent: Effects,
    <Parent as Effects>::Action: From<Action>,
{
    type Action = Action;

    fn send(&self, action: impl Into<Self::Action>) {
        let action: Action = action.into();
        self.0.send(Parent::Action::from(action));
    }
}
