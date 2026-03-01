use super::{Accelerator, Command};
use crate::ApplicationAction::*;
use keyboard_types::{Code, Modifiers};
use ui_id::ui_id;

pub const Undo: Command = Command {
    action: Noop,
    keys: None,
    name: "Undo",
    id: ui_id!(),
};

pub const Redo: Command = Command {
    action: Noop,
    keys: None,
    name: "Redo",
    id: ui_id!(),
};

pub const Cut: Command = Command {
    action: Noop,
    keys: None,
    name: "Cut",
    id: ui_id!(),
};

pub const Copy: Command = Command {
    action: Noop,
    keys: None,
    name: "Copy",
    id: ui_id!(),
};

pub const Paste: Command = Command {
    action: Noop,
    keys: None,
    name: "Paste",
    id: ui_id!(),
};

pub const Delete: Command = Command {
    action: Noop,
    keys: None,
    name: "Delete",
    id: ui_id!(),
};

pub const PrevEdit: Command = Command {
    action: Noop,
    keys: None,
    name: "Last Edit Location",
    id: ui_id!(),
};

pub const NextEdit: Command = Command {
    action: Noop,
    keys: None,
    name: "Next Edit Location",
    id: ui_id!(),
};

pub const SelectAll: Command = Command {
    action: Noop,
    keys: Some(Accelerator(Modifiers::META, Code::KeyA)),
    name: "Select All",
    id: ui_id!(),
};

pub const SelectMore: Command = Command {
    action: Noop,
    keys: Some(Accelerator(
        Modifiers::union(Modifiers::META, Modifiers::SHIFT),
        Code::KeyA,
    )),
    name: "Expand Selection",
    id: ui_id!(),
};

pub const SelectLess: Command = Command {
    action: Noop,
    keys: Some(Accelerator(
        Modifiers::union(Modifiers::ALT, Modifiers::SHIFT),
        Code::KeyA,
    )),
    name: "Shrink Selection",
    id: ui_id!(),
};

pub const SelectEvery: Command = Command {
    action: Noop,
    keys: None,
    name: "Select All Occurrences",
    id: ui_id!(),
};

pub const Duplicate: Command = Command {
    action: Noop,
    keys: None,
    name: "Duplicate",
    id: ui_id!(),
};

pub const Join: Command = Command {
    action: Noop,
    keys: None,
    name: "Join Lines",
    id: ui_id!(),
};

pub const CopyUp: Command = Command {
    action: Noop,
    keys: None,
    name: "Copy Line Up",
    id: ui_id!(),
};

pub const CopyDown: Command = Command {
    action: Noop,
    keys: None,
    name: "Copy Line Down",
    id: ui_id!(),
};

pub const MoveUp: Command = Command {
    action: Noop,
    keys: None,
    name: "Move Line Up",
    id: ui_id!(),
};

pub const MoveDown: Command = Command {
    action: Noop,
    keys: None,
    name: "Move Line Down",
    id: ui_id!(),
};

pub const CursorAbove: Command = Command {
    action: Noop,
    keys: None,
    name: "Add Cursor Above",
    id: ui_id!(),
};

pub const CursorBelow: Command = Command {
    action: Noop,
    keys: None,
    name: "Add Cursor Below",
    id: ui_id!(),
};

pub const AppendCursors: Command = Command {
    action: Noop,
    keys: None,
    name: "Add Cursor to Line Ends",
    id: ui_id!(),
};
