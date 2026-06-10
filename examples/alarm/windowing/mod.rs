use composable::*;
use muda::Menu;
use std::sync::mpsc::Sender;
use tracing::error;
use winit::event_loop::EventLoopProxy;
use winit::window::WindowId;

mod menu_bar;

#[derive(Clone, From, TryInto)]
pub enum Action {
    Resize { width: u32, height: u32 },
    Rescale { scale: f32 },
    Redraw,
}

pub struct State {
    proxy: EventLoopProxy,
    send: Sender<crate::Action>,
    id: WindowId,
}

impl State {
    pub fn new(proxy: EventLoopProxy, send: Sender<crate::Action>, id: WindowId) -> Self {
        Self { proxy, send, id }
    }

    pub fn menu_bar(proxy: EventLoopProxy, send: Sender<crate::Action>) -> Menu {
        menu_bar::new(proxy, send)
    }

    pub fn visible(&self, visible: bool) {
        let _ = self
            .send
            .send(crate::Action::Visible(self.id, visible))
            .inspect_err(|err| error!(target: module_path!(), "{err}")) //
            .map(|_| self.proxy.wake_up());
    }

    pub fn reset(&self) {
        let _ = self
            .send
            .send(crate::Action::Reset(self.id))
            .inspect_err(|err| error!(target: module_path!(), "{err}")) //
            .map(|_| self.proxy.wake_up());
    }
}
