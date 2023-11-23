#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

pub use effects::{Effects, Task};
pub use reducer::Reducer;
pub use store::{testing::TestStore, Store};

pub use dependency::{with_dependencies, with_dependency};

mod dependency;
mod effects;
mod reducer;
mod store;
