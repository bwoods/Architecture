#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod effects;
mod reducer;
mod store;

pub use effects::Effects;
pub use reducer::Reducer;
pub use store::Store;

pub use store::testing::Store as TestStore;
