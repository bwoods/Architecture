#![allow(non_upper_case_globals)]

pub mod edit;
pub mod file;

use crate::ApplicationAction;
use keyboard_types::{Code, Modifiers};
use std::num::NonZeroU128;

#[derive(Clone)]
pub struct Accelerator(pub(super) Modifiers, pub(super) Code);

pub struct Command {
    pub action: ApplicationAction,
    pub keys: Option<Accelerator>,
    pub name: &'static str,
    pub id: NonZeroU128,
}

pub const All: &[Command] = &[
    file::NewWindow,
    file::NewTab,
    file::CloseTab,
    file::Exit,
    edit::Undo,
    edit::Redo,
    edit::Cut,
    edit::Copy,
    edit::Paste,
    edit::Delete,
    edit::PrevEdit,
    edit::NextEdit,
    edit::SelectAll,
    edit::SelectMore,
    edit::SelectLess,
    edit::SelectEvery,
    edit::Duplicate,
    edit::Join,
    edit::CopyUp,
    edit::CopyDown,
    edit::MoveUp,
    edit::MoveDown,
    edit::CursorAbove,
    edit::CursorBelow,
    edit::AppendCursors,
];
