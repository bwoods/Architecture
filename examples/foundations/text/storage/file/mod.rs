use crate::text::Position;
use std::ops::RangeBounds;

mod large;
mod small;

trait Text {
    fn characters(
        &self,
        range: impl RangeBounds<Position>,
    ) -> impl Iterator<Item = (Position, char)>;
}
