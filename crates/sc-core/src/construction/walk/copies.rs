//! Actual walk side-copy/pattern association;shared pattern ownership does not coalesce marks.
use super::BoundWalkSeamEdit;
use crate::ontology::{
    CutCopy, CutPlan, CutPlanError, EntityId, Piece, PieceInventory, SeamSideId,
};
use core::fmt;
/// Complete actual copy,Piece and transient logical-workspace allowances.
/// ```compile_fail
/// let limits:sc_core::construction::WalkCopyInventoryLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WalkCopyInventoryLimits {
    copies: usize,
    pieces: usize,
    workspace_entries: usize,
}
impl WalkCopyInventoryLimits {
    /// Select whole C/I and checked I+3C allowances before any lookup,scan or allocation.
    #[must_use]
    pub const fn new(copies: usize, pieces: usize, workspace_entries: usize) -> Self {
        Self {
            copies,
            pieces,
            workspace_entries,
        }
    }
    /// Whole actual plan allowance,including unrelated copies.
    #[must_use]
    pub const fn copies(self) -> usize {
        self.copies
    }
    /// Whole actual candidate inventory allowance,including unrelated Pieces.
    #[must_use]
    pub const fn pieces(self) -> usize {
        self.pieces
    }
    /// Temporary logical-entry bound,not allocation bytes or earlier model storage.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// Complete resource,canonical association or first original A/B side-reference refusal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WalkCopyInventoryRefusal {
    /// Checked I+3C cannot be represented.
    WorkspaceOverflow,
    /// Whole actual plan exceeds policy.
    Copies {
        /// Complete actual count.
        required: usize,
        /// Caller allowance.
        allowed: usize,
    },
    /// Whole actual inventory exceeds policy.
    Pieces {
        /// Complete actual count.
        required: usize,
        /// Caller allowance.
        allowed: usize,
    },
    /// Complete transient canonical validation exceeds the logical-entry allowance.
    Workspace {
        /// Checked complete upper bound.
        required: usize,
        /// Caller allowance.
        allowed: usize,
    },
    /// Actual plan fails the canonical CutPlan contract against this actual Piece inventory.
    Inventory(CutPlanError),
    /// First original A/B copy identity is absent from the actual complete plan.
    MissingCopy {
        /// Original source side.
        side: SeamSideId,
        /// Actual held physical-copy identity.
        copy: EntityId,
    },
    /// Actual side copy names no actual Piece;kept explicit rather than assuming birth ownership.
    MissingPiece {
        /// Original source side.
        side: SeamSideId,
        /// Actual physical copy.
        copy: EntityId,
        /// Required actual pattern owner.
        piece: EntityId,
    },
}
/// Whole actual bound edit,plan,inventory,policy and first cause;no bound prefix escapes.
pub struct WalkCopyInventoryError<'a> {
    source: &'a BoundWalkSeamEdit<'a>,
    plan: &'a CutPlan,
    inventory: &'a PieceInventory,
    limits: &'a WalkCopyInventoryLimits,
    refusal: WalkCopyInventoryRefusal,
}
impl<'a> WalkCopyInventoryError<'a> {
    /// Actual original graph/span/pair association.
    #[must_use]
    pub const fn source(&self) -> &'a BoundWalkSeamEdit<'a> {
        self.source
    }
    /// Actual complete supplied immutable physical-copy plan.
    #[must_use]
    pub const fn plan(&self) -> &'a CutPlan {
        self.plan
    }
    /// Actual complete supplied structurally companion-consistent inventory.
    #[must_use]
    pub const fn inventory(&self) -> &'a PieceInventory {
        self.inventory
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> &'a WalkCopyInventoryLimits {
        self.limits
    }
    /// First whole resource,canonical or original A/B cause.
    #[must_use]
    pub const fn refusal(&self) -> &WalkCopyInventoryRefusal {
        &self.refusal
    }
}
impl fmt::Debug for WalkCopyInventoryError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WalkCopyInventoryError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for WalkCopyInventoryError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "walk copy inventory: {:?}", self.refusal)
    }
}
impl std::error::Error for WalkCopyInventoryError<'_> {}
impl<'s> BoundWalkSeamEdit<'s> {
    /// Associate actual A/B physical copies and pattern Pieces under whole C/I/I+3C limits.
    /// Canonical CutPlan validation runs against this actual inventory before side lookup;its temporary
    /// proof is dropped before caching actual A/B records. Work is O((C+I)log(C+I+2)),with bounded
    /// transient logical workspace and constant cached references;no pair/registry/geometry scan.
    /// Shared pattern identity alone supplies no mark coalescing,anchor ownership,currentness,global
    /// namespace,copy effects or authority. Existing input/graph/resource scopes remain unchanged.
    /// # Errors
    /// Checked workspace,C/I/workspace quotas,canonical association,then first A/B absent copy/Piece.
    pub fn bind_copy_inventory<'a>(
        &'a self,
        plan: &'a CutPlan,
        inventory: &'a PieceInventory,
        limits: &'a WalkCopyInventoryLimits,
    ) -> Result<BoundWalkCopyInventory<'a>, WalkCopyInventoryError<'a>>
    where
        's: 'a,
    {
        use WalkCopyInventoryRefusal as R;
        let refuse = |refusal| WalkCopyInventoryError {
            source: self,
            plan,
            inventory,
            limits,
            refusal,
        };
        let c = plan.copies().len();
        let i = inventory.pieces().len();
        let workspace = c
            .checked_mul(3)
            .and_then(|count| count.checked_add(i))
            .ok_or_else(|| refuse(R::WorkspaceOverflow))?;
        if c > limits.copies {
            return Err(refuse(R::Copies {
                required: c,
                allowed: limits.copies,
            }));
        }
        if i > limits.pieces {
            return Err(refuse(R::Pieces {
                required: i,
                allowed: limits.pieces,
            }));
        }
        if workspace > limits.workspace_entries {
            return Err(refuse(R::Workspace {
                required: workspace,
                allowed: limits.workspace_entries,
            }));
        }
        let definitions = plan
            .copies()
            .iter()
            .map(|copy| *copy.definition())
            .collect();
        drop(
            CutPlan::new(definitions, inventory.pieces())
                .map_err(|error| refuse(R::Inventory(error)))?,
        );
        let resolve = |side| {
            let copy = match side {
                SeamSideId::A => self.span().definition().a.copy,
                SeamSideId::B => self.span().definition().b.copy,
            };
            let actual = plan
                .copy(copy)
                .ok_or_else(|| refuse(R::MissingCopy { side, copy }))?;
            let piece = inventory.piece(actual.piece()).ok_or_else(|| {
                refuse(R::MissingPiece {
                    side,
                    copy,
                    piece: actual.piece(),
                })
            })?;
            Ok((actual, piece))
        };
        let (a_copy, a_piece) = resolve(SeamSideId::A)?;
        let (b_copy, b_piece) = resolve(SeamSideId::B)?;
        Ok(BoundWalkCopyInventory {
            source: self,
            plan,
            inventory,
            limits,
            workspace,
            a_copy,
            b_copy,
            a_piece,
            b_piece,
        })
    }
}
/// Authentic complete copy/pattern association;no semantic mark or copy pairing is inferred.
/// ```compile_fail
/// fn forge<'a>(mut bound:sc_core::construction::BoundWalkCopyInventory<'a>,other:&'a sc_core::ontology::CutPlan) {bound.plan=other;}
/// ```
/// ```compile_fail
/// fn escape<'a>(source:&'a sc_core::construction::BoundWalkSeamEdit<'a>)
///     ->sc_core::construction::BoundWalkCopyInventory<'a> {
///     use sc_core::{construction::*,ontology::*};let plan=CutPlan::new(vec![],&[]).unwrap();
///     let inventory=PieceInventory::new(vec![]).unwrap();let limits=WalkCopyInventoryLimits::new(0,0,0);
///     source.bind_copy_inventory(&plan,&inventory,&limits).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoundWalkCopyInventory<'a> {
    source: &'a BoundWalkSeamEdit<'a>,
    plan: &'a CutPlan,
    inventory: &'a PieceInventory,
    limits: &'a WalkCopyInventoryLimits,
    workspace: usize,
    a_copy: &'a CutCopy,
    b_copy: &'a CutCopy,
    a_piece: &'a Piece,
    b_piece: &'a Piece,
}
impl<'a> BoundWalkCopyInventory<'a> {
    /// Actual complete original graph/span/pair association.
    #[must_use]
    pub const fn source(&self) -> &'a BoundWalkSeamEdit<'a> {
        self.source
    }
    /// Actual complete supplied immutable physical-copy plan,in authored order.
    #[must_use]
    pub const fn plan(&self) -> &'a CutPlan {
        self.plan
    }
    /// Actual complete supplied structurally companion-consistent Piece inventory,in authored order.
    #[must_use]
    pub const fn inventory(&self) -> &'a PieceInventory {
        self.inventory
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> &'a WalkCopyInventoryLimits {
        self.limits
    }
    /// Checked I+3C upper bound for temporary canonical validation,not bytes/whole storage.
    #[must_use]
    pub const fn workspace_bound(&self) -> usize {
        self.workspace
    }
    /// Actual cached physical copy for the original canonical side;no ordinal lookup.
    #[must_use]
    pub const fn copy(&self, side: SeamSideId) -> &'a CutCopy {
        match side {
            SeamSideId::A => self.a_copy,
            SeamSideId::B => self.b_copy,
        }
    }
    /// Actual cached pattern owner of the side's actual physical copy,not inferred from an edge id.
    #[must_use]
    pub const fn piece(&self, side: SeamSideId) -> &'a Piece {
        match side {
            SeamSideId::A => self.a_piece,
            SeamSideId::B => self.b_piece,
        }
    }
    /// Whether both actual copies share one pattern Piece. This alone supplies no mark-sharing
    /// policy,anchor/profile equality,geometric correspondence or accepted semantic output.
    #[must_use]
    pub fn same_pattern_piece(&self) -> bool {
        self.a_piece.id() == self.b_piece.id()
    }
}
