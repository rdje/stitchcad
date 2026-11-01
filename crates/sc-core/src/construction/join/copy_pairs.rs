//! Exact native A+B/result physical-copy bookkeeping,not actual per-pair geometry or effects.
use super::{DeclaredJoinCopySources, JoinCopySourceAction, JoinResultCopyRequest};
use crate::ontology::{CutCopy, EntityId, SeamSideId};
use core::fmt;
use std::collections::BTreeMap;

/// Complete original C,result Q and checked3Q peak new entry allowances.
/// ```compile_fail
/// let limits:sc_core::construction::JoinCopyPairLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinCopyPairLimits {
    copies: usize,
    results: usize,
    workspace_entries: usize,
}
impl JoinCopyPairLimits {
    /// Admit all original copy actions and requested results before selected-copy fields or scans.
    /// Peak3Q counts result index,staging slots and final associations;successful storage2Q.
    #[must_use]
    pub const fn new(copies: usize, results: usize, workspace_entries: usize) -> Self {
        Self {
            copies,
            results,
            workspace_entries,
        }
    }
    /// Every actual original physical copy/action,including retained/unrelated members.
    #[must_use]
    pub const fn copies(self) -> usize {
        self.copies
    }
    /// Every actual requested result copy.
    #[must_use]
    pub const fn results(self) -> usize {
        self.results
    }
    /// Complete peak3Q logical new entries,not allocator bytes or earlier immutable storage.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// First complete resource,selected-copy or original source/result-group cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinCopyPairRefusal {
    /// Checked complete3Q cannot be represented.
    WorkspaceOverflow,
    /// Complete actual original copy population exceeds policy.
    Copies {
        /// Actual whole C.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete actual requested result population exceeds policy.
    Results {
        /// Actual whole Q.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete peak3Q entries exceed policy.
    Workspace {
        /// Whole required entries.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Actual selected A/B physical copy cannot be retained by an executed join.
    SelectedRetained {
        /// Actual original seam side,checked A before B.
        side: SeamSideId,
    },
    /// Actual selected A/B actions request different result copies;they cannot be silently paired.
    SelectedResultMismatch,
    /// Two actual source copies of the same A/B Piece target one result.
    DuplicateSide {
        /// Actual explicitly requested result identity.
        result_copy: EntityId,
        /// Actual repeated source role.
        side: SeamSideId,
        /// First original action row.
        first: usize,
        /// Repeated original action row.
        second: usize,
    },
    /// An actual result lacks an original A/B source;requested result order,A before B.
    MissingSide {
        /// Actual explicitly requested result identity.
        result_copy: EntityId,
        /// Missing actual source role.
        side: SeamSideId,
    },
    /// Internal fault:closed original-copy coverage lost an actual selected source.
    SourceInvariant {
        /// Actual selected original role.
        side: SeamSideId,
    },
    /// Internal fault:closed actual result membership/ownership no longer matches its staging index.
    ResultInvariant {
        /// Original source action row,if a row exposed the inconsistency.
        source: Option<usize>,
    },
}
fn selected_copy<'a>(sources: &'a DeclaredJoinCopySources<'a>, side: SeamSideId) -> &'a CutCopy {
    sources
        .results()
        .folds()
        .fold()
        .construction()
        .targets()
        .coverage()
        .groups()
        .sources()
        .uses()
        .connectivity()
        .contours()
        .directions()
        .coverage()
        .edges()
        .identity()
        .roles()
        .banks()
        .sources()
        .population()
        .metadata()
        .splice()
        .source()
        .side(side)
        .copy()
}
fn selected_action<'a>(
    sources: &'a DeclaredJoinCopySources<'a>,
    side: SeamSideId,
) -> Option<JoinCopySourceAction<'a>> {
    sources.assignment(selected_copy(sources, side).id())
}
/// Whole original source/result/selected context,policy and first cause survive refusal.
pub struct JoinCopyPairError<'a> {
    sources: &'a DeclaredJoinCopySources<'a>,
    limits: JoinCopyPairLimits,
    refusal: JoinCopyPairRefusal,
}
impl<'a> JoinCopyPairError<'a> {
    /// Actual complete original copy actions/result/fold/construction/inventory parents.
    #[must_use]
    pub const fn sources(&self) -> &'a DeclaredJoinCopySources<'a> {
        self.sources
    }
    /// Original complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinCopyPairLimits {
        self.limits
    }
    /// First whole resource,selected correspondence or original group cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinCopyPairRefusal {
        self.refusal
    }
    /// Actual original selected source action for selected-retention/mismatch causes;other causes None.
    #[must_use]
    pub fn selected_source(&self, side: SeamSideId) -> Option<JoinCopySourceAction<'a>> {
        if !matches!(
            self.refusal,
            JoinCopyPairRefusal::SelectedRetained { .. }
                | JoinCopyPairRefusal::SelectedResultMismatch
        ) {
            return None;
        }
        selected_action(self.sources, side)
    }
    /// Actual selected retained or second duplicate source action;no fabricated missing-side source.
    #[must_use]
    pub fn refused_source(&self) -> Option<JoinCopySourceAction<'a>> {
        match self.refusal {
            JoinCopyPairRefusal::SelectedRetained { side } => selected_action(self.sources, side),
            JoinCopyPairRefusal::DuplicateSide { second, .. } => {
                self.sources.assignments().nth(second)
            }
            JoinCopyPairRefusal::ResultInvariant {
                source: Some(index),
            } => self.sources.assignments().nth(index),
            _ => None,
        }
    }
    /// Actual result request for duplicate/missing-side causes;whole/selected causes None.
    #[must_use]
    pub fn refused_result(&self) -> Option<JoinResultCopyRequest<'a>> {
        let id = match self.refusal {
            JoinCopyPairRefusal::DuplicateSide { result_copy, .. }
            | JoinCopyPairRefusal::MissingSide { result_copy, .. } => result_copy,
            _ => return None,
        };
        self.sources.results().copy(id)
    }
}
impl fmt::Debug for JoinCopyPairError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinCopyPairError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinCopyPairError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join exact copy correspondence: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinCopyPairError<'_> {}
#[derive(Debug)]
struct PairSlot<'a> {
    result: JoinResultCopyRequest<'a>,
    a: Option<JoinCopySourceAction<'a>>,
    b: Option<JoinCopySourceAction<'a>>,
}
#[derive(Debug)]
struct PairAssociation<'a> {
    result: JoinResultCopyRequest<'a>,
    a: JoinCopySourceAction<'a>,
    b: JoinCopySourceAction<'a>,
}
impl<'a> DeclaredJoinCopySources<'a> {
    /// Require actual selected A/B to JoinTo the same result,then exactly one actual A+B source per
    /// explicitly requested result. Whole C/Q/checked3Q precede selected fields,scans and allocation.
    /// Complete unique original source coverage already prevents one physical copy from appearing
    /// twice;this stage prevents multiple A/B contributors or missing original material per result.
    /// Retained/unrelated copies stay on the original parent. Source/result/row order stays authored.
    /// Work O(C log(C+Q+2)+Q log(Q+2)+1),peak3Q new entries,successful2Q;earlier immutable
    /// contexts/storage remain separate. No provider,geometry,evidence,copy transform,namespace
    /// currentness,issuance,source retirement or atomic effect is certified. Every actual pair still
    /// requires its own bank/pose/ease/stop/consumer checks;selected-span evidence cannot certify
    /// another physical pair. Complete candidate inventory/CutPlan/mirroring also remains required.
    /// # Errors
    /// Whole C/Q,3Q arithmetic/quota;selected A then B retention,selected result mismatch;first
    /// original duplicate source side;requested result order's missing A then B. Invariant causes
    /// are internal faults to diagnose,not user incompatibility;no accepted pairing prefix escapes.
    pub fn require_pairs(
        &'a self,
        limits: JoinCopyPairLimits,
    ) -> Result<RequiredJoinCopyCorrespondence<'a>, JoinCopyPairError<'a>> {
        use JoinCopyPairRefusal as R;
        let refuse = |refusal| JoinCopyPairError {
            sources: self,
            limits,
            refusal,
        };
        let c = self.copy_count();
        let q = self.results().copy_count();
        if c > limits.copies {
            return Err(refuse(R::Copies {
                required: c,
                allowed: limits.copies,
            }));
        }
        if q > limits.results {
            return Err(refuse(R::Results {
                required: q,
                allowed: limits.results,
            }));
        }
        let workspace = q
            .checked_mul(3)
            .ok_or_else(|| refuse(R::WorkspaceOverflow))?;
        if workspace > limits.workspace_entries {
            return Err(refuse(R::Workspace {
                required: workspace,
                allowed: limits.workspace_entries,
            }));
        }
        let a = selected_action(self, SeamSideId::A).ok_or_else(|| {
            refuse(R::SourceInvariant {
                side: SeamSideId::A,
            })
        })?;
        let selected = a.result().ok_or_else(|| {
            refuse(R::SelectedRetained {
                side: SeamSideId::A,
            })
        })?;
        let b = selected_action(self, SeamSideId::B).ok_or_else(|| {
            refuse(R::SourceInvariant {
                side: SeamSideId::B,
            })
        })?;
        let other = b.result().ok_or_else(|| {
            refuse(R::SelectedRetained {
                side: SeamSideId::B,
            })
        })?;
        if selected.id() != other.id() {
            return Err(refuse(R::SelectedResultMismatch));
        }
        let mut by_result = BTreeMap::new();
        let mut slots = Vec::with_capacity(q);
        for result in self.results().copies() {
            by_result.insert(result.id(), slots.len());
            slots.push(PairSlot {
                result,
                a: None,
                b: None,
            });
        }
        let mut consumed = 0;
        for source in self.assignments() {
            let Some(result) = source.result() else {
                continue;
            };
            let index = *by_result.get(&result.id()).ok_or_else(|| {
                refuse(R::ResultInvariant {
                    source: Some(source.index()),
                })
            })?;
            let slot = slots.get_mut(index).ok_or_else(|| {
                refuse(R::ResultInvariant {
                    source: Some(source.index()),
                })
            })?;
            let side = source.side().ok_or_else(|| {
                refuse(R::ResultInvariant {
                    source: Some(source.index()),
                })
            })?;
            let held = match side {
                SeamSideId::A => &mut slot.a,
                SeamSideId::B => &mut slot.b,
            };
            if let Some(first) = held {
                return Err(refuse(R::DuplicateSide {
                    result_copy: result.id(),
                    side,
                    first: first.index(),
                    second: source.index(),
                }));
            }
            *held = Some(source);
            consumed += 1; // At most C unique original actions,already admitted.
        }
        let mut pairs = Vec::with_capacity(q);
        for slot in slots {
            let a = slot.a.ok_or_else(|| {
                refuse(R::MissingSide {
                    result_copy: slot.result.id(),
                    side: SeamSideId::A,
                })
            })?;
            let b = slot.b.ok_or_else(|| {
                refuse(R::MissingSide {
                    result_copy: slot.result.id(),
                    side: SeamSideId::B,
                })
            })?;
            pairs.push(PairAssociation {
                result: slot.result,
                a,
                b,
            });
        }
        Ok(RequiredJoinCopyCorrespondence {
            sources: self,
            limits,
            workspace,
            selected,
            consumed,
            by_result,
            pairs,
        })
    }
}
/// Exact complete native physical-copy pairs;canonical inventory and all physical/effect truth pending.
/// ```compile_fail
/// fn forge<'a>(mut pairs:sc_core::construction::RequiredJoinCopyCorrespondence<'a>,other:&'a sc_core::construction::DeclaredJoinCopySources<'a>) {pairs.sources=other;}
/// ```
/// ```compile_fail
/// fn escape(sources:sc_core::construction::DeclaredJoinCopySources<'_>)->sc_core::construction::RequiredJoinCopyCorrespondence<'static> {
///     sources.require_pairs(sc_core::construction::JoinCopyPairLimits::new(0,0,0)).unwrap()
/// }
#[derive(Debug)]
pub struct RequiredJoinCopyCorrespondence<'a> {
    sources: &'a DeclaredJoinCopySources<'a>,
    limits: JoinCopyPairLimits,
    workspace: usize,
    selected: JoinResultCopyRequest<'a>,
    consumed: usize,
    by_result: BTreeMap<EntityId, usize>,
    pairs: Vec<PairAssociation<'a>>,
}
impl<'a> RequiredJoinCopyCorrespondence<'a> {
    /// Actual whole original source/action/result/fold/construction/inventory parents.
    #[must_use]
    pub const fn sources(&self) -> &'a DeclaredJoinCopySources<'a> {
        self.sources
    }
    /// Original complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinCopyPairLimits {
        self.limits
    }
    /// Peak3Q index/staging/final association entries,not bytes;successful retained storage2Q.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace
    }
    /// Actual requested result shared by the original selected A/B physical copies,unissued.
    #[must_use]
    pub const fn selected_result(&self) -> JoinResultCopyRequest<'a> {
        self.selected
    }
    /// Complete requested result pair population.
    #[must_use]
    pub fn pair_count(&self) -> usize {
        self.pairs.len()
    }
    /// Complete unique original copies contributing to pairs;two per actual result.
    #[must_use]
    pub const fn consumed_copy_count(&self) -> usize {
        self.consumed
    }
    /// Authentic complete A+B pairs in original requested-result order,and reverse.
    pub fn pairs(&self) -> impl ExactSizeIterator<Item = JoinCopyPair<'_>> + DoubleEndedIterator {
        self.pairs.iter().map(move |association| JoinCopyPair {
            context: self,
            association,
        })
    }
    /// Authentic actual result pair by its stable explicitly requested copy id.
    #[must_use]
    pub fn pair(&self, id: EntityId) -> Option<JoinCopyPair<'_>> {
        let i = self.by_result.get(&id)?;
        self.pairs.get(*i).map(|association| JoinCopyPair {
            context: self,
            association,
        })
    }
}
/// One authentic explicit requested result and its distinct actual A/B original source actions.
#[derive(Clone, Copy, Debug)]
pub struct JoinCopyPair<'a> {
    context: &'a RequiredJoinCopyCorrespondence<'a>,
    association: &'a PairAssociation<'a>,
}
impl<'a> JoinCopyPair<'a> {
    /// Actual complete native correspondence owner.
    #[must_use]
    pub const fn context(self) -> &'a RequiredJoinCopyCorrespondence<'a> {
        self.context
    }
    /// Actual explicitly requested result copy,not a born current copy or geometric placement.
    #[must_use]
    pub const fn result(self) -> JoinResultCopyRequest<'a> {
        self.association.result
    }
    /// Actual guarded source action for original A/B role;no ordinal or geometry matching.
    #[must_use]
    pub const fn source(self, side: SeamSideId) -> JoinCopySourceAction<'a> {
        match side {
            SeamSideId::A => self.association.a,
            SeamSideId::B => self.association.b,
        }
    }
}
