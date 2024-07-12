pub use winit::application::ApplicationHandler;
pub use winit::dpi::{LogicalSize, Position, Size};
pub use winit::error::EventLoopError;
pub use winit::event::{StartCause, WindowEvent};
pub use winit::event_loop::{ActiveEventLoop, ControlFlow, DeviceEvents};
pub use winit::window::{Window, WindowId};

pub type EventLoopProxy = winit::event_loop::EventLoopProxy<Action>;

pub mod menubar;

pub const DEFAULT_SIZE: (f32, f32) = (1366.0, 1024.0);
pub const MIN_SIZE: (f32, f32) = (1024.0, 768.0);

#[derive(Clone, Debug)]
pub enum Action {
    NewWindow,
    DefaultSize,
}

pub fn build(event_loop: &ActiveEventLoop) -> Window {
    #[allow(unused_mut)]
    let mut attributes = Window::default_attributes()
        .with_title("")
        .with_theme(None) // None → current
        .with_position(Position::Logical(Default::default()))
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
        use winit::platform::macos::WindowAttributesExtMacOS;

        attributes = attributes
            .with_titlebar_transparent(true)
            .with_fullsize_content_view(true)
    }

    let window = event_loop.create_window(attributes).expect("create_window");

    #[cfg(target_os = "macos")]
    set_toolbar_thickness(&window, ToolbarThickness::Medium);

    window
}

#[cfg(target_os = "macos")]
enum ToolbarThickness {
    Thick,
    Medium,
    Thin,
}

#[cfg(target_os = "macos")]
fn set_toolbar_thickness(window: &Window, thickness: ToolbarThickness) {
    use cocoa::appkit::{NSEvent, NSWindow};
    use cocoa::appkit::{NSToolbar, NSWindowTitleVisibility};
    use cocoa::base::id;

    use wgpu::rwh::{HasWindowHandle, RawWindowHandle};

    unsafe {
        let id = match window.window_handle().unwrap().as_raw() {
            RawWindowHandle::AppKit(raw) => raw.ns_view.as_ptr() as id,
            RawWindowHandle::UiKit(raw) => raw.ui_view.as_ptr() as id,
            _ => unreachable!(),
        }
        .window();

        id.setTitlebarAppearsTransparent_(cocoa::base::YES);

        let make_toolbar = |id: id| {
            let new_toolbar = NSToolbar::alloc(id);
            new_toolbar.init_();
            id.setToolbar_(new_toolbar);
        };

        match thickness {
            ToolbarThickness::Thick => {
                window.set_title("");
                make_toolbar(id);
            }
            ToolbarThickness::Medium => {
                id.setTitleVisibility_(NSWindowTitleVisibility::NSWindowTitleHidden);
                make_toolbar(id);
            }
            ToolbarThickness::Thin => {
                id.setTitleVisibility_(NSWindowTitleVisibility::NSWindowTitleHidden);
            }
        }
    }
}
