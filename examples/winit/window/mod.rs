pub use winit::dpi::{LogicalSize, Position::Logical, Size};
pub use winit::error::EventLoopError;
pub use winit::event::{Event, WindowEvent};
pub use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};
pub use winit::window::{Window, WindowBuilder};

pub type EventLoopProxy = winit::event_loop::EventLoopProxy<Action>;

mod menu;

pub const DEFAULT_SIZE: (f32, f32) = (1366.0, 1024.0);
pub const MIN_SIZE: (f32, f32) = (1024.0, 768.0);

#[derive(Clone, Debug)]
pub enum Action {
    DefaultSize,
}

pub fn build() -> (Window, menu::MenuBar, EventLoop<Action>) {
    let mut event_loop_builder = EventLoopBuilder::<Action>::with_user_event();

    let mut menu = menu::MenuBar::new(&mut event_loop_builder);
    let event_loop = event_loop_builder.build().unwrap();

    #[allow(unused_mut)]
    let mut builder = WindowBuilder::new()
        .with_title("")
        .with_theme(None) // None → current
        .with_position(Logical(Default::default()))
        .with_inner_size(LogicalSize {
            width: DEFAULT_SIZE.0,
            height: DEFAULT_SIZE.1,
        })
        .with_min_inner_size(LogicalSize {
            width: MIN_SIZE.0,
            height: MIN_SIZE.1,
        });

    #[cfg(target_os = "macos")]
    {
        use winit::platform::macos::WindowBuilderExtMacOS;

        builder = builder
            .with_titlebar_transparent(true)
            .with_fullsize_content_view(true)
    }

    let window = builder.build(&event_loop).unwrap();
    menu.attach_to(&window, &event_loop);

    (window, menu, event_loop)
}
