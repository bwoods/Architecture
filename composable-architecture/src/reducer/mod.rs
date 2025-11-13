use crate::effects::Effects;
use std::ops::DerefMut;

/// `Reducer`s are responsible for updating a `Store`’s state in response to its `Action`s.
pub trait Reducer {
    /// All of the possible actions that can be used to modify state.
    type Action;

    /// Updates the `Reducer`’s state in response to the action received.
    ///
    /// Additional `Action`s that need to be performed as a side-effect of an `Action` should be
    /// [invoked][`crate::effects::Effects`] on `send`.
    #[doc = include_str!("README.md")]
    fn reduce(&mut self, action: Self::Action, send: impl Effects<Action = Self::Action>);

    #[doc(hidden)]
    #[inline(always)]
    fn recurse(&mut self, _action: Self::Action, _send: impl Effects<Action = Self::Action>) {}
}

impl<T> Reducer for Box<T>
where
    T: Reducer,
{
    type Action = T::Action;

    fn reduce(&mut self, action: Self::Action, send: impl Effects<Action = Self::Action>) {
        self.deref_mut().reduce(action, send)
    }
}

impl<T> Reducer for Option<T>
where
    T: Reducer,
{
    type Action = T::Action;

    fn reduce(&mut self, action: Self::Action, send: impl Effects<Action = Self::Action>) {
        if let Some(state) = self {
            state.reduce(action, send)
        }
    }
}
