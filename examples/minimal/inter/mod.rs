#![allow(unused)]
use composable_views::text::{Direction, Font, FontConfig, Language, Script};

/// # The Inter typeface family
///
/// Inter is a workhorse of a typeface carefully crafted & designed for a wide
/// range of applications, from detailed user interfaces to marketing & signage.
/// The Inter typeface family features over 2000 glyphs covering 147 languages.
/// Weights range from a delicate thin 100 all the way up to a heavy 900.
/// Each glyph has three dedicated designs for weights 100, 400 and 900 to
/// ensure excellent quality at any weight. Optical size ranges from "text" to
/// "display" and there is a true italic variant.
///
/// — https://rsms.me/inter/
pub struct Inter;

#[inline(always)]
fn font() -> FontConfig<'static> {
    Font::from(&Inter).unwrap()
}

impl Inter {
    #[inline(always)]
    pub fn direction(self, direction: Direction) -> FontConfig<'static> {
        font().direction(direction)
    }

    #[inline(always)]
    pub fn script(self, script: Script) -> FontConfig<'static> {
        font().script(script)
    }

    #[inline(always)]
    pub fn language(self, language: Language) -> FontConfig<'static> {
        font().language(language)
    }

    #[inline(always)]
    pub fn feature(mut self, tag: &[u8; 4], value: u32) -> FontConfig<'static> {
        font().feature(tag, value)
    }

    #[inline(always)]
    pub fn variation(mut self, tag: &[u8; 4], value: f32) -> FontConfig<'static> {
        font().variation(tag, value)
    }

    #[inline(always)]
    pub fn weight(self, weight: f32) -> FontConfig<'static> {
        font().weight(weight)
    }

    #[inline(always)]
    pub fn size(mut self, size: f32) -> Font<'static> {
        font().size(size)
    }
}

impl AsRef<[u8]> for Inter {
    #[inline(never)]
    fn as_ref(&self) -> &[u8] {
        include_bytes!("InterVariable.ttf")
    }
}
