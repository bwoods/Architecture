use std::rc::Rc;

use ambience::thread::{get, set, AmbientGuard as Inner};

pub struct Guard<T: 'static> {
    _inner: Inner<T>,
}

impl<T: 'static> Guard<T> {
    pub(crate) fn new(value: T) -> Self {
        Self { _inner: set(value) }
    }

    pub(crate) fn get() -> Option<Rc<T>> {
        get::<T>().ok()
    }
}
