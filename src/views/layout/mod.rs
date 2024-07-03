#![allow(unused_imports)]
use crate::views::{Bounds, Event, Output, Point, Size, View}; // some of these are used in the macro

pub use spacing::Spacer;

mod spacing;

#[doc(hidden)]
pub trait Layout {
    fn size(size: Size, next: Size) -> Size;
    fn bounds(bounds: Bounds, previous: Size) -> Bounds;
    fn space(n: u32, required: Size, available: Size) -> Size;
}

struct Vertical;

impl Layout for Vertical {
    fn size(size: Size, next: Size) -> Size {
        Size::new(f32::max(size.width, next.width), size.height + next.height)
    }

    fn bounds(mut bounds: Bounds, previous: Size) -> Bounds {
        bounds.min.y += previous.height;
        bounds.min.y = f32::min(bounds.min.y, bounds.max.y);
        bounds
    }

    fn space(n: u32, required: Size, mut available: Size) -> Size {
        let surplus = available.height - required.height;

        if surplus <= 0.0 {
            Size::zero()
        } else {
            available.height = surplus / n as f32;
            available.width = 0.0;
            available
        }
    }
}

struct Horizontal;

impl Layout for Horizontal {
    fn size(size: Size, next: Size) -> Size {
        Size::new(size.width + next.width, f32::max(size.height, next.height))
    }

    fn bounds(mut bounds: Bounds, previous: Size) -> Bounds {
        bounds.min.x += previous.width;
        bounds.min.x = f32::min(bounds.min.x, bounds.max.x);
        bounds
    }

    fn space(n: u32, required: Size, mut available: Size) -> Size {
        let surplus = available.width - required.width;

        if surplus <= 0.0 {
            Size::zero()
        } else {
            available.width = surplus / n as f32;
            available.height = 0.0;
            available
        }
    }
}

///
#[doc(hidden)]
struct Row<T>(T);

macro_rules! tuple_impl {
    ( $($val:ident)+ ) => {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        impl<$($val: View),+> View for ( $($val,)+ ) {
            #[inline]
            fn size(&self) -> Size {
                let ( $(ref $val,)+ ) = self;
                let size = Size::zero();

                $(
                    let size = Vertical::size(size, $val.size());
                )+

                size
            }

            #[inline]
            fn event(&self, event: Event, offset: Point, bounds: Bounds) {
                self.update_layout(self.size(), bounds);

                let ( $(ref $val,)+ ) = self;
                $(
                    let size = $val.size();
                    $val.event(event, offset, bounds);
                    let bounds = Vertical::bounds(bounds, size);
                )+
            }

            #[inline]
            fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
                self.update_layout(self.size(), bounds);

                let ( $(ref $val,)+ ) = self;
                $(
                    let size = $val.size();
                    $val.draw(bounds, onto);
                    let bounds = Vertical::bounds(bounds, size);
                )+
            }

            fn update_layout(&self, size: Size, bounds: Bounds) {
                let ( $(ref $val,)+ ) = self;
                let mut size = Size::zero();
                let mut n = 0;

                $(
                    size = Vertical::size(size, $val.size());
                    n += $val.needs_layout() as u32;
                )+

                if n != 0 {
                    let space = Vertical::space(n, size, bounds.size());
                    $(
                        if $val.needs_layout() {
                            $val.update_layout(space, bounds);
                        }
                    )+;
                }
            }

            #[inline(always)]
            fn across(self) -> impl View {
                Row(self)
            }
        }

        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        impl<$($val: View),+> View for Row<( $($val,)+ )> {
            fn size(&self) -> Size {
                let ( $(ref $val,)+ ) = self.0;
                let size = Size::zero();

                $(
                    let size = Horizontal::size(size, $val.size());
                )+

                size
            }

            fn event(&self, event: Event, offset: Point, bounds: Bounds) {
                let ( $(ref $val,)+ ) = self.0;

                $(
                    let size = $val.size();
                    $val.event(event, offset, bounds);
                    let bounds = Horizontal::bounds(bounds, size);
                )+
            }

            fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
                let ( $(ref $val,)+ ) = self.0;

                $(
                    let size = $val.size();
                    $val.draw(bounds, onto);
                    let bounds = Horizontal::bounds(bounds, size);
                )+
            }

            #[inline(always)]
            fn needs_layout(&self) -> bool {
                self.0.needs_layout()
            }

            #[inline(always)]
            fn update_layout(&self, size: Size, bounds: Bounds) {
                let ( $(ref $val,)+ ) = self.0;
                let size = Size::zero();
                let n = 0;

                $(
                    let size = Horizontal::size(size, $val.size());
                    let n = n + $val.needs_layout() as u32;
                )+

                if n != 0 {
                    let space = Horizontal::space(n, size, bounds.size());
                    $(
                        if $val.needs_layout() {
                            $val.update_layout(space, bounds);
                        }
                    )+;
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
