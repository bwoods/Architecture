#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

pub use effects::{Effects, Task};
pub use reducer::Reducer;
pub use store::{Store, testing::TestStore};

pub mod dependencies;

mod effects;
mod reducer;
mod store;
