use std::time::Duration;

use composable::dependencies::{with_dependency, Dependency};
use composable::effects::Interval;
use composable::views::gpu::Output;
use composable::views::ui::font::{body, label, title};
use composable::views::ui::{spacer, spacing, Inter};
use composable::views::View;
use composable::{Effects, From, Reducer, Task, TryInto};

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

    fn reduce(&mut self, action: Action, send: impl Effects<Action>) {
        match action {
            Action::Resize { width, height } => {
                self.wgpu.resize(width, height);

                send.throttle(
                    Action::Redraw,
                    &mut self.resizing,
                    Interval::Leading(Duration::from_secs_f32(1.0 / 100.0)),
                );
            }
            Action::Redraw => with_dependency(self.wgpu.transform(), || {
                let mut output = Output::new(8.0);
                self.view(send).draw(self.wgpu.bounds(), &mut output);

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

        let title = Dependency::<Inter<title::L>>::static_ref();
        let body = Dependency::<Inter<body::L>>::static_ref();
        let label = Dependency::<Inter<label::L>>::static_ref();

        let top = title.text(black, "This space intentionally left blank.");
        let right = body.text(black, "except for this bit on the right…");
        let bottom = label.text(black, "except for this bit on the bottom…");

        (
            self.header.view(effects.scope()),
            (
                (top, spacer::fill(), right).across(),
                spacer::fill(),
                bottom,
            )
                .padding_all(spacing::S),
        )
    }
}
