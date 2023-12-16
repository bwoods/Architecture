//! The end result of view drawing.
//!
//! [`View`]: super::View

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

///
pub trait Output: Sized {
    /// A default implementation of rectangle drawing.
    fn rectangle(&mut self, x: f32, y: f32, w: f32, h: f32) {
        rounded(self, x, y, w, h, 0.0, 0.0, 0.0);
    }
    /// A default implementation of rounded rectangle drawing (with circular arcs).
    fn rounded(&mut self, x: f32, y: f32, w: f32, h: f32, rx: f32, ry: f32) {
        rounded(self, x, y, w, h, rx, ry, (1.0 - KAPPA) as f32);
    }
    /// A default implementation of rounded rectangle drawing (with continuous arcs).
    fn continuous(&mut self, x: f32, y: f32, w: f32, h: f32, rx: f32, ry: f32) {
        rounded(self, x, y, w, h, rx, ry, 0.0);
    }
    /// A default implementation of ellipse drawing.
    fn ellipse(&mut self, x: f32, y: f32, w: f32, h: f32) {
        rounded(self, x, y, w, h, w / 2.0, h / 2.0, (1.0 - KAPPA) as f32);
    }
    /// A default implementation of circle drawing.
    fn circle(&mut self, x: f32, y: f32, radius: f32) {
        let w = 2.0 * radius;
        rounded(self, x, y, w, w, radius, radius, (1.0 - KAPPA) as f32);
    }

    /// Begins a new path.
    ///
    /// The path should be continued with a series of [`line_to`], [`quadratic_bezier_to`], and/or
    /// [`cubic_bezier_to`] calls and ended with a call to [`close`].
    ///
    /// [`line_to`]: Self::line_to
    /// [`quadratic_bezier_to`]: Self::quadratic_bezier_to
    /// [`cubic_bezier_to`]: Self::cubic_bezier_to
    /// [`close`]: Self::close
    fn move_to(&mut self, x: f32, y: f32);
    /// Draws a line.
    fn line_to(&mut self, x: f32, y: f32);
    /// Draws a quadratic Bézier.
    ///
    /// (`x1`, `y1`) represents the Bézier control point.
    fn quadratic_bezier_to(&mut self, x1: f32, y1: f32, x: f32, y: f32);
    /// Draws a cubic Bézier.
    ///
    /// (`x1`, `y1`) and (`x2`, `y2`) represent the Bézier control points.
    fn cubic_bezier_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32);

    /// Closes the current path.
    fn close(&mut self);
}

#[allow(clippy::too_many_arguments)]
/// ## Notes
/// - diagram (Fourth Trial):
/// https://nacho4d-nacho4d.blogspot.com/2011/05/bezier-paths-rounded-corners-rectangles.html
/// - compiler explorer: https://godbolt.org/z/WEcv17hvb
fn rounded(output: &mut impl Output, x: f32, y: f32, w: f32, h: f32, rx: f32, ry: f32, k: f32) {
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

    output.move_to(p0.0, p0.1);
    output.cubic_bezier_to(c0.0, c0.1, c1.0, c1.1, p1.0, p1.1);
    output.line_to(p2.0, p2.1);
    output.cubic_bezier_to(c2.0, c2.1, c3.0, c3.1, p3.0, p3.1);
    output.line_to(p4.0, p4.1);
    output.cubic_bezier_to(c4.0, c4.1, c5.0, c5.1, p5.0, p5.1);
    output.line_to(p6.0, p6.1);
    output.cubic_bezier_to(c6.0, c6.1, c7.0, c7.1, p7.0, p7.1);
    output.line_to(p0.0, p0.1);
    output.close();
}

#[test]
fn snapshot_testing() {
    use insta::assert_debug_snapshot;

    let mut output = svg::Output::new(256.0, 256.0);
    output.circle(16.0, 16.0, 112.0);
    output.close();
    assert_debug_snapshot!("circle", output);

    let mut output = svg::Output::new(256.0, 128.0);
    output.ellipse(16.0, 16.0, 224.0, 112.0);
    output.close();
    assert_debug_snapshot!("ellipse", output);

    let mut output = svg::Output::new(256.0, 128.0);
    output.rectangle(16.0, 16.0, 224.0, 112.0);
    output.close();
    assert_debug_snapshot!("rectangle", output);

    let mut output = svg::Output::new(256.0, 128.0);
    output.rounded(16.0, 16.0, 224.0, 112.0, 16.0, 16.0);
    output.close();
    assert_debug_snapshot!("rounded", output);

    let mut output = svg::Output::new(256.0, 128.0);
    output.continuous(16.0, 16.0, 224.0, 112.0, 16.0, 16.0);
    output.close();
    assert_debug_snapshot!("continuous", output);
}
