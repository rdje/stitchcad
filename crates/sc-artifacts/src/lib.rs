#![doc = include_str!("../../../docs/book/src/spec/canonical-artifacts.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod canonical;
mod closures;
mod compare;
mod dxf;
mod judge;

#[doc = include_str!("../../../docs/book/src/spec/mutation-tests.md")]
mod mutation_tests_examples {}
#[doc = include_str!("../../../docs/book/src/spec/dxf-output.md")]
mod dxf_examples {}
mod pattern;
mod pdf;
mod seams;
#[doc = include_str!("../../../docs/book/src/spec/pdf-output.md")]
mod pdf_examples {}

pub use canonical::{from_canonical_text, to_canonical_text, CanonicalError};
pub use closures::{along, closure_reports, ClosureReport};
pub use compare::{compare, Comparison, Difference};
pub use dxf::{write_dxf, DxfArtifact, DxfError, Flattening, LineSemantics, TARGET, WRITER};
pub use judge::{
    judge_dxf, judge_inputs, judge_pattern, judge_pdf_square, Expectations, Finding,
    PieceExpectation, Span, SpanSide,
};
pub use pattern::{
    Closure, ClosureKind, DrawnPiece, GradePoint, Line, Mark, MarkKind, Pattern, Repair, Seam,
    SeamSide, Text,
};
pub use pdf::{
    write_pdf, PageLayout, PdfArtifact, PdfError, Sheet, PDF_TARGET, PDF_WRITER, SCALE_SQUARE,
};
pub use seams::{stretch, stretch_length, walk, WalkError, WalkReport};
