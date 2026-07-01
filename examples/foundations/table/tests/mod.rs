#![allow(unused_imports)]
use crate::table::Table;
use composable_views::text::Font;
use composable_views::{Bounds, Size, View, svg::Output};
use insta::assert_snapshot;
use itertools::Itertools;
use noto::*;

#[path = "../../noto/mod.rs"]
mod noto;

#[path = "../../inter/mod.rs"]
mod inter;

#[test]
fn snapshot_testing() {
    use crate::text::*;

    let mut table = Table::with_offset(0);
    let book = include_str!("Alice's Adventures in Wonderland (1865).md");
    let lines = book.lines().collect_vec();

    let size = Size::new(800.0, 330.0);
    let bounds = Bounds::from_size(size);

    let black = [0, 0, 0, 0xff];
    let font = Noto.serif().weight(500.0).size(20.0);

    //
    let mut output = Output::new(size.width, size.height);
    step_forward(&mut table, &lines, black, &font, size).draw(bounds, &mut output);
    assert_snapshot!("step_forward", output.into_inner());

    //
    let mut output = Output::new(size.width, size.height);
    step_backward(&mut table, &lines, black, &font, size).draw(bounds, &mut output);
    assert_snapshot!("step_backward", output.into_inner());

    //
    let mut output = Output::new(size.width, size.height);
    table.step_backward(|range| 0..range.end);
    jump_forward(&mut table, &lines, black, &font, size).draw(bounds, &mut output);
    assert_snapshot!("jump_forward", output.into_inner());

    //
    let mut output = Output::new(size.width, size.height);
    jump_backward(&mut table, &lines, black, &font, size).draw(bounds, &mut output);
    assert_snapshot!("jump_backward", output.into_inner());
}

#[cfg(test)]
fn step_backward(
    table: &mut Table<usize>,
    lines: &[&str],
    color: [u8; 4],
    font: &Font,
    size: Size,
) -> impl View {
    table.step_backward(|range| 0..range.end);

    table.view(size, move |range| {
        lines[range].iter().map(move |str| font.text(color, str))
    })
}

#[cfg(test)]
fn step_forward(
    table: &mut Table<usize>,
    lines: &[&str],
    color: [u8; 4],
    font: &Font,
    size: Size,
) -> impl View {
    table.step_forward(|range| range.into_iter());

    table.view(size, move |range| {
        lines[range].iter().map(move |str| font.text(color, str))
    })
}

#[cfg(test)]
fn jump_forward(
    table: &mut Table<usize>,
    lines: &[&str],
    color: [u8; 4],
    font: &Font,
    size: Size,
) -> impl View {
    table.jump_forward(size, |range| {
        range
            .clone()
            .zip(lines[range].iter().map(|str| font.text(color, str)))
    });

    table.view(size, move |range| {
        lines[range].iter().map(move |str| font.text(color, str))
    })
}

#[cfg(test)]
fn jump_backward(
    table: &mut Table<usize>,
    lines: &[&str],
    color: [u8; 4],
    font: &Font,
    size: Size,
) -> impl View {
    table.jump_backward(size, |range| {
        (0..range.end).zip(
            lines[range]
                .iter() //
                .map(|str| font.text(color, str)),
        )
    });

    table.view(size, move |range| {
        lines[range].iter().map(move |str| font.text(color, str))
    })
}
