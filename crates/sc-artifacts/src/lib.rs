#![doc = include_str!("../../../docs/book/src/spec/canonical-artifacts.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod canonical;
mod compare;
mod dxf;
#[doc = include_str!("../../../docs/book/src/spec/dxf-output.md")]
mod dxf_examples {}
mod pattern;
mod pdf;
#[doc = include_str!("../../../docs/book/src/spec/pdf-output.md")]
mod pdf_examples {}

pub use canonical::{from_canonical_text, to_canonical_text, CanonicalError};
pub use compare::{compare, Comparison, Difference};
pub use dxf::{write_dxf, DxfArtifact, DxfError, Flattening, LineSemantics, TARGET, WRITER};
pub use pattern::{DrawnPiece, Line, Mark, MarkKind, Pattern, Text};
pub use pdf::{
    write_pdf, PageLayout, PdfArtifact, PdfError, Sheet, PDF_TARGET, PDF_WRITER, SCALE_SQUARE,
};
