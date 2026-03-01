//! This Reducer exists as a bridge between the application’s main thread and
//! the tread running within the application’s `Store`.
//!
//! - Windowing changes instigated by the application itself are sent to this
//!   Reducer, which then forwards an event to its event loop.
//! - Windowing changes triggered by user actions, or other external events,
//!   begin in the event loop and sre sent to the application’s `Store`; ending
//!   up in this `Reducer`,

#[cfg(target_os = "macos")]
mod menu_bar;
mod options;
mod sizes;

use crate::versioning::{NAME, SOURCE, VERSION};
use crate::{ApplicationAction, rendering};
use composable::Store;
use dpi::PhysicalInsets;
use dpi::PhysicalSize;
use log::{debug, error, trace};
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, channel};
use winit::application::ApplicationHandler;
use winit::error::{EventLoopError, OsError, RequestError};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, DeviceEvents, EventLoop, EventLoopProxy};
use winit::window::{Theme, Window, WindowId};

pub struct State {
    window: WindowId,
    // options: Options,
    proxy: EventLoopProxy,
    send: Sender<ApplicationAction>,
}

/// These methods are used to tell the `MainThread` to perform various windowing
/// actions.
impl State {
    pub fn new(window: WindowId, proxy: EventLoopProxy, send: Sender<ApplicationAction>) -> Self {
        Self {
            window,
            proxy,
            send,
        }
    }

    pub fn visible(&self, visible: bool) {
        self.send
            .send(Action::Visible(self.window, visible).into())
            .unwrap_or_else(|err| error!(target: module_path!(), "{err}"));

        self.proxy.wake_up();
    }
}

/// `Action`s are dual purpose. The same `Action` type received by the reducer
///  is sent to the `MainThread` for windowing effects that, on some platforms,
///  muse be performed on the main thread.
#[derive(Clone)]
pub enum Action {
    MainWindow,
    Visible(WindowId, bool),
    Resized {
        size: PhysicalSize<u32>,
        bounds: PhysicalInsets<u32>,
    },
    Rescaled {
        scale: f64,
    },

    NewTab,
    CloseTab,
    Exit,
}

pub struct MainThread {
    target: BTreeMap<WindowId, Target>,
    snap: PhysicalSize<u32>,

    recv: Receiver<ApplicationAction>,
    send: Sender<ApplicationAction>,
}

impl MainThread {
    pub fn new() -> Self {
        let (send, recv) = channel();

        Self {
            target: Default::default(),
            snap: Default::default(),
            recv,
            send,
        }
    }

    fn new_window(&mut self, event_loop: &dyn ActiveEventLoop) -> Result<(), RequestError> {
        let attributes = options::read_prefs(event_loop)
            .inspect_err(|err| error!(target: module_path!(), "{err}"))
            .map_err(|err| RequestError::Os(OsError::new(line!(), file!(), err)))?
            .with_title(format!("{NAME} — v{VERSION} ({SOURCE})"))
            .with_theme(Some(Theme::Dark))
            .with_visible(false);

        let handle = event_loop.create_window(attributes.clone())?;
        let window: Arc<dyn Window> = Arc::from(handle);

        // > If you need to precisely position the top left corner of the whole window you have to
        // use Window::set_outer_position after creating the window.
        //
        //  — https://docs.rs/winit/latest/winit/window/struct.WindowAttributes.html#method.with_position
        if let Some(position) = attributes.position {
            window.set_outer_position(position)
        }

        let scale = window.scale_factor();
        let size = window.surface_size();

        let instance = wgpu::Instance::default();
        let surface = instance
            .create_surface(window.clone())
            .inspect_err(|err| error!(target: module_path!(), "{err}"))
            .map_err(|err| RequestError::Os(OsError::new(line!(), file!(), err)))?;
        let proxy = event_loop.create_proxy();
        let send = self.send.clone();
        let id = window.id();
        let bounds = window.safe_area();

        let store = Store::new(move || crate::State::new(size, scale, proxy, send, id));
        store.send(rendering::setup(surface, instance, bounds));

        self.target
            .insert(window.id(), Target::Windowed { store, window });

        Ok(())
    }

    pub fn run(self: Box<Self>) -> Result<(), EventLoopError> {
        let mut event_loop_builder = EventLoop::builder();

        #[cfg(target_os = "macos")]
        {
            use winit::platform::macos::EventLoopBuilderExtMacOS;
            event_loop_builder.with_default_menu(false);
        }

        let event_loop = event_loop_builder.build()?;
        event_loop.listen_device_events(DeviceEvents::Never);
        event_loop.set_control_flow(ControlFlow::Wait);

        #[cfg(target_os = "macos")] // must come after `event_loop_builder.build()`
        let _menu_bar = menu_bar::new(self.send.clone(), event_loop.create_proxy());

        event_loop.run_app(self)
    }
}

impl ApplicationHandler for MainThread {
    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        self.new_window(event_loop).expect("new_window");
    }

    fn proxy_wake_up(&mut self, event_loop: &dyn ActiveEventLoop) {
        use crate::ApplicationAction::*;

        while let Ok(event) = self.recv.try_recv() {
            match event {
                Windowing(Action::MainWindow) => {
                    let _ = self.new_window(event_loop);
                }
                Windowing(Action::Visible(id, visible)) => {
                    if let Some(Target::Windowed { window, .. }) = self.target.get(&id) {
                        window.set_visible(visible);
                    }
                }
                _ => {}
            }
        }
    }

    fn window_event(&mut self, event_loop: &dyn ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match &event {
            WindowEvent::RedrawRequested => {
                if let Some(Target::Windowed { store, .. }) = self.target.get(&id) {
                    store.send(ApplicationAction::Redraw)
                }
            }
            WindowEvent::SurfaceResized(size) => {
                if let Some(Target::Windowed { store, window }) = self.target.get(&id) {
                    trace!(target: module_path!(), "SurfaceResized(size: {size:?})");
                    let mut snap = sizes::fit_height(size.height);

                    trace!(target: module_path!(), "SurfaceResized(snap: {snap:?})");
                    if snap != *size {
                        // snap to this size instead of the one requested
                        match window.request_surface_size(snap.into()) {
                            Some(forced) => snap = forced, // snap rejected
                            None => return,                // snap accepted
                        }
                    }

                    if snap == self.snap {
                        return; // redundant resize request
                    } else {
                        self.snap = snap;
                    }

                    debug!(target: module_path!(), "Resizing: {snap:?})");
                    store.send(Action::Resized {
                        bounds: window.safe_area(),
                        size: snap,
                    })
                }
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                if let Some(Target::Windowed { store, .. }) = self.target.get(&id) {
                    store.send(Action::Rescaled {
                        scale: *scale_factor,
                    })
                }
            }
            WindowEvent::CloseRequested => {
                if let Some((_, Target::Windowed { store, window })) = self.target.remove_entry(&id)
                {
                    let _ = options::save_prefs(window); // FIXME:
                    let _ = store.into_inner(); // shutdown Store
                }
            }
            WindowEvent::Destroyed => {
                if self.target.is_empty() {
                    event_loop.exit(); // no windows remaining
                }
            }
            _ => {}
        }
    }

    fn destroy_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        self.target.clear();
    }
}

enum Target {
    Windowed {
        store: Store<crate::State, ApplicationAction>,
        window: Arc<dyn Window>,
    },
}
