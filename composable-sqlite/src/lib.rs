#![deny(unsafe_code)]
#![allow(dead_code)]
mod ffi;
mod serde;

use futures::Stream;
use sqlite3_sys::sqlite3;
use std::hash::{DefaultHasher, Hasher};

pub struct Commits<H = DefaultHasher> {
    commits: ffi::Commits<H>,
}

impl<H> Commits<H>
where
    H: Default + Hasher,
{
    pub fn new(db: *mut sqlite3) -> Self {
        Self {
            commits: ffi::Commits::new(db),
        }
    }

    pub fn parse_sql(&self, sql: &str) -> Result<impl Stream<Item = Event>, Failed> {
        self.commits.parse_sql(sql)
    }

    pub fn track<F, R, E>(&self, compilation: F) -> Result<(impl Stream<Item = Event>, R), E>
    where
        F: FnOnce() -> Result<R, E>,
        E: From<Failed>,
    {
        self.commits.track(compilation)
    }

    pub fn shrink_to_fit(&self) {
        self.commits.shrink_to_fit();
    }
}

#[derive(Copy, Clone)]
pub enum Event {
    Commit,
    Rollback,
}

#[derive(Copy, Clone)]
pub enum Failed {
    Authorizer(i32),
    Prepare(i32),
}
