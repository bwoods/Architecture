#![allow(unused_imports)]
use crate::views::{Bounds, Event, Output, Point, Size, View};

#[doc(hidden)]
pub trait Layout {
    fn size(size: Size, next: Size) -> Size;
    fn bounds(bounds: Bounds, previous: Size) -> Bounds;
}

struct Vertical;

impl Layout for Vertical {
    fn size(size: Size, next: Size) -> Size {
        Size::new(f32::max(size.width, next.width), size.height + next.height)
    }

    fn bounds(mut bounds: Bounds, previous: Size) -> Bounds {
        bounds.min.y += previous.height;
        bounds
    }
}

struct Horizontal;

impl Layout for Horizontal {
    fn size(size: Size, next: Size) -> Size {
        Size::new(size.width + next.width, f32::max(size.height, next.height))
    }

    fn bounds(mut bounds: Bounds, previous: Size) -> Bounds {
        bounds.min.x += previous.width;
        bounds
    }
}

macro_rules! tuple_impl {
    ( $($val:ident)+ ) => {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        impl<$($val: View),+> View for ( $($val,)+ ) {
            fn size(&self) -> Size {
                let ( $(ref $val,)+ ) = self;
                let size = Size::zero();

                $(
                    let size = Vertical::size(size, $val.size());
                )+

                size
            }

            fn event(&self, event: Event, offset: Point, bounds: Bounds) {
                let ( $(ref $val,)+ ) = self;

                $(
                    $val.event(event, offset, bounds);
                    let bounds = Vertical::bounds(bounds, $val.size());
                )+
            }

            fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
                let ( $(ref $val,)+ ) = self;

                $(
                    $val.draw(bounds, onto);
                    let bounds = Vertical::bounds(bounds, $val.size());
                )+
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
// up to 25 views are supported
