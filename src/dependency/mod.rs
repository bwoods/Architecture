mod guard;

/// Inject dependencies for the supplied closure
pub fn with_dependencies<Tuple, F, R>(with: Tuple, f: F) -> R
where
    Tuple: DependencyValues,
    F: FnOnce() -> R,
{
    let _guards = with.guards();
    f()
}

/// A convenience…
pub(crate) fn with_dependency<T, F, R>(with: T, f: F) -> R
where
    T: 'static,
    F: FnOnce() -> R,
{
    with_dependencies((with,), f)
}

pub trait DependencyValues {
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
        impl<$($val: 'static),+> DependencyValues for ( $($val,)+ ) {
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
