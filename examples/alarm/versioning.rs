#![allow(dead_code)]
use git_version::git_version;

pub const NAME: &str = env!("CARGO_BIN_NAME");
pub const CRATE: &str = env!("CARGO_CRATE_NAME");
pub const PACKAGE: &str = env!("CARGO_PKG_NAME");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");

pub const SOURCE: &str = git_version!(args = ["--always", "--dirty=+"]);
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
pub const MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
pub const PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");

pub const PRERELEASE: &str = env!("CARGO_PKG_VERSION_PRE");
