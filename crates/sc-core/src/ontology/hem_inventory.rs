//! Owned Hems and complete current layer/finish/Facing targets, without executed physical folds.
use super::{
    EntityId, Hem, HemError, HemMethod, IdentityLedger, LayerInventory, LayerInventoryError, Piece,
};
use core::fmt;
use std::collections::BTreeMap;

/// Original collection/current-source refusal with actual authored Hem positions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HemInventoryError {
    /// Two authored Hem objects claim one owned identity.
    DuplicateIdentity {
        /// Exact repeated identity.
        identity: EntityId,
        /// Zero-based first authored position.
        first: usize,
        /// Zero-based first repeated authored position.
        duplicate: usize,
    },
    /// Original whole current layer/Piece/source cause, including unused layers.
    Layers(Box<LayerInventoryError>),
    /// An actual Hem's original Piece is missing; no surviving peer substitutes.
    MissingPiece {
        /// Zero-based authored Hem position.
        index: usize,
        /// Exact Hem identity.
        hem: EntityId,
        /// Exact original Piece target.
        piece: EntityId,
    },
    /// Original actual Hem current owner/depth/finish/Facing refusal.
    Hem {
        /// Zero-based authored Hem position.
        index: usize,
        /// Exact Hem identity.
        hem: EntityId,
        /// Original concrete current target cause.
        issue: Box<HemError>,
    },
}
impl fmt::Display for HemInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity {
                identity,
                first,
                duplicate,
            } => write!(
                f,
                "Hem identity {identity} repeats at {duplicate}, first supplied at {first}"
            ),
            Self::Layers(issue) => write!(f, "Hem inventory current layers: {issue}"),
            Self::MissingPiece { index, hem, piece } => {
                write!(f, "Hem {hem} at {index} lacks original Piece {piece}")
            }
            Self::Hem { index, hem, issue } => write!(f, "Hem {hem} at {index}: {issue}"),
        }
    }
}
impl std::error::Error for HemInventoryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Layers(issue) => Some(issue.as_ref()),
            Self::Hem { issue, .. } => Some(issue.as_ref()),
            _ => None,
        }
    }
}

/// Private immutable authored Hems, with current contexts borrowed only for validation.
///
/// Construction checks collection ids and preserves original objects, including held damage. Current
/// validation checks complete supplied layer sources first, then every original Hem target. Material
/// assignments, lining execution scope, declarations, global Design ids and G2/G3/G4 proof stay
/// separate. An explicit empty collection makes no whole Design or assembly readiness claim.
///
/// ```compile_fail
/// fn edit(value: &mut sc_core::ontology::HemInventory) { value.hems.clear(); }
/// ```
/// ```compile_fail
/// fn edit(value: &mut sc_core::ontology::HemInventory) { value.hems().get_mut(0); }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct HemInventory {
    hems: Vec<Hem>,
}
impl fmt::Debug for HemInventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HemInventory")
            .field("hem_count", &self.hems.len())
            .finish_non_exhaustive()
    }
}
impl HemInventory {
    /// Own exact authored order without sorting, retargeting or repairing references.
    /// # Errors
    /// Returns the first repeated Hem identity with both original positions.
    pub fn new(hems: Vec<Hem>) -> Result<Self, HemInventoryError> {
        let mut seen = BTreeMap::new();
        for (duplicate, hem) in hems.iter().enumerate() {
            if let Some(first) = seen.insert(hem.id(), duplicate) {
                return Err(HemInventoryError::DuplicateIdentity {
                    identity: hem.id(),
                    first,
                    duplicate,
                });
            }
        }
        Ok(Self { hems })
    }
    /// Exact original objects; shared inspection supplies no mutator or current source proof.
    #[must_use]
    pub fn hems(&self) -> &[Hem] {
        &self.hems
    }
    /// Inspect one held Hem by exact identity, without current validation or substitution.
    #[must_use]
    pub fn hem(&self, identity: EntityId) -> Option<&Hem> {
        self.hems.iter().find(|hem| hem.id() == identity)
    }
    /// Check all supplied current layer sources, then every Hem's original owner/finish/Facing.
    ///
    /// Even unused layers and an empty Hem collection check the complete provided layer context.
    /// Facing targets select only actual Facings; a same-id Lining/Interfacing never substitutes.
    /// Original raw choices/repairs and depth domains are unchanged. This supplies no Material,
    /// declaration, supported-execution, physical fold or release-readiness proof.
    ///
    /// # Errors
    /// Returns the original whole-layer cause first, then first Hem failure in authored order.
    pub fn validate_current(
        &self,
        layers: &LayerInventory,
        pieces: &[Piece],
        ledger: &IdentityLedger,
    ) -> Result<(), HemInventoryError> {
        layers
            .validate_current(pieces, ledger)
            .map_err(|issue| HemInventoryError::Layers(Box::new(issue)))?;
        for (index, hem) in self.hems().iter().enumerate() {
            let piece = pieces
                .iter()
                .find(|piece| piece.id() == hem.piece())
                .ok_or(HemInventoryError::MissingPiece {
                    index,
                    hem: hem.id(),
                    piece: hem.piece(),
                })?;
            let facing = match hem.definition().method {
                HemMethod::Turned => None,
                HemMethod::Faced { facing } => layers.facing(facing),
            };
            hem.validate_current(piece, ledger, facing)
                .map_err(|issue| HemInventoryError::Hem {
                    index,
                    hem: hem.id(),
                    issue: Box::new(issue),
                })?;
        }
        Ok(())
    }
}
