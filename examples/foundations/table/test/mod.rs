use super::Table;
use composable_views::{Bounds, Size, View, svg::Output};
use insta::assert_snapshot;

pub mod alice;

#[test]
fn snapshot_testing() {
    use alice::*;

    let size = Size::new(800.0, 356.0);
    let bounds = Bounds::from_size(size);

    let alice = Alice::default();
    let mut table = Table::with_offset(0);

    let mut output = Output::new(size.width, size.height);
    table
        .view(size, |range| alice.views(range))
        .draw(bounds, &mut output);
    assert_snapshot!("start", output.into_inner());

    let mut output = Output::new(size.width, size.height);
    table.step_forward(|range| range);
    table
        .view(size, |range| alice.views(range))
        .draw(bounds, &mut output);
    assert_snapshot!("step_forward", output.into_inner());

    let mut output = Output::new(size.width, size.height);
    table.step_backward(|range| 0..range.end);
    table
        .view(size, |range| alice.views(range))
        .draw(bounds, &mut output);
    assert_snapshot!("step_backward", output.into_inner());

    let mut output = Output::new(size.width, size.height);
    table.jump_forward(size, |range| range.clone().zip(alice.views(range)));
    table
        .view(size, |range| alice.views(range))
        .draw(bounds, &mut output);
    assert_snapshot!("jump_forward", output.into_inner());

    let mut output = Output::new(size.width, size.height);
    table.jump_backward(size, |range| (0..range.end + 1).zip(alice.views(range)));
    table
        .view(size, |range| alice.views(range))
        .draw(bounds, &mut output);
    assert_snapshot!("jump_backward", output.into_inner());
}
