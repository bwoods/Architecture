use rustybuzz::ttf_parser::name_id::{FAMILY, FULL_NAME, SUBFAMILY, UNIQUE_ID, VERSION};
use rustybuzz::ttf_parser::Tag;
use rustybuzz::{shape_with_plan, Face, ShapePlan, UnicodeBuffer};
pub use rustybuzz::{Direction, GlyphBuffer as Glyphs, Language, Script};

use crate::views::Text;

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

        // (clamp and) set supported/common variable-font axes
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

    /// Should begin with the syntax “Version <number>.<number>”
    /// (upper case, lower case, or mixed, with a space between “Version” and the number).
    pub fn version(&self) -> Option<String> {
        self.name(VERSION)
    }

    /// Size of font in points.
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
    pub fn text(&self, string: &str) -> Text {
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
            glyphs,
            height,
            width,
            scale,
        }
    }
}
