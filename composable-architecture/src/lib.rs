#![doc = include_str!("../../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))] // show features flags in documentation
// #![deny(rustdoc::broken_intra_doc_links)]
#![deny(unsafe_code)]
#![allow(missing_docs)]
#![allow(dead_code)]

#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[path = "../../about/mod.rs"]
pub mod about;

mod derive_macros;
mod effects;
mod reducer;
mod store;

#[doc(no_inline)]
pub use derive_macros::{From, TryInto, derive_more};
// pub use derive_reducers;

pub use effects::Effects;
pub use effects::Scheduler;
pub use effects::Task;
pub use reducer::Reducer;
pub use store::Store;
