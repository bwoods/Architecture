//!

pub use lyon::math::{Box2D as Bounds, Point, Size, Transform};

pub use output::{gpu, svg, Output};
#[doc(inline)]
pub use text::Text;

mod output;
/// Text handling for `View` construction.
pub mod text;

pub mod minimal;

/// User interface element and modifiers to re-configure it.
pub trait View: Sized {
    /// The intrinsic size of the `View`
    fn size(&self) -> Size;
    /// User-interface [`Event`] handling of the `View`
    fn event(&self, event: Event, offset: Point, bounds: Bounds);
    /// How the `View` is drawn
    fn draw(&self, bounds: Bounds, onto: &mut impl Output);
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
