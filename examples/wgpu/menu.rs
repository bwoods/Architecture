#![allow(unused)]
use ::winit::window::Window;
use muda::{Menu, PredefinedMenuItem, Submenu};

use composable::*;

#[derive(Debug, Default)]
pub struct State {}

#[derive(Clone, Debug)]
pub enum Action {
    Setup(&'static Window),
}

impl Reducer for State {
    type Action = Action;
    type Output = ();

    fn into_inner(self) -> Self::Output {}

    fn reduce(&mut self, action: Self::Action, _effects: impl Effects<Action = Self::Action>) {
        match action {
            Action::Setup(_window) => {
                let menu_bar = Menu::new();

                #[cfg(target_os = "macos")]
                {
                    let app_m = Submenu::new("App", true);
                    menu_bar.append(&app_m);
                    app_m.append_items(&[
                        &PredefinedMenuItem::about(None, None),
                        &PredefinedMenuItem::separator(),
                        &PredefinedMenuItem::services(None),
                        &PredefinedMenuItem::separator(),
                        &PredefinedMenuItem::hide(None),
                        &PredefinedMenuItem::hide_others(None),
                        &PredefinedMenuItem::show_all(None),
                        &PredefinedMenuItem::separator(),
                        &PredefinedMenuItem::quit(None),
                    ]);
                }

                let file_m = Submenu::new("&File", true);
                let edit_m = Submenu::new("&Edit", true);
                let window_m = Submenu::new("&Window", true);

                menu_bar.append_items(&[&file_m, &edit_m, &window_m]);
                window_m.append_items(&[
                    &PredefinedMenuItem::minimize(None),
                    &PredefinedMenuItem::maximize(None),
                    &PredefinedMenuItem::close_window(Some("Close")),
                    &PredefinedMenuItem::fullscreen(None),
                    &PredefinedMenuItem::bring_all_to_front(None),
                ]);

                #[cfg(target_os = "windows")]
                {
                    use winit::raw_window_handle::*;
                    if let RawWindowHandle::Win32(handle) = window.window_handle().unwrap().as_raw()
                    {
                        menu_bar.init_for_hwnd(handle.hwnd.get());
                    }
                }

                #[cfg(target_os = "macos")]
                {
                    menu_bar.init_for_nsapp();
                    window_m.set_as_windows_menu_for_nsapp();
                }
            }
        }
    }
}
