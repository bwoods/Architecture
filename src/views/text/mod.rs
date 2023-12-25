use font::Glyphs;
pub use font::{Direction, Font, FontConfig, Language, Script};

use crate::views::{Bounds, Event, Layer, Point, Size, View};

mod font;
mod layout;

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
        let (x, y) = bounds.min.into();
        self.font.layout(self, x, y, onto);
    }
}
