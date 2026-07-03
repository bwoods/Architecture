use crate::{Bounds, Event, Fixed, FixedHeight, FixedWidth, Output, Size, View};
use std::cell::Cell;

pub struct Spacer(pub(crate) Cell<Size>);

#[doc(hidden)]
impl Default for Spacer {
    fn default() -> Self {
        Spacer(Size::splat(f32::INFINITY).into())
    }
}

impl Spacer {
    #[inline(always)]
    pub fn fill() -> impl View {
        Spacer::default()
    }

    /// - `Spacer::frac::<1>()` is equivalent to `Spacer::fill()`.
    /// - `Spacer::frac::<3>()` acts more like three `Spacer::fill()` in its place.
    #[inline(always)]
    pub fn frac<const N: usize>() -> impl View {
        Frac::<_, N>(Spacer::fill())
    }

    #[inline(always)]
    pub fn fixed(width: f32, height: f32) -> impl View {
        let size = Size::new(width, height);
        Fixed { view: (), size }
    }

    #[inline(always)]
    pub fn width(width: f32) -> impl View {
        FixedWidth { view: (), width }
    }

    #[inline(always)]
    pub fn height(height: f32) -> impl View {
        FixedHeight { view: (), height }
    }

    #[inline(always)]
    pub fn empty() -> impl View {}
}

impl View for Spacer {
    #[inline]
    fn size(&self, within: Size) -> Size {
        let size = self.0.get();

        match (size.width != f32::INFINITY, size.height != f32::INFINITY) {
            (true, true) => size,
            (false, false) => within,
            (true, false) => Size::new(size.width, within.height),
            (false, true) => Size::new(within.width, size.height),
        }
    }

    #[inline(always)]
    fn draw(&self, _bounds: Bounds, _onto: &mut impl Output) {}

    fn adjust_width(&self, width: f32) {
        self.0.update(|mut size| {
            if size.width == f32::INFINITY {
                size.width = width;
            };

            size
        });
    }

    fn adjust_height(&self, height: f32) {
        self.0.update(|mut size| {
            if size.height == f32::INFINITY {
                size.height = height;
            };

            size
        });
    }
}

struct Frac<V, const N: usize>(V);

impl<V: View, const N: usize> View for Frac<V, N> {
    #[inline(always)]
    fn size(&self, within: Size) -> Size {
        self.0.size(within)
    }

    #[inline(always)]
    fn event(&self, event: Event, bounds: Bounds) {
        self.0.event(event, bounds)
    }

    #[inline(always)]
    fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
        self.0.draw(bounds, onto)
    }

    #[inline(always)]
    fn adjust_width(&self, width: f32) {
        self.0.adjust_width(width * N as f32)
    }

    #[inline(always)]
    fn adjust_height(&self, height: f32) {
        self.0.adjust_height(height * N as f32)
    }

    #[inline(always)]
    fn frac(&self) -> usize {
        N
    }
}
