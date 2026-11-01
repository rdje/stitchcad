//! Explicit retained-source reference to actual result occurrence assignments,not leftover coverage.
use super::{
    CoveredJoinConstructionGroups, JoinConstructionGroupOutput, JoinConstructionOutputGroup,
    JoinPlacedConstructionUse,
};
use crate::ontology::{EntityId, SeamSideId};
use core::fmt;
use std::collections::BTreeMap;

/// Explicit additional construction content policy;an evidence id is a request,not accepted truth.
/// ```compile_fail
/// let intent:sc_core::construction::JoinAdditionalConstructionIntent=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinAdditionalConstructionIntent {
    /// Every proposed occurrence must be assigned a retained source reference at the later whole scan.
    SourceMappedOnly,
    /// Additional proposed content is explicitly requested;actual justification remains mandatory.
    AllowAdditional {
        /// Original authored evidence source,no implied kind/state/currentness/approval.
        evidence_source: EntityId,
    },
}
/// One original retained-source selector and ordered actual proposed-use snapshot ordinals.
/// Ordinals disambiguate repeated occurrences;they are not persistent geometry identities.
/// ```compile_fail
/// use sc_core::{construction::*,ontology::*};
/// JoinConstructionTargetAssignment::new(SeamSideId::A,0,vec![LocalTag::FIRST]);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JoinConstructionTargetAssignment {
    side: SeamSideId,
    source_index: usize,
    uses: Vec<usize>,
}
impl JoinConstructionTargetAssignment {
    /// Preserve one target occurrence selector per original ordered retained reference.
    #[must_use]
    pub const fn new(side: SeamSideId, source_index: usize, uses: Vec<usize>) -> Self {
        Self {
            side,
            source_index,
            uses,
        }
    }
    /// Original explicit source A/B role.
    #[must_use]
    pub const fn side(&self) -> SeamSideId {
        self.side
    }
    /// Guarded original construction source-action selector.
    #[must_use]
    pub const fn source_index(&self) -> usize {
        self.source_index
    }
    /// Original ordered proposed-use snapshot selectors,including invalid/repeated values.
    #[must_use]
    pub fn uses(&self) -> &[usize] {
        &self.uses
    }
}
/// Complete raw group M,nested selector T,actual proposed U and peak M+2T entry allowances.
/// ```compile_fail
/// let limits:sc_core::construction::JoinConstructionTargetLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinConstructionTargetLimits {
    groups: usize,
    references: usize,
    uses: usize,
    workspace_entries: usize,
}
impl JoinConstructionTargetLimits {
    /// Admit all raw headers before child lengths,then complete T/U/checked M+2T before fields.
    /// Prior immutable source coverage/actions/group/candidate resource policies remain separate.
    #[must_use]
    pub const fn new(
        groups: usize,
        references: usize,
        uses: usize,
        workspace_entries: usize,
    ) -> Self {
        Self {
            groups,
            references,
            uses,
            workspace_entries,
        }
    }
    /// Complete raw target-assignment headers.
    #[must_use]
    pub const fn groups(self) -> usize {
        self.groups
    }
    /// Complete raw nested occurrence selectors.
    #[must_use]
    pub const fn references(self) -> usize {
        self.references
    }
    /// All actual proposed result occurrences,including unmapped additional content.
    #[must_use]
    pub const fn uses(self) -> usize {
        self.uses
    }
    /// Peak logical source-index/target-index/association entries,not allocator bytes.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// First complete arithmetic/quota,owner/count or original source/target assignment refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinConstructionTargetRefusal {
    /// Complete nested T cannot be represented.
    ReferenceOverflow,
    /// Complete M+2T cannot be represented.
    WorkspaceOverflow,
    /// Whole raw assignment headers exceed policy.
    Groups {
        /// Complete actual M.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole nested target selectors exceed policy.
    References {
        /// Complete actual T.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole actual proposed construction population exceeds policy.
    Uses {
        /// Complete actual U.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Peak complete M+2T entries exceed policy.
    Workspace {
        /// Complete required entries.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Raw operation differs from the authentic whole join context.
    Operation,
    /// Exactly one assignment per actual covered Retain group is required.
    Count {
        /// Complete actual Retain groups.
        expected: usize,
        /// Raw assignment headers.
        actual: usize,
    },
    /// Complete target selectors must equal complete covered retained references.
    ReferenceCount {
        /// Complete actual retained references.
        expected: usize,
        /// Raw nested target selectors.
        actual: usize,
    },
    /// A raw source selector repeats across assignment rows.
    DuplicateSource {
        /// First original row.
        first: usize,
        /// Repeated original row.
        second: usize,
    },
    /// Selector names no actual covered Retain group.
    MissingSource {
        /// Original row.
        group: usize,
    },
    /// One ordered target selector is required per reference in this actual source group.
    GroupReferenceCount {
        /// Original row.
        group: usize,
        /// Actual source group's references.
        expected: usize,
        /// Raw nested selectors.
        actual: usize,
    },
    /// Two retained references cannot consume one actual result occurrence.
    DuplicateUse {
        /// First original row.
        first_group: usize,
        /// First row's reference.
        first_index: usize,
        /// Repeated original row.
        group: usize,
        /// Repeated row's reference.
        index: usize,
    },
    /// Original target ordinal names no actual proposed construction occurrence.
    MissingUse {
        /// Original row.
        group: usize,
        /// Original reference position.
        index: usize,
    },
    /// Actual proposed occurrence has another persistent output tag.
    Output {
        /// Original row.
        group: usize,
        /// Original reference position.
        index: usize,
    },
    /// Actual proposed traversal differs from the original field/output orientation requirement.
    Traversal {
        /// Original row.
        group: usize,
        /// Original reference position.
        index: usize,
    },
}
/// Original proposal,whole covered source/use context,policy and first cause remain borrowed.
pub struct JoinConstructionTargetError<'a> {
    manifest: &'a JoinConstructionTargetManifest,
    coverage: &'a CoveredJoinConstructionGroups<'a>,
    limits: JoinConstructionTargetLimits,
    refusal: JoinConstructionTargetRefusal,
}
impl<'a> JoinConstructionTargetError<'a> {
    /// Actual complete original target assignment/intent proposal.
    #[must_use]
    pub const fn manifest(&self) -> &'a JoinConstructionTargetManifest {
        self.manifest
    }
    /// Actual whole covered sources and original proposed uses,without replacement context.
    #[must_use]
    pub const fn coverage(&self) -> &'a CoveredJoinConstructionGroups<'a> {
        self.coverage
    }
    /// Original explicit complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinConstructionTargetLimits {
        self.limits
    }
    /// First whole resource,owner/count or original field cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinConstructionTargetRefusal {
        self.refusal
    }
    /// Actual original refused assignment row;whole resource/operation/count causes return None.
    #[must_use]
    pub fn refused_assignment(&self) -> Option<&'a JoinConstructionTargetAssignment> {
        use JoinConstructionTargetRefusal as R;
        let group = match self.refusal {
            R::DuplicateSource { second, .. } => second,
            R::MissingSource { group }
            | R::GroupReferenceCount { group, .. }
            | R::DuplicateUse { group, .. }
            | R::MissingUse { group, .. }
            | R::Output { group, .. }
            | R::Traversal { group, .. } => group,
            _ => return None,
        };
        self.manifest.assignments.get(group)
    }
    fn reference_index(&self) -> Option<usize> {
        use JoinConstructionTargetRefusal as R;
        match self.refusal {
            R::DuplicateUse { index, .. }
            | R::MissingUse { index, .. }
            | R::Output { index, .. }
            | R::Traversal { index, .. } => Some(index),
            _ => None,
        }
    }
    /// Actual raw target occurrence ordinal only for a reference-specific cause.
    #[must_use]
    pub fn refused_use_index(&self) -> Option<&'a usize> {
        self.refused_assignment()?.uses.get(self.reference_index()?)
    }
    /// Actual known source group for a row cause,if that original selector exists.
    #[must_use]
    pub fn refused_source_group(&self) -> Option<JoinConstructionOutputGroup<'a>> {
        let row = self.refused_assignment()?;
        self.coverage.groups().group(row.side, row.source_index)
    }
    /// Actual original retained reference only for a reference-specific cause.
    #[must_use]
    pub fn refused_source_output(&self) -> Option<JoinConstructionGroupOutput<'a>> {
        self.refused_source_group()?
            .outputs()
            .nth(self.reference_index()?)
    }
    /// Actual proposed occurrence for duplicate/tag/traversal causes;missing-use/other causes None.
    #[must_use]
    pub fn refused_use(&self) -> Option<JoinPlacedConstructionUse<'a>> {
        use JoinConstructionTargetRefusal as R;
        if !matches!(
            self.refusal,
            R::DuplicateUse { .. } | R::Output { .. } | R::Traversal { .. }
        ) {
            return None;
        }
        self.coverage
            .groups()
            .sources()
            .uses()
            .uses()
            .nth(*self.refused_use_index()?)
    }
}
impl fmt::Debug for JoinConstructionTargetError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinConstructionTargetError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinConstructionTargetError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "join construction target assignments: {:?}",
            self.refusal
        )
    }
}
impl std::error::Error for JoinConstructionTargetError<'_> {}
/// Explicit operation,ordered retained-source target assignments and additional-content intent.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JoinConstructionTargetManifest {
    operation: EntityId,
    assignments: Vec<JoinConstructionTargetAssignment>,
    additional: JoinAdditionalConstructionIntent,
}
impl JoinConstructionTargetManifest {
    /// Retain every original selector/order/intent;no implicit additional-content policy.
    #[must_use]
    pub const fn new(
        operation: EntityId,
        assignments: Vec<JoinConstructionTargetAssignment>,
        additional: JoinAdditionalConstructionIntent,
    ) -> Self {
        Self {
            operation,
            assignments,
            additional,
        }
    }
    /// Original explicit operation.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Complete raw source-group/target occurrence assignments.
    #[must_use]
    pub fn assignments(&self) -> &[JoinConstructionTargetAssignment] {
        &self.assignments
    }
    /// Actual original additional-content intent;its request is not yet discharged here.
    #[must_use]
    pub const fn additional(&self) -> &JoinAdditionalConstructionIntent {
        &self.additional
    }
    /// Admit M raw headers before nested lengths,then checked T/M+2T and complete T/U/workspace
    /// quotas before owner/counts/fields/allocation. Require one unique actual covered Retain group
    /// per row and one target selector per ordered source reference. Each selected actual proposed
    /// occurrence must be globally distinct and match that source reference's persistent tag and
    /// required traversal. Repeated source fields require distinct actual result occurrences.
    /// Target ordinals are original snapshot selectors,not persistent identities;no list is reordered.
    /// Additional intent is retained only;complete leftover-use reconciliation is required next.
    /// Work O(M log(M+G+2)+T log(T+2)+1),peak M+2T entries for G actual covered groups;successful
    /// retained storage2T. Earlier immutable contexts/storage remain. No provider/evidence/geometry,
    /// currentness,omission justification or effect/identity issuance follows this native declaration.
    /// # Errors
    /// Whole arithmetic/resources,operation,group/reference counts,then first original row/reference
    /// source/count/duplicate/missing/tag/traversal cause. No accepted mapping prefix escapes.
    pub fn admit<'a>(
        &'a self,
        coverage: &'a CoveredJoinConstructionGroups<'a>,
        limits: JoinConstructionTargetLimits,
    ) -> Result<DeclaredJoinConstructionTargets<'a>, JoinConstructionTargetError<'a>> {
        use JoinConstructionTargetRefusal as R;
        let refuse = |refusal| JoinConstructionTargetError {
            manifest: self,
            coverage,
            limits,
            refusal,
        };
        let m = self.assignments.len();
        if m > limits.groups {
            return Err(refuse(R::Groups {
                required: m,
                allowed: limits.groups,
            }));
        }
        let t = self
            .assignments
            .iter()
            .map(|a| a.uses.len())
            .try_fold(0usize, usize::checked_add)
            .ok_or_else(|| refuse(R::ReferenceOverflow))?;
        let workspace = t
            .checked_mul(2)
            .and_then(|n| n.checked_add(m))
            .ok_or_else(|| refuse(R::WorkspaceOverflow))?;
        let uses = coverage.groups().sources().uses();
        let u = uses.use_count();
        if t > limits.references {
            return Err(refuse(R::References {
                required: t,
                allowed: limits.references,
            }));
        }
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
        if self.operation != uses.manifest().operation() {
            return Err(refuse(R::Operation));
        }
        if m != coverage.group_count() {
            return Err(refuse(R::Count {
                expected: coverage.group_count(),
                actual: m,
            }));
        }
        if t != coverage.reference_count() {
            return Err(refuse(R::ReferenceCount {
                expected: coverage.reference_count(),
                actual: t,
            }));
        }
        let mut by_source = BTreeMap::new();
        let mut by_use = BTreeMap::new();
        let mut mappings = Vec::with_capacity(t);
        for (group, row) in self.assignments.iter().enumerate() {
            let key = (row.side == SeamSideId::B, row.source_index);
            if let Some(first) = by_source.insert(key, group) {
                return Err(refuse(R::DuplicateSource {
                    first,
                    second: group,
                }));
            }
            let source_group = coverage
                .groups()
                .group(row.side, row.source_index)
                .ok_or_else(|| refuse(R::MissingSource { group }))?;
            let expected = source_group.definition().outputs().len();
            if row.uses.len() != expected {
                return Err(refuse(R::GroupReferenceCount {
                    group,
                    expected,
                    actual: row.uses.len(),
                }));
            }
            for ((index, use_index), source) in
                row.uses.iter().enumerate().zip(source_group.outputs())
            {
                if let Some((first_group, first_index, _)) =
                    by_use.insert(*use_index, (group, index, mappings.len()))
                {
                    return Err(refuse(R::DuplicateUse {
                        first_group,
                        first_index,
                        group,
                        index,
                    }));
                }
                let target = uses
                    .uses()
                    .nth(*use_index)
                    .ok_or_else(|| refuse(R::MissingUse { group, index }))?;
                if target.declared().tag() != source.tag() {
                    return Err(refuse(R::Output { group, index }));
                }
                if *target.direction() != source.required_traversal() {
                    return Err(refuse(R::Traversal { group, index }));
                }
                mappings.push(TargetAssociation {
                    group,
                    index,
                    row,
                    use_index,
                    source_group,
                    source,
                    target,
                });
            }
        }
        Ok(DeclaredJoinConstructionTargets {
            manifest: self,
            coverage,
            limits,
            workspace,
            by_use,
            mappings,
        })
    }
}
#[derive(Debug)]
struct TargetAssociation<'a> {
    group: usize,
    index: usize,
    row: &'a JoinConstructionTargetAssignment,
    use_index: &'a usize,
    source_group: JoinConstructionOutputGroup<'a>,
    source: JoinConstructionGroupOutput<'a>,
    target: JoinPlacedConstructionUse<'a>,
}
/// Authentic distinct actual target occurrence per retained reference;leftover uses remain unchecked.
/// ```compile_fail
/// fn forge<'a>(mut proof:sc_core::construction::DeclaredJoinConstructionTargets<'a>,other:&'a sc_core::construction::JoinConstructionTargetManifest) {proof.manifest=other;}
/// ```
/// ```compile_fail
/// fn escape(raw:sc_core::construction::JoinConstructionTargetManifest,coverage:sc_core::construction::CoveredJoinConstructionGroups<'_>)->sc_core::construction::DeclaredJoinConstructionTargets<'static> {
///     raw.admit(&coverage,sc_core::construction::JoinConstructionTargetLimits::new(0,0,0,0)).unwrap()
/// }
#[derive(Debug)]
pub struct DeclaredJoinConstructionTargets<'a> {
    manifest: &'a JoinConstructionTargetManifest,
    coverage: &'a CoveredJoinConstructionGroups<'a>,
    limits: JoinConstructionTargetLimits,
    workspace: usize,
    by_use: BTreeMap<usize, (usize, usize, usize)>,
    mappings: Vec<TargetAssociation<'a>>,
}
impl<'a> DeclaredJoinConstructionTargets<'a> {
    /// Actual complete original assignments and additional intent.
    #[must_use]
    pub const fn manifest(&self) -> &'a JoinConstructionTargetManifest {
        self.manifest
    }
    /// Actual complete retained source partitions and proposed-use context.
    #[must_use]
    pub const fn coverage(&self) -> &'a CoveredJoinConstructionGroups<'a> {
        self.coverage
    }
    /// Original complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinConstructionTargetLimits {
        self.limits
    }
    /// Peak M+2T logical entries,including transient source index;successful storage2T.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace
    }
    /// Complete retained-reference to distinct actual-use assignment count.
    #[must_use]
    pub fn mapping_count(&self) -> usize {
        self.mappings.len()
    }
    /// Every authentic mapping in original raw row/reference order,and reverse.
    pub fn mappings(
        &self,
    ) -> impl ExactSizeIterator<Item = JoinMappedConstructionTarget<'_>> + DoubleEndedIterator {
        self.mappings
            .iter()
            .map(move |association| JoinMappedConstructionTarget {
                context: self,
                association,
            })
    }
    /// Authentic assigned occurrence by actual proposed-use snapshot ordinal;unmapped/absent None.
    #[must_use]
    pub fn mapping_for_use(&self, index: usize) -> Option<JoinMappedConstructionTarget<'_>> {
        let (_, _, i) = self.by_use.get(&index)?;
        self.mappings
            .get(*i)
            .map(|association| JoinMappedConstructionTarget {
                context: self,
                association,
            })
    }
}
/// One actual original row/reference,retained source group/output and distinct result occurrence.
#[derive(Clone, Copy, Debug)]
pub struct JoinMappedConstructionTarget<'a> {
    context: &'a DeclaredJoinConstructionTargets<'a>,
    association: &'a TargetAssociation<'a>,
}
impl<'a> JoinMappedConstructionTarget<'a> {
    /// Actual whole mapping context.
    #[must_use]
    pub const fn context(self) -> &'a DeclaredJoinConstructionTargets<'a> {
        self.context
    }
    /// Original raw row ordinal.
    #[must_use]
    pub const fn group_index(self) -> usize {
        self.association.group
    }
    /// Original reference position in its actual source group and raw target row.
    #[must_use]
    pub const fn index(self) -> usize {
        self.association.index
    }
    /// Actual raw original source/occurrence assignment row.
    #[must_use]
    pub const fn assignment(self) -> &'a JoinConstructionTargetAssignment {
        self.association.row
    }
    /// Actual raw original result occurrence ordinal field,not a persistent identity.
    #[must_use]
    pub const fn use_index(self) -> &'a usize {
        self.association.use_index
    }
    /// Authentic complete retained source group.
    #[must_use]
    pub const fn source_group(self) -> JoinConstructionOutputGroup<'a> {
        self.association.source_group
    }
    /// Authentic original retained source reference/range/direction requirement.
    #[must_use]
    pub const fn source(self) -> JoinConstructionGroupOutput<'a> {
        self.association.source
    }
    /// Authentic actual proposed occurrence with its original tag/traversal fields.
    #[must_use]
    pub const fn target(self) -> JoinPlacedConstructionUse<'a> {
        self.association.target
    }
}
