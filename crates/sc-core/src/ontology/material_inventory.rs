//! Owned Materials and actual Piece assignment targets; presence is not physical readiness.
use super::{EntityId, Material, MaterialAssignment, MaterialError, Piece, PieceInventory};
use crate::value::CanonicalInputs;
use core::fmt;
use std::collections::BTreeMap;

/// Assignment data returned by inventory queries; direct construction supplies no validation proof.
/// The query's canonical context remains borrowed while the returned view is used.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PieceMaterialView<'a> {
    /// The actual Piece and its exact target Material, checked in the supplied current scope.
    Assigned {
        /// Actual supplied Piece.
        piece: &'a Piece,
        /// Actual stored target; unknown properties remain unknown.
        material: &'a Material,
    },
    /// Explicit unresolved intent, without selecting a material or inventing a default.
    Unresolved {
        /// Actual supplied Piece.
        piece: &'a Piece,
        /// Original untrimmed reason, borrowed from that Piece's assignment.
        reason: &'a str,
    },
}
impl fmt::Debug for PieceMaterialView<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = match self {
            Self::Assigned { .. } => "assigned",
            Self::Unresolved { .. } => "unresolved",
        };
        f.debug_struct("PieceMaterialView")
            .field("state", &state)
            .finish_non_exhaustive()
    }
}

/// Actual owned-inventory or assignment refusal, retaining original Material property errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MaterialInventoryError {
    /// Repeated owned Material id; identical records are not silently deduplicated.
    DuplicateMaterial {
        /// Repeated identity.
        material: EntityId,
        /// First zero-based authored position.
        first: usize,
        /// First repeated authored position.
        duplicate: usize,
    },
    /// Actual current Material fails canonical identity/property/width validation.
    InvalidMaterial {
        /// Material carrying the failed obligation.
        material: EntityId,
        /// Original typed refusal.
        issue: Box<MaterialError>,
    },
    /// A supplied Piece identity aliases a stored Material identity.
    PieceIdentityCollision {
        /// Shared identity, without inferring either role from its label.
        identity: EntityId,
    },
    /// An actual Piece's assigned Material id is absent; peers/labels do not substitute.
    MissingMaterial {
        /// Piece carrying the assignment.
        piece: EntityId,
        /// Exact absent Material id.
        material: EntityId,
    },
}
impl fmt::Display for MaterialInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateMaterial {
                material,
                first,
                duplicate,
            } => write!(
                f,
                "Material {material} repeats at {duplicate}, first supplied at {first}"
            ),
            Self::InvalidMaterial { material, issue } => {
                write!(f, "Material inventory member {material}: {issue}")
            }
            Self::PieceIdentityCollision { identity } => {
                write!(f, "Piece identity {identity} aliases a Material")
            }
            Self::MissingMaterial { piece, material } => {
                write!(f, "Piece {piece} names absent Material {material}")
            }
        }
    }
}
impl std::error::Error for MaterialInventoryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidMaterial { issue, .. } => Some(issue.as_ref()),
            _ => None,
        }
    }
}

/// Immutable ordered Materials over current canonical declarations, without physical/source approval.
/// Duplicate display names and shared canonical targets are legal. No missing Material is generated.
///
/// ```compile_fail
/// fn clear(inventory: &mut sc_core::ontology::MaterialInventory) { inventory.materials.clear(); }
/// ```
/// ```compile_fail
/// fn clear(inventory: &mut sc_core::ontology::MaterialInventory) { inventory.materials().clear(); }
/// ```
/// ```compile_fail
/// fn replace_context(inventory: &sc_core::ontology::MaterialInventory,
///     piece: &sc_core::ontology::Piece, mut canonical: sc_core::value::CanonicalInputs) {
///     let view = inventory.assignment(piece, &canonical).unwrap();
///     canonical = sc_core::value::CanonicalInputs::new(vec![], vec![]).unwrap();
///     let _still_borrowed = view;
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct MaterialInventory {
    materials: Vec<Material>,
}
impl fmt::Debug for MaterialInventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MaterialInventory")
            .field("material_count", &self.materials.len())
            .finish_non_exhaustive()
    }
}
impl MaterialInventory {
    /// Own exact Materials after complete identity and actual current property validation.
    /// Empty drafts are legal and mint no record; source/treatment references are uses, not owners.
    ///
    /// # Errors
    /// Returns the first duplicate id, then the first original current Material refusal.
    pub fn new(
        materials: Vec<Material>,
        canonical: &CanonicalInputs,
    ) -> Result<Self, MaterialInventoryError> {
        let inventory = Self { materials };
        inventory.validate_current(canonical)?;
        Ok(inventory)
    }

    /// Exact immutable authored Material order.
    #[must_use]
    pub fn materials(&self) -> &[Material] {
        &self.materials
    }

    /// Borrow the canonical Material by id, never display name or position.
    #[must_use]
    pub fn material(&self, identity: EntityId) -> Option<&Material> {
        self.materials
            .iter()
            .find(|material| material.id() == identity)
    }

    /// Recheck owned uniqueness, then every Material against actual current canonical declarations.
    /// This checks canonical-id disjointness through each Material's existing validator. It does not
    /// certify global Design ids, source/evidence facts, numeric availability, geometry or readiness.
    ///
    /// # Errors
    /// Returns the first duplicate, then first current Material cause in authored order.
    pub fn validate_current(
        &self,
        canonical: &CanonicalInputs,
    ) -> Result<(), MaterialInventoryError> {
        let mut seen = BTreeMap::new();
        for (index, material) in self.materials.iter().enumerate() {
            if let Some(first) = seen.insert(material.id(), index) {
                return Err(MaterialInventoryError::DuplicateMaterial {
                    material: material.id(),
                    first,
                    duplicate: index,
                });
            }
        }
        for material in &self.materials {
            Self::validate_material(material, canonical)?;
        }
        Ok(())
    }

    /// Inspect one actual Piece's exact assigned target or original explicit unresolved reason.
    /// Checks this Piece's Material-id disjointness and only its selected Material's current fields.
    /// Unrelated invalid Materials do not block this targeted query; it is no complete-inventory or
    /// physical-readiness certificate. Assigned metadata may still have unknown/derived properties.
    ///
    /// # Errors
    /// Returns Piece/Material identity collision, missing held target, or selected Material's cause.
    pub fn assignment<'a>(
        &'a self,
        piece: &'a Piece,
        canonical: &'a CanonicalInputs,
    ) -> Result<PieceMaterialView<'a>, MaterialInventoryError> {
        self.check_piece_identity(piece)?;
        match &piece.definition().material {
            MaterialAssignment::Assigned(identity) => {
                let material =
                    self.material(*identity)
                        .ok_or(MaterialInventoryError::MissingMaterial {
                            piece: piece.id(),
                            material: *identity,
                        })?;
                Self::validate_material(material, canonical)?;
                Ok(PieceMaterialView::Assigned { piece, material })
            }
            MaterialAssignment::Unresolved(reason) => {
                Ok(PieceMaterialView::Unresolved { piece, reason })
            }
        }
    }

    /// Inspect the complete actual Piece inventory, retaining both assigned and unresolved states.
    /// Validates every current Material first, then all Piece/Material id collisions, then every
    /// assignment in authored Piece order. No partial vector escapes on failure and no Piece, copy,
    /// Material or assignment is rewritten. Other global Design roles remain separate.
    ///
    /// # Errors
    /// Returns whole-inventory/current Material refusal, then Piece identity or held-target refusal.
    pub fn assignments<'a>(
        &'a self,
        pieces: &'a PieceInventory,
        canonical: &'a CanonicalInputs,
    ) -> Result<Vec<PieceMaterialView<'a>>, MaterialInventoryError> {
        self.validate_current(canonical)?;
        for piece in pieces.pieces() {
            self.check_piece_identity(piece)?;
        }
        pieces
            .pieces()
            .iter()
            .map(|piece| self.assignment(piece, canonical))
            .collect()
    }

    fn check_piece_identity(&self, piece: &Piece) -> Result<(), MaterialInventoryError> {
        if self.material(piece.id()).is_some() {
            Err(MaterialInventoryError::PieceIdentityCollision {
                identity: piece.id(),
            })
        } else {
            Ok(())
        }
    }
    fn validate_material(
        material: &Material,
        canonical: &CanonicalInputs,
    ) -> Result<(), MaterialInventoryError> {
        material.validate_current(canonical).map_err(|issue| {
            MaterialInventoryError::InvalidMaterial {
                material: material.id(),
                issue: Box::new(issue),
            }
        })
    }
}
