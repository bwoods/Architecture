#[allow(unused_imports)]
use crate::{Bounds, Event, FixedHeight, FixedWidth, Output, Padding, Size, Spacer, View};

pub enum HorizontalAlignment {
    Top,
    Middle,
    Bottom,
}

pub trait Horizontal {
    fn align(self, align: HorizontalAlignment) -> impl View;

    fn inline(self) -> impl View;
}

/// Aligns a group of `View`s horizontally
struct Aligned<V>(V);

macro_rules! horizontal_impl {
    ( $($val:ident)+ ) => {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        impl<$( $val: View ),+> Horizontal for ( $( $val, )+ ) {
            fn align(self, align: HorizontalAlignment) -> impl View {
                let ( $( $val, )+ ) = self;

                let mut total = 0.0;
                $( total = f32::max(total, $val.size(Size::default()).height); )+
                let fit = Size::new(0.0, total);

                let view = match align {
                    HorizontalAlignment::Top => ( $( Padding::bottom(total - $val.size(fit).height, $val), )+ ),
                    HorizontalAlignment::Middle =>  ( $( Padding::vertical((total - $val.size(fit).height) / 2.0, $val), )+ ),
                    HorizontalAlignment::Bottom => ( $( Padding::top(total - $val.size(fit).height, $val), )+ ),
                };

                Aligned(view)
            }

            fn inline(self) -> impl View {
                Aligned(self)
            }
        }

        #[doc(hidden)]
        #[allow(unused)]
        #[allow(non_snake_case)]
        impl<$( $val: View ),+> View for Aligned<( $( $val, )+ )> {
            #[inline]
            fn size(&self, size: Size) -> Size {
                let &( $( ref $val, )+ ) = &self.0;

                let mut n = 0;
                let flexible = Size::new(f32::INFINITY, 0.0);
                let mut total = Size::zero();

                $(
                    let next = $val.size(flexible);
                    match next.width == f32::INFINITY {
                        true => n += $val.frac(), // count the number of indeterminate widths
                        false => total = Size::new(
                            total.width + next.width,
                            f32::max(total.height, next.height)
                        ),
                    }
                )+

                if n > 0 && size.width != 0.0 && size.width != f32::INFINITY {
                    let frac = f32::max(0.0, (size.width - total.width) / n as f32);
                    $( $val.adjust_width(frac); )+
                }

                total
            }

            #[inline]
            fn event(&self, event: Event, mut bounds: Bounds) {
                let _ = self.size(bounds.size()); // adjusts sizes before .event()

                let &( $(ref $val,)+ ) = &self.0;
                $(
                    let width = $val.size(bounds.size()).width;
                    $val.event(event.clone(), bounds);
                    bounds.min.x += width;
                    bounds.min.x = f32::min(bounds.min.x, bounds.max.x);
                )+
            }

            #[inline]
            fn draw(&self, mut bounds: Bounds, onto: &mut impl Output) {
                let _ = self.size(bounds.size()); // adjusts sizes before .draw()

                let &( $(ref $val,)+ ) = &self.0;
                $(
                    let width = $val.size(bounds.size()).width;
                    $val.draw(bounds, onto);
                    bounds.min.x += width;
                    bounds.min.x = f32::min(bounds.min.x, bounds.max.x);
                )+
            }

            #[inline(always)]
            fn adjust_width(&self, width: f32) {
                let &( $(ref $val,)+ ) = &self.0;
                $(
                    $val.adjust_width(width);
                )+
            }

            #[inline(always)]
            fn adjust_height(&self, height: f32) {
                let &( $(ref $val,)+ ) = &self.0;
                $(
                    $val.adjust_height(height);
                )+
            }

            #[inline(always)]
            fn frac(&self) -> usize {
                let mut n = 0;
                let &( $(ref $val,)+ ) = &self.0;
                $(
                    n += $val.frac();
                )+

                n
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
        Aligned(self)
    }
}

#[doc(hidden)]
impl<T: View, const N: usize> View for Aligned<[T; N]> {
    #[inline]
    fn size(&self, within: Size) -> Size {
        let mut n = 0;
        let flexible = Size::new(f32::INFINITY, 0.0);
        let mut total = Size::zero();

        for view in &self.0 {
            let next = view.size(flexible);

            match next.width == f32::INFINITY {
                true => n += view.frac(), // count the number of indeterminate widths
                false => {
                    total = Size::new(
                        total.width + next.width,
                        f32::max(total.height, next.height),
                    )
                }
            }
        }

        if n > 0 && within.width != 0.0 && within.width != f32::INFINITY {
            let frac = f32::max(0.0, (within.width - total.width) / n as f32);
            for view in &self.0 {
                view.adjust_width(frac);
            }
        }

        total
    }

    #[inline]
    fn event(&self, event: Event, bounds: Bounds) {
        let _ = self.size(bounds.size()); // adjusts sizes before .event()

        self.0.iter().fold(bounds, |mut bounds, view| {
            view.event(event.clone(), bounds);

            bounds.min.x += view.size(bounds.size()).width;
            bounds.min.x = f32::min(bounds.min.x, bounds.max.x);
            bounds
        });
    }

    #[inline]
    fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
        let _ = self.size(bounds.size()); // adjusts sizes before .draw()

        self.0.iter().fold(bounds, |mut bounds, view| {
            view.draw(bounds, onto);

            bounds.min.x += view.size(bounds.size()).width;
            bounds.min.x = f32::min(bounds.min.x, bounds.max.x);
            bounds
        });
    }

    #[inline(always)]
    fn adjust_width(&self, width: f32) {
        for view in &self.0 {
            view.adjust_width(width);
        }
    }

    #[inline(always)]
    fn adjust_height(&self, height: f32) {
        for view in &self.0 {
            view.adjust_height(height);
        }
    }

    #[inline(always)]
    fn frac(&self) -> usize {
        self.0.iter().fold(0, |n, view| n + view.frac())
    }
}
