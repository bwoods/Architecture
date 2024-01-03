#![allow(non_snake_case)]
use crate::views::Spacer;

pub mod spacing {
    /// The amount of space taken up by a [`spacer::XXS()`]
    ///
    /// [`spacer::XXS()`]: super::XXS
    pub const XXS: f32 = 2.0;

    /// The amount of space taken up by a [`spacer::XS()`]
    ///
    /// [`spacer::XS()`]: super::XS
    pub const XS: f32 = 4.0;

    /// The amount of space taken up by a [`spacer::S()`]
    ///
    /// [`spacer::S()`]: super::S
    pub const S: f32 = 8.0;

    /// The amount of space taken up by a [`spacer::M()`]
    /// - A good minimum target size for trackpads/mice
    ///
    /// [`spacer::M()`]: super::M
    pub const M: f32 = 18.0; // tableview row height (after the dividing line)

    /// The amount of space taken up by a [`spacer::L()`]
    /// - A good minimum target height for “[a sentence or block of text][WCAG]”
    ///
    /// [`spacer::L()`]: super::L
    /// [WCAG]: https://www.smashingmagazine.com/2023/04/accessible-tap-target-sizes-rage-taps-clicks/#not-all-pixels-are-the-same
    pub const L: f32 = 28.0;

    /// The amount of space taken up by a [`spacer::XL()`]
    ///
    /// [`spacer::XL()`]: super::XL
    pub const XL: f32 = 38.0; // medium toolbar height (before the dividing line)

    /// The amount of space taken up by a [`spacer::XXL()`]
    /// - A good minimum target size for tablets/phones/touchscreen
    ///
    /// [`spacer::XXL()`]: super::XXL
    pub const XXL: f32 = 48.0; // phone tab bar height (before the (invisible) dividing line)

    /// The amount of space taken up by a [`spacer::XXXL()`]
    // - A good minimum target size for VR elements
    ///
    /// [`spacer::XXXL()`]: super::XXXL
    pub const XXXL: f32 = 58.0;
}

#[inline(always)]
pub fn fill() -> Spacer {
    Spacer::fill()
}

#[inline(always)]
pub fn fixed(width: f32, height: f32) -> Spacer {
    Spacer::fixed(width, height)
}

#[inline(always)]
pub fn width(width: f32) -> Spacer {
    Spacer::width(width)
}

#[inline(always)]
pub fn height(height: f32) -> Spacer {
    Spacer::height(height)
}

#[inline(always)]
pub fn empty() -> Spacer {
    Spacer::empty()
}

#[inline(always)]
#[allow(clippy::should_implement_trait)]
pub fn default() -> Spacer {
    S()
}

#[inline(always)]
pub fn XXS() -> Spacer {
    fixed(spacing::XXS, spacing::XXS)
}

#[inline(always)]
pub fn XS() -> Spacer {
    fixed(spacing::XS, spacing::XS)
}

#[inline(always)]
pub fn S() -> Spacer {
    fixed(spacing::S, spacing::S)
}

#[inline(always)]
pub fn M() -> Spacer {
    fixed(spacing::M, spacing::M)
}

#[inline(always)]
pub fn L() -> Spacer {
    fixed(spacing::L, spacing::L)
}

#[inline(always)]
pub fn XL() -> Spacer {
    fixed(spacing::XL, spacing::XL)
}

#[inline(always)]
pub fn XXL() -> Spacer {
    fixed(spacing::XXL, spacing::XXL)
}

#[inline(always)]
pub fn XXXL() -> Spacer {
    fixed(spacing::XXXL, spacing::XXXL)
}
