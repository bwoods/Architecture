pub use rustybuzz::ttf_parser::Tag;
use rustybuzz::ttf_parser::name_id::{FAMILY, FULL_NAME, SUBFAMILY, UNIQUE_ID, VERSION};
use rustybuzz::ttf_parser::{GlyphId, OutlineBuilder};
pub use rustybuzz::{Direction, Feature, GlyphBuffer as Glyphs, Language, Script};
use rustybuzz::{Face, ShapePlan, UnicodeBuffer, shape_with_plan};

use crate::Text;

///
pub struct Font<'a> {
    face: Face<'a>,
    plan: ShapePlan,
    size: f32,
}

impl Font<'_> {
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

    #[inline(never)]
    fn name(&self, id: u16) -> Option<String> {
        self.face
            .names()
            .into_iter()
            .find(|name| name.name_id == id)
            .and_then(|name| name.to_string())
    }

    /// Font size in points.
    #[inline]
    pub fn size(&self) -> f32 {
        self.size
    }

    /// Horizontal face ascender.
    pub fn ascender(&self) -> f32 {
        self.face.ascender() as f32
    }

    /// Horizontal face descender,
    pub fn descender(&self) -> f32 {
        self.face.descender() as f32
    }

    /// Horizontal height,
    #[inline]
    pub fn height(&self) -> f32 {
        self.face.height() as f32
    }

    /// Capital height,
    #[inline]
    pub fn capital_height(&self) -> Option<f32> {
        self.face.capital_height().map(|x| x as f32)
    }

    /// x height.
    #[inline]
    pub fn x_height(&self) -> Option<f32> {
        self.face.x_height().map(|x| x as f32)
    }

    /// Line gap,
    #[inline]
    pub fn line_gap(&self) -> f32 {
        self.face.line_gap() as f32
    }

    /// Returns a `Text` in this font.
    #[inline(never)]
    pub fn text(&self, rgba: [u8; 4], string: &str) -> Text<'_> {
        let mut unicode = UnicodeBuffer::new();
        unicode.push_str(string);

        unicode.set_script(Script::from_iso15924_tag(Tag::from_bytes(b"Latn")).unwrap());

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

        Text {
            #[cfg(debug_assertions)]
            text: string.to_owned(),
            font: self,
            glyphs,
            width,
            scale,
            rgba,
        }
    }

    #[inline(always)]
    pub(crate) fn outline_glyph(&self, glyph: u32, builder: &mut impl OutlineBuilder) {
        self.face.outline_glyph(GlyphId(glyph as u16), builder);
    }
}

impl<'a> Font<'a> {
    /// Create a `Font` from the raw font data.
    #[inline(always)]
    pub fn from<T: AsRef<[u8]>>(data: &'a T) -> Option<FontConfig<'a>> {
        Self::from_collection(data.as_ref(), 0)
    }

    /// Create a `Font` from a font collection.
    /// Returns the font at `index`, if any
    #[inline(never)]
    pub fn from_collection(data: &'a [u8], index: u32) -> Option<FontConfig<'a>> {
        let face = Face::from_slice(data, index)?;

        Some(FontConfig {
            face,
            features: Vec::default(),
            variations: Vec::default(),
            direction: None,
            script: Script::from_iso15924_tag(Tag::from_bytes(b"Latn")),
            language: None,
        })
    }
}

///
#[derive(Clone)]
pub struct FontConfig<'a> {
    face: Face<'a>,
    features: Vec<Feature>,
    variations: Vec<(Tag, f32)>,
    direction: Option<Direction>,
    script: Option<Script>,
    language: Option<Language>,
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
    pub fn variation(mut self, tag: &[u8; 4], value: f32) -> Self {
        self.variations.push((Tag::from_bytes(tag), value));

        self
    }

    ///
    #[inline]
    pub fn weight(self, weight: f32) -> Self {
        self.variation(b"wght", weight)
    }

    /// The final step in building a Font.
    #[inline(never)]
    pub fn size(mut self, size: f32) -> Font<'a> {
        // Always attempt to match optical sizing
        self.face.set_variation(Tag::from_bytes(b"opsz"), size);

        for (tag, value) in self.variations {
            self.face.set_variation(tag, value);
        }

        // Using direction.unwrap_or_default() would give an Direction::Invalid
        // and that will panic!() in ShapePlan::new()
        let direction = self.direction.unwrap_or(Direction::LeftToRight);

        let script = self
            .script
            .or_else(|| Script::from_iso15924_tag(Tag::from_bytes(b"Latn")));

        let plan = ShapePlan::new(
            &self.face,
            direction,
            script,
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
