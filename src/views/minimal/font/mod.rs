//! Minimal `Font` handling.

mod typography;

pub use typography::body;

use crate::dependencies::with_dependencies;
use crate::views::text::Font;

use std::marker::PhantomData;
use std::ops::Deref;

/// The Inter font v4
pub struct Inter<'a, Design> {
    font: Font<'a>,
    marker: PhantomData<Design>,
}

impl<'a, T> Deref for Inter<'a, T> {
    type Target = Font<'a>;

    fn deref(&self) -> &Self::Target {
        &self.font
    }
}

/// Sets the default font for the supplied closure.
pub fn with_default_font<F: FnOnce() -> R, R>(f: F) -> R {
    with_dependencies(
        (
            Inter::<body::L>::default(),
            Inter::<body::M>::default(),
            Inter::<body::S>::default(),
        ),
        f,
    )
}

#[test]
fn test_font_defaults() {
    use crate::dependencies::Dependency;

    with_default_font(|| {
        let inter: Dependency<Inter<body::M>> = Default::default();
        assert!(inter.is_some());
    });
}
