use muda::{MenuItem, PredefinedMenuItem, Submenu};

pub fn menu() -> Submenu {
    let menu = Submenu::new("Help", true);
    menu.set_as_help_menu_for_nsapp();

    menu.append_items(&[
        &MenuItem::new("Check for Updates…", false, None),
        &PredefinedMenuItem::separator(),
        &MenuItem::new("What’s New…", false, None),
        &MenuItem::new("Getting Started", false, None),
    ])
    .unwrap();

    menu
}
