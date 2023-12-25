#![allow(clippy::too_many_arguments)]
//! The end result of view drawing.
//!
//! [`View`]: super::View

use crate::views::Layer;

pub mod gpu;
pub mod svg;

/// [Least-squares approximation of the circle using cubic Bézier curves][site]
///
/// > David Ellsworth found the optimal value of c:  
/// >
/// > c ≈ 0.5519703814011128603134107  
/// >
/// > This is the least-squares approximation to the right circular arc  
///
/// [site]: https://spencermortensen.com/articles/least-squares-bezier-circle/
const KAPPA: f64 = 0.5519703814011129; // rounded to f64

/// A surface, or file format, that views may be rendered to.
pub trait Output: Sized {
    /// A default implementation of rectangle drawing.
    fn rectangle(&mut self, x: f32, y: f32, w: f32, h: f32, rgba: [u8; 4]) {
        rounded(self, x, y, w, h, 0.0, 0.0, 0.0, rgba);
    }
    /// A default implementation of rounded rectangle drawing (with circular arcs).
    fn rounded(&mut self, x: f32, y: f32, w: f32, h: f32, rx: f32, ry: f32, rgba: [u8; 4]) {
        let rx = rx.min(w / 2.0);
        let ry = ry.min(h / 2.0);
        rounded(self, x, y, w, h, rx, ry, (1.0 - KAPPA) as f32, rgba);
    }
    /// A default implementation of rounded rectangle drawing (with continuous arcs).
    fn continuous(&mut self, x: f32, y: f32, w: f32, h: f32, rx: f32, ry: f32, rgba: [u8; 4]) {
        // continuous corners are much smaller than circular ones; scale them up a bit
        let c = std::f32::consts::E;
        let rx = (rx * c).min(w / 2.0);
        let ry = (ry * c).min(h / 2.0);
        rounded(self, x, y, w, h, rx, ry, 0.0, rgba);
    }
    /// A default implementation of ellipse drawing.
    fn ellipse(&mut self, x: f32, y: f32, w: f32, h: f32, rgba: [u8; 4]) {
        let rx = w / 2.0;
        let ry = h / 2.0;
        rounded(self, x, y, w, h, rx, ry, (1.0 - KAPPA) as f32, rgba);
    }
    /// A default implementation of circle drawing.
    fn circle(&mut self, x: f32, y: f32, d: f32, rgba: [u8; 4]) {
        let r = d / 2.0;
        rounded(self, x, y, d, d, r, r, (1.0 - KAPPA) as f32, rgba);
    }

    /// Begins a new path (with the color: `rgba`).
    ///
    /// The path should be continued with a series of [`line_to`], [`quadratic_bezier_to`], and/or
    /// [`cubic_bezier_to`] calls and ended with a call to [`close`].
    ///
    /// [`line_to`]: Self::line_to
    /// [`quadratic_bezier_to`]: Self::quadratic_bezier_to
    /// [`cubic_bezier_to`]: Self::cubic_bezier_to
    /// [`close`]: Self::close
    fn begin(&mut self, x: f32, y: f32, rgba: [u8; 4]);
    /// Adds a line to the current path.
    fn line_to(&mut self, x: f32, y: f32);
    /// Adds a quadratic Bézier to the current path.
    ///
    /// (`x1`, `y1`) represents the Bézier control point.
    fn quadratic_bezier_to(&mut self, x1: f32, y1: f32, x: f32, y: f32);
    /// Adds a cubic Bézier to the current path.
    ///
    /// (`x1`, `y1`) and (`x2`, `y2`) represent the Bézier control points.
    fn cubic_bezier_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32);

    /// Closes the current path.
    ///
    /// Once this method has been called there is no current path until [`move_to`] is called again.
    ///
    /// [`move_to`]: Self::move_to
    fn end(&mut self, close: bool);

    /// …
    fn as_fn_mut(&mut self) -> impl FnMut(Layer) {
        |layer: Layer| match layer {
            Layer::Rect { x, y, w, h, rgba } => self.rectangle(x, y, w, h, rgba),
            Layer::Ellipse { x, y, w, h, rgba } => self.ellipse(x, y, w, h, rgba),
            Layer::Circle { x, y, r, rgba } => self.circle(x, y, r, rgba),
            Layer::Begin { x, y, rgba } => self.begin(x, y, rgba),
            Layer::Line { x, y } => self.line_to(x, y),
            Layer::Quadratic { x1, y1, x, y } => self.quadratic_bezier_to(x1, y1, x, y),
            #[rustfmt::skip]
                Layer::Cubic { x1, y1, x2, y2, x, y, } => self.cubic_bezier_to(x1, y1, x2, y2, x, y),
            Layer::End { close } => self.end(close),
        }
    }
}

/// ## Notes
/// - diagram (Fourth Trial):
/// https://nacho4d-nacho4d.blogspot.com/2011/05/bezier-paths-rounded-corners-rectangles.html
/// - compiler explorer: https://godbolt.org/z/WEcv17hvb
#[inline(never)]
fn rounded(
    output: &mut impl Output,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    rx: f32,
    ry: f32,
    k: f32,
    rgba: [u8; 4],
) {
    let p0 = (x, y + ry);
    let c0 = (x, y + ry * k);
    let c1 = (x + rx * k, y);
    let p1 = (x + rx, y);

    let p2 = (x + w - rx, y);
    let c2 = (x + w - rx * k, y);
    let c3 = (x + w, y + ry * k);
    let p3 = (x + w, y + ry);

    let p4 = (x + w, y + h - ry);
    let c4 = (x + w, y + h - ry * k);
    let c5 = (x + w - rx * k, y + h);
    let p5 = (x + w - rx, y + h);

    let p6 = (x + rx, y + h);
    let c6 = (x + rx * k, y + h);
    let c7 = (x, y + h - ry * k);
    let p7 = (x, y + h - ry);

    output.begin(p0.0, p0.1, rgba);
    output.cubic_bezier_to(c0.0, c0.1, c1.0, c1.1, p1.0, p1.1);
    output.line_to(p2.0, p2.1);
    output.cubic_bezier_to(c2.0, c2.1, c3.0, c3.1, p3.0, p3.1);
    output.line_to(p4.0, p4.1);
    output.cubic_bezier_to(c4.0, c4.1, c5.0, c5.1, p5.0, p5.1);
    output.line_to(p6.0, p6.1);
    output.cubic_bezier_to(c6.0, c6.1, c7.0, c7.1, p7.0, p7.1);
    output.end(true);
}

#[test]
fn snapshot_testing() {
    use insta::assert_snapshot;

    let black = [0, 0, 0, 0xff];

    let mut output = svg::Output::new(256.0, 256.0);
    output.circle(16.0, 16.0, 224.0, black);
    assert_snapshot!("circle", output.into_inner());

    let mut output = svg::Output::new(256.0, 128.0);
    output.ellipse(16.0, 16.0, 224.0, 112.0, black);
    assert_snapshot!("ellipse", output.into_inner());

    let mut output = svg::Output::new(256.0, 128.0);
    output.rectangle(16.0, 16.0, 224.0, 112.0, black);
    assert_snapshot!("rectangle", output.into_inner());

    let mut output = svg::Output::new(256.0, 128.0);
    output.rounded(16.0, 16.0, 224.0, 112.0, 16.0, 16.0, black);
    assert_snapshot!("rounded", output.into_inner());

    let mut output = svg::Output::new(256.0, 128.0);
    output.continuous(16.0, 16.0, 224.0, 112.0, 16.0, 16.0, black);
    assert_snapshot!("continuous", output.into_inner());
}
