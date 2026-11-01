//! Explicit complete cut-use declarations,not connected/geometrically valid joined contours.
use super::{AdmittedJoinDirections, JoinDirectionalOutput, JoinSourceOccurrenceLocation};
use crate::ontology::{Direction, EdgeRef, EntityId, LocalTag};
use core::fmt;
use std::collections::BTreeMap;

/// One explicit persistent output tag and traversal in that output's own parameter frame.
/// ```compile_fail
/// use sc_core::{construction::*,ontology::LocalTag};
/// JoinCutContourUse::new(LocalTag::FIRST,JoinParameterDirection::SameAsSource);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinCutContourUse {
    tag: LocalTag,
    direction: Direction,
}
impl JoinCutContourUse {
    /// Retain explicit fields;infer no traversal from source,copy,sewing or parameter direction.
    #[must_use]
    pub const fn new(tag: LocalTag, direction: Direction) -> Self {
        Self { tag, direction }
    }
    /// Actual original persistent-tag field,not a source/target list ordinal.
    #[must_use]
    pub const fn tag(&self) -> &LocalTag {
        &self.tag
    }
    /// Actual original traversal field,distinct from source-relative parameter orientation.
    #[must_use]
    pub const fn direction(&self) -> &Direction {
        &self.direction
    }
}
/// Original declared result cut-use location,not a persistent identity or source location.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinCutContourUseLocation {
    /// Original proposed outer-boundary position.
    Boundary(usize),
    /// Original proposed hole and position within it.
    Hole {
        /// Original target hole group.
        hole: usize,
        /// Original use within that hole.
        index: usize,
    },
}
/// Whole actual output N,target hole H,use U and checked2U+H new entry allowances.
/// ```compile_fail
/// let limits:sc_core::construction::JoinCutContourLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinCutContourLimits {
    outputs: usize,
    holes: usize,
    uses: usize,
    workspace_entries: usize,
}
impl JoinCutContourLimits {
    /// Admit all N/H before child lengths,then all U/2U+H before use fields or allocation.
    /// Entry counts do not measure allocator bytes or earlier caller/context storage.
    #[must_use]
    pub const fn new(outputs: usize, holes: usize, uses: usize, workspace_entries: usize) -> Self {
        Self {
            outputs,
            holes,
            uses,
            workspace_entries,
        }
    }
    /// All actual candidates,including noncut outputs.
    #[must_use]
    pub const fn outputs(self) -> usize {
        self.outputs
    }
    /// All original target hole headers,including empty groups.
    #[must_use]
    pub const fn holes(self) -> usize {
        self.holes
    }
    /// All original boundary/hole use occurrences.
    #[must_use]
    pub const fn uses(self) -> usize {
        self.uses
    }
    /// Complete2U+H index/use/hole association entries,not bytes.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// First whole resource,owner,nonempty/count or original cut-membership refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinCutContourRefusal {
    /// Whole U cannot be represented after complete H admission.
    UseCountOverflow,
    /// Checked complete2U+H cannot be represented.
    WorkspaceOverflow,
    /// Complete actual candidate population exceeds policy.
    Outputs {
        /// Complete actual N.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete original target hole headers exceed policy.
    Holes {
        /// Complete actual H.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete original target use population exceeds policy.
    Uses {
        /// Complete actual U.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete additional index/use/hole population exceeds policy.
    Workspace {
        /// Complete actual2U+H.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Proposed operation differs from the actual original join operation.
    Operation,
    /// A proposed outer boundary must have at least one explicit use.
    EmptyBoundary,
    /// A proposed target hole must have at least one explicit use.
    EmptyHole {
        /// First original empty group.
        hole: usize,
    },
    /// Exactly one use per actual cut candidate is required,not one per every noncut output.
    Count {
        /// Actual cut-candidate population.
        expected: usize,
        /// Complete original boundary/hole uses.
        actual: usize,
    },
    /// An explicit persistent tag repeats across the original boundary/hole declarations.
    DuplicateTag {
        /// First original declared use.
        first: JoinCutContourUseLocation,
        /// Repeated original declared use.
        second: JoinCutContourUseLocation,
        /// Actual repeated tag.
        tag: LocalTag,
    },
    /// Tag names no candidate in the actual complete direction context.
    UnknownOutput {
        /// Original declared use.
        location: JoinCutContourUseLocation,
        /// Actual original tag.
        tag: LocalTag,
    },
    /// Actual candidate is attributed to construction/fold rather than an original cut field.
    NonCutOutput {
        /// Original declared use.
        location: JoinCutContourUseLocation,
        /// Actual original tag.
        tag: LocalTag,
    },
}
/// Whole original proposal,actual directions/context,policy and first cause survive refusal.
pub struct JoinCutContourError<'a> {
    manifest: &'a JoinCutContourManifest,
    directions: &'a AdmittedJoinDirections<'a>,
    limits: JoinCutContourLimits,
    refusal: JoinCutContourRefusal,
}
impl<'a> JoinCutContourError<'a> {
    /// Actual complete original boundary/hole proposal,including invalid unaccepted fields.
    #[must_use]
    pub const fn manifest(&self) -> &'a JoinCutContourManifest {
        self.manifest
    }
    /// Actual complete original direction/coverage/identity/source context.
    #[must_use]
    pub const fn directions(&self) -> &'a AdmittedJoinDirections<'a> {
        self.directions
    }
    /// Original explicit complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinCutContourLimits {
        self.limits
    }
    /// First whole-resource,owner,nonempty/count or original use cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinCutContourRefusal {
        self.refusal
    }
    /// Actual refused original field only for duplicate/unknown/noncut use causes;others return None.
    #[must_use]
    pub fn refused_use(&self) -> Option<&'a JoinCutContourUse> {
        let location = match self.refusal {
            JoinCutContourRefusal::DuplicateTag { second, .. } => second,
            JoinCutContourRefusal::UnknownOutput { location, .. }
            | JoinCutContourRefusal::NonCutOutput { location, .. } => location,
            _ => return None,
        };
        match location {
            JoinCutContourUseLocation::Boundary(index) => self.manifest.boundary.get(index),
            JoinCutContourUseLocation::Hole { hole, index } => {
                self.manifest.holes.get(hole)?.get(index)
            }
        }
    }
}
impl fmt::Debug for JoinCutContourError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinCutContourError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinCutContourError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join cut-use declarations: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinCutContourError<'_> {}
/// Original explicit operation,boundary and independently ordered hole-use lists.
/// A raw list is neither an admitted use set nor a connected,CCW/simple canonical Piece.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JoinCutContourManifest {
    operation: EntityId,
    boundary: Vec<JoinCutContourUse>,
    holes: Vec<Vec<JoinCutContourUse>>,
}
impl JoinCutContourManifest {
    /// Retain complete original lists/order/invalid fields without reordering or issuing identities.
    #[must_use]
    pub const fn new(
        operation: EntityId,
        boundary: Vec<JoinCutContourUse>,
        holes: Vec<Vec<JoinCutContourUse>>,
    ) -> Self {
        Self {
            operation,
            boundary,
            holes,
        }
    }
    /// Original operation field,compared to the actual join operation during admission.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Complete original proposed outer-boundary uses,without a connectivity/winding certificate.
    #[must_use]
    pub fn boundary(&self) -> &[JoinCutContourUse] {
        &self.boundary
    }
    /// Complete original independently ordered proposed hole groups.
    #[must_use]
    pub fn holes(&self) -> &[Vec<JoinCutContourUse>] {
        &self.holes
    }
    /// Admit N/H before hole-child lengths,then complete checked U/2U+H before fields/allocation.
    /// Operation,nonempty boundary/holes and U=C precede original use duplicate/unknown/noncut
    /// checks. Unique membership in actual cut candidates plus U=C proves complete cut placement
    /// only. Original order/traversal survives;no source path,endpoint gluing,loop classification,
    /// CCW/closure/simplicity,noncut/fold reconciliation,currentness or geometry proof is inferred.
    /// Work O(H+U log(U+N+2)),2U+H new index/use/hole entries;no source scan,registry/provider call.
    /// Prior complete original/candidate/direction/coverage resources and caller storage remain.
    /// # Errors
    /// N/H quotas,U/2U+H arithmetic,U/workspace quotas;operation,empty boundary/first empty hole,
    /// exact cut count;then original boundary/hole use duplicate,unknown or noncut output.
    pub fn admit<'a>(
        &'a self,
        directions: &'a AdmittedJoinDirections<'a>,
        limits: JoinCutContourLimits,
    ) -> Result<DeclaredJoinCutContours<'a>, JoinCutContourError<'a>> {
        use JoinCutContourRefusal as R;
        let refuse = |refusal| JoinCutContourError {
            manifest: self,
            directions,
            limits,
            refusal,
        };
        let coverage = directions.coverage();
        let n = coverage.edges().edge_count();
        let h = self.holes.len();
        if n > limits.outputs {
            return Err(refuse(R::Outputs {
                required: n,
                allowed: limits.outputs,
            }));
        }
        if h > limits.holes {
            return Err(refuse(R::Holes {
                required: h,
                allowed: limits.holes,
            }));
        }
        let u = self
            .holes
            .iter()
            .map(Vec::len)
            .try_fold(self.boundary.len(), usize::checked_add)
            .ok_or_else(|| refuse(R::UseCountOverflow))?;
        let workspace = u
            .checked_mul(2)
            .and_then(|count| count.checked_add(h))
            .ok_or_else(|| refuse(R::WorkspaceOverflow))?;
        if u > limits.uses {
            return Err(refuse(R::Uses {
                required: u,
                allowed: limits.uses,
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
        if self.boundary.is_empty() {
            return Err(refuse(R::EmptyBoundary));
        }
        if let Some(hole) = self.holes.iter().position(Vec::is_empty) {
            return Err(refuse(R::EmptyHole { hole }));
        }
        let c = coverage.cut_edge_count();
        if u != c {
            return Err(refuse(R::Count {
                expected: c,
                actual: u,
            }));
        }
        let mut by_tag = BTreeMap::new();
        let mut uses = Vec::with_capacity(u);
        let mut holes = Vec::with_capacity(h);
        let mut accept = |location, declared: &'a JoinCutContourUse| {
            let tag = declared.tag;
            if let Some((first, _)) = by_tag.insert(tag, (location, uses.len())) {
                return Err(refuse(R::DuplicateTag {
                    first,
                    second: location,
                    tag,
                }));
            }
            let output = directions
                .assignment(tag)
                .ok_or_else(|| refuse(R::UnknownOutput { location, tag }))?;
            if !matches!(
                output.source().occurrence().location(),
                JoinSourceOccurrenceLocation::Boundary(_)
                    | JoinSourceOccurrenceLocation::Hole { .. }
            ) {
                return Err(refuse(R::NonCutOutput { location, tag }));
            }
            uses.push(UseAssociation {
                location,
                declared,
                output,
            });
            Ok(())
        };
        for (index, declared) in self.boundary.iter().enumerate() {
            accept(JoinCutContourUseLocation::Boundary(index), declared)?;
        }
        let mut start = self.boundary.len();
        for (hole, group) in self.holes.iter().enumerate() {
            for (index, declared) in group.iter().enumerate() {
                accept(JoinCutContourUseLocation::Hole { hole, index }, declared)?;
            }
            holes.push(HoleAssociation {
                index: hole,
                declared: group,
                start,
            });
            start += group.len(); // Complete checked U already bounds every prefix.
        }
        Ok(DeclaredJoinCutContours {
            manifest: self,
            directions,
            limits,
            workspace,
            by_tag,
            uses,
            holes,
        })
    }
}
#[derive(Debug)]
struct UseAssociation<'a> {
    location: JoinCutContourUseLocation,
    declared: &'a JoinCutContourUse,
    output: JoinDirectionalOutput<'a>,
}
#[derive(Debug)]
struct HoleAssociation<'a> {
    index: usize,
    declared: &'a [JoinCutContourUse],
    start: usize,
}
/// Complete authentic cut-use declarations only;not connected/valid/canonical result geometry.
/// ```compile_fail
/// fn forge<'a>(mut contours:sc_core::construction::DeclaredJoinCutContours<'a>,other:&'a sc_core::construction::JoinCutContourManifest) {contours.manifest=other;}
/// ```
/// ```compile_fail
/// fn escape(raw:sc_core::construction::JoinCutContourManifest,directions:sc_core::construction::AdmittedJoinDirections<'_>)
///     ->sc_core::construction::DeclaredJoinCutContours<'static> {
///     raw.admit(&directions,sc_core::construction::JoinCutContourLimits::new(0,0,0,0)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct DeclaredJoinCutContours<'a> {
    manifest: &'a JoinCutContourManifest,
    directions: &'a AdmittedJoinDirections<'a>,
    limits: JoinCutContourLimits,
    workspace: usize,
    by_tag: BTreeMap<LocalTag, (JoinCutContourUseLocation, usize)>,
    uses: Vec<UseAssociation<'a>>,
    holes: Vec<HoleAssociation<'a>>,
}
impl<'a> DeclaredJoinCutContours<'a> {
    /// Actual complete original proposal,including explicit traversal and loop ordering.
    #[must_use]
    pub const fn manifest(&self) -> &'a JoinCutContourManifest {
        self.manifest
    }
    /// Actual complete original directions/coverage/identity/source parents.
    #[must_use]
    pub const fn directions(&self) -> &'a AdmittedJoinDirections<'a> {
        self.directions
    }
    /// Original explicit complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinCutContourLimits {
        self.limits
    }
    /// Complete new2U+H index/use/hole entries,not allocator bytes or earlier storage.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace
    }
    /// Complete explicit cut-use count,equal to actual cut-candidate population.
    #[must_use]
    pub fn use_count(&self) -> usize {
        self.uses.len()
    }
    /// Complete explicit result hole-group count.
    #[must_use]
    pub fn hole_count(&self) -> usize {
        self.holes.len()
    }
    /// Authentic uses in original boundary then hole-group/field order,and reverse.
    pub fn uses(
        &self,
    ) -> impl ExactSizeIterator<Item = JoinPlacedCutUse<'_>> + DoubleEndedIterator {
        self.uses.iter().map(move |association| JoinPlacedCutUse {
            context: self,
            association,
        })
    }
    /// Authentic explicitly ordered boundary uses,without achieved adjacency/winding proof.
    pub fn boundary(
        &self,
    ) -> impl ExactSizeIterator<Item = JoinPlacedCutUse<'_>> + DoubleEndedIterator {
        self.uses().take(self.manifest.boundary.len())
    }
    /// Authentic original hole groups in authored order,and reverse.
    pub fn holes(&self) -> impl ExactSizeIterator<Item = JoinCutHole<'_>> + DoubleEndedIterator {
        self.holes.iter().map(move |association| JoinCutHole {
            context: self,
            association,
        })
    }
    /// Authentic unique cut use for an actual explicit output tag.
    #[must_use]
    pub fn use_for(&self, tag: LocalTag) -> Option<JoinPlacedCutUse<'_>> {
        self.by_tag
            .get(&tag)
            .and_then(|(_, index)| self.uses.get(*index))
            .map(|association| JoinPlacedCutUse {
                context: self,
                association,
            })
    }
}
/// Authentic one original cut-use field and its complete direction/source association.
/// ```compile_fail
/// fn forge<'a>(mut placed:sc_core::construction::JoinPlacedCutUse<'a>,other:&'a sc_core::construction::DeclaredJoinCutContours<'a>) {placed.context=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct JoinPlacedCutUse<'a> {
    context: &'a DeclaredJoinCutContours<'a>,
    association: &'a UseAssociation<'a>,
}
impl<'a> JoinPlacedCutUse<'a> {
    /// Actual whole admitted cut-use declaration context.
    #[must_use]
    pub const fn context(self) -> &'a DeclaredJoinCutContours<'a> {
        self.context
    }
    /// Actual original declared result field location,not an output identity.
    #[must_use]
    pub const fn location(self) -> JoinCutContourUseLocation {
        self.association.location
    }
    /// Actual original explicit tag/traversal field.
    #[must_use]
    pub const fn declared(self) -> &'a JoinCutContourUse {
        self.association.declared
    }
    /// Actual original direction assignment and source attribution.
    #[must_use]
    pub const fn output(self) -> JoinDirectionalOutput<'a> {
        self.association.output
    }
    /// Actual original explicit proposed traversal,not inferred from parameter direction.
    #[must_use]
    pub const fn direction(self) -> &'a Direction {
        self.association.declared.direction()
    }
    /// Declared actual operation/persistent-tag tuple,without issuing or publishing an edge.
    #[must_use]
    pub fn logical_output(self) -> EdgeRef {
        self.association.output.source().logical_output()
    }
}
/// One authentic original hole header and its explicit use group,not a geometric containment proof.
/// ```compile_fail
/// fn forge<'a>(mut hole:sc_core::construction::JoinCutHole<'a>,other:&'a sc_core::construction::DeclaredJoinCutContours<'a>) {hole.context=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct JoinCutHole<'a> {
    context: &'a DeclaredJoinCutContours<'a>,
    association: &'a HoleAssociation<'a>,
}
impl<'a> JoinCutHole<'a> {
    /// Actual complete original cut-use declaration context.
    #[must_use]
    pub const fn context(self) -> &'a DeclaredJoinCutContours<'a> {
        self.context
    }
    /// Actual original result hole-group index,not a persistent entity identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.association.index
    }
    /// Actual original independently ordered use group.
    #[must_use]
    pub const fn declared(self) -> &'a [JoinCutContourUse] {
        self.association.declared
    }
    /// Authentic original hole uses and reverse traversal;no reordering/normalization.
    pub fn uses(self) -> impl ExactSizeIterator<Item = JoinPlacedCutUse<'a>> + DoubleEndedIterator {
        self.context
            .uses()
            .skip(self.association.start)
            .take(self.association.declared.len())
    }
}
