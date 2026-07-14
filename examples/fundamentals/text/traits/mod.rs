use crate::text::Position;
use itertools::Itertools;
use std::ops::RangeBounds;
use unicode_segmentation::UnicodeSegmentation;

pub use large::Large;
pub use small::Small;

mod large;
mod small;

pub trait Text {
    fn characters<R>(&self, range: R) -> impl Iterator<Item = (Position, char)>
    where
        R: RangeBounds<Position> + Clone;

    fn newlines<R>(&self, range: R) -> impl Iterator<Item = Position>
    where
        R: RangeBounds<Position> + Clone;

    fn graphemes<R>(&self, range: R) -> impl Iterator<Item = (Position, Position)>
    where
        R: RangeBounds<Position> + Clone,
    {
        GraphemeBoundary {
            iter: self
                .characters(range)
                .chain(std::iter::once((Position::last(), '␄'))), // needed by `tuple_windows()`
            string: Default::default(),
        }
        .tuple_windows()
    }

    #[cfg(test)]
    fn string(&self) -> String {
        self.characters(..).map(|(_, ch)| ch).collect()
    }
}

struct GraphemeBoundary<Iter>
where
    Iter: Iterator<Item = (Position, char)>,
{
    iter: Iter,
    string: String,
}

impl<Iter> Iterator for GraphemeBoundary<Iter>
where
    Iter: Iterator<Item = (Position, char)>,
{
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        for (pos, ch) in self.iter.by_ref() {
            if self.string.is_empty() {
                self.string.push(ch);
                return Some(pos); // return the first one
            }

            self.string.push(ch);

            if let Some((_, (next, _))) = self
                .string
                .grapheme_indices(true) // make a parameter?
                .tuple_windows()
                .next()
            {
                self.string.replace_range(..next, "");
                return Some(pos);
            }
        }

        None
    }
}
