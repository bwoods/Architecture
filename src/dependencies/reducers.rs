use std::marker::PhantomData;

use crate::dependencies::{with_dependency, Dependency};
use crate::{Effects, Reducer};

/// `Reducer` modifiers for manipulating dependencies.
///
/// These method are ostly used when building
pub trait ReducerDependencies: Reducer + Sized {
    /// Supply a dependency for this `Reducer`.
    ///
    /// Consumes the `Reducer` and produces a new one with the passed in dependency.
    fn with_dependency<T: Clone + 'static>(self, with: T) -> impl Reducer<Action = Self::Action> {
        struct ReducerWithDependency<R: Reducer, T> {
            inner: R,
            dependency: T,
        }

        impl<R: Reducer, T: Clone + 'static> Reducer for ReducerWithDependency<R, T> {
            type Action = R::Action;
            type Output = R::Output;

            fn reduce(&mut self, action: Self::Action, effects: impl Effects<Self::Action>) {
                with_dependency(self.dependency.clone(), || {
                    self.inner.reduce(action, effects)
                });
            }
        }

        ReducerWithDependency {
            inner: self,
            dependency: with,
        }
    }

    /// Transforms a dependency for this `Reducer`.
    ///
    /// Consumes the `Reducer` and produces a new one that performs the mapping.
    ///
    /// ## Note
    /// Because dependency handing is based upon the dependency’s type, if `transform` returns a
    /// different type than the dependency being mapped, it will actually be creating a new dependency
    /// of _that_ type.
    fn map_dependency<T: 'static, F: FnMut(&T) -> U + Clone, U: 'static>(
        self,
        transform: F,
    ) -> impl Reducer<Action = Self::Action> {
        struct ReducerDependencyTransformer<R: Reducer, F: FnMut(&T) -> U, T, U> {
            inner: R,
            transform: F,
            marker: PhantomData<(T, U)>,
        }

        impl<R: Reducer, F: FnMut(&T) -> U, T: 'static, U: 'static> Reducer
            for ReducerDependencyTransformer<R, F, T, U>
        {
            type Action = R::Action;
            type Output = R::Output;

            fn reduce(&mut self, action: Self::Action, effects: impl Effects<Self::Action>) {
                match Dependency::<T>::new().as_deref() {
                    None => self.reduce(action, effects),
                    Some(current) => {
                        with_dependency((self.transform)(current), || {
                            self.inner.reduce(action, effects)
                        });
                    }
                }
            }
        }

        ReducerDependencyTransformer {
            inner: self,
            transform,
            marker: PhantomData,
        }
    }
}
