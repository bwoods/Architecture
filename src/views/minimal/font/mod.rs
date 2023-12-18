mod typography;

use rustybuzz::{ttf_parser::fonts_in_collection, Face, Tag};

use crate::dependencies::{with_dependency, Dependency, DependencyDefault};

///
pub struct Inter<'a>(&'a [u8]);

impl Inter<'_> {
    ///
    #[inline(never)]
    pub fn with<F, R>(&self, weight: f32, size: f32, f: F) -> Option<R>
    where
        F: FnOnce(&Face, f32) -> R,
    {
        let opsz = size.clamp(14.0, 32.0);
        let wght = weight.clamp(100.0, 900.0);
        let face_index = 0;

        Face::from_slice(self.0, face_index).as_mut().map(|face| {
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

        Self(data)
    }
}

impl DependencyDefault for Inter<'_> {}

///
pub fn with_default_font<F: FnOnce() -> R, R>(f: F) -> R {
    with_dependency(Inter::default(), f)
}

#[test]
fn test_font_defaults() {
    with_default_font(|| {
        let inter: Dependency<Inter> = Default::default();
        assert!(inter.is_some());
    });
}
