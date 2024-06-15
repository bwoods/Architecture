#![allow(dead_code)]
#![allow(unused_variables)]

use futures::executor::block_on;
use std::sync::Arc;

use composable::Store;
use window::{
    Action, ControlFlow, Event, EventLoopError, LogicalSize, Size, StartCause, WindowEvent,
};

mod frames;
mod wgpu;
mod window;

fn main() -> Result<(), EventLoopError> {
    let (window, menu, event_loop) = window::build();
    event_loop.set_control_flow(ControlFlow::Wait); // turn off polling
    let id = window.id();

    let window = Arc::new(window);
    let proxy = event_loop.create_proxy();
    let wgpu = block_on(wgpu::Surface::new(window.clone()));

    let store = Store::new(move || frames::State::new(wgpu, proxy, id));

    event_loop.run(move |event, target| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            window_id,
            ..
        } => target.exit(),
        Event::WindowEvent {
            event: WindowEvent::Resized(size),
            window_id,
            ..
        } => {
            let (width, height) = size.into();
            store.send(frames::Action::Resize { width, height });
            window.request_redraw();
        }
        Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            window_id,
            ..
        } => {
            store.send(frames::Action::Redraw);
        }

        Event::UserEvent(Action::DefaultSize) => {
            let _ = window
                .request_inner_size(Size::from(LogicalSize::<f32>::from(window::DEFAULT_SIZE)));
        }
        Event::NewEvents(StartCause::Init) => {
            //
        }
        _ => {}
    })
}
