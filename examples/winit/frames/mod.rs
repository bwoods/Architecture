use composable::dependencies::{with_dependency, Dependency};
use composable::views::ui::font::body;
use composable::views::ui::{spacer, spacing, Inter};
use composable::views::{Transform, View};
use composable::{Effects, From, Reducer, Task, TryInto};

use std::time::Duration;

use crate::{wgpu, window};

mod header;

pub struct State {
    wgpu: wgpu::Surface<'static>,
    window: window::WindowId,
    proxy: window::EventLoopProxy,

    resizing: Option<Task>,

    header: header::State,
}

#[derive(Clone, Debug, From, TryInto)]
pub enum Action {
    Resize { width: u32, height: u32 },
    Redraw,

    Header(header::Action),
}

impl Reducer for State {
    type Action = Action;
    type Output = Self;

    fn reduce(&mut self, action: Action, effects: impl Effects<Action>) {
        match action {
            Action::Resize { width, height } => {
                self.wgpu.resize(width, height);

                effects.throttle(
                    Action::Redraw,
                    &mut self.resizing,
                    Interval::Leading(Duration::from_secs_f32(1.0 / 100.0)),
                );
            }
            Action::Redraw => with_dependency(self.wgpu.transform(), || {
                let mut output = Output::new(8.0);
                self.view(effects).draw(self.wgpu.bounds(), &mut output);

                let (vertices, indices) = output.into_inner();
                self.wgpu.render(&vertices, &indices).ok();
            }),
            Action::Header(_) => {
                //
            }
        }
    }
}

impl State {
    pub fn new(
        wgpu: wgpu::Surface<'static>,
        proxy: window::EventLoopProxy,
        window: window::WindowId,
    ) -> Self {
        Self {
            wgpu,
            proxy,
            window,

            resizing: None,
            header: Default::default(),
        }
    }

    pub fn view(&self, effects: impl Effects<Action>) -> impl View {
        let black = [0, 0, 0, 0xff];

        let medium = Dependency::<Inter<body::M>>::static_ref();
        let small = Dependency::<Inter<body::S>>::static_ref();

        let body = medium.text(black, "This space intentionally left blank.");
        let caption = small.text(black, "except for this, I mean…");

        (
            self.header.view(effects.scope()),
            (body, spacer::fill(), caption)
                // .across()
                .padding_all(spacing::S),
        )
    }
}
