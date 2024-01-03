use crate::views::{Output, Transform};

#[inline(never)]
#[allow(clippy::too_many_arguments)]
pub(crate) fn rectangle(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    rx: f32,
    ry: f32,
    k: f32,
    rgba: [u8; 4],
    transform: &Transform,
    output: &mut impl Output,
) {
    let p0 = (x, y + ry);
    let c0 = (x, y + ry * k);
    let c1 = (x + rx * k, y);
    let p1 = (x + rx, y);

    let p2 = (x + w - rx, y);
    let c2 = (x + w - rx * k, y);
    let c3 = (x + w, y + ry * k);
    let p3 = (x + w, y + ry);

    let p4 = (x + w, y + h - ry);
    let c4 = (x + w, y + h - ry * k);
    let c5 = (x + w - rx * k, y + h);
    let p5 = (x + w - rx, y + h);

    let p6 = (x + rx, y + h);
    let c6 = (x + rx * k, y + h);
    let c7 = (x, y + h - ry * k);
    let p7 = (x, y + h - ry);

    output.begin(p0.0, p0.1, rgba, transform);
    output.cubic_bezier_to(c0.0, c0.1, c1.0, c1.1, p1.0, p1.1);
    output.line_to(p2.0, p2.1);
    output.cubic_bezier_to(c2.0, c2.1, c3.0, c3.1, p3.0, p3.1);
    output.line_to(p4.0, p4.1);
    output.cubic_bezier_to(c4.0, c4.1, c5.0, c5.1, p5.0, p5.1);
    output.line_to(p6.0, p6.1);
    output.cubic_bezier_to(c6.0, c6.1, c7.0, c7.1, p7.0, p7.1);
    output.close();
}

#[test]
fn snapshot_testing() {
    use super::Path;
    use insta::assert_snapshot;

    let black = [0, 0, 0, 0xff];
    let transform = Default::default();

    let mut output = crate::views::svg::Output::new(256.0, 256.0);
    let circle = super::Circle { rgba: black };
    circle.draw(16.0, 16.0, 224.0, 224.0, &transform, &mut output);
    assert_snapshot!("circle", output.into_inner());

    let mut output = crate::views::svg::Output::new(256.0, 128.0);
    let ellipse = super::Ellipse { rgba: black };
    ellipse.draw(16.0, 16.0, 224.0, 112.0, &transform, &mut output);
    assert_snapshot!("ellipse", output.into_inner());

    let mut output = crate::views::svg::Output::new(256.0, 128.0);
    let rectangle = super::Rectangle { rgba: black };
    rectangle.draw(16.0, 16.0, 224.0, 112.0, &transform, &mut output);
    assert_snapshot!("rectangle", output.into_inner());

    let mut output = crate::views::svg::Output::new(256.0, 128.0);
    let rounded = rectangle.rounded(16.0, 16.0);
    rounded.draw(16.0, 16.0, 224.0, 112.0, &transform, &mut output);
    assert_snapshot!("rounded", output.into_inner());

    let mut output = crate::views::svg::Output::new(256.0, 128.0);
    let continuous = rounded.continuous();
    continuous.draw(16.0, 16.0, 224.0, 112.0, &transform, &mut output);
    assert_snapshot!("continuous", output.into_inner());
}
