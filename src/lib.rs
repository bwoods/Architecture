#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))] // show features flags in documentation
#![deny(unsafe_code)]
#![allow(missing_docs)]
#![allow(dead_code)]

#[doc(no_inline)]
pub use derive_macros::*;
#[doc(inline)]
pub use effects::Task;
pub use reducer::Reducer;
pub use store::{testing::TestStore, Store};

pub mod dependencies;
/// Optional view feature.
#[cfg(all(feature = "unreleased", feature = "views"))]
pub mod views;

/// `Effects` are used within `Reducer`s to propagate `Action`s as side-effects of performing other
/// `Action`s.
///
/// This is a “trait alias” (to the actual [`Effects`][`crate::effects::Effects`] trait) to simplify
/// `Reducer` signatures and set the lifetime to `'static`.
pub trait Effects<Action>: effects::Effects<Action = Action> + 'static {}

/// Until actual [trait aliases] are stabilized this [work around] allows the trait shown above
/// to be used anywhere that the [original trait] can.
///
/// [trait aliases]: https://github.com/rust-lang/rust/issues/63063
/// [work around]: https://github.com/rust-lang/rust/issues/41517#issuecomment-1100644808
/// [original trait]: crate::effects::Effects
impl<T, Action> Effects<Action> for T where T: effects::Effects<Action = Action> + 'static {}

pub mod derive_macros;
pub mod effects;
mod reducer;
mod store;
