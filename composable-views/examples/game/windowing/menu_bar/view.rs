use muda::accelerator::{Accelerator, Code, Modifiers};
use muda::{CheckMenuItem, MenuItem, PredefinedMenuItem, Submenu};
use ui_id::ui_id;

pub fn menu() -> Submenu {
    let menu = Submenu::new("View", true);
    menu.append_items(&[
        &MenuItem::with_id(
            SHOW_COMMANDS,
            "Show All Commands…",
            false,
            Some(Accelerator::new(
                Some(Modifiers::META | Modifiers::SHIFT),
                Code::KeyP,
            )),
        ),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id(
            NAVIGATE_BACK,
            "Navigate Back",
            true,
            Some(Accelerator::new(
                Some(Modifiers::META | Modifiers::CONTROL),
                Code::ArrowLeft,
            )),
        ),
        &MenuItem::with_id(
            NAVIGATE_FWD,
            "Navigate Forward",
            true,
            Some(Accelerator::new(
                Some(Modifiers::META | Modifiers::CONTROL),
                Code::ArrowRight,
            )),
        ),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id(
            FONT_SIZE_DEFAULT,
            "Default Font Size",
            true, //
            None,
        ),
        &MenuItem::with_id(
            FONT_SIZE_UP,
            "Increase Font Size",
            true,
            Some(Accelerator::new(
                Some(Modifiers::META | Modifiers::SHIFT),
                Code::Period,
            )),
        ),
        &MenuItem::with_id(
            FONT_SIZE_DOWN,
            "Decrease Font Size",
            true,
            Some(Accelerator::new(Some(Modifiers::META), Code::Period)),
        ),
        &PredefinedMenuItem::separator(),
        &CheckMenuItem::with_id(
            SCROLL_TOP,
            "Scroll to Top",
            true,
            false,
            Some(Accelerator::new(Some(Modifiers::META), Code::Home)),
        ),
        &MenuItem::with_id(
            SCROLL_BOTTOM,
            "Scroll to Bottom",
            true,
            Some(Accelerator::new(Some(Modifiers::META), Code::End)),
        ),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id(
            SCROLL_PAGE_UP,
            "Scroll Page Up",
            true,
            Some(Accelerator::new(Some(Modifiers::META), Code::PageUp)),
        ),
        &MenuItem::with_id(
            SCROLL_PAGE_DOWN,
            "Scroll Page Down",
            true,
            Some(Accelerator::new(Some(Modifiers::META), Code::PageDown)),
        ),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id(
            SCROLL_LINE_UP,
            "Scroll Line Up",
            true,
            Some(Accelerator::new(
                Some(Modifiers::META | Modifiers::ALT),
                Code::PageUp,
            )),
        ),
        &MenuItem::with_id(
            SCROLL_LINE_DOWN,
            "Scroll Line Down",
            true,
            Some(Accelerator::new(
                Some(Modifiers::META | Modifiers::ALT),
                Code::PageDown,
            )),
        ),
    ])
    .unwrap();

    menu
}

pub const SHOW_COMMANDS: u128 = ui_id!().get();
pub const NAVIGATE_BACK: u128 = ui_id!().get();
pub const NAVIGATE_FWD: u128 = ui_id!().get();
pub const FONT_SIZE_DEFAULT: u128 = ui_id!().get();
pub const FONT_SIZE_UP: u128 = ui_id!().get();
pub const FONT_SIZE_DOWN: u128 = ui_id!().get();
pub const SCROLL_TOP: u128 = ui_id!().get();
pub const SCROLL_BOTTOM: u128 = ui_id!().get();
pub const SCROLL_PAGE_UP: u128 = ui_id!().get();
pub const SCROLL_PAGE_DOWN: u128 = ui_id!().get();
pub const SCROLL_LINE_UP: u128 = ui_id!().get();
pub const SCROLL_LINE_DOWN: u128 = ui_id!().get();
