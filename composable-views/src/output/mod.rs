#![allow(clippy::too_many_arguments)]
//! The end result of view drawing.
//!
//! [`View`]: super::View
use crate::Transform;

pub mod gpu;
pub mod svg;

/// A surface, or file format, that views may be rendered to.
///
/// ## Note
/// If a `Dependency::<Transform>` is in scope, the `Output` is expected to
/// transform its input accordingly.
#[rustfmt::skip]
pub trait Output: Sized {
    /// Sets up the uniform parameters of the following paths.
    ///
    /// Path should always begin with a [`move_to`].
    ///
    /// [`move_to`]: Self::move_to
    fn begin(&mut self, rgba: [u8; 4], transform: &Transform);

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
    /// Once this method has been called there is no current path until [`begin`] is called again.
    ///
    /// [`begin`]: Self::begin
    fn close(&mut self);
}
