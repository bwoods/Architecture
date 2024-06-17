use composable::{dependencies::with_dependency, views::View, Effects, From, Reducer, TryInto};

use crate::{wgpu, window};

mod header;

pub struct State {
    wgpu: wgpu::Surface<'static>,
    window: window::WindowId,
    proxy: window::EventLoopProxy,

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
            }
            Action::Redraw => with_dependency(self.wgpu.transform(), || {
                use composable::views::gpu::Output;
                use lyon::lyon_tessellation::FillOptions;

                let options = FillOptions::default();
                let mut output = Output::new(options);

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

            header: Default::default(),
        }
    }

    pub fn view(&self, effects: impl Effects<Action>) -> impl View {
        (self.header.view(effects.scope()),)
    }
}
