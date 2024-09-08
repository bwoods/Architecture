use iui::controls::{Area, AreaHandler, Window, WindowType};
use iui::{UIError, UI};

struct Scrolling {}

impl AreaHandler for Scrolling {}

fn main() -> Result<(), UIError> {
    let title = "Untitled";
    let (w, h) = (1024, 4096);

    let ui = UI::init()?;
    let mut win = Window::new(&ui, title, w, h, WindowType::HasMenubar);

    let scroll = Scrolling {};
    let area = Area::new_scrolling(&ui, Box::new(scroll), w as i64, h as i64);

    win.set_margined(&ui, false);
    win.set_child(&ui, area);
    win.show(&ui);

    ui.main();
    Ok(())
}
