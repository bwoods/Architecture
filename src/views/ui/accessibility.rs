use crate::dependencies::DependencyDefault;

/// `Scale` defines a predefined scale for scalable content.
#[derive(Copy, Clone)]
pub enum Scale {
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

impl Default for Scale {
    fn default() -> Self {
        if cfg!(ios) || cfg!(android) {
            Scale::L
        } else {
            Scale::XXS
        }
    }
}

impl DependencyDefault for Scale {}
