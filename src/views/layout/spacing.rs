use std::cell::OnceCell;

use crate::views::{Bounds, Event, Output, Point, Size, View};

pub struct Spacer(pub(crate) OnceCell<Size>);

impl Spacer {
    #[inline(always)]
    pub fn fill() -> Self {
        Spacer(OnceCell::new()) // a flexible spacer has no size (yet)
    }

    #[inline]
    pub(crate) fn is_flexible(&self) -> bool {
        self.0.get().is_none()
    }

    #[inline]
    pub(crate) fn set_size(&self, size: Size) {
        self.0.set(size).expect("Size set twice")
    }

    #[inline(always)]
    pub fn fixed(width: f32, height: f32) -> Self {
        let spacer = Self::fill();
        spacer.0.set(Size::new(width, height)).ok();
        spacer
    }

    #[inline(always)]
    pub fn width(width: f32) -> Self {
        Spacer::fixed(width, 1.0)
    }

    #[inline(always)]
    pub fn height(height: f32) -> Self {
        Spacer::fixed(1.0, height)
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
        self.0.get().cloned().unwrap_or_default()
    }

    #[inline(always)]
    fn draw(&self, bounds: Bounds, onto: &mut impl Output) {}
}
