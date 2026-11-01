//! Explicit source-relative output parameter orientation;not contour traversal or geometric maps.
use super::{AdmittedTruingOutsideRoles, TruingOutsideOutput, TruingSourceDomainEdge};
use crate::ontology::{EdgeRange, EdgeRef, EntityId, LocalTag};
use core::fmt;
use std::collections::BTreeMap;

/// Requested output start/end orientation relative to the attributed ascending source range.
/// This is distinct from Piece traversal,physical-copy reflection and sewing direction.
/// ```compile_fail
/// let direction:sc_core::construction::TruingParameterDirection=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruingParameterDirection {
    /// Output start/end correspond to source from/to;actual correspondence still needs evidence.
    SameAsSource,
    /// Output start/end correspond to source to/from;no interior or affine map is certified.
    ReverseSource,
}
/// Original persistent tag and explicit requested source-relative parameter direction.
/// ```compile_fail
/// use sc_core::{construction::TruingDirectionAssignment,ontology::{LocalTag,Direction}};
/// TruingDirectionAssignment::new(LocalTag::FIRST,Direction::Original);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TruingDirectionAssignment {
    tag: LocalTag,
    direction: TruingParameterDirection,
}
impl TruingDirectionAssignment {
    /// Retain both explicit fields without deriving direction from a tag or list position.
    #[must_use]
    pub const fn new(tag: LocalTag, direction: TruingParameterDirection) -> Self {
        Self { tag, direction }
    }
    /// Actual original tag field.
    #[must_use]
    pub const fn tag(&self) -> &LocalTag {
        &self.tag
    }
    /// Actual original explicit orientation request.
    #[must_use]
    pub const fn direction(&self) -> &TruingParameterDirection {
        &self.direction
    }
}
/// Explicit whole output,assignment and index/association entry allowances;no defaults.
/// ```compile_fail
/// let limits:sc_core::construction::TruingDirectionLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TruingDirectionLimits {
    outputs: usize,
    assignments: usize,
    workspace_entries: usize,
}
impl TruingDirectionLimits {
    /// Choose complete T=N+R outputs,M raw assignments and checked2M additional workspace entries.
    #[must_use]
    pub const fn new(outputs: usize, assignments: usize, workspace_entries: usize) -> Self {
        Self {
            outputs,
            assignments,
            workspace_entries,
        }
    }
    /// Every actual selected/outside logical output,not physical copies or current geometry.
    #[must_use]
    pub const fn outputs(self) -> usize {
        self.outputs
    }
    /// Every original assignment,including duplicate and unknown fields.
    #[must_use]
    pub const fn assignments(self) -> usize {
        self.assignments
    }
    /// Additional index/association logical entries,not allocation bytes or earlier storage.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// First complete resource,owner,cardinality or original assignment failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruingDirectionRefusal {
    /// Complete selected/outside output population cannot be represented.
    OutputOverflow,
    /// Complete2M workspace population cannot be represented.
    WorkspaceOverflow,
    /// Whole actual output population exceeds policy.
    Outputs {
        /// Actual complete population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole raw assignment population exceeds policy.
    Assignments {
        /// Actual complete population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole new index/association population exceeds policy.
    Workspace {
        /// Actual complete entries.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Declared operation differs from the actual bound original truing operation.
    Operation,
    /// Exactly one assignment is required per actual selected/outside output.
    Count {
        /// Actual complete output population.
        expected: usize,
        /// Complete original assignment population.
        actual: usize,
    },
    /// A persistent output tag repeats in original assignment order.
    Duplicate {
        /// First original assignment.
        first: usize,
        /// Repeated original assignment.
        second: usize,
        /// Actual repeated tag.
        tag: LocalTag,
    },
    /// Tag names no actual selected/outside output,even if it exists in another metadata scope.
    UnknownOutput {
        /// Original assignment position.
        index: usize,
        /// Actual supplied tag.
        tag: LocalTag,
    },
}
/// Actual complete raw manifest,outside/source parents,explicit policy and first cause.
pub struct TruingDirectionError<'a> {
    manifest: &'a TruingDirectionManifest,
    outside: &'a AdmittedTruingOutsideRoles<'a>,
    limits: TruingDirectionLimits,
    refusal: TruingDirectionRefusal,
}
impl<'a> TruingDirectionError<'a> {
    /// Actual complete original manifest,including every unaccepted assignment.
    #[must_use]
    pub const fn manifest(&self) -> &'a TruingDirectionManifest {
        self.manifest
    }
    /// Actual whole outside/evidence/lock/outcome/selected/source context.
    #[must_use]
    pub const fn outside(&self) -> &'a AdmittedTruingOutsideRoles<'a> {
        self.outside
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> TruingDirectionLimits {
        self.limits
    }
    /// First complete resource,owner,count or original assignment cause.
    #[must_use]
    pub const fn refusal(&self) -> TruingDirectionRefusal {
        self.refusal
    }
}
impl fmt::Debug for TruingDirectionError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TruingDirectionError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for TruingDirectionError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "truing output parameter directions: {:?}", self.refusal)
    }
}
impl std::error::Error for TruingDirectionError<'_> {}

/// Original operation and ordered explicit assignments;no direction or complete coverage is inferred.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TruingDirectionManifest {
    operation: EntityId,
    assignments: Vec<TruingDirectionAssignment>,
}
impl TruingDirectionManifest {
    /// Retain original operation and assignment order,including invalid duplicate/unknown fields.
    #[must_use]
    pub const fn new(operation: EntityId, assignments: Vec<TruingDirectionAssignment>) -> Self {
        Self {
            operation,
            assignments,
        }
    }
    /// Original declared operation.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Every original assignment in declaration order.
    #[must_use]
    pub fn assignments(&self) -> &[TruingDirectionAssignment] {
        &self.assignments
    }
    /// Admit complete T/M/2M before operation,exact cardinality,index allocation or field scans.
    /// Unique actual-population membership plus M=T proves one assignment per output;count alone
    /// proves nothing. Selected and outside tags are already disjoint in the actual parent.
    /// Work O(M log(M+T+2)),with2M new logical index/association entries. No geometry,registry,
    /// source rescaling,interior map,copy/sewing direction or authoring authority is inferred.
    /// # Errors
    /// T overflow,2M overflow,T/M/workspace quotas,operation,exact count;then each original
    /// assignment's duplicate or unknown output tag. No accepted prefix escapes.
    pub fn admit<'a>(
        &'a self,
        outside: &'a AdmittedTruingOutsideRoles<'a>,
        limits: TruingDirectionLimits,
    ) -> Result<AdmittedTruingDirections<'a>, TruingDirectionError<'a>> {
        use TruingDirectionRefusal as R;
        let refuse = |refusal| TruingDirectionError {
            manifest: self,
            outside,
            limits,
            refusal,
        };
        let domain = outside.evidence().locks().outcome().source_domain();
        let outputs = domain
            .edge_count()
            .checked_add(outside.roles().count())
            .ok_or_else(|| refuse(R::OutputOverflow))?;
        let m = self.assignments.len();
        let workspace = m
            .checked_mul(2)
            .ok_or_else(|| refuse(R::WorkspaceOverflow))?;
        if outputs > limits.outputs {
            return Err(refuse(R::Outputs {
                required: outputs,
                allowed: limits.outputs,
            }));
        }
        if m > limits.assignments {
            return Err(refuse(R::Assignments {
                required: m,
                allowed: limits.assignments,
            }));
        }
        if workspace > limits.workspace_entries {
            return Err(refuse(R::Workspace {
                required: workspace,
                allowed: limits.workspace_entries,
            }));
        }
        if self.operation != outside.evidence().locks().source().edit().operation() {
            return Err(refuse(R::Operation));
        }
        if m != outputs {
            return Err(refuse(R::Count {
                expected: outputs,
                actual: m,
            }));
        }
        let mut by_tag = BTreeMap::new();
        let mut assignments = Vec::with_capacity(m);
        for (index, assignment) in self.assignments.iter().enumerate() {
            let tag = assignment.tag;
            if let Some(first) = by_tag.insert(tag, index) {
                return Err(refuse(R::Duplicate {
                    first,
                    second: index,
                    tag,
                }));
            }
            let source = domain
                .edge(tag)
                .map(TruingDirectionSource::Selected)
                .or_else(|| {
                    outside
                        .roles()
                        .find(|output| *output.role().tag() == tag)
                        .map(TruingDirectionSource::Outside)
                })
                .ok_or_else(|| refuse(R::UnknownOutput { index, tag }))?;
            assignments.push(DirectionAssociation {
                index,
                assignment,
                source,
            });
        }
        Ok(AdmittedTruingDirections {
            manifest: self,
            outside,
            limits,
            workspace,
            by_tag,
            assignments,
        })
    }
}
/// Authentic source member;this enum alone is not a complete direction-manifest certificate.
#[derive(Clone, Copy, Debug)]
pub enum TruingDirectionSource<'a> {
    /// Actual original selected-domain candidate.
    Selected(TruingSourceDomainEdge<'a>),
    /// Actual admitted fixed outside source portion.
    Outside(TruingOutsideOutput<'a>),
}
impl<'a> TruingDirectionSource<'a> {
    /// Actual attributed held source range,not a source/output numeric map.
    #[must_use]
    pub const fn range(self) -> &'a EdgeRange {
        match self {
            Self::Selected(source) => source.source(),
            Self::Outside(source) => source.source(),
        }
    }
    /// Actual original operation/tag declaration,still unissued.
    #[must_use]
    pub fn logical_output(self) -> EdgeRef {
        match self {
            Self::Selected(source) => source.candidate().logical_output(),
            Self::Outside(source) => source.logical_output(),
        }
    }
}
#[derive(Debug)]
struct DirectionAssociation<'a> {
    index: usize,
    assignment: &'a TruingDirectionAssignment,
    source: TruingDirectionSource<'a>,
}

/// Authentic complete selected/outside orientation requests,not achieved maps or contour placement.
/// ```compile_fail
/// fn forge<'a>(mut directions:sc_core::construction::AdmittedTruingDirections<'a>,
///     other:&'a sc_core::construction::AdmittedTruingOutsideRoles<'a>) {directions.outside=other;}
/// ```
/// ```compile_fail
/// fn escape<'a>(outside:&'a sc_core::construction::AdmittedTruingOutsideRoles<'a>)
///     ->sc_core::construction::AdmittedTruingDirections<'a> {
///     use sc_core::{construction::*,ontology::EntityId};let raw=TruingDirectionManifest::new(EntityId::from_bits(0),vec![]);
///     raw.admit(outside,TruingDirectionLimits::new(0,0,0)).unwrap()
/// }
/// ```
pub struct AdmittedTruingDirections<'a> {
    manifest: &'a TruingDirectionManifest,
    outside: &'a AdmittedTruingOutsideRoles<'a>,
    limits: TruingDirectionLimits,
    workspace: usize,
    by_tag: BTreeMap<LocalTag, usize>,
    assignments: Vec<DirectionAssociation<'a>>,
}
impl fmt::Debug for AdmittedTruingDirections<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AdmittedTruingDirections")
            .field("operation", &self.manifest.operation)
            .field("assignments", &self.assignments.len())
            .field("limits", &self.limits)
            .finish_non_exhaustive()
    }
}
impl<'a> AdmittedTruingDirections<'a> {
    /// Actual complete original manifest.
    #[must_use]
    pub const fn manifest(&self) -> &'a TruingDirectionManifest {
        self.manifest
    }
    /// Actual complete original outside/evidence/lock/outcome/source parents.
    #[must_use]
    pub const fn outside(&self) -> &'a AdmittedTruingOutsideRoles<'a> {
        self.outside
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> TruingDirectionLimits {
        self.limits
    }
    /// Actual complete logical index/association entries,not bytes or prior storage.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace
    }
    /// All authentic original assignments in declaration order,including reverse traversal.
    pub fn assignments(
        &self,
    ) -> impl ExactSizeIterator<Item = TruingDirectionalOutput<'_>> + DoubleEndedIterator {
        self.assignments
            .iter()
            .map(|association| TruingDirectionalOutput {
                context: self,
                association,
            })
    }
    /// Authentic assignment for an actual declared output tag,in O(log(M+2)) work.
    #[must_use]
    pub fn assignment(&self, tag: LocalTag) -> Option<TruingDirectionalOutput<'_>> {
        self.by_tag
            .get(&tag)
            .and_then(|index| self.assignments.get(*index))
            .map(|association| TruingDirectionalOutput {
                context: self,
                association,
            })
    }
}
/// Authentic actual original direction request and source attribution,not a geometric map.
/// ```compile_fail
/// fn forge<'a>(mut output:sc_core::construction::TruingDirectionalOutput<'a>,
///     other:&'a sc_core::construction::AdmittedTruingDirections<'a>) {output.context=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct TruingDirectionalOutput<'a> {
    context: &'a AdmittedTruingDirections<'a>,
    association: &'a DirectionAssociation<'a>,
}
impl<'a> TruingDirectionalOutput<'a> {
    /// Actual complete admitted direction/source owner.
    #[must_use]
    pub const fn context(self) -> &'a AdmittedTruingDirections<'a> {
        self.context
    }
    /// Original assignment position,not a persistent identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.association.index
    }
    /// Actual original tag/direction record.
    #[must_use]
    pub const fn assignment(self) -> &'a TruingDirectionAssignment {
        self.association.assignment
    }
    /// Authentic actual selected/outside source member in this complete whole context.
    #[must_use]
    pub const fn source(self) -> TruingDirectionSource<'a> {
        self.association.source
    }
    /// Original explicit source-relative orientation request,not Piece traversal direction.
    #[must_use]
    pub const fn parameter_direction(self) -> TruingParameterDirection {
        self.association.assignment.direction
    }
}
