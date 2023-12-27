pub use font::{Direction, Font, FontConfig, Glyphs, Language, Script};

use crate::views::{Bounds, Event, Layer, Point, Size, Transform, View};

mod font;

/// Text data
#[doc(hidden)] // documented as views::Text
pub struct Text<'a> {
    font: &'a Font<'a>,
    glyphs: Glyphs,
    height: f32,
    width: f32,
    scale: f32,
    rgba: [u8; 4],
}

impl View for Text<'_> {
    fn size(&self) -> Size {
        (self.width, self.height).into()
    }

    fn event(&self, _event: Event, _offset: Point, _bounds: Bounds) {}

    fn draw(&self, bounds: Bounds, onto: &mut impl FnMut(Layer)) {
        struct Builder<'a, F: FnMut(Layer)> {
            transform: Transform,
            onto: &'a mut F,
            rgba: [u8; 4],
        }

        impl<'a, F: FnMut(Layer)> ttf_parser::OutlineBuilder for Builder<'a, F> {
            fn move_to(&mut self, x: f32, y: f32) {
                let (x, y) = self.transform.transform_point((x, y).into()).into();

                #[rustfmt::skip]
                (self.onto)(Layer::Begin { x, y, rgba: self.rgba });
            }

            fn line_to(&mut self, x: f32, y: f32) {
                let (x, y) = self.transform.transform_point((x, y).into()).into();
                (self.onto)(Layer::Line { x, y });
            }

            fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
                let (x1, y1) = self.transform.transform_point((x1, y1).into()).into();
                let (x, y) = self.transform.transform_point((x, y).into()).into();

                (self.onto)(Layer::Quadratic { x1, y1, x, y });
            }

            fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
                let (x1, y1) = self.transform.transform_point((x1, y1).into()).into();
                let (x2, y2) = self.transform.transform_point((x2, y2).into()).into();
                let (x, y) = self.transform.transform_point((x, y).into()).into();

                #[rustfmt::skip]
                (self.onto)(Layer::Cubic { x1, y1, x2, y2, x, y });
            }

            fn close(&mut self) {
                (self.onto)(Layer::End { close: true });
            }
        }

        let mut builder = Builder {
            transform: Transform::scale(self.scale, -self.scale) // negate y-axis
                .then_translate((0.0, self.height).into()) // font baseline
                .then_translate(bounds.min.to_vector()), // start position,
            rgba: self.rgba,
            onto,
        };

        let positions = self.glyphs.glyph_positions().iter();
        let glyphs = self.glyphs.glyph_infos().iter();

        for (glyph, position) in Iterator::zip(glyphs, positions) {
            builder.transform = builder
                .transform // “How much the glyph moves on the [X/Y]-axis before drawing it”
                .pre_translate((position.x_offset as f32, position.y_offset as f32).into());

            self.font.outline_glyph(glyph.glyph_id, &mut builder);

            builder.transform = builder
                .transform // “How much the line advances after drawing this glyph”
                .pre_translate((position.x_advance as f32, position.y_advance as f32).into());
        }
    }
}
