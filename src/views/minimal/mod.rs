//! A minimal, _but viable_, user interface layer.

mod font;
pub use font::{with_default_font, Inter};

#[derive(Copy, Clone)]
/// `Accessibility` defines a predefined scale for scalable content.
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
