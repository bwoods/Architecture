use itertools::Itertools;
use pos::{ETX, Position, STX, strategy::Strategy};
use std::collections::BTreeMap;
use std::ops::Range;

pub mod file;
pub mod pos;
mod ranges;

pub struct Storage {
    characters: BTreeMap<Position, char>,
    algorithm: Strategy,
    clock: u16,
}

impl Storage {
    pub fn with_algorithm(algorithm: Strategy) -> Self {
        Storage {
            algorithm,
            ..Default::default()
        }
    }

    pub fn replace_range<I>(&mut self, range: Range<Position>, iter: I)
    where
        I: Iterator<Item = char>,
    {
        self.remove_range(range.clone());
        self.insert_between(range, iter)
    }

    pub fn remove_range(&mut self, range: Range<Position>) {
        let keys = self
            .characters
            .range(range)
            .map(|(pos, _)| pos.clone())
            .collect_vec();

        for key in keys {
            self.characters.remove(&key);
        }
    }

    pub fn remove_after(&mut self, after: Position) {
        let range = self.range_after(after);

        if range.start == Position::last() {
            return;
        }

        self.remove_range(range);
    }

    pub fn remove_before(&mut self, before: Position) {
        let range = self.range_before(before);

        if range.start == Position::first() {
            return;
        }

        self.remove_range(range);
    }

    pub fn insert_between<I>(&mut self, range: Range<Position>, chars: I)
    where
        I: Iterator<Item = char>,
    {
        let clock = self.next_clock();

        let positions =
            self.algorithm
                .generate(clock, range.start.identifier(), range.end.identifier());

        for (pos, ch) in positions.zip(chars) {
            self.characters.insert(pos, ch);
        }
    }

    pub fn insert_after<I>(&mut self, after: Position, iter: I)
    where
        I: Iterator<Item = char>,
    {
        self.insert_between(self.range_after(after), iter);
    }

    pub fn insert_before<I>(&mut self, before: Position, iter: I)
    where
        I: Iterator<Item = char>,
    {
        self.insert_between(self.range_before(before), iter);
    }

    fn range_after(&self, after: Position) -> Range<Position> {
        let (left, right) = self
            .characters
            .range(after.clone()..)
            .map(|(pos, _)| pos.clone())
            .tuple_windows()
            .next()
            .unwrap(); // SAFETY: `characters` always contains STX and ETX

        let right = if left == after { right } else { left };

        after..right
    }

    fn range_before(&self, before: Position) -> Range<Position> {
        let (right, left) = self
            .characters
            .range(..=before.clone())
            .rev() // grab `pos` and its predecessor
            .map(|(pos, _)| pos.clone())
            .tuple_windows()
            .next()
            .unwrap(); // SAFETY: `characters` always contains STX and ETX

        let left = if right == before { left } else { right };

        left..before
    }

    /// The `clock` is incremented every insert to avoid the
    /// [ABA problem](https://en.wikipedia.org/wiki/ABA_problem)
    /// inherent in an insert-delete-insert at the same location.
    fn next_clock(&mut self) -> u16 {
        self.clock = u16::wrapping_add(self.clock, 1);
        self.clock
    }
}

impl Default for Storage {
    fn default() -> Self {
        let mut characters = BTreeMap::default();
        characters.insert(Position::first(), '\u{2402}'); // ␂ (start-of-text)
        characters.insert(Position::last(), '\u{2403}'); // ␃ (end-of-text)

        Storage {
            characters,
            algorithm: Default::default(),
            clock: Default::default(),
        }
    }
}

impl FromIterator<char> for Storage {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut new = Self::default();

        let positions = new.algorithm.generate(new.clock, STX, ETX);
        let chars = iter.into_iter();
        let iter = positions.zip(chars);

        new.characters.extend(iter);
        new
    }
}

impl Extend<char> for Storage {
    fn extend<I: IntoIterator<Item = char>>(&mut self, iter: I) {
        let after = self
            .characters
            .iter()
            .rev() // back-to-front
            .skip(1) // skip ETX
            .map(|(pos, _)| pos.clone())
            .next()
            .unwrap(); // SAFETY: `characters` always contains STX and ETX

        self.insert_after(after, iter.into_iter());
    }
}
