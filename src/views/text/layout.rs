use crate::views::Layer;

use lyon::math::{Point, Transform};
use lyon::path::builder::{PathBuilder, WithSvg};
use lyon::path::{Attributes, EndpointId as Id};

use rustybuzz::ttf_parser::OutlineBuilder;

/// Used by [`View`]s to layout their [`Text`].
///
/// The `Text` should have been created with this `Font`.
///
/// [`View`]: crate::views::View
pub(crate) struct Builder<'a, F: FnMut(Layer)> {
    transform: &'a Transform,
    onto: &'a mut F,
    rgba: [u8; 4],
}

impl<'a, F: FnMut(Layer)> PathBuilder for Builder<'a, F> {
    fn num_attributes(&self) -> usize {
        0
    }

    fn begin(&mut self, at: Point, _attr: Attributes) -> Id {
        let (x, y) = self.transform.transform_point(at).into();

        #[rustfmt::skip]
        (self.onto)(Layer::Begin { x, y, rgba: self.rgba });
        Id::INVALID
    }

    fn end(&mut self, close: bool) {
        (self.onto)(Layer::End { close });
    }

    fn line_to(&mut self, to: Point, _attr: Attributes) -> Id {
        let (x, y) = self.transform.transform_point(to).into();

        (self.onto)(Layer::Line { x, y });
        Id::INVALID
    }

    fn quadratic_bezier_to(&mut self, ctrl: Point, to: Point, _attr: Attributes) -> Id {
        let (x1, y1) = self.transform.transform_point(ctrl).into();
        let (x, y) = self.transform.transform_point(to).into();

        (self.onto)(Layer::Quadratic { x1, y1, x, y });
        Id::INVALID
    }

    fn cubic_bezier_to(&mut self, ctrl1: Point, ctrl2: Point, to: Point, _attr: Attributes) -> Id {
        let (x1, y1) = self.transform.transform_point(ctrl1).into();
        let (x2, y2) = self.transform.transform_point(ctrl2).into();
        let (x, y) = self.transform.transform_point(to).into();

        #[rustfmt::skip]
        (self.onto)(Layer::Cubic { x1, y1, x2, y2, x, y });
        Id::INVALID
    }
}

pub(crate) struct Layout<'a, F: FnMut(Layer)>(WithSvg<Builder<'a, F>>);

impl<'a, F: FnMut(Layer)> Layout<'a, F> {
    pub fn new(transform: &'a Transform, rgba: [u8; 4], onto: &'a mut F) -> Self {
        Layout(
            Builder {
                transform,
                onto,
                rgba,
            }
            .with_svg(),
        )
    }
}

impl<'a, F: FnMut(Layer)> OutlineBuilder for Layout<'a, F> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.0.move_to((x, y).into());
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.0.line_to((x, y).into());
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.0.quadratic_bezier_to((x1, y1).into(), (x, y).into());
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.0
            .cubic_bezier_to((x1, y1).into(), (x2, y2).into(), (x, y).into());
    }

    fn close(&mut self) {
        self.0.close();
    }
}
