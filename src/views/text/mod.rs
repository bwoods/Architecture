use font::Glyphs;
pub use font::{Direction, Font, Language, Script};

mod font;

/// Text data
#[doc(hidden)] // documented as views::Text
pub struct Text {
    glyphs: Glyphs,
    height: f32,
    width: f32,
    scale: f32,
}

impl Text {}
