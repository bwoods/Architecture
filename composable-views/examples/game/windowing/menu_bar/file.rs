use muda::accelerator::{Accelerator, Code, Modifiers};
use muda::{MenuItem, PredefinedMenuItem, Submenu};
use ui_id::ui_id;

pub fn menu() -> Submenu {
    let new = Submenu::new("New", true);
    new.append_items(&[
        &MenuItem::with_id(NEW_WIN, "New Window", false, None),
        &MenuItem::with_id(NEW_TAB, "New Tab", false, None),
    ])
    .unwrap();

    let recent = Submenu::new("Open Recent…", false);

    let menu = Submenu::new("File", true);
    menu.append_items(&[
        &new,
        &MenuItem::with_id(CLOSE, "Close", false, None),
        &PredefinedMenuItem::separator(),
        &MenuItem::new("Open…", false, None),
        &recent,
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id(
            HISTORY,
            "Play History",
            false,
            Some(Accelerator::new(
                Some(Modifiers::META | Modifiers::SHIFT),
                Code::KeyH,
            )),
        ),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id(VERIFY, "Verify Caches…", false, None),
        &MenuItem::with_id(OPTIMIZE, "Optimize Caches…", false, None),
        &MenuItem::with_id(RESET, "Reset Caches…", false, None),
    ])
    .unwrap();

    menu
}

pub const NEW_WIN: u128 = ui_id!().get();
pub const NEW_TAB: u128 = ui_id!().get();
pub const CLOSE: u128 = ui_id!().get();
pub const HISTORY: u128 = ui_id!().get();
pub const VERIFY: u128 = ui_id!().get();
pub const OPTIMIZE: u128 = ui_id!().get();
pub const RESET: u128 = ui_id!().get();
