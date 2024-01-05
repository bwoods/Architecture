use crate::dependencies::Dependency;
use crate::views::{Bounds, Output, Size, Transform, View};

mod rounded;

pub trait Path: Sized {
    fn draw(&self, x: f32, y: f32, w: f32, h: f32, transform: &Transform, onto: &mut impl Output);

    fn fill(self) -> Shape<Self> {
        self.fixed(f32::INFINITY, f32::INFINITY)
    }

    fn fixed(self, width: f32, height: f32) -> Shape<Self> {
        Shape {
            size: Size::new(width, height),
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
pub(crate) const K: f32 = 0.4480296; // 1 - 0.5519703814011128603134107 rounded to f32

pub struct Rectangle {
    pub rgba: [u8; 4],
}

impl Path for Rectangle {
    #[inline(always)]
    fn draw(&self, x: f32, y: f32, w: f32, h: f32, transform: &Transform, onto: &mut impl Output) {
        rounded::rectangle(x, y, w, h, 0.0, 0.0, 0.0, self.rgba, transform, onto);
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

pub struct RoundedRectangle {
    rgba: [u8; 4],
    rx: f32,
    ry: f32,
}

impl Path for RoundedRectangle {
    #[inline(always)]
    fn draw(&self, x: f32, y: f32, w: f32, h: f32, transform: &Transform, onto: &mut impl Output) {
        rounded::rectangle(x, y, w, h, self.rx, self.ry, K, self.rgba, transform, onto);
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

pub struct ContinuousRoundedRectangle {
    rgba: [u8; 4],
    rx: f32,
    ry: f32,
}

impl Path for ContinuousRoundedRectangle {
    #[inline(always)]
    fn draw(&self, x: f32, y: f32, w: f32, h: f32, transform: &Transform, onto: &mut impl Output) {
        // continuous corners are much smaller than circular ones; scale them up a bit
        let c = std::f32::consts::E;
        let rx = (self.rx * c).min(w / 2.0);
        let ry = (self.ry * c).min(h / 2.0);
        rounded::rectangle(x, y, w, h, rx, ry, 0.0, self.rgba, transform, onto);
    }
}

pub struct Ellipse {
    pub rgba: [u8; 4],
}

impl Path for Ellipse {
    #[inline(always)]
    fn draw(&self, x: f32, y: f32, w: f32, h: f32, transform: &Transform, onto: &mut impl Output) {
        let rx = w / 2.0;
        let ry = h / 2.0;
        rounded::rectangle(x, y, w, h, rx, ry, K, self.rgba, transform, onto);
    }
}

pub struct Circle {
    pub rgba: [u8; 4],
}

impl Path for Circle {
    #[inline(always)]
    fn draw(&self, x: f32, y: f32, w: f32, h: f32, transform: &Transform, onto: &mut impl Output) {
        let r = f32::min(w, h) / 2.0;
        rounded::rectangle(x, y, w, h, r, r, K, self.rgba, transform, onto);
    }
}

#[doc(hidden)]
pub struct Shape<T> {
    size: Size,
    path: T,
}

impl<T: Path> View for Shape<T> {
    #[inline(always)]
    fn size(&self) -> Size {
        self.size
    }

    #[inline]
    fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
        let size = match (self.size.width.is_finite(), self.size.height.is_finite()) {
            (true, true) => self.size,
            (false, false) => bounds.size(),
            (true, false) => Size::new(self.size.width, bounds.height()),
            (false, true) => Size::new(bounds.width(), self.size.height),
        };

        self.path.draw(
            bounds.min.x,
            bounds.min.y,
            size.width,
            size.height,
            &Dependency::<Transform>::new().unwrap_or_default(),
            onto,
        );
    }

    #[inline(always)]
    #[allow(refining_impl_trait)]
    fn fixed(mut self, width: f32, height: f32) -> Self {
        self.size = Size::new(width, height);
        self
    }

    #[inline(always)]
    #[allow(refining_impl_trait)]
    fn width(mut self, width: f32) -> Self {
        self.size.width = width;
        self
    }

    #[inline(always)]
    #[allow(refining_impl_trait)]
    fn height(mut self, height: f32) -> Self {
        self.size.height = height;
        self
    }
}
