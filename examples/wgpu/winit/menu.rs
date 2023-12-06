use muda::{Menu, MenuItem, PredefinedMenuItem, Submenu};
#[cfg(target_os = "macos")]
use muda::AboutMetadata;
use winit::event_loop::EventLoopBuilder;
#[cfg(target_os = "macos")]
use winit::platform::macos::EventLoopBuilderExtMacOS;
#[cfg(target_os = "windows")]
use winit::platform::windows::EventLoopBuilderExtWindows;
use winit::window::Window;

use composable::*;

pub struct State {
    menu_bar: Menu,
    windows: Option<Submenu>,
}

#[derive(Clone, Debug)]
pub enum Action {}

impl Reducer for State {
    type Action = Action;
    type Output = ();

    fn into_inner(self) -> Self::Output {}

    fn reduce(&mut self, action: Self::Action, _effects: impl Effects<Action = Self::Action>) {
        match action {}
    }
}

impl State {
    pub fn new(event_loop_builder: &mut EventLoopBuilder<()>) -> Self {
        let menu_bar = Menu::new();

        #[cfg(target_os = "macos")]
        event_loop_builder.with_default_menu(false);

        #[cfg(target_os = "windows")]
        {
            let menu_bar = menu_bar.clone();
            event_loop_builder.with_msg_hook(move |msg| {
                use windows_sys::Win32::UI::WindowsAndMessaging::{MSG, TranslateAcceleratorW};
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

    // Note: `_window` is only needed when `cfg(target_os = "windows")`
    pub fn attach_to(&mut self, _window: &Window) {
        #[cfg(target_os = "macos")]
        {
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
        self.menu_bar.append_items(&[&windows]).unwrap();
        windows
            .append_items(&[
                &PredefinedMenuItem::minimize(None),
                &PredefinedMenuItem::maximize(None),
                &PredefinedMenuItem::separator(),
                &PredefinedMenuItem::fullscreen(None),
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
            if let RawWindowHandle::Win32(handle) = _window.window_handle().unwrap().as_raw() {
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

        self.windows = Some(windows); // must be after winit has been started
    }
}
