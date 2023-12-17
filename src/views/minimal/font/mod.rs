mod typography;

use std::cell::RefCell;
use std::sync::Arc;

use fontdb::{Database, Source, ID};
use rustybuzz::{Face, Tag};

use crate::dependencies::{Dependency, DependencyDefault};

#[derive(Default)]
pub(crate) struct Repository(RefCell<Database>);

///
pub struct Inter(ID);

impl Inter {
    ///
    #[inline(never)]
    pub fn with_face<F, R>(&self, weight: f32, size: f32, f: F) -> Option<R>
    where
        F: FnOnce(&Face, f32) -> R,
    {
        let opsz = size.clamp(14.0, 32.0);
        let wght = weight.clamp(100.0, 900.0);

        let db: Dependency<Repository> = Default::default();
        let result =
            db.0.borrow()
                .with_face_data(self.0, move |data, face_index| {
                    Face::from_slice(data, face_index).as_mut().map(|face| {
                        face.set_variation(Tag::from_bytes(b"opsz"), opsz);
                        face.set_variation(Tag::from_bytes(b"wght"), wght);

                        let scale = size / face.units_per_em() as f32;
                        f(face, scale)
                    })
                })
                .flatten();

        result
    }
}

impl Default for Inter {
    fn default() -> Self {
        let db: Dependency<Repository> = Default::default();
        let source = Source::Binary(Arc::new(include_bytes!("InterVariable.ttf")));

        let ids = db.0.borrow_mut().load_font_source(source);
        assert!(ids.len() == 1); // not a font collection

        Self(ids[0])
    }
}

impl DependencyDefault for Repository {}
impl DependencyDefault for Inter {}

///
pub fn with_default_font<F: FnOnce() -> R, R>(f: F) -> R {
    use crate::dependencies::with_dependency;

    // Repository must be registered before Inter::default() is called or else the
    // “using `DependencyDefault` is not allowed in a unit test” will be triggered.
    with_dependency(Repository::default(), || {
        with_dependency(Inter::default(), f)
    })
}

#[test]
fn test_font_defaults() {
    with_default_font(|| {
        let inter: Dependency<Inter> = Default::default();
        assert_ne!(inter.0, ID::dummy());
    });
}
