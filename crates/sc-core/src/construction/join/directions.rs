//! Explicit source-relative output parameter orientation;not contour traversal or geometric maps.
use super::{CoveredJoinCutSources, JoinOutputEdge};
use crate::ontology::{EntityId, LocalTag};
use core::fmt;
use std::collections::BTreeMap;

/// Requested output start/end orientation relative to the attributed ascending source range.
/// This is distinct from Piece traversal,physical-copy reflection and sewing direction.
/// ```compile_fail
/// let direction:sc_core::construction::JoinParameterDirection=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinParameterDirection {
    /// Output start/end correspond to source from/to;actual correspondence still needs evidence.
    SameAsSource,
    /// Output start/end correspond to source to/from;no interior or affine map is certified.
    ReverseSource,
}
/// Original persistent tag and explicit requested source-relative parameter direction.
/// ```compile_fail
/// use sc_core::{construction::JoinDirectionAssignment,ontology::{LocalTag,Direction}};
/// JoinDirectionAssignment::new(LocalTag::FIRST,Direction::Original);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinDirectionAssignment {
    tag: LocalTag,
    direction: JoinParameterDirection,
}
impl JoinDirectionAssignment {
    /// Retain both explicit fields without deriving direction from a tag or list position.
    #[must_use]
    pub const fn new(tag: LocalTag, direction: JoinParameterDirection) -> Self {
        Self { tag, direction }
    }
    /// Actual original tag field.
    #[must_use]
    pub const fn tag(&self) -> &LocalTag {
        &self.tag
    }
    /// Actual original explicit orientation request.
    #[must_use]
    pub const fn direction(&self) -> &JoinParameterDirection {
        &self.direction
    }
}
/// Explicit whole output,assignment and index/association entry allowances;no defaults.
/// ```compile_fail
/// let limits:sc_core::construction::JoinDirectionLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinDirectionLimits {
    outputs: usize,
    assignments: usize,
    workspace_entries: usize,
}
impl JoinDirectionLimits {
    /// Choose complete N outputs,M raw assignments and checked2M additional workspace entries.
    #[must_use]
    pub const fn new(outputs: usize, assignments: usize, workspace_entries: usize) -> Self {
        Self {
            outputs,
            assignments,
            workspace_entries,
        }
    }
    /// Every actual original candidate output,including noncut fields;not physical copies or geometry.
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
pub enum JoinDirectionRefusal {
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
    /// Declared operation differs from the actual bound original join operation.
    Operation,
    /// Exactly one assignment is required per actual candidate output.
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
    /// Tag names no actual candidate output,even if it is active in the same catalogue.
    UnknownOutput {
        /// Original assignment position.
        index: usize,
        /// Actual supplied tag.
        tag: LocalTag,
    },
}
/// Actual complete raw manifest,cut-coverage/source parents,policy and first cause.
pub struct JoinDirectionError<'a> {
    manifest: &'a JoinDirectionManifest,
    coverage: &'a CoveredJoinCutSources<'a>,
    limits: JoinDirectionLimits,
    refusal: JoinDirectionRefusal,
}
impl<'a> JoinDirectionError<'a> {
    /// Actual complete original manifest,including every unaccepted assignment.
    #[must_use]
    pub const fn manifest(&self) -> &'a JoinDirectionManifest {
        self.manifest
    }
    /// Actual complete cut coverage and original candidate/identity/catalogue/source parents.
    #[must_use]
    pub const fn coverage(&self) -> &'a CoveredJoinCutSources<'a> {
        self.coverage
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinDirectionLimits {
        self.limits
    }
    /// First complete resource,owner,count or original assignment cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinDirectionRefusal {
        self.refusal
    }
    /// Actual original assignment only for Duplicate/UnknownOutput;other causes return None.
    #[must_use]
    pub fn refused_assignment(&self) -> Option<&'a JoinDirectionAssignment> {
        let index = match self.refusal {
            JoinDirectionRefusal::Duplicate { second, .. } => second,
            JoinDirectionRefusal::UnknownOutput { index, .. } => index,
            _ => return None,
        };
        self.manifest.assignments.get(index)
    }
}
impl fmt::Debug for JoinDirectionError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinDirectionError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinDirectionError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join output parameter directions: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinDirectionError<'_> {}

/// Original operation and ordered explicit assignments;no direction or complete coverage is inferred.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JoinDirectionManifest {
    operation: EntityId,
    assignments: Vec<JoinDirectionAssignment>,
}
impl JoinDirectionManifest {
    /// Retain original operation and assignment order,including invalid duplicate/unknown fields.
    #[must_use]
    pub const fn new(operation: EntityId, assignments: Vec<JoinDirectionAssignment>) -> Self {
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
    pub fn assignments(&self) -> &[JoinDirectionAssignment] {
        &self.assignments
    }
    /// Admit complete N/M/2M before operation,exact cardinality,index allocation or field scans.
    /// Unique actual-candidate membership plus M=N proves one assignment per output;count alone
    /// proves nothing. This includes every noncut candidate,not only the cut-coverage index.
    /// Work O(M log(M+N+2)),with2M new logical index/association entries. No geometry,registry,
    /// source rescaling,interior map,copy/sewing direction or authoring authority is inferred.
    /// # Errors
    /// 2M overflow,N/M/workspace quotas,operation,exact count;then each original
    /// assignment's duplicate or unknown output tag. No accepted prefix escapes.
    pub fn admit<'a>(
        &'a self,
        coverage: &'a CoveredJoinCutSources<'a>,
        limits: JoinDirectionLimits,
    ) -> Result<AdmittedJoinDirections<'a>, JoinDirectionError<'a>> {
        use JoinDirectionRefusal as R;
        let refuse = |refusal| JoinDirectionError {
            manifest: self,
            coverage,
            limits,
            refusal,
        };
        let outputs = coverage.edges().edge_count();
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
        if self.operation
            != coverage
                .edges()
                .identity()
                .roles()
                .catalog()
                .catalog()
                .operation()
        {
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
            let source = coverage
                .edges()
                .edge(tag)
                .ok_or_else(|| refuse(R::UnknownOutput { index, tag }))?;
            assignments.push(DirectionAssociation {
                index,
                assignment,
                source,
            });
        }
        Ok(AdmittedJoinDirections {
            manifest: self,
            coverage,
            limits,
            workspace,
            by_tag,
            assignments,
        })
    }
}
#[derive(Debug)]
struct DirectionAssociation<'a> {
    index: usize,
    assignment: &'a JoinDirectionAssignment,
    source: JoinOutputEdge<'a>,
}

/// Authentic complete candidate orientation requests,not achieved maps or contour placement.
/// ```compile_fail
/// fn forge<'a>(mut directions:sc_core::construction::AdmittedJoinDirections<'a>,
///     other:&'a sc_core::construction::CoveredJoinCutSources<'a>) {directions.coverage=other;}
/// ```
/// ```compile_fail
/// fn escape<'a>(coverage:&'a sc_core::construction::CoveredJoinCutSources<'a>)
///     ->sc_core::construction::AdmittedJoinDirections<'a> {
///     use sc_core::{construction::*,ontology::EntityId};let raw=JoinDirectionManifest::new(EntityId::from_bits(0),vec![]);
///     raw.admit(coverage,JoinDirectionLimits::new(0,0,0)).unwrap()
/// }
/// ```
pub struct AdmittedJoinDirections<'a> {
    manifest: &'a JoinDirectionManifest,
    coverage: &'a CoveredJoinCutSources<'a>,
    limits: JoinDirectionLimits,
    workspace: usize,
    by_tag: BTreeMap<LocalTag, usize>,
    assignments: Vec<DirectionAssociation<'a>>,
}
impl fmt::Debug for AdmittedJoinDirections<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AdmittedJoinDirections")
            .field("operation", &self.manifest.operation)
            .field("assignments", &self.assignments.len())
            .field("limits", &self.limits)
            .finish_non_exhaustive()
    }
}
impl<'a> AdmittedJoinDirections<'a> {
    /// Actual complete original manifest.
    #[must_use]
    pub const fn manifest(&self) -> &'a JoinDirectionManifest {
        self.manifest
    }
    /// Actual complete original cut coverage and candidate/identity/catalogue/source parents.
    #[must_use]
    pub const fn coverage(&self) -> &'a CoveredJoinCutSources<'a> {
        self.coverage
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinDirectionLimits {
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
    ) -> impl ExactSizeIterator<Item = JoinDirectionalOutput<'_>> + DoubleEndedIterator {
        self.assignments
            .iter()
            .map(|association| JoinDirectionalOutput {
                context: self,
                association,
            })
    }
    /// Authentic assignment for an actual declared output tag,in O(log(M+2)) work.
    #[must_use]
    pub fn assignment(&self, tag: LocalTag) -> Option<JoinDirectionalOutput<'_>> {
        self.by_tag
            .get(&tag)
            .and_then(|index| self.assignments.get(*index))
            .map(|association| JoinDirectionalOutput {
                context: self,
                association,
            })
    }
}
/// Authentic actual original direction request and source attribution,not a geometric map.
/// ```compile_fail
/// fn forge<'a>(mut output:sc_core::construction::JoinDirectionalOutput<'a>,
///     other:&'a sc_core::construction::AdmittedJoinDirections<'a>) {output.context=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct JoinDirectionalOutput<'a> {
    context: &'a AdmittedJoinDirections<'a>,
    association: &'a DirectionAssociation<'a>,
}
impl<'a> JoinDirectionalOutput<'a> {
    /// Actual complete admitted direction/source owner.
    #[must_use]
    pub const fn context(self) -> &'a AdmittedJoinDirections<'a> {
        self.context
    }
    /// Original assignment position,not a persistent identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.association.index
    }
    /// Actual original tag/direction record.
    #[must_use]
    pub const fn assignment(self) -> &'a JoinDirectionAssignment {
        self.association.assignment
    }
    /// Authentic actual candidate,role and original occurrence in this complete whole context.
    #[must_use]
    pub const fn source(self) -> JoinOutputEdge<'a> {
        self.association.source
    }
    /// Original explicit source-relative orientation request,not Piece traversal direction.
    #[must_use]
    pub const fn parameter_direction(self) -> JoinParameterDirection {
        self.association.assignment.direction
    }
}
