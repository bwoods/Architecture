use crate::views::{Bounds, Event, Output, Point, Size, View};

///
pub struct Fixed<V> {
    pub(crate) view: V,
    pub(crate) size: Size,
}

impl<V: View> View for Fixed<V> {
    #[inline(always)]
    fn size(&self) -> Size {
        self.size
    }

    #[inline]
    fn event(&self, event: Event, offset: Point, mut bounds: Bounds) {
        bounds.set_size(self.size);
        self.view.event(event, offset, bounds)
    }

    #[inline]
    fn draw(&self, mut bounds: Bounds, onto: &mut impl Output) {
        bounds.set_size(self.size);
        self.view.draw(bounds, onto)
    }
}

///
pub struct FixedWidth<V: View> {
    pub(crate) view: V,
    pub(crate) width: f32,
}

impl<V: View> View for FixedWidth<V> {
    #[inline]
    fn size(&self) -> Size {
        let mut size = self.view.size();
        size.width = self.width;

        size
    }

    #[inline]
    fn event(&self, event: Event, offset: Point, mut bounds: Bounds) {
        let mut size = bounds.size();
        size.width = self.width;
        bounds.set_size(size);

        self.view.event(event, offset, bounds)
    }

    #[inline]
    fn draw(&self, mut bounds: Bounds, onto: &mut impl Output) {
        let mut size = bounds.size();
        size.width = self.width;
        bounds.set_size(size);

        self.view.draw(bounds, onto)
    }
}

///
pub struct FixedHeight<V: View> {
    pub(crate) view: V,
    pub(crate) height: f32,
}

impl<V: View> View for FixedHeight<V> {
    #[inline]
    fn size(&self) -> Size {
        let mut size = self.view.size();
        size.height = self.height;

        size
    }

    #[inline]
    fn event(&self, event: Event, offset: Point, mut bounds: Bounds) {
        let mut size = bounds.size();
        size.height = self.height;
        bounds.set_size(size);

        self.view.event(event, offset, bounds)
    }

    #[inline]
    fn draw(&self, mut bounds: Bounds, onto: &mut impl Output) {
        let mut size = bounds.size();
        size.height = self.height;
        bounds.set_size(size);

        self.view.draw(bounds, onto)
    }
}
