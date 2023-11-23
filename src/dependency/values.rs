#![allow(dead_code)]

use std::borrow::Borrow;
use std::ops::Deref;
use std::rc::Rc;

use crate::dependency::guard::Guard;

struct DependencyValues {}

impl DependencyValues {
    pub fn set<T: 'static>(&self, value: T) -> Guard<T> {
        Guard::new(value)
    }

    pub fn clone_from<T: 'static>(value: Rc<T>) -> Guard<T> {
        Guard::clone_from(value)
    }
}

///
struct Dependency<T: DependencyKey> {
    inner: Rc<T>,
}

impl<T: DependencyKey> Deref for Dependency<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: DependencyKey> AsRef<T> for Dependency<T> {
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T: DependencyKey> Borrow<T> for Dependency<T> {
    fn borrow(&self) -> &T {
        self.deref()
    }
}

///
trait DependencyKey {
    fn live() -> Self;
}
