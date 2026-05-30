use crate::{Bounds, Event, Offsets, Output, Size, View};

pub struct Padding<V> {
    pub view: V,
    pub offsets: Offsets,
}

impl<V: View> View for Padding<V> {
    #[inline]
    fn size(&self, within: Size) -> Size {
        let mut size = self.view.size(within);
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

    #[inline]
    fn adjust_width(&self, width: f32) {
        self.view.adjust_width(width);
    }

    #[inline]
    fn adjust_height(&self, height: f32) {
        self.view.adjust_height(height);
    }
}

/// Typed constructors for use withing the crate.
#[allow(dead_code)]
impl<V: View> Padding<V> {
    fn new(view: V, top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            offsets: Offsets::new(top, right, bottom, left),
            view,
        }
    }

    pub(crate) fn top(pad: f32, view: V) -> Self {
        Self::new(view, pad, 0.0, 0.0, 0.0)
    }

    pub(crate) fn right(pad: f32, view: V) -> Self {
        Self::new(view, 0.0, pad, 0.0, 0.0)
    }

    pub(crate) fn bottom(pad: f32, view: V) -> Self {
        Self::new(view, 0.0, 0.0, pad, 0.0)
    }

    pub(crate) fn left(pad: f32, view: V) -> Self {
        Self::new(view, 0.0, 0.0, 0.0, pad)
    }

    pub(crate) fn horizontal(pad: f32, view: V) -> Self {
        Self::new(view, 0.0, pad, 0.0, pad)
    }

    pub(crate) fn vertical(pad: f32, view: V) -> Self {
        Self::new(view, pad, 0.0, pad, 0.0)
    }
}
