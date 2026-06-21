#![doc = include_str!("README.md")]
pub mod fonts;
pub mod spacing;

pub const fn nth_smoothed_fibonacci(n: u32) -> f32 {
    let f = (n / 3) * 2;
    let p = (1 << f) as f32; // ×4 for every multiple of 3
    let q = match n % 3 {
        0 => 1.0,
        1 => 1.5,
        2 => 2.5,
        _ => unreachable!(),
    };

    p * q
}

pub const fn nth(step: u32, x: u32) -> f32 {
    let n = match step {
        0 => return 0.0, // zero is zero
        _ => step - 1,   // n is base zero
    };

    nth_smoothed_fibonacci(n) * x as f32
}

#[test]
fn base_values() {
    assert_eq!(nth_smoothed_fibonacci(0), 1.0); //    → 2
    assert_eq!(nth_smoothed_fibonacci(1), 1.5); // ×2 → 3
    assert_eq!(nth_smoothed_fibonacci(2), 2.5); //    → 5
    assert_eq!(nth_smoothed_fibonacci(3), 4.0); //    → 8
}
