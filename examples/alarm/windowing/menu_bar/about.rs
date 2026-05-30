use crate::Action;
use crate::versioning::{AUTHORS, DESCRIPTION, HOMEPAGE, NAME, SOURCE, VERSION};
use chrono::Datelike;
use composable_views::ui_id;
use itertools::Itertools;
use muda::accelerator::{Accelerator, Code, Modifiers};
use muda::{AboutMetadata, MenuItem, PredefinedMenuItem, Submenu};
use std::collections::HashMap;

pub fn menu(_commands: &mut HashMap<String, Action>) -> Submenu {
    let menu = Submenu::new("…", true);
    menu.append_items(&[
        &PredefinedMenuItem::about(
            None,
            Some(AboutMetadata {
                name: Some(NAME.to_owned()),
                authors: Some(AUTHORS.split(":").map(str::to_owned).collect()),
                version: Some(VERSION.to_owned()),
                short_version: Some(SOURCE.to_owned()),
                comments: Some(DESCRIPTION.to_owned()),
                website: Some(HOMEPAGE.to_owned()),
                copyright: Some(
                    format!(
                        "Copyright © 2024–{} {}.\nAll rights reserved.",
                        chrono::Local::now().year(),
                        AUTHORS.split(":").join(", ")
                    )
                    .to_owned(),
                ),
                ..Default::default()
            }),
        ),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id(
            SETTINGS,
            "Settings…",
            true,
            Some(Accelerator::new(Some(Modifiers::META), Code::Comma)),
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

    menu
}

pub const SETTINGS: u128 = ui_id!().get();
