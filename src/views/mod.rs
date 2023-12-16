//!

use lyon::math::{Box2D as Bounds, Point, Size};

pub mod output;

/// Represents an individual user interface element and provides modifiers that can be
/// uses to re-configure it.
pub trait View: Sized {
    /// The intrinsic size of the `View`
    fn size(&self) -> Size;
    /// User-interface [`Event`] handling of the `View`
    fn event(&self, event: Event, offset: Point, bounds: Bounds) -> bool;
    /// How the `View` is drawn
    fn draw(&self, bounds: Bounds, onto: &mut impl FnMut(Layer) -> u32);
}

#[allow(missing_docs)]
#[derive(Copy, Clone)]
/// [`View`] events.
pub enum Event {
    // #[from]
    // Gesture(Gesture),
    Redraw,
    Resize { width: u32, height: u32 },
}

#[rustfmt::skip]
#[allow(missing_docs)]
#[derive(Copy, Clone)]
/// Primitives for [`View`] drawing.
/// 
/// [`View`]s use pass these primitives within their [`draw`] method to render themselves.
///
/// [`draw`]: View::draw
pub enum Layer {
    /// Adds a rectangle path.
    Rect { x: f32, y: f32, w: f32, h: f32, rx: f32, ry: f32 },
    /// Adds a ellipse path.
    Ellipse { x: f32, y: f32, w: f32, h: f32, rx: f32, ry: f32 },
    /// Adds a circle path.
    Circle { x: f32, y: f32, r: f32 },
    
    /// Adds the beginning of a path.
    Move { x: f32, y: f32 },
    /// Adds a line to `x`, `y` from the last point.
    Line { x: f32, y: f32 },
    /// Adds a quadratic curve to `x`, `y` from the last point.
    Quadratic { x1: f32, y1: f32, x: f32, y: f32 },
    /// Adds a cubic curve to `x`, `y` from the last point.
    Cubic { x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32 },
    /// Closes the current path.
    Close,
}
