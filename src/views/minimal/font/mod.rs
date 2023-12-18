mod typography;

use crate::dependencies::{with_dependency, DependencyDefault};
use crate::views::text::Font;

/// The Inter font v4
pub struct Inter<'a> {
    font: Font<'a>,
}

impl Default for Inter<'_> {
    fn default() -> Self {
        Self {
            font: Font::new(
                include_bytes!("InterVariable.ttf"),
                None,
                None,
                None,
                400.0, // TODO: Accessibility and Typography
                14.0,
            )
            .unwrap(),
        }
    }
}

impl<'a> std::ops::Deref for Inter<'a> {
    type Target = Font<'a>;

    fn deref(&self) -> &Self::Target {
        &self.font
    }
}

impl DependencyDefault for Inter<'_> {}

/// Sets the default font for the supplied closure.
pub fn with_default_font<F: FnOnce() -> R, R>(f: F) -> R {
    with_dependency(Inter::default(), f)
}

#[test]
fn test_font_defaults() {
    use crate::dependencies::Dependency;

    with_default_font(|| {
        let inter: Dependency<Inter> = Default::default();
        assert!(inter.is_some());
    });
}
