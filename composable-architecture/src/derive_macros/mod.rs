//! Derive macros used to ease the creation of recursive reducers.
//!
//! - [`RecursiveReducer`]  
//!   `#[derive(RecursiveReducer)]` on a `enum` or `struct` that contains other [`Reducer`]
//!   types will derive a [`Reducer`] implementation for it.
//! - [`TryInto`]  
//!   `#[derive(TryInto)]` on a `Action` whose variants contain another [`Reducer`]’s `Action`s
//!   allows an attempted conversion to…
//! - [`From`]  
//!   `#[derive(TryInto)]` on a `Action` whose variants contain another [`Reducer`]’s `Action`s
//!   allows an attempted conversion from…
//!
//! These macros produce efficient implementations of the [`Reducer`], [`std::convert::TryInto`]
//! and [`std::convert::From`] traits so that they do not have to be implemented manually.
//!
//! ##### Automatic Derived Reducers
//!
//! Two other types are valid [`Reducer`]s whenever they contain a [`Reducer`].
//! - [`Option`]  
//! - [`Box`]  
//!
//! These do not require the [`RecursiveReducer`] and [automatically apply][auto].
//!
//! [auto]: crate::Reducer#foreign-impls
//! [`RecursiveReducer`]: derive_reducers::RecursiveReducer
//! [`Reducer`]: crate::Reducer
//! [`TryInto`]: #reexports
//! [`From`]: #reexports
//!
//! # Composite Reducers
//!
//! A `RecursiveReducer` **`struct`** represents a parent-child relationship between `Reducer`s.
//! This is the most common use of `RecursiveReducer` in large applications and forms the core
//! “Composability” of the Composable Architecture.
//!
//! The application is broken up into different `mod`ules representing the different Domains or
//! Features of the application; each with its own `Action`s and `State`.
//!
//! These `Reducer`s are then collected into various composite `Reducer`s that contain and
//! coordinate between them.  
//!
//! Each composite `Reducer` is written with the knowledge of its own `Action`s and the `Action`s
//! of its immediate children. The `Action`s of its parent are unknown to it and (by convention) it
//! does not traffic in the `Action`s of its grandchildren.
//!
//! Deciding which Domains need to be coordinated between, and thus should be siblings under a
//! parent Domains, is the art of designing the application with an architecture like this one.
//!
//! Even though the application `struct` recursively contains the `State`s of all of its Features
//! it usually does not end up being very “tall.”
//!
//! See [Unidirectional Event Architecture][crate] for more.
//!
//! ```rust
//! mod A {
//! #   use composable::*;
//!     #[derive(Default)]
//!     pub struct State { /* … */ }
//!
//!     #[derive(Clone)] // ⒈
//!     pub enum Action { /* … */ }
//!
//!     impl Reducer for State {
//!         type Action = Action;
//!         type Output = Self;
//!         
//!         fn reduce(&mut self, action: Action, send: impl Effects<Action>) {
//!             match action { /* … */ }
//!         }
//!     }
//! }
//!
//! mod B {
//! #   use composable::*;
//!     #[derive(Default)]
//!     pub struct State;
//!
//!     #[derive(Clone)] // ⒈
//!     pub enum Action { /* … */ }
//!
//!     impl Reducer for State {
//!         type Action = Action;
//!         type Output = Self;
//!
//!         fn reduce(&mut self, action: Action, send: impl Effects<Action>) {
//!             match action { /* … */ }
//!         }
//!     }
//! }
//!
//! # use composable::*;
//! #[derive(Default, RecursiveReducer)] // ⒉
//! struct State {
//!     a: A::State,
//!     b: B::State,
//! #
//! #   #[reducer(skip)]
//! #   c: Vec<u32>,
//! }
//!
//! #[derive(Clone, From, TryInto)] // ⒊
//! enum Action {
//!     SomeAction, // parent actions
//!     SomeOtherAction,
//!
//!     A(A::Action), // ⒋
//!     B(B::Action),
//! }
//!
//! impl RecursiveReducer for State { // ⒌
//!     type Action = Action;
//!
//!     fn reduce(&mut self, action: Action, send: impl Effects<Action>) {
//!         match action {
//!             Action::SomeAction => { /* … */ }
//!             Action::SomeOtherAction => { /* … */ }
//!
//!             // in this example, the parent reducer has
//!             // no explicit handling of any child actions
//!             _ => {}
//!         }
//!     }
//! }
//!
//! # let store = Store::with_initial(State::default());
//! ```
//! 1. Now that `Action`s are being passed to multiple `Reducers` they must be `Clone`.
//! 2. The `RecursiveReducer` derive macro constructs a recursive `Reducer` from the `struct`.
//! 3. The `From` and `TryInfo` derive macros ensure that conversions work, when they should,
//!    between parent and child `Action`s. These conversions utilize #4…
//! 4. The parent has one (and only one) `Action` for the `Action`s of each of its children.
//! 5. Finally, an implementation of the `RecursiveReducer` trait containing the parent’s `reduce`
//!    method. `RecursiveReducer::reduce` is run before the `Reducer::reduce` methods of its
//!    fields. Resulting in:
//!
//!    - `self.reduce()`, then
//!    - `self.a.reduce()`, then
//!    - `self.b.reduce()`.
//!
//! ### Ignoring fields
//!
//! Compound `Reducer`s often contain fields other than the child `Reducer`s. After all, it has
//! its own `Reducer` and that `Reducer` may need its own state.
//!
//! The `RecursiveReducer` macro comes with an associated attribute that allows it to skip
//! `struct` members that should not ne made part of the `Reducer` recursion.
//!
//! ```ignore
//! #[derive(RecursiveReducer)]
//! struct State {
//!     a: A::State,
//!     b: B::State,
//!
//!     #[reducer(skip)]
//!     c: Vec<u32>,
//! }
//! ```
//!
//! # Alternate Reducers
//!
//! A `RecursiveReducer` **`enum`** represents a single state that is best
//! represented by an enumeration a separate reducers.
//!
//! **Alternate `Reducer`s** are less common than **Composite `Reducer`s** so a more concrete example may
//! help…
//!
//! ```
//! # mod authenticated {
//! #    #[derive(Clone)]
//! #    pub enum Action {}
//! #    pub struct State {}
//! #
//! #    use composable::*;
//! #    impl Reducer for State {
//! #        type Action = Action;
//! #        type Output = Self;
//! #        fn reduce(&mut self, action: Action, send: impl Effects<Action>) {}
//! #    }
//! # }
//! #
//! # mod unauthenticated {
//! #    #[derive(Clone)]
//! #    pub enum Action {}
//! #    pub struct State {}
//! #
//! #    use composable::*;
//! #    impl Reducer for State {
//! #        type Action = Action;
//! #        type Output = Self;
//! #        fn reduce(&mut self, action: Action, send: impl Effects<Action>) {}
//! #    }
//! # }
//! # use composable::*;
//! #[derive(RecursiveReducer)]
//! enum State {
//!     LoggedIn(authenticated::State),
//!     LoggedOut(unauthenticated::State),
//! #
//! #   #[reducer(skip)]
//! #   Other,
//! }
//!
//! #[derive(Clone, From, TryInto)]
//! enum Action {
//!     LoggedIn(authenticated::Action),
//!     LoggedOut(unauthenticated::Action),
//! }
//!
//! impl RecursiveReducer for State {
//!     type Action = Action;
//!
//!     fn reduce(&mut self, action: Action, send: impl Effects<Action>) {
//!         // logic independent of the user’s authentication
//!     }
//! }
//! ```
//!
//! `authenticated::Action`s will only run when the state is `LoggedIn` and vice-versa..
//!
//! ---
//! <br />
//!
//! Now, the [automatic derive reducer] behavior of [`Option`] is easy to described.
//! It behaves is as if it were:
//!
//! ```ignore
//! #[derive(RecursiveReducer)]
//! enum Option<T: Reducer> {
//!     #[reducer(skip)]
//!     None,
//!     Some(T),
//! }
//! ```
//! Although, currently, the `RecursiveReducer` macro does not work with generic parameters on the
//! type it is attempting to derive the `Reducer` trait for.
//!
//! [automatic derive reducer]: #automatic-derived-reducers

#[doc(no_inline)]
pub use derive_more::{From, TryInto};

pub use derive_reducers::RecursiveReducer;

use crate::Effects;

/// See the [`RecursiveReducer`][`derive_reducers::RecursiveReducer`] macro for example usage.
pub trait RecursiveReducer {
    /// All of the possible actions that can be used to modify state.
    /// Equivalent to [`Reducer::Action`][`crate::Reducer::Action`].
    type Action;

    /// This `reduce` should perform any actions that are needed _before_ the macro recurses
    /// into the other reducers.
    fn reduce(&mut self, action: Self::Action, send: impl Effects<Self::Action>);
}
