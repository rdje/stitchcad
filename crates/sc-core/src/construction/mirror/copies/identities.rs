//! Bounded actual copy lookup and known typed exclusions;unlisted identity is not global freshness.
use super::BoundMirrorPieceSourceCopies;
use crate::ontology::{CutCopy, EntityId, MaterialAssignment, Mirroring};
use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

/// Explicit complete plan,source-occurrence,point-candidate and workspace-entry limits.
/// ```compile_fail
/// let limits:sc_core::construction::MirrorPieceCopyIdentityLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MirrorPieceCopyIdentityLimits {
    copies: usize,
    source_occurrences: usize,
    points: usize,
    workspace_entries: usize,
}
impl MirrorPieceCopyIdentityLimits {
    /// Bound whole C plan records,R original occurrences,P point candidates and checked
    /// 2C+R+P+A+7 workspace entries,where A is the actual original axis dependency count.
    #[must_use]
    pub const fn new(
        copies: usize,
        source_occurrences: usize,
        points: usize,
        workspace_entries: usize,
    ) -> Self {
        Self {
            copies,
            source_occurrences,
            points,
            workspace_entries,
        }
    }
    /// Complete actual CutPlan population allowance,including unrelated copies.
    #[must_use]
    pub const fn copies(self) -> usize {
        self.copies
    }
    /// Complete original boundary/hole/construction/fold occurrence allowance.
    #[must_use]
    pub const fn source_occurrences(self) -> usize {
        self.source_occurrences
    }
    /// Complete original point-candidate allowance,including repeated source operations.
    #[must_use]
    pub const fn points(self) -> usize {
        self.points
    }
    /// Complete local index entry upper bound,not allocation bytes or earlier storage.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// Whole resource or definite known Copy/Piece identity-kind conflict;no partial index escapes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPieceCopyIdentityRefusal {
    /// Complete 2C+R+P+A+7 count cannot be represented.
    WorkspaceOverflow,
    /// Whole actual plan population exceeds policy before any identity scan.
    Copies {
        /// Complete actual population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole original source occurrences exceed policy before inspection.
    SourceOccurrences {
        /// Complete actual occurrences.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole original point candidates exceed policy before inspection.
    Points {
        /// Complete actual candidates.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete workspace upper bound exceeds policy before allocation.
    Workspace {
        /// Complete required upper bound.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// A source/result/companion Piece identity also names a known creating operation or Material.
    ContextIdentityCollision {
        /// First such Piece id in stable id order.
        id: EntityId,
    },
    /// Original actual copy id aliases a known Piece,creating operation or Material identity.
    CopyIdentityCollision {
        /// Original complete plan position.
        index: usize,
        /// Actual conflicting copy id.
        id: EntityId,
    },
    /// Original plan pattern Piece id aliases a known creating operation or Material identity.
    PieceIdentityCollision {
        /// Original complete plan position.
        index: usize,
        /// Actual conflicting Piece id.
        piece: EntityId,
    },
}
/// Actual source inventory/context/plan and caller-owned limits survive every refusal.
pub struct MirrorPieceCopyIdentityError<'a> {
    source: &'a BoundMirrorPieceSourceCopies<'a>,
    limits: &'a MirrorPieceCopyIdentityLimits,
    refusal: MirrorPieceCopyIdentityRefusal,
}
impl<'a> MirrorPieceCopyIdentityError<'a> {
    /// Actual complete source-copy association and whole native declaration context.
    #[must_use]
    pub const fn source(&self) -> &'a BoundMirrorPieceSourceCopies<'a> {
        self.source
    }
    /// Original explicit complete policy,not a substituted copy of its fields.
    #[must_use]
    pub const fn limits(&self) -> &'a MirrorPieceCopyIdentityLimits {
        self.limits
    }
    /// First complete resource or original identity-kind failure.
    #[must_use]
    pub const fn refusal(&self) -> MirrorPieceCopyIdentityRefusal {
        self.refusal
    }
}
impl fmt::Debug for MirrorPieceCopyIdentityError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MirrorPieceCopyIdentityError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for MirrorPieceCopyIdentityError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mirror copy identity exclusions: {:?}", self.refusal)
    }
}
impl std::error::Error for MirrorPieceCopyIdentityError<'_> {}
impl<'s> BoundMirrorPieceSourceCopies<'s> {
    /// Index actual whole plan copies and known typed exclusions before result-copy proposals.
    /// All counts precede identity inspection/allocation. Own workspace is bounded by 2C+R+P+A+7
    /// entries:copy and Piece indexes of at most C each,at most R+P+A+3 operation/Material ids,
    /// plus four source/result/companion Piece slots. Work is O((C+R+P+A)log(C+R+P+A+2)).
    /// This grants no global namespace,issued-history,provider/currentness or freshness certificate.
    /// # Errors
    /// Checked workspace count,C/R/P/workspace quotas;then first context Piece conflict in stable
    /// id order,then each original plan record's copy-id and pattern-Piece-id conflicts.
    pub fn bind_copy_identities<'a>(
        &'a self,
        limits: &'a MirrorPieceCopyIdentityLimits,
    ) -> Result<BoundMirrorPieceCopyIdentities<'a>, MirrorPieceCopyIdentityError<'a>>
    where
        's: 'a,
    {
        use MirrorPieceCopyIdentityRefusal as R;
        let refuse = |refusal| MirrorPieceCopyIdentityError {
            source: self,
            limits,
            refusal,
        };
        let context = self.context();
        let contours = context.contours();
        let metadata = contours.coverage().edges().identity().roles().metadata();
        let original = metadata.cut().source();
        let input = original.input();
        let c = self.plan().copies().len();
        let r = contours.coverage().occurrence_count();
        let p = context.points().point_count();
        let a = input.dependency_count();
        let workspace_bound = c
            .checked_mul(2)
            .and_then(|v| v.checked_add(r))
            .and_then(|v| v.checked_add(p))
            .and_then(|v| v.checked_add(a))
            .and_then(|v| v.checked_add(7))
            .ok_or_else(|| refuse(R::WorkspaceOverflow))?;
        if c > limits.copies {
            return Err(refuse(R::Copies {
                required: c,
                allowed: limits.copies,
            }));
        }
        if r > limits.source_occurrences {
            return Err(refuse(R::SourceOccurrences {
                required: r,
                allowed: limits.source_occurrences,
            }));
        }
        if p > limits.points {
            return Err(refuse(R::Points {
                required: p,
                allowed: limits.points,
            }));
        }
        if workspace_bound > limits.workspace_entries {
            return Err(refuse(R::Workspace {
                required: workspace_bound,
                allowed: limits.workspace_entries,
            }));
        }
        let mut non_copy_ids = BTreeSet::new();
        non_copy_ids.insert(input.operation());
        for occurrence in original.edges() {
            non_copy_ids.insert(occurrence.source().edge().creator());
        }
        for point in context.points().points() {
            non_copy_ids.insert(point.source().creator());
        }
        for dependency in input.dependencies() {
            non_copy_ids.insert(dependency.source().creator());
        }
        for material in [&original.piece().definition().material, metadata.material()] {
            if let MaterialAssignment::Assigned(id) = material {
                non_copy_ids.insert(*id);
            }
        }
        let mut piece_ids = BTreeSet::from([
            original.piece().id(),
            *contours.coverage().edges().identity().piece_id(),
        ]);
        for mirroring in [
            &original.piece().definition().mirroring,
            metadata.cut().mirroring(),
        ] {
            if let Mirroring::PairMember { companion, .. } = mirroring {
                piece_ids.insert(*companion);
            }
        }
        if let Some(id) = piece_ids.intersection(&non_copy_ids).next() {
            return Err(refuse(R::ContextIdentityCollision { id: *id }));
        }
        let mut by_id = BTreeMap::new();
        for (index, copy) in self.plan().copies().iter().enumerate() {
            if piece_ids.contains(&copy.id()) || non_copy_ids.contains(&copy.id()) {
                return Err(refuse(R::CopyIdentityCollision {
                    index,
                    id: copy.id(),
                }));
            }
            if non_copy_ids.contains(&copy.piece()) {
                return Err(refuse(R::PieceIdentityCollision {
                    index,
                    piece: copy.piece(),
                }));
            }
            // The immutable CutPlan already proved copy-id uniqueness and its complete Piece membership.
            by_id.insert(copy.id(), copy);
            piece_ids.insert(copy.piece());
        }
        Ok(BoundMirrorPieceCopyIdentities {
            source: self,
            limits,
            workspace_bound,
            by_id,
            piece_ids,
            non_copy_ids,
        })
    }
}
/// Actual copy index and scoped known typed exclusions,not whole Design or global freshness evidence.
/// ```compile_fail
/// fn reparent<'a>(mut ids:sc_core::construction::BoundMirrorPieceCopyIdentities<'a>,
///     other:&'a sc_core::construction::BoundMirrorPieceSourceCopies<'a>) {ids.source=other;}
/// ```
/// ```compile_fail
/// fn escape<'a>(source:&'a sc_core::construction::BoundMirrorPieceSourceCopies<'a>)
///     ->sc_core::construction::BoundMirrorPieceCopyIdentities<'a> {
///     let limits=sc_core::construction::MirrorPieceCopyIdentityLimits::new(0,0,0,0);
///     source.bind_copy_identities(&limits).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct BoundMirrorPieceCopyIdentities<'a> {
    source: &'a BoundMirrorPieceSourceCopies<'a>,
    limits: &'a MirrorPieceCopyIdentityLimits,
    workspace_bound: usize,
    by_id: BTreeMap<EntityId, &'a CutCopy>,
    piece_ids: BTreeSet<EntityId>,
    non_copy_ids: BTreeSet<EntityId>,
}
impl<'a> BoundMirrorPieceCopyIdentities<'a> {
    /// Actual complete source-copy/plan/context parent.
    #[must_use]
    pub const fn source(&self) -> &'a BoundMirrorPieceSourceCopies<'a> {
        self.source
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> &'a MirrorPieceCopyIdentityLimits {
        self.limits
    }
    /// Admitted index entry upper bound,not allocation bytes or global storage.
    #[must_use]
    pub const fn workspace_bound(&self) -> usize {
        self.workspace_bound
    }
    /// Complete actual plan copies indexed,including unrelated Pieces and existing result copies.
    #[must_use]
    pub fn indexed_copy_count(&self) -> usize {
        self.by_id.len()
    }
    /// Actual original CutCopy by explicit persistent id;no position-derived identity.
    #[must_use]
    pub fn copy(&self, id: EntityId) -> Option<&'a CutCopy> {
        self.by_id.get(&id).copied()
    }
    /// Known typed Piece id from actual plan or source/result/companion declarations,not existence proof.
    #[must_use]
    pub fn knows_piece(&self, id: EntityId) -> bool {
        self.piece_ids.contains(&id)
    }
    /// True rejects a newly issued copy id in this local context;false does not prove global freshness.
    /// Existing copy ids stay excluded even when a later proposal explicitly retires those copies.
    #[must_use]
    pub fn excludes_new_copy(&self, id: EntityId) -> bool {
        self.by_id.contains_key(&id)
            || self.piece_ids.contains(&id)
            || self.non_copy_ids.contains(&id)
    }
}
