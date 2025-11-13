use crate::{Bounds, Event, Output, Path, Size, View};

///
pub struct Background<V, P> {
    pub(crate) view: V,
    pub(crate) background: P,
}

impl<V: View, P: Path> View for Background<V, P> {
    #[inline(always)]
    fn size(&self) -> Size {
        self.view.size()
    }

    #[inline]
    fn event(&self, event: Event, bounds: Bounds) {
        self.view.event(event, bounds)
    }

    #[inline]
    fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
        // since the background is drawn first, it needs the view’s layout to be up to date.
        self.view.update_layout(self.view.size(), bounds);

        let shape = self.background.clone().fill();
        shape.draw(
            Bounds::from_origin_and_size(bounds.min, self.view.size()),
            onto,
        );

        self.view.draw(bounds, onto);
    }
}
