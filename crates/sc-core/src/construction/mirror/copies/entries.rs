//! Explicit result-copy intents on actual inventory;no copy issuance,retirement or model mutation.
use super::BoundMirrorPieceCopyIdentities;
use crate::ontology::{CopyOrientation, CutCopy, CutCopyDefinition, EntityId, Mirroring};
use core::fmt;
use std::collections::BTreeMap;

/// Raw complete result-copy choices,not execution authority or an issued physical copy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPieceResultCopyIntent {
    /// Keep this actual existing result copy's original definition and identity.
    Retain(EntityId),
    /// Explicitly request retirement of this existing result copy;effects still require proof.
    Retire(EntityId),
    /// Request a new copy with an explicit canonical id,pattern Piece and physical orientation.
    Issue(CutCopyDefinition),
}
impl MirrorPieceResultCopyIntent {
    /// Actual explicit existing or proposed copy id,never a vector position.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        match self {
            Self::Retain(id) | Self::Retire(id) => *id,
            Self::Issue(definition) => definition.id,
        }
    }
}
/// Whole original operation and ordered copy choices,including invalid raw entries.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MirrorPieceResultCopyManifest {
    operation: EntityId,
    intents: Vec<MirrorPieceResultCopyIntent>,
}
impl MirrorPieceResultCopyManifest {
    /// Retain every choice without inferring preservation,issuing an id or dropping an old result.
    #[must_use]
    pub const fn new(operation: EntityId, intents: Vec<MirrorPieceResultCopyIntent>) -> Self {
        Self { operation, intents }
    }
    /// Actual original operation request.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Whole original ordered proposal,without normalization or prefix selection.
    #[must_use]
    pub fn intents(&self) -> &[MirrorPieceResultCopyIntent] {
        &self.intents
    }
}
/// Explicit whole intent/actual-plan/workspace allowances,without defaults.
/// ```compile_fail
/// let limits:sc_core::construction::MirrorPieceResultCopyLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MirrorPieceResultCopyLimits {
    intents: usize,
    plan_copies: usize,
    workspace_entries: usize,
}
impl MirrorPieceResultCopyLimits {
    /// Bound complete N proposal entries,C actual plan copies,and checked 2N index/association entries.
    #[must_use]
    pub const fn new(intents: usize, plan_copies: usize, workspace_entries: usize) -> Self {
        Self {
            intents,
            plan_copies,
            workspace_entries,
        }
    }
    /// Whole proposal allowance,including retirement requests.
    #[must_use]
    pub const fn intents(self) -> usize {
        self.intents
    }
    /// Whole actual plan allowance,including source and unrelated copies.
    #[must_use]
    pub const fn plan_copies(self) -> usize {
        self.plan_copies
    }
    /// Complete index/association entry allowance,not allocation bytes or prior index storage.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// First complete resource,operation,original intent or whole result-population cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPieceResultCopyRefusal {
    /// Complete 2N workspace count cannot be represented.
    WorkspaceOverflow,
    /// Whole original intent population exceeds policy.
    Intents {
        /// Complete actual entries.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole actual CutPlan population exceeds policy before any membership/coverage scan.
    PlanCopies {
        /// Complete actual copies.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole index/association population exceeds policy before allocation.
    Workspace {
        /// Complete required entries.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Requested operation differs from the actual mirror input.
    Operation,
    /// A copy id repeats across any choices,including Retire followed by Issue.
    DuplicateId {
        /// First original proposal position.
        first: usize,
        /// Repeated original position.
        second: usize,
        /// Actual repeated persistent id.
        id: EntityId,
    },
    /// Retain/Retire names no actual immutable CutCopy in this whole plan.
    MissingExisting {
        /// Original proposal position.
        index: usize,
        /// Actual original requested id.
        id: EntityId,
    },
    /// Source or unrelated copies cannot be retained,retired or retargeted as this result.
    ForeignExisting {
        /// Original proposal position.
        index: usize,
        /// Actual existing copy id.
        id: EntityId,
        /// Actual existing pattern Piece.
        piece: EntityId,
    },
    /// A proposed new copy names a different pattern Piece.
    IssuePiece {
        /// Original proposal position.
        index: usize,
        /// Actual canonical result Piece.
        expected: EntityId,
        /// Original proposed Piece.
        actual: EntityId,
    },
    /// A proposed issuance reuses a known copy,Piece,creating-operation or Material id.
    KnownIssueId {
        /// Original proposal position.
        index: usize,
        /// Actual conflicting proposed id.
        id: EntityId,
    },
    /// Active proposed reflection contradicts actual result Single/PairMember cut policy.
    UnexpectedReflection {
        /// Original proposal position.
        index: usize,
        /// Actual existing or proposed copy id.
        id: EntityId,
    },
    /// An actual old result copy has no explicit Retain/Retire choice.
    UnmentionedExisting {
        /// Original complete plan position.
        plan_index: usize,
        /// Actual unmentioned old result copy.
        id: EntityId,
    },
    /// Complete retained/new population differs from actual canonical result quantity.
    Quantity {
        /// Actual canonical result quantity.
        expected: u32,
        /// Complete proposed active population.
        actual: usize,
    },
    /// Actual result MirroredPairs policy requires equal proposed authored/reflected populations.
    UnbalancedPairs {
        /// Complete proposed authored copies.
        authored: usize,
        /// Complete proposed reflected copies.
        reflected: usize,
    },
}
/// Whole actual identity/inventory context,raw manifest,limits and original first cause survive refusal.
pub struct MirrorPieceResultCopyError<'a> {
    manifest: &'a MirrorPieceResultCopyManifest,
    identities: &'a BoundMirrorPieceCopyIdentities<'a>,
    limits: MirrorPieceResultCopyLimits,
    refusal: MirrorPieceResultCopyRefusal,
}
impl<'a> MirrorPieceResultCopyError<'a> {
    /// Actual whole original proposal,including every unaccepted intent.
    #[must_use]
    pub const fn manifest(&self) -> &'a MirrorPieceResultCopyManifest {
        self.manifest
    }
    /// Actual whole plan/source/contour/Piece/point and known identity context.
    #[must_use]
    pub const fn identities(&self) -> &'a BoundMirrorPieceCopyIdentities<'a> {
        self.identities
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> MirrorPieceResultCopyLimits {
        self.limits
    }
    /// First complete resource or original-field failure.
    #[must_use]
    pub const fn refusal(&self) -> MirrorPieceResultCopyRefusal {
        self.refusal
    }
}
impl fmt::Debug for MirrorPieceResultCopyError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MirrorPieceResultCopyError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for MirrorPieceResultCopyError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mirror result-copy proposal: {:?}", self.refusal)
    }
}
impl std::error::Error for MirrorPieceResultCopyError<'_> {}
impl MirrorPieceResultCopyManifest {
    /// Admit complete explicit choices against actual old result records and canonical result cut policy.
    /// Whole N/C/2N precede operation comparison,scans and allocation. Work is O((N+C)log(N+K+2))
    /// for K previously indexed known identities;new workspace is one N-entry id index and one
    /// N-entry association vector. Prior index/input storage retains its own limits. No copy is
    /// issued,retired,retargeted,physically transformed or published;these are metadata intents.
    /// # Errors
    /// Checked2N,N/C/workspace quotas,operation;then each original intent's duplicate id and kind/
    /// membership/identity/active-orientation checks. First unmentioned old result copy follows
    /// original plan order,then complete result quantity and paired-population balance.
    pub fn admit<'a>(
        &'a self,
        identities: &'a BoundMirrorPieceCopyIdentities<'a>,
        limits: MirrorPieceResultCopyLimits,
    ) -> Result<AdmittedMirrorPieceResultCopies<'a>, MirrorPieceResultCopyError<'a>> {
        use MirrorPieceResultCopyRefusal as R;
        let refuse = |refusal| MirrorPieceResultCopyError {
            manifest: self,
            identities,
            limits,
            refusal,
        };
        let n = self.intents.len();
        let c = identities.indexed_copy_count();
        let workspace_entries = n
            .checked_mul(2)
            .ok_or_else(|| refuse(R::WorkspaceOverflow))?;
        if n > limits.intents {
            return Err(refuse(R::Intents {
                required: n,
                allowed: limits.intents,
            }));
        }
        if c > limits.plan_copies {
            return Err(refuse(R::PlanCopies {
                required: c,
                allowed: limits.plan_copies,
            }));
        }
        if workspace_entries > limits.workspace_entries {
            return Err(refuse(R::Workspace {
                required: workspace_entries,
                allowed: limits.workspace_entries,
            }));
        }
        let identity = identities
            .source()
            .context()
            .contours()
            .coverage()
            .edges()
            .identity();
        let cut = identity.roles().metadata().cut();
        let piece = *identity.piece_id();
        if self.operation != cut.source().input().operation() {
            return Err(refuse(R::Operation));
        }
        let mut by_id = BTreeMap::new();
        let mut actions = Vec::with_capacity(n);
        let mut retention_count = 0usize;
        let mut retirement_count = 0usize;
        let mut issuance_count = 0usize;
        let mut authored = 0usize;
        let mut reflected = 0usize;
        for (index, intent) in self.intents.iter().enumerate() {
            let id = intent.id();
            if let Some(first) = by_id.insert(id, index) {
                return Err(refuse(R::DuplicateId {
                    first,
                    second: index,
                    id,
                }));
            }
            let source = match intent {
                MirrorPieceResultCopyIntent::Retain(id)
                | MirrorPieceResultCopyIntent::Retire(id) => {
                    let copy = identities
                        .copy(*id)
                        .ok_or_else(|| refuse(R::MissingExisting { index, id: *id }))?;
                    if copy.piece() != piece {
                        return Err(refuse(R::ForeignExisting {
                            index,
                            id: *id,
                            piece: copy.piece(),
                        }));
                    }
                    if matches!(intent, MirrorPieceResultCopyIntent::Retain(_)) {
                        retention_count += 1;
                        MirrorPieceResultCopySource::Retain(copy)
                    } else {
                        retirement_count += 1;
                        MirrorPieceResultCopySource::Retire(copy)
                    }
                }
                MirrorPieceResultCopyIntent::Issue(definition) => {
                    if definition.piece != piece {
                        return Err(refuse(R::IssuePiece {
                            index,
                            expected: piece,
                            actual: definition.piece,
                        }));
                    }
                    if identities.excludes_new_copy(id) {
                        return Err(refuse(R::KnownIssueId { index, id }));
                    }
                    issuance_count += 1;
                    MirrorPieceResultCopySource::Issue(definition)
                }
            };
            if !matches!(source, MirrorPieceResultCopySource::Retire(_)) {
                match source.definition().orientation {
                    CopyOrientation::Authored => authored += 1,
                    CopyOrientation::Reflected => {
                        if cut.mirroring() != &Mirroring::MirroredPairs {
                            return Err(refuse(R::UnexpectedReflection { index, id }));
                        }
                        reflected += 1;
                    }
                }
            }
            actions.push(Association {
                index,
                intent,
                source,
            });
        }
        for (plan_index, copy) in identities.source().plan().copies().iter().enumerate() {
            if copy.piece() == piece && !by_id.contains_key(&copy.id()) {
                return Err(refuse(R::UnmentionedExisting {
                    plan_index,
                    id: copy.id(),
                }));
            }
        }
        // These are disjoint subsets of the actual N intent entries,so all counts fit usize.
        let result_count = retention_count + issuance_count;
        if u32::try_from(result_count).ok() != Some(*cut.quantity()) {
            return Err(refuse(R::Quantity {
                expected: *cut.quantity(),
                actual: result_count,
            }));
        }
        if cut.mirroring() == &Mirroring::MirroredPairs && authored != reflected {
            return Err(refuse(R::UnbalancedPairs {
                authored,
                reflected,
            }));
        }
        Ok(AdmittedMirrorPieceResultCopies {
            manifest: self,
            identities,
            limits,
            workspace_entries,
            by_id,
            actions,
            retention_count,
            retirement_count,
            issuance_count,
            result_count,
            authored,
            reflected,
        })
    }
}
/// Borrowed payload of an authenticated intent view;the public enum alone carries no membership proof.
/// All variants describe proposed actions;none performs its named effect.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPieceResultCopySource<'a> {
    /// Actual existing result copy whose complete definition must remain unchanged.
    Retain(&'a CutCopy),
    /// Actual existing result copy named for explicit future retirement/effect reconciliation.
    Retire(&'a CutCopy),
    /// Actual original proposed new definition,not an issued immutable CutCopy.
    Issue(&'a CutCopyDefinition),
}
impl<'a> MirrorPieceResultCopySource<'a> {
    /// Actual original existing or proposed definition;retirement returns its original before-value.
    #[must_use]
    pub const fn definition(self) -> &'a CutCopyDefinition {
        match self {
            Self::Retain(copy) | Self::Retire(copy) => copy.definition(),
            Self::Issue(definition) => definition,
        }
    }
}
#[derive(Clone, Copy, Debug)]
struct Association<'a> {
    index: usize,
    intent: &'a MirrorPieceResultCopyIntent,
    source: MirrorPieceResultCopySource<'a>,
}

/// Whole native result-copy plan,not a CutPlan,global reservation or applied physical/semantic effect.
/// ```compile_fail
/// fn reparent<'a>(mut copies:sc_core::construction::AdmittedMirrorPieceResultCopies<'a>,
///     other:&'a sc_core::construction::MirrorPieceResultCopyManifest) {copies.manifest=other;}
/// ```
/// ```compile_fail
/// fn escape<'a>(identities:&'a sc_core::construction::BoundMirrorPieceCopyIdentities<'a>)
///     ->sc_core::construction::AdmittedMirrorPieceResultCopies<'a> {
///     use sc_core::{construction::*,ontology::EntityId};
///     let raw=MirrorPieceResultCopyManifest::new(EntityId::from_bits(10),vec![]);
///     raw.admit(identities,MirrorPieceResultCopyLimits::new(0,0,0)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct AdmittedMirrorPieceResultCopies<'a> {
    manifest: &'a MirrorPieceResultCopyManifest,
    identities: &'a BoundMirrorPieceCopyIdentities<'a>,
    limits: MirrorPieceResultCopyLimits,
    workspace_entries: usize,
    by_id: BTreeMap<EntityId, usize>,
    actions: Vec<Association<'a>>,
    retention_count: usize,
    retirement_count: usize,
    issuance_count: usize,
    result_count: usize,
    authored: usize,
    reflected: usize,
}
impl<'a> AdmittedMirrorPieceResultCopies<'a> {
    /// Actual complete original ordered intent proposal.
    #[must_use]
    pub const fn manifest(&self) -> &'a MirrorPieceResultCopyManifest {
        self.manifest
    }
    /// Actual whole source/inventory/known-id and original metadata context.
    #[must_use]
    pub const fn identities(&self) -> &'a BoundMirrorPieceCopyIdentities<'a> {
        self.identities
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> MirrorPieceResultCopyLimits {
        self.limits
    }
    /// Complete actual 2N index/association entries,not allocation bytes.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace_entries
    }
    /// Complete explicit retention-request count.
    #[must_use]
    pub const fn retention_count(&self) -> usize {
        self.retention_count
    }
    /// Complete explicit retirement-request count;no retirement is applied.
    #[must_use]
    pub const fn retirement_count(&self) -> usize {
        self.retirement_count
    }
    /// Complete explicit issuance-request count;no immutable copy is minted.
    #[must_use]
    pub const fn issuance_count(&self) -> usize {
        self.issuance_count
    }
    /// Complete retained/new result population,excluding explicit retirements.
    #[must_use]
    pub const fn result_count(&self) -> usize {
        self.result_count
    }
    /// Complete proposed active authored population.
    #[must_use]
    pub const fn authored_count(&self) -> usize {
        self.authored
    }
    /// Complete proposed active reflected population.
    #[must_use]
    pub const fn reflected_count(&self) -> usize {
        self.reflected
    }
    /// Actual authenticated intent by explicit persistent copy id.
    #[must_use]
    pub fn action(&self, id: EntityId) -> Option<MirrorPieceResultCopyAction<'_>> {
        self.by_id
            .get(&id)
            .and_then(|index| self.actions.get(*index))
            .map(|association| MirrorPieceResultCopyAction {
                owner: self,
                association,
            })
    }
    /// Every proposed action in original request order,including retirements and reverse iteration.
    pub fn actions(
        &self,
    ) -> impl ExactSizeIterator<Item = MirrorPieceResultCopyAction<'_>> + DoubleEndedIterator {
        self.actions
            .iter()
            .map(move |association| MirrorPieceResultCopyAction {
                owner: self,
                association,
            })
    }
    /// All proposed active result actions in original request order,without retired entries.
    pub fn result_actions(
        &self,
    ) -> impl DoubleEndedIterator<Item = MirrorPieceResultCopyAction<'_>> {
        self.actions()
            .filter(|action| !matches!(action.source(), MirrorPieceResultCopySource::Retire(_)))
    }
}
/// Authentic original intent association;its payload enum alone is not a plan-membership proof.
/// ```compile_fail
/// fn reparent<'a>(mut action:sc_core::construction::MirrorPieceResultCopyAction<'a>,
///     other:&'a sc_core::construction::AdmittedMirrorPieceResultCopies<'a>) {action.owner=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct MirrorPieceResultCopyAction<'a> {
    owner: &'a AdmittedMirrorPieceResultCopies<'a>,
    association: &'a Association<'a>,
}
impl<'a> MirrorPieceResultCopyAction<'a> {
    /// Actual complete native result-copy owner.
    #[must_use]
    pub const fn owner(self) -> &'a AdmittedMirrorPieceResultCopies<'a> {
        self.owner
    }
    /// Original request position,not a persistent identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.association.index
    }
    /// Actual original raw choice field.
    #[must_use]
    pub const fn intent(self) -> &'a MirrorPieceResultCopyIntent {
        self.association.intent
    }
    /// Actual associated existing copy or proposed definition,without performing any action.
    #[must_use]
    pub const fn source(self) -> MirrorPieceResultCopySource<'a> {
        self.association.source
    }
    /// Actual original before/proposed definition;Issue does not produce an immutable CutCopy.
    #[must_use]
    pub const fn definition(self) -> &'a CutCopyDefinition {
        self.association.source.definition()
    }
}
