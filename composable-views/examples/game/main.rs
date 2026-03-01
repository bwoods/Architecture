#![allow(dead_code)]
#![allow(unused_variables)]

mod commands;
mod preferences;
mod rendering;
mod versioning;
mod windowing;

use composable::*;
use dpi::PhysicalSize;
use log::error;
use std::sync::mpsc::Sender;
use winit::event_loop::EventLoopProxy;
use winit::window::WindowId;

#[derive(Composable)]
struct State {
    rendering: rendering::State,

    #[reducer(ignore)]
    windowing: windowing::State,
}

/// ApplicationAction, rather than Action so that features such as `windowing`
/// and `menu_bar`
#[derive(Clone, From, TryInto, Reducers)]
enum ApplicationAction {
    Rendering(rendering::Action),
    Windowing(windowing::Action),
    Redraw,
    Noop,
}

impl Reducers for State {
    type Action = ApplicationAction;

    fn rendering(&mut self, action: rendering::Action, send: impl Effects<Action = Self::Action>) {
        if matches!(action, rendering::Action::Initialized(..)) {
            self.windowing.visible(true);
        }
    }

    fn windowing(&mut self, action: windowing::Action, send: impl Effects<Action = Self::Action>) {
        match action {
            windowing::Action::Resized { size, bounds } => self.rendering.resize(size, bounds),
            windowing::Action::Rescaled { scale } => self.rendering.rescale(scale),
            _ => {}
        }
    }

    fn redraw(&mut self, send: impl Effects<Action = Self::Action>) {
        if let Err(err) = self.rendering.render(&[], &[]) {
            error!(target: module_path!(), "{err}")
        }
    }

    fn noop(&mut self, send: impl Effects<Action = Self::Action>) {}
}

impl State {
    pub fn new(
        size: PhysicalSize<u32>,
        scale: f64,
        proxy: EventLoopProxy,
        send: Sender<ApplicationAction>,
        id: WindowId,
    ) -> Self {
        let pending = rendering::State::Pending {
            size,
            scale,
            vsync: Default::default(),
        };

        let windowing = windowing::State::new(id, proxy, send);

        Self {
            rendering: pending,
            windowing,
        }
    }
}

fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Error)
        .init()
        .unwrap();

    Box::new(windowing::MainThread::new()).run().unwrap();
}
