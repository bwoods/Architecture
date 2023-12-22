#![allow(unused_imports)]

use composable::dependencies::Dependency;
use composable::views::minimal::{font, with_default_font, Inter};

use divan::{bench as benchmark, main as run_benchmarks};

use std::hint::black_box;

fn main() {
    run_benchmarks();
}

#[benchmark]
fn font_face_parsing() -> f32 {
    with_default_font(|| {
        let font: Dependency<Inter<font::body::M>> = Default::default();
        black_box(font.size())
    })
}
