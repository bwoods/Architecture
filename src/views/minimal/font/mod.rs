//! Minimal `Font` handling.

use std::marker::PhantomData;
use std::ops::Deref;

pub use typography::{body, label, title};

use crate::dependencies::{with_dependencies, with_dependency};
use crate::views::minimal::Accessibility;
use crate::views::text::Font;

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
    use crate::dependencies::Dependency;

    with_default_fonts(|| {
        let body = Dependency::<Inter<body::M>>::new();
        assert!(body.is_some());
    });
}

#[test]
fn snapshot_testing() {
    use crate::dependencies::Dependency;
    use crate::views::svg::Output as Svg;
    use crate::views::{Bounds, View};
    use insta::assert_snapshot;

    with_default_fonts(|| {
        let (w, h) = (256.0, 22.0);
        let black = [0, 0, 0, 0xff];

        let mut output = Svg::new(w, h);
        let body = Dependency::<Inter<body::M>>::new();
        let text = body.text(black, "This space intentionally left blank.");

        text.draw(Bounds::from_size((w, h).into()), &mut output);
        assert_snapshot!("body text", output.into_inner());
    });
}
