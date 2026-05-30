use crate::{rendering, windowing};
use composable::*;
use composable_views::{gpu::Output, *};
use log::{debug, error, warn};

mod clock;
mod player;

#[derive(Clone, From, TryInto, Reducers)]
pub enum Action {
    Event(Event),
    Windowing(windowing::Action),
    Rendering(rendering::Action),

    Player(player::Action),
    Clock(clock::Action),
}

#[derive(Composable)]
pub struct State {
    #[reducer(ignore)]
    windowing: windowing::State,
    rendering: rendering::State,
    player: player::State,
    clock: clock::State,
}

impl State {
    pub fn new(rendering: rendering::State, windowing: windowing::State) -> Self {
        let player = Default::default();
        let clock = Default::default();

        Self {
            rendering,
            windowing,
            player,
            clock,
        }
    }

    fn view(&self, send: impl Effects<Action = Action>) -> impl View {
        let player = self.player.view(send.scope()).padding_both(32.0, 36.0);
        let clock = self.clock.view(send.scope());
        let margin = player.size(Size::zero()).width;

        (
            player,
            Spacer::fill(),
            clock,
            Spacer::fill(),
            // match the width of the `player` so the clock may center properly
            Spacer::width(margin),
        )
            .inline()
    }
}

impl Reducers for State {
    type Action = Action;

    fn event(&mut self, event: Event, send: impl Effects<Action = Action>) {
        let size = self.rendering.logical_size();
        let bounds = Bounds::from_size(Size::new(size.width, size.height));

        self.view(send).event(event, bounds)
    }

    fn windowing(&mut self, action: windowing::Action, send: impl Effects<Action = Action>) {
        use windowing::Action::*;

        match action {
            Resize { width, height } => self.rendering.resize(width, height),
            Rescale { scale } => self.rendering.rescale(scale),
            Redraw => {
                let size = self.rendering.logical_size();
                let bounds = Bounds::from_size(Size::new(size.width, size.height));

                let mut output = Output::new(8.0, size.width, size.height);
                self.view(send.clone()).draw(bounds, &mut output);

                let (vertices, indices) = output.into_inner();
                self.rendering.render(&vertices, &indices, send.scope())
            }
        }
    }

    fn rendering(&mut self, action: rendering::Action, _send: impl Effects<Action = Action>) {
        use rendering::Action::*;

        match action {
            Ready(_) => self.windowing.visible(true),
            Error(err) => {
                use rendering::Error::*;

                match err {
                    Occluded => debug!(target: module_path!(), "{err:?}"),
                    Timeout => error!(target: module_path!(), "{err:?}"),
                    Suboptimal | Outdated => {
                        warn!(target: module_path!(), "{err:?}");
                        self.windowing.reset()
                    }
                    Lost | Validation => {
                        error!(target: module_path!(), "{err:?}");
                        self.windowing.reset()
                    }
                }
            }
            _ => {}
        }
    }

    fn player(&mut self, action: player::Action, send: impl Effects<Action = Self::Action>) {
        if let player::Action::Set(wake) = action {
            self.clock.show(wake);
        }

        send.action(windowing::Action::Redraw);
    }

    fn clock(&mut self, _action: clock::Action, send: impl Effects<Action = Action>) {
        send.action(windowing::Action::Redraw);
    }
}

/// Needed for [`Store::into_inner`]
impl From<State> for () {
    fn from(_: State) -> Self {}
}
