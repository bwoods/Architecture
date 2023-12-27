//! Minimal `Font` handling.

use std::marker::PhantomData;
use std::ops::Deref;

use dependencies::{with_dependencies, with_dependency};
pub use typography::{body, label, title};

use crate::text::Font;
use crate::ui::Accessibility;

mod typography;

/// The Inter font v4
pub struct Inter<'a, Style> {
    marker: PhantomData<Style>,
    font: Font<'a>,
}

impl<'a, Style> Deref for Inter<'a, Style> {
    type Target = Font<'a>;

    fn deref(&self) -> &Self::Target {
        &self.font
    }
}

/// Sets the default font for the supplied closure.
pub fn with_default_fonts<F: FnOnce() -> R, R>(f: F) -> R {
    with_dependency(Accessibility::default(), || {
        with_dependencies(
            (
                Inter::<body::L>::default(),
                Inter::<body::M>::default(),
                Inter::<body::S>::default(),
                Inter::<title::L>::default(),
                Inter::<title::M>::default(),
                Inter::<title::S>::default(),
                Inter::<label::L>::default(),
                Inter::<label::M>::default(),
                Inter::<label::S>::default(),
            ),
            f,
        )
    })
}

#[test]
fn test_font_defaults() {
    use dependencies::Dependency;

    with_default_fonts(|| {
        let body = Dependency::<Inter<body::M>>::new();
        assert!(body.is_some());
    });
}

#[test]
fn snapshot_testing() {
    use crate::svg::Output as Svg;
    use crate::{Bounds, View};
    use dependencies::Dependency;
    use insta::assert_snapshot;

    with_default_fonts(|| {
        let black = [0, 0, 0, 0xff];
        let body = Dependency::<Inter<body::M>>::new();

        let text = body.text(black, "This space intentionally left blank.");
        let size = text.size().ceil();

        let mut output = Svg::new(size.width, size.height);
        text.draw(Bounds::from_size(size), &mut output);
        assert_snapshot!("body text", output.into_inner());
    });
}
