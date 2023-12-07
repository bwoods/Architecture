use futures::executor::block_on;
use winit::dpi::{LogicalSize, Position::Logical};
use winit::event_loop::{EventLoop, EventLoopBuilder};
use winit::window::{Window, WindowBuilder};

use composable::*;

mod menu;

mod wgpu;

#[derive(RecursiveReducer)]
pub struct State {
    menu: menu::State,
    wgpu: wgpu::State,

    #[not_a_reducer]
    window: Box<Window>, // must be dropped after wgpu
}

#[derive(Clone, From, TryInto)]
pub enum Action {
    Resize { width: u32, height: u32 },
    Render,
    Redraw,

    Menu(menu::Action),
    Wgpu(wgpu::Action),
}

impl RecursiveReducer for State {
    type Action = Action;

    fn reduce(&mut self, action: Action, effects: impl Effects<Action = Action>) {
        match action {
            Action::Render => {
                effects.send(wgpu::Action::Render);
            }
            Action::Resize { width, height } => {
                effects.send(wgpu::Action::Resize { width, height });
                effects.send(Action::Redraw);
            }
            Action::Redraw => {
                self.window.request_redraw();
            }
            Action::Menu(_) => {}
            Action::Wgpu(_) => {}
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
