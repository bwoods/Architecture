#![allow(dead_code)]

use std::marker::PhantomData;

use crate::dependencies::{Dependency, DependencyDefault};
use crate::views::text::Font;
use crate::views::ui::{Accessibility, Inter};

mod scale;

#[allow(non_upper_case_globals)]
const InterVariable: &[u8] = include_bytes!("../InterVariable.ttf");

#[inline(never)]
fn font<Design>(weight: f32, scale: f32) -> Inter<'static, Design> {
    let accessibility = Dependency::<Accessibility>::new();

    Inter {
        marker: PhantomData,
        font: Font::from(InterVariable)
            .unwrap()
            .weight(weight)
            .size(accessibility.scale(scale)),
    }
}

/// Title `Font` styles.
pub mod title {
    use super::*;

    /// Large variant
    pub struct L;
    /// Medium variant
    pub struct M;
    /// Small variant
    pub struct S;

    impl DependencyDefault for Inter<'static, L> {}
    impl DependencyDefault for Inter<'static, M> {}
    impl DependencyDefault for Inter<'static, S> {}

    impl Default for Inter<'static, L> {
        fn default() -> Self {
            font(650.0, 6.0)
        }
    }

    impl Default for Inter<'static, M> {
        fn default() -> Self {
            font(650.0, 4.0)
        }
    }

    impl Default for Inter<'static, S> {
        fn default() -> Self {
            font(650.0, 3.0)
        }
    }
}

/// Body `Font` styles.
pub mod body {
    use super::*;

    /// Large variant
    pub struct L;
    /// Medium variant
    pub struct M;
    /// Small variant
    pub struct S;

    impl DependencyDefault for Inter<'static, L> {}
    impl DependencyDefault for Inter<'static, M> {}
    impl DependencyDefault for Inter<'static, S> {}

    impl Default for Inter<'static, L> {
        fn default() -> Self {
            font(500.0, 4.0)
        }
    }

    impl Default for Inter<'static, M> {
        fn default() -> Self {
            font(500.0, 3.0)
        }
    }

    impl Default for Inter<'static, S> {
        fn default() -> Self {
            font(400.0, 2.0)
        }
    }
}

/// Label `Font` styles.
pub mod label {
    use super::*;

    /// Large variant
    pub struct L;
    /// Medium variant
    pub struct M;
    /// Small variant
    pub struct S;

    impl DependencyDefault for Inter<'static, L> {}
    impl DependencyDefault for Inter<'static, M> {}
    impl DependencyDefault for Inter<'static, S> {}

    impl Default for Inter<'static, L> {
        fn default() -> Self {
            font(600.0, 2.0)
        }
    }

    impl Default for Inter<'static, M> {
        fn default() -> Self {
            font(600.0, 1.0)
        }
    }

    impl Default for Inter<'static, S> {
        fn default() -> Self {
            font(400.0, 0.0)
        }
    }
}
