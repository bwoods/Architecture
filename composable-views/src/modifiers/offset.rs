use crate::{Bounds, Event, Output, Point, Size, View};

pub struct Offset<V> {
    pub(crate) view: V,
    pub(crate) offsets: Point,
}

impl<V: View> View for Offset<V> {
    #[inline(always)]
    fn size(&self) -> Size {
        self.view.size()
    }

    #[inline]
    fn event(&self, event: Event, offset: Point, mut bounds: Bounds) {
        bounds.min += self.offsets.to_vector();
        self.view.event(event, offset, bounds)
    }

    #[inline]
    fn draw(&self, mut bounds: Bounds, onto: &mut impl Output) {
        bounds.min += self.offsets.to_vector();
        self.view.draw(bounds, onto)
    }
}
