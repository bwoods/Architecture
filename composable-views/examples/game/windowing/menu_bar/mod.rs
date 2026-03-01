mod about;
mod audio;
mod edit;
mod file;
mod help;
mod video;
mod view;
mod window;

use crate::ApplicationAction;
use crate::commands::Command;
use muda::accelerator::Accelerator;
use muda::{Menu, MenuEvent, MenuItem};
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use winit::event_loop::EventLoopProxy;

pub fn new(send: Sender<ApplicationAction>, proxy: EventLoopProxy) -> Menu {
    let mut commands = HashMap::<String, ApplicationAction>::default();

    let menu_bar = Menu::new();
    menu_bar.init_for_nsapp();

    menu_bar.append(&about::menu(&mut commands)).unwrap();
    menu_bar.append(&file::menu()).unwrap();
    menu_bar.append(&edit::menu()).unwrap();
    menu_bar.append(&view::menu()).unwrap();
    menu_bar.append(&video::menu()).unwrap();
    menu_bar.append(&audio::menu()).unwrap();

    let window = window::menu();
    window.set_as_windows_menu_for_nsapp();
    menu_bar.append(&window).unwrap();

    let help = help::menu();
    help.set_as_help_menu_for_nsapp();
    menu_bar.append(&help).unwrap();

    MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
        if let Some(action) = commands.get(&event.id.0) {
            let _ = send.send(action.clone());
        }
    }));

    menu_bar
}

impl Command {
    fn menu(&self) -> MenuItem {
        MenuItem::with_id(
            self.id,
            self.name,
            matches!(self.action, ApplicationAction::Noop) == false,
            self.keys
                .clone()
                .map(|keys| Accelerator::new(Some(keys.0), keys.1)),
        )
    }
}
