use crate::{Bounds, Event, Size, Transform, View};

pub struct Opacity<V> {
    pub view: V,
    pub alpha: f32,
}

impl<V: View> View for Opacity<V> {
    fn size(&self, within: Size) -> Size {
        self.view.size(within)
    }

    fn event(&self, event: Event, bounds: Bounds) {
        self.view.event(event, bounds)
    }

    fn draw(&self, bounds: Bounds, onto: &mut impl crate::Output) {
        self.view.draw(
            bounds,
            &mut Output {
                output: onto,
                alpha: self.alpha,
            },
        )
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

struct Output<'a, T> {
    output: &'a mut T,
    alpha: f32,
}

impl<T: crate::Output> crate::Output for Output<'_, T> {
    fn begin(&mut self, mut rgba: [u8; 4], transform: &Transform) {
        debug_assert!(self.alpha >= 0.0 && self.alpha <= 1.0);
        rgba[3] = ((rgba[3] as f32 * 255.0 * self.alpha) / 255.0) as u8;

        self.output.begin(rgba, transform)
    }

    fn move_to(&mut self, x: f32, y: f32) {
        self.output.move_to(x, y)
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.output.line_to(x, y)
    }

    fn quadratic_bezier_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.output.quadratic_bezier_to(x1, y1, x, y)
    }

    fn cubic_bezier_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.output.cubic_bezier_to(x1, y1, x2, y2, x, y)
    }

    fn close(&mut self) {
        self.output.close()
    }
}
