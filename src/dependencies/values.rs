use std::borrow::Borrow;
use std::cell::OnceCell;
use std::ops::Deref;
use std::rc::Rc;

use maybe_owned::MaybeOwned;

use crate::dependencies::guard::Guard;

///
pub struct Dependency<T: 'static> {
    inner: OnceCell<Rc<T>>,
}

impl<T> Default for Dependency<T> {
    fn default() -> Self {
        let cell = OnceCell::new();

        if let Some(inner) = Guard::get() {
            let result = cell.set(inner);
            debug_assert!(result.is_ok());
        }

        Self { inner: cell }
    }
}

/// - `Dependency` implements very similar methods to [`Option`][`std::option`], as dependencies
///    are *optionally* present.
/// - However, a `Dependency` on a [`DefaultDependency`] also implements [`Deref`], [`AsRef`] and [`Borrow`]  
///   as default dependencies are registered as needed.
impl<T> Dependency<T> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    fn as_deref(&self) -> Option<&T> {
        self.inner.get().map(|inner| inner.deref())
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
    pub fn is_none(&self) -> bool {
        self.inner.get().is_none()
    }

    #[inline(always)]
    pub fn is_some(&self) -> bool {
        self.inner.get().is_some()
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

impl<T: DefaultDependency> Deref for Dependency<T> {
    type Target = T;

    #[track_caller]
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_deref().unwrap_or_else(|| {
            if cfg!(test) {
                let detailed_explanation = r#" 
DefaultDependency types are not allowed to use their default implementation within units tests.
Either register the dependency on the TestStore or use with_dependency within the test itself."#;
                panic!(
                    "Dependency<{0}> was constructed during a test, but {0} was not registered.{1}",
                    std::any::type_name::<T>(),
                    detailed_explanation
                );
            }

            let guard = Guard::new(T::default());
            std::mem::forget(guard);

            let result = self.inner.set(Guard::get().unwrap());
            debug_assert!(result.is_ok());
            self.as_deref().unwrap()
        })
    }
}

impl<T: DefaultDependency> AsRef<T> for Dependency<T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T: DefaultDependency> Borrow<T> for Dependency<T> {
    #[inline(always)]
    fn borrow(&self) -> &T {
        self.deref()
    }
}

/// A dependency with a default value.
///
/// There may be many different versions of dependencies for testingm but there is often just
/// a single default implementation for use in the the actual application.
///
/// Implementing this trait for a type ensures that a [`Dependency`] on it will always have
/// a value. If the `DefaultDependency` has not been [overridden][`super::with_dependencies`]
/// this default value will be returned.
///
/// <div class="warning">
/// Failing to override a default used in a unit test <em>will fail the test</em>
/// as tests are <u>required</u> to explicitly supply all of their dependencies.
/// </div>
///
/// # Note
/// `DefaultDependency`s are only created as needed. When its first [`Dependency`] is
///  created, [`default`][`Default::default`] will be called once and the returned value will
///  be cached.
pub trait DefaultDependency: Default {}
