use muda::accelerator::{Accelerator, Code, Modifiers};
use muda::{MenuItem, PredefinedMenuItem, Submenu};
use ui_id::ui_id;

pub fn menu() -> Submenu {
    let menu = Submenu::new("Window", true);
    menu.append_items(&[
        &PredefinedMenuItem::minimize(None),
        &PredefinedMenuItem::maximize(None), // Zoom
        &MenuItem::new(
            "Fill",
            true,
            Some(Accelerator::new(
                Some(Modifiers::CONTROL | Modifiers::FN),
                Code::KeyF,
            )),
        ),
        &MenuItem::new(
            "Center",
            true,
            Some(Accelerator::new(
                Some(Modifiers::CONTROL | Modifiers::FN),
                Code::KeyC,
            )),
        ),
        &PredefinedMenuItem::separator(),
        &PredefinedMenuItem::close_window(Some("Close Tab")), // Disable when one tab?
        // &PredefinedMenuItem::separator(),
        // &MenuItem::new("Move to…", false, None),
        &PredefinedMenuItem::separator(),
        &MenuItem::new("Rename Tab…", false, None),
        &MenuItem::new("Always Show Tabs", false, None),
        &MenuItem::new("Show All Tabs", false, None),
        &PredefinedMenuItem::separator(),
        &MenuItem::new(
            "Show Previous Tab",
            true,
            Some(Accelerator::new(
                Some(Modifiers::CONTROL | Modifiers::SHIFT),
                Code::Tab,
            )),
        ),
        &MenuItem::new(
            "Show Next Tab",
            true,
            Some(Accelerator::new(Some(Modifiers::CONTROL), Code::Tab)),
        ),
        &MenuItem::new("Mode Tab to New Window", false, None),
        &MenuItem::new("Merge All Windows", false, None),
        &PredefinedMenuItem::separator(),
        &MenuItem::new("Show All Tabs", false, None),
        &PredefinedMenuItem::separator(),
        &PredefinedMenuItem::fullscreen(None),
        &PredefinedMenuItem::separator(),
        &PredefinedMenuItem::bring_all_to_front(None),
    ])
    .unwrap();

    menu
}

pub const WINDOW_SIZE_INCREASE: u128 = ui_id!().get();
pub const WINDOW_SIZE_DECREASE: u128 = ui_id!().get();
pub const TABLET_S: u128 = ui_id!().get();
pub const TABLET_M: u128 = ui_id!().get();
pub const HD: u128 = ui_id!().get();
pub const HD_PLUS: u128 = ui_id!().get();
pub const HD_FULL: u128 = ui_id!().get();
pub const HD_QUAD: u128 = ui_id!().get();
pub const HD_3K: u128 = ui_id!().get();
pub const HD_4K: u128 = ui_id!().get();
pub const HD_5K: u128 = ui_id!().get();
pub const HD_8K: u128 = ui_id!().get();
