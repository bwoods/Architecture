//! Body `Font` styles.

use std::marker::PhantomData;

use crate::dependencies::DependencyDefault;
use crate::views::minimal::Inter;
use crate::views::text::Font;

///
pub struct S;
///
pub struct M;
///
pub struct L;

impl Default for Inter<'static, S> {
    fn default() -> Self {
        let font = Font::from(super::InterVariable)
            .unwrap()
            .weight(400.0)
            .size(12.0);

        Inter {
            font,
            marker: PhantomData,
        }
    }
}

impl DependencyDefault for Inter<'static, S> {}

impl Default for Inter<'static, M> {
    fn default() -> Self {
        let font = Font::from(super::InterVariable)
            .unwrap()
            .weight(500.0)
            .size(13.0);

        Inter {
            font,
            marker: PhantomData,
        }
    }
}

impl DependencyDefault for Inter<'static, M> {}

impl Default for Inter<'static, L> {
    fn default() -> Self {
        Inter {
            font: Font::from(super::InterVariable)
                .unwrap()
                .weight(500.0)
                .size(14.0),
            marker: PhantomData,
        }
    }
}

impl DependencyDefault for Inter<'static, L> {}
