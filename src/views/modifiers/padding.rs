use crate::views::{Bounds, Event, Offsets, Output, Point, Size, View};

pub struct Padding<V> {
    pub(crate) view: V,
    pub(crate) offsets: Offsets,
}

impl<V: View> View for Padding<V> {
    #[inline]
    fn size(&self) -> Size {
        let mut size = self.view.size();
        size.width += self.offsets.horizontal();
        size.height += self.offsets.vertical();

        size
    }

    #[inline(always)]
    fn event(&self, event: Event, offset: Point, bounds: Bounds) {
        self.view.event(event, offset, bounds)
    }

    #[inline]
    fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
        self.view.draw(bounds.inner_box(self.offsets), onto)
    }
}
