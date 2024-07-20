use std::time::Duration;

/// When testing, it is important to have control over the (simulated) passage of time.
///
/// …
/// ```rust
/// # use std::time::Duration;
/// # use composable::*;
/// #
/// #[derive(Debug, Default)]
///  struct State {
///      previous: Option<Task>,
///      n: usize,
///  }
///
/// # impl PartialEq for State {
/// #     fn eq(&self, other: &Self) -> bool {
/// #         self.n.eq(&other.n)
/// #     }
/// # }
/// #
/// # impl Clone for State {
/// #     fn clone(&self) -> Self {
/// #         Self {
/// #             previous: None,
/// #             n: self.n
/// #         }
/// #     }
/// # }
/// #[derive(Clone, Debug, PartialEq)]
/// enum Action {
///     Send,
///     Recv,
/// }
///
/// use Action::*;
///
/// impl Reducer for State {
///     type Action = Action;
///     type Output = Self;
///
///     fn reduce(&mut self, action: Action, effects: impl Effects<Action>) {
///         match action {
///             Send => {
///                 effects.debounce(
///                     Recv,
///                     &mut self.previous,
///                     Interval::Trailing(Duration::from_secs(4)),
///                 );
///             }
///             Recv => {
///                 self.n += 1;
///             }
///         }
///     }
/// }
///
/// let mut store = TestStore::<State>::default();
/// let no_change: fn(&mut State) = |State| {};
///
/// store.send(Send, no_change);
/// store.advance(Duration::from_secs(3));
///
/// store.send(Send, no_change);
/// store.advance(Duration::from_secs(8));
/// store.recv(Recv, |state| state.n = 1);
///
/// store.send(Send, no_change);
/// store.advance(Duration::from_secs(1));
/// store.advance(Duration::from_secs(1));
/// store.advance(Duration::from_secs(1));
/// store.advance(Duration::from_secs(1));
/// store.recv(Recv, |state| state.n = 2);
/// ```
pub trait TestClock {
    fn advance(&mut self, duration: Duration);
}
