use composable::{Effects, Reducer};
use composable_views::ui::spacing;
use composable_views::*;

#[derive(Debug, Default)]
pub struct State {}

#[derive(Clone, Debug)]
pub enum Action {}

impl Reducer for State {
    type Action = Action;
    type Output = Self;

    fn reduce(&mut self, action: Action, send: impl Effects<Action>) {
        match action {}
    }
}

impl State {
    pub fn view(&self, send: impl Effects<Action>) -> impl View {
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
