//! Complete actual candidate CutPlan association with canonical inventory and native copy choices.
use super::{BoundMirrorPieceCopyInventory, MirrorPieceResultCopySource};
use crate::ontology::{CutCopy, CutPlan, CutPlanError, EntityId};
use core::fmt;
use std::collections::BTreeMap;

/// Explicit complete before/after copy,inventory,intent and workspace-entry allowances.
/// ```compile_fail
/// let limits:sc_core::construction::MirrorPieceCopyPlanLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MirrorPieceCopyPlanLimits {
    before_copies: usize,
    after_copies: usize,
    pieces: usize,
    intents: usize,
    workspace_entries: usize,
}
impl MirrorPieceCopyPlanLimits {
    /// Select whole B/C/I/N and checked I+3C allowances. Entries are not allocation bytes.
    #[must_use]
    pub const fn new(
        before_copies: usize,
        after_copies: usize,
        pieces: usize,
        intents: usize,
        workspace_entries: usize,
    ) -> Self {
        Self {
            before_copies,
            after_copies,
            pieces,
            intents,
            workspace_entries,
        }
    }
    /// Whole original plan,including old result and unrelated copies.
    #[must_use]
    pub const fn before_copies(self) -> usize {
        self.before_copies
    }
    /// Whole actual candidate plan,including unrelated copies.
    #[must_use]
    pub const fn after_copies(self) -> usize {
        self.after_copies
    }
    /// Whole actual candidate inventory.
    #[must_use]
    pub const fn pieces(self) -> usize {
        self.pieces
    }
    /// Whole original native intent population,including retirements.
    #[must_use]
    pub const fn intents(self) -> usize {
        self.intents
    }
    /// Peak logical entry allowance for temporary canonical validation or candidate index.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// Whole resource,canonical association or explicit original-definition refusal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MirrorPieceCopyPlanRefusal {
    /// Checked I+3C cannot be represented,before scans or allocation.
    WorkspaceOverflow,
    /// Whole original plan exceeds policy.
    BeforeCopies {
        /// Complete actual count.
        required: usize,
        /// Caller allowance.
        allowed: usize,
    },
    /// Whole candidate plan exceeds policy.
    AfterCopies {
        /// Complete actual count.
        required: usize,
        /// Caller allowance.
        allowed: usize,
    },
    /// Whole candidate inventory exceeds policy.
    Pieces {
        /// Complete actual count.
        required: usize,
        /// Caller allowance.
        allowed: usize,
    },
    /// Whole native intent collection exceeds policy.
    Intents {
        /// Complete actual count.
        required: usize,
        /// Caller allowance.
        allowed: usize,
    },
    /// Full temporary canonical validation/index workspace exceeds policy.
    Workspace {
        /// Checked complete upper bound.
        required: usize,
        /// Caller allowance.
        allowed: usize,
    },
    /// Candidate copies do not form a canonical plan against this actual inventory.
    Inventory(CutPlanError),
    /// Actual result copy has no active native proposal.
    UnexpectedResult {
        /// Original candidate record position.
        index: usize,
        /// Actual copy identity.
        id: EntityId,
    },
    /// Native retirement is still present in the candidate.
    RetiredPresent {
        /// Original candidate record position.
        index: usize,
        /// Actual copy identity.
        id: EntityId,
    },
    /// Actual result definition differs from its complete retained/proposed definition.
    ResultChanged {
        /// Original candidate record position.
        index: usize,
        /// Actual copy identity.
        id: EntityId,
    },
    /// A non-result candidate copy has no original non-result definition.
    UnexpectedOther {
        /// Original candidate record position.
        index: usize,
        /// Actual copy identity.
        id: EntityId,
    },
    /// Source/unrelated copy changed an original field.
    PreservedChanged {
        /// Original candidate record position.
        index: usize,
        /// Actual copy identity.
        id: EntityId,
    },
    /// First original source/unrelated copy is absent from the complete candidate.
    MissingPreserved {
        /// Original before-plan record position.
        index: usize,
        /// Required copy identity.
        id: EntityId,
    },
    /// First active native result choice is absent from the complete candidate.
    MissingResult {
        /// Original proposal position.
        index: usize,
        /// Required copy identity.
        id: EntityId,
    },
}
/// Whole actual inventory/native proposal,candidate plan,policy and first refusal.
pub struct MirrorPieceCopyPlanError<'a> {
    inventory: &'a BoundMirrorPieceCopyInventory<'a>,
    plan: &'a CutPlan,
    limits: &'a MirrorPieceCopyPlanLimits,
    refusal: MirrorPieceCopyPlanRefusal,
}
impl<'a> MirrorPieceCopyPlanError<'a> {
    /// Actual whole candidate inventory and native copy context.
    #[must_use]
    pub const fn inventory(&self) -> &'a BoundMirrorPieceCopyInventory<'a> {
        self.inventory
    }
    /// Actual complete candidate CutPlan,not a reconstructed success prefix.
    #[must_use]
    pub const fn plan(&self) -> &'a CutPlan {
        self.plan
    }
    /// Original whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> &'a MirrorPieceCopyPlanLimits {
        self.limits
    }
    /// First full-resource,canonical or original-definition refusal.
    #[must_use]
    pub const fn refusal(&self) -> &MirrorPieceCopyPlanRefusal {
        &self.refusal
    }
}
impl fmt::Debug for MirrorPieceCopyPlanError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MirrorPieceCopyPlanError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for MirrorPieceCopyPlanError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mirror candidate cut plan: {:?}", self.refusal)
    }
}
impl std::error::Error for MirrorPieceCopyPlanError<'_> {}
impl<'s> BoundMirrorPieceCopyInventory<'s> {
    /// Associate all actual candidate copies against the actual inventory and complete native choices.
    /// Checked I+3C precedes B/C/I/N/workspace quotas,then canonical CutPlan validation against this
    /// inventory. That transient proof is dropped before indexing authentic candidate borrows. Every
    /// candidate definition is checked in authored order,then preserved before copies and active native
    /// results are checked for completeness in their original orders. No candidate prefix escapes.
    /// Canonical validation preserves its own error ordering. It certifies no global identity issuance,
    /// currentness,unrelated Piece-content authorization,physical pairing,geometry or publication.
    /// # Errors
    /// Whole resource failure;canonical inventory association;then changed/extra/missing copies.
    pub fn bind_cut_plan<'a>(
        &'a self,
        plan: &'a CutPlan,
        limits: &'a MirrorPieceCopyPlanLimits,
    ) -> Result<BoundMirrorPieceCopyPlan<'a>, MirrorPieceCopyPlanError<'a>>
    where
        's: 'a,
    {
        use MirrorPieceCopyPlanRefusal as R;
        let refuse = |refusal| MirrorPieceCopyPlanError {
            inventory: self,
            plan,
            limits,
            refusal,
        };
        let copies = self.copies();
        let before = copies.identities().source().plan();
        let b = before.copies().len();
        let c = plan.copies().len();
        let i = self.inventory().pieces().len();
        let n = copies.manifest().intents().len();
        let workspace = c
            .checked_mul(3)
            .and_then(|count| count.checked_add(i))
            .ok_or_else(|| refuse(R::WorkspaceOverflow))?;
        if b > limits.before_copies {
            return Err(refuse(R::BeforeCopies {
                required: b,
                allowed: limits.before_copies,
            }));
        }
        if c > limits.after_copies {
            return Err(refuse(R::AfterCopies {
                required: c,
                allowed: limits.after_copies,
            }));
        }
        if i > limits.pieces {
            return Err(refuse(R::Pieces {
                required: i,
                allowed: limits.pieces,
            }));
        }
        if n > limits.intents {
            return Err(refuse(R::Intents {
                required: n,
                allowed: limits.intents,
            }));
        }
        if workspace > limits.workspace_entries {
            return Err(refuse(R::Workspace {
                required: workspace,
                allowed: limits.workspace_entries,
            }));
        }
        // Reuse the canonical model contract;the immutable plan does not retain its birth Pieces.
        // The temporary proof is neither the returned plan nor a new issuance/publication effect.
        let definitions = plan
            .copies()
            .iter()
            .map(|copy| *copy.definition())
            .collect();
        drop(
            CutPlan::new(definitions, self.inventory().pieces())
                .map_err(|error| refuse(R::Inventory(error)))?,
        );
        let mut index = BTreeMap::new();
        let result = self.result_piece().id();
        for (position, copy) in plan.copies().iter().enumerate() {
            index.insert(copy.id(), copy);
            if copy.piece() == result {
                let action = copies.action(copy.id()).ok_or_else(|| {
                    refuse(R::UnexpectedResult {
                        index: position,
                        id: copy.id(),
                    })
                })?;
                if matches!(action.source(), MirrorPieceResultCopySource::Retire(_)) {
                    return Err(refuse(R::RetiredPresent {
                        index: position,
                        id: copy.id(),
                    }));
                }
                if copy.definition() != action.definition() {
                    return Err(refuse(R::ResultChanged {
                        index: position,
                        id: copy.id(),
                    }));
                }
            } else {
                let old = copies
                    .identities()
                    .copy(copy.id())
                    .filter(|old| old.piece() != result)
                    .ok_or_else(|| {
                        refuse(R::UnexpectedOther {
                            index: position,
                            id: copy.id(),
                        })
                    })?;
                if copy.definition() != old.definition() {
                    return Err(refuse(R::PreservedChanged {
                        index: position,
                        id: copy.id(),
                    }));
                }
            }
        }
        for (position, copy) in before.copies().iter().enumerate() {
            if copy.piece() != result && !index.contains_key(&copy.id()) {
                return Err(refuse(R::MissingPreserved {
                    index: position,
                    id: copy.id(),
                }));
            }
        }
        for action in copies.result_actions() {
            if !index.contains_key(&action.definition().id) {
                return Err(refuse(R::MissingResult {
                    index: action.index(),
                    id: action.definition().id,
                }));
            }
        }
        Ok(BoundMirrorPieceCopyPlan {
            inventory: self,
            plan,
            limits,
            index,
            workspace,
        })
    }
}
/// Authentic complete actual candidate association;metadata proof grants no applied effect.
/// ```compile_fail
/// fn reparent<'a>(mut bound:sc_core::construction::BoundMirrorPieceCopyPlan<'a>,
///     other:&'a sc_core::ontology::CutPlan) {bound.plan=other;}
/// ```
/// ```compile_fail
/// fn escape<'a>(inventory:&'a sc_core::construction::BoundMirrorPieceCopyInventory<'a>)
///     ->sc_core::construction::BoundMirrorPieceCopyPlan<'a> {
///     use sc_core::{construction::*,ontology::CutPlan};
///     let plan=CutPlan::new(vec![],&[]).unwrap();let limits=MirrorPieceCopyPlanLimits::new(0,0,0,0,0);
///     inventory.bind_cut_plan(&plan,&limits).unwrap()
/// }
/// ```
#[derive(Clone, Debug)]
pub struct BoundMirrorPieceCopyPlan<'a> {
    inventory: &'a BoundMirrorPieceCopyInventory<'a>,
    plan: &'a CutPlan,
    limits: &'a MirrorPieceCopyPlanLimits,
    index: BTreeMap<EntityId, &'a CutCopy>,
    workspace: usize,
}
impl<'a> BoundMirrorPieceCopyPlan<'a> {
    /// Actual complete candidate inventory/native copy context.
    #[must_use]
    pub const fn inventory(&self) -> &'a BoundMirrorPieceCopyInventory<'a> {
        self.inventory
    }
    /// Actual complete candidate plan,in authored order.
    #[must_use]
    pub const fn plan(&self) -> &'a CutPlan {
        self.plan
    }
    /// Original explicit caller-owned whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> &'a MirrorPieceCopyPlanLimits {
        self.limits
    }
    /// Checked I+3C transient logical workspace upper bound,not bytes or whole storage.
    #[must_use]
    pub const fn workspace_bound(&self) -> usize {
        self.workspace
    }
    /// Actual candidate record selected by persistent id,in O(log(C+2)) work.
    #[must_use]
    pub fn copy(&self, id: EntityId) -> Option<&'a CutCopy> {
        self.index.get(&id).copied()
    }
}
