#![allow(unused_imports)]

use composable::dependencies::Dependency;
use composable::views::ui::{font, with_default_fonts, Inter};

use divan::{bench as benchmark, main as run_benchmarks};

use std::hint::black_box;

fn main() {
    run_benchmarks();
}

#[benchmark(min_time = 2)]
fn font_face_parsing(bencher: divan::Bencher) {
    with_default_fonts(|| {
        bencher.bench_local(|| {
            let font: Dependency<Inter<font::body::M>> = black_box(Default::default());
            black_box(font.size())
        });
    });
}
