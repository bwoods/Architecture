use std::ops::Deref;

pub use lyon::math::{Box2D as Bounds, Point, Size, Transform};

pub use layout::Spacer;
pub use modifiers::fixed::{Fixed, FixedHeight, FixedWidth};
pub use modifiers::padding::Padding;
pub use output::{gpu, svg, Output};
pub use shapes::{Circle, ContinuousRoundedRectangle, Ellipse, Rectangle, RoundedRectangle};
pub use shapes::{Path, Shape};
#[doc(inline)]
pub use text::Text;

use crate::{From, TryInto};

/// Alias for `euclid::default::SideOffsets2D<f32>`
pub type Offsets = lyon::geom::euclid::default::SideOffsets2D<f32>;

mod output;
/// Text handling for `View` construction.
pub mod text;

pub mod gesture;
mod layout;
mod modifiers;

mod shapes;
#[cfg(feature = "default_ui")]
pub mod ui;

/// User interface element and modifiers to re-configure it.
pub trait View: Sized {
    /// The intrinsic size of the `View`
    fn size(&self) -> Size;
    /// User-interface [`Event`] handling of the `View`
    #[allow(unused_variables)]
    fn event(&self, event: Event, offset: Point, bounds: Bounds) {}
    /// How the `View` is drawn
    fn draw(&self, bounds: Bounds, onto: &mut impl Output);

    /// Add padding to all sides of the `View`
    fn padding(self, top: f32, right: f32, bottom: f32, left: f32) -> Padding<Self> {
        Padding {
            offsets: Offsets::new(top, right, bottom, left),
            view: self,
        }
    }

    /// Add padding to the top of the `View`
    fn padding_top(self, pad: f32) -> Padding<Self> {
        self.padding(pad, 0.0, 0.0, 0.0)
    }

    /// Add padding to the right side of the `View`
    fn padding_right(self, pad: f32) -> Padding<Self> {
        self.padding(0.0, pad, 0.0, 0.0)
    }

    /// Add padding to the bottom of the `View`
    fn padding_bottom(self, pad: f32) -> Padding<Self> {
        self.padding(0.0, 0.0, pad, 0.0)
    }

    /// Add padding to the left side of the `View`
    fn padding_left(self, pad: f32) -> Padding<Self> {
        self.padding(0.0, 0.0, 0.0, pad)
    }

    /// Add padding to the horizontal sides of the `View`
    fn padding_horizontal(self, pad: f32) -> Padding<Self> {
        self.padding(0.0, pad, 0.0, pad)
    }

    /// Add padding to the vertical sides of the `View`
    fn padding_vertical(self, pad: f32) -> Padding<Self> {
        self.padding(0.0, 0.0, pad, 0.0)
    }

    /// Add different padding to the horizontal and vertical sides of the `View`
    fn padding_both(self, horizontal: f32, vertical: f32) -> Padding<Self> {
        self.padding(vertical, horizontal, vertical, horizontal)
    }

    /// Add the same padding to all sides of the `View`
    fn padding_all(self, pad: f32) -> Padding<Self> {
        self.padding(pad, pad, pad, pad)
    }

    /// Set the size of the `View` to a fixed value.
    fn fixed(self, width: f32, height: f32) -> impl View {
        let size = self.size();
        if size.is_empty() {
            return Err(Fixed {
                size: Size::new(width, height),
                view: self,
            });
        }

        let horizontal = (width - size.width) / 2.0;
        let vertical = (height - size.height) / 2.0;
        Ok(self.padding_both(horizontal, vertical))
    }

    fn width(self, width: f32) -> impl View {
        let size = self.size();
        if size.is_empty() {
            return Err(FixedWidth { width, view: self });
        }

        let horizontal = (width - size.width) / 2.0;
        Ok(self.padding_horizontal(horizontal))
    }

    fn height(self, height: f32) -> impl View {
        let size = self.size();
        if size.is_empty() {
            return Err(FixedHeight { height, view: self });
        }

        let vertical = (height - size.height) / 2.0;
        Ok(self.padding_vertical(vertical))
    }
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
        if let Some(view) = self {
            return view.size();
        }

        Size::zero()
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
#[derive(Copy, Clone, From, TryInto)]
pub enum Event {
    Gesture(Gesture),
    Resize { width: u32, height: u32 },
    Redraw,
}

/// touches… buttons…
#[derive(Copy, Clone)]
pub enum Gesture {
    Began { n: u8 },
    Moved { n: u8 },
    Ended { n: u8 },
}
