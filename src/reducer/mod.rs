use crate::Effects;

/// `Reducer`s are responsible for updating a `Store`’s state in response to its `Action`s.
pub trait Reducer {
    /// All of the possible actions that can be used to modify state.
    type Action;

    /// Both unit tests and command line applications often need to return their `Store`’s final
    /// state. Therefore `Store`’s [`into_inner`] method shuts down the `Store` and converts its
    /// `Reducer` into its `Output` type.
    ///
    /// Using a separate `Output` type, rather than returning the `Reducer` itself, allows the
    /// `Store`s to support `Reducer` types that are not [`Send`].
    ///  
    /// - `Reducer`s that do not need to support [`into_inner`] should use declare
    ///   `type Output = Self;` as it is a simple, recognizable default.
    /// - A `Reducer` that _is_ `Send` can also default to `type Output = Self;`.
    /// - Otherwise, the `Reducer` will need to declare an `Output` type that _is_ `Send` and
    ///   that can be crated [`From`] the `Reducer`’s state.
    ///
    /// ```rust
    /// # use std::rc::Rc;
    /// # use std::cell::Cell;
    /// # use composable::{Effects, Store};
    /// # #[derive(Default)]
    /// struct State {
    ///     n: Rc<Cell<usize>>, // Rc<Cell<…>> is not Send
    /// };
    ///
    /// enum Action { /**/ }
    ///
    /// impl composable::Reducer for State {
    ///     type Action = Action;
    ///     type Output = usize; // but the usize itself _is_
    ///
    ///     fn reduce(&mut self, action: Self::Action, effects: impl Effects<Self::Action>) { /**/ }
    /// }
    ///
    /// impl std::convert::From<State> for usize {
    ///     fn from(value: State) -> Self {
    ///         Cell::into_inner(Rc::into_inner(value.n).unwrap_or_default())
    ///     }
    /// }
    /// # let store = Store::<State>::default();
    /// ```
    ///
    /// [`into_inner`]: crate::Store::into_inner
    ///
    /// In short, you can use `type Output = Self;` until the compiler says that you can’t.
    type Output;

    #[allow(unused_variables)]
    /// Updates the `Reducer`’s state in response to the action received.
    ///
    /// Additional `Action`s that need to be performed as a side-effect of an `Action` should be
    /// [invoked][`crate::effects::Effects`] on `effects`.
    #[doc = include_str!("README.md")]
    fn reduce(&mut self, action: Self::Action, effects: impl Effects<Self::Action>);
}

impl<T: Reducer> Reducer for Option<T> {
    type Action = T::Action;

    type Output = Option<T::Output>;

    fn reduce(&mut self, action: Self::Action, effects: impl Effects<Self::Action>) {
        if let Some(state) = self {
            state.reduce(action, effects)
        }
    }
}

impl<T: Reducer> Reducer for Box<T> {
    type Action = T::Action;

    type Output = T::Output;

    fn reduce(&mut self, action: Self::Action, effects: impl Effects<Self::Action>) {
        use std::ops::DerefMut;
        self.deref_mut().reduce(action, effects)
    }
}
