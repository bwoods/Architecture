#![allow(dead_code)]

pub(crate) fn size_xxs(i: f32) -> f32 {
    // fi = f₀ × r^(i/n)
    let r = 1.618034f32; // Φ
    let n = 5.0f32;
    let f = 10.0f32;

    (f * r.powf(i / n)).round()
}

pub(crate) fn size_xs(i: f32) -> f32 {
    size_xxs(i)
        + match i {
            i if i < 6.5 => 1.0,
            _ => 2.0,
        }
}

pub(crate) fn size_s(i: f32) -> f32 {
    size_xs(i)
        + match i {
            i if i < 0.5 => 0.0,
            i if i < 6.5 => 1.0,
            _ => 2.0,
        }
}

pub(crate) fn size_m(i: f32) -> f32 {
    size_s(i)
        + match i {
            i if i < 6.5 => 1.0,
            _ => 2.0,
        }
}

pub(crate) fn size_l(i: f32) -> f32 {
    size_m(i)
        + match i {
            i if i < 0.5 => 0.0,
            i if i < 6.5 => 1.0,
            _ => 2.0,
        }
}

pub(crate) fn size_xl(i: f32) -> f32 {
    size_l(i) + 2.0
}

pub(crate) fn size_xxl(i: f32) -> f32 {
    size_xl(i) + 2.0
}

pub(crate) fn size_xxxl(i: f32) -> f32 {
    size_xxl(i) + 2.0
}
