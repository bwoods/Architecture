#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

#[doc(inline)]
pub use derive_more::{AsMut, AsRef, From, TryInto};
pub use derive_reducers::Reducers;

pub use effects::{Effects, Task};
pub use reducer::Reducer;
pub use store::{testing::TestStore, Store};

pub mod dependencies;

mod effects;
mod reducer;
mod store;
