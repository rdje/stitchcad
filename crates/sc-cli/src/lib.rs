#![doc = include_str!("../../../docs/book/src/spec/command-line.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod design;
mod draft;

#[doc = include_str!("../../../docs/book/src/spec/closures.md")]
mod closures_examples {}

#[doc = include_str!("../../../docs/book/src/spec/walking-and-truing.md")]
mod walking_examples {}

#[doc = include_str!("../../../docs/book/src/spec/stable-references.md")]
mod references_examples {}

#[doc = include_str!("../../../docs/book/src/spec/reference-bodice.md")]
mod bodice_examples {}
mod outline;
mod session;

pub use sc_core::command::CommandAuthority;
pub use session::{replay, Caller, Failure, Running, STEPS};
