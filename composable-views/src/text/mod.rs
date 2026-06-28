use crate::{Bounds, Output, Size, Transform, View};
pub use font::{Direction, Family, Font, Glyphs, Language, Script, Tag};

mod font;

/// Text data
#[doc(hidden)] // documented as views::Text
pub struct Text<'a> {
    font: &'a Font<'a>,
    glyphs: Glyphs,
    width: f32,
    scale: f32,
    rgba: [u8; 4],

    #[allow(unused)]
    #[cfg(debug_assertions)]
    text: String,
}

impl Text<'_> {
    /// Height of the Text’s font.
    #[inline]
    pub fn height(&self) -> f32 {
        self.font.height() * self.scale
    }

    /// Ascender height of the Text’s font.
    #[inline]
    pub fn ascender(&self) -> f32 {
        self.font.ascender() * self.scale
    }

    /// Descender height of the Text’s font.  
    /// Note that this is a negative value.
    #[inline]
    pub fn descender(&self) -> f32 {
        self.font.descender() * self.scale
    }

    /// Capital height of the Text’s font.
    #[inline]
    pub fn capital_height(&self) -> Option<f32> {
        self.font.capital_height().map(|x| x * self.scale)
    }

    /// x height of the Text’s font.
    #[inline]
    pub fn x_height(&self) -> Option<f32> {
        self.font.capital_height().map(|x| x * self.scale)
    }

    /// Line gap of the Text’s font.
    #[inline]
    pub fn line_gap(&self) -> f32 {
        self.font.line_gap() * self.scale
    }

    /// A line spaced view of the Text
    #[inline]
    pub fn line_spacing(self, spacing: f32) -> impl View {
        let spacing = f32::max(0.0, spacing);

        let single_spacing = Text::height(&self) + self.line_gap();
        let adjustment = spacing * single_spacing;

        let pad = adjustment - Text::height(&self);
        self.padding_bottom(pad)
    }

    /// A single-spaced view of the Text
    ///
    /// See [`line_spacing`][`Self::line_spacing`]
    #[inline(always)]
    pub fn single_spaced(self) -> impl View {
        let pad = self.line_gap();
        self.padding_bottom(pad)
    }

    /// A double-spaced view of the Text
    ///
    /// See [`line_spacing`][`Self::line_spacing`]
    #[inline(always)]
    pub fn double_spaced(self) -> impl View {
        self.line_spacing(2.0)
    }
}

impl View for Text<'_> {
    #[inline(always)]
    fn size(&self, _within: Size) -> Size {
        Size::new(self.width, self.height())
    }

    fn draw(&self, bounds: Bounds, output: &mut impl Output) {
        struct Builder<'a, T: Output> {
            output: &'a mut T,
            rgba: [u8; 4],
        }

        impl<F: Output> rustybuzz::ttf_parser::OutlineBuilder for Builder<'_, F> {
            fn move_to(&mut self, x: f32, y: f32) {
                self.output.move_to(x, y);
            }

            fn line_to(&mut self, x: f32, y: f32) {
                self.output.line_to(x, y);
            }

            fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
                self.output.quadratic_bezier_to(x1, y1, x, y);
            }

            fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
                self.output.cubic_bezier_to(x1, y1, x2, y2, x, y);
            }

            fn close(&mut self) {
                self.output.close();
            }
        }

        let mut transform = // …
            Transform::scale(self.scale, -self.scale) // negate y-axis
            .then_translate((0.0, self.ascender()).into()) // font baseline
            .then_translate(bounds.min.to_vector()); // start position,

        let mut builder = Builder {
            rgba: self.rgba,
            output,
        };

        let positions = self.glyphs.glyph_positions().iter();
        let glyphs = self.glyphs.glyph_infos().iter();

        for (glyph, position) in Iterator::zip(glyphs, positions) {
            transform = transform.pre_translate(
                // “How much the glyph moves on the [X/Y]-axis before drawing it”
                (position.x_offset as f32, position.y_offset as f32).into(),
            );

            builder.output.begin(builder.rgba, &transform);
            self.font.outline_glyph(glyph.glyph_id, &mut builder);

            transform = transform.pre_translate(
                // “How much the line advances after drawing this glyph”
                (position.x_advance as f32, position.y_advance as f32).into(),
            );
        }
    }
}
