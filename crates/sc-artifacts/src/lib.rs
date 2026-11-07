#![doc = include_str!("../../../docs/book/src/spec/canonical-artifacts.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod canonical;
mod closures;
mod compare;
mod dxf;
mod graded_dxf;
mod judge;

#[doc = include_str!("../../../docs/book/src/spec/mutation-tests.md")]
mod mutation_tests_examples {}
#[doc = include_str!("../../../docs/book/src/spec/dxf-output.md")]
mod dxf_examples {}
#[doc = include_str!("../../../docs/book/src/spec/notch-types.md")]
mod notch_examples {}
mod pattern;
mod pdf;
mod seams;
#[doc = include_str!("../../../docs/book/src/spec/pdf-output.md")]
mod pdf_examples {}

pub use canonical::{from_canonical_text, to_canonical_text, CanonicalError};
pub use closures::{along, closure_reports, ClosureReport};
pub use compare::{compare, Comparison, Difference};
pub use dxf::{write_dxf, DxfArtifact, DxfError, Flattening, LineSemantics, TARGET, WRITER};
pub use graded_dxf::{
    write_all_contours, write_base_plus_rules, write_embedded_rules, GradedArtifact, GradingMode,
    NestPiece, NotchEncoding, Paired, RulePoint,
};
pub use judge::{
    judge_dxf, judge_inputs, judge_pattern, judge_pdf_square, Expectations, Finding,
    PieceExpectation, Span, SpanSide,
};
pub use pattern::{
    ButtonPair, Closure, ClosureKind, DrawnPiece, Fit, GradePoint, Line, Mark, MarkKind, Pattern,
    Pocket, Repair, Seam, SeamSide, Text,
};
pub use pdf::{
    write_pdf, PageLayout, PdfArtifact, PdfError, Sheet, PDF_TARGET, PDF_WRITER, SCALE_SQUARE,
};
pub use seams::{stretch, stretch_length, walk, WalkError, WalkReport, WALK_TOLERANCE};
