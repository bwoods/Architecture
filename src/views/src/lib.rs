use std::ops::Deref;

pub use lyon::math::{Box2D as Bounds, Point, Size, Transform};

pub use output::{gpu, svg, Output};
#[doc(inline)]
pub use text::Text;

mod output;
/// Text handling for `View` construction.
pub mod text;

pub mod gesture;
mod layout;
mod modifiers;
#[cfg(feature = "default_ui")]
pub mod ui;

/// User interface element and modifiers to re-configure it.
pub trait View: Sized {
    /// The intrinsic size of the `View`
    fn size(&self) -> Size;
    /// User-interface [`Event`] handling of the `View`
    fn event(&self, event: Event, offset: Point, bounds: Bounds);
    /// How the `View` is drawn
    fn draw(&self, bounds: Bounds, onto: &mut impl Output);
}

impl<T: View> View for Box<T> {
    fn size(&self) -> Size {
        self.deref().size()
    }

    fn event(&self, event: Event, offset: Point, bounds: Bounds) {
        self.deref().event(event, offset, bounds)
    }

    fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
        self.deref().draw(bounds, onto)
    }
}

impl<T: View> View for Option<T> {
    fn size(&self) -> Size {
        self.as_ref().map(|view| view.size()).unwrap_or_default()
    }

    fn event(&self, event: Event, offset: Point, bounds: Bounds) {
        if let Some(view) = self {
            view.event(event, offset, bounds)
        }
    }

    fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
        if let Some(view) = self {
            view.draw(bounds, onto)
        }
    }
}

impl<T: View, E: View> View for Result<T, E> {
    fn size(&self) -> Size {
        match self {
            Ok(view) => view.size(),
            Err(view) => view.size(),
        }
    }

    fn event(&self, event: Event, offset: Point, bounds: Bounds) {
        match self {
            Ok(view) => view.event(event, offset, bounds),
            Err(view) => view.event(event, offset, bounds),
        }
    }

    fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
        match self {
            Ok(view) => view.draw(bounds, onto),
            Err(view) => view.draw(bounds, onto),
        }
    }
}

/// [`View`] events.
#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub enum Event {
    Gesture(Gesture),
    Redraw,
    Resize { width: u32, height: u32 },
}

/// touches… buttons…
#[derive(Copy, Clone)]
pub enum Gesture {
    Began { n: u8 },
    Moved { n: u8 },
    Ended { n: u8 },
}
