use composable_dependencies::DependencyDefault;
use std::cell::Cell;

mod recognizer;
pub use recognizer::*;
/// Alias for `std::num::NonZeroU128`
///
pub use std::num::NonZeroU128 as Id;

mod tap;

pub use tap::{TapGesture, Target};

#[non_exhaustive] // must use `State::default()`
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Values {
    pub active: Option<Id>,
    pub hover: Option<Id>,
    pub focus: Option<Id>,
}

impl DependencyDefault for Values {}

/// The user interface state carried between cycles by the application.
///
/// ```ignore
/// let state = State::default();
///
/// let mut values = state.get();
/// # let id: Id = 1u128.try_into().unwrap();
/// values.active = Some(id);
///
/// // …
///
/// state.set(values);
///
/// ```
pub type State = Cell<Values>;
