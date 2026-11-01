//! Ordered declared curve alternatives and complete assignment priority;no feasibility evaluation.
use super::{RequiredTruingLineage, TruingDirectionSource, TruingDirectionalOutput};
use crate::ontology::{DirectedRange, EntityId, LocalTag};
use core::fmt;
use std::collections::BTreeMap;

/// Explicit candidate priority,independent of identity ordering;no default or selected result.
/// ```compile_fail
/// let policy:sc_core::construction::TruingCurveSelectionPolicy=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruingCurveSelectionPolicy {
    /// Evaluate complete assignments in authored group/choice order,rightmost group fastest.
    /// Select the first with every actual certified predicate passing. Unknown,unsupported,
    /// uncertified,resource and cancellation causes cannot be skipped as infeasible assignments.
    /// Exhaustion is scoped to the declared alternatives,not every possible geometric curve.
    FirstFeasibleCompleteAssignment,
}
/// Raw selected-output role and ordered positive directed curve sources;not evaluated geometry.
/// ```compile_fail
/// use sc_core::{construction::TruingCurveAlternatives,ontology::{EdgeRange,EdgeRef,EntityId,LocalTag}};
/// TruingCurveAlternatives::new(LocalTag::FIRST,vec![EdgeRange::whole(EdgeRef::new(EntityId::from_bits(2),LocalTag::FIRST))]);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TruingCurveAlternatives {
    tag: LocalTag,
    geometries: Vec<DirectedRange>,
}
impl TruingCurveAlternatives {
    /// Retain the original tag and every authored alternative,including repeated directed ranges.
    #[must_use]
    pub const fn new(tag: LocalTag, geometries: Vec<DirectedRange>) -> Self {
        Self { tag, geometries }
    }
    /// Actual original selected-output tag field.
    #[must_use]
    pub const fn tag(&self) -> &LocalTag {
        &self.tag
    }
    /// Every original geometry input in authored priority order;no default or deduplication.
    #[must_use]
    pub fn geometries(&self) -> &[DirectedRange] {
        &self.geometries
    }
}
/// Complete selected-output,raw-group,choice,combination and index/association allowances.
/// ```compile_fail
/// let limits:sc_core::construction::TruingCurveLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TruingCurveLimits {
    selected_outputs: usize,
    groups: usize,
    choices: usize,
    combinations: usize,
    workspace_entries: usize,
}
impl TruingCurveLimits {
    /// Choose complete N/M/K/P and checked2M additional logical-entry allowances.
    #[must_use]
    pub const fn new(
        selected_outputs: usize,
        groups: usize,
        choices: usize,
        combinations: usize,
        workspace_entries: usize,
    ) -> Self {
        Self {
            selected_outputs,
            groups,
            choices,
            combinations,
            workspace_entries,
        }
    }
    /// Every actual selected-domain output,excluding fixed outside fragments.
    #[must_use]
    pub const fn selected_outputs(self) -> usize {
        self.selected_outputs
    }
    /// Every original raw group,including duplicates and unknown tags.
    #[must_use]
    pub const fn groups(self) -> usize {
        self.groups
    }
    /// Every original directed curve-source occurrence,including repeated values.
    #[must_use]
    pub const fn choices(self) -> usize {
        self.choices
    }
    /// All declared complete combinations;not feasible results or execution cost.
    #[must_use]
    pub const fn combinations(self) -> usize {
        self.combinations
    }
    /// Additional group index/association logical entries,not bytes or earlier input storage.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// First whole resource,owner,complete-group or original field cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruingCurveRefusal {
    /// Complete choice sum cannot be represented.
    ChoiceCountOverflow,
    /// Complete nonempty Cartesian product cannot be represented.
    CombinationOverflow,
    /// Complete2M index/association population cannot be represented.
    WorkspaceOverflow,
    /// Whole selected output population exceeds policy.
    SelectedOutputs {
        /// Complete count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole raw group population exceeds policy.
    Groups {
        /// Complete count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole raw directed-choice population exceeds policy.
    Choices {
        /// Complete count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole raw Cartesian product exceeds policy.
    Combinations {
        /// Complete count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole new index/association population exceeds policy.
    Workspace {
        /// Complete count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Original declared operation differs from the actual bound truing operation.
    Operation,
    /// Exactly one group is required for every actual selected-domain output.
    Count {
        /// Complete actual selected output count.
        expected: usize,
        /// Complete raw group count.
        actual: usize,
    },
    /// Selected-output tag repeats in original group order.
    Duplicate {
        /// First original group.
        first: usize,
        /// Repeated original group.
        second: usize,
        /// Actual supplied tag.
        tag: LocalTag,
    },
    /// Tag names no actual selected-domain output;outside/catalogue roles do not substitute.
    UnknownSelectedOutput {
        /// Original group.
        index: usize,
        /// Actual supplied tag.
        tag: LocalTag,
    },
    /// An actual selected output has no explicit curve alternative;no geometry is inferred.
    EmptyGroup {
        /// Original group.
        index: usize,
        /// Actual selected tag.
        tag: LocalTag,
    },
    /// A geometry prerequisite is supplied by this same truing operation.
    SelfSource {
        /// Original group.
        group: usize,
        /// Original alternative within that group.
        choice: usize,
    },
}
/// Actual entire original proposal,lineage/source context,policy and first cause.
pub struct TruingCurveError<'a> {
    manifest: &'a TruingCurveManifest,
    lineage: &'a RequiredTruingLineage<'a>,
    limits: TruingCurveLimits,
    refusal: TruingCurveRefusal,
}
impl<'a> TruingCurveError<'a> {
    /// Actual whole original manifest,including every unaccepted group and choice.
    #[must_use]
    pub const fn manifest(&self) -> &'a TruingCurveManifest {
        self.manifest
    }
    /// Actual complete lineage/fold/contour/source parents.
    #[must_use]
    pub const fn lineage(&self) -> &'a RequiredTruingLineage<'a> {
        self.lineage
    }
    /// Original explicit complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> TruingCurveLimits {
        self.limits
    }
    /// First complete quota,owner,cardinality or original field cause.
    #[must_use]
    pub const fn refusal(&self) -> TruingCurveRefusal {
        self.refusal
    }
    /// Actual original self-supplied directed geometry field,or None for every other cause.
    /// The whole source remains borrowed in the manifest rather than copied into the refusal.
    #[must_use]
    pub fn refused_geometry(&self) -> Option<&'a DirectedRange> {
        match self.refusal {
            TruingCurveRefusal::SelfSource { group, choice } => self
                .manifest
                .groups
                .get(group)
                .and_then(|g| g.geometries.get(choice)),
            _ => None,
        }
    }
}
impl fmt::Debug for TruingCurveError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TruingCurveError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for TruingCurveError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "truing curve alternatives: {:?}", self.refusal)
    }
}
impl std::error::Error for TruingCurveError<'_> {}
/// Original operation,explicit selection rule and ordered selected-role curve groups.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TruingCurveManifest {
    operation: EntityId,
    policy: TruingCurveSelectionPolicy,
    groups: Vec<TruingCurveAlternatives>,
}
impl TruingCurveManifest {
    /// Retain every original field. This creates neither evaluated curves nor a selected assignment.
    #[must_use]
    pub const fn new(
        operation: EntityId,
        policy: TruingCurveSelectionPolicy,
        groups: Vec<TruingCurveAlternatives>,
    ) -> Self {
        Self {
            operation,
            policy,
            groups,
        }
    }
    /// Original declared operation identity.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Actual original explicit complete-assignment priority rule.
    #[must_use]
    pub const fn policy(&self) -> &TruingCurveSelectionPolicy {
        &self.policy
    }
    /// Every original group in authored priority order.
    #[must_use]
    pub fn groups(&self) -> &[TruingCurveAlternatives] {
        &self.groups
    }
    /// Bind one nonempty original group to every actual selected-domain output.
    /// Whole N then M quotas precede group-header scans. Checked K/P/2M and whole K/P/workspace
    /// quotas precede owner/cardinality,tag/choice inspection or allocation. P0 for no raw groups
    /// or any empty group;otherwise P is the complete product,not a truncatable prefix.
    /// Original group/choice order remains authoritative;outside fragments receive no new curves.
    /// O(M log(M+N+2)+K) work,at most2M additional logical entries;no fitting/provider calls.
    /// # Errors
    /// Whole N/M;checked K/P/2M and K/P/workspace quotas;operation,M=N;then original duplicate,
    /// unknown selected tag,empty group or first same-operation geometry prerequisite.
    pub fn admit<'a>(
        &'a self,
        lineage: &'a RequiredTruingLineage<'a>,
        limits: TruingCurveLimits,
    ) -> Result<AdmittedTruingCurveManifest<'a>, TruingCurveError<'a>> {
        use TruingCurveRefusal as R;
        let refuse = |refusal| TruingCurveError {
            manifest: self,
            lineage,
            limits,
            refusal,
        };
        let n = lineage
            .evidence()
            .locks()
            .outcome()
            .source_domain()
            .edge_count();
        let m = self.groups.len();
        if n > limits.selected_outputs {
            return Err(refuse(R::SelectedOutputs {
                required: n,
                allowed: limits.selected_outputs,
            }));
        }
        if m > limits.groups {
            return Err(refuse(R::Groups {
                required: m,
                allowed: limits.groups,
            }));
        }
        let mut choices = 0usize;
        let mut empty = false;
        for group in &self.groups {
            choices = choices
                .checked_add(group.geometries.len())
                .ok_or_else(|| refuse(R::ChoiceCountOverflow))?;
            empty |= group.geometries.is_empty();
        }
        let combinations = if m == 0 || empty {
            0
        } else {
            self.groups
                .iter()
                .try_fold(1usize, |p, g| p.checked_mul(g.geometries.len()))
                .ok_or_else(|| refuse(R::CombinationOverflow))?
        };
        let workspace = m
            .checked_mul(2)
            .ok_or_else(|| refuse(R::WorkspaceOverflow))?;
        if choices > limits.choices {
            return Err(refuse(R::Choices {
                required: choices,
                allowed: limits.choices,
            }));
        }
        if combinations > limits.combinations {
            return Err(refuse(R::Combinations {
                required: combinations,
                allowed: limits.combinations,
            }));
        }
        if workspace > limits.workspace_entries {
            return Err(refuse(R::Workspace {
                required: workspace,
                allowed: limits.workspace_entries,
            }));
        }
        let directions = lineage.fold().contours().directions();
        if self.operation != directions.manifest().operation() {
            return Err(refuse(R::Operation));
        }
        if m != n {
            return Err(refuse(R::Count {
                expected: n,
                actual: m,
            }));
        }
        let mut by_tag = BTreeMap::new();
        let mut groups = Vec::with_capacity(m);
        for (index, group) in self.groups.iter().enumerate() {
            let tag = group.tag;
            if let Some(first) = by_tag.insert(tag, index) {
                return Err(refuse(R::Duplicate {
                    first,
                    second: index,
                    tag,
                }));
            }
            let output = directions
                .assignment(tag)
                .filter(|o| matches!(o.source(), TruingDirectionSource::Selected(_)))
                .ok_or_else(|| refuse(R::UnknownSelectedOutput { index, tag }))?;
            if group.geometries.is_empty() {
                return Err(refuse(R::EmptyGroup { index, tag }));
            }
            for (choice, geometry) in group.geometries.iter().enumerate() {
                let edge = geometry.range.edge();
                if edge.creator() == self.operation {
                    return Err(refuse(R::SelfSource {
                        group: index,
                        choice,
                    }));
                }
            }
            groups.push(CurveGroupAssociation {
                index,
                proposal: group,
                output,
            });
        }
        Ok(AdmittedTruingCurveManifest {
            manifest: self,
            lineage,
            limits,
            choices,
            combinations,
            workspace,
            by_tag,
            groups,
        })
    }
}
#[derive(Debug)]
struct CurveGroupAssociation<'a> {
    index: usize,
    proposal: &'a TruingCurveAlternatives,
    output: TruingDirectionalOutput<'a>,
}
/// Authentic complete local priority/source proposal;not feasible geometry or a chosen assignment.
/// ```compile_fail
/// fn forge<'a>(mut curves:sc_core::construction::AdmittedTruingCurveManifest<'a>,
///     other:&'a sc_core::construction::RequiredTruingLineage<'a>) {curves.lineage=other;}
/// ```
/// ```compile_fail
/// fn escape<'a>(lineage:&'a sc_core::construction::RequiredTruingLineage<'a>)
///     ->sc_core::construction::AdmittedTruingCurveManifest<'a> {
///     use sc_core::{construction::*,ontology::EntityId};
///     let raw=TruingCurveManifest::new(EntityId::from_bits(0),TruingCurveSelectionPolicy::FirstFeasibleCompleteAssignment,vec![]);
///     raw.admit(lineage,TruingCurveLimits::new(0,0,0,0,0)).unwrap()
/// }
/// ```
pub struct AdmittedTruingCurveManifest<'a> {
    manifest: &'a TruingCurveManifest,
    lineage: &'a RequiredTruingLineage<'a>,
    limits: TruingCurveLimits,
    choices: usize,
    combinations: usize,
    workspace: usize,
    by_tag: BTreeMap<LocalTag, usize>,
    groups: Vec<CurveGroupAssociation<'a>>,
}
impl fmt::Debug for AdmittedTruingCurveManifest<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AdmittedTruingCurveManifest")
            .field("limits", &self.limits)
            .field("groups", &self.groups.len())
            .field("choices", &self.choices)
            .field("combinations", &self.combinations)
            .finish_non_exhaustive()
    }
}
impl<'a> AdmittedTruingCurveManifest<'a> {
    /// Actual complete original operation/policy/groups.
    #[must_use]
    pub const fn manifest(&self) -> &'a TruingCurveManifest {
        self.manifest
    }
    /// Actual complete lineage/fold/contour/source parents.
    #[must_use]
    pub const fn lineage(&self) -> &'a RequiredTruingLineage<'a> {
        self.lineage
    }
    /// Actual original explicit priority rule;not an executed choice.
    #[must_use]
    pub const fn policy(&self) -> &'a TruingCurveSelectionPolicy {
        self.manifest.policy()
    }
    /// Original whole N/M/K/P/workspace allowances.
    #[must_use]
    pub const fn limits(&self) -> TruingCurveLimits {
        self.limits
    }
    /// All original directed geometry-source occurrences,including equal repeated alternatives.
    #[must_use]
    pub const fn choice_count(&self) -> usize {
        self.choices
    }
    /// Complete declared Cartesian count,P0 for explicit empty preservation;not feasible results.
    #[must_use]
    pub const fn combination_count(&self) -> usize {
        self.combinations
    }
    /// Additional group index/association logical entries,not bytes or execution workspace.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace
    }
    /// Every authentic selected-role group in original priority order and reverse.
    pub fn groups(
        &self,
    ) -> impl ExactSizeIterator<Item = TruingCurveGroup<'_>> + DoubleEndedIterator {
        self.groups.iter().map(|association| TruingCurveGroup {
            context: self,
            association,
        })
    }
    /// Actual selected-role group,in O(log(M+2)) work;outside and unassigned metadata roles are absent.
    #[must_use]
    pub fn group(&self, tag: LocalTag) -> Option<TruingCurveGroup<'_>> {
        self.by_tag
            .get(&tag)
            .and_then(|index| self.groups.get(*index))
            .map(|association| TruingCurveGroup {
                context: self,
                association,
            })
    }
}
/// Authentic raw selected-role group and original priority position,not an independently bound curve.
/// ```compile_fail
/// fn forge<'a>(mut group:sc_core::construction::TruingCurveGroup<'a>,
///     other:&'a sc_core::construction::AdmittedTruingCurveManifest<'a>) {group.context=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct TruingCurveGroup<'a> {
    context: &'a AdmittedTruingCurveManifest<'a>,
    association: &'a CurveGroupAssociation<'a>,
}
impl<'a> TruingCurveGroup<'a> {
    /// Actual complete manifest/lineage owner.
    #[must_use]
    pub const fn context(self) -> &'a AdmittedTruingCurveManifest<'a> {
        self.context
    }
    /// Original authored group priority,not a persistent identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.association.index
    }
    /// Actual original entire tag/alternative group.
    #[must_use]
    pub const fn proposal(self) -> &'a TruingCurveAlternatives {
        self.association.proposal
    }
    /// Actual selected-domain output direction/source member,never an outside fragment.
    #[must_use]
    pub const fn output(self) -> TruingDirectionalOutput<'a> {
        self.association.output
    }
    /// All actual original choice fields in authored priority order and reverse;duplicates remain.
    pub fn alternatives(
        self,
    ) -> impl ExactSizeIterator<Item = TruingCurveAlternative<'a>> + DoubleEndedIterator {
        self.association
            .proposal
            .geometries
            .iter()
            .enumerate()
            .map(move |(index, geometry)| TruingCurveAlternative {
                group: self,
                index,
                geometry,
            })
    }
}
/// Authentic original directed curve-source field,not evaluated geometry or accepted correspondence.
/// ```compile_fail
/// fn forge<'a>(mut choice:sc_core::construction::TruingCurveAlternative<'a>,
///     other:&'a sc_core::ontology::DirectedRange) {choice.geometry=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct TruingCurveAlternative<'a> {
    group: TruingCurveGroup<'a>,
    index: usize,
    geometry: &'a DirectedRange,
}
impl<'a> TruingCurveAlternative<'a> {
    /// Actual complete group/manifest/lineage context.
    #[must_use]
    pub const fn group(self) -> TruingCurveGroup<'a> {
        self.group
    }
    /// Original priority within that group;no source identity is inferred from an ordinal.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }
    /// Actual original directed geometry-source field,including its distinct held frame/orientation.
    #[must_use]
    pub const fn geometry(self) -> &'a DirectedRange {
        self.geometry
    }
}
