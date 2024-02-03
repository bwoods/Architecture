use composable::views::ui::spacing;
use composable::{views::*, Effects, Reducer};

#[derive(Debug, Default)]
pub struct State {}

#[derive(Clone, Debug)]
pub enum Action {}

impl Reducer for State {
    type Action = Action;
    type Output = Self;

    fn reduce(&mut self, action: Action, _effects: impl Effects<Action>) {
        match action {}
    }
}

impl State {
    pub fn view(&self, effects: impl Effects<Action>) -> impl View {
        (
            Rectangle {
                rgba: [246, 248, 250, 255],
            }
            .fill()
            .height(spacing::XL),
            Rectangle {
                rgba: [217, 222, 227, 255],
            }
            .fill()
            .height(1.0),
        )
    }
}
