#![doc = include_str!("../../../docs/book/src/spec/command-line.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod design;
mod session;

pub use session::{replay, Failure};
