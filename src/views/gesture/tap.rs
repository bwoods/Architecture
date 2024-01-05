use crate::views::gesture::{self, Id};
use crate::views::{Bounds, Event, Output, Point, Size, View};
use crate::Effects;

pub struct TapGesture<V, E, A> {
    id: Id,
    view: V,
    action: A,
    effects: E,
}

impl<V, E, A> View for TapGesture<V, E, A>
where
    V: View,
    E: Effects<A>,
    A: Clone,
{
    #[inline(always)]
    fn size(&self) -> Size {
        self.view.size()
    }

    #[inline]
    fn event(&self, event: Event, offset: Point, bounds: Bounds) {
        if let Ok(gesture) = event.try_into() {
            if let Some(gesture::Response::UpInside) = gesture::recognizer(
                self.id,
                gesture,
                offset,
                Bounds::from_origin_and_size(bounds.min, self.size()),
            ) {
                self.effects.send(self.action.clone())
            }
        }
    }

    #[inline(always)]
    fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
        self.view.draw(bounds, onto)
    }
}
