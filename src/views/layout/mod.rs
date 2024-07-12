#![allow(unused_imports)]
use crate::views::{Bounds, Event, Output, Point, Size, View}; // some of these are used in the macro

pub use spacing::Spacer;

mod spacing;

#[doc(hidden)]
struct Horizontal<T>(T);

macro_rules! tuple_impl {
    ( $($val:ident)+ ) => {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        impl<$($val: View),+> View for ( $($val,)+ ) {
            #[inline]
            fn size(&self) -> Size {
                let ( $(ref $val,)+ ) = self;

                let mut size = Size::zero();
                $(
                    let next = $val.size();
                    size = Size::new(f32::max(size.width, next.width), size.height + next.height);
                )+

                size
            }

            #[inline]
            fn event(&self, event: Event, offset: Point, mut bounds: Bounds) {
                self.update_layout(self.size(), bounds);

                let ( $(ref $val,)+ ) = self;
                $(
                    $val.event(event, offset, bounds);
                    bounds.min.y += $val.size().height;
                    bounds.min.y = f32::min(bounds.min.y, bounds.max.y);
                )+
            }

            #[inline]
            fn draw(&self, mut bounds: Bounds, onto: &mut impl Output) {
                self.update_layout(self.size(), bounds);

                let ( $(ref $val,)+ ) = self;
                $(
                    $val.draw(bounds, onto);
                    bounds.min.y += $val.size().height;
                    bounds.min.y = f32::min(bounds.min.y, bounds.max.y);
                )+
            }

            fn update_layout(&self, size: Size, bounds: Bounds) {
                let ( $(ref $val,)+ ) = self;

                let mut n = 0;
                $( n += $val.needs_layout() as u32; )+ // effectively const

                if n != 0 {
                    let mut height = 0.0;
                    $( height += $val.size().height; )+

                    let space = f32::max((bounds.height() - height) / n as f32, 0.0);
                    $( $val.update_layout(Size::new(0.0, space), bounds); )+
                }
            }

            #[inline(always)]
            fn across(self) -> impl View {
                Horizontal(self)
            }
        }

        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        impl<$($val: View),+> View for Horizontal<( $($val,)+ )> {
            #[inline]
            fn size(&self) -> Size {
                let ( $(ref $val,)+ ) = self.0;

                let mut size = Size::zero();
                $(
                    let next = $val.size();
                    size = Size::new(size.width + next.width, f32::max(size.height, next.height));
                )+

                size
            }

            #[inline]
            fn event(&self, event: Event, offset: Point, mut bounds: Bounds) {
                self.update_layout(self.size(), bounds);

                let ( $(ref $val,)+ ) = self.0;
                $(
                    $val.event(event, offset, bounds);
                    bounds.min.x += $val.size().width;
                    bounds.min.x = f32::min(bounds.min.x, bounds.max.x);
                )+
            }

            #[inline]
            fn draw(&self, mut bounds: Bounds, onto: &mut impl Output) {
                self.update_layout(self.size(), bounds);

                let ( $(ref $val,)+ ) = self.0;
                $(
                    $val.draw(bounds, onto);
                    bounds.min.x += $val.size().width;
                    bounds.min.x = f32::min(bounds.min.x, bounds.max.x);
                )+
            }

            #[inline(always)]
            fn needs_layout(&self) -> bool {
                self.0.needs_layout()
            }

            fn update_layout(&self, size: Size, bounds: Bounds) {
                let ( $(ref $val,)+ ) = self.0;

                let mut n = 0;
                $( n += $val.needs_layout() as u32;)+

                if n != 0 {
                    let mut width = 0.0;
                    $( width += $val.size().width; )+

                    let space = f32::max((bounds.width() - width) / n as f32, 0.0);
                    $( $val.update_layout(Size::new(space, 0.0), bounds); )+
                }
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
