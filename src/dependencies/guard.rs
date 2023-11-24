use std::rc::Rc;

use ambience::thread::{AmbientGuard as Inner, get, has, set, set_rc};

pub struct Guard<T: 'static> {
    _inner: Inner<T>,
}

impl<T: 'static> Guard<T> {
    pub(crate) fn new(value: T) -> Self {
        Self { _inner: set(value) }
    }

    pub(crate) fn clone_from(value: Rc<T>) -> Self {
        Self {
            _inner: set_rc(value),
        }
    }

    pub(crate) fn exists() -> bool {
        has::<T>()
    }

    pub(crate) fn get() -> Option<Rc<T>> {
        get::<T>().ok()
    }
}
