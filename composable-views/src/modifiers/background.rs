use crate::{Bounds, Event, Output, Path, Size, View};

pub struct Background<V, P> {
    pub view: V,
    pub background: P,
}

impl<V: View, P: Path> View for Background<V, P> {
    #[inline(always)]
    fn size(&self, within: Size) -> Size {
        self.view.size(within)
    }

    #[inline]
    fn event(&self, event: Event, bounds: Bounds) {
        self.view.event(event, bounds)
    }

    #[inline]
    fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
        // since the background is drawn first, adjusts size, if necessary
        let size = self.view.size(bounds.size());

        let shape = self.background.clone().fill();
        shape.draw(Bounds::from_origin_and_size(bounds.min, size), onto);

        self.view.draw(bounds, onto);
    }

    #[inline(always)]
    fn adjust_width(&self, width: f32) {
        self.view.adjust_width(width);
    }

    #[inline(always)]
    fn adjust_height(&self, height: f32) {
        self.view.adjust_height(height);
    }

    #[inline(always)]
    fn frac(&self) -> usize {
        self.view.frac()
    }
}
