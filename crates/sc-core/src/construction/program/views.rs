//! Allocation-free loans of actual complete normalized owners, with separate source positions.
use super::{DraftingProgramStep, NormalizedDraftingOperation, NormalizedDraftingProgram};
use crate::{
    construction::DraftingContextSource, design::DesignMetadataHost,
    recipe::FormulaNormalizedStatement,
};
use core::{fmt, iter::FusedIterator};

/// Borrowed actual scalar statement or native argument owner; no copied/extracted source arena.
#[derive(Clone, Copy)]
pub enum DraftingProgramStepKind<'n, 'a> {
    /// Actual statement in the complete original scalar recipe, including assertion operands.
    Scalar {
        /// Original one-based scalar-source ordinal, separate from the mixed position.
        ordinal: usize,
        /// Actual whole-source normalized statement, borrowing its real arena owner.
        statement: &'n FormulaNormalizedStatement<'a>,
    },
    /// Actual native argument arena owner, or original no-argument input family.
    Drafting(&'n NormalizedDraftingOperation<'a>),
}
impl fmt::Debug for DraftingProgramStepKind<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scalar { ordinal, .. } => f
                .debug_struct("Scalar")
                .field("ordinal", ordinal)
                .finish_non_exhaustive(),
            Self::Drafting(input) => input.fmt(f),
        }
    }
}

/// Actual normalized step loan with a private genuine one-based combined-program position.
/// The original scalar ordinal and actual statement/native owners remain separate fields.
/// This is normalized syntax, not a namespace/kind/dependency/value or executable-prefix proof.
/// ```compile_fail,E0616
/// fn renumber(step:&mut sc_core::construction::DraftingProgramStepView<'_, '_>) {step.position=1;}
/// ```
/// A step cannot outlive the normalized arena it borrows:
/// ```compile_fail
/// use sc_core::{construction::{DraftingContextSource,NormalizedDraftingProgram,DraftingProgramStepView},
///     design::DesignMetadataHost};
/// fn escape<'n,'p,'a,H:DesignMetadataHost,S:DraftingContextSource<H>>(
///     program:&'n NormalizedDraftingProgram<'p,'a,H,S>) -> Option<DraftingProgramStepView<'static,'a>> {
///     program.steps().next()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct DraftingProgramStepView<'n, 'a> {
    position: usize,
    kind: DraftingProgramStepKind<'n, 'a>,
}
impl<'n, 'a> DraftingProgramStepView<'n, 'a> {
    /// Genuine original mixed-program position, counted from one without scalar renumbering.
    #[must_use]
    pub const fn position(self) -> usize {
        self.position
    }
    /// Original scalar ordinal and actual arena loan, or actual native owner loan.
    #[must_use]
    pub const fn kind(self) -> DraftingProgramStepKind<'n, 'a> {
        self.kind
    }
}
impl<'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    NormalizedDraftingProgram<'_, 'a, H, S>
{
    /// Borrow all actual normalized steps in original mixed order without allocating or reparsing.
    /// Direct original-ordinal lookup keeps nth/skip/reverse traversal tied to the same actual
    /// statement; no mutable scalar cursor, extracted source, fabricated span or prefix is used.
    /// Native argument views continue to borrow their actual complete normalized owners.
    pub fn steps(
        &self,
    ) -> impl ExactSizeIterator<Item = DraftingProgramStepView<'_, 'a>>
           + DoubleEndedIterator
           + FusedIterator {
        self.admitted()
            .inputs()
            .steps()
            .iter()
            .enumerate()
            .map(move |(index, request)| {
                let kind = match (request, self.operations().get(index)) {
                    (DraftingProgramStep::Scalar { ordinal }, Some(None)) => {
                        let statement = ordinal
                            .checked_sub(1)
                            .and_then(|i| self.recipe().normalized().statements().get(i))
                            .unwrap_or_else(|| {
                                unreachable!(
                                    "private whole scalar coverage and normalization invariant"
                                )
                            });
                        DraftingProgramStepKind::Scalar {
                            ordinal: *ordinal,
                            statement,
                        }
                    }
                    (DraftingProgramStep::Drafting(_), Some(Some(operation))) => {
                        DraftingProgramStepKind::Drafting(operation)
                    }
                    _ => unreachable!("private complete mixed-slot normalization invariant"),
                };
                // Original Vec elements are nonzero-size; its one-based positions fit usize.
                DraftingProgramStepView {
                    position: index + 1,
                    kind,
                }
            })
    }
}
