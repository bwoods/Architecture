use crate::table;
use crate::table::traits::Table;
use composable_views::text::Font;
use std::sync::LazyLock;

#[path = "../../../noto/mod.rs"]
mod noto;

use noto::*;

pub struct Alice<'a> {
    table: table::State<[&'a str]>,
    lines: Vec<&'a str>,
}

impl Alice<'_> {
    pub fn table(&self) -> impl Table {
        self.table.view(&self.lines, |str| {
            let (font, str) = match str {
                str if str.starts_with("# ") => (&H1, str.strip_prefix("# ").unwrap()),
                str if str.starts_with("## ") => (&H2, str.strip_prefix("## ").unwrap()),
                str => (&P, *str), // otherwise use default paragraph styling
            };

            let color = [0, 0, 0, 0xff];
            font.text(color, str)
        })
    }
}

static TEXT: &str = include_str!("Alice's Adventures in Wonderland (1865).md");

impl Default for Alice<'_> {
    fn default() -> Self {
        Self {
            table: Default::default(),
            lines: TEXT.lines().collect(),
        }
    }
}

static H1: LazyLock<Font<'static>> = //
    LazyLock::new(|| Noto.serif().weight(600.0).size(36.0));
static H2: LazyLock<Font<'static>> = //
    LazyLock::new(|| Noto.serif().weight(400.0).feature(b"smcp", 1).size(24.0));
static P: LazyLock<Font<'static>> = //
    LazyLock::new(|| Noto.serif().weight(400.0).size(20.0));
