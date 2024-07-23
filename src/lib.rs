#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))] // show features flags in documentation
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(unsafe_code)]
#![allow(missing_docs)]
#![allow(dead_code)]

#[doc(no_inline)]
pub use derive_macros::*;
#[doc(inline)]
pub use effects::{Interval, Task};
pub use reducer::Reducer;
pub use store::{testing::TestClock, testing::TestStore, Store};

pub mod dependencies;
/// Optional view feature.
#[cfg(all(feature = "unstable", feature = "views"))]
pub mod views;

#[path = "../about/mod.rs"]
pub mod about;

/// `Effects` are used within `Reducer`s to propagate `Action`s as side-effects of performing other
/// `Action`s.
///
/// `Effects` are also `Schedulers` — able to apply modifiers to when (and how often) `Action`s. are sent.
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
