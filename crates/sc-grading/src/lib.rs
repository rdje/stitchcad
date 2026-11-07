#![doc = include_str!("../../../docs/book/src/spec/grade-rules.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod rul;
mod table;

pub use rul::{from_rul, to_rul, RulArtifact, RulError, RulFile, RulRule};
pub use table::{Attributes, Convention, GradeError, GradeTable, Provenance, Rule, ZeroState};
