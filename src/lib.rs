#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

pub use effects::{Effects, Task};
pub use reducer::Reducer;
pub use store::{testing::TestStore, Store};

pub mod dependencies;

mod effects;
mod reducer;
mod store;
