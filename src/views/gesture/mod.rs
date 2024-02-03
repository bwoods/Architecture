use std::cell::Cell;

use crate::dependencies::DependencyDefault;

mod recognizer;
pub use recognizer::*;

mod tap;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id(pub(crate) std::num::NonZeroU128);

#[doc(hidden)]
impl TryFrom<u128> for Id {
    type Error = std::num::TryFromIntError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

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
/// ```
/// # use composable::views::gesture::{Id, State};
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

#[test]
fn confirm_id_niche_optimization() {
    assert_eq!(std::mem::size_of::<Id>(), std::mem::size_of::<u128>());
}
