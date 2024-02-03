use theme::Theme;

mod theme;

pub fn primary_content() -> [u8; 4] {
    theme::current().primary_content()
}

pub fn secondary_content() -> [u8; 4] {
    theme::current().secondary_content()
}

pub fn tertiary_content() -> [u8; 4] {
    theme::current().tertiary_content()
}

pub fn quaternary_content() -> [u8; 4] {
    theme::current().quaternary_content()
}

pub fn primary_shapes() -> [u8; 4] {
    theme::current().primary_shapes()
}

pub fn secondary_shapes() -> [u8; 4] {
    theme::current().secondary_shapes()
}

pub fn tertiary_shapes() -> [u8; 4] {
    theme::current().tertiary_shapes()
}

pub fn quaternary_shapes() -> [u8; 4] {
    theme::current().quaternary_shapes()
}

pub fn primary_background() -> [u8; 4] {
    theme::current().primary_background()
}

pub fn secondary_background() -> [u8; 4] {
    theme::current().secondary_background()
}

pub fn tertiary_background() -> [u8; 4] {
    theme::current().tertiary_background()
}

pub fn quaternary_background() -> [u8; 4] {
    theme::current().quaternary_background()
}
