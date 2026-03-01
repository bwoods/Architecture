use dpi::PhysicalSize;

pub const PREDEFINED: [PhysicalSize<u32>; 10] = [
    TABLET_S, TABLET_M, HD, HD_PLUS, HD_FULL, HD_QUAD, HD_3K, HD_4K, HD_5K, HD_8K,
];

fn fit<P>(predicate: P) -> PhysicalSize<u32>
where
    P: FnMut(&&PhysicalSize<u32>) -> bool,
{
    *PREDEFINED.iter().rfind(predicate).unwrap_or(DEFAULT)
}

/// Used for new windows so that 16:10 monitors do not try to create windows
/// that are wider than they are.
pub fn fit_width(width: u32) -> PhysicalSize<u32> {
    fit(|screen| screen.width <= width)
}

/// Used for user interaction so that if a user drags from `TABLET_M` they can
/// drag back to it; `fit_width` with always find `HD` instead.
pub fn fit_height(height: u32) -> PhysicalSize<u32> {
    fit(|screen| screen.height <= height)
}

pub const DEFAULT: &PhysicalSize<u32> = &HD;

pub const TABLET_S: PhysicalSize<u32> = PhysicalSize::new(1024, 768);
pub const TABLET_M: PhysicalSize<u32> = PhysicalSize::new(1280, 800);

pub const HD: PhysicalSize<u32> = PhysicalSize::new(1280, 720);
pub const HD_PLUS: PhysicalSize<u32> = PhysicalSize::new(1600, 900);
pub const HD_FULL: PhysicalSize<u32> = PhysicalSize::new(1920, 1080);
pub const HD_QUAD: PhysicalSize<u32> = PhysicalSize::new(2560, 1440);

pub const HD_3K: PhysicalSize<u32> = PhysicalSize::new(2880, 1620);
pub const HD_4K: PhysicalSize<u32> = PhysicalSize::new(3840, 2160);
pub const HD_5K: PhysicalSize<u32> = PhysicalSize::new(5120, 2880);
pub const HD_8K: PhysicalSize<u32> = PhysicalSize::new(7680, 4320);
