use crate::{Bounds, Event, Output, Size, View};

///
pub struct Fixed<V> {
    pub view: V,
    pub size: Size,
}

impl<V: View> View for Fixed<V> {
    #[inline]
    fn size(&self, mut bounds: Bounds) -> Size {
        bounds.set_size(self.size);
        self.view.size(bounds);

        self.size
    }

    #[inline]
    fn event(&self, event: Event, mut bounds: Bounds) {
        bounds.set_size(self.size);
        self.view.event(event, bounds)
    }

    #[inline]
    fn draw(&self, mut bounds: Bounds, onto: &mut impl Output) {
        bounds.set_size(self.size);
        self.view.draw(bounds, onto)
    }
}

///
pub struct FixedWidth<V: View> {
    pub view: V,
    pub width: f32,
}

impl<V: View> View for FixedWidth<V> {
    #[inline]
    fn size(&self, mut bounds: Bounds) -> Size {
        let mut size = bounds.size();
        size.width = self.width;
        bounds.set_size(size);

        size = self.view.size(bounds);
        size.width = self.width;

        size
    }

    #[inline]
    fn event(&self, event: Event, mut bounds: Bounds) {
        let mut size = bounds.size();
        size.width = self.width;
        bounds.set_size(size);

        self.view.event(event, bounds)
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
    fn size(&self, mut bounds: Bounds) -> Size {
        let mut size = bounds.size();
        size.height = self.height;
        bounds.set_size(size);

        size = self.view.size(bounds);
        size.height = self.height;

        size
    }

    #[inline]
    fn event(&self, event: Event, mut bounds: Bounds) {
        let mut size = bounds.size();
        size.height = self.height;
        bounds.set_size(size);

        self.view.event(event, bounds)
    }

    #[inline]
    fn draw(&self, mut bounds: Bounds, onto: &mut impl Output) {
        let mut size = bounds.size();
        size.height = self.height;
        bounds.set_size(size);

        self.view.draw(bounds, onto)
    }

    #[inline]
    fn adjust_width(&self, width: f32) {
        self.view.adjust_width(width);
    }

    #[inline]
    fn adjust_height(&self, height: f32) {
        self.view.adjust_height(height);
    }
}
