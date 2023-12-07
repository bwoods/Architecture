#![doc = include_str!("../README.md")]
#![feature(doc_auto_cfg)] // show features flags in documentation
#![forbid(unsafe_code)]

#[doc(no_inline)]
pub use derive_macros::*;

#[doc(inline)]
pub use effects::Task;
pub use reducer::Reducer;
pub use store::{testing::TestStore, Store};

/// `Effects` are used within `Reducer`s to propagate `Action`s as side-effects of performing other
/// `Action`s.
///
/// This is a “trait alias” (to the actual [`Effects`][`crate::effects::Effects`] trait) to simplify
/// `Reducer` signatures.
pub trait Effects<Action>: effects::Effects<Action = Action> {}

#[doc(hidden)]
impl<T, Action> Effects<Action> for T where T: effects::Effects<Action = Action> {}

pub mod dependencies;
pub mod derive_macros;
pub mod effects;
mod reducer;
mod store;
