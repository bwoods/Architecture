use futures::executor::block_on;
use winit::dpi::{LogicalSize, Position::Logical};
use winit::event_loop::{EventLoop, EventLoopBuilder};
use winit::window::{Window, WindowBuilder};

use composable::*;

mod menu;

mod wgpu;

pub struct State {
    #[allow(dead_code)]
    menu: menu::State,
    wgpu: wgpu::State,

    window: Box<Window>, // must be dropped after wgpu
}

#[derive(Clone, Debug)]
pub enum Action {
    Render,
    Resize { width: u32, height: u32 },
}

impl Reducer for State {
    type Action = Action;
    type Output = ();

    fn into_inner(self) -> Self::Output {}

    fn reduce(&mut self, action: Self::Action, _effects: impl Effects<Action = Self::Action>) {
        match action {
            Action::Render => {
                self.wgpu.render().ok();
            }
            Action::Resize { width, height } => {
                self.wgpu.resize(width, height);
                self.window.request_redraw();
            }
        }
    }
}

impl State {
    pub(crate) fn new() -> (Self, EventLoop<()>) {
        let mut event_loop_builder = EventLoopBuilder::new();
        let mut menu = menu::State::new(&mut event_loop_builder);

        let event_loop = event_loop_builder.build().unwrap();
        let window = Box::new(
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
        );

        menu.attach_to(&window);

        let raw = Box::<Window>::into_raw(window);
        let window: &'static _ = unsafe { raw.as_ref() }.unwrap();
        let wgpu = block_on(wgpu::State::new(window));

        let window = unsafe { Box::from_raw(raw) };
        let state = Self { window, menu, wgpu };

        (state, event_loop)
    }
}
