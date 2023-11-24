use std::borrow::Borrow;
use std::ops::Deref;

/// Used to return either a dependency’s reference or a separate owned value.
pub enum Ref<'a, T: 'a> {
    Borrowed(&'a T),
    Owned(T),
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Ref::Borrowed(reference) => reference,
            Ref::Owned(value) => &value,
        }
    }
}

impl<T> AsRef<T> for Ref<'_, T> {
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T> Borrow<T> for Ref<'_, T> {
    fn borrow(&self) -> &T {
        self.deref()
    }
}
