#![allow(dead_code)]

const fn scale(n: u32) -> f32 {
    super::nth(n + 3, 3) // spacers use a ×3 scale
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
    assert_eq!(XXXS, 12.0);
    assert_eq!(XXS, 18.0);
    assert_eq!(XS, 30.0);
    assert_eq!(S, 48.0);
    assert_eq!(M, 72.0);
    assert_eq!(L, 120.0);
    assert_eq!(XL, 192.0);
    assert_eq!(XXL, 288.0);
    assert_eq!(XXXL, 480.0);
    assert_eq!(HUGE, 768.0);
}
