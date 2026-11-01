//! Bounded literal occurrences in the actual held Piece;no current-reference resolution.
use super::AdmittedTruingDirections;
use crate::ontology::{DirectedEdge, EdgeRef, Piece};
use core::fmt;

/// Complete output,hole and held occurrence allowances;no default or filtered population.
/// ```compile_fail
/// let limits:sc_core::construction::TruingPieceSourceLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TruingPieceSourceLimits {
    outputs: usize,
    holes: usize,
    occurrences: usize,
}
impl TruingPieceSourceLimits {
    /// Choose all T outputs,H hole headers and R boundary/hole/construction/fold occurrences.
    #[must_use]
    pub const fn new(outputs: usize, holes: usize, occurrences: usize) -> Self {
        Self {
            outputs,
            holes,
            occurrences,
        }
    }
    /// Complete actual selected/outside output population.
    #[must_use]
    pub const fn outputs(self) -> usize {
        self.outputs
    }
    /// Every original hole header,including holes with no selected edge.
    #[must_use]
    pub const fn holes(self) -> usize {
        self.holes
    }
    /// All original contour occurrences plus separate fold declarations.
    #[must_use]
    pub const fn occurrences(self) -> usize {
        self.occurrences
    }
}
/// First complete resource cause or absence of a literal source contour occurrence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruingPieceSourceRefusal {
    /// Whole actual output population exceeds policy.
    Outputs {
        /// Complete count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole original hole-header population exceeds policy.
    Holes {
        /// Complete count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete original occurrence sum cannot be represented.
    OccurrenceOverflow,
    /// Whole occurrence population exceeds policy,before any edge-field scan.
    Occurrences {
        /// Complete count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// No contour field literally names the selected held edge. This is not a currentness verdict.
    NoExactOccurrence,
}
/// Actual complete directions/source parents,explicit policy and first cause.
pub struct TruingPieceSourceError<'a> {
    directions: &'a AdmittedTruingDirections<'a>,
    limits: TruingPieceSourceLimits,
    refusal: TruingPieceSourceRefusal,
}
impl<'a> TruingPieceSourceError<'a> {
    /// Actual complete output/outside/evidence/lock/source context.
    #[must_use]
    pub const fn directions(&self) -> &'a AdmittedTruingDirections<'a> {
        self.directions
    }
    /// Actual selected held Piece,including every unaccepted occurrence.
    #[must_use]
    pub fn piece(&self) -> &'a Piece {
        self.directions
            .outside()
            .evidence()
            .locks()
            .copies()
            .selected_piece()
    }
    /// Original whole-population policy.
    #[must_use]
    pub const fn limits(&self) -> TruingPieceSourceLimits {
        self.limits
    }
    /// First quota,overflow or scoped literal-occurrence cause.
    #[must_use]
    pub const fn refusal(&self) -> TruingPieceSourceRefusal {
        self.refusal
    }
}
impl fmt::Debug for TruingPieceSourceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TruingPieceSourceError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for TruingPieceSourceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "truing exact Piece source: {:?}", self.refusal)
    }
}
impl std::error::Error for TruingPieceSourceError<'_> {}

impl<'s> AdmittedTruingDirections<'s> {
    /// Admit literal original Piece occurrences under complete T,H,R allowances.
    /// T then H quotas precede an O(H) length-only sum;checked R quota precedes all edge scans.
    /// O(H+R) work,constant additional space,no allocation or label/material string scan.
    /// This exact-held variant does not resolve aliases,check a current Design or prove geometry.
    /// Earlier parent resource obligations remain separate.
    /// # Errors
    /// Whole T,H,checked R and R quota,then absence of a literal source contour occurrence.
    pub fn admit_exact_piece_source<'a>(
        &'a self,
        limits: TruingPieceSourceLimits,
    ) -> Result<BoundExactTruingPieceSource<'a>, TruingPieceSourceError<'a>>
    where
        's: 'a,
    {
        use TruingPieceSourceRefusal as R;
        let refuse = |refusal| TruingPieceSourceError {
            directions: self,
            limits,
            refusal,
        };
        let outputs = self.manifest().assignments().len();
        if outputs > limits.outputs {
            return Err(refuse(R::Outputs {
                required: outputs,
                allowed: limits.outputs,
            }));
        }
        let piece = self.outside().evidence().locks().copies().selected_piece();
        let d = piece.definition();
        if d.holes.len() > limits.holes {
            return Err(refuse(R::Holes {
                required: d.holes.len(),
                allowed: limits.holes,
            }));
        }
        let count = d
            .holes
            .iter()
            .map(Vec::len)
            .chain([
                d.boundary.len(),
                d.construction_lines.len(),
                d.fold_edges.len(),
            ])
            .try_fold(0usize, usize::checked_add)
            .ok_or_else(|| refuse(R::OccurrenceOverflow))?;
        if count > limits.occurrences {
            return Err(refuse(R::Occurrences {
                required: count,
                allowed: limits.occurrences,
            }));
        }
        let target = self
            .outside()
            .evidence()
            .locks()
            .outcome()
            .source_domain()
            .domain()
            .edge();
        let contour_matches = d
            .boundary
            .iter()
            .chain(d.holes.iter().flatten())
            .chain(d.construction_lines.iter())
            .filter(|field| field.edge == target)
            .count();
        if contour_matches == 0 {
            return Err(refuse(R::NoExactOccurrence));
        }
        let fold_matches = d.fold_edges.iter().filter(|edge| **edge == target).count();
        Ok(BoundExactTruingPieceSource {
            directions: self,
            limits,
            count,
            contour_matches,
            fold_matches,
        })
    }
}

/// Original location;indices identify held fields,not persistent output identities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruingPieceOccurrenceLocation {
    /// Original outer-boundary position.
    Boundary(usize),
    /// Original hole and position within that hole.
    Hole {
        /// Original hole position.
        hole: usize,
        /// Original edge position.
        index: usize,
    },
    /// Original authored construction-line position.
    Construction(usize),
    /// Separate original fold declaration position.
    Fold(usize),
}
/// Actual held field;this sum alone does not certify whole-context admission.
#[derive(Clone, Copy, Debug)]
pub enum TruingPieceOccurrenceField<'a> {
    /// Original contour field,including its unchanged Piece traversal direction.
    Directed(&'a DirectedEdge),
    /// Original separate fold field,which carries no traversal direction.
    Fold(&'a EdgeRef),
}
impl<'a> TruingPieceOccurrenceField<'a> {
    /// Actual original edge field,without resolution,normalization or retargeting.
    #[must_use]
    pub const fn edge(self) -> &'a EdgeRef {
        match self {
            Self::Directed(field) => &field.edge,
            Self::Fold(edge) => edge,
        }
    }
}
/// Authentic bounded exact-held source;no geometric fit,currentness or replacement certificate.
/// ```compile_fail
/// fn forge<'a>(mut source:sc_core::construction::BoundExactTruingPieceSource<'a>,
///     other:&'a sc_core::construction::AdmittedTruingDirections<'a>) {source.directions=other;}
/// ```
/// ```compile_fail
/// fn escape(directions:sc_core::construction::AdmittedTruingDirections<'_>)
///     ->sc_core::construction::BoundExactTruingPieceSource<'static> {
///     directions.admit_exact_piece_source(sc_core::construction::TruingPieceSourceLimits::new(0,0,0)).unwrap()
/// }
pub struct BoundExactTruingPieceSource<'a> {
    directions: &'a AdmittedTruingDirections<'a>,
    limits: TruingPieceSourceLimits,
    count: usize,
    contour_matches: usize,
    fold_matches: usize,
}
impl fmt::Debug for BoundExactTruingPieceSource<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundExactTruingPieceSource")
            .field("limits", &self.limits)
            .field("occurrences", &self.count)
            .field("contour_matches", &self.contour_matches)
            .field("fold_matches", &self.fold_matches)
            .finish_non_exhaustive()
    }
}
impl<'a> BoundExactTruingPieceSource<'a> {
    /// Actual complete original output-direction context.
    #[must_use]
    pub const fn directions(&self) -> &'a AdmittedTruingDirections<'a> {
        self.directions
    }
    /// Actual selected held Piece,not a detached reconstruction.
    #[must_use]
    pub fn piece(&self) -> &'a Piece {
        self.directions
            .outside()
            .evidence()
            .locks()
            .copies()
            .selected_piece()
    }
    /// Original selected held edge;aliases are not resolved here.
    #[must_use]
    pub const fn target(&self) -> EdgeRef {
        self.directions
            .outside()
            .evidence()
            .locks()
            .outcome()
            .source_domain()
            .domain()
            .edge()
    }
    /// Original whole T,H,R policy.
    #[must_use]
    pub const fn limits(&self) -> TruingPieceSourceLimits {
        self.limits
    }
    /// Actual complete contour and separate fold-field population.
    #[must_use]
    pub const fn occurrence_count(&self) -> usize {
        self.count
    }
    /// Actual literal selected-edge contour occurrences,including repeated construction uses.
    #[must_use]
    pub const fn contour_matches(&self) -> usize {
        self.contour_matches
    }
    /// Actual literal selected-edge fold declarations,distinct from their boundary occurrences.
    #[must_use]
    pub const fn fold_matches(&self) -> usize {
        self.fold_matches
    }
    /// Every original boundary,hole,construction and fold field,in that order and reverse.
    /// O(H+R) complete traversal,constant additional space;unrelated fields remain present.
    pub fn occurrences(&self) -> impl DoubleEndedIterator<Item = TruingPieceOccurrence<'_>> {
        use TruingPieceOccurrenceField as F;
        use TruingPieceOccurrenceLocation as L;
        let d = self.piece().definition();
        d.boundary
            .iter()
            .enumerate()
            .map(|(i, f)| (L::Boundary(i), F::Directed(f)))
            .chain(d.holes.iter().enumerate().flat_map(|(hole, fields)| {
                fields
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
            .map(|(location, field)| TruingPieceOccurrence {
                context: self,
                location,
                field,
            })
    }
}
/// Authentic original field and its whole bounded exact-source context.
/// ```compile_fail
/// fn forge<'a>(mut field:sc_core::construction::TruingPieceOccurrence<'a>,
///     other:&'a sc_core::construction::BoundExactTruingPieceSource<'a>) {field.context=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct TruingPieceOccurrence<'a> {
    context: &'a BoundExactTruingPieceSource<'a>,
    location: TruingPieceOccurrenceLocation,
    field: TruingPieceOccurrenceField<'a>,
}
impl<'a> TruingPieceOccurrence<'a> {
    /// Actual whole bounded original Piece/output context.
    #[must_use]
    pub const fn context(self) -> &'a BoundExactTruingPieceSource<'a> {
        self.context
    }
    /// Original field location;no sorting or deduplication.
    #[must_use]
    pub const fn location(self) -> TruingPieceOccurrenceLocation {
        self.location
    }
    /// Actual original field,including contour direction when present.
    #[must_use]
    pub const fn field(self) -> TruingPieceOccurrenceField<'a> {
        self.field
    }
    /// Whether this literal edge field equals the actual selected held edge.
    #[must_use]
    pub fn is_target(self) -> bool {
        *self.field.edge() == self.context.target()
    }
}
