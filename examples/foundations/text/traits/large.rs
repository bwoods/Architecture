use crate::text::Position;
use crate::text::storage::pos::rle::RLE;
use crate::text::traits::Text;
use crate::text::traits::small::Small;
use itertools::Itertools;
use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::ops::RangeBounds;

pub struct Large<'a> {
    small: Small,   // edits to the buffer
    large: &'a str, // read-only source
    regions: RLE,   // active regions (of large)
    newlines: BTreeSet<u64>,
}

impl<'a> From<&'a str> for Large<'a> {
    /// # Note
    /// For large `mmap`’d files it might be advantageous to use `advise` to
    /// [`DontNeed`] the `slice` before this scan
    ///
    /// [`DontNeed`]: https://docs.rs/mmap-io/latest/mmap_io/advise/enum.MmapAdvice.html
    fn from(str: &'a str) -> Self {
        let small = Small::default();

        let newlines = BTreeSet::from_iter(
            str.chars()
                .enumerate() //
                .filter_map(|(offset, ch)| match ch {
                    '\n' => Some(offset as u64),
                    _ => None,
                }),
        );

        let mut regions = RLE::default();
        regions.insert_range(0..str.len() as u64);

        Self {
            small,
            large: str,
            regions,
            newlines,
        }
    }
}

impl Text for Large<'_> {
    fn characters<R>(&self, range: R) -> impl Iterator<Item = (Position, char)>
    where
        R: RangeBounds<Position> + Clone,
    {
        self.small.characters(range.clone()).merge(
            self.regions
                .range(half_open(range)) //
                .filter_map(|offset| {
                    Position::from_offset(offset).ok().and_then(|pos| {
                        self.large
                            .get(offset as usize..) // skip invalid utf-8 boundaries
                            .and_then(|str| str.chars().next().map(|ch| (pos, ch)))
                    })
                }),
        )
    }

    fn newlines<R>(&self, range: R) -> impl Iterator<Item = Position>
    where
        R: RangeBounds<Position> + Clone,
    {
        self.small.newlines(range.clone()).merge(
            self.newlines
                .range(half_open(range))
                .map(|offset| Position::from_offset(*offset).unwrap()),
        )
    }
}

/// Converts a `RangeBounds<Position>` to a comparable `Range<u64>`
fn half_open<R>(range: R) -> std::ops::Range<u64>
where
    R: RangeBounds<Position>,
{
    let min = match range.start_bound() {
        Included(included) if included.len() == 1 => included.identifier().0,
        Included(included) => included.identifier().0 + 1,
        Excluded(excluded) => excluded.identifier().0 + 1,
        Unbounded => u64::MIN,
    };

    let max = match range.end_bound() {
        Excluded(excluded) if excluded.len() == 1 => excluded.identifier().0,
        Excluded(excluded) => excluded.identifier().0 + 1,
        Included(included) => included.identifier().0 + 1,
        Unbounded => u64::MAX,
    };

    min..max
}
