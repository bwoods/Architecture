#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

pub use effects::{Effects, Task};
pub use reducer::Reducer;
pub use store::testing::Store as TestStore;
pub use store::Store;

pub mod dependency;
mod effects;
mod reducer;
mod store;
