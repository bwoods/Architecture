#[allow(unused_imports)]
use crate::{Bounds, FixedWidth, Horizontal, Padding, Size, Spacer, View};

pub enum VerticalAlignment {
    Left,
    Center,
    Right,
}

pub trait Vertical: View {
    fn justify(self, align: VerticalAlignment) -> impl View;
}

// TODO:
// struct LeftAligned<V>(V);

macro_rules! vertical_impl {
    ( $($val:ident)+ ) => {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        impl<$($val: View),+> Vertical for ( $($val,)+ ) {
            fn justify(self, align: VerticalAlignment) -> impl View {
                let ( $( $val,)+ ) = self;

                let mut total = 0.0;
                $( total = f32::max(total, $val.size(Size::default()).width); )+
                let fit = Size::new(total, 0.0);

                let view = match align {
                    VerticalAlignment::Left => {
                        ( $( Padding::right(total - $val.size(fit).width, $val) ),+ )
                    },
                    VerticalAlignment::Center => {
                        ( $( Padding::horizontal((total - $val.size(fit).width) / 2.0, $val) ),+ )
                    },
                    VerticalAlignment::Right => {
                        ( $( Padding::left(total - $val.size(fit).width, $val) ),+ )
                    },
                };

                view
            }
        }
    };
}

vertical_impl! { A }
vertical_impl! { A B }
vertical_impl! { A B C }
vertical_impl! { A B C D }
vertical_impl! { A B C D E }
vertical_impl! { A B C D E F }
vertical_impl! { A B C D E F G }
vertical_impl! { A B C D E F G H }
vertical_impl! { A B C D E F G H I }
vertical_impl! { A B C D E F G H I J }
vertical_impl! { A B C D E F G H I J K }
vertical_impl! { A B C D E F G H I J K L }
vertical_impl! { A B C D E F G H I J K L M }
vertical_impl! { A B C D E F G H I J K L M N }
vertical_impl! { A B C D E F G H I J K L M N O }
vertical_impl! { A B C D E F G H I J K L M N O P }
vertical_impl! { A B C D E F G H I J K L M N O P Q }
vertical_impl! { A B C D E F G H I J K L M N O P Q R }
vertical_impl! { A B C D E F G H I J K L M N O P Q R S }
vertical_impl! { A B C D E F G H I J K L M N O P Q R S T }
vertical_impl! { A B C D E F G H I J K L M N O P Q R S T U }
vertical_impl! { A B C D E F G H I J K L M N O P Q R S T U V }
vertical_impl! { A B C D E F G H I J K L M N O P Q R S T U V W }
vertical_impl! { A B C D E F G H I J K L M N O P Q R S T U V W X }
vertical_impl! { A B C D E F G H I J K L M N O P Q R S T U V W X Y }
// up to 25 views are supported
