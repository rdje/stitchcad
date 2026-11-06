#![doc = include_str!("../../../docs/book/src/spec/viewport.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod corpus;
mod hit;
mod view;

pub use corpus::{
    corpus, Corpus, Piece, Probe, ANNOTATIONS_PER_PIECE, PICK_PROBES, PIECES, SNAP_PROBES,
    VERTICES_PER_PIECE,
};
pub use hit::{nearest_edge, pick, snap, EdgeRef, Hit};
pub use view::{Point, View, ViewError, ZOOMS};
