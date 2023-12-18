mod typography;

use rustybuzz::{ttf_parser::fonts_in_collection, Face, Tag};

use crate::dependencies::{with_dependency, DependencyDefault};

/// Font data
pub struct Font<'a>(&'a [u8]);

/// The Inter font v4
pub struct Inter<'a> {
    font: Font<'a>,
}

impl Inter<'_> {
    /// Calls the supplied closure with the font data and the appropriate scale for that data.
    #[inline(never)]
    pub fn with<F, R>(&self, weight: f32, size: f32, f: F) -> Option<R>
    where
        F: FnOnce(&Face, f32) -> R,
    {
        let opsz = size.clamp(14.0, 32.0);
        let wght = weight.clamp(100.0, 900.0);
        let face_index = 0;

        Face::from_slice(self.font.0, face_index)
            .as_mut()
            .map(|face| {
                face.set_variation(Tag::from_bytes(b"opsz"), opsz);
                face.set_variation(Tag::from_bytes(b"wght"), wght);

                let scale = size / face.units_per_em() as f32;
                f(face, scale)
            })
    }
}

impl Default for Inter<'_> {
    fn default() -> Self {
        let data = include_bytes!("InterVariable.ttf");
        let n = fonts_in_collection(data).unwrap_or(1);
        assert_eq!(n, 1); // not a font collection

        Self { font: Font(data) }
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

        inter.with(400.0, 14.0, |face, _scale| {
            //
            let names: Vec<_> = face
                .names()
                .into_iter()
                .filter(|name| name.is_unicode())
                .map(|name| (name.name_id, name.to_string()))
                .collect();

            assert!(names.len() > 0)
        });
    });
}
