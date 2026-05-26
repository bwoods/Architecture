//! `View` maybe grouped together with tuples. A tuple of `View`s is a valid
//! `View`.
//!
//! By default, a tuple of `View`s is [`Vertical`]; [`VerticalAlignment::Leading`] specifically.
use crate::{Bounds, Event, Output, Size, View};

pub mod horizontal;
pub mod vertical;

impl View for () {
    #[inline(always)]
    fn size(&self, _bounds: Bounds) -> Size {
        Size::zero()
    }

    #[inline(always)]
    fn event(&self, _event: Event, _bounds: Bounds) {}

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
            fn size(&self, bounds: Bounds) -> Size {
                let &( $( ref $val, )+ ) = self;

                let mut n = 0;
                let flexible = Bounds::from_size(Size::new(0.0, f32::INFINITY));
                let mut total = Size::zero();

                $(
                    let next = $val.size(flexible);
                    match next.height == f32::INFINITY {
                        true => n += 1, // count the number of indeterminate heights
                        false => total = Size::new(f32::max(total.width, next.width), total.height + next.height),
                    }
                )+

                if n > 0 && bounds.max.y != 0.0 && bounds.max.y != f32::INFINITY {
                    let frac = f32::max(0.0, (bounds.height() - total.height) / n as f32);
                    $( $val.adjust_height(frac); )+
                }

                total
            }

            #[inline]
            fn event(&self, event: Event, mut bounds: Bounds) {
                let _ = self.size(bounds); // adjusts sizes before .event()

                let &( $(ref $val,)+ ) = self;
                $(
                    let height = $val.size(bounds).height;
                    $val.event(event.clone(), bounds);
                    bounds.min.y += height;
                    bounds.min.y = f32::min(bounds.min.y, bounds.max.y);
                )+
            }

            #[inline]
            fn draw(&self, mut bounds: Bounds, onto: &mut impl Output) {
                let _ = self.size(bounds); // adjusts sizes before .draw()

                let &( $(ref $val,)+ ) = self;
                $(
                    let height = $val.size(bounds).height;
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
