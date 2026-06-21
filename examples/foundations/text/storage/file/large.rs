use crate::text::Position;
use crate::text::storage::{Storage, file::Text, pos::rle::RLE};
use itertools::Itertools;
use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::ops::RangeBounds;

pub struct Large<'a> {
    storage: Storage, // edits to the buffer
    newlines: BTreeSet<Position>,

    buffer: &'a str, // read-only source
    regions: RLE,    // (soft-)deleted regions
}

impl Text for Large<'_> {
    fn characters(
        &self,
        range: impl RangeBounds<Position>,
    ) -> impl Iterator<Item = (Position, char)> {
        let min = match range.start_bound() {
            Included(included) => included.identifier().0,
            Excluded(excluded) => excluded.identifier().0 + 1,
            Unbounded => i64::MIN,
        };

        let max = match range.end_bound() {
            Included(included) => included.identifier().0 + 1,
            Excluded(excluded) => excluded.identifier().0,
            Unbounded => i64::MAX,
        };

        self.storage.characters(range).merge(
            self.regions
                .range(min..max)
                .map(|n| n as usize)
                .filter_map(|offset| {
                    let pos = Position::from_offset(offset + 1) // base₁ (STX is zero…)
                        .unwrap();

                    self.buffer
                        .get(offset..) // skip invalid utf-8 sequence boundaries…
                        .and_then(|slice| slice.chars().next().map(|ch| (pos, ch)))
                }),
        )
    }
}

// In addition to using RLE for read-only content offsets, attributes could
// be stored as `BtreeMap<Attribute, RLE>` and then range queries can be done
// with an RLE “logical and”.
//
// Render a span of text may require a (k-ary?) merge of the Storage and the
// Attributes spans, while maintaining a “stack” of relevant Attributes.
