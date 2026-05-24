use std::cell::{Cell, OnceCell};

use crate::{Bounds, Output, Size, View};

pub struct Spacer(pub(crate) Cell<Size>);

impl Spacer {
    #[inline(always)]
    pub fn fill() -> Self {
        Self::fixed(f32::INFINITY, f32::INFINITY)
    }

    #[inline(always)]
    pub fn fixed(width: f32, height: f32) -> Self {
        Spacer(Size::new(width, height).into())
    }

    #[inline(always)]
    pub fn width(width: f32) -> Self {
        Spacer::fixed(width, 0.0)
    }

    #[inline(always)]
    pub fn height(height: f32) -> Self {
        Spacer::fixed(0.0, height)
    }

    #[inline(always)]
    pub fn empty() -> Self {
        Self::fixed(0.0, 0.0)
    }
}

#[allow(unused_variables)]
impl View for Spacer {
    #[inline]
    fn size(&self) -> Size {
        let size = self.0.get();

        match (size.width != f32::INFINITY, size.height != f32::INFINITY) {
            (true, true) => size,
            (false, false) => Size::zero(),
            (true, false) => Size::new(size.width, 0.0),
            (false, true) => Size::new(0.0, size.height),
        }
    }

    #[inline(always)]
    fn draw(&self, bounds: Bounds, onto: &mut impl Output) {}

    #[inline(always)]
    fn needs_layout_x(&self) -> bool {
        self.0.get().width == f32::INFINITY
    }

    #[inline(always)]
    fn needs_layout_y(&self) -> bool {
        self.0.get().height == f32::INFINITY
    }

    #[inline]
    fn update_layout(&self, size: Size, _bounds: Bounds) {
        self.0.set(size);
    }
}
