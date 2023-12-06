use ::winit::error::EventLoopError;
use ::winit::event::{Event, WindowEvent};
use ::winit::event_loop::ControlFlow;
use composable::*;

mod winit;

fn main() -> Result<(), EventLoopError> {
    let (state, event_loop) = winit::State::new();
    let mut store = Store::blocking(state);

    event_loop.set_control_flow(ControlFlow::Wait); // turn off polling
    event_loop.run(move |event, target| {
        use winit::Action::*;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                target.exit();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                let (width, height) = size.into();
                store.send(Resize { width, height });
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                store.send(Render);
            }
            _ => {}
        }
    })
}
