use winit::{ControlFlow, Event, EventLoopError};

use composable::*;

mod winit;

fn main() -> Result<(), EventLoopError> {
    let (state, event_loop) = winit::State::build();
    let mut store = Store::blocking(state);

    event_loop.set_control_flow(ControlFlow::Wait); // turn off polling

    event_loop.run(move |event, target| match event {
        Event::UserEvent(..) => target.exit(),
        _ => store.send(event),
    })
}
