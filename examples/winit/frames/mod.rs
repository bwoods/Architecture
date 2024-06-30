use composable::dependencies::Dependency;
use composable::views::ui::font::{body, title};
use composable::views::ui::Inter;
use composable::views::Transform;
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
                use lyon::lyon_tessellation::{FillOptions, FillRule};

                let transform = *Dependency::<Transform>::new().unwrap();
                let scale = f32::max(transform.m11, -transform.m21);
                let tolerance = f32::min(std::f32::consts::FRAC_1_PI * scale, 0.000125);

                let options = FillOptions::default()
                    .with_fill_rule(FillRule::NonZero)
                    .with_intersections(true)
                    .with_tolerance(tolerance);

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
        let black = [0, 0, 0, 0xff];

        let large = Dependency::<Inter<title::L>>::static_ref();
        let small = Dependency::<Inter<body::S>>::static_ref();

        let title = large.text(black, "This space intentionally left blank.");
        let body = small.text(black, "except for this, of course…");

        (self.header.view(effects.scope()), title, body)
    }
}
