use crate::dependencies::{Dependency, DependencyDefault};

use super::gray;

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

impl Theme for Light {
    fn primary_content(&self) -> [u8; 4] {
        gray::pure(0)
    }

    fn secondary_content(&self) -> [u8; 4] {
        gray::foreground(3)
    }

    fn tertiary_content(&self) -> [u8; 4] {
        gray::foreground(6)
    }

    fn quaternary_content(&self) -> [u8; 4] {
        gray::foreground(9)
    }

    fn primary_shapes(&self) -> [u8; 4] {
        gray::foreground(10)
    }

    fn secondary_shapes(&self) -> [u8; 4] {
        gray::foreground(11)
    }

    fn tertiary_shapes(&self) -> [u8; 4] {
        gray::foreground(12)
    }

    fn quaternary_shapes(&self) -> [u8; 4] {
        gray::foreground(14) // progression break…
    }

    fn primary_background(&self) -> [u8; 4] {
        gray::background(14)
    }

    fn secondary_background(&self) -> [u8; 4] {
        gray::background(12)
    }

    fn tertiary_background(&self) -> [u8; 4] {
        gray::background(10)
    }

    fn quaternary_background(&self) -> [u8; 4] {
        gray::background(8)
    }
}
