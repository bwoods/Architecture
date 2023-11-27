use winit::dpi::LogicalSize;
use winit::dpi::Position::Logical;
use winit::error::EventLoopError;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy};
use winit::window::{Window, WindowBuilder};

#[allow(unused)]
#[cfg(target_os = "macos")]
use winit::platform::macos::{EventLoopBuilderExtMacOS, WindowExtMacOS};
#[cfg(target_os = "windows")]
use winit::platform::windows::{EventLoopBuilderExtWindows, WindowExtWindows};

use composable::*;

mod menu;
mod wgpu;

#[derive(Debug, Default, Reducers)]
struct State {
    menu: menu::State,
    wgpu: wgpu::State,
}

#[derive(Clone, Debug, From, TryInto)]
enum Action {
    Render,
    Resize { width: u32, height: u32 },
    Shutdown(EventLoopProxy<()>),
    Setup(&'static Window),
    Menu(menu::Action),
    Wgpu(wgpu::Action),
}

impl State {
    async fn reduce_async(&mut self, action: Action, effects: impl Effects<Action = Action>) {
        use Action::*;

        match action {
            Render => {
                self.wgpu.render().ok();
            }
            Resize { width, height } => {
                self.wgpu.resize(width, height);
            }
            Shutdown(proxy) => {
                proxy.send_event(()).ok();
            }
            Setup(window) => {
                // effects.send(menu::Action::Setup(window));
                effects.send(wgpu::Action::Setup(window));
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), EventLoopError> {
    let mut event_loop_builder = EventLoopBuilder::new();

    // #[cfg(target_os = "macos")]
    // event_loop_builder.with_default_menu(false);

    let event_loop = event_loop_builder.build().unwrap();
    let event_loop_proxy = event_loop.create_proxy();

    event_loop.set_control_flow(ControlFlow::Wait); // turn off polling

    let window: &'static _ = Box::leak(Box::new(
        WindowBuilder::new()
            .with_title("")
            .with_theme(None) // None → current
            .with_position(Logical(Default::default()))
            .with_inner_size(LogicalSize {
                width: 1366.0,
                height: 1024.0,
            })
            .with_min_inner_size(LogicalSize {
                width: 1024.0,
                height: 768.0,
            })
            .build(&event_loop)
            .unwrap(),
    ));

    let store = Store::<State>::new(Default::default);

    event_loop.run(move |event, target| {
        use Action::*;

        match event {
            Event::NewEvents(StartCause::Init) => {
                store.send(Setup(window));
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                store.send(Shutdown(event_loop_proxy.clone()));
            }
            Event::UserEvent(()) => {
                target.exit();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                let (width, height) = size.into();
                store.send(Resize { width, height });

                // https://raphlinus.github.io/rust/gui/2019/06/21/smooth-resize-test.html
                //   Specifically: Synchronous delivery of events
                //
                // Have we re-created the problem they solved?

                // store.send(Render); // slams the CPU and lags the animation
                window.request_redraw();
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
