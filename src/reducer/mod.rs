use crate::Effects;

#[doc = include_str!("README.md")]
pub trait Reducer {
    /// All of the possible actions that can be used to modify state.
    type Action;

    /// Returned by [`into_inner`][`Reducer::into_inner`].
    type Output;

    /// Tests and command line applications often need the final state of their `Store`’s `Reducer`.
    /// The `Store`’s [`into_inner`] method uses this method to retrieve that value.
    /// - `Reducer` states that are `Send` can just return `self`.
    /// - `Reducer` states that are not will need to extract a type that _is_ `Send` and return that.
    /// - Most `Reducer`s never need to return their final state and should just return ‘nothing’:
    /// ```rust
    /// # struct State();
    /// # impl composable::Reducer for State {
    /// # type Action = ();
    /// type Output = ();
    ///
    /// fn into_inner(self) -> Self::Output { }
    /// # }
    /// ```
    ///
    /// [`into_inner`]: crate::Store::into_inner
    fn into_inner(self) -> Self::Output;

    #[allow(unused_variables)]
    /// Updates the `Reducer`’s state in response to the action received.
    ///
    /// Additional `Action`s that need to be performed as a side-effect of an `Action` should be
    /// [invoked][`crate::Effects`] on `effects`.
    fn reduce(&mut self, action: Self::Action, effects: impl Effects<Action = Self::Action>) {}
}

impl<T: Reducer> Reducer for Option<T> {
    type Action = T::Action;

    type Output = Option<T::Output>;

    fn into_inner(self) -> Self::Output {
        self.map(|state| state.into_inner())
    }

    fn reduce(&mut self, action: Self::Action, effects: impl Effects<Action = Self::Action>) {
        if let Some(state) = self {
            state.reduce(action, effects)
        }
    }
}
