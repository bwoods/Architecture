#![allow(unused_imports)]
use crate::table::Table;
use composable_views::text::Font;
use composable_views::{Bounds, Size, View, svg::Output};
use insta::assert_snapshot;
use itertools::Itertools;
use noto::*;

#[path = "../../../foundations/noto/mod.rs"]
mod noto;

#[path = "../../../foundations/inter/mod.rs"]
mod inter;

#[test]
fn snapshot_testing() {
    use super::*;

    let mut table = Table::with_offset(0);
    let book = include_str!("Alice's Adventures in Wonderland (1865).md");
    let lines = book.lines().collect_vec();

    let size = Size::new(800.0, 318.0);
    let bounds = Bounds::from_size(size);

    let black = [0, 0, 0, 0xff];
    let font = Noto.serif().weight(500.0).size(20.0);

    //
    let mut output = Output::new(size.width, size.height);
    row_up(&mut table, &lines, size, black, &font).draw(bounds, &mut output);
    assert_snapshot!("after-row-up", output.into_inner());

    //
    let mut output = Output::new(size.width, size.height);
    row_down(&mut table, &lines, size, black, &font).draw(bounds, &mut output);
    assert_snapshot!("after-row-down", output.into_inner());

    //
    table.row_up(|range| 0..range.end);
    let mut output = Output::new(size.width, size.height);
    page_down(&mut table, &lines, size, black, &font).draw(bounds, &mut output);
    assert_snapshot!("after-page-down", output.into_inner());

    //
    let mut output = Output::new(size.width, size.height);
    page_up(&mut table, &lines, size, black, &font).draw(bounds, &mut output);
    assert_snapshot!("after-page-up", output.into_inner());
}

#[cfg(test)]
fn row_up(
    table: &mut Table<usize>,
    lines: &[&str],
    size: Size,
    black: [u8; 4],
    font: &Font,
) -> impl View {
    table.row_up(|range| 0..range.end);

    table.view(size, move |range| {
        lines[range].iter().map(move |str| font.text(black, str))
    })
}

#[cfg(test)]
fn row_down(
    table: &mut Table<usize>,
    lines: &[&str],
    size: Size,
    black: [u8; 4],
    font: &Font,
) -> impl View {
    table.row_down(|range| range.into_iter());

    table.view(size, move |range| {
        lines[range].iter().map(move |str| font.text(black, str))
    })
}

#[cfg(test)]
fn page_down(
    table: &mut Table<usize>,
    lines: &[&str],
    size: Size,
    color: [u8; 4],
    font: &Font,
) -> impl View {
    table.page_down(size, |range| {
        range
            .clone()
            .zip(lines[range].iter().map(|str| font.text(color, str)))
    });

    table.view(size, move |range| {
        lines[range].iter().map(move |str| font.text(color, str))
    })
}

#[cfg(test)]
fn page_up(
    table: &mut Table<usize>,
    lines: &[&str],
    size: Size,
    color: [u8; 4],
    font: &Font,
) -> impl View {
    table.page_up(size, |range| {
        (0..range.end).zip(lines[range].iter().map(|str| font.text(color, str)))
    });

    table.view(size, move |range| {
        lines[range].iter().map(move |str| font.text(color, str))
    })
}
