#![doc = include_str!("../../../docs/book/src/spec/geometry.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod curve;
#[doc = include_str!("../../../docs/book/src/spec/offsets.md")]
mod offset_examples {}
mod offset;
mod outline;
mod predicate;

pub use curve::{Curve, CurveKindError, DEFAULT_TOLERANCE};
pub use offset::{Allowance, Corner, Offset, OffsetError, MITER_LIMIT};
pub use outline::{Crossing, Outline, OutlineError, Winding};
pub use predicate::{
    orientation, segment_crossing, Fraction, Orientation, Point, PointError, SegmentCrossing,
    MAX_COORDINATE,
};
