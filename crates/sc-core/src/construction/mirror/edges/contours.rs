//! Desired reflected contour declarations under explicit per-edge parameter directions;no geometry.
use super::MirrorPieceEdgeCoverage;
use crate::ontology::{DirectedEdge, Direction, EdgeRef, EntityId, LocalTag};
use core::fmt;
use std::collections::BTreeMap;

/// Explicit declared output parameter direction relative to the reflected original source frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MirrorPieceEdgeParameterization {
    tag: LocalTag,
    direction: Direction,
}
impl MirrorPieceEdgeParameterization {
    /// Original means output t follows reflected source t;Reversed means it follows 1-source t.
    /// This is a requirement for later correspondence proof,not a current frame certificate.
    #[must_use]
    pub const fn new(tag: LocalTag, direction: Direction) -> Self {
        Self { tag, direction }
    }
    /// Actual explicit persistent output role,not a proposal or contour position.
    #[must_use]
    pub const fn tag(&self) -> LocalTag {
        self.tag
    }
    /// Actual declared parameter-direction field,not a computed geometric orientation.
    #[must_use]
    pub const fn direction(&self) -> &Direction {
        &self.direction
    }
}
/// Whole original contour policy;every candidate needs an explicit parameter direction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MirrorPieceContourPolicy {
    operation: EntityId,
    parameterizations: Vec<MirrorPieceEdgeParameterization>,
}
impl MirrorPieceContourPolicy {
    /// Retain all authored fields without issuing references or guessing missing directions.
    #[must_use]
    pub const fn new(
        operation: EntityId,
        parameterizations: Vec<MirrorPieceEdgeParameterization>,
    ) -> Self {
        Self {
            operation,
            parameterizations,
        }
    }
    /// Original requested operation,checked against the actual mirror input.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Complete original ordered parameter declarations,including invalid raw duplicates.
    #[must_use]
    pub fn parameterizations(&self) -> &[MirrorPieceEdgeParameterization] {
        &self.parameterizations
    }
}
/// Explicit whole parameter/output,hole,source-occurrence and workspace allowances;no defaults.
/// ```compile_fail
/// let limits:sc_core::construction::MirrorPieceContourLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MirrorPieceContourLimits {
    parameters: usize,
    holes: usize,
    source_occurrences: usize,
    workspace_entries: usize,
}
impl MirrorPieceContourLimits {
    /// Bound both raw parameter entries and actual outputs by P,all H holes,all source R uses,
    /// and the checked 2K+H+R workspace upper bound for K actual raw parameter entries.
    #[must_use]
    pub const fn new(
        parameters: usize,
        holes: usize,
        source_occurrences: usize,
        workspace_entries: usize,
    ) -> Self {
        Self {
            parameters,
            holes,
            source_occurrences,
            workspace_entries,
        }
    }
    /// Whole raw parameter and actual output-candidate allowance.
    #[must_use]
    pub const fn parameters(self) -> usize {
        self.parameters
    }
    /// Whole original hole-group allowance.
    #[must_use]
    pub const fn holes(self) -> usize {
        self.holes
    }
    /// Whole original source-occurrence allowance,including folds even when explicitly dropped.
    #[must_use]
    pub const fn source_occurrences(self) -> usize {
        self.source_occurrences
    }
    /// Index,parameter associations,hole groups and output edge occurrences;entries,not bytes.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// First complete resource,operation or original parameter association cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPieceContourRefusal {
    /// Complete 2K+H+R workspace population cannot be counted.
    WorkspaceOverflow,
    /// Whole raw parameter population exceeds policy.
    Parameters {
        /// Actual whole entries.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole actual output-candidate population exceeds policy even if raw parameters are empty.
    Outputs {
        /// Actual whole candidates.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole original hole groups exceed policy.
    Holes {
        /// Actual whole groups.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole original edge occurrences exceed policy.
    SourceOccurrences {
        /// Actual whole uses.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole workspace upper bound exceeds policy before allocation.
    Workspace {
        /// Required upper bound.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Requested operation differs from the actual original mirror input.
    Operation,
    /// An explicit parameter tag repeats in original policy order.
    DuplicateTag {
        /// First original parameter entry.
        first: usize,
        /// Repeated original parameter entry.
        second: usize,
        /// Actual persistent tag.
        tag: LocalTag,
    },
    /// Original parameter entry names no actually covered output candidate.
    UnknownCandidate {
        /// Original parameter entry.
        index: usize,
        /// Original tag.
        tag: LocalTag,
    },
    /// First actual candidate in proposal order lacks an explicit parameter declaration.
    MissingParameterization {
        /// Actual candidate's persistent tag.
        tag: LocalTag,
    },
    /// An original source field lacks a complete admitted mapping;no fallback edge is fabricated.
    MissingSource {
        /// Actual original source identity.
        source: EdgeRef,
    },
}
/// Actual complete policy,coverage/source/Piece parents,limits and first cause survive refusal.
pub struct MirrorPieceContourError<'a> {
    coverage: &'a MirrorPieceEdgeCoverage<'a>,
    policy: &'a MirrorPieceContourPolicy,
    limits: MirrorPieceContourLimits,
    refusal: MirrorPieceContourRefusal,
}
impl<'a> MirrorPieceContourError<'a> {
    /// Actual whole source/role/candidate coverage owner.
    #[must_use]
    pub const fn coverage(&self) -> &'a MirrorPieceEdgeCoverage<'a> {
        self.coverage
    }
    /// Actual original whole policy,including every unaccepted parameter entry.
    #[must_use]
    pub const fn policy(&self) -> &'a MirrorPieceContourPolicy {
        self.policy
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> MirrorPieceContourLimits {
        self.limits
    }
    /// First whole-resource or original field cause.
    #[must_use]
    pub const fn refusal(&self) -> MirrorPieceContourRefusal {
        self.refusal
    }
}
impl fmt::Debug for MirrorPieceContourError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MirrorPieceContourError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for MirrorPieceContourError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mirror contour declaration: {:?}", self.refusal)
    }
}
impl std::error::Error for MirrorPieceContourError<'_> {}
impl<'s> MirrorPieceEdgeCoverage<'s> {
    /// Declare source loops in reverse traversal so a supported reflection can retain their winding.
    /// Every output edge's parameter direction must be explicit. Construction order is retained;
    /// its traversal reflects the original directed use. Fold presence derives only from actual cut
    /// policy. This creates logical edge declarations,not a Piece,registry edit or geometry proof.
    /// Whole K/N/H/R/2K+H+R checks precede any parameter/source scan or allocation. Work is
    /// O((K+N+R)log(K+N+2)+H);workspace is at most the admitted entry bound,not allocation bytes.
    /// # Errors
    /// Workspace count,raw parameter/output/hole/source/workspace quotas,operation;then original
    /// parameter duplicate/unknown tag and first candidate lacking a parameter. No partial result.
    pub fn declare_contours<'a>(
        &'a self,
        policy: &'a MirrorPieceContourPolicy,
        limits: MirrorPieceContourLimits,
    ) -> Result<MirrorPieceContours<'a>, MirrorPieceContourError<'a>>
    where
        's: 'a,
    {
        use MirrorPieceContourRefusal as R;
        let refuse = |refusal| MirrorPieceContourError {
            coverage: self,
            policy,
            limits,
            refusal,
        };
        let identity = self.edges().identity();
        let cut = identity.roles().metadata().cut();
        let definition = cut.source().piece().definition();
        let k = policy.parameterizations.len();
        let n = self.edges().edge_count();
        let h = definition.holes.len();
        let r = self.occurrence_count();
        let workspace_bound = k
            .checked_mul(2)
            .and_then(|v| v.checked_add(h))
            .and_then(|v| v.checked_add(r))
            .ok_or_else(|| refuse(R::WorkspaceOverflow))?;
        if k > limits.parameters {
            return Err(refuse(R::Parameters {
                required: k,
                allowed: limits.parameters,
            }));
        }
        if n > limits.parameters {
            return Err(refuse(R::Outputs {
                required: n,
                allowed: limits.parameters,
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
        if workspace_bound > limits.workspace_entries {
            return Err(refuse(R::Workspace {
                required: workspace_bound,
                allowed: limits.workspace_entries,
            }));
        }
        if policy.operation != cut.source().input().operation() {
            return Err(refuse(R::Operation));
        }
        let mut by_tag = BTreeMap::new();
        let mut parameters = Vec::with_capacity(k);
        for (index, parameter) in policy.parameterizations.iter().enumerate() {
            let tag = parameter.tag;
            if let Some(first) = by_tag.insert(tag, index) {
                return Err(refuse(R::DuplicateTag {
                    first,
                    second: index,
                    tag,
                }));
            }
            let output = self
                .edges()
                .edge(tag)
                .ok_or_else(|| refuse(R::UnknownCandidate { index, tag }))?;
            parameters.push((parameter, output));
        }
        for output in self.edges().edges() {
            let tag = output.candidate().tag();
            if !by_tag.contains_key(&tag) {
                return Err(refuse(R::MissingParameterization { tag }));
            }
        }
        let map_edge = |source: &EdgeRef| {
            let output = self
                .edge_for_source(source)
                .ok_or_else(|| refuse(R::MissingSource { source: *source }))?;
            let tag = output.candidate().tag();
            let parameter = by_tag
                .get(&tag)
                .and_then(|index| parameters.get(*index))
                .ok_or_else(|| refuse(R::MissingParameterization { tag }))?;
            Ok::<_, MirrorPieceContourError<'a>>((output.logical_output(), parameter.0.direction))
        };
        let map_directed = |source: &DirectedEdge, reverse_loop: bool| {
            let (edge, parameter) = map_edge(&source.edge)?;
            let direction = if parameter == Direction::Reversed {
                source.direction.reversed()
            } else {
                source.direction
            };
            Ok::<_, MirrorPieceContourError<'a>>(DirectedEdge {
                edge,
                direction: if reverse_loop {
                    direction.reversed()
                } else {
                    direction
                },
            })
        };
        let boundary = definition
            .boundary
            .iter()
            .rev()
            .map(|edge| map_directed(edge, true))
            .collect::<Result<Vec<_>, _>>()?;
        let holes = definition
            .holes
            .iter()
            .map(|hole| {
                hole.iter()
                    .rev()
                    .map(|edge| map_directed(edge, true))
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        let construction = definition
            .construction_lines
            .iter()
            .map(|edge| map_directed(edge, false))
            .collect::<Result<Vec<_>, _>>()?;
        let folds = cut
            .fold_source()
            .map(|source| map_edge(source).map(|(edge, _)| edge))
            .transpose()?
            .into_iter()
            .collect();
        Ok(MirrorPieceContours {
            coverage: self,
            policy,
            limits,
            workspace_bound,
            by_tag,
            parameters,
            boundary,
            holes,
            construction,
            folds,
        })
    }
}
/// Desired logical contour/field declarations under actual source and cut policy;no geometry proof.
/// ```compile_fail
/// fn reparent<'a>(mut result:sc_core::construction::MirrorPieceContours<'a>,
///     other:&'a sc_core::construction::MirrorPieceContourPolicy) { result.policy=other; }
/// ```
/// ```compile_fail
/// fn escape(coverage:sc_core::construction::MirrorPieceEdgeCoverage<'_>,policy:sc_core::construction::MirrorPieceContourPolicy)
///     ->sc_core::construction::MirrorPieceContours<'static> {
///     coverage.declare_contours(&policy,sc_core::construction::MirrorPieceContourLimits::new(0,0,0,0)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct MirrorPieceContours<'a> {
    coverage: &'a MirrorPieceEdgeCoverage<'a>,
    policy: &'a MirrorPieceContourPolicy,
    limits: MirrorPieceContourLimits,
    workspace_bound: usize,
    by_tag: BTreeMap<LocalTag, usize>,
    parameters: Vec<(
        &'a MirrorPieceEdgeParameterization,
        super::MirrorPieceOutputEdge<'a>,
    )>,
    boundary: Vec<DirectedEdge>,
    holes: Vec<Vec<DirectedEdge>>,
    construction: Vec<DirectedEdge>,
    folds: Vec<EdgeRef>,
}
impl<'a> MirrorPieceContours<'a> {
    /// Actual complete source/candidate/Piece coverage owner.
    #[must_use]
    pub const fn coverage(&self) -> &'a MirrorPieceEdgeCoverage<'a> {
        self.coverage
    }
    /// Actual complete original parameter policy,unchanged and in original order.
    #[must_use]
    pub const fn policy(&self) -> &'a MirrorPieceContourPolicy {
        self.policy
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> MirrorPieceContourLimits {
        self.limits
    }
    /// Admitted 2K+H+R entry upper bound;explicit NoFold can use fewer output entries.
    #[must_use]
    pub const fn workspace_bound(&self) -> usize {
        self.workspace_bound
    }
    /// Actual admitted original parameter field for a persistent output role.
    #[must_use]
    pub fn parameterization(&self, tag: LocalTag) -> Option<&'a MirrorPieceEdgeParameterization> {
        self.by_tag
            .get(&tag)
            .and_then(|index| self.parameters.get(*index))
            .map(|entry| entry.0)
    }
    /// Desired logical outer contour,in reverse original loop order with explicit parameter direction.
    #[must_use]
    pub fn boundary(&self) -> &[DirectedEdge] {
        &self.boundary
    }
    /// Desired logical holes,original group order and reversed traversal within each group.
    #[must_use]
    pub fn holes(&self) -> &[Vec<DirectedEdge>] {
        &self.holes
    }
    /// Desired logical construction uses,retaining original occurrence order.
    #[must_use]
    pub fn construction_lines(&self) -> &[DirectedEdge] {
        &self.construction
    }
    /// Desired logical fold edge,derived only from actual cut policy;no manufactured direction.
    #[must_use]
    pub fn fold_edges(&self) -> &[EdgeRef] {
        &self.folds
    }
}
