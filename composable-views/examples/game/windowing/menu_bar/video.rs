use muda::{MenuItem, PredefinedMenuItem, Submenu};
use ui_id::ui_id;

pub fn menu() -> Submenu {
    let menu = Submenu::new("Video", true);
    menu.append_items(&[
        &PredefinedMenuItem::close_window(Some("Show Video Options")),
        &PredefinedMenuItem::separator(),
        &MenuItem::new("Return to Default Size", false, None),
        &MenuItem::with_id(WINDOW_SIZE_INCREASE, "Larger Video", true, None),
        &MenuItem::with_id(WINDOW_SIZE_DECREASE, "Smaller Video", true, None),
        &PredefinedMenuItem::separator(),
        &MenuItem::new("Tablet Sizes", false, None),
        &MenuItem::with_id(TABLET_S, "\u{3000}1024 × 768", true, None),
        &MenuItem::with_id(TABLET_M, "\u{3000}1280 × 800", true, None),
        &PredefinedMenuItem::separator(),
        &MenuItem::new("Display Sizes", false, None),
        &MenuItem::with_id(HD, "\u{3000}720p", true, None),
        &MenuItem::with_id(HD_PLUS, "\u{3000}900p", true, None),
        &MenuItem::with_id(HD_FULL, "\u{3000}1080p", true, None),
        &MenuItem::with_id(HD_QUAD, "\u{3000}1440p", true, None),
        &MenuItem::with_id(HD_3K, "\u{3000}3K", true, None),
        &MenuItem::with_id(HD_4K, "\u{3000}4K", true, None),
        &MenuItem::with_id(HD_5K, "\u{3000}5K", true, None),
        &MenuItem::with_id(HD_8K, "\u{3000}8K", true, None),
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
