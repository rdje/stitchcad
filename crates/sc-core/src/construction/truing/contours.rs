//! Deterministic complete literal-source contour declarations;not a canonical Piece or geometry.
use super::{
    AdmittedTruingDirections, BoundExactTruingPieceSource, TruingDirectionalOutput,
    TruingOutcomeIntent, TruingParameterDirection, TruingPieceOccurrence,
    TruingPieceOccurrenceField, TruingPieceOccurrenceLocation,
};
use crate::ontology::{DirectedEdge, Direction};
use core::fmt;

/// Complete additional contour resource allowances;no defaults or per-occurrence truncation.
/// ```compile_fail
/// let limits:sc_core::construction::TruingContourLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TruingContourLimits {
    outputs: usize,
    holes: usize,
    source_occurrences: usize,
    generated_occurrences: usize,
    workspace_entries: usize,
}
impl TruingContourLimits {
    /// Choose whole T,H,R,G and additional T+G logical-entry allowances.
    #[must_use]
    pub const fn new(
        outputs: usize,
        holes: usize,
        source_occurrences: usize,
        generated_occurrences: usize,
        workspace_entries: usize,
    ) -> Self {
        Self {
            outputs,
            holes,
            source_occurrences,
            generated_occurrences,
            workspace_entries,
        }
    }
    /// Every actual selected/outside output,including unused-by-fold declarations.
    #[must_use]
    pub const fn outputs(self) -> usize {
        self.outputs
    }
    /// All original hole headers.
    #[must_use]
    pub const fn holes(self) -> usize {
        self.holes
    }
    /// All original contour and separate fold occurrences.
    #[must_use]
    pub const fn source_occurrences(self) -> usize {
        self.source_occurrences
    }
    /// Every generated contour use;separate fold choice is a later declaration.
    #[must_use]
    pub const fn generated_occurrences(self) -> usize {
        self.generated_occurrences
    }
    /// Additional source-order index and generated-association entries,not bytes or prior storage.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// First whole resource or checked population cause;no partial contour plan escapes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruingContourRefusal {
    /// Complete generated population or additional workspace cannot be represented.
    CountOverflow,
    /// Actual complete output count exceeds policy.
    Outputs {
        /// Whole count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Actual complete hole-header count exceeds policy.
    Holes {
        /// Whole count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Actual complete original contour/fold count exceeds policy.
    SourceOccurrences {
        /// Whole count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Actual complete generated contour-use count exceeds policy.
    GeneratedOccurrences {
        /// Whole count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Additional complete T+G logical workspace exceeds policy.
    Workspace {
        /// Whole count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
}
/// Complete actual source/direction parents,explicit policy and first resource cause.
pub struct TruingContourError<'a> {
    source: &'a BoundExactTruingPieceSource<'a>,
    limits: TruingContourLimits,
    refusal: TruingContourRefusal,
}
impl<'a> TruingContourError<'a> {
    /// Actual complete original held Piece/direction/outcome/source context.
    #[must_use]
    pub const fn source(&self) -> &'a BoundExactTruingPieceSource<'a> {
        self.source
    }
    /// Original complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> TruingContourLimits {
        self.limits
    }
    /// First whole population or overflow cause.
    #[must_use]
    pub const fn refusal(&self) -> TruingContourRefusal {
        self.refusal
    }
}
impl fmt::Debug for TruingContourError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TruingContourError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for TruingContourError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "truing contour declaration: {:?}", self.refusal)
    }
}
impl std::error::Error for TruingContourError<'_> {}

impl<'s> BoundExactTruingPieceSource<'s> {
    /// Declare all original boundary,hole and construction contours,retaining their order.
    /// Refit replaces every literal selected-edge use with the complete ascending source partition;
    /// reversed original traversal reverses the sequence and composes each explicit direction.
    /// PreserveSource retains every original directed field. Fold choice remains separate.
    /// Whole T,H,R quotas and checked G/T+G bounds precede field scans,sorting or allocation.
    /// O(T log(T+2)+H+R+G) work;additional workspace at most T+G logical entries,not bytes.
    /// # Errors
    /// Whole T,H,R quotas;checked generated/workspace counts;whole G,then workspace quota.
    pub fn declare_contours<'a>(
        &'a self,
        limits: TruingContourLimits,
    ) -> Result<TruingPieceContours<'a>, TruingContourError<'a>>
    where
        's: 'a,
    {
        use TruingContourRefusal as R;
        let refuse = |refusal| TruingContourError {
            source: self,
            limits,
            refusal,
        };
        let directions = self.directions();
        let t = directions.manifest().assignments().len();
        let d = self.piece().definition();
        let h = d.holes.len();
        let r = self.occurrence_count();
        if t > limits.outputs {
            return Err(refuse(R::Outputs {
                required: t,
                allowed: limits.outputs,
            }));
        }
        if h > limits.holes {
            return Err(refuse(R::Holes {
                required: h,
                allowed: limits.holes,
            }));
        }
        if r > limits.source_occurrences {
            return Err(refuse(R::SourceOccurrences {
                required: r,
                allowed: limits.source_occurrences,
            }));
        }
        let refit = directions.outside().evidence().locks().outcome().intent()
            == TruingOutcomeIntent::Refit;
        let c = r
            .checked_sub(d.fold_edges.len())
            .ok_or_else(|| refuse(R::CountOverflow))?;
        let g = if refit {
            self.contour_matches()
                .checked_mul(t)
                .and_then(|v| {
                    c.checked_sub(self.contour_matches())
                        .and_then(|u| u.checked_add(v))
                })
                .ok_or_else(|| refuse(R::CountOverflow))?
        } else {
            c
        };
        let workspace = t.checked_add(g).ok_or_else(|| refuse(R::CountOverflow))?;
        if g > limits.generated_occurrences {
            return Err(refuse(R::GeneratedOccurrences {
                required: g,
                allowed: limits.generated_occurrences,
            }));
        }
        if workspace > limits.workspace_entries {
            return Err(refuse(R::Workspace {
                required: workspace,
                allowed: limits.workspace_entries,
            }));
        }
        let mut ordered: Vec<_> = directions.assignments().collect();
        ordered.sort_unstable_by_key(|output| {
            (output.source().range().from(), output.source().range().to())
        });
        let mut associations = Vec::with_capacity(g);
        let mut previous = None;
        let mut group_start = 0;
        for original in self.occurrences() {
            let (Some(group), TruingPieceOccurrenceField::Directed(field)) = (
                ContourGroup::from_location(original.location()),
                original.field(),
            ) else {
                continue;
            };
            if previous != Some(group) {
                group_start = associations.len();
                previous = Some(group);
            }
            if refit && original.is_target() {
                let mut outputs = ordered.iter();
                while let Some(output) = if field.direction == Direction::Original {
                    outputs.next()
                } else {
                    outputs.next_back()
                } {
                    let direction = match (field.direction, output.parameter_direction()) {
                        (Direction::Original, TruingParameterDirection::SameAsSource)
                        | (Direction::Reversed, TruingParameterDirection::ReverseSource) => {
                            Direction::Original
                        }
                        _ => Direction::Reversed,
                    };
                    let location = group.location(associations.len() - group_start);
                    associations.push(ContourAssociation {
                        original,
                        output: Some(*output),
                        location,
                        directed: DirectedEdge {
                            edge: output.source().logical_output(),
                            direction,
                        },
                    });
                }
            } else {
                let location = group.location(associations.len() - group_start);
                associations.push(ContourAssociation {
                    original,
                    output: None,
                    location,
                    directed: *field,
                });
            }
        }
        Ok(TruingPieceContours {
            source: self,
            limits,
            workspace,
            associations,
        })
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum ContourGroup {
    Boundary,
    Hole(usize),
    Construction,
}
impl ContourGroup {
    const fn from_location(location: TruingPieceOccurrenceLocation) -> Option<Self> {
        use TruingPieceOccurrenceLocation as L;
        match location {
            L::Boundary(_) => Some(Self::Boundary),
            L::Hole { hole, .. } => Some(Self::Hole(hole)),
            L::Construction(_) => Some(Self::Construction),
            L::Fold(_) => None,
        }
    }
    const fn location(self, index: usize) -> TruingPieceOccurrenceLocation {
        use TruingPieceOccurrenceLocation as L;
        match self {
            Self::Boundary => L::Boundary(index),
            Self::Hole(hole) => L::Hole { hole, index },
            Self::Construction => L::Construction(index),
        }
    }
}
#[derive(Debug)]
struct ContourAssociation<'a> {
    original: TruingPieceOccurrence<'a>,
    output: Option<TruingDirectionalOutput<'a>>,
    location: TruingPieceOccurrenceLocation,
    directed: DirectedEdge,
}
/// Authentic complete contour declaration,not a canonical Piece,identity issuance or geometric proof.
/// All non-contour Piece metadata and original fold declarations remain available through source().
/// ```compile_fail
/// fn forge<'a>(mut plan:sc_core::construction::TruingPieceContours<'a>,
///     other:&'a sc_core::construction::BoundExactTruingPieceSource<'a>) {plan.source=other;}
/// ```
/// ```compile_fail
/// fn escape(source:sc_core::construction::BoundExactTruingPieceSource<'_>)
///     ->sc_core::construction::TruingPieceContours<'static> {
///     source.declare_contours(sc_core::construction::TruingContourLimits::new(0,0,0,0,0)).unwrap()
/// }
/// ```
pub struct TruingPieceContours<'a> {
    source: &'a BoundExactTruingPieceSource<'a>,
    limits: TruingContourLimits,
    workspace: usize,
    associations: Vec<ContourAssociation<'a>>,
}
impl fmt::Debug for TruingPieceContours<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TruingPieceContours")
            .field("limits", &self.limits)
            .field("workspace", &self.workspace)
            .field("occurrences", &self.associations.len())
            .finish_non_exhaustive()
    }
}
impl<'a> TruingPieceContours<'a> {
    /// Actual original complete held Piece/direction context.
    #[must_use]
    pub const fn source(&self) -> &'a BoundExactTruingPieceSource<'a> {
        self.source
    }
    /// Actual complete original output-direction/outside/source context.
    #[must_use]
    pub const fn directions(&self) -> &'a AdmittedTruingDirections<'a> {
        self.source.directions()
    }
    /// Original explicit T/H/R/G/workspace policy.
    #[must_use]
    pub const fn limits(&self) -> TruingContourLimits {
        self.limits
    }
    /// Checked additional source-order/association entry bound,not bytes or previous storage.
    #[must_use]
    pub const fn workspace_bound(&self) -> usize {
        self.workspace
    }
    /// Complete generated contour population;separate fold fields are not guessed here.
    #[must_use]
    pub fn occurrence_count(&self) -> usize {
        self.associations.len()
    }
    /// Every original hole remains in its original position.
    #[must_use]
    pub fn hole_count(&self) -> usize {
        self.source.piece().definition().holes.len()
    }
    /// Every generated boundary,hole and construction use,in declared order and reverse.
    pub fn occurrences(
        &self,
    ) -> impl ExactSizeIterator<Item = TruingPlacedContourEdge<'_>> + DoubleEndedIterator {
        self.associations
            .iter()
            .enumerate()
            .map(|(index, association)| TruingPlacedContourEdge {
                context: self,
                index,
                association,
            })
    }
}
/// Authentic generated location,directed logical edge and actual original/source attribution.
/// ```compile_fail
/// fn forge<'a>(mut edge:sc_core::construction::TruingPlacedContourEdge<'a>,
///     other:&'a sc_core::construction::TruingPieceContours<'a>) {edge.context=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct TruingPlacedContourEdge<'a> {
    context: &'a TruingPieceContours<'a>,
    index: usize,
    association: &'a ContourAssociation<'a>,
}
impl<'a> TruingPlacedContourEdge<'a> {
    /// Actual whole complete contour plan.
    #[must_use]
    pub const fn context(self) -> &'a TruingPieceContours<'a> {
        self.context
    }
    /// Global generated occurrence ordinal,not a persistent identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }
    /// Generated boundary/hole/construction position;never a fold declaration.
    #[must_use]
    pub const fn location(self) -> TruingPieceOccurrenceLocation {
        self.association.location
    }
    /// Actual original held field and whole bounded parent.
    #[must_use]
    pub const fn original(self) -> TruingPieceOccurrence<'a> {
        self.association.original
    }
    /// Actual explicit output direction/source attribution,or None for an unchanged original use.
    #[must_use]
    pub const fn output(self) -> Option<TruingDirectionalOutput<'a>> {
        self.association.output
    }
    /// Generated complete directed logical edge declaration;unissued outputs remain unissued.
    #[must_use]
    pub const fn directed(self) -> &'a DirectedEdge {
        &self.association.directed
    }
}
