pub use values::{DefaultDependency, Dependency};

mod guard;
mod values;

/// Supply a tuple of dependencies for the supplied closure
pub fn with_dependencies<T, F, R>(with: T, f: F) -> R
where
    T: Tuple,
    F: FnOnce() -> R,
{
    let _guards = with.guards();
    f()
}

/// Supply a single dependency for the supplied closure.
///
/// A convenience function that just forwards to [`with_dependencies`].
pub fn with_dependency<T, F, R>(with: T, f: F) -> R
where
    T: 'static,
    F: FnOnce() -> R,
{
    with_dependencies((with,), f)
}

/// A [`tuple`] of up to twenty-five values.
///
/// Used by [`with_dependencies`] to set the current [`Dependency`] values for its closure.
pub trait Tuple {
    #[doc(hidden)]
    type Output;

    #[doc(hidden)]
    fn guards(self) -> Self::Output;
}

macro_rules! tuple_impl {
    ( $($val:ident)+ ) => {
        #[doc(hidden)]
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        impl<$($val: 'static),+> Tuple for ( $($val,)+ ) {
            type Output = ( $(guard::Guard<$val>,)+ );

            fn guards(self) -> Self::Output {
                let ( $($val,)+ ) = self;
                ( $(guard::Guard::new($val),)+ )
            }
        }
    };
}

tuple_impl! { A }
tuple_impl! { A B }
tuple_impl! { A B C }
tuple_impl! { A B C D }
tuple_impl! { A B C D E }
tuple_impl! { A B C D E F }
tuple_impl! { A B C D E F G }
tuple_impl! { A B C D E F G H }
tuple_impl! { A B C D E F G H I }
tuple_impl! { A B C D E F G H I J }
tuple_impl! { A B C D E F G H I J K }
tuple_impl! { A B C D E F G H I J K L }
tuple_impl! { A B C D E F G H I J K L M }
tuple_impl! { A B C D E F G H I J K L M N }
tuple_impl! { A B C D E F G H I J K L M N O }
tuple_impl! { A B C D E F G H I J K L M N O P }
tuple_impl! { A B C D E F G H I J K L M N O P Q }
tuple_impl! { A B C D E F G H I J K L M N O P Q R }
tuple_impl! { A B C D E F G H I J K L M N O P Q R S }
tuple_impl! { A B C D E F G H I J K L M N O P Q R S T }
tuple_impl! { A B C D E F G H I J K L M N O P Q R S T U }
tuple_impl! { A B C D E F G H I J K L M N O P Q R S T U V }
tuple_impl! { A B C D E F G H I J K L M N O P Q R S T U V W }
tuple_impl! { A B C D E F G H I J K L M N O P Q R S T U V W X }
tuple_impl! { A B C D E F G H I J K L M N O P Q R S T U V W X Y }
// up to 25 dependencies supported
