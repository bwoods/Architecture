use muda::{CheckMenuItem, MenuItem, PredefinedMenuItem, Submenu};
use ui_id::ui_id;

pub fn menu() -> Submenu {
    let lang = Submenu::new("Language", true);
    lang.append(&CheckMenuItem::new("English", false, true, None))
        .unwrap();

    let menu = Submenu::new("Audio", true);
    menu.append_items(&[
        &MenuItem::with_id(SHOW_AUDIO_OPTIONS, "Show Audio Options", true, None),
        &PredefinedMenuItem::separator(),
        &lang,
        &PredefinedMenuItem::separator(),
        &MenuItem::new("Volume: 100", false, None),
        &MenuItem::with_id(VOLUME_INCREASE, "Volume Increase", true, None),
        &MenuItem::with_id(VOLUME_DECREASE, "Volume Decrease", true, None),
        &CheckMenuItem::with_id(MUTE_AUDIO, "Mute Audio", true, false, None),
        &CheckMenuItem::with_id(MUTE_IN_BACKGROUND, "Mute in Background", true, true, None),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id(SHOW_SUBTITLE_OPTIONS, "Show Subtitle Options", true, None),
        &MenuItem::new("Show Subtitle Options", false, None),
        &CheckMenuItem::with_id(HIDE_SUBTITLES, "Hide Subtitles", true, false, None),
    ])
    .unwrap();

    menu
}

pub const SHOW_AUDIO_OPTIONS: u128 = ui_id!().get();
pub const SHOW_SUBTITLE_OPTIONS: u128 = ui_id!().get();
pub const VOLUME_INCREASE: u128 = ui_id!().get();
pub const VOLUME_DECREASE: u128 = ui_id!().get();
pub const MUTE_AUDIO: u128 = ui_id!().get();
pub const MUTE_IN_BACKGROUND: u128 = ui_id!().get();
pub const HIDE_SUBTITLES: u128 = ui_id!().get();
