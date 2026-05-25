use crate::{Bounds, Fixed, FixedHeight, FixedWidth, Output, Size, View};
use std::cell::{Cell, OnceCell};

pub struct Spacer(pub(crate) Cell<Size>);

impl Spacer {
    #[inline(always)]
    pub fn fill() -> impl View {
        Spacer(Size::splat(f32::INFINITY).into())
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
    fn size(&self, bounds: Bounds) -> Size {
        let size = self.0.get();

        match (size.width != f32::INFINITY, size.height != f32::INFINITY) {
            (true, true) => size,
            (false, false) => bounds.size(),
            (true, false) => Size::new(size.width, bounds.height()),
            (false, true) => Size::new(bounds.width(), size.height),
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
