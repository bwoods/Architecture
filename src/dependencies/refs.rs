use std::borrow::Borrow;
use std::ops::Deref;

/// Used to return a reference to a dependency or a separate owned value.
///
/// Used by:
/// - [`unwrap_or`][`super::Dependency::unwrap_or`]
/// - [`unwrap_or_else`][`super::Dependency::unwrap_or_else`]
/// - [`unwrap_or_default`][`super::Dependency::unwrap_or_default`]
/// - [`or`][`super::Dependency::or`]
/// - [`or_else`][`super::Dependency::or_else`]
/// - [`xor`][`super::Dependency::xor`]
pub enum Ref<'a, T: 'a> {
    Borrowed(&'a T),
    Owned(T),
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Ref::Borrowed(reference) => reference,
            Ref::Owned(value) => value,
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
