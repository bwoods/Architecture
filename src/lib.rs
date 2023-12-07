#![doc = include_str!("../README.md")]
#![feature(doc_auto_cfg)] // show features flags in documentation
#![forbid(unsafe_code)]

#[doc(no_inline)]
pub use derive_macros::{From, RecursiveReducer, TryInto};

pub use effects::{Effects, Task};
pub use reducer::Reducer;
pub use store::{testing::TestStore, Store};

pub mod dependencies;

pub mod derive_macros;

mod effects;
mod reducer;
mod store;
