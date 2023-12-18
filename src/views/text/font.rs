use rustybuzz::ttf_parser::name_id::{FAMILY, FULL_NAME, SUBFAMILY, UNIQUE_ID, VERSION};
use rustybuzz::{shape_with_plan, Face, ShapePlan, UnicodeBuffer};
use rustybuzz::{ttf_parser::GlyphId, ttf_parser::OutlineBuilder};
pub use rustybuzz::{Direction, GlyphBuffer as Glyphs, Language, Script};

use crate::views::{Layer, Text};

/// Font data
pub struct Font<'a> {
    face: Face<'a>,
    plan: ShapePlan,
    size: f32,
}

impl<'a> Font<'a> {
    ///
    pub fn new(
        data: &'a [u8],
        direction: Option<Direction>,
        script: Option<Script>,
        language: Option<&Language>,
        weight: f32,
        size: f32,
    ) -> Option<Self> {
        Self::from_collection(data, direction, script, language, weight, size, 0)
    }

    ///
    pub fn from_collection(
        data: &'a [u8],
        direction: Option<Direction>,
        script: Option<Script>,
        language: Option<&Language>,
        weight: f32,
        size: f32,
        index: u32,
    ) -> Option<Self> {
        let mut face = Face::from_slice(data, index)?;

        for axis in face.variation_axes().into_iter() {
            match &axis.tag.to_bytes() {
                b"opsz" => {
                    let opsz = size.clamp(axis.min_value, axis.max_value);
                    face.set_variation(axis.tag, opsz);
                }
                b"wght" => {
                    let wght = weight.clamp(axis.min_value, axis.max_value);
                    face.set_variation(axis.tag, wght);
                }
                _ => {}
            }
        }

        let plan = ShapePlan::new(
            &face,
            direction.unwrap_or(Direction::LeftToRight),
            script,
            language,
            &[], // TODO: support font features
        );

        Some(Self { face, plan, size })
    }

    /// Full font name that reflects all family and relevant subfamily descriptors.
    pub fn full_name(&self) -> Option<String> {
        self.name(FULL_NAME)
    }

    /// Family name.
    pub fn family(&self) -> Option<String> {
        self.name(FAMILY)
    }

    /// Subfamily name.
    pub fn style(&self) -> Option<String> {
        self.name(SUBFAMILY)
    }

    /// Unique font identifier
    pub fn identifier(&self) -> Option<String> {
        self.name(UNIQUE_ID)
    }

    /// Should begin with the syntax “Version _N_._M_”
    /// (upper case, lower case, or mixed, with a space between “Version” and the number).
    pub fn version(&self) -> Option<String> {
        self.name(VERSION)
    }

    /// Font size in points.
    pub fn size(&self) -> f32 {
        self.size
    }

    fn name(&self, id: u16) -> Option<String> {
        self.face
            .names()
            .into_iter()
            .find(|name| name.name_id == id)
            .and_then(|name| name.to_string())
    }

    /// Returns a `Text` in this font.
    pub fn text(&self, rgba: [u8; 4], string: &str) -> Text {
        let mut unicode = UnicodeBuffer::new();
        unicode.push_str(string);

        let glyphs = shape_with_plan(&self.face, &self.plan, unicode);
        let scale = self.size / self.face.units_per_em() as f32;

        let width = glyphs
            .glyph_positions()
            .iter()
            .fold(0.0, |width, position| {
                width + (position.x_offset + position.x_advance) as f32 * scale
            });

        let height = self
            .face
            .capital_height()
            .unwrap_or_else(|| self.face.ascender()) as f32
            * scale;

        Text {
            font: self,
            glyphs,
            height,
            width,
            scale,
            rgba,
        }
    }

    /// Used by [`View`]s to layout their [`Text`].
    ///
    /// The `Text` should have been created with this `Font`.
    ///
    /// [`View`]: crate::views::View
    pub fn layout(&self, text: &Text, x: f32, y: f32, onto: &mut impl FnMut(Layer)) {
        use lyon::math::Transform;

        struct Layout<'a, F: FnMut(Layer)> {
            transform: Transform,
            face: &'a Face<'a>,
            glyphs: &'a Glyphs,
            rgba: [u8; 4],
            onto: &'a mut F,
        }

        impl<'a, F: FnMut(Layer)> Layout<'a, F> {
            fn outline_glyphs(&mut self) {
                let positions = self.glyphs.glyph_positions().iter();
                let glyphs = self.glyphs.glyph_infos().iter();

                for (glyph, position) in Iterator::zip(glyphs, positions) {
                    self.transform = self
                        .transform // “How much the glyph moves on the [X/Y]-axis before drawing it”
                        .pre_translate((position.x_offset as f32, position.y_offset as f32).into());

                    self.face
                        .outline_glyph(GlyphId(glyph.glyph_id as u16), self);

                    self.transform = self
                        .transform // “How much the line advances after drawing this glyph”
                        .pre_translate(
                            (position.x_advance as f32, position.y_advance as f32).into(),
                        );
                }
            }
        }

        impl<F: FnMut(Layer)> OutlineBuilder for Layout<'_, F> {
            fn move_to(&mut self, x: f32, y: f32) {
                #[rustfmt::skip]
                (self.onto)(Layer::Move { x, y, rgba: self.rgba });
            }

            fn line_to(&mut self, x: f32, y: f32) {
                (self.onto)(Layer::Line { x, y });
            }

            fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
                (self.onto)(Layer::Quadratic { x1, y1, x, y });
            }

            fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
                #[rustfmt::skip]
                (self.onto)(Layer::Cubic { x1, y1, x2, y2, x, y });
            }

            fn close(&mut self) {
                (self.onto)(Layer::Close);
            }
        }

        let transform = Transform::scale(text.scale, -text.scale)
            .then_translate((0.0, text.height).into())
            .then_translate((x, y).into());

        let mut layout = Layout {
            transform,
            face: &self.face,
            glyphs: &text.glyphs,
            rgba: text.rgba,
            onto,
        };

        layout.outline_glyphs();
    }
}
