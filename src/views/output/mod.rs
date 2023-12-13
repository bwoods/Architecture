//! The end result of view drawing.
//!
//! [`View`]: super::View

mod gpu;
mod svg;

///
pub trait Output {
    /// A default implementation of rectangle drawing.
    fn rectangle(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let xe = x + w;
        let ye = y + h;

        self.move_to(x, y);
        self.line_to(xe, y);
        self.line_to(xe, ye);
        self.line_to(x, ye);
        self.line_to(x, y);
        self.close();
    }
    /// A default implementation of ellipse drawing.
    fn ellipse(&mut self, x: f32, y: f32, w: f32, h: f32) {
        // https://spencermortensen.com/articles/least-squares-bezier-circle/
        const K: f32 = 0.55197036;

        let kw = (w / 2.0) * K; // control-points offset
        let kh = (h / 2.0) * K;
        let xm = x + w / 2.0; // middle
        let ym = y + h / 2.0;
        let xe = x + w; // end
        let ye = y + h;

        self.move_to(x, ym);
        self.cubic_bezier_to(x, ym - kh, xm - kw, y, xm, y);
        self.cubic_bezier_to(xm + kw, y, xe, ym - kh, xe, ym);
        self.cubic_bezier_to(xe, ym + kh, xm + kw, ye, xm, ye);
        self.cubic_bezier_to(xm - kw, ye, x, ym + kh, x, ym);
        self.close();
    }
    /// A default implementation of circle drawing.
    fn circle(&mut self, x: f32, y: f32, r: f32) {
        self.ellipse(x, y, r, r)
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
