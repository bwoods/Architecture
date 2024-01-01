use crate::views::{Bounds, Event, Output, Point, Size, View};

pub struct Empty;

impl View for Empty {
    #[inline(always)]
    fn size(&self) -> Size {
        Size::zero()
    }
    #[inline(always)]
    fn event(&self, _event: Event, _offset: Point, _bounds: Bounds) {}
    #[inline(always)]
    fn draw(&self, _bounds: Bounds, _onto: &mut impl Output) {}
}

pub struct Spacer(pub(crate) std::cell::Cell<Size>);

impl Spacer {
    #[inline(always)]
    pub fn empty() -> Empty {
        Empty
    }

    #[inline(always)]
    pub fn fill() -> Self {
        Spacer(Default::default())
    }
}
