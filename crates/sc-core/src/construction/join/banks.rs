//! Literal held cut-bank association under complete original source admission;no geometry/currentness.
use super::{
    BoundJoinSourcePieces, JoinSourceOccurrence, JoinSourceOccurrenceField,
    JoinSourceOccurrenceLocation,
};
use crate::ontology::{CutCopy, DirectedEdge, EdgeRange, Piece, SeamSide, SeamSideId};
use core::fmt;

/// Scoped exact-held cut-bank refusal,not a global absence or current-alias judgement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinCutBankRefusal {
    /// Original selected range edge has no literal boundary/hole occurrence on this actual Piece.
    /// Construction/fold fields cannot substitute for a cut contour;history projection is separate.
    NoExactCutBank {
        /// First missing canonical A/B role.
        side: SeamSideId,
    },
}
/// Whole actual admitted original sources and first missing side;no accepted bank prefix escapes.
pub struct JoinCutBankError<'a> {
    sources: &'a BoundJoinSourcePieces<'a>,
    refusal: JoinCutBankRefusal,
}
impl<'a> JoinCutBankError<'a> {
    /// Actual whole source/population/metadata/splice context and original resource policies.
    #[must_use]
    pub const fn sources(&self) -> &'a BoundJoinSourcePieces<'a> {
        self.sources
    }
    /// First scoped original A/B bank-membership cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinCutBankRefusal {
        self.refusal
    }
    /// Actual original refused range field,without a replacement frame or source lookup.
    #[must_use]
    pub fn source_range(&self) -> &'a EdgeRange {
        let JoinCutBankRefusal::NoExactCutBank { side } = self.refusal;
        &self
            .sources
            .population()
            .metadata()
            .splice()
            .source()
            .side(side)
            .source_side()
            .range
    }
}
impl fmt::Debug for JoinCutBankError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinCutBankError")
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinCutBankError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join cut bank: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinCutBankError<'_> {}
#[derive(Clone, Copy, Debug)]
struct Selected<'a> {
    occurrence: JoinSourceOccurrence<'a>,
    directed: &'a DirectedEdge,
}
impl<'s> BoundJoinSourcePieces<'s> {
    /// Bind A then B to literal original boundary/hole fields,under this immutable full H/R admission.
    /// Canonical Piece birth already excludes repeated cut-edge identities across its cut loops.
    /// Construction repeats and separate fold declarations are not candidate cut banks.
    /// Work O(H+R+1),constant two cached associations,no allocation,string/provider/geometry work.
    /// A partial source range stays the actual positive held field;outside geometry is not removed.
    /// This proves neither current ownership/alias projection,coincidence,numeric correspondence,
    /// joined topology,source retirement nor authoring authority. Prior resource obligations remain.
    /// # Errors
    /// First A/B source range edge without a literal cut-contour occurrence on its actual Piece.
    pub fn bind_cut_banks<'a>(&'a self) -> Result<BoundJoinCutBanks<'a>, JoinCutBankError<'a>>
    where
        's: 'a,
    {
        let find = |side| {
            let range = &self
                .population()
                .metadata()
                .splice()
                .source()
                .side(side)
                .source_side()
                .range;
            self.occurrences()
                .find_map(|occurrence| {
                    if occurrence.side() != side {
                        return None;
                    }
                    if !matches!(
                        occurrence.location(),
                        JoinSourceOccurrenceLocation::Boundary(_)
                            | JoinSourceOccurrenceLocation::Hole { .. }
                    ) {
                        return None;
                    }
                    match occurrence.field() {
                        JoinSourceOccurrenceField::Directed(directed)
                            if directed.edge == range.edge() =>
                        {
                            Some(Selected {
                                occurrence,
                                directed,
                            })
                        }
                        _ => None,
                    }
                })
                .ok_or(JoinCutBankError {
                    sources: self,
                    refusal: JoinCutBankRefusal::NoExactCutBank { side },
                })
        };
        let a = find(SeamSideId::A)?;
        let b = find(SeamSideId::B)?;
        Ok(BoundJoinCutBanks {
            sources: self,
            a,
            b,
        })
    }
}
/// Authentic complete A/B held bank association,not a geometric or current join certificate.
/// ```compile_fail
/// fn forge<'a>(mut banks:sc_core::construction::BoundJoinCutBanks<'a>,
///     other:&'a sc_core::construction::BoundJoinSourcePieces<'a>) {banks.sources=other;}
/// ```
/// ```compile_fail
/// fn escape(source:sc_core::construction::BoundJoinSourcePieces<'_>)->sc_core::construction::BoundJoinCutBanks<'static> {
///     source.bind_cut_banks().unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct BoundJoinCutBanks<'a> {
    sources: &'a BoundJoinSourcePieces<'a>,
    a: Selected<'a>,
    b: Selected<'a>,
}
impl<'a> BoundJoinCutBanks<'a> {
    /// Actual original complete source admission and its unchanged H/R policy/context.
    #[must_use]
    pub const fn sources(&self) -> &'a BoundJoinSourcePieces<'a> {
        self.sources
    }
    /// One authentic canonical A/B bank,without copy-order or traversal inference.
    #[must_use]
    pub fn bank(&self, side: SeamSideId) -> JoinCutBank<'_> {
        JoinCutBank {
            context: self,
            side,
        }
    }
    /// Both banks in canonical A/B order and reverse,independent of original CutPlan order.
    pub fn banks(&self) -> impl ExactSizeIterator<Item = JoinCutBank<'_>> + DoubleEndedIterator {
        [SeamSideId::A, SeamSideId::B]
            .into_iter()
            .map(move |side| self.bank(side))
    }
}
/// One authentic cut bank with its whole A/B context and actual original directed field/range.
/// ```compile_fail
/// fn forge<'a>(context:&'a sc_core::construction::BoundJoinCutBanks<'a>)->sc_core::construction::JoinCutBank<'a> {
///     sc_core::construction::JoinCutBank{context,side:sc_core::ontology::SeamSideId::A}
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct JoinCutBank<'a> {
    context: &'a BoundJoinCutBanks<'a>,
    side: SeamSideId,
}
impl<'a> JoinCutBank<'a> {
    const fn selected(self) -> Selected<'a> {
        match self.side {
            SeamSideId::A => self.context.a,
            SeamSideId::B => self.context.b,
        }
    }
    /// Actual complete two-bank/original-source context.
    #[must_use]
    pub const fn context(self) -> &'a BoundJoinCutBanks<'a> {
        self.context
    }
    /// Original canonical A/B role.
    #[must_use]
    pub const fn side(self) -> SeamSideId {
        self.side
    }
    /// Actual original authenticated source occurrence;its location is Boundary or Hole.
    #[must_use]
    pub const fn occurrence(self) -> JoinSourceOccurrence<'a> {
        self.selected().occurrence
    }
    /// Actual original cut-contour field and Piece traversal,not a supplied equal-looking field.
    #[must_use]
    pub const fn directed(self) -> &'a DirectedEdge {
        self.selected().directed
    }
    /// Actual original selected range field;partial bounds are neither expanded nor normalized.
    #[must_use]
    pub fn range(self) -> &'a EdgeRange {
        &self.source_side().range
    }
    /// Actual original selected span-side field,including physical-copy identity.
    #[must_use]
    pub fn source_side(self) -> &'a SeamSide {
        self.occurrence().source_side()
    }
    /// Actual original source Piece owning this literal cut bank.
    #[must_use]
    pub fn piece(self) -> &'a Piece {
        self.occurrence().piece()
    }
    /// Actual selected physical copy,without reflection applied to source parameters.
    #[must_use]
    pub fn copy(self) -> &'a CutCopy {
        self.occurrence().copy()
    }
}
