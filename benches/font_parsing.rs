#![allow(unused_imports)]

use composable::dependencies::Dependency;
use composable::views::minimal::{with_default_font, Inter};

use divan::{bench as benchmark, main as run_benchmarks};

use std::hint::black_box;

fn main() {
    run_benchmarks();
}

#[benchmark]
fn font_face_parsing() -> f32 {
    with_default_font(|| {
        let inter: Dependency<Inter> = Default::default();
        inter.with(400.0, 14.0, |_face, scale| black_box(scale))
    })
    .unwrap()
}
