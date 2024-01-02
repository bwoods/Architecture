use std::thread::Builder;

#[cfg(target_os = "macos")]
use cocoa::appkit::{NSEvent, NSToolbar, NSWindow, NSWindowTitleVisibility};
#[cfg(target_os = "macos")]
use cocoa::base::id;
#[cfg(target_os = "macos")]
use muda::AboutMetadata;
use muda::{Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu};
use winit::event_loop::{EventLoop, EventLoopBuilder};
#[cfg(target_os = "macos")]
use winit::platform::macos::EventLoopBuilderExtMacOS;
#[cfg(target_os = "windows")]
use winit::platform::windows::EventLoopBuilderExtWindows;
#[cfg(target_os = "macos")]
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use winit::window::Window;

use window::Action;

use crate::window;

pub struct MenuBar {
    menu_bar: Menu,
    windows: Option<Submenu>,
}

impl MenuBar {
    pub fn new(event_loop_builder: &mut EventLoopBuilder<Action>) -> Self {
        let menu_bar = Menu::new();

        #[cfg(target_os = "macos")]
        event_loop_builder.with_default_menu(false);

        #[cfg(target_os = "windows")]
        {
            let menu_bar = menu_bar.clone();
            event_loop_builder.with_msg_hook(move |msg| {
                use windows_sys::Win32::UI::WindowsAndMessaging::{TranslateAcceleratorW, MSG};
                unsafe {
                    let msg = msg as *const MSG;
                    let translated = TranslateAcceleratorW((*msg).hwnd, menu_bar.haccel(), msg);
                    translated == 1
                }
            });
        }

        Self {
            menu_bar,
            windows: None,
        }
    }

    pub fn attach_to(&mut self, window: &Window, event_loop: &EventLoop<Action>) {
        #[cfg(target_os = "macos")]
        {
            set_toolbar_thickness(window, ToolbarThickness::Thick);

            let application = Submenu::new("App", true);
            self.menu_bar.append(&application).unwrap();
            application
                .append_items(&[
                    &PredefinedMenuItem::about(
                        None,
                        Some(AboutMetadata {
                            // add fields as needed
                            ..Default::default()
                        }),
                    ),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::services(None),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::hide(None),
                    &PredefinedMenuItem::hide_others(None),
                    &PredefinedMenuItem::show_all(None),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::quit(None),
                ])
                .unwrap();
        }

        let windows = Submenu::new("&Window", true);
        let default_size = MenuItem::new("Return to Default Size", true, None);
        self.menu_bar.append_items(&[&windows]).unwrap();
        windows
            .append_items(&[
                &PredefinedMenuItem::minimize(None),
                &PredefinedMenuItem::maximize(None),
                &PredefinedMenuItem::separator(),
                &PredefinedMenuItem::fullscreen(None),
                &default_size,
                &PredefinedMenuItem::separator(),
                &PredefinedMenuItem::bring_all_to_front(None),
                // &PredefinedMenuItem::separator(),
            ])
            .unwrap();

        #[cfg(target_os = "windows")]
        {
            windows
                .append_items(&[
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::close_window(Some("Exit")),
                ])
                .unwrap();

            use winit::raw_window_handle::*;
            if let RawWindowHandle::Win32(handle) = window.window_handle().unwrap().as_raw() {
                self.menu_bar.init_for_hwnd(handle.hwnd.get()).ok();
            }
        }

        #[cfg(target_os = "macos")]
        {
            let help = Submenu::new("Help", true);
            self.menu_bar.append_items(&[&help]).unwrap();

            let top = MenuItem::new("Share Ideas and Feedback…", true, None);
            help.append_items(&[&top]).unwrap();

            self.menu_bar.init_for_nsapp();
            windows.set_as_windows_menu_for_nsapp();
            help.set_as_help_menu_for_nsapp();
        }

        // gather the ids for the polling thread
        let default_size = default_size.id().clone();
        let proxy = event_loop.create_proxy();

        Builder::new()
            .name("menu".into())
            .spawn(move || {
                while let Ok(event) = MenuEvent::receiver().recv() {
                    let action = match event.id {
                        id if id == default_size => Action::DefaultSize,
                        _ => continue,
                    };

                    proxy.send_event(action).ok();
                }
            })
            .unwrap();

        self.windows = Some(windows); // must be after winit has been started
    }
}

#[cfg(target_os = "macos")]
enum ToolbarThickness {
    Thick,
    Medium,
    Thin,
}

#[cfg(target_os = "macos")]
fn set_toolbar_thickness(window: &Window, thickness: ToolbarThickness) {
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
