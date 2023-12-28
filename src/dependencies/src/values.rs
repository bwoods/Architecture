use std::borrow::Borrow;
use std::cell::OnceCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::guard::Guard;
use crate::refs::Ref;

/// A wrapper type for accessing dependencies
pub struct Dependency<T: 'static> {
    inner: OnceCell<Rc<T>>,
}

impl<T> Default for Dependency<T> {
    fn default() -> Self {
        let cell = OnceCell::new();

        if let Some(inner) = Guard::get() {
            cell.set(inner).ok();
        }

        Self { inner: cell }
    }
}

/// - The methods of `Dependency` are very similar to those of [`std::option::Option`], as
///   dependencies are *optionally* present.
/// - However, a `Dependency` on a type with a [`DependencyDefault`] also implements the
///   [`AsRef`], [`Deref`] and [`Borrow`] traits. Event if a value has not been explicitly
///   registered for it, the `Dependency` will still be able to [`as_ref`], [`deref`] and
///   [`borrow`] this default value.
///
///  [`std::option::Option`]: std::option
///  [`as_ref`]: Dependency::as_ref
///  [`deref`]: Dependency::deref
///  [`borrow`]: Dependency::borrow
impl<T> Dependency<T> {
    #[inline]
    /// Creates a optional reference to the dependency of type `T`.
    pub fn new() -> Self {
        Self::default()
    }

    #[inline(always)]
    /// Returns `true` if the dependency is a [`Some`] value.
    pub fn is_some(&self) -> bool {
        self.inner.get().is_some()
    }

    #[inline(always)]
    /// Returns `true` if the dependency is a [`Some`] and the value inside of it matches a predicate.
    pub fn is_some_and(&self, f: impl FnOnce(&T) -> bool) -> bool {
        self.as_deref().filter(|inner| f(*inner)).is_some()
    }

    #[inline(always)]
    /// Returns `true` if the dependency is a [`None`] value.
    pub fn is_none(&self) -> bool {
        self.inner.get().is_none()
    }

    #[inline(always)]
    /// Returns a slice of the dependency value, if any. If this is [`None`], an empty slice is returned.
    pub fn as_slice(&self) -> &[T] {
        self.as_deref().map_or(&[], std::slice::from_ref)
    }

    #[inline(always)]
    /// Returns an iterator over the dependency value, if any.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.as_slice().iter()
    }

    #[inline(always)]
    /// Returns the dependency [`Some`] value.
    ///
    /// # Panics
    /// Panics if the dependency is a [`None`] with a custom panic message provided by `msg`.
    pub fn expect(&self, msg: &str) -> &T {
        self.as_deref().expect(msg)
    }

    #[inline(always)]
    /// Returns the contained [`Some`] value.
    ///
    /// # Panics
    /// Panics if the dependency value equals [`None`].
    pub fn unwrap(&self) -> &T {
        self.as_deref().unwrap()
    }

    #[inline(always)]
    /// Returns the dependency [`Some`] value or a provided default.
    pub fn unwrap_or(&self, default: T) -> Ref<'_, T> {
        self.as_deref()
            .map(Ref::Borrowed)
            .unwrap_or(Ref::Owned(default))
    }

    #[inline(always)]
    /// Returns the dependency [`Some`] value or computes it from a closure.
    pub fn unwrap_or_else<F>(&self, f: F) -> Ref<'_, T>
    where
        F: FnOnce() -> T,
    {
        self.as_deref()
            .map(Ref::Borrowed)
            .unwrap_or_else(|| Ref::Owned(f()))
    }

    #[inline(always)]
    /// Returns the dependency [`Some`] value or a default.
    pub fn unwrap_or_default(&self) -> Ref<'_, T>
    where
        T: Default,
    {
        self.as_deref()
            .map(Ref::Borrowed)
            .unwrap_or_else(|| Ref::Owned(T::default()))
    }

    #[inline(always)]
    /// Maps to [`Option<U>`] by applying a function to a dependency value (if [`Some`])
    /// or returns [`None`] (if [`None`]).
    pub fn map<U, F>(&self, f: F) -> Option<U>
    where
        F: FnOnce(&T) -> U,
    {
        self.as_deref().map(f)
    }

    #[inline(always)]
    /// Calls the provided closure with a reference to the dependency value (if [`Some`]).
    pub fn inspect<F>(&self, f: F) -> Option<&T>
    where
        F: FnOnce(&T),
    {
        self.as_deref().map(|inner| {
            f(inner);
            inner
        })
    }

    #[inline(always)]
    /// Returns the provided default result (if [`None`]),
    /// or applies a function to the dependency value (if [`Some`]).
    pub fn map_or<U, F>(&self, default: U, f: F) -> U
    where
        F: FnOnce(&T) -> U,
    {
        self.as_deref().map_or(default, f)
    }

    #[inline(always)]
    /// Computes a default function result (if [`None`]), or
    /// applies a different function to the dependency value (if [`Some`]).
    pub fn map_or_else<U, D, F>(&self, default: D, f: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(&T) -> U,
    {
        self.as_deref().map_or_else(default, f)
    }

    #[inline(always)]
    /// Transforms into a [`Result<&T, E>`], mapping [`Some`] to
    /// [`Ok`] and [`None`] to [`Err`].
    pub fn ok_or<E>(&self, err: E) -> Result<&T, E> {
        self.as_deref().ok_or(err)
    }

    #[inline(always)]
    /// Transforms into a [`Result<&T, E>`], mapping [`Some`] to
    /// [`Ok`] and [`None`] to [`Err`].
    pub fn ok_or_else<E, F>(&self, err: F) -> Result<&T, E>
    where
        F: FnOnce() -> E,
    {
        self.as_deref().ok_or_else(err)
    }

    #[inline]
    /// Converts into a [`Option<&T>`].
    ///
    /// # Note
    /// This is the preferred method for producing an [`Option`] to use with the
    /// [question mark operator][`?`].[^try]
    ///
    /// [`?`]: https://doc.rust-lang.org/nightly/core/option/index.html#the-question-mark-operator-
    /// [^try]: Once the [Try trait](https://github.com/rust-lang/rust/issues/84277) is stabilized
    ///         it will be implemented for `Dependency`.
    pub fn as_deref(&self) -> Option<&T> {
        self.inner.get().map(|inner| inner.deref())
    }

    #[inline(always)]
    /// Returns [`None`] if the dependency is [`None`], otherwise returns `rhs`.
    pub fn and<U>(&self, rhs: Option<U>) -> Option<U> {
        self.as_deref().and(rhs)
    }

    #[inline(always)]
    /// Returns [`None`] if the dependency is [`None`], otherwise calls `f` with the
    /// dependency value and returns the result.
    pub fn and_then<U, F: FnOnce(&T) -> Option<U>>(&self, f: F) -> Option<U> {
        self.as_deref().and_then(f)
    }

    #[inline(always)]
    /// Returns [`None`] if the dependency is [`None`], otherwise calls `predicate`
    /// with the dependency value and returns:
    pub fn filter<P>(&self, predicate: P) -> Option<&T>
    where
        P: FnOnce(&T) -> bool,
    {
        self.as_deref()
            .filter(|inner| predicate(*inner))
            .and(self.as_deref())
    }

    #[inline(always)]
    /// Returns the dependency if it is [`Some`], otherwise returns `rhs`.
    pub fn or(&self, rhs: Option<T>) -> Option<Ref<'_, T>> {
        self.as_deref().map(Ref::Borrowed).or(rhs.map(Ref::Owned))
    }

    #[inline(always)]
    /// Returns the dependency if it is [`Some`], otherwise calls `f` and returns the result.
    pub fn or_else<F>(&self, f: F) -> Option<Ref<'_, T>>
    where
        F: FnOnce() -> Option<T>,
    {
        self.as_deref()
            .map(Ref::Borrowed)
            .or_else(|| f().map(Ref::Owned))
    }

    #[inline(always)]
    /// Returns [`Some`] if only one of
    /// - the dependency, or
    /// - `rhs`
    ///
    /// is [`Some`], otherwise returns [`None`].
    pub fn xor(&self, rhs: Option<T>) -> Option<Ref<'_, T>> {
        self.as_deref().map(Ref::Borrowed).xor(rhs.map(Ref::Owned))
    }

    #[inline(always)]
    /// Maps the dependency to an [`Option<T>`] by **copying** the contents of the option.
    pub fn copied(&self) -> Option<T>
    where
        T: Copy,
    {
        self.as_deref().copied()
    }

    #[inline(always)]
    /// Maps the dependency to an [`Option<T>`] by **cloning** the contents of the option.
    pub fn cloned(&self) -> Option<T>
    where
        T: Clone,
    {
        self.as_deref().cloned()
    }
}

impl<T: DependencyDefault> Deref for Dependency<T> {
    type Target = T;

    #[track_caller]
    #[inline(never)]
    fn deref(&self) -> &Self::Target {
        self.as_deref().unwrap_or_else(|| {
            if cfg!(test) {
                let detailed_explanation = r#".
DependencyDefault types are not allowed to use their default implementation within units tests.
Either register the dependency on the TestStore or use with_dependency within the test itself.
"#;
                panic!(
                    "Dependency<{0}> was constructed during a test, but {0} was not registered{1}",
                    std::any::type_name::<T>(),
                    detailed_explanation
                );
            }

            let guard = Guard::new(T::default());
            std::mem::forget(guard);

            self.inner.set(Guard::get().unwrap()).ok();
            self.as_deref().unwrap()
        })
    }
}

impl<T: DependencyDefault> AsRef<T> for Dependency<T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T: DependencyDefault> Borrow<T> for Dependency<T> {
    #[inline(always)]
    fn borrow(&self) -> &T {
        self.deref()
    }
}

/// The default value for a dependency.
///
/// There may be many different versions of dependencies for testing but there is often just
/// a single default implementation for use in the the actual application.
///
/// Implementing this trait for a type ensures that a [`Dependency`] on it will always have
/// a value. If the `DependencyDefault` value has not been [overridden][`super::with_dependencies`]
/// it will be returned.
///
/// <div class="warning">
/// Attempting to use this default behavior in a unit test <em>will fail the test</em>,
/// as tests are <u>required</u> to explicitly supply all of their dependencies.
/// </div>
///
/// # Note
/// `DependencyDefault`s are only created as needed. When its first [`Dependency`] is
///  created, [`default`][`Default::default`] will be called once and the returned value will
///  be cached.
pub trait DependencyDefault: Default {}
