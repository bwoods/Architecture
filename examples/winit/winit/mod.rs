use futures::executor::block_on;
use ouroboros::self_referencing;

use winit::dpi::{LogicalSize, Position::Logical};
use winit::event::WindowEvent;
use winit::event_loop::{EventLoop, EventLoopBuilder, EventLoopProxy};
use winit::window::{Window, WindowBuilder};

pub use winit::error::EventLoopError;
pub use winit::event::Event;
pub use winit::event_loop::ControlFlow;

use composable::*;

mod menu;

mod wgpu;

#[derive(SelfReferentialRecursiveReducer)]
#[self_referencing]
pub struct State {
    #[not_a_reducer]
    proxy: EventLoopProxy<()>,

    #[not_a_reducer]
    window: Window,

    #[borrows(window)] // ⬅︎ THIS is what makes this struct “self referential”
    #[not_covariant] // do not bother generating a `borrow` method for this
    wgpu: wgpu::State<'this>, // lifetime generated/replaced by `ouroboros` macro
    menu: menu::State,
}

#[derive(Clone, From, TryInto)]
pub enum Action {
    Render,
    Redraw,
    Resize { width: u32, height: u32 },

    Winit(Event<()>),
    Menu(menu::Action),
    Wgpu(wgpu::Action),
}

impl RecursiveReducer for State {
    type Action = Action;

    fn reduce(&mut self, action: Action, effects: impl Effects<Action = Action>) {
        use Action::*;

        match action {
            Redraw => self.borrow_window().request_redraw(),
            Render => effects.send(wgpu::Action::Render),
            Resize { width, height } => {
                effects.send(wgpu::Action::Resize { width, height });
                effects.send(Redraw);
            }

            Winit(event) => match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    // call back out to the `event_loop` in main to exit
                    self.borrow_proxy().send_event(()).ok();
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    let (width, height) = size.into();
                    effects.send(Resize { width, height });
                }
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => effects.send(Render),
                _ => {}
            },

            Menu(_) => {}
            Wgpu(_) => {}
        }
    }
}

impl State {
    pub(crate) fn build() -> (Self, EventLoop<()>) {
        let mut event_loop_builder = EventLoopBuilder::new();
        let mut menu = menu::State::new(&mut event_loop_builder);

        let event_loop = event_loop_builder.build().unwrap();
        let proxy = event_loop.create_proxy();

        let window = WindowBuilder::new()
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
            .unwrap();

        menu.attach_to(&window);

        // use of the ouroboros crate changes how we build State
        let state = StateBuilder {
            wgpu_builder: |window| block_on(wgpu::State::new(window)),
            window,
            proxy,
            menu,
        }
        .build();

        (state, event_loop)
    }
}
