use flume::Sender;

pub trait Effects: Clone {
    type Action;

    fn send(&self, action: Self::Action);

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
    /// Initializes an effect that immediately emits the action passed in.
    fn send(&self, action: Action) {
        let _ = self.send(action);
    }
}

use std::marker::PhantomData as Marker;

#[doc(hidden)]
/// Nested tuples are used by the [`scope`] function
impl<Action, Parent> Effects for (Parent, Marker<Action>)
where
    Parent: Effects,
    <Parent as Effects>::Action: From<Action>,
{
    type Action = Action;

    fn send(&self, action: Action) {
        self.0.send(action.into());
    }
}
