#![allow(dead_code)]

const fn scale(n: u32) -> f32 {
    super::nth(n, 4) // fonts use a ×4 scale
}

pub const XXXS: f32 = scale(1);
pub const XXS: f32 = scale(2);
pub const XS: f32 = scale(3);
pub const S: f32 = scale(4);
pub const M: f32 = scale(5);
pub const L: f32 = scale(6);
pub const XL: f32 = scale(7);
pub const XXL: f32 = scale(8);
pub const XXXL: f32 = scale(9);
pub const HUGE: f32 = scale(10);

#[test]
fn documenting_resulting_values() {
    assert_eq!(XXXS, 4.0);
    assert_eq!(XXS, 6.0);
    assert_eq!(XS, 10.0);
    assert_eq!(S, 16.0);
    assert_eq!(M, 24.0);
    assert_eq!(L, 40.0);
    assert_eq!(XL, 64.0);
    assert_eq!(XXL, 96.0);
    assert_eq!(XXXL, 160.0);
    assert_eq!(HUGE, 256.0);
}
