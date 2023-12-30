//! A minimal, _but viable_, user interface layer.
//!
//! # Minimal
//!
//! **Minimal** exist as a separate feature so that applications that need
//! completely custom user interface elements are not weighned down by the
//! implementations here; while still having access to those same interface
//! elements to use as reference when building their own.
//!
//! - **Configurable**  
//!   Much of the design of Minimal’s user interface elements has been
//!   configured via `dependencies` as default values. As such they can be
//!   overridden by the applications as needed.
//! - **Incremental**  
//!   Furthermore, use of Minimal does not need to be all or nothing.
//!   Development of an application can begin with Minimal’s default
//!   look-and-feel and custom `View` implementations can be added
//!   incrementally to the application as its configurability becomes
//!   insufficient.

use crate::dependencies::DependencyDefault;
pub use font::{with_default_fonts, Inter};

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
