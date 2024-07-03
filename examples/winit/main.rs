#![allow(dead_code)]
#![allow(unused_variables)]

use futures::executor::block_on;
use std::collections::BTreeMap;
use std::sync::Arc;

use composable::Store;
use window::Action;

use window::menubar::Menu;
use window::{
    ActiveEventLoop, ApplicationHandler, ControlFlow, DeviceEvents, EventLoopError, EventLoopProxy,
    LogicalSize, Size, StartCause, Window, WindowEvent, WindowId,
};

mod frames;
mod wgpu;
mod window;

struct State {
    stores: BTreeMap<WindowId, (Store<frames::State>, Arc<Window>)>,

    proxy: EventLoopProxy,
    menubar: Menu,
}

impl State {
    fn front_window(&self) -> Option<&Window> {
        self.stores
            .values()
            .map(|tuple| &*tuple.1)
            .find(|window| window.has_focus())
    }
}

impl ApplicationHandler<Action> for State {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        match cause {
            StartCause::Init => self.proxy.send_event(Action::NewWindow).unwrap(),
            _ => {}
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {}

    fn user_event(&mut self, active: &ActiveEventLoop, event: Action) {
        match event {
            Action::NewWindow => {
                let window = Arc::new(window::build(&active));
                window::menubar::attach(&self.menubar, &window);

                let id = window.id();
                let proxy = self.proxy.clone();
                let wgpu = block_on(wgpu::Surface::new(window.clone())); // must be on main thread

                let store = Store::new(move || frames::State::new(wgpu, proxy, id));
                self.stores.insert(id, (store, window));
            }
            Action::DefaultSize => {
                if let Some(window) = self.front_window() {
                    let size = Size::from(LogicalSize::<f32>::from(window::DEFAULT_SIZE));
                    let _ = window.request_inner_size(size);
                }
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::RedrawRequested => {
                self.stores
                    .entry(id)
                    .and_modify(|store| store.0.send(frames::Action::Redraw));
            }
            WindowEvent::Resized(size) => {
                self.stores.entry(id).and_modify(|store| {
                    let (width, height) = size.into();
                    let resize = frames::Action::Resize { width, height };

                    store.0.send(resize);
                });
            }
            WindowEvent::CloseRequested => {
                self.stores.remove(&id);
                if self.stores.is_empty() {
                    event_loop.exit();
                }
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), EventLoopError> {
    let (menubar, event_loop) = window::menubar::build()?;

    event_loop.listen_device_events(DeviceEvents::Never);
    event_loop.set_control_flow(ControlFlow::Wait);

    let proxy = event_loop.create_proxy();

    let mut state = State {
        stores: BTreeMap::default(),
        menubar,
        proxy,
    };

    event_loop.run_app(&mut state)
}
