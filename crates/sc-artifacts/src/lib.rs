#![doc = include_str!("../../../docs/book/src/spec/canonical-artifacts.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod canonical;
mod compare;
mod pattern;

pub use canonical::{from_canonical_text, to_canonical_text, CanonicalError};
pub use compare::{compare, Comparison, Difference};
pub use pattern::{DrawnPiece, Line, Mark, MarkKind, Pattern, Text};
