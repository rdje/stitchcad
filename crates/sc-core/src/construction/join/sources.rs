//! Complete original A/B occurrence admission before namespace/output work;no currentness proof.
use super::BoundJoinCutPopulation;
use crate::ontology::{CutCopy, DirectedEdge, EdgeRef, Piece, SeamSide, SeamSideId};
use core::fmt;

/// Whole original A+B hole/edge-use allowances,including unrelated/repeated fields and folds.
/// ```compile_fail
/// let limits:sc_core::construction::JoinSourcePieceLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinSourcePieceLimits {
    holes: usize,
    occurrences: usize,
}
impl JoinSourcePieceLimits {
    /// Bound complete H before hole-length scans,then complete R before edge-field scans.
    #[must_use]
    pub const fn new(holes: usize, occurrences: usize) -> Self {
        Self { holes, occurrences }
    }
    /// Every original A+B hole group.
    #[must_use]
    pub const fn holes(self) -> usize {
        self.holes
    }
    /// Every boundary/hole/construction occurrence and separate fold field across A+B.
    #[must_use]
    pub const fn occurrences(self) -> usize {
        self.occurrences
    }
}
/// Original location within one named A/B Piece;indices are not persistent output identities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinSourceOccurrenceLocation {
    /// Original outer-boundary position.
    Boundary(usize),
    /// Original hole group and position within that group.
    Hole {
        /// Original hole group.
        hole: usize,
        /// Original position within that hole.
        index: usize,
    },
    /// Original construction-line position,including repeats.
    Construction(usize),
    /// Original separately declared undirected fold position.
    Fold(usize),
}
/// Actual original field;fold declarations do not manufacture a traversal direction.
#[derive(Clone, Copy, Debug)]
pub enum JoinSourceOccurrenceField<'a> {
    /// Actual boundary/hole/construction field and original traversal.
    Directed(&'a DirectedEdge),
    /// Actual separate fold field.
    Fold(&'a EdgeRef),
}
impl<'a> JoinSourceOccurrenceField<'a> {
    /// Actual original persistent edge field,without normalization or current mapping.
    #[must_use]
    pub const fn edge(self) -> &'a EdgeRef {
        match self {
            Self::Directed(field) => &field.edge,
            Self::Fold(edge) => edge,
        }
    }
}
/// First whole count/quota or direct current-operation edge-source cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinSourcePieceRefusal {
    /// Checked whole A+B hole count cannot be represented.
    HoleOverflow,
    /// Whole original H exceeds policy before child-length scans.
    Holes {
        /// Complete original count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Checked complete original R cannot be represented.
    OccurrenceOverflow,
    /// Whole original R exceeds policy before edge-field scans.
    Occurrences {
        /// Complete original count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// An original prerequisite edge is produced by this same join operation.
    SelfOperation {
        /// Original A/B role.
        side: SeamSideId,
        /// First original field in family order.
        location: JoinSourceOccurrenceLocation,
    },
}
/// Whole original population/context,limits and first cause survive every refusal.
pub struct JoinSourcePieceError<'a> {
    population: &'a BoundJoinCutPopulation<'a>,
    limits: JoinSourcePieceLimits,
    refusal: JoinSourcePieceRefusal,
}
fn original<'a>(population: &BoundJoinCutPopulation<'a>, side: SeamSideId) -> &'a Piece {
    population.metadata().splice().source().side(side).piece()
}
impl<'a> JoinSourcePieceError<'a> {
    /// Actual original complete result population/metadata/splice/source parents.
    #[must_use]
    pub const fn population(&self) -> &'a BoundJoinCutPopulation<'a> {
        self.population
    }
    /// Original explicit whole H/R policy.
    #[must_use]
    pub const fn limits(&self) -> JoinSourcePieceLimits {
        self.limits
    }
    /// First whole count/quota or original direct self-source cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinSourcePieceRefusal {
        self.refusal
    }
    /// Actual original refused edge for SelfOperation;quota/count causes return None.
    /// Constant work,no scan/allocation or copied replacement field.
    #[must_use]
    pub fn refused_source(&self) -> Option<&'a EdgeRef> {
        let JoinSourcePieceRefusal::SelfOperation { side, location } = self.refusal else {
            return None;
        };
        let d = original(self.population, side).definition();
        match location {
            JoinSourceOccurrenceLocation::Boundary(index) => d.boundary.get(index).map(|f| &f.edge),
            JoinSourceOccurrenceLocation::Hole { hole, index } => {
                d.holes.get(hole)?.get(index).map(|f| &f.edge)
            }
            JoinSourceOccurrenceLocation::Construction(index) => {
                d.construction_lines.get(index).map(|f| &f.edge)
            }
            JoinSourceOccurrenceLocation::Fold(index) => d.fold_edges.get(index),
        }
    }
}
impl fmt::Debug for JoinSourcePieceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinSourcePieceError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinSourcePieceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join source pieces: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinSourcePieceError<'_> {}
fn fields(
    piece: &Piece,
) -> impl DoubleEndedIterator<Item = (JoinSourceOccurrenceLocation, JoinSourceOccurrenceField<'_>)>
{
    use JoinSourceOccurrenceField as F;
    use JoinSourceOccurrenceLocation as L;
    let d = piece.definition();
    d.boundary
        .iter()
        .enumerate()
        .map(|(i, f)| (L::Boundary(i), F::Directed(f)))
        .chain(d.holes.iter().enumerate().flat_map(|(hole, group)| {
            group
                .iter()
                .enumerate()
                .map(move |(index, f)| (L::Hole { hole, index }, F::Directed(f)))
        }))
        .chain(
            d.construction_lines
                .iter()
                .enumerate()
                .map(|(i, f)| (L::Construction(i), F::Directed(f))),
        )
        .chain(
            d.fold_edges
                .iter()
                .enumerate()
                .map(|(i, f)| (L::Fold(i), F::Fold(f))),
        )
}
impl<'s> BoundJoinCutPopulation<'s> {
    /// Admit every original A/B boundary,hole,construction and separate fold occurrence.
    /// Checked full H and H quota precede all hole-child lengths. Checked full R and R quota
    /// precede any edge-field scan. Direct self-sources then refuse in A/B,family,field order.
    /// O(H+R+1) work,constant extra space,no allocation/string scan/provider/geometry call.
    /// Earlier inventory/input storage/lookup and result-population obligations remain separate.
    /// This grants no literal selected-bank membership,current Design/alias resolution or fit proof.
    /// # Errors
    /// Whole H arithmetic/quota,whole R arithmetic/quota,then first direct self-source field.
    pub fn admit_source_pieces<'a>(
        &'a self,
        limits: JoinSourcePieceLimits,
    ) -> Result<BoundJoinSourcePieces<'a>, JoinSourcePieceError<'a>>
    where
        's: 'a,
    {
        use JoinSourcePieceRefusal as R;
        let refuse = |refusal| JoinSourcePieceError {
            population: self,
            limits,
            refusal,
        };
        let pieces = [original(self, SeamSideId::A), original(self, SeamSideId::B)];
        let holes = pieces
            .iter()
            .map(|p| p.definition().holes.len())
            .try_fold(0usize, usize::checked_add)
            .ok_or_else(|| refuse(R::HoleOverflow))?;
        if holes > limits.holes {
            return Err(refuse(R::Holes {
                required: holes,
                allowed: limits.holes,
            }));
        }
        let occurrences = pieces
            .iter()
            .flat_map(|p| {
                let d = p.definition();
                d.holes.iter().map(Vec::len).chain([
                    d.boundary.len(),
                    d.construction_lines.len(),
                    d.fold_edges.len(),
                ])
            })
            .try_fold(0usize, usize::checked_add)
            .ok_or_else(|| refuse(R::OccurrenceOverflow))?;
        if occurrences > limits.occurrences {
            return Err(refuse(R::Occurrences {
                required: occurrences,
                allowed: limits.occurrences,
            }));
        }
        let operation = self.metadata().splice().source().edit().operation();
        for side in [SeamSideId::A, SeamSideId::B] {
            for (location, field) in fields(original(self, side)) {
                if field.edge().creator() == operation {
                    return Err(refuse(R::SelfOperation { side, location }));
                }
            }
        }
        Ok(BoundJoinSourcePieces {
            population: self,
            limits,
            holes,
            occurrences,
        })
    }
}
/// Authentic complete bounded original sources,not a currentness/topology/namespace certificate.
/// ```compile_fail
/// fn forge<'a>(mut source:sc_core::construction::BoundJoinSourcePieces<'a>,
///     other:&'a sc_core::construction::BoundJoinCutPopulation<'a>) {source.population=other;}
/// ```
/// ```compile_fail
/// fn escape(population:sc_core::construction::BoundJoinCutPopulation<'_>)
///     ->sc_core::construction::BoundJoinSourcePieces<'static> {
///     population.admit_source_pieces(sc_core::construction::JoinSourcePieceLimits::new(0,0)).unwrap()
/// }
/// ```
pub struct BoundJoinSourcePieces<'a> {
    population: &'a BoundJoinCutPopulation<'a>,
    limits: JoinSourcePieceLimits,
    holes: usize,
    occurrences: usize,
}
impl<'a> BoundJoinSourcePieces<'a> {
    /// Actual original complete population/metadata/splice/source context.
    #[must_use]
    pub const fn population(&self) -> &'a BoundJoinCutPopulation<'a> {
        self.population
    }
    /// Original whole H/R admission policy.
    #[must_use]
    pub const fn limits(&self) -> JoinSourcePieceLimits {
        self.limits
    }
    /// Checked complete original A+B hole-group count.
    #[must_use]
    pub const fn hole_count(&self) -> usize {
        self.holes
    }
    /// Checked complete original A+B contour/construction/fold occurrence count.
    #[must_use]
    pub const fn occurrence_count(&self) -> usize {
        self.occurrences
    }
    /// Actual original source Piece for the canonical A/B role.
    #[must_use]
    pub fn piece(&self, side: SeamSideId) -> &'a Piece {
        original(self.population, side)
    }
    /// Every original occurrence in A/B,then boundary/hole/construction/fold order,and reverse.
    /// Full traversal O(H+R+1),constant extra space;no deduplication or current-source mapping.
    pub fn occurrences(&self) -> impl DoubleEndedIterator<Item = JoinSourceOccurrence<'_>> {
        [SeamSideId::A, SeamSideId::B]
            .into_iter()
            .flat_map(move |side| {
                fields(self.piece(side)).map(move |(location, field)| JoinSourceOccurrence {
                    context: self,
                    side,
                    location,
                    field,
                })
            })
    }
}
impl fmt::Debug for BoundJoinSourcePieces<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundJoinSourcePieces")
            .field("holes", &self.holes)
            .field("occurrences", &self.occurrences)
            .finish_non_exhaustive()
    }
}
/// One authentic original source field with actual Piece/copy/held-range context.
/// ```compile_fail
/// fn forge<'a>(mut occurrence:sc_core::construction::JoinSourceOccurrence<'a>,
///     other:&'a sc_core::construction::BoundJoinSourcePieces<'a>) {occurrence.context=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct JoinSourceOccurrence<'a> {
    context: &'a BoundJoinSourcePieces<'a>,
    side: SeamSideId,
    location: JoinSourceOccurrenceLocation,
    field: JoinSourceOccurrenceField<'a>,
}
impl<'a> JoinSourceOccurrence<'a> {
    /// Actual complete bounded original context.
    #[must_use]
    pub const fn context(self) -> &'a BoundJoinSourcePieces<'a> {
        self.context
    }
    /// Original canonical A/B role.
    #[must_use]
    pub const fn side(self) -> SeamSideId {
        self.side
    }
    /// Original location within this Piece,not an identity or a global normalized index.
    #[must_use]
    pub const fn location(self) -> JoinSourceOccurrenceLocation {
        self.location
    }
    /// Actual original field and unchanged traversal where present.
    #[must_use]
    pub const fn field(self) -> JoinSourceOccurrenceField<'a> {
        self.field
    }
    /// Actual original source Piece,without a detached equal-content replacement.
    #[must_use]
    pub fn piece(self) -> &'a Piece {
        self.context.piece(self.side)
    }
    /// Actual selected physical copy for this original source role.
    #[must_use]
    pub fn copy(self) -> &'a CutCopy {
        self.context
            .population
            .metadata()
            .splice()
            .source()
            .side(self.side)
            .copy()
    }
    /// Actual original span-side field,retaining its held range and selected-copy identity.
    #[must_use]
    pub fn source_side(self) -> &'a SeamSide {
        self.context
            .population
            .metadata()
            .splice()
            .source()
            .side(self.side)
            .source_side()
    }
}
