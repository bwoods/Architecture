use crate::gesture::{self, Id};
use crate::{Bounds, Event, Output, Size, View};
use composable::Effects;

pub struct TapGesture<V, A, E> {
    pub id: Id,
    pub view: V,
    pub action: A,
    pub send: E,
}

impl<V, A, E> View for TapGesture<V, A, E>
where
    V: View,
    A: Clone,
    E: Effects<Action = A>,
{
    #[inline(always)]
    fn size(&self, within: Size) -> Size {
        self.view.size(within)
    }

    #[inline]
    fn event(&self, event: Event, bounds: Bounds) {
        if let Ok((gesture, offset)) = event.try_into()
            && let Some(gesture::Response::UpInside) = gesture::recognizer(
                self.id,
                gesture,
                offset,
                Bounds::from_origin_and_size(bounds.min, self.size(bounds.size())),
            )
        {
            self.send.action(self.action.clone())
        }
    }

    #[inline(always)]
    fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
        self.view.draw(bounds, onto)
    }

    #[inline(always)]
    fn adjust_width(&self, width: f32) {
        self.view.adjust_width(width);
    }

    #[inline(always)]
    fn adjust_height(&self, height: f32) {
        self.view.adjust_height(height);
    }
}

pub struct Target<V> {
    pub(crate) view: V,
    pub(crate) minimum: Size,
}

impl<V: View> View for Target<V> {
    #[inline(always)]
    fn size(&self, within: Size) -> Size {
        self.view.size(within)
    }

    #[inline]
    fn event(&self, event: Event, bounds: Bounds) {
        let mut target = bounds;
        let size = self.size(bounds.size());

        target.min -= (self.minimum - size) / 2.0;
        target.set_size(self.minimum.max(size));

        match event {
            Event::Gesture(gesture, location) if target.contains_inclusive(location) => {
                self.view.event(
                    Event::Gesture(gesture, location.clamp(bounds.min, bounds.max)),
                    bounds,
                );
            }
            _ => self.view.event(event, bounds),
        }
    }

    #[inline]
    fn draw(&self, bounds: Bounds, onto: &mut impl Output) {
        self.view.draw(bounds, onto)
    }

    #[inline(always)]
    fn adjust_width(&self, width: f32) {
        self.view.adjust_width(width);
    }

    #[inline(always)]
    fn adjust_height(&self, height: f32) {
        self.view.adjust_height(height);
    }

    #[inline(always)]
    fn frac(&self) -> usize {
        self.view.frac()
    }
}
