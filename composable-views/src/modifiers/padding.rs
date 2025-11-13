use crate::{Bounds, Event, Offsets, Output, Size, View};

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
    fn event(&self, event: Event, bounds: Bounds) {
        self.view.event(event, bounds.inner_box(self.offsets))
    }

    #[inline]
    fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
        self.view.draw(bounds.inner_box(self.offsets), onto)
    }
}
