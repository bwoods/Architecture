use ambience::thread::{set, AmbientGuard};

pub struct Guard<T: 'static> {
    _inner: AmbientGuard<T>,
}

impl<T: 'static> Guard<T> {
    pub(crate) fn new(new_val: T) -> Self {
        Self {
            _inner: set(new_val),
        }
    }
}
