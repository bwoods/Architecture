use crate::dependencies::{Dependency, DependencyDefault};

pub struct Current(Box<dyn Theme>);

impl Default for Current {
    fn default() -> Self {
        Current(Box::new(Light))
    }
}

pub fn current() -> Dependency<Current> {
    Dependency::<Current>::new()
}

impl Theme for Current {
    fn primary_content(&self) -> [u8; 4] {
        self.0.primary_content()
    }

    fn secondary_content(&self) -> [u8; 4] {
        self.0.secondary_content()
    }

    fn tertiary_content(&self) -> [u8; 4] {
        self.0.tertiary_content()
    }

    fn quaternary_content(&self) -> [u8; 4] {
        self.0.quaternary_content()
    }

    fn primary_shapes(&self) -> [u8; 4] {
        self.0.primary_shapes()
    }

    fn secondary_shapes(&self) -> [u8; 4] {
        self.0.secondary_shapes()
    }

    fn tertiary_shapes(&self) -> [u8; 4] {
        self.0.tertiary_shapes()
    }

    fn quaternary_shapes(&self) -> [u8; 4] {
        self.0.quaternary_shapes()
    }

    fn primary_background(&self) -> [u8; 4] {
        self.0.primary_background()
    }

    fn secondary_background(&self) -> [u8; 4] {
        self.0.secondary_background()
    }

    fn tertiary_background(&self) -> [u8; 4] {
        self.0.tertiary_background()
    }

    fn quaternary_background(&self) -> [u8; 4] {
        self.0.quaternary_background()
    }
}

impl DependencyDefault for Current {}

pub trait Theme {
    fn primary_content(&self) -> [u8; 4];
    fn secondary_content(&self) -> [u8; 4];
    fn tertiary_content(&self) -> [u8; 4];
    fn quaternary_content(&self) -> [u8; 4];

    fn primary_shapes(&self) -> [u8; 4];
    fn secondary_shapes(&self) -> [u8; 4];
    fn tertiary_shapes(&self) -> [u8; 4];
    fn quaternary_shapes(&self) -> [u8; 4];

    fn primary_background(&self) -> [u8; 4];
    fn secondary_background(&self) -> [u8; 4];
    fn tertiary_background(&self) -> [u8; 4];
    fn quaternary_background(&self) -> [u8; 4];
}

struct Light;
struct Dark;

const fn gray(blue: i32, shift: i32) -> [u8; 4] {
    let n = blue + shift;
    [n as u8, n as u8, blue as u8, 0xFF]
}

impl Theme for Light {
    fn primary_content(&self) -> [u8; 4] {
        [0x0, 0x0, 0x0, 0xFF]
    }

    fn secondary_content(&self) -> [u8; 4] {
        gray(0x7F, -1)
    }

    fn tertiary_content(&self) -> [u8; 4] {
        gray(0xBF, -1)
    }

    fn quaternary_content(&self) -> [u8; 4] {
        gray(0xDF, -1)
    }

    fn primary_shapes(&self) -> [u8; 4] {
        gray(0xE6, -1)
    }

    fn secondary_shapes(&self) -> [u8; 4] {
        gray(0xEB, -1)
    }

    fn tertiary_shapes(&self) -> [u8; 4] {
        gray(0xEF, -1)
    }

    fn quaternary_shapes(&self) -> [u8; 4] {
        gray(0xF2, -1)
    }

    fn primary_background(&self) -> [u8; 4] {
        gray(0x99, -5)
    }

    fn secondary_background(&self) -> [u8; 4] {
        gray(0xBF, -5)
    }

    fn tertiary_background(&self) -> [u8; 4] {
        gray(0xD7, -5)
    }

    fn quaternary_background(&self) -> [u8; 4] {
        gray(0xE6, -5)
    }
}
