use super::Command;
use crate::ApplicationAction::*;
use crate::windowing::Action;
use ui_id::ui_id;

pub const NewWindow: Command = Command {
    action: Windowing(Action::MainWindow),
    keys: None,
    name: "New Window",
    id: ui_id!(),
};

pub const NewTab: Command = Command {
    action: Windowing(Action::NewTab),
    keys: None,
    name: "New Tab",
    id: ui_id!(),
};

pub const CloseTab: Command = Command {
    action: Windowing(Action::CloseTab),
    keys: None,
    name: "Close",
    id: ui_id!(),
};

pub const Exit: Command = Command {
    action: Windowing(Action::Exit),
    keys: None,
    name: "Exit",
    id: ui_id!(),
};
