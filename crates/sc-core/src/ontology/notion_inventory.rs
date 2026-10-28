//! Owned notion placements and their original current physical-copy/geometry sources.
use super::{
    CutPlan, CutPlanError, EntityId, IdentityLedger, NotionPlacement, NotionPlacementError, Piece,
};
use core::fmt;
use std::collections::BTreeMap;

/// Collection/current source refusal retaining the original authored position and cause.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NotionPlacementInventoryError {
    /// Two owned placements claim the same identity.
    DuplicateIdentity {
        /// Exact repeated identity.
        identity: EntityId,
        /// Zero-based first authored position.
        first: usize,
        /// Zero-based first repeated authored position.
        duplicate: usize,
    },
    /// Original complete current physical-copy plan refusal.
    CutPlan(Box<CutPlanError>),
    /// A placement's original source Piece is absent, even if its copy was reassigned.
    MissingPiece {
        /// Zero-based authored placement position.
        index: usize,
        /// Exact placement identity.
        placement: EntityId,
        /// Original source Piece identity.
        piece: EntityId,
    },
    /// Original current copy/anchor/direction refusal for one actual placement.
    Placement {
        /// Zero-based authored placement position.
        index: usize,
        /// Exact placement identity.
        placement: EntityId,
        /// Original typed current source cause.
        issue: Box<NotionPlacementError>,
    },
}
impl fmt::Display for NotionPlacementInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity { identity, first, duplicate } => write!(f, "notion placement identity {identity} repeats at {duplicate}, first supplied at {first}"),
            Self::CutPlan(issue) => write!(f, "notion inventory current copies: {issue}"),
            Self::MissingPiece { index, placement, piece } => write!(f, "notion placement {placement} at {index} lacks original Piece {piece}"),
            Self::Placement { index, placement, issue } => write!(f, "notion placement {placement} at {index}: {issue}"),
        }
    }
}
impl std::error::Error for NotionPlacementInventoryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::CutPlan(issue) => Some(issue.as_ref()),
            Self::Placement { issue, .. } => Some(issue.as_ref()),
            Self::DuplicateIdentity { .. } | Self::MissingPiece { .. } => None,
        }
    }
}

/// Private immutable authored placements; current contexts are borrowed only for validation.
///
/// Construction preserves exact objects, including references held across edits. Current validation
/// checks the complete plan and every placement, including those unused by any Closure. Global
/// Design identities, size declarations and G2/G3/G4 physical validation remain separate.
/// An explicit empty collection makes no whole Design or assembly readiness claim.
///
/// ```compile_fail
/// fn edit(value: &mut sc_core::ontology::NotionPlacementInventory) { value.placements.clear(); }
/// ```
/// ```compile_fail
/// fn edit(value: &mut sc_core::ontology::NotionPlacementInventory) { value.placements().get_mut(0); }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct NotionPlacementInventory {
    placements: Vec<NotionPlacement>,
}
impl fmt::Debug for NotionPlacementInventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NotionPlacementInventory")
            .field("placement_count", &self.placements.len())
            .finish_non_exhaustive()
    }
}
impl NotionPlacementInventory {
    pub(crate) fn resource_placements(&self) -> &[NotionPlacement] {
        let Self { placements } = self;
        placements
    }
    /// Own exact authored order without choosing, retargeting or repairing references.
    ///
    /// # Errors
    /// Returns the first repeated placement identity with both original positions.
    pub fn new(placements: Vec<NotionPlacement>) -> Result<Self, NotionPlacementInventoryError> {
        let mut seen = BTreeMap::new();
        for (duplicate, placement) in placements.iter().enumerate() {
            if let Some(first) = seen.insert(placement.id(), duplicate) {
                return Err(NotionPlacementInventoryError::DuplicateIdentity {
                    identity: placement.id(),
                    first,
                    duplicate,
                });
            }
        }
        Ok(Self { placements })
    }
    /// Exact authored objects; shared inspection grants no mutator or current source proof.
    #[must_use]
    pub fn placements(&self) -> &[NotionPlacement] {
        &self.placements
    }
    /// Inspect one held placement by exact identity, without retargeting or current validation.
    #[must_use]
    pub fn placement(&self, identity: EntityId) -> Option<&NotionPlacement> {
        self.placements
            .iter()
            .find(|placement| placement.id() == identity)
    }
    /// Check the complete current plan and every placement's original Piece, anchor and direction.
    ///
    /// Existing raw choice/repair and whole direction ownership rules are unchanged. No current
    /// context is cached, no references are rewritten and no hardware geometry is proved.
    ///
    /// # Errors
    /// Returns the original complete-plan cause first, then the first placement failure in order.
    pub fn validate_current(
        &self,
        plan: &CutPlan,
        pieces: &[Piece],
        ledger: &IdentityLedger,
    ) -> Result<(), NotionPlacementInventoryError> {
        CutPlan::new(
            plan.copies()
                .iter()
                .map(|copy| *copy.definition())
                .collect(),
            pieces,
        )
        .map_err(|issue| NotionPlacementInventoryError::CutPlan(Box::new(issue)))?;
        for (index, placement) in self.placements().iter().enumerate() {
            let piece = pieces
                .iter()
                .find(|piece| piece.id() == placement.piece())
                .ok_or(NotionPlacementInventoryError::MissingPiece {
                    index,
                    placement: placement.id(),
                    piece: placement.piece(),
                })?;
            placement
                .validate_current(plan, piece, ledger)
                .map_err(|issue| NotionPlacementInventoryError::Placement {
                    index,
                    placement: placement.id(),
                    issue: Box::new(issue),
                })?;
        }
        Ok(())
    }
}
