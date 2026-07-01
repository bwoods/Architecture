use composable_views::View;
use composable_views::text::Font;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::ops::RangeBounds;
use std::sync::LazyLock;

#[path = "../../../noto/mod.rs"]
mod noto;

use noto::*;

static TEXT: &str = include_str!("Alice's Adventures in Wonderland (1865).md");

pub struct Alice {
    lines: Vec<&'static str>,
}

impl Default for Alice {
    fn default() -> Self {
        Self {
            lines: TEXT.lines().collect(),
        }
    }
}

impl Alice {
    /// Returns `View`s for each of the lines in
    /// **Alice's Adventures in Wonderland (1865).md**,
    /// within the `range` requested.
    pub fn views(
        &self,
        range: impl RangeBounds<usize>,
    ) -> impl DoubleEndedIterator<Item = impl View> + ExactSizeIterator {
        let min = match range.start_bound() {
            Included(included) => *included,
            Excluded(excluded) => *excluded + 1,
            Unbounded => 0,
        };

        let max = match range.end_bound() {
            Excluded(excluded) => *excluded,
            Included(included) => *included + 1,
            Unbounded => self.lines.len(),
        };

        self.lines[min..max].iter().map(|str| self.view(str))
    }

    fn view(&self, str: &str) -> impl View {
        let (font, str) = match str {
            str if str.starts_with("# ") => (&H1, str.strip_prefix("# ").unwrap()),
            str if str.starts_with("## ") => (&H2, str.strip_prefix("## ").unwrap()),
            str => (&P, str),
        };

        let color = [0, 0, 0, 0xff];
        font.text(color, str)
    }
}

static H1: LazyLock<Font<'static>> = LazyLock::new(|| Noto.serif().weight(600.0).size(36.0));
static H2: LazyLock<Font<'static>> =
    LazyLock::new(|| Noto.serif().weight(400.0).feature(b"smcp", 1).size(24.0));
static P: LazyLock<Font<'static>> = LazyLock::new(|| Noto.serif().weight(400.0).size(20.0));
