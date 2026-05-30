//! User interface element and modifiers to re-configure it.

use composable::*;
pub use lyon::math::{Box2D as Bounds, Point, Size, Transform};
/// Alias for `euclid::default::SideOffsets2D<f32>`
pub type Offsets = lyon::geom::euclid::default::SideOffsets2D<f32>;
pub use gesture::{Id, TapGesture, Target};
pub use grouping::horizontal::{Horizontal, HorizontalAlignment::*};
pub use grouping::vertical::{Vertical, VerticalAlignment::*};
pub use layout::Spacer;
use modifiers::background::Background;
use modifiers::fixed::{Fixed, FixedHeight, FixedWidth};
use modifiers::padding::Padding;
pub use output::{Output, gpu, svg};
pub use shapes::{Circle, ContinuousRoundedRectangle, Ellipse, Path, Rectangle, RoundedRectangle};
pub use text::Text;
pub use ui_id::ui_id;

/// Warnings logged by `View` modifiers should only be logged once, not on _every_ draw cycle.
#[allow(unused_macros)]
macro_rules! warn_once {
    ( $( $x:expr ),+ ) => {
        if cfg!(debug_assertions) {
            static ONCE: std::sync::Once = std::sync::Once::new();
            let caller = std::panic::Location::caller();

            ONCE.call_once(|| {
                log::warn!("{}", format!("{}: {}", caller, $( $x )*));
            });
        }
    };
}
pub(crate) use warn_once;

mod gesture;
mod grouping;
mod layout;
mod modifiers;
mod output;
mod shapes;
pub mod text;

pub trait View: Sized {
    /// The intrinsic size of the `View`
    fn size(&self, within: Size) -> Size;
    /// User-interface [`Event`] handling of the `View`
    #[allow(unused_variables)]
    fn event(&self, event: Event, bounds: Bounds) {}
    /// How the `View` is drawn
    fn draw(&self, bounds: Bounds, onto: &mut impl Output);

    /// Add a background shape to the `View`; as defined by a [`Path`]
    fn background<P>(self, path: P) -> Background<Self, P> {
        Background {
            view: self,
            background: path,
        }
    }

    /// Add padding to all sides of the `View`
    fn padding(self, top: f32, right: f32, bottom: f32, left: f32) -> impl View {
        Padding {
            offsets: Offsets::new(top, right, bottom, left),
            view: self,
        }
    }

    /// Add padding to the top of the `View`
    fn padding_top(self, pad: f32) -> impl View {
        self.padding(pad, 0.0, 0.0, 0.0)
    }

    /// Add padding to the right side of the `View`
    fn padding_right(self, pad: f32) -> impl View {
        self.padding(0.0, pad, 0.0, 0.0)
    }

    /// Add padding to the bottom of the `View`
    fn padding_bottom(self, pad: f32) -> impl View {
        self.padding(0.0, 0.0, pad, 0.0)
    }

    /// Add padding to the left side of the `View`
    fn padding_left(self, pad: f32) -> impl View {
        self.padding(0.0, 0.0, 0.0, pad)
    }

    /// Add padding to the horizontal sides of the `View`
    fn padding_horizontal(self, pad: f32) -> impl View {
        self.padding(0.0, pad, 0.0, pad)
    }

    /// Add padding to the vertical sides of the `View`
    fn padding_vertical(self, pad: f32) -> impl View {
        self.padding(pad, 0.0, pad, 0.0)
    }

    /// Add different padding to the horizontal and vertical sides of the `View`
    fn padding_both(self, x: f32, y: f32) -> impl View {
        self.padding(y, x, y, x)
    }

    /// Add the same padding to all sides of the `View`
    fn padding_all(self, pad: f32) -> impl View {
        self.padding(pad, pad, pad, pad)
    }

    /// Set the size of the `View` to a fixed value.
    fn fixed(self, width: f32, height: f32) -> impl View {
        let size = Size::new(width, height);
        Fixed { view: self, size }
    }

    /// Set the width of the `View` to a fixed value.
    fn width(self, width: f32) -> impl View {
        FixedWidth { view: self, width }
    }

    /// Set the height of the `View` to a fixed value.
    fn height(self, height: f32) -> impl View {
        FixedHeight { view: self, height }
    }

    /// # Flexible View handling
    /// Flexible views re-compute their size based upon the bounds of their
    /// containing view. [`Spacer::fill`] for example.
    #[inline(always)]
    #[allow(unused_variables)]
    fn adjust_width(&self, width: f32) {}

    /// # Flexible View handling
    /// Flexible views re-compute their size based upon the bounds of their
    /// containing view. [`Spacer::fill`] for example.
    #[inline(always)]
    #[allow(unused_variables)]
    fn adjust_height(&self, height: f32) {}

    /// # Flexible View handling
    /// Flexible views may be sized proportionally to each other.
    #[doc(hidden)]
    #[inline(always)]
    fn frac(&self) -> usize {
        1
    }

    /// ###### Gesture handling
    /// The `View`’s response to a tap
    fn on_tap<A, E>(self, id: Id, action: A, send: E) -> TapGesture<Self, A, E>
    where
        A: Clone,
        E: Effects<Action = A>,
    {
        TapGesture {
            id,
            view: self,
            action,
            send,
        }
    }

    /// ###### Gesture handling
    /// The `View`’s response to a tap, but presenting a larger tap surface than is drawn
    fn on_tap_target<A, E>(
        self,
        id: Id,
        action: A,
        send: E,
        size: Size,
    ) -> Target<TapGesture<Self, A, E>>
    where
        A: Clone,
        E: Effects<Action = A>,
    {
        Target {
            view: TapGesture {
                id,
                view: self,
                action,
                send,
            },
            minimum: size,
        }
    }
}

/// [`View`] events.
#[allow(missing_docs)]
#[derive(Clone, Debug, From, TryInto)]
pub enum Event {
    Gesture(Gesture, Point),
}

/// touches… buttons…
#[derive(Copy, Clone, Debug)]
pub enum Gesture {
    Began { n: u8 },
    Moved { n: u8 },
    Ended { n: u8 },
}
