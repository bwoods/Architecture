#![allow(clippy::type_complexity)]
use crate::{versioning::NAME, versioning::SOURCE, versioning::VERSION};
use composable::*;
use composable_views::{Event, Gesture};
use dpi::LogicalSize;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, channel};
use tracing::{error, trace};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, DeviceEvents, EventLoop};
use winit::window::{Theme, Window, WindowAttributes, WindowId};

mod alarm;
mod rendering;
mod versioning;
mod windowing;

#[path = "../foundations/inter/mod.rs"]
mod inter;
#[path = "../foundations/nord/mod.rs"]
mod nord;

#[derive(Clone, From, TryInto)]
pub enum Action {
    MainWindow,
    Visible(WindowId, bool),
    Reset(WindowId),

    View(Event),
}

pub struct State {
    target: BTreeMap<WindowId, (Store<alarm::State, ()>, Arc<dyn Window>)>,
    recv: Receiver<Action>,
    send: Sender<Action>,
}

impl ApplicationHandler for State {
    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        self.new_window(event_loop);
    }

    /// Used by `Reducer`s to forward `Action`s to the main thread
    fn proxy_wake_up(&mut self, event_loop: &dyn ActiveEventLoop) {
        while let Ok(event) = self.recv.try_recv() {
            match event {
                Action::MainWindow => {
                    self.new_window(event_loop);
                }
                Action::Visible(id, visible) => {
                    if let Some((_, window)) = self.target.get(&id) {
                        window.set_visible(visible);
                    }
                }
                Action::Reset(id) => {
                    if let Some((store, window)) = self.target.get(&id) {
                        let instance = wgpu::Instance::default();
                        let surface = instance
                            .create_surface(window.clone())
                            .inspect_err(|err| error!(target: module_path!(), "{err}"))
                            .expect("surface");

                        store.send(rendering::setup(surface, instance));
                    }
                }
                _ => {}
            }
        }
    }

    fn window_event(&mut self, event_loop: &dyn ActiveEventLoop, id: WindowId, event: WindowEvent) {
        #[allow(clippy::bool_comparison)]
        if cfg!(debug_assertions) == false
            && matches!(event, WindowEvent::PointerMoved { .. }) == false
        {
            trace!(target: module_path!(), "{event:?}");
        }

        match event {
            WindowEvent::RedrawRequested | WindowEvent::Occluded(false) => {
                if let Some((store, ..)) = self.target.get(&id) {
                    store.send(windowing::Action::Redraw)
                }
            }
            WindowEvent::PointerButton {
                state,
                position,
                button,
                ..
            } => {
                let n = match button.clone().mouse_button() {
                    Some(which) => which as u8,
                    None => {
                        error!(target: module_path!(), "{button:?}");
                        return;
                    }
                };

                let gesture = match state {
                    ElementState::Pressed => Gesture::Began { n },
                    ElementState::Released => Gesture::Ended { n },
                };

                if let Some((store, window)) = self.target.get(&id) {
                    let logical = position.to_logical(window.scale_factor());
                    store.send(Event::Gesture(gesture, (logical.x, logical.y).into()))
                }
            }
            WindowEvent::SurfaceResized(size) => {
                if let Some((store, window)) = self.target.get(&id) {
                    let fixed = self.fixed_size().to_physical(window.scale_factor());

                    if size != fixed && window.fullscreen().is_none() {
                        match window.request_surface_size(fixed.into()) {
                            None => return, // request accepted
                            Some(forced) => {
                                debug_assert_eq!(size, forced); // request rejected
                            }
                        }
                    }

                    store.send(windowing::Action::Resize {
                        width: size.width,
                        height: size.height,
                    })
                }
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                if let Some((store, ..)) = self.target.get(&id) {
                    store.send(windowing::Action::Rescale {
                        scale: scale_factor as f32,
                    })
                }
            }
            WindowEvent::CloseRequested => {
                if let Some((_, (store, _))) = self.target.remove_entry(&id) {
                    store.into_inner(); // shutdown Store
                }
            }
            WindowEvent::Destroyed if self.target.is_empty() => {
                event_loop.exit(); // no windows remaining
            }
            _ => {}
        }
    }

    fn destroy_surfaces(&mut self, _event_loop: &dyn ActiveEventLoop) {
        self.target.clear()
    }
}

impl State {
    fn fixed_size(&self) -> LogicalSize<u32> {
        LogicalSize::new(1024, 768)
        // LogicalSize::new(800, 680)
    }

    pub fn new_window(&mut self, event_loop: &dyn ActiveEventLoop) {
        let mut attributes = WindowAttributes::default()
            .with_title(format!("{NAME} — v{VERSION} ({SOURCE})"))
            .with_theme(Some(Theme::Dark))
            .with_min_surface_size(self.fixed_size())
            .with_resizable(true)
            .with_visible(false);

        #[cfg(target_os = "macos")]
        {
            use winit::platform::macos::WindowAttributesMacOS;
            use winit::window::PlatformWindowAttributes;

            attributes = attributes.with_platform_attributes(
                WindowAttributesMacOS::default()
                    .with_titlebar_transparent(true)
                    .with_fullsize_content_view(true)
                    .box_clone(),
            );
        }

        let handle = event_loop
            .create_window(attributes.clone())
            .expect("window");

        let window: Arc<dyn Window> = Arc::from(handle);

        // > If you need to precisely position the top left corner of the whole window you have to
        // use Window::set_outer_position after creating the window.
        //
        //  — https://docs.rs/winit/latest/winit/window/struct.WindowAttributes.html#method.with_position
        if let Some(position) = attributes.position {
            window.set_outer_position(position)
        }

        let scale = window.scale_factor() as f32;
        let size = window.surface_size();

        let rendering = rendering::State::Pending { size, scale };
        let windowing =
            windowing::State::new(event_loop.create_proxy(), self.send.clone(), window.id());

        let store = Store::new(move || alarm::State::new(rendering, windowing));
        let proxy = event_loop.create_proxy();
        let id = window.id();

        self.target.insert(window.id(), (store, window));
        self.send.send(Action::Reset(id)).expect("setup");
        proxy.wake_up();
    }

    pub fn run() {
        let mut event_loop_builder = EventLoop::builder();

        #[cfg(target_os = "macos")]
        {
            use winit::platform::macos::EventLoopBuilderExtMacOS;
            event_loop_builder.with_default_menu(false);
        }

        let event_loop = event_loop_builder.build().expect("event_loop");
        event_loop.listen_device_events(DeviceEvents::Never);
        event_loop.set_control_flow(ControlFlow::Wait);

        let (send, recv) = channel();

        #[cfg(target_os = "macos")] // must come after `event_loop_builder.build()`
        let _menu_bar = windowing::State::menu_bar(event_loop.create_proxy(), send.clone());

        event_loop
            .run_app(Box::new(State {
                target: Default::default(),
                recv,
                send,
            }))
            .expect("running")
    }
}

fn main() {
    use tracing_subscriber::layer::SubscriberExt;

    tracing::subscriber::set_global_default(
        tracing_subscriber::registry().with(tracing_tracy::TracyLayer::default()),
    )
    .expect("setup tracy layer");

    let level = if cfg!(debug_assertions) {
        log::LevelFilter::Info
    } else {
        log::LevelFilter::Error
    };

    simple_logger::SimpleLogger::new()
        .with_level(level)
        .init()
        .unwrap();

    State::run()
}
