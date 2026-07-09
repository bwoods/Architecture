pub mod alice;

#[test]
fn snapshot_testing() {
    use crate::table::traits::Table;
    use composable_views::svg::Output;
    use composable_views::{Bounds, Size, View};
    use insta::assert_snapshot;

    use alice::*;

    let alice = Alice::default();
    let size = Size::new(800.0, 356.0);
    let bounds = Bounds::from_size(size);

    let mut output = Output::new(size.width, size.height);
    alice.table().draw(bounds, &mut output);
    assert_snapshot!("start", output.into_inner());

    let mut output = Output::new(size.width, size.height);
    alice.table().step_forward();
    alice.table().draw(bounds, &mut output);
    assert_snapshot!("step_forward", output.into_inner());

    let mut output = Output::new(size.width, size.height);
    alice.table().step_backward();
    alice.table().draw(bounds, &mut output);
    assert_snapshot!("step_backward", output.into_inner());

    let mut output = Output::new(size.width, size.height);
    alice.table().jump_forward();
    alice.table().draw(bounds, &mut output);
    assert_snapshot!("jump_forward", output.into_inner());

    let mut output = Output::new(size.width, size.height);
    alice.table().jump_backward();
    alice.table().draw(bounds, &mut output);
    assert_snapshot!("jump_backward", output.into_inner());
}
