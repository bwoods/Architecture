mod header;

use composable::dependencies::with_dependency;
use composable::*;

use crate::wgpu;
use crate::window;

pub(crate) struct State {
    wgpu: wgpu::Surface,
    proxy: window::EventLoopProxy,
}

#[derive(Clone, Debug)]
pub enum Action {
    Resize { width: u32, height: u32 },
    Redraw,
}

impl Reducer for State {
    type Action = Action;
    type Output = Self;

    fn reduce(&mut self, action: Self::Action, effects: impl Effects<Self::Action>) {
        match action {
            Action::Resize { width, height } => {
                self.wgpu.resize(width, height);
            }
            Action::Redraw => with_dependency(self.wgpu.transform(), || {
                self.wgpu.render().ok();
            }),
        }
    }
}

impl State {
    pub fn new(wgpu: wgpu::Surface, proxy: window::EventLoopProxy) -> Self {
        Self { wgpu, proxy }
    }
}
