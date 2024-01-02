#![allow(dead_code)]
#![allow(unused_variables)]

use futures::executor::block_on;

use composable::Store;
use window::{Action, ControlFlow, Event, EventLoopError, LogicalSize, Size, WindowEvent};

mod frames;
mod wgpu;
mod window;

fn main() -> Result<(), EventLoopError> {
    let (window, menu, event_loop) = window::build();
    event_loop.set_control_flow(ControlFlow::Wait); // turn off polling

    let proxy = event_loop.create_proxy();
    let wgpu = block_on(wgpu::Surface::new(&window));

    let store = Store::new(move || frames::State::new(wgpu, proxy));

    event_loop.run(move |event, target| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => target.exit(),
        Event::WindowEvent {
            event: WindowEvent::Resized(size),
            ..
        } => {
            let (width, height) = size.into();
            store.send(frames::Action::Resize { width, height });
            window.request_redraw();
        }
        Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } => {
            store.send(frames::Action::Redraw);
        }

        Event::UserEvent(Action::DefaultSize) => {
            let _ = window
                .request_inner_size(Size::from(LogicalSize::<f32>::from(window::DEFAULT_SIZE)));
        }
        _ => {}
    })
}
