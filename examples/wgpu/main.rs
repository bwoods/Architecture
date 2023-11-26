use winit::dpi::LogicalSize;
use winit::dpi::Position::Logical;
use winit::error::EventLoopError;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy};
use winit::window::WindowBuilder;

use composable::*;

mod wgpu;

#[derive(Debug, Default)]
struct State {
    wgpu: Option<wgpu::State>,
}

#[derive(Debug)]
enum Action {
    Render,
    Resize {
        width: u32,
        height: u32,
    },
    Shutdown(EventLoopProxy<()>),
    Setup {
        instance: wgpu::Instance,
        surface: wgpu::Surface<'static>,
        width: u32,
        height: u32,
    },
}

impl Reducer for State {
    type Action = Action;

    #[allow(clippy::option_map_unit_fn)]
    async fn reduce_async(&mut self, action: Action, _effects: impl Effects<Action = Action>) {
        use Action::*;

        match action {
            Render => {
                self.wgpu.as_mut().map(|state| state.render());
            }
            Resize { width, height } => {
                self.wgpu.as_mut().map(|state| state.resize(width, height));
            }
            Shutdown(proxy) => {
                self.wgpu.take(); // tear down the wgpu::State
                proxy.send_event(()).ok();
            }
            #[rustfmt::skip]
            Setup { instance, surface, width, height } => {
                self.wgpu = Some(wgpu::State::new(instance, surface, width, height).await);
            }
        }
    }

    type Output = ();

    fn into_inner(self) -> Self::Output {}
}

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new().unwrap();
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
                // let window = window.take().unwrap();
                let (width, height) = window.inner_size().into();

                let instance = wgpu::Instance::default();
                let surface = instance.create_surface(window).unwrap();
                // note: “On macOS/Metal: will panic if not called on the main thread.”
                //        So we the surface here…

                store.send(Setup {
                    instance,
                    surface,
                    width,
                    height,
                });
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
