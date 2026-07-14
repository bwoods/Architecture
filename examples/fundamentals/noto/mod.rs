//! The Noto typeface families
//!
//! # Noto
//! Noto is a global font collection for writing in all modern and ancient
//! languages.
//!
//! Noto Serif is a modulated (“serif”) design for texts in the Latin,
//! Cyrillic and Greek scripts, also suitable as the complementary font for
//! other script-specific Noto Serif fonts.
//!
//! Noto Sans is an unmodulated (“sans serif”) design for texts in the Latin,
//! Cyrillic and Greek scripts, which is also suitable as the complementary
//! choice for other script-specific Noto Sans fonts.
//!
//! — https://fonts.google.com/noto
#![allow(rustdoc::bare_urls)]
#![allow(unused)]
use composable_views::text::{Family, Font};

pub struct Noto;

impl Noto {
    #[inline(never)]
    pub fn sans(self) -> Family<'static> {
        Font::from(include_bytes!("NotoSans-VariableFont_wdth,wght.ttf")).unwrap()
    }

    #[inline(never)]
    pub fn serif(self) -> Family<'static> {
        Font::from(include_bytes!("NotoSerif-VariableFont_wdth,wght.ttf")).unwrap()
    }
}
