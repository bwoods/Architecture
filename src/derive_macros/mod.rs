#![doc = include_str!("README.md")]

#[doc(no_inline)]
pub use derive_more::{From, TryInto};

/// Macros used to ease creation of recursive reducers.
///
/// `Reducer`s that are primarily made up of other `Reducer`s can have much of their behavior
/// written by the compiler. Reducing both the amount of work required and the number of bugs
/// that may creep into a complex application.
///
/// Given two preexisting `Reducer`s, `A` and `B`,
///
/// ```rust
/// # use composable::*;
/// struct A;
/// struct B;
///
/// #[derive(Clone)]
/// enum Action { /* … */ }
///
/// impl Reducer for A {
///     type Action = Action;
///     type Output = ();
///     
///     fn into_inner(self) -> Self::Output {}
///     fn reduce(&mut self, action: Self::Action, effects: impl Effects<Action>) {
///         match action { /* … */ }
///     }
/// }
///
/// impl Reducer for B {
///     type Action = Action;
///     type Output = ();
///     
///     fn into_inner(self) -> Self::Output {}
///     fn reduce(&mut self, action: Self::Action, effects: impl Effects<Action>) {
///         match action { /* … */ }
///     }
/// }
/// ```
///
/// ## `enum` Example
///
/// A `RecursiveReducer` `enum` can be used to model `Reducer` alternatives.
///
/// ```rust
/// # struct A;
/// # struct B;
/// #
/// # use composable::*;
/// #
/// # #[derive(Clone)]
/// # enum Action { /* … */ }
/// #
/// # impl Reducer for A {
/// #     type Action = Action;
/// #     type Output = ();
/// #     
/// #     fn into_inner(self) -> Self::Output {}
/// #     fn reduce(&mut self, action: Self::Action, effects: impl Effects<Action>) {
///         match action { /* … */ }
/// #     }
/// # }
/// #
/// # impl Reducer for B {
/// #     type Action = Action;
/// #     type Output = ();
/// #     
/// #     fn into_inner(self) -> Self::Output {}
/// #     fn reduce(&mut self, action: Self::Action, effects: impl Effects<Action>) {
///         match action { /* … */ }
/// #     }
/// # }
/// #
/// #[derive(RecursiveReducer)]
/// enum Either {
///     A(A),
///     B(B),
/// }
///
/// impl RecursiveReducer for Either {
///     type Action = Action;
///
///     fn reduce(&mut self, action: Self::Action, effects: impl Effects<Self::Action>) {
///         // …
///     }
/// }
///
/// let store = Store::with_initial(Either::A(A));
/// ```
///
/// ## `struct` Example
///
/// A `RecursiveReducer` `struct` represents a parent-child relationship between `Reducer`s.
/// This is the most common use of `RecursiveReducer` in large applications. It forms the core
/// “Compatibility” of the Composbale Architecture.
///
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
