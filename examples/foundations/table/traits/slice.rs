use super::Data;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::ops::RangeBounds;

impl<T> Data for [T] {
    type Key = usize;
    type Value = T;

    fn range(
        &self,
        range: impl RangeBounds<Self::Key>,
    ) -> impl DoubleEndedIterator<Item = (Self::Key, &Self::Value)> {
        let min = match range.start_bound() {
            Included(included) => *included,
            Excluded(excluded) => *excluded + 1,
            Unbounded => 0,
        };

        let max = match range.end_bound() {
            Excluded(excluded) => *excluded,
            Included(included) => *included + 1,
            Unbounded => self.len(),
        };

        let range = min..max;
        range.clone().zip(self[range].iter())
    }
}
