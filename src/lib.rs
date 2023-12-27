#![doc = include_str!("../README.md")]
#![feature(doc_auto_cfg)] // show features flags in documentation
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[doc(no_inline)]
pub use derive_macros::*;

#[doc(inline)]
pub use effects::Task;
pub use reducer::Reducer;
pub use store::{testing::TestStore, Store};

#[cfg(all(feature = "views", feature = "unreleased"))]
/// Optional view feature.
pub mod views {
    pub use composable_views::*;
}

/// `Effects` are used within `Reducer`s to propagate `Action`s as side-effects of performing other
/// `Action`s.
///
/// This is a “trait alias” (to the actual [`Effects`][`crate::effects::Effects`] trait) to simplify
/// `Reducer` signatures.
pub trait Effects<Action>: effects::Effects<Action = Action> {}

/// Until actual [trait aliases] are stabilized this [work around] allows the trait shown above
/// to be used anywhere that the [original trait] can.
///
/// [trait aliases]: https://github.com/rust-lang/rust/issues/63063
/// [work around]: https://github.com/rust-lang/rust/issues/41517#issuecomment-1100644808
/// [original trait]: crate::effects::Effects
impl<T, Action> Effects<Action> for T where T: effects::Effects<Action = Action> {}

pub mod derive_macros;
pub mod effects;
mod reducer;
mod store;
