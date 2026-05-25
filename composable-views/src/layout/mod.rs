#![allow(unused_assignments)]
#![allow(unused_imports)] // some of these are used in the macro
use crate::{Bounds, Event, Fixed, FixedHeight, FixedWidth, Output, Size, View, warn_once};

pub use spacing::Spacer;

// mod alignment;
mod spacing;

// pub enum Edge {
//     Leading,
//     Center,
//     Trailing,
//     Top,
//     Middle,
//     Bottom,
// }
//
// #[doc(hidden)]
// struct Horizontal<T>(T);
//
// macro_rules! tuple_impl {
//     ( $($val:ident)+ ) => {
//         #[doc(hidden)]
//         #[allow(non_snake_case)]
//         #[allow(unused_variables)]
//         impl<$($val: View),+> View for ( $($val,)+ ) {
//             #[inline]
//             fn size(&self) -> Size {
//                 let &( $(ref $val,)+ ) = self;
//
//                 let mut size = Size::zero();
//                 $(
//                     let next = $val.size();
//                     size = Size::new(f32::max(size.width, next.width), size.height + next.height);
//                 )+
//
//                 size
//             }
//
//             #[inline]
//             fn event(&self, event: Event, mut bounds: Bounds) {
//                 self.update_width(self.size().width, bounds);
//                 self.update_height(self.size().height, bounds);
//
//                 let &( $(ref $val,)+ ) = self;
//                 $(
//                     $val.event(event.clone(), bounds);
//                     bounds.min.y += $val.size().height;
//                     bounds.min.y = f32::min(bounds.min.y, bounds.max.y);
//                 )+
//             }
//
//             #[inline]
//             fn draw(&self, mut bounds: Bounds, onto: &mut impl Output) {
//                 self.update_width(self.size().width, bounds);
//                 self.update_height(self.size().height, bounds);
//
//                 let &( $(ref $val,)+ ) = self;
//                 $(
//                     $val.draw(bounds, onto);
//                     bounds.min.y += $val.size().height;
//                     bounds.min.y = f32::min(bounds.min.y, bounds.max.y);
//                 )+
//             }
//
//             #[inline(always)]
//             #[allow(refining_impl_trait)]
//             fn fixed(self, width: f32, height: f32) -> impl View {
//                 Fixed {
//                     size: Size::new(width, height),
//                     view: self,
//                 }
//             }
//
//             #[inline(always)]
//             #[allow(refining_impl_trait)]
//             fn width(self, width: f32) -> impl View {
//                 FixedWidth { width, view: self }
//             }
//
//             #[inline(always)]
//             #[allow(refining_impl_trait)]
//             fn height(self, height: f32) -> impl View {
//                 FixedHeight { height, view: self }
//             }
//
//             fn update_width(&self, current: f32, bounds: Bounds) {
//                 // let &( $( $val, )+ ) = &self;
//                 //
//                 // let mut n = 0;
//                 // $( n += $val.has_unknown_width() as u32;)+
//                 //
//                 // if n != 0 {
//                 //     let mut width = 0.0;
//                 //     $( width += $val.size().width; )+
//                 //
//                 //     let space = f32::max((bounds.width() - width) / n as f32, 0.0);
//                 //     $( $val.update_width(space, bounds); )+
//                 // }
//             }
//
//             fn update_height(&self, current: f32, bounds: Bounds) {
//                 let &( $( $val, )+ ) = &self;
//
//                 let mut n = 0;
//                 $( n += $val.has_unknown_height() as u32;)+
//
//                 if n != 0 {
//                     let mut height = 0.0;
//                     $( height += $val.size().height; )+
//
//                     let space = f32::max((bounds.height() - height) / n as f32, 0.0);
//                     $( $val.update_height(space, bounds); )+
//                 }
//             }
//
//             fn has_unknown_height(&self) -> bool {
//                 let &( $(ref $val,)+ ) = self;
//
//                 let mut n = 0;
//                 $( n += $val.has_unknown_height() as u32; )+
//
//                 n != 0
//             }
//
//             #[inline(always)]
//             fn across(self) -> impl View {
//                 Horizontal(self)
//             }
//         }
//
//         #[doc(hidden)]
//         #[allow(non_snake_case)]
//         #[allow(unused_variables)]
//         impl<$($val: View),+> View for Horizontal<( $($val,)+ )> {
//             #[inline]
//             fn size(&self) -> Size {
//                 let &( $(ref $val,)+ ) = &self.0;
//
//                 let mut size = Size::zero();
//                 $(
//                     let next = $val.size();
//                     size = Size::new(size.width + next.width, f32::max(size.height, next.height));
//                 )+
//
//                 size
//             }
//
//             #[inline]
//             fn event(&self, event: Event, mut bounds: Bounds) {
//                 // self.update_width(self.size().width, bounds);
//                 // self.update_height(self.size().height, bounds);
//
//                 let &( $(ref $val,)+ ) = &self.0;
//                 $(
//                     $val.event(event.clone(), bounds);
//                     bounds.min.x += $val.size().width;
//                     bounds.min.x = f32::min(bounds.min.x, bounds.max.x);
//                 )+
//             }
//
//             #[inline]
//             fn draw(&self, mut bounds: Bounds, onto: &mut impl Output) {
//                 // self.update_width(self.size().width, bounds);
//                 // self.update_height(self.size().height, bounds);
//
//                 let &( $(ref $val,)+ ) = &self.0;
//                 $(
//                     $val.draw(bounds, onto);
//                     bounds.min.x += $val.size().width;
//                     bounds.min.x = f32::min(bounds.min.x, bounds.max.x);
//                 )+
//             }
//
//             #[inline(always)]
//             fn fixed(self, width: f32, height: f32) -> impl View {
//                 Fixed {
//                     size: Size::new(width, height),
//                     view: self,
//                 }
//             }
//
//             #[inline(always)]
//             fn width(self, width: f32) -> impl View {
//                 FixedWidth { width, view: self }
//             }
//
//             #[inline(always)]
//             fn height(self, height: f32) -> impl View {
//                 FixedHeight { height, view: self }
//             }
//
//             #[inline(always)]
//             fn has_unknown_width(&self) -> bool {
//                 self.0.has_unknown_width()
//             }
//
//             fn update_width(&self, current: f32, bounds: Bounds) {
//                 let &( $(ref $val,)+ ) = &self.0;
//
//                 let mut n = 0;
//                 $( n += $val.has_unknown_width() as u32;)+
//
//                 if n != 0 {
//                     let mut width = 0.0;
//                     $( width += $val.size().width; )+
//
//                     let space = f32::max((bounds.width() - width) / n as f32, 0.0);
//                     $( $val.update_width(space, bounds); )+
//                 }
//             }
//
//             fn update_height(&self, current: f32, bounds: Bounds) {
//                 // let &( $(ref $val, )+ ) = &self.0;
//                 //
//                 // let mut n = 0;
//                 // $( n += $val.has_unknown_height() as u32;)+
//                 //
//                 // if n != 0 {
//                 //     let mut height = 0.0;
//                 //     $( height += $val.size().height; )+
//                 //
//                 //     let space = f32::max((bounds.height() - height) / n as f32, 0.0);
//                 //     $( $val.update_height(space, bounds); )+
//                 // }
//             }
//         }
//     };
// }

// tuple_impl! { A }
// tuple_impl! { A B }
// tuple_impl! { A B C }
// tuple_impl! { A B C D }
// tuple_impl! { A B C D E }
// tuple_impl! { A B C D E F }
// tuple_impl! { A B C D E F G }
// tuple_impl! { A B C D E F G H }
// tuple_impl! { A B C D E F G H I }
// tuple_impl! { A B C D E F G H I J }
// tuple_impl! { A B C D E F G H I J K }
// tuple_impl! { A B C D E F G H I J K L }
// tuple_impl! { A B C D E F G H I J K L M }
// tuple_impl! { A B C D E F G H I J K L M N }
// tuple_impl! { A B C D E F G H I J K L M N O }
// tuple_impl! { A B C D E F G H I J K L M N O P }
// tuple_impl! { A B C D E F G H I J K L M N O P Q }
// tuple_impl! { A B C D E F G H I J K L M N O P Q R }
// tuple_impl! { A B C D E F G H I J K L M N O P Q R S }
// tuple_impl! { A B C D E F G H I J K L M N O P Q R S T }
// tuple_impl! { A B C D E F G H I J K L M N O P Q R S T U }
// tuple_impl! { A B C D E F G H I J K L M N O P Q R S T U V }
// tuple_impl! { A B C D E F G H I J K L M N O P Q R S T U V W }
// tuple_impl! { A B C D E F G H I J K L M N O P Q R S T U V W X }
// tuple_impl! { A B C D E F G H I J K L M N O P Q R S T U V W X Y }
// // up to 25 views are supported
//
// pub trait Layout {
//     /// Adapts an array of `View`s to cascade horizontally; similar to a tuple.
//     ///
//     /// See [`View::across`].
//     #[allow(dead_code)]
//     fn across(self) -> impl View;
// }
//
// impl<V: View, const N: usize> Layout for [V; N] {
//     #[inline(always)]
//     fn across(self) -> impl View {
//         Horizontal(self)
//     }
// }
//
// impl<T: View, const N: usize> View for Horizontal<[T; N]> {
//     #[inline]
//     fn size(&self) -> Size {
//         self.0.iter().fold(Size::zero(), |mut size, view| {
//             let next = view.size();
//
//             size.width += next.width;
//             size.height = f32::max(size.height, next.height);
//             size
//         })
//     }
//
//     #[inline]
//     fn event(&self, event: Event, bounds: Bounds) {
//         self.0.iter().fold(bounds, |mut bounds, view| {
//             view.event(event.clone(), bounds);
//
//             bounds.min.x += view.size().width;
//             bounds.min.x = f32::min(bounds.min.x, bounds.max.x);
//             bounds
//         });
//     }
//
//     #[inline]
//     fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
//         self.0.iter().fold(bounds, |mut bounds, view| {
//             view.draw(bounds, onto);
//
//             bounds.min.x += view.size().width;
//             bounds.min.x = f32::min(bounds.min.x, bounds.max.x);
//             bounds
//         });
//     }
// }
