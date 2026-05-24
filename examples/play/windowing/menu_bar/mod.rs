use muda::{Menu, MenuEvent};
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use winit::event_loop::EventLoopProxy;

pub mod about;

use crate::Action;

pub fn new(proxy: EventLoopProxy, send: Sender<Action>) -> Menu {
    let mut commands = HashMap::<String, Action>::default();
    let menu_bar = Menu::new();

    #[cfg(target_os = "macos")]
    {
        menu_bar.init_for_nsapp();
        menu_bar.append(&about::menu(&mut commands)).unwrap();
    }

    MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
        if let Some(action) = commands.get(&event.id.0) {
            let _ = send.send(action.clone());
            proxy.wake_up();
        }
    }));

    menu_bar
}
