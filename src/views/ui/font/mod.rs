//! Minimal `Font` handling.

use std::marker::PhantomData;
use std::ops::Deref;

use crate::dependencies::{with_dependencies, with_dependency};
pub use typography::{body, label, title};

use crate::views::text::Font;
use crate::views::ui::Accessibility;

mod typography;

/// The [Inter font](https://github.com/rsms/inter/blob/master/README.md)
///
/// [Release 4.0](https://github.com/rsms/inter/releases/tag/v4.0)
///
/// > # Inter
/// >
/// > Inter is a typeface carefully crafted & designed for computer screens.
/// > Inter features a tall x-height to aid in readability of mixed-case and
/// > lower-case text.
/// > Inter is a [variable font](https://rsms.me/inter/#variable) with
/// > several [OpenType features](https://rsms.me/inter/#features),
/// > like contextual alternates that adjusts punctuation depending on the shape
/// > of surrounding glyphs, slashed zero for when you need to disambiguate
/// > "0" from "o", tabular numbers, etc.
/// >  
/// > <br>
/// >
/// > ![Sample](https://github.com/rsms/inter/raw/master/misc/readme/intro.png)
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
    use crate::views::{svg::Output as Svg, Bounds, View};
    use insta::assert_snapshot;

    with_default_fonts(|| {
        let black = [0, 0, 0, 0xff];
        let body = Dependency::<Inter<body::M>>::new();
        let caption = Dependency::<Inter<body::S>>::new();

        let text = (
            body.text(black, "This space intentionally left blank."),
            caption.text(black, "except for this, I mean…"),
        );
        let size = text.size().ceil();

        let mut output = Svg::new(size.width, size.height);
        text.draw(Bounds::from_size(size), &mut output);
        assert_snapshot!("body text", output.into_inner());
    });
}
