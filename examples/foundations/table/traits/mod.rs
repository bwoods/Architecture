use composable_views::{Size, View};
use std::ops::RangeBounds;

pub mod btree;
pub mod slice;

pub trait Data {
    type Key;
    type Value;

    fn range(
        &self,
        range: impl RangeBounds<Self::Key>,
    ) -> impl DoubleEndedIterator<Item = (Self::Key, &Self::Value)>;
}

pub trait Table: View {
    fn step_forward(&mut self);
    fn step_backward(&mut self);
    fn jump_forward(&mut self, size: Size);
    fn jump_backward(&mut self, size: Size);
}
