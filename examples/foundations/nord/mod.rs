//! # An arctic, north-bluish color palette
//!
//! Created for the clean and uncluttered design pattern to achieve a optimal focus and readability
//! for code syntax highlighting and UI components.
//!
//! — https://www.nordtheme.com/
#![allow(rustdoc::bare_urls)]
#![allow(unused)]
use std::ops::Deref;

pub struct Nord;

#[allow(non_upper_case_globals)]
#[allow(clippy::unusual_byte_groupings)]
const sRGBA: [[u8; 4]; 16] = [
    0x2e3440_ff_u32.to_be_bytes(), // nord0
    0x3b4252_ff_u32.to_be_bytes(), // nord1
    0x434c5e_ff_u32.to_be_bytes(), // nord2
    0x4c566a_ff_u32.to_be_bytes(), // nord3
    0xd8dee9_ff_u32.to_be_bytes(), // nord4
    0xe5e9f0_ff_u32.to_be_bytes(), // nord5
    0xeceff4_ff_u32.to_be_bytes(), // nord6
    0x8fbcbb_ff_u32.to_be_bytes(), // nord7
    0x88c0d0_ff_u32.to_be_bytes(), // nord8
    0x81a1c1_ff_u32.to_be_bytes(), // nord9
    0x5e81ac_ff_u32.to_be_bytes(), // nord10
    0xbf616a_ff_u32.to_be_bytes(), // nord11 (red)
    0xd08770_ff_u32.to_be_bytes(), // nord12 (orange)
    0xebcb8b_ff_u32.to_be_bytes(), // nord13 (yellow)
    0xa3be8c_ff_u32.to_be_bytes(), // nord14 (green)
    0xb48ead_ff_u32.to_be_bytes(), // nord15 (purple)
];

#[rustfmt::skip]
const PALETTE: (
    [u8; 4],[u8; 4],[u8; 4],[u8; 4],[u8; 4],[u8; 4],[u8; 4],[u8; 4],
    [u8; 4],[u8; 4],[u8; 4],[u8; 4],[u8; 4],[u8; 4],[u8; 4],[u8; 4],
) = (
    sRGBA[ 0],sRGBA[ 1],sRGBA[ 2],sRGBA[ 3],sRGBA[ 4],sRGBA[ 5],sRGBA[ 6],sRGBA[ 7],
    sRGBA[ 8],sRGBA[ 9],sRGBA[10],sRGBA[11],sRGBA[12],sRGBA[13],sRGBA[14],sRGBA[15],
);

impl Deref for Nord {
    #[rustfmt::skip]
    type Target = (
        [u8; 4],[u8; 4],[u8; 4],[u8; 4],[u8; 4],[u8; 4],[u8; 4],[u8; 4],
        [u8; 4],[u8; 4],[u8; 4],[u8; 4],[u8; 4],[u8; 4],[u8; 4],[u8; 4],
    );

    #[inline]
    fn deref(&self) -> &Self::Target {
        &PALETTE
    }
}

impl Nord {
    /// Converts the sRGB color values to Linear
    #[inline] // since we can’t const
    pub fn f64(&self, nord: usize) -> [f64; 4] {
        let srgb = sRGBA[nord];
        [linear(srgb[0]), linear(srgb[1]), linear(srgb[2]), 1.0]
    }

    pub const fn white(&self) -> [u8; 4] {
        0xffffff_ff_u32.to_be_bytes()
    }

    pub const fn black(&self) -> [u8; 4] {
        0x000000_ff_u32.to_be_bytes()
    }

    pub const fn red(&self) -> [u8; 4] {
        sRGBA[11]
    }

    pub const fn orange(&self) -> [u8; 4] {
        sRGBA[12]
    }

    pub const fn yellow(&self) -> [u8; 4] {
        sRGBA[13]
    }

    pub const fn green(&self) -> [u8; 4] {
        sRGBA[14]
    }

    pub const fn purple(&self) -> [u8; 4] {
        sRGBA[15]
    }

    pub const fn n(&self, nord: usize) -> [u8; 4] {
        sRGBA[nord]
    }
}

/// “I guess that if you’re a perfectionist, and want no discontinuity at all, you can tweak the formulas like this:”
///
///  — https://entropymine.com/imageworsener/srgbformula/
#[inline]
fn linear(s: u8) -> f64 {
    let s = s as f64 / 255.0;
    if s <= 0.0404482362771082 {
        s / 12.92
    } else {
        ((s + 0.055) / 1.055).powf(2.4) // ⬅︎ prevents const
    }
}
