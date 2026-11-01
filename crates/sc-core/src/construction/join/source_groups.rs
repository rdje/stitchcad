//! Explicit retained construction output-role groups,not whole-field/target occurrence coverage.
use super::{
    DeclaredJoinConstructionSources, JoinConstructionSourceAction, JoinConstructionSourceIntent,
    JoinDirectionalOutput, JoinParameterDirection,
};
use crate::ontology::{Direction, EdgeRange, EntityId, LocalTag, SeamSideId};
use core::fmt;
use std::collections::BTreeMap;
/// One explicit Retain-source snapshot selector and original ordered persistent output references.
/// Source fields are guarded by the actual action parent;indices are not geometry identities.
/// ```compile_fail
/// use sc_core::{construction::*,ontology::*};
/// JoinConstructionMappingGroup::new(SeamSideId::A,0,vec![PointRef::new(EntityId::from_bits(1),LocalTag::FIRST)]);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JoinConstructionMappingGroup {
    side: SeamSideId,
    source_index: usize,
    outputs: Vec<LocalTag>,
}
impl JoinConstructionMappingGroup {
    /// Retain original source selector and every ordered output reference without normalization.
    #[must_use]
    pub const fn new(side: SeamSideId, source_index: usize, outputs: Vec<LocalTag>) -> Self {
        Self {
            side,
            source_index,
            outputs,
        }
    }
    /// Original explicit A/B source role.
    #[must_use]
    pub const fn side(&self) -> SeamSideId {
        self.side
    }
    /// Original source-action snapshot field selector,not a persistent output identity.
    #[must_use]
    pub const fn source_index(&self) -> usize {
        self.source_index
    }
    /// Complete original ordered output references,including repeats before partition checks.
    #[must_use]
    pub fn outputs(&self) -> &[LocalTag] {
        &self.outputs
    }
}
/// Whole source-action K,group G,nested reference L and checked2G+L workspace allowances.
/// ```compile_fail
/// let limits:sc_core::construction::JoinConstructionGroupLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinConstructionGroupLimits {
    sources: usize,
    groups: usize,
    references: usize,
    workspace_entries: usize,
}
impl JoinConstructionGroupLimits {
    /// Bound complete source scopes and group headers before child lengths,then every tag reference
    /// and2G+L entries before fields/allocation. Earlier input/context/candidate storage stays separate.
    #[must_use]
    pub const fn new(
        sources: usize,
        groups: usize,
        references: usize,
        workspace_entries: usize,
    ) -> Self {
        Self {
            sources,
            groups,
            references,
            workspace_entries,
        }
    }
    /// All actual original construction action scopes,including Omit.
    #[must_use]
    pub const fn sources(self) -> usize {
        self.sources
    }
    /// Every original raw group header.
    #[must_use]
    pub const fn groups(self) -> usize {
        self.groups
    }
    /// Every original nested tag reference,including repetitions.
    #[must_use]
    pub const fn references(self) -> usize {
        self.references
    }
    /// Complete2G+L source-index/group/output association entries,not bytes.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// First complete resource,owner/count,original source group or output attribution cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinConstructionGroupRefusal {
    /// Complete nested L cannot be represented.
    ReferenceOverflow,
    /// Checked complete2G+L cannot be represented.
    WorkspaceOverflow,
    /// Complete actual source scopes exceed policy before source-intent scans.
    Sources {
        /// Whole actual K.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete raw groups exceed policy before child-length reads.
    Groups {
        /// Whole actual G.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete nested references exceed policy before tag fields.
    References {
        /// Whole actual L.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete2G+L workspace exceeds policy.
    Workspace {
        /// Whole required entries.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Raw operation differs from the actual original join operation.
    Operation,
    /// Exactly one group per actual Retain action is required.
    Count {
        /// Actual complete Retain population.
        expected: usize,
        /// Complete original raw groups.
        actual: usize,
    },
    /// An original source-action selector repeats across raw groups.
    DuplicateSource {
        /// First original group.
        first: usize,
        /// Repeated original group.
        second: usize,
    },
    /// Selector names no actual guarded original construction action.
    MissingSource {
        /// Original group.
        group: usize,
    },
    /// Selected source action explicitly requests Omit,not Retain.
    OmittedSource {
        /// Original group.
        group: usize,
    },
    /// A Retain group requires explicit positive output content.
    EmptyGroup {
        /// Original group.
        group: usize,
    },
    /// Tag names no actual directional candidate output.
    UnknownOutput {
        /// Original group.
        group: usize,
        /// Original nested tag position.
        index: usize,
    },
    /// Candidate uses another original A/B source role,even for equal EdgeRefs.
    SourceSide {
        /// Original group.
        group: usize,
        /// Original nested tag position.
        index: usize,
    },
    /// Candidate range belongs to another original persistent source edge.
    SourceEdge {
        /// Original group.
        group: usize,
        /// Original nested tag position.
        index: usize,
    },
}
/// Whole raw groups,actual source-action/use context,limits and first cause survive refusal.
pub struct JoinConstructionGroupError<'a> {
    manifest: &'a JoinConstructionGroupManifest,
    sources: &'a DeclaredJoinConstructionSources<'a>,
    limits: JoinConstructionGroupLimits,
    refusal: JoinConstructionGroupRefusal,
}
impl<'a> JoinConstructionGroupError<'a> {
    /// Actual complete original group proposal,including unaccepted suffixes.
    #[must_use]
    pub const fn manifest(&self) -> &'a JoinConstructionGroupManifest {
        self.manifest
    }
    /// Actual complete guarded source-action/construction/cut-path/candidate parents.
    #[must_use]
    pub const fn sources(&self) -> &'a DeclaredJoinConstructionSources<'a> {
        self.sources
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinConstructionGroupLimits {
        self.limits
    }
    /// First whole resource,owner/count or original attribution cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinConstructionGroupRefusal {
        self.refusal
    }
    /// Actual first refused raw group;whole resource/operation/count causes return None.
    #[must_use]
    pub fn refused_group(&self) -> Option<&'a JoinConstructionMappingGroup> {
        use JoinConstructionGroupRefusal as R;
        let index = match self.refusal {
            R::DuplicateSource { second, .. } => second,
            R::MissingSource { group }
            | R::OmittedSource { group }
            | R::EmptyGroup { group }
            | R::UnknownOutput { group, .. }
            | R::SourceSide { group, .. }
            | R::SourceEdge { group, .. } => group,
            _ => return None,
        };
        self.manifest.groups.get(index)
    }
    /// Actual original tag only for output-specific causes;other causes return None.
    #[must_use]
    pub fn refused_tag(&self) -> Option<&'a LocalTag> {
        use JoinConstructionGroupRefusal as R;
        let index = match self.refusal {
            R::UnknownOutput { index, .. }
            | R::SourceSide { index, .. }
            | R::SourceEdge { index, .. } => index,
            _ => return None,
        };
        self.refused_group()?.outputs.get(index)
    }
    /// Actual guarded source action for a group-specific cause,if that selector exists.
    #[must_use]
    pub fn refused_source_action(&self) -> Option<JoinConstructionSourceAction<'a>> {
        let group = self.refused_group()?;
        self.sources.assignment(group.side, group.source_index)
    }
    /// Actual directional output only for source-side/edge mismatch;other causes return None.
    #[must_use]
    pub fn refused_output(&self) -> Option<JoinDirectionalOutput<'a>> {
        if !matches!(
            self.refusal,
            JoinConstructionGroupRefusal::SourceSide { .. }
                | JoinConstructionGroupRefusal::SourceEdge { .. }
        ) {
            return None;
        }
        self.sources
            .uses()
            .connectivity()
            .contours()
            .directions()
            .assignment(*self.refused_tag()?)
    }
}
impl fmt::Debug for JoinConstructionGroupError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinConstructionGroupError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinConstructionGroupError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join retained construction groups: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinConstructionGroupError<'_> {}
fn key(side: SeamSideId, index: usize) -> (bool, usize) {
    (side == SeamSideId::B, index)
}
/// Original explicit operation and ordered retained-source output reference groups.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JoinConstructionGroupManifest {
    operation: EntityId,
    groups: Vec<JoinConstructionMappingGroup>,
}
impl JoinConstructionGroupManifest {
    /// Retain exact original operation/groups/references,including invalid or repeated fields.
    #[must_use]
    pub const fn new(operation: EntityId, groups: Vec<JoinConstructionMappingGroup>) -> Self {
        Self { operation, groups }
    }
    /// Actual original declared operation.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Complete original raw group headers and ordered references.
    #[must_use]
    pub fn groups(&self) -> &[JoinConstructionMappingGroup] {
        &self.groups
    }
    /// Admit all source scopes K/group headers G before child lengths,then complete checked L/2G+L
    /// before fields/allocation. Operation/exact Retain count precede original duplicate/missing/
    /// omitted/empty source groups,then each actual directional output's A/B and literal edge frame.
    /// One unique actual Retain source per group plus exact count covers every Retain action.
    /// Repeated references/source geometry remain original;whole0..1 ordered partitions,target use
    /// presence/traversal/multiplicity,omission evidence,currentness and effects remain required later.
    /// Work O(K+G log(G+K+2)+L log(N+2)+1),2G+L new entries for N original candidates. No registry,
    /// provider,geometry or evidence call;earlier immutable input/source/candidate resources remain.
    /// # Errors
    /// Whole K/G quotas,L/2G+L arithmetic,L/workspace quotas,operation,Retain count;original group/
    /// source/role/frame causes. No accepted prefix escapes;no sorting/rescaling/issuance occurs.
    pub fn admit<'a>(
        &'a self,
        sources: &'a DeclaredJoinConstructionSources<'a>,
        limits: JoinConstructionGroupLimits,
    ) -> Result<DeclaredJoinConstructionGroups<'a>, JoinConstructionGroupError<'a>> {
        use JoinConstructionGroupRefusal as R;
        let refuse = |refusal| JoinConstructionGroupError {
            manifest: self,
            sources,
            limits,
            refusal,
        };
        let k = sources.source_count();
        let g = self.groups.len();
        if k > limits.sources {
            return Err(refuse(R::Sources {
                required: k,
                allowed: limits.sources,
            }));
        }
        if g > limits.groups {
            return Err(refuse(R::Groups {
                required: g,
                allowed: limits.groups,
            }));
        }
        let l = self
            .groups
            .iter()
            .map(|group| group.outputs.len())
            .try_fold(0usize, usize::checked_add)
            .ok_or_else(|| refuse(R::ReferenceOverflow))?;
        let workspace = g
            .checked_mul(2)
            .and_then(|entries| entries.checked_add(l))
            .ok_or_else(|| refuse(R::WorkspaceOverflow))?;
        if l > limits.references {
            return Err(refuse(R::References {
                required: l,
                allowed: limits.references,
            }));
        }
        if workspace > limits.workspace_entries {
            return Err(refuse(R::Workspace {
                required: workspace,
                allowed: limits.workspace_entries,
            }));
        }
        if self.operation != sources.manifest().operation() {
            return Err(refuse(R::Operation));
        }
        let retained = sources
            .assignments()
            .filter(|action| matches!(action.intent(), JoinConstructionSourceIntent::Retain))
            .count();
        if g != retained {
            return Err(refuse(R::Count {
                expected: retained,
                actual: g,
            }));
        }
        let directions = sources.uses().connectivity().contours().directions();
        let mut by_source = BTreeMap::new();
        let mut groups = Vec::with_capacity(g);
        let mut outputs = Vec::with_capacity(l);
        for (group_index, group) in self.groups.iter().enumerate() {
            if let Some(first) = by_source.insert(key(group.side, group.source_index), group_index)
            {
                return Err(refuse(R::DuplicateSource {
                    first,
                    second: group_index,
                }));
            }
            let source = sources
                .assignment(group.side, group.source_index)
                .ok_or_else(|| refuse(R::MissingSource { group: group_index }))?;
            if !matches!(source.intent(), JoinConstructionSourceIntent::Retain) {
                return Err(refuse(R::OmittedSource { group: group_index }));
            }
            if group.outputs.is_empty() {
                return Err(refuse(R::EmptyGroup { group: group_index }));
            }
            let start = outputs.len();
            for (index, tag) in group.outputs.iter().enumerate() {
                let output = directions.assignment(*tag).ok_or_else(|| {
                    refuse(R::UnknownOutput {
                        group: group_index,
                        index,
                    })
                })?;
                if output.source().candidate().side() != source.assignment().side() {
                    return Err(refuse(R::SourceSide {
                        group: group_index,
                        index,
                    }));
                }
                if output.source().source().edge() != source.source_field().edge {
                    return Err(refuse(R::SourceEdge {
                        group: group_index,
                        index,
                    }));
                }
                outputs.push(OutputAssociation {
                    group_index,
                    group,
                    index,
                    tag,
                    source,
                    output,
                });
            }
            groups.push(GroupAssociation {
                index: group_index,
                group,
                source,
                start,
            });
        }
        Ok(DeclaredJoinConstructionGroups {
            manifest: self,
            sources,
            limits,
            workspace,
            by_source,
            groups,
            outputs,
        })
    }
}
#[derive(Debug)]
struct GroupAssociation<'a> {
    index: usize,
    group: &'a JoinConstructionMappingGroup,
    source: JoinConstructionSourceAction<'a>,
    start: usize,
}
#[derive(Debug)]
struct OutputAssociation<'a> {
    group_index: usize,
    group: &'a JoinConstructionMappingGroup,
    index: usize,
    tag: &'a LocalTag,
    source: JoinConstructionSourceAction<'a>,
    output: JoinDirectionalOutput<'a>,
}
/// Complete actual Retain-source role grouping,not whole-field/target occurrence mapping proof.
/// ```compile_fail
/// fn forge<'a>(mut groups:sc_core::construction::DeclaredJoinConstructionGroups<'a>,other:&'a sc_core::construction::JoinConstructionGroupManifest) {groups.manifest=other;}
/// ```
/// ```compile_fail
/// fn escape(raw:sc_core::construction::JoinConstructionGroupManifest,sources:sc_core::construction::DeclaredJoinConstructionSources<'_>)
///     ->sc_core::construction::DeclaredJoinConstructionGroups<'static> {
///     raw.admit(&sources,sc_core::construction::JoinConstructionGroupLimits::new(0,0,0,0)).unwrap()
/// }
/// ```
pub struct DeclaredJoinConstructionGroups<'a> {
    manifest: &'a JoinConstructionGroupManifest,
    sources: &'a DeclaredJoinConstructionSources<'a>,
    limits: JoinConstructionGroupLimits,
    workspace: usize,
    by_source: BTreeMap<(bool, usize), usize>,
    groups: Vec<GroupAssociation<'a>>,
    outputs: Vec<OutputAssociation<'a>>,
}
impl fmt::Debug for DeclaredJoinConstructionGroups<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeclaredJoinConstructionGroups")
            .field("operation", &self.manifest.operation)
            .field("groups", &self.groups.len())
            .field("references", &self.outputs.len())
            .field("limits", &self.limits)
            .finish_non_exhaustive()
    }
}
impl<'a> DeclaredJoinConstructionGroups<'a> {
    /// Actual complete original proposal.
    #[must_use]
    pub const fn manifest(&self) -> &'a JoinConstructionGroupManifest {
        self.manifest
    }
    /// Actual complete original guarded source-action/use/cut-path context.
    #[must_use]
    pub const fn sources(&self) -> &'a DeclaredJoinConstructionSources<'a> {
        self.sources
    }
    /// Original complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinConstructionGroupLimits {
        self.limits
    }
    /// Complete new2G+L source-index/group/output entries,not bytes or earlier storage.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace
    }
    /// Complete actual Retain group count.
    #[must_use]
    pub fn group_count(&self) -> usize {
        self.groups.len()
    }
    /// Complete original tag-reference population,including repetitions across source scopes.
    #[must_use]
    pub fn reference_count(&self) -> usize {
        self.outputs.len()
    }
    /// Authentic groups in original declaration order,and reverse.
    pub fn groups(
        &self,
    ) -> impl ExactSizeIterator<Item = JoinConstructionOutputGroup<'_>> + DoubleEndedIterator {
        self.groups
            .iter()
            .map(move |association| JoinConstructionOutputGroup {
                context: self,
                association,
            })
    }
    /// Authentic group for an actual Retain-source snapshot slot.
    #[must_use]
    pub fn group(
        &self,
        side: SeamSideId,
        source_index: usize,
    ) -> Option<JoinConstructionOutputGroup<'_>> {
        self.by_source
            .get(&key(side, source_index))
            .and_then(|i| self.groups.get(*i))
            .map(|association| JoinConstructionOutputGroup {
                context: self,
                association,
            })
    }
    /// Every authentic reference in original group/tag order,and reverse.
    pub fn outputs(
        &self,
    ) -> impl ExactSizeIterator<Item = JoinConstructionGroupOutput<'_>> + DoubleEndedIterator {
        self.outputs
            .iter()
            .map(move |association| JoinConstructionGroupOutput {
                context: self,
                association,
            })
    }
}
/// One authentic original group and actual guarded Retain-source action.
/// ```compile_fail
/// fn forge<'a>(mut group:sc_core::construction::JoinConstructionOutputGroup<'a>,other:&'a sc_core::construction::DeclaredJoinConstructionGroups<'a>) {group.context=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct JoinConstructionOutputGroup<'a> {
    context: &'a DeclaredJoinConstructionGroups<'a>,
    association: &'a GroupAssociation<'a>,
}
impl<'a> JoinConstructionOutputGroup<'a> {
    /// Actual complete declared grouping context.
    #[must_use]
    pub const fn context(self) -> &'a DeclaredJoinConstructionGroups<'a> {
        self.context
    }
    /// Original group ordinal,not a persistent identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.association.index
    }
    /// Actual original source selector and ordered tag group.
    #[must_use]
    pub const fn definition(self) -> &'a JoinConstructionMappingGroup {
        self.association.group
    }
    /// Authentic actual guarded source action/field,not a replacement expected field.
    #[must_use]
    pub const fn source_action(self) -> JoinConstructionSourceAction<'a> {
        self.association.source
    }
    /// Authentic original ordered reference members,and reverse;no partition/target-use proof.
    pub fn outputs(
        self,
    ) -> impl ExactSizeIterator<Item = JoinConstructionGroupOutput<'a>> + DoubleEndedIterator {
        self.context
            .outputs()
            .skip(self.association.start)
            .take(self.association.group.outputs.len())
    }
}
/// One authentic original reference,actual source action and directional candidate attribution.
/// ```compile_fail
/// fn forge<'a>(mut output:sc_core::construction::JoinConstructionGroupOutput<'a>,other:&'a sc_core::construction::DeclaredJoinConstructionGroups<'a>) {output.context=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct JoinConstructionGroupOutput<'a> {
    context: &'a DeclaredJoinConstructionGroups<'a>,
    association: &'a OutputAssociation<'a>,
}
impl<'a> JoinConstructionGroupOutput<'a> {
    /// Actual complete grouping context.
    #[must_use]
    pub const fn context(self) -> &'a DeclaredJoinConstructionGroups<'a> {
        self.context
    }
    /// Original group ordinal,not an output identity.
    #[must_use]
    pub const fn group_index(self) -> usize {
        self.association.group_index
    }
    /// Actual original group/source selector.
    #[must_use]
    pub const fn group_definition(self) -> &'a JoinConstructionMappingGroup {
        self.association.group
    }
    /// Original reference position within its original group.
    #[must_use]
    pub const fn index(self) -> usize {
        self.association.index
    }
    /// Actual original persistent role-reference field.
    #[must_use]
    pub const fn tag(self) -> &'a LocalTag {
        self.association.tag
    }
    /// Authentic actual original source action and field.
    #[must_use]
    pub const fn source_action(self) -> JoinConstructionSourceAction<'a> {
        self.association.source
    }
    /// Actual original directional output/candidate/source attribution.
    #[must_use]
    pub const fn output(self) -> JoinDirectionalOutput<'a> {
        self.association.output
    }
    /// Actual candidate's positive ascending source interval;not whole-field/target coverage.
    #[must_use]
    pub const fn source(self) -> &'a EdgeRange {
        self.association.output.source().source()
    }
    /// Required proposed target traversal to preserve the original field direction under the
    /// explicit requested parameter orientation. It is not an authored/achieved target-use value.
    #[must_use]
    pub const fn required_traversal(self) -> Direction {
        match (
            self.association.source.source_field().direction,
            self.association.output.parameter_direction(),
        ) {
            (Direction::Original, JoinParameterDirection::SameAsSource)
            | (Direction::Reversed, JoinParameterDirection::ReverseSource) => Direction::Original,
            _ => Direction::Reversed,
        }
    }
}
