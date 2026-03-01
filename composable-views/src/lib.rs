use composable::{From, TryInto, derive_more};
pub use lyon::math::{Box2D as Bounds, Point, Size, Transform};
use std::cell::Cell;
pub use ui_id::ui_id;

#[allow(missing_docs)]
#[derive(Clone, Debug, From, TryInto)]
pub enum Event {
    Gesture(Gesture, Cell<Point>),
    Resize { width: u32, height: u32 },
    Rescale { scale: f32 },
    Redraw,
}

#[derive(Copy, Clone, Debug)]
pub enum Gesture {
    Began { n: u8 },
    Moved { n: u8 },
    Ended { n: u8 },
}

#[test]
fn ui_ids_are_uuid_v4() {
    let ui_id = ui_id!();
    let uuid = uuid::Uuid::from_u128(ui_id.get());
    assert_eq!(uuid.get_version_num(), 4);
    assert_eq!(uuid.get_variant(), uuid::Variant::RFC4122);
}
