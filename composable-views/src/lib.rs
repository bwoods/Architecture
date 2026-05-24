pub use gesture::{Id, TapGesture, Target};
pub use layout::{Layout, Spacer};
pub use lyon::math::{Box2D as Bounds, Point, Size, Transform};
pub use modifiers::fixed::{Fixed, FixedHeight, FixedWidth};
pub use modifiers::padding::Padding;
pub use output::{Output, gpu, svg};
pub use shapes::{Circle, ContinuousRoundedRectangle, Ellipse, Path, Rectangle, RoundedRectangle};
use std::ops::Deref;
#[doc(inline)]
pub use text::Text;
pub use ui_id::ui_id;

use crate::modifiers::background::Background;
use composable::*;

/// Alias for `euclid::default::SideOffsets2D<f32>`
pub type Offsets = lyon::geom::euclid::default::SideOffsets2D<f32>;

mod output;
/// Text handling for `View` construction.
pub mod text;

pub mod gesture;
mod layout;
mod modifiers;

mod shapes;

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

/// User interface element and modifiers to re-configure it.
pub trait View: Sized {
    /// The intrinsic size of the `View`
    fn size(&self) -> Size;
    /// User-interface [`Event`] handling of the `View`
    #[allow(unused_variables)]
    fn event(&self, event: Event, bounds: Bounds) {}
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
        self.padding(pad, 0.0, pad, 0.0)
    }

    /// Add different padding to the horizontal and vertical sides of the `View`
    fn padding_both(self, horizontal: f32, vertical: f32) -> Padding<Self> {
        self.padding(vertical, horizontal, vertical, horizontal)
    }

    /// Add the same padding to all sides of the `View`
    fn padding_all(self, pad: f32) -> Padding<Self> {
        self.padding(pad, pad, pad, pad)
    }

    /// Add a background shape to the `View`
    fn background<P>(self, path: P) -> Background<Self, P> {
        Background {
            view: self,
            background: path,
        }
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

    #[doc(hidden)]
    #[inline(always)]
    fn needs_layout_x(&self) -> bool {
        false
    }

    #[doc(hidden)]
    #[inline(always)]
    fn needs_layout_y(&self) -> bool {
        false
    }

    #[doc(hidden)]
    #[inline(always)]
    fn update_layout(&self, _size: Size, _bounds: Bounds) {}

    /// Causes a tuple of `View`s to cascade horizontally, rather than vertically.
    /// ## Note
    /// For other views nothing changes (and a warning will be [logged][log]).
    ///
    /// [log]: https://docs.rs/log/latest/log/index.html
    #[track_caller]
    fn across(self) -> impl View {
        warn_once!(".across() only has an affect on a tuple of views");
        self
    }

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

impl<T: View> View for Box<T> {
    #[inline(always)]
    fn size(&self) -> Size {
        self.deref().size()
    }

    #[inline(always)]
    fn event(&self, event: Event, bounds: Bounds) {
        self.deref().event(event, bounds)
    }

    #[inline(always)]
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

    fn event(&self, event: Event, bounds: Bounds) {
        if let Some(view) = self {
            view.event(event, bounds)
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

    fn event(&self, event: Event, bounds: Bounds) {
        match self {
            Ok(view) => view.event(event, bounds),
            Err(view) => view.event(event, bounds),
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
