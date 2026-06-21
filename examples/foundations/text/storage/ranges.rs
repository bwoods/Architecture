use crate::text::Position;
use crate::text::storage::Storage;
use itertools::Itertools;
use std::{ops::Bound::Unbounded, ops::RangeBounds};
use unicode_segmentation::UnicodeSegmentation;

impl Storage {
    pub fn string(&self, range: impl RangeBounds<Position>) -> String {
        self.characters(range).map(|(_, ch)| ch).collect()
    }

    pub fn characters(
        &self,
        range: impl RangeBounds<Position>,
    ) -> impl Iterator<Item = (Position, char)> {
        // skip `Position::first()` as is it an `Exclusive` bound
        let skip = (range.start_bound() == Unbounded) as usize;

        // drop `Position::last()` as is it an `Exclusive` bound
        let drop = (range.end_bound() == Unbounded) as usize;

        self.characters
            .range(range)
            .dropping(skip)
            .dropping_back(drop)
            .map(|(pos, ch)| (pos.clone(), *ch))
    }

    pub fn graphemes<'a>(
        &'a self,
        range: impl RangeBounds<Position> + 'a,
    ) -> impl Iterator<Item = (&'a Position, &'a Position)> + 'a {
        // skip `Position::first()` as is it an `Exclusive` bound
        let skip = (range.start_bound() == Unbounded) as usize;

        // preserves `Position::last()` as is it needed by `tuple_windows()`
        GraphemeBoundary {
            iter: self.characters.range(range).dropping(skip),
            string: Default::default(),
        }
        .tuple_windows()
    }
}

struct GraphemeBoundary<'a, Iter>
where
    Iter: Iterator<Item = (&'a Position, &'a char)>,
{
    iter: Iter,
    string: String,
}

impl<'a, Iter> Iterator for GraphemeBoundary<'a, Iter>
where
    Iter: Iterator<Item = (&'a Position, &'a char)>,
{
    type Item = &'a Position;

    fn next(&mut self) -> Option<Self::Item> {
        for (pos, ch) in self.iter.by_ref() {
            if self.string.is_empty() {
                self.string.push(*ch);
                return Some(pos); // return the first one
            }

            self.string.push(*ch);

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

#[test]
fn grapheme_segmentation() {
    use super::Strategy;

    let string = "👧👧🏻👧🏼👧🏽👧🏾👧🏿";
    assert_eq!(string.len(), 44); // 44 utf-8 bytes

    let mut storage = Storage::with_algorithm(Strategy::boundary());
    storage.extend(string.chars());

    assert_eq!(storage.string(..), string);
    assert_eq!(storage.graphemes(..).count(), 6);

    let ours = storage
        .graphemes(..)
        .map(|(start, stop)| {
            let mut string = String::default();
            string.extend(storage.characters(start..stop).map(|(_, ch)| ch));
            string
        })
        .collect_vec();

    let theirs = string
        .grapheme_indices(true)
        .map(|(_, str)| str.to_owned())
        .collect_vec();

    assert_eq!(theirs, ours);
}
