use super::Text;
use crate::text::{Position, storage::Storage};
use std::collections::BTreeSet;
use std::ops::RangeBounds;

pub struct Small {
    storage: Storage,
    newlines: BTreeSet<Position>,
}

impl FromIterator<char> for Small {
    /// # Note
    /// Implicitly removes `'\r'` characters (if any). If DOS linefeeds are
    /// needed they should be recreated on save.
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let storage = Storage::from_iter(iter.into_iter().filter(|ch| *ch != '\r'));

        let mut newlines = BTreeSet::default();
        newlines.insert(Position::first());

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
    fn characters(
        &self,
        range: impl RangeBounds<Position>,
    ) -> impl Iterator<Item = (Position, char)> {
        self.storage.characters(range)
    }
}
