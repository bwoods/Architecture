use std::borrow::Borrow;
use std::ops::Deref;
use std::rc::Rc;

use maybe_owned::MaybeOwned;

use crate::dependencies::guard::Guard;

/// …
pub struct DependencyValues {}

impl DependencyValues {
    pub fn set<T: 'static>(&self, value: T) -> Guard<T> {
        Guard::new(value)
    }

    pub fn clone_from<T: 'static>(value: Rc<T>) -> Guard<T> {
        Guard::clone_from(value)
    }

    /// Can be used to [`assert!`][`std::assert`] (or [`debug_assert!`][`std::debug_assert`])
    /// that a dependency has been set rather that waiting for a runtime failure.
    pub fn contains<T: 'static>() -> bool {
        Guard::<T>::exists()
    }
}

///
pub struct Dependency<T: 'static> {
    inner: Option<Rc<T>>,
}

impl<T: 'static> Default for Dependency<T> {
    #[inline]
    fn default() -> Self {
        Self {
            inner: Guard::get(),
        }
    }
}

/// - Dependency implements most of the methods of [`Option`][`std::option`], as each dependency
///   is effectively *optionally* present.
/// - See [`DependencyKey`] for registering dependencies that are always present; allow you to
///   [`AsRef`],[`Deref`] or [`Borrow`] their values freely.
impl<T: 'static> Dependency<T> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    fn as_deref(&self) -> Option<&T> {
        self.inner.as_deref()
    }

    #[inline(always)]
    pub fn and<U>(&self, rhs: Option<U>) -> Option<U> {
        self.as_deref().and(rhs)
    }

    #[inline(always)]
    pub fn and_then<U, F: FnOnce(&T) -> Option<U>>(&self, f: F) -> Option<U> {
        self.as_deref().and_then(f)
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        self.as_deref().map_or(&[], std::slice::from_ref)
    }

    #[inline(always)]
    pub fn cloned(&self) -> Option<T>
    where
        T: Clone,
    {
        self.as_deref().cloned()
    }

    #[inline(always)]
    pub fn expect(&self, msg: &str) -> &T {
        self.as_deref().expect(msg)
    }

    #[inline(always)]
    pub fn filter<P>(&self, predicate: P) -> Option<&T>
    where
        P: FnOnce(&T) -> bool,
    {
        self.as_deref()
            .filter(|rr| predicate(*rr))
            .and(self.as_deref())
    }

    #[inline(always)]
    pub fn inspect<F>(&self, f: F) -> Option<&T>
    where
        F: FnOnce(&T),
    {
        self.as_deref().inspect(|rr| f(*rr)).and(self.as_deref())
    }

    #[inline(always)]
    pub const fn is_none(&self) -> bool {
        self.inner.is_none()
    }

    #[inline(always)]
    pub const fn is_some(&self) -> bool {
        self.inner.is_some()
    }

    #[inline(always)]
    pub fn is_some_and(&self, f: impl FnOnce(&T) -> bool) -> bool {
        self.as_deref().filter(|rr| f(*rr)).is_some()
    }

    #[inline(always)]
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.as_slice().iter()
    }

    #[inline(always)]
    pub fn map<U, F>(&self, f: F) -> Option<U>
    where
        F: FnOnce(&T) -> U,
    {
        self.as_deref().map(f)
    }

    #[inline(always)]
    pub fn map_or<U, F>(&self, default: U, f: F) -> U
    where
        F: FnOnce(&T) -> U,
    {
        self.as_deref().map_or(default, f)
    }

    #[inline(always)]
    pub fn map_or_else<U, D, F>(&self, default: D, f: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(&T) -> U,
    {
        self.as_deref().map_or_else(default, f)
    }

    #[inline(always)]
    pub fn ok_or<E>(&self, err: E) -> Result<&T, E> {
        self.as_deref().ok_or(err)
    }

    #[inline(always)]
    pub fn ok_or_else<E, F>(&self, err: F) -> Result<&T, E>
    where
        F: FnOnce() -> E,
    {
        self.as_deref().ok_or_else(err)
    }

    #[inline(always)]
    pub fn or(&self, rhs: Option<T>) -> Option<MaybeOwned<'_, T>> {
        self.as_deref()
            .map(MaybeOwned::Borrowed)
            .or(rhs.map(MaybeOwned::Owned))
    }

    #[inline(always)]
    pub fn or_else<F>(&self, f: F) -> Option<MaybeOwned<'_, T>>
    where
        F: FnOnce() -> Option<T>,
    {
        self.as_deref()
            .map(MaybeOwned::Borrowed)
            .or_else(|| f().map(MaybeOwned::Owned))
    }

    #[inline(always)]
    pub fn unwrap(&self) -> &T {
        self.as_deref().unwrap()
    }

    #[inline(always)]
    pub fn unwrap_or(&self, default: T) -> MaybeOwned<'_, T> {
        self.as_deref()
            .map(MaybeOwned::Borrowed)
            .unwrap_or(MaybeOwned::Owned(default))
    }

    #[inline(always)]
    pub fn unwrap_or_else<F>(&self, f: F) -> MaybeOwned<'_, T>
    where
        F: FnOnce() -> T,
    {
        self.as_deref()
            .map(MaybeOwned::Borrowed)
            .unwrap_or_else(|| MaybeOwned::Owned(f()))
    }

    #[inline(always)]
    pub fn unwrap_or_default(&self) -> MaybeOwned<'_, T>
    where
        T: Default,
    {
        self.as_deref()
            .map(MaybeOwned::Borrowed)
            .unwrap_or_else(|| MaybeOwned::Owned(T::default()))
    }

    #[inline(always)]
    pub fn xor(&self, rhs: Option<T>) -> Option<MaybeOwned<'_, T>> {
        self.as_deref()
            .map(MaybeOwned::Borrowed)
            .xor(rhs.map(MaybeOwned::Owned))
    }
}

impl<T: DependencyKey> Deref for Dependency<T> {
    type Target = T;

    #[track_caller]
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_deref()
            .or_else(|| {
                // This exact instance of Dependency registered the .live dependency in a
                // previous call to this method (see the `.unwrap_or_else` block below)
                // but, because we can not update `self.inner` here, this same instance
                // will have to do this lookup everytime.
                Some(Rc::make_mut(&mut Guard::<&T>::get().unwrap()))

                //  Note that wrapping `inner` in a RefCell to allow an update breaks
                // `Dependency::as_deref()` because the compiler then considers the reference
                //  to be coming from a temporary.

                // In practice, this only really affects uses of Dependency as struct field;
                // those created as variables or via `with_dependency` will be refreshed on
                // the very next call.
            })
            .unwrap_or_else(|| {
                if cfg!(test) {
                    panic!(
                        "A .live() DependencyKey was requested during a test: {}",
                        std::any::type_name::<T>()
                    );
                }

                let leaked: &T = Box::leak(Box::new(T::live()));

                // Unfortunately, this means that anyone creating a Dependency<&T> (note the ref)
                // will always get the .live() version of DependencyKey<T>, if any — unwittingly
                // bypassing every dependency override in scope.
                let guard = Guard::<&T>::new(leaked);
                std::mem::forget(guard);

                Rc::make_mut(&mut Guard::<&T>::get().unwrap())
            })
    }
}

impl<T: DependencyKey> AsRef<T> for Dependency<T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T: DependencyKey> Borrow<T> for Dependency<T> {
    #[inline(always)]
    fn borrow(&self) -> &T {
        self.deref()
    }
}

///
pub trait DependencyKey {
    fn live() -> Self;
}
