#![allow(dead_code)]

#[allow(unused_imports)]
use crate::{Bounds, Event, FixedHeight, FixedWidth, Output, Size, Spacer, View};

pub enum HorizontalAlignment {
    Top,
    Middle,
    Bottom,
}

struct Aligned<T>(T);

pub trait Horizontal {
    fn align(self, align: HorizontalAlignment) -> impl View;

    fn inline(self) -> impl View;
}

/// Top-aligned behavior is the default, so overheads are eliminated with this
/// custom implementation.
struct TopAligned<V>(V);

macro_rules! horizontal_impl {
    ( $($val:ident)+ ) => {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        impl<$($val: View),+> Horizontal for ( $($val,)+ ) {
            fn align(self, _align: HorizontalAlignment) -> impl View {
                // TODO:
            }

            fn inline(self) -> impl View {
                TopAligned(self)

                // let height = self.size(Bounds::from_size(Size::new(f32::INFINITY, 0.0))).height;
                // FixedHeight { view: TopAligned(self), height }
            }
        }

        #[doc(hidden)]
        #[allow(unused)]
        #[allow(non_snake_case)]
        impl<$($val: View),+> View for TopAligned<( $($val,)+ )> {
            #[inline(always)]
            fn size(&self, bounds: Bounds) -> Size {
                let &( $( ref $val, )+ ) = &self.0;

                let mut n = 0;
                let flexible = Bounds::from_size(Size::new(f32::INFINITY, 0.0));
                let mut total = Size::zero();

                $(
                    let next = $val.size(flexible);
                    match next.width == f32::INFINITY {
                        true => n += 1, // count the number of indeterminate widths
                        false => total = Size::new(total.width + next.width, f32::max(total.height, next.height)),
                    }
                )+

                if n > 0 && bounds.max.x != 0.0 && bounds.max.x != f32::INFINITY {
                    let frac = f32::max(0.0, (bounds.width() - total.width) / n as f32);
                    $( $val.adjust_width(frac); )+
                }

                total
            }

            #[inline(always)]
            fn event(&self, event: Event, mut bounds: Bounds) {
                let _ = self.size(bounds); // adjusts sizes before .event()

                let &( $(ref $val,)+ ) = &self.0;
                $(
                    let width = $val.size(bounds).width;
                    $val.event(event.clone(), bounds);
                    bounds.min.x += width;
                    bounds.min.x = f32::min(bounds.min.x, bounds.max.x);
                )+
            }

            #[inline(always)]
            fn draw(&self, mut bounds: Bounds, onto: &mut impl Output) {
                let _ = self.size(bounds); // adjusts sizes before .draw()

                let &( $(ref $val,)+ ) = &self.0;
                $(
                    let width = $val.size(bounds).width;
                    $val.draw(bounds, onto);
                    bounds.min.x += width;
                    bounds.min.x = f32::min(bounds.min.x, bounds.max.x);
                )+
            }

            #[inline]
            fn adjust_width(&self, width: f32) {
                let &( $(ref $val,)+ ) = &self.0;
                $(
                    $val.adjust_width(width);
                )+
            }

            #[inline]
            fn adjust_height(&self, height: f32) {
                let &( $(ref $val,)+ ) = &self.0;
                $(
                    $val.adjust_height(height);
                )+
            }
        }
    };
}

horizontal_impl! { A }
horizontal_impl! { A B }
horizontal_impl! { A B C }
horizontal_impl! { A B C D }
horizontal_impl! { A B C D E }
horizontal_impl! { A B C D E F }
horizontal_impl! { A B C D E F G }
horizontal_impl! { A B C D E F G H }
horizontal_impl! { A B C D E F G H I }
horizontal_impl! { A B C D E F G H I J }
horizontal_impl! { A B C D E F G H I J K }
horizontal_impl! { A B C D E F G H I J K L }
horizontal_impl! { A B C D E F G H I J K L M }
horizontal_impl! { A B C D E F G H I J K L M N }
horizontal_impl! { A B C D E F G H I J K L M N O }
horizontal_impl! { A B C D E F G H I J K L M N O P }
horizontal_impl! { A B C D E F G H I J K L M N O P Q }
horizontal_impl! { A B C D E F G H I J K L M N O P Q R }
horizontal_impl! { A B C D E F G H I J K L M N O P Q R S }
horizontal_impl! { A B C D E F G H I J K L M N O P Q R S T }
horizontal_impl! { A B C D E F G H I J K L M N O P Q R S T U }
horizontal_impl! { A B C D E F G H I J K L M N O P Q R S T U V }
horizontal_impl! { A B C D E F G H I J K L M N O P Q R S T U V W }
horizontal_impl! { A B C D E F G H I J K L M N O P Q R S T U V W X }
horizontal_impl! { A B C D E F G H I J K L M N O P Q R S T U V W X Y }
// up to 25 views are supported

#[doc(hidden)]
impl<V: View, const N: usize> Horizontal for [V; N] {
    fn align(self, _align: HorizontalAlignment) -> impl View {
        // TODO:
    }

    fn inline(self) -> impl View {
        let view = TopAligned(self);
        let height = view.size(Bounds::default()).height;
        FixedHeight { view, height }
    }
}

#[doc(hidden)]
impl<T: View, const N: usize> View for TopAligned<[T; N]> {
    #[inline]
    fn size(&self, bounds: Bounds) -> Size {
        let mut n = 0;
        let flexible = Bounds::from_size(Size::new(f32::INFINITY, 0.0));
        let mut total = Size::zero();

        for view in &self.0 {
            let next = view.size(flexible);

            match next.width == f32::INFINITY {
                true => n += 1, // count the number of indeterminate widths
                false => {
                    total = Size::new(
                        total.width + next.width,
                        f32::max(total.height, next.height),
                    )
                }
            }
        }

        if n > 0 && bounds.max.x != 0.0 && bounds.max.x != f32::INFINITY {
            let frac = f32::max(0.0, (bounds.width() - total.width) / n as f32);
            for view in &self.0 {
                view.adjust_width(frac);
            }
        }

        total
    }

    #[inline]
    fn event(&self, event: Event, bounds: Bounds) {
        let _ = self.size(bounds); // adjusts sizes before .event()

        self.0.iter().fold(bounds, |mut bounds, view| {
            view.event(event.clone(), bounds);

            bounds.min.x += view.size(bounds).width;
            bounds.min.x = f32::min(bounds.min.x, bounds.max.x);
            bounds
        });
    }

    #[inline]
    fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
        let _ = self.size(bounds); // adjusts sizes before .draw()

        self.0.iter().fold(bounds, |mut bounds, view| {
            view.draw(bounds, onto);

            bounds.min.x += view.size(bounds).width;
            bounds.min.x = f32::min(bounds.min.x, bounds.max.x);
            bounds
        });
    }

    #[inline]
    fn adjust_width(&self, width: f32) {
        for view in &self.0 {
            view.adjust_width(width);
        }
    }

    #[inline]
    fn adjust_height(&self, height: f32) {
        for view in &self.0 {
            view.adjust_height(height);
        }
    }
}
