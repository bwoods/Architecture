use crate::commands::edit::*;
use muda::accelerator::{Accelerator, Code, Modifiers};
use muda::{MenuItem, PredefinedMenuItem, Submenu};
use ui_id::ui_id;

pub fn menu() -> Submenu {
    let menu = Submenu::new("Edit", true);
    menu.append_items(&[
        &Undo.menu(),
        &Redo.menu(),
        &PredefinedMenuItem::separator(),
        &Cut.menu(),
        &Copy.menu(),
        &Paste.menu(),
        &Delete.menu(),
        &PredefinedMenuItem::separator(),
        &PrevEdit.menu(),
        &NextEdit.menu(),
        &PredefinedMenuItem::separator(),
        &SelectAll.menu(),
        &SelectMore.menu(),
        &SelectLess.menu(),
        &SelectEvery.menu(),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id(
            DUPLICATE,
            "Duplicate",
            false,
            Some(Accelerator::new(Some(Modifiers::META), Code::KeyD)),
        ),
        &MenuItem::with_id(
            JOIN_LINES,
            "Join Lines",
            false,
            Some(Accelerator::new(
                Some(Modifiers::META | Modifiers::SHIFT),
                Code::KeyJ,
            )),
        ),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id(
            DUPLICATE,
            "Copy Line Up",
            false,
            Some(Accelerator::new(
                Some(Modifiers::SHIFT | Modifiers::ALT),
                Code::ArrowUp,
            )),
        ),
        &MenuItem::with_id(
            JOIN_LINES,
            "Copy Line Down",
            false,
            Some(Accelerator::new(
                Some(Modifiers::SHIFT | Modifiers::ALT),
                Code::ArrowDown,
            )),
        ),
        &MenuItem::with_id(
            DUPLICATE,
            "Move Line Up",
            false,
            Some(Accelerator::new(Some(Modifiers::ALT), Code::ArrowUp)),
        ),
        &MenuItem::with_id(
            JOIN_LINES,
            "Move Line Down",
            false,
            Some(Accelerator::new(Some(Modifiers::ALT), Code::ArrowDown)),
        ),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id(
            DUPLICATE,
            "Add Cursor Above",
            false,
            Some(Accelerator::new(
                Some(Modifiers::META | Modifiers::ALT),
                Code::ArrowUp,
            )),
        ),
        &MenuItem::with_id(
            JOIN_LINES,
            "Add Cursor Below",
            false,
            Some(Accelerator::new(
                Some(Modifiers::META | Modifiers::ALT),
                Code::ArrowDown,
            )),
        ),
        &MenuItem::with_id(
            JOIN_LINES,
            "Add Cursors to Line Ends",
            false,
            Some(Accelerator::new(
                Some(Modifiers::SHIFT | Modifiers::ALT),
                Code::KeyI,
            )),
        ),
    ])
    .unwrap();

    menu
}

pub const LAST_EDIT: u128 = ui_id!().get();
pub const NEXT_EDIT: u128 = ui_id!().get();
pub const DELETE: u128 = ui_id!().get();
pub const DUPLICATE: u128 = ui_id!().get();
pub const JOIN_LINES: u128 = ui_id!().get();
pub const SELECT_MORE: u128 = ui_id!().get();
pub const SELECT_LESS: u128 = ui_id!().get();
pub const SELECT_OCCURRENCES: u128 = ui_id!().get();
