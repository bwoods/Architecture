use futures::future::Future;

use crate::effects::Effects;

#[doc = include_str!("README.md")]
pub trait Reducer {
    /// …
    type Action;

    #[allow(unused_variables)]
    /// Updates the `Reducer`’s state in response to the action received.
    ///
    /// Additional `Action`s that need to be performed as a side-effect of an `Action` should be
    /// [invoked][`crate::Effects`] on `effects`.
    fn reduce(&mut self, action: Self::Action, effects: impl Effects<Action = Self::Action>) {}

    /// Updates the `Reducer`’s state in response to the action received.
    /// - Implement `reduce_async` for `Reducer`s that need to call `async` functions.
    /// - Only one of [`reduce`][`Reducer::reduce`] and [`reduce_async`][`Reducer::reduce_async`]
    ///   should be implemented for a given type.
    ///
    /// Additional `Action`s that need to be performed as a side-effect of an `Action` should be
    /// [invoked][`crate::Effects`] on `effects`.
    fn reduce_async(
        &mut self,
        action: Self::Action,
        effects: impl Effects<Action = Self::Action>,
    ) -> impl Future<Output = ()> {
        async { self.reduce(action, effects) }
    }

    /// …
    type Output;
    /// …
    fn into_inner(self) -> Self::Output;
}
