#![doc = include_str!("README.md")]

#[doc(no_inline)]
pub use derive_more::{From, TryInto};
pub use derive_reducers::RecursiveReducer;

#[cfg(feature = "ouroboros")]
pub use derive_reducers::SelfReferentialRecursiveReducer;

use crate::Effects;

/// See the [`RecursiveReducer`][`derive_reducers::RecursiveReducer`] macro for example usage.
pub trait RecursiveReducer {
    /// All of the possible actions that can be used to modify state.
    /// Equivalent to [`Reducer::Action`][`crate::Reducer::Action`].
    type Action;

    /// This `reduce` should perform any actions that are needed _before_ the macro recurses
    /// into the other reducers.
    fn reduce(&mut self, action: Self::Action, effects: impl Effects<Self::Action>);
}
