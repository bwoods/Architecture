use std::thread::Builder;

#[cfg(target_os = "macos")]
use muda::AboutMetadata;
use muda::{MenuEvent, MenuItem, PredefinedMenuItem, Submenu};
use winit::event_loop::EventLoop;
#[cfg(target_os = "macos")]
use winit::platform::macos::EventLoopBuilderExtMacOS;
#[cfg(target_os = "windows")]
use winit::platform::windows::EventLoopBuilderExtWindows;
use winit::window::Window;

pub use muda::Menu;
use winit::error::EventLoopError;

use window::Action;

use crate::window;

pub struct MenuBar {
    menu_bar: Menu,
    windows: Option<Submenu>,
}

pub fn build() -> Result<(Menu, EventLoop<Action>), EventLoopError> {
    let mut event_loop_builder = EventLoop::with_user_event();
    let menu_bar = Menu::new();

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

    #[cfg(target_os = "macos")]
    event_loop_builder.with_default_menu(false);

    let event_loop = event_loop_builder.build()?; // must come before the menus

    #[cfg(target_os = "macos")]
    {
        let application = Submenu::new("App", true);
        menu_bar.append(&application).unwrap();
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
    menu_bar.append_items(&[&windows]).unwrap();
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
    }

    #[cfg(target_os = "macos")]
    {
        let help = Submenu::new("Help", true);
        menu_bar.append_items(&[&help]).unwrap();

        let top = MenuItem::new("Share Ideas and Feedback…", true, None);
        help.append_items(&[&top]).unwrap();

        menu_bar.init_for_nsapp();
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
                    // Menu items are mapped to `window::Action`s here…
                    //
                    id if id == default_size => Action::DefaultSize,
                    _ => continue,
                };

                proxy.send_event(action).ok();
            }
        })
        .unwrap();

    Ok((menu_bar, event_loop))
}

pub fn attach(menu_bar: &Menu, window: &Window) {
    #[cfg(target_os = "windows")]
    {
        use winit::raw_window_handle::*;

        match window.window_handle().expect("window_handle").as_raw() {
            RawWindowHandle::Win32(handle) => menu_bar
                .init_for_hwnd(handle.hwnd.get())
                .expect("init_for_hwnd"),
            _ => unreachable!(),
        }
    }
}
