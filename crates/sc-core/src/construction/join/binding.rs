//! Actual graph/copy/Piece associations,without current Design or feasible joined geometry proof.
use super::JoinPiecesAtSeam;
use crate::ontology::{CutCopy, CutPlan, Piece, SeamSide, SeamSideId, SeamSpan, SewingGraph};
use core::fmt;

/// First original A/B association cause;no first-match choice resolves duplicate named Pieces.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinBindingRefusal {
    /// Original required span is absent from the provided graph.
    MissingSpan,
    /// Actual span side's physical copy is absent from the provided plan.
    MissingCopy {
        /// Original side.
        side: SeamSideId,
    },
    /// Actual selected copy belongs to a different pattern Piece than the request names.
    Piece {
        /// Original side.
        side: SeamSideId,
    },
    /// No provided Piece has this role's required identity.
    MissingPiece {
        /// Original side.
        side: SeamSideId,
    },
    /// More than one provided Piece has the required identity;no equal-content deduplication.
    DuplicatePiece {
        /// Original side.
        side: SeamSideId,
        /// Actual first index in the provided Piece slice.
        first: usize,
        /// Actual next index in that same slice.
        second: usize,
    },
}
/// Original request and actual supplied sources survive every association refusal unchanged.
pub struct JoinBindingError<'a> {
    edit: &'a JoinPiecesAtSeam,
    graph: &'a SewingGraph,
    plan: &'a CutPlan,
    pieces: &'a [Piece],
    refusal: JoinBindingRefusal,
}
impl<'a> JoinBindingError<'a> {
    /// Original join intent.
    #[must_use]
    pub const fn edit(&self) -> &'a JoinPiecesAtSeam {
        self.edit
    }
    /// Actual provided graph.
    #[must_use]
    pub const fn graph(&self) -> &'a SewingGraph {
        self.graph
    }
    /// Actual provided physical-copy plan.
    #[must_use]
    pub const fn plan(&self) -> &'a CutPlan {
        self.plan
    }
    /// Complete provided Piece slice,including missing/duplicate role evidence.
    #[must_use]
    pub const fn pieces(&self) -> &'a [Piece] {
        self.pieces
    }
    /// First actual association cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinBindingRefusal {
        self.refusal
    }
}
impl fmt::Debug for JoinBindingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinBindingError")
            .field("operation", &self.edit.operation())
            .field("span", &self.edit.span_id())
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinBindingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "join {} source binding: {:?}",
            self.edit.operation(),
            self.refusal
        )
    }
}
impl std::error::Error for JoinBindingError<'_> {}
#[derive(Clone, Copy, Debug)]
struct Sources<'a> {
    piece: &'a Piece,
    copy: &'a CutCopy,
}

/// Private complete two-sided association with actual sources;no joined output or semantic choice.
/// ```compile_fail
/// fn escape(edit:sc_core::construction::JoinPiecesAtSeam,graph:sc_core::ontology::SewingGraph,
///     plan:sc_core::ontology::CutPlan,pieces:Vec<sc_core::ontology::Piece>)
///     ->sc_core::construction::BoundJoinPiecesAtSeam<'static> {
///     edit.bind_source(&graph,&plan,&pieces).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoundJoinPiecesAtSeam<'a> {
    edit: &'a JoinPiecesAtSeam,
    graph: &'a SewingGraph,
    plan: &'a CutPlan,
    pieces: &'a [Piece],
    span: &'a SeamSpan,
    a: Sources<'a>,
    b: Sources<'a>,
}
impl<'a> BoundJoinPiecesAtSeam<'a> {
    /// Original deliberate join request.
    #[must_use]
    pub const fn edit(self) -> &'a JoinPiecesAtSeam {
        self.edit
    }
    /// Actual graph supplying the named span;current Design membership is separate.
    #[must_use]
    pub const fn graph(self) -> &'a SewingGraph {
        self.graph
    }
    /// Actual provided copy plan.
    #[must_use]
    pub const fn plan(self) -> &'a CutPlan {
        self.plan
    }
    /// Complete actual provided Piece slice;not a certified current Design inventory.
    #[must_use]
    pub const fn pieces(self) -> &'a [Piece] {
        self.pieces
    }
    /// Actual canonical span,retaining direction,allocated ease and stops unchanged.
    #[must_use]
    pub const fn span(self) -> &'a SeamSpan {
        self.span
    }
    /// Actual side association,borrowing this private proof and its original sources.
    #[must_use]
    pub fn side(&self, side: SeamSideId) -> BoundJoinSide<'_> {
        BoundJoinSide { bound: self, side }
    }
}
/// One authentic A/B role from a complete bound join,without copied metadata or selected defaults.
/// ```compile_fail
/// fn forge<'a>(bound:&'a sc_core::construction::BoundJoinPiecesAtSeam<'a>)
///     ->sc_core::construction::BoundJoinSide<'a> {
///     sc_core::construction::BoundJoinSide { bound,side:sc_core::ontology::SeamSideId::A }
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoundJoinSide<'a> {
    bound: &'a BoundJoinPiecesAtSeam<'a>,
    side: SeamSideId,
}
impl<'a> BoundJoinSide<'a> {
    /// Actual complete association that supplies this view.
    #[must_use]
    pub const fn bound(self) -> &'a BoundJoinPiecesAtSeam<'a> {
        self.bound
    }
    /// Original A/B role,not a geometric left/right inference.
    #[must_use]
    pub const fn side(self) -> SeamSideId {
        self.side
    }
    /// Actual original span-side field,including physical-copy id and source-frame bounds.
    #[must_use]
    pub const fn source_side(self) -> &'a SeamSide {
        match self.side {
            SeamSideId::A => &self.bound.span.definition().a,
            SeamSideId::B => &self.bound.span.definition().b,
        }
    }
    /// Actual selected copy,including original orientation.
    #[must_use]
    pub const fn copy(self) -> &'a CutCopy {
        match self.side {
            SeamSideId::A => self.bound.a.copy,
            SeamSideId::B => self.bound.b.copy,
        }
    }
    /// Actual provided pattern Piece,including all unresolved material/label/cut metadata.
    #[must_use]
    pub const fn piece(self) -> &'a Piece {
        match self.side {
            SeamSideId::A => self.bound.a.piece,
            SeamSideId::B => self.bound.b.piece,
        }
    }
    /// All provided-plan copies of this role's Piece,in authored order. This inventory grants
    /// neither consumption/retirement authority nor complete observer validation.
    pub fn source_copies(self) -> impl Iterator<Item = &'a CutCopy> + 'a {
        self.bound.plan.copies_for_piece(self.piece().id())
    }
}
impl JoinPiecesAtSeam {
    /// Bind the actual span,then both A/B copy/required-Piece/unique provided-Piece associations.
    /// Original metadata,ease and source intervals remain borrowed. This supplies no current-root,
    /// boundary membership,pose/congruence,ease compatibility,cut quantity,semantic reconciliation,
    /// output identity,retirement or command authority proof. Whole lookup resources are separate.
    /// # Errors
    /// Missing span,then A completely before B:missing copy,copy's Piece mismatch,missing or
    /// duplicate named Piece. Unrelated provided Piece duplicates remain whole-inventory work.
    pub fn bind_source<'a>(
        &'a self,
        graph: &'a SewingGraph,
        plan: &'a CutPlan,
        pieces: &'a [Piece],
    ) -> Result<BoundJoinPiecesAtSeam<'a>, JoinBindingError<'a>> {
        let refuse = |refusal| JoinBindingError {
            edit: self,
            graph,
            plan,
            pieces,
            refusal,
        };
        let span = graph
            .spans()
            .iter()
            .find(|span| span.id() == self.span_id())
            .ok_or_else(|| refuse(JoinBindingRefusal::MissingSpan))?;
        let bind = |side| {
            let source = match side {
                SeamSideId::A => &span.definition().a,
                SeamSideId::B => &span.definition().b,
            };
            let copy = plan
                .copy(source.copy)
                .ok_or_else(|| refuse(JoinBindingRefusal::MissingCopy { side }))?;
            let expected = self.piece_id(side);
            if copy.piece() != expected {
                return Err(refuse(JoinBindingRefusal::Piece { side }));
            }
            let mut matches = pieces
                .iter()
                .enumerate()
                .filter(|(_, piece)| piece.id() == expected);
            let (first, piece) = matches
                .next()
                .ok_or_else(|| refuse(JoinBindingRefusal::MissingPiece { side }))?;
            if let Some((second, _)) = matches.next() {
                return Err(refuse(JoinBindingRefusal::DuplicatePiece {
                    side,
                    first,
                    second,
                }));
            }
            Ok(Sources { piece, copy })
        };
        let a = bind(SeamSideId::A)?;
        let b = bind(SeamSideId::B)?;
        Ok(BoundJoinPiecesAtSeam {
            edit: self,
            graph,
            plan,
            pieces,
            span,
            a,
            b,
        })
    }
}
