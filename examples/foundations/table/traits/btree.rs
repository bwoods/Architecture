use super::Data;
use std::collections::BTreeMap;
use std::ops::RangeBounds;

impl<K, V> Data for BTreeMap<K, V>
where
    K: Ord + Clone,
{
    type Key = K;
    type Value = V;

    fn range(
        &self,
        range: impl RangeBounds<Self::Key>,
    ) -> impl DoubleEndedIterator<Item = (Self::Key, &Self::Value)> {
        self.range(range).map(|(k, v)| (k.clone(), v))
    }
}
