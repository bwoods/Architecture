use super::Text;
use crate::text::{Position, storage::Storage};
use std::collections::BTreeSet;
use std::ops::RangeBounds;

pub struct Small {
    storage: Storage,
    newlines: BTreeSet<Position>,
}

impl Default for Small {
    fn default() -> Self {
        let storage = Storage::default();

        let mut newlines = BTreeSet::default();
        newlines.insert(Position::first());
        newlines.insert(Position::last());

        Self { storage, newlines }
    }
}

impl FromIterator<char> for Small {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let storage = Storage::from_iter(iter);

        let mut newlines = BTreeSet::default();
        newlines.insert(Position::first());

        // this second pass is fine; this is for Small text
        newlines.extend(
            storage
                .characters(..) //
                .filter_map(|(pos, ch)| match ch {
                    '\n' => Some(pos),
                    _ => None,
                }),
        );

        newlines.insert(Position::last());
        Self { storage, newlines }
    }
}

impl Text for Small {
    fn characters<R>(&self, range: R) -> impl Iterator<Item = (Position, char)>
    where
        R: RangeBounds<Position> + Clone,
    {
        self.storage.characters(range)
    }

    fn newlines<R>(&self, range: R) -> impl Iterator<Item = Position>
    where
        R: RangeBounds<Position> + Clone,
    {
        self.newlines.range(range).cloned()
    }
}
