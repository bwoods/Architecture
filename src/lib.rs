#![doc = include_str!("../README.md")]
#![feature(doc_auto_cfg)] // show features flags in documentation
#![forbid(unsafe_code)]

#[doc(no_inline)]
pub use derive_more::{From, TryInto};
pub use derive_reducers::RecursiveReducer;

pub use effects::{Effects, Task};
pub use reducer::Reducer;
pub use store::{testing::TestStore, Store};

pub mod dependencies;

mod effects;
mod reducer;
mod store;
