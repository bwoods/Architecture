//! `View` maybe grouped together with tuples. A tuple of `View`s is a valid
//! `View`.
//!
//! By default, a tuple of `View`s is [`Vertical`]; [`VerticalAlignment::Leading`] specifically.
use crate::{Bounds, Event, Output, Size, View};

pub mod horizontal;
pub mod vertical;

impl View for () {
    #[inline(always)]
    fn size(&self, _within: Size) -> Size {
        Size::zero()
    }

    #[inline(always)]
    fn draw(&self, _bounds: Bounds, _onto: &mut impl Output) {}
}

macro_rules! tuple_impl {
    ( $( $val:ident )+ ) => {
        #[doc(hidden)]
        #[allow(unused)]
        #[allow(non_snake_case)]
        impl<$( $val: View ),+> View for ( $( $val, )+ ) {
            #[inline]
            fn size(&self, size: Size) -> Size {
                let &( $( ref $val, )+ ) = self;

                let mut n = 0;
                let flexible = Size::new(0.0, f32::INFINITY);
                let mut total = Size::zero();

                $(
                    let next = $val.size(flexible);
                    match next.height == f32::INFINITY {
                        true => n += $val.frac(), // count the number of indeterminate heights
                        false => total = Size::new(f32::max(total.width, next.width), total.height + next.height),
                    }
                )+

                if n > 0 && size.height != 0.0 && size.height != f32::INFINITY {
                    let frac = f32::max(0.0, (size.height - total.height) / n as f32);
                    $( $val.adjust_height(frac); )+
                }

                total
            }

            #[inline]
            fn event(&self, event: Event, mut bounds: Bounds) {
                let within = bounds.size();
                let _ = self.size(within); // adjusts sizes before .event()

                let &( $(ref $val,)+ ) = self;
                $(
                    let height = $val.size(within).height;
                    $val.event(event.clone(), bounds);
                    bounds.min.y += height;
                    bounds.min.y = f32::min(bounds.min.y, bounds.max.y);
                )+
            }

            #[inline]
            fn draw(&self, mut bounds: Bounds, onto: &mut impl Output) {
                let within = bounds.size();
                let _ = self.size(within); // adjusts sizes before .draw()

                let &( $(ref $val,)+ ) = self;
                $(
                    let height = $val.size(within).height;
                    $val.draw(bounds, onto);
                    bounds.min.y += height;
                    bounds.min.y = f32::min(bounds.min.y, bounds.max.y);
                )+
            }

            #[inline(always)]
            fn adjust_width(&self, width: f32) {
                let &( $(ref $val,)+ ) = self;
                $(
                    $val.adjust_width(width);
                )+
            }

            #[inline(always)]
            fn adjust_height(&self, height: f32) {
                let &( $(ref $val,)+ ) = self;
                $(
                    $val.adjust_height(height);
                )+
            }

            #[inline(always)]
            fn frac(&self) -> usize {
                let mut n = 0;
                let &( $(ref $val,)+ ) = self;
                $(
                    n += $val.frac();
                )+

                n
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
