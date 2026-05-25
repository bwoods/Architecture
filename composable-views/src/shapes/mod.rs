use crate::{Bounds, Output, Size, Spacer, View};

mod rounded;

pub trait Path: Clone + Sized {
    fn draw(&self, x: f32, y: f32, w: f32, h: f32, onto: &mut impl Output);

    fn fill(self) -> impl View {
        Shape {
            view: Spacer::fill(),
            path: self,
        }
    }

    fn fixed(self, width: f32, height: f32) -> impl View {
        Shape {
            view: Spacer::fixed(width, height),
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
pub(crate) struct Shape<V, P> {
    view: V,
    path: P,
}

impl<V: View, P: Path> View for Shape<V, P> {
    #[inline(always)]
    fn size(&self, within: Size) -> Size {
        self.view.size(within)
    }

    #[inline]
    fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
        let size = self.view.size(bounds.size());
        self.path
            .draw(bounds.min.x, bounds.min.y, size.width, size.height, onto);
    }

    fn adjust_width(&self, width: f32) {
        self.view.adjust_width(width)
    }

    fn adjust_height(&self, height: f32) {
        self.view.adjust_height(height)
    }
}
