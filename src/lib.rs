#![allow(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

pub use effects::Effects;
pub use reducer::Reducer;
pub use store::Store;
pub use store::testing::Store as TestStore;

mod effects;
mod reducer;
mod store;
