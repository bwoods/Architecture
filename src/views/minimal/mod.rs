//! A minimal, _but viable_, user interface layer.

pub use font::{with_default_fonts, Inter};

use crate::dependencies::DependencyDefault;

pub mod font;

/// `Accessibility` defines a predefined scale for scalable content.
#[derive(Copy, Clone)]
pub enum Accessibility {
    /// Use extra extra small sizes.  
    /// This is the default on desktop platforms.
    XXS,
    /// use extra small sizes.
    XS,
    /// Use small sizes.
    S,
    /// Use medium sizes.
    M,
    /// Use large sizes.  
    /// This is the default on mobile platforms.
    L,
    /// use extra large sizes.
    XL,
    /// Use extra extra large sizes.
    XXL,
    /// Use extra extra extra large sizes.
    XXXL,
}

impl Default for Accessibility {
    fn default() -> Self {
        if cfg!(ios) || cfg!(android) {
            Accessibility::L
        } else {
            Accessibility::XXS
        }
    }
}

impl DependencyDefault for Accessibility {}
