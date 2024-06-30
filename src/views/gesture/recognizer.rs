use crate::dependencies::Dependency;
use crate::views::{Bounds, Gesture, Point};

use super::{Id, State};

///
#[inline(never)]
pub fn recognizer(id: Id, gesture: Gesture, location: Point, bounds: Bounds) -> Option<Response> {
    let current = Dependency::<State>::new();
    let mut state = current.get();

    let id = Some(id);
    let inside = (state.active.is_none() || state.active == id) && bounds.contains(location);
    // only ‘inside’ if someone else isn’t already active

    const Y: bool = true; // to make the table below easier to read
    const N: bool = false;

    use Response::*;
    #[rustfmt::skip] // keep the table compact and legible
    let response = match (state.active == id, state.hover == id, inside, gesture) {
        (_, _, Y, Gesture::Began {..}) => { state.active = id; state.hover = id; Some(DownInside) },
        (Y, _, Y, Gesture::Ended {..}) => { state.active = None; state.hover = None; Some(UpInside) },
        (Y, _, N, Gesture::Ended {..}) => { state.active = None; state.hover = None; Some(UpOutside) },
        (Y, Y, Y, Gesture::Moved {..}) => Some(DragInside),
        (Y, N, N, Gesture::Moved {..}) => Some(DragOutside),
        (Y, Y, N, Gesture::Moved {..}) => { state.hover = None; Some(DragExit) },
        (Y, N, Y, Gesture::Moved {..}) => { state.hover = id; Some(DragEnter) },
        _ => None
    };

    current.set(state);
    response
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Response {
    DownInside,
    UpInside,
    UpOutside,
    DragInside,
    DragExit,
    DragOutside,
    DragEnter,
}
