//! Whole held join inventory admission;no substitute source records or copy consumption.
use super::BoundJoinPiecesAtSeam;
use crate::ontology::{CutPlan, CutPlanError, PieceInventory};
use core::fmt;

/// Complete actual plan,inventory and transient logical-entry allowances.
/// ```compile_fail
/// let limits:sc_core::construction::JoinCopyInventoryLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinCopyInventoryLimits {
    copies: usize,
    pieces: usize,
    workspace_entries: usize,
}
impl JoinCopyInventoryLimits {
    /// Bound whole C/I and checked I+3C before source association,field scans or allocation.
    #[must_use]
    pub const fn new(copies: usize, pieces: usize, workspace_entries: usize) -> Self {
        Self {
            copies,
            pieces,
            workspace_entries,
        }
    }
    /// Every original plan copy,including unrelated Pieces.
    #[must_use]
    pub const fn copies(self) -> usize {
        self.copies
    }
    /// Every actual inventory member,including unrelated Pieces.
    #[must_use]
    pub const fn pieces(self) -> usize {
        self.pieces
    }
    /// Temporary logical entries,not bytes or earlier owned input storage.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// First complete resource,original-source or canonical copy-plan cause.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JoinCopyInventoryRefusal {
    /// Checked I+3C cannot be represented.
    WorkspaceOverflow,
    /// Whole actual plan exceeds policy.
    Copies {
        /// Full original count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole actual inventory exceeds policy.
    Pieces {
        /// Full original count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete transient canonical validation exceeds policy.
    Workspace {
        /// Checked I+3C.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// The source join was not bound from this exact inventory's complete borrowed slice.
    SourceInventory,
    /// The original complete plan fails canonical validation against this same inventory.
    Inventory(CutPlanError),
}
/// Actual whole original source,inventory,policy and first refusal;no accepted prefix escapes.
pub struct JoinCopyInventoryError<'a> {
    source: &'a BoundJoinPiecesAtSeam<'a>,
    inventory: &'a PieceInventory,
    limits: &'a JoinCopyInventoryLimits,
    refusal: JoinCopyInventoryRefusal,
}
impl<'a> JoinCopyInventoryError<'a> {
    /// Actual complete original join association.
    #[must_use]
    pub const fn source(&self) -> &'a BoundJoinPiecesAtSeam<'a> {
        self.source
    }
    /// Actual complete proposed source inventory.
    #[must_use]
    pub const fn inventory(&self) -> &'a PieceInventory {
        self.inventory
    }
    /// Actual original complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> &'a JoinCopyInventoryLimits {
        self.limits
    }
    /// First whole-resource,original association or canonical cause.
    #[must_use]
    pub const fn refusal(&self) -> &JoinCopyInventoryRefusal {
        &self.refusal
    }
}
impl fmt::Debug for JoinCopyInventoryError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinCopyInventoryError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinCopyInventoryError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join copy inventory: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinCopyInventoryError<'_> {}

impl<'s> BoundJoinPiecesAtSeam<'s> {
    /// Admit the same actual source PieceInventory and revalidate the whole original CutPlan.
    /// Whole C/I/I+3C quotas precede exact borrowed-slice identity and canonical validation.
    /// Bind the source with inventory.pieces();a separate equal-looking slice is not this source.
    /// Existing immutable PieceInventory construction supplies companion consistency. Canonical
    /// CutPlan validation checks all quantities,orientations and identities against this inventory.
    /// Its temporary plan is dropped;original A/B Piece/copy pointers and authored order survive.
    /// Work O((C+I)log(C+I+2)),temporary at most I+3C logical entries,constant cached references.
    /// No labels,material reasons,contours,graph bodies,geometry or provider bodies are scanned.
    /// Prior input/lookup resources,current Design membership,output policy and authority stay separate.
    /// # Errors
    /// Checked workspace,whole C/I/workspace quotas,source-slice mismatch,then canonical CutPlan cause.
    pub fn bind_copy_inventory<'a>(
        &'a self,
        inventory: &'a PieceInventory,
        limits: &'a JoinCopyInventoryLimits,
    ) -> Result<BoundJoinCopyInventory<'a>, JoinCopyInventoryError<'a>>
    where
        's: 'a,
    {
        use JoinCopyInventoryRefusal as R;
        let refuse = |refusal| JoinCopyInventoryError {
            source: self,
            inventory,
            limits,
            refusal,
        };
        let c = self.plan().copies().len();
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
        if !core::ptr::eq(self.pieces(), inventory.pieces()) {
            return Err(refuse(R::SourceInventory));
        }
        let definitions = self
            .plan()
            .copies()
            .iter()
            .map(|copy| *copy.definition())
            .collect();
        drop(
            CutPlan::new(definitions, inventory.pieces())
                .map_err(|cause| refuse(R::Inventory(cause)))?,
        );
        Ok(BoundJoinCopyInventory {
            source: self,
            inventory,
            limits,
            workspace,
        })
    }
}
/// Authentic complete original inventory and copy association;no result/copy consumption is declared.
/// ```compile_fail
/// fn forge<'a>(mut bound:sc_core::construction::BoundJoinCopyInventory<'a>,
///     other:&'a sc_core::ontology::PieceInventory) { bound.inventory=other; }
/// ```
/// ```compile_fail
/// fn escape<'a>(source:&'a sc_core::construction::BoundJoinPiecesAtSeam<'a>,
///     inventory:&'a sc_core::ontology::PieceInventory)->sc_core::construction::BoundJoinCopyInventory<'a> {
///     let limits=sc_core::construction::JoinCopyInventoryLimits::new(2,2,8);
///     source.bind_copy_inventory(inventory,&limits).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoundJoinCopyInventory<'a> {
    source: &'a BoundJoinPiecesAtSeam<'a>,
    inventory: &'a PieceInventory,
    limits: &'a JoinCopyInventoryLimits,
    workspace: usize,
}
impl<'a> BoundJoinCopyInventory<'a> {
    /// Actual original source join,retaining original side/copy/Piece fields and span metadata.
    #[must_use]
    pub const fn source(&self) -> &'a BoundJoinPiecesAtSeam<'a> {
        self.source
    }
    /// Actual companion-consistent source inventory;its slice is the original source slice.
    #[must_use]
    pub const fn inventory(&self) -> &'a PieceInventory {
        self.inventory
    }
    /// Actual original complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> &'a JoinCopyInventoryLimits {
        self.limits
    }
    /// Checked temporary I+3C upper bound,not bytes or prior input storage.
    #[must_use]
    pub const fn workspace_bound(&self) -> usize {
        self.workspace
    }
}
