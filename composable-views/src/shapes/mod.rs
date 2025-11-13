use crate::{Bounds, Output, Size, View};
use std::cell::Cell;

mod rounded;

pub trait Path: Clone + Sized {
    fn draw(&self, x: f32, y: f32, w: f32, h: f32, onto: &mut impl Output);

    fn fill(self) -> impl View {
        self.fixed(f32::INFINITY, f32::INFINITY)
    }

    fn fixed(self, width: f32, height: f32) -> impl View {
        Shape {
            size: Size::new(width, height).into(),
            path: self,
        }
    }
}

/// [Least-squares approximation of the circle using cubic Bézier curves][site]
///
/// > David Ellsworth found the optimal value of c:
/// >
/// > c ≈ 0.5519703814011128603134107
///
/// [site]: https://spencermortensen.com/articles/least-squares-bezier-circle/
#[allow(clippy::excessive_precision)]
pub(crate) const K: f32 = 1.0 - 0.5519703814011128603134107;

#[derive(Clone)]
pub struct Rectangle {
    pub rgba: [u8; 4],
}

impl Path for Rectangle {
    #[inline(always)]
    fn draw(&self, x: f32, y: f32, w: f32, h: f32, onto: &mut impl Output) {
        rounded::rectangle(x, y, w, h, 0.0, 0.0, 0.0, self.rgba, onto);
    }
}

impl Rectangle {
    pub fn rounded(self, rx: f32, ry: f32) -> RoundedRectangle {
        RoundedRectangle {
            rgba: self.rgba,
            rx,
            ry,
        }
    }
}

#[derive(Clone)]
pub struct RoundedRectangle {
    rgba: [u8; 4],
    rx: f32,
    ry: f32,
}

impl Path for RoundedRectangle {
    #[inline(always)]
    fn draw(&self, x: f32, y: f32, w: f32, h: f32, onto: &mut impl Output) {
        rounded::rectangle(x, y, w, h, self.rx, self.ry, K, self.rgba, onto);
    }
}

impl RoundedRectangle {
    pub fn continuous(self) -> ContinuousRoundedRectangle {
        ContinuousRoundedRectangle {
            rgba: self.rgba,
            rx: self.rx,
            ry: self.ry,
        }
    }
}

#[derive(Clone)]
pub struct ContinuousRoundedRectangle {
    rgba: [u8; 4],
    rx: f32,
    ry: f32,
}

impl Path for ContinuousRoundedRectangle {
    #[inline(always)]
    fn draw(&self, x: f32, y: f32, w: f32, h: f32, onto: &mut impl Output) {
        // continuous corners are much smaller than circular ones; scale them up a bit
        let c = std::f32::consts::E;
        let rx = (self.rx * c).min(w / 2.0);
        let ry = (self.ry * c).min(h / 2.0);
        rounded::rectangle(x, y, w, h, rx, ry, 0.0, self.rgba, onto);
    }
}

#[derive(Clone)]
pub struct Ellipse {
    pub rgba: [u8; 4],
}

impl Path for Ellipse {
    #[inline(always)]
    fn draw(&self, x: f32, y: f32, w: f32, h: f32, onto: &mut impl Output) {
        let rx = w / 2.0;
        let ry = h / 2.0;
        rounded::rectangle(x, y, w, h, rx, ry, K, self.rgba, onto);
    }
}

#[derive(Clone)]
pub struct Circle {
    pub rgba: [u8; 4],
}

impl Path for Circle {
    #[inline(always)]
    fn draw(&self, x: f32, y: f32, w: f32, h: f32, onto: &mut impl Output) {
        let r = f32::min(w, h) / 2.0;
        rounded::rectangle(x, y, w, h, r, r, K, self.rgba, onto);
    }
}

#[doc(hidden)]
pub(crate) struct Shape<T> {
    size: Cell<Size>,
    path: T,
}

impl<T: Path> View for Shape<T> {
    #[inline(always)]
    fn size(&self) -> Size {
        let size = self.size.get();

        match (size.width != f32::INFINITY, size.height != f32::INFINITY) {
            (true, true) => size,
            (false, false) => Size::zero(),
            (true, false) => Size::new(size.width, 0.0),
            (false, true) => Size::new(0.0, size.height),
        }
    }

    #[inline]
    fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
        let current = self.size.get();

        let size = match (current.width.is_finite(), current.height.is_finite()) {
            (true, true) => current,
            (false, false) => bounds.size(),
            (true, false) => Size::new(current.width, bounds.height()),
            (false, true) => Size::new(bounds.width(), current.height),
        };

        self.path
            .draw(bounds.min.x, bounds.min.y, size.width, size.height, onto);
    }

    #[inline(always)]
    #[allow(refining_impl_trait)]
    fn fixed(mut self, width: f32, height: f32) -> Self {
        *self.size.get_mut() = Size::new(width, height);
        self
    }

    #[inline(always)]
    #[allow(refining_impl_trait)]
    fn width(mut self, width: f32) -> Self {
        self.size.get_mut().width = width;
        self
    }

    #[inline(always)]
    #[allow(refining_impl_trait)]
    fn height(mut self, height: f32) -> Self {
        self.size.get_mut().height = height;
        self
    }

    #[inline(always)]
    #[allow(clippy::bool_comparison)]
    fn needs_layout_x(&self) -> bool {
        self.size.get().width == f32::INFINITY
    }

    #[inline(always)]
    #[allow(clippy::bool_comparison)]
    fn needs_layout_y(&self) -> bool {
        self.size.get().height == f32::INFINITY
    }

    #[inline]
    fn update_layout(&self, size: Size, _bounds: Bounds) {
        let current = self.size.get();

        let size = match (current.width.is_finite(), current.height.is_finite()) {
            (true, true) => current,
            (false, false) => size,
            (true, false) => Size::new(current.width, size.height),
            (false, true) => Size::new(size.width, current.height),
        };

        self.size.set(size);
    }
}
