use rustybuzz::ttf_parser::name_id::{FAMILY, FULL_NAME, SUBFAMILY, UNIQUE_ID, VERSION};
use rustybuzz::ttf_parser::{GlyphId, OutlineBuilder, Tag};
use rustybuzz::{shape_with_plan, Face, ShapePlan, UnicodeBuffer};
pub use rustybuzz::{Direction, Feature, GlyphBuffer as Glyphs, Language, Script};

use crate::views::{Layer, Text};

///
pub struct Font<'a> {
    face: Face<'a>,
    plan: ShapePlan,
    size: f32,
}

impl<'a> Font<'a> {
    /// Full font name that reflects all family and relevant subfamily descriptors.
    #[inline]
    pub fn full_name(&self) -> Option<String> {
        self.name(FULL_NAME)
    }

    /// Family name.
    #[inline]
    pub fn family(&self) -> Option<String> {
        self.name(FAMILY)
    }

    /// Subfamily name.
    #[inline]
    pub fn style(&self) -> Option<String> {
        self.name(SUBFAMILY)
    }

    /// Unique font identifier
    #[inline]
    pub fn identifier(&self) -> Option<String> {
        self.name(UNIQUE_ID)
    }

    /// Should begin with the syntax “Version _N_._M_”
    /// (upper case, lower case, or mixed, with a space between “Version” and the number).
    #[inline]
    pub fn version(&self) -> Option<String> {
        self.name(VERSION)
    }

    /// Font size in points.
    #[inline]
    pub fn size(&self) -> f32 {
        self.size
    }

    #[inline(never)]
    fn name(&self, id: u16) -> Option<String> {
        self.face
            .names()
            .into_iter()
            .find(|name| name.name_id == id)
            .and_then(|name| name.to_string())
    }

    /// Returns a `Text` in this font.
    #[inline(never)]
    pub fn text(&self, rgba: [u8; 4], string: &str) -> Text {
        let mut unicode = UnicodeBuffer::new();
        unicode.push_str(string);

        let glyphs = shape_with_plan(&self.face, &self.plan, unicode);
        let scale = self.size / self.face.units_per_em() as f32;

        // TODO: both of these assume Direction::LeftToRight or RightToLeft
        let width = glyphs
            .glyph_positions()
            .iter()
            .fold(0.0, |width, position| {
                width + (position.x_offset + position.x_advance) as f32
            })
            * scale;

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
    #[inline(never)]
    pub fn layout(&self, text: &Text, x: f32, y: f32, onto: &mut impl FnMut(Layer)) {
        use lyon::math::Transform;

        struct Layout<'a, F: FnMut(Layer)> {
            transform: Transform,
            glyphs: &'a Glyphs,
            face: &'a Face<'a>,
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

        let transform = Transform::scale(text.scale, -text.scale) // negate y-axis
            .then_translate((0.0, text.height).into())
            .then_translate((x, y).into());

        let mut layout = Layout {
            transform,
            glyphs: &text.glyphs,
            face: &self.face,
            rgba: text.rgba,
            onto,
        };

        layout.outline_glyphs();
    }
}

impl<'a> Font<'a> {
    /// Create a `Font` from the raw font data.
    #[inline(always)]
    pub fn from(data: &'a [u8]) -> Option<FontConfig> {
        Self::from_collection(data, 0)
    }

    /// Create a `Font` from a font collection.
    /// Returns the font at `index`, if any
    #[inline(never)]
    pub fn from_collection(data: &'a [u8], index: u32) -> Option<FontConfig> {
        let face = Face::from_slice(data, index)?;

        Some(FontConfig {
            face,
            features: Vec::default(),
            direction: None,
            script: None,
            language: None,
            weight: None,
        })
    }
}

///
#[derive(Clone)]
pub struct FontConfig<'a> {
    face: Face<'a>,
    features: Vec<Feature>,
    direction: Option<Direction>,
    script: Option<Script>,
    language: Option<Language>,
    weight: Option<f32>,
}

impl<'a> FontConfig<'a> {
    ///
    #[inline]
    pub fn direction(self, direction: Direction) -> Self {
        Self {
            direction: Some(direction),
            ..self
        }
    }

    ///
    #[inline]
    pub fn script(self, script: Script) -> Self {
        Self {
            script: Some(script),
            ..self
        }
    }

    ///
    #[inline]
    pub fn language(self, language: Language) -> Self {
        Self {
            language: Some(language),
            ..self
        }
    }

    ///
    #[inline]
    pub fn feature(mut self, tag: &[u8; 4], value: u32) -> Self {
        self.features
            .push(Feature::new(Tag::from_bytes(tag), value, ..));

        self
    }

    ///
    #[inline]
    pub fn weight(self, weight: f32) -> Self {
        Self {
            weight: Some(weight),
            ..self
        }
    }

    /// The final step in building a Font.
    #[inline(never)]
    pub fn size(mut self, size: f32) -> Font<'a> {
        for axis in self.face.variation_axes().into_iter() {
            match &axis.tag.to_bytes() {
                b"opsz" => {
                    let opsz = size.clamp(axis.min_value, axis.max_value);
                    self.face.set_variation(axis.tag, opsz);
                }
                b"wght" => {
                    let wght = self
                        .weight
                        .map(|w| w.clamp(axis.min_value, axis.max_value))
                        .unwrap_or(axis.def_value);

                    self.face.set_variation(axis.tag, wght);
                }
                _ => {}
            }
        }

        let direction = self.direction.unwrap_or(Direction::LeftToRight);
        // Using direction.unwrap_or_default() would give an Direction::Invalid
        // and that will panic!() in ShapePlan::new()

        let plan = ShapePlan::new(
            &self.face,
            direction,
            self.script,
            self.language.as_ref(),
            &self.features,
        );

        Font {
            face: self.face,
            plan,
            size,
        }
    }
}
