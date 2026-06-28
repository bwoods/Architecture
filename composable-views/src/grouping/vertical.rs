#[allow(unused_imports)]
use crate::{Bounds, Event, FixedWidth, Horizontal, Output, Padding, Size, Spacer, View};

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
        impl<$( $val: View ),+> Vertical for ( $( $val, )+ ) {
            fn justify(self, align: VerticalAlignment) -> impl View {
                let ( $( $val, )+ ) = self;

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

#[doc(hidden)]
impl<T: View> View for &[T] {
    #[inline]
    fn size(&self, within: Size) -> Size {
        let mut n = 0;
        let flexible = Size::new(0.0, f32::INFINITY);
        let mut total = Size::zero();

        for view in self.iter() {
            let next = view.size(flexible);

            match next.height == f32::INFINITY {
                true => n += view.frac(), // count the number of indeterminate heights
                false => {
                    total = Size::new(
                        f32::max(total.width, next.width),
                        total.height + next.height,
                    )
                }
            }
        }

        if n > 0 && within.height != 0.0 && within.height != f32::INFINITY {
            let frac = f32::max(0.0, (within.height - total.height) / n as f32);
            for view in self.iter() {
                view.adjust_height(frac);
            }
        }

        total
    }

    #[inline]
    fn event(&self, event: Event, bounds: Bounds) {
        let _ = self.size(bounds.size()); // adjusts sizes before .event()

        self.iter().fold(bounds, |mut bounds, view| {
            view.event(event.clone(), bounds);

            bounds.min.y += view.size(bounds.size()).height;
            bounds.min.y = f32::min(bounds.min.y, bounds.max.y);
            bounds
        });
    }

    #[inline]
    fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
        let _ = self.size(bounds.size()); // adjusts sizes before .draw()

        self.iter().fold(bounds, |mut bounds, view| {
            view.draw(bounds, onto);

            bounds.min.y += view.size(bounds.size()).height;
            bounds.min.y = f32::min(bounds.min.y, bounds.max.y);
            bounds
        });
    }

    #[inline(always)]
    fn adjust_width(&self, width: f32) {
        for view in self.iter() {
            view.adjust_width(width);
        }
    }

    #[inline(always)]
    fn adjust_height(&self, height: f32) {
        for view in self.iter() {
            view.adjust_height(height);
        }
    }

    #[inline(always)]
    fn frac(&self) -> usize {
        self.iter().fold(0, |n, view| n + view.frac())
    }
}
