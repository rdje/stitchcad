//! Explicit physical cut-copy identities, distinct from pattern-piece identity (ontology §4.2).
use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use super::{EntityId, GeometricValidation, Mirroring, Piece};

/// Physical-copy orientation relative to its pattern Piece, independent of journal traversal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CopyOrientation {
    /// Retain the pattern Piece's authored handedness.
    Authored,
    /// Reflect the pattern Piece; G2/V1 supplies the actual transform.
    Reflected,
}

/// Editable physical-copy input. Position in a list never supplies its identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CutCopyDefinition {
    /// Stable identity of this physical copy, explicitly supplied by the caller.
    pub id: EntityId,
    /// Pattern Piece whose recipe and source-frame edge references this copy uses.
    pub piece: EntityId,
    /// Authored or reflected geometry; not a second pattern definition.
    pub orientation: CopyOrientation,
}

/// Immutable physical copy, constructed only within a validated complete [`CutPlan`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CutCopy {
    definition: CutCopyDefinition,
}

impl CutCopy {
    /// Stable physical-copy identity.
    #[must_use]
    pub const fn id(self) -> EntityId {
        self.definition.id
    }

    /// Pattern Piece identity.
    #[must_use]
    pub const fn piece(self) -> EntityId {
        self.definition.piece
    }

    /// Geometry orientation relative to the source Piece.
    #[must_use]
    pub const fn orientation(self) -> CopyOrientation {
        self.definition.orientation
    }

    /// Authored input; a shared view grants no mutation.
    #[must_use]
    pub const fn definition(&self) -> &CutCopyDefinition {
        &self.definition
    }
}

/// A complete immutable physical-copy plan, in caller-authored order.
///
/// ```compile_fail
/// fn remove_copy(plan: &mut sc_core::ontology::CutPlan) {
///     plan.copies.clear();
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CutPlan {
    copies: Vec<CutCopy>,
}

/// Structural cut-plan refusal; no missing copy is silently manufactured.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CutPlanError {
    /// Two supplied Pieces have the same entity identity.
    DuplicatePiece {
        /// Repeated identity.
        piece: EntityId,
    },
    /// Two physical copies have the same identity.
    DuplicateCopy {
        /// Repeated identity.
        copy: EntityId,
    },
    /// A copy identity aliases a Piece identity in the same entity namespace.
    IdentityCollision {
        /// Aliased entity identity.
        id: EntityId,
    },
    /// A copy names no supplied Piece.
    MissingPiece {
        /// Physical copy carrying the invalid reference.
        copy: EntityId,
        /// Absent pattern Piece.
        piece: EntityId,
    },
    /// Copy count disagrees with the Piece's authoritative cut quantity.
    QuantityMismatch {
        /// Pattern Piece.
        piece: EntityId,
        /// Requested quantity.
        expected: u32,
        /// Supplied physical copies.
        actual: u32,
    },
    /// A reflected copy is incompatible with an authored-only cut plan.
    UnexpectedReflection {
        /// Physical copy.
        copy: EntityId,
        /// Pattern Piece.
        piece: EntityId,
    },
    /// A mirrored-pair request has unequal authored/reflected populations.
    UnbalancedPairs {
        /// Pattern Piece.
        piece: EntityId,
        /// Authored copies supplied.
        authored: u32,
        /// Reflected copies supplied.
        reflected: u32,
    },
    /// The supplied population cannot be counted in the cut-quantity domain.
    CountOverflow {
        /// Pattern Piece.
        piece: EntityId,
    },
}

impl fmt::Display for CutPlanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicatePiece { piece } => write!(f, "cut plan repeats pattern piece {piece}"),
            Self::DuplicateCopy { copy } => write!(f, "cut plan repeats physical copy {copy}"),
            Self::IdentityCollision { id } => write!(f, "physical copy {id} aliases a pattern piece identity"),
            Self::MissingPiece { copy, piece } => write!(f, "physical copy {copy} names absent piece {piece}"),
            Self::QuantityMismatch { piece, expected, actual } => write!(f, "piece {piece} requests {expected} copies, plan supplies {actual}"),
            Self::UnexpectedReflection { copy, piece } => write!(f, "copy {copy} reflects authored-only piece {piece}"),
            Self::UnbalancedPairs { piece, authored, reflected } => write!(f, "piece {piece} requires paired orientations, got {authored} authored / {reflected} reflected"),
            Self::CountOverflow { piece } => write!(f, "copy count for piece {piece} exceeds u32"),
        }
    }
}
impl std::error::Error for CutPlanError {}

#[derive(Default)]
struct Population {
    total: u32,
    authored: u32,
    reflected: u32,
}

impl CutPlan {
    /// Validate a complete plan against the current Piece collection, without generating identities.
    ///
    /// # Errors
    /// Returns [`CutPlanError`] for duplicate/unknown identities, wrong counts or orientations.
    /// Companion-collection checks, geometry and design-level revision checks remain separate.
    pub fn new(
        definitions: Vec<CutCopyDefinition>,
        pieces: &[Piece],
    ) -> Result<Self, CutPlanError> {
        let mut patterns = BTreeMap::new();
        for piece in pieces {
            if patterns
                .insert(piece.id(), (piece, Population::default()))
                .is_some()
            {
                return Err(CutPlanError::DuplicatePiece { piece: piece.id() });
            }
        }
        let mut copy_ids = BTreeSet::new();
        for copy in &definitions {
            if !copy_ids.insert(copy.id) {
                return Err(CutPlanError::DuplicateCopy { copy: copy.id });
            }
            if patterns.contains_key(&copy.id) {
                return Err(CutPlanError::IdentityCollision { id: copy.id });
            }
            let (piece, population) =
                patterns
                    .get_mut(&copy.piece)
                    .ok_or(CutPlanError::MissingPiece {
                        copy: copy.id,
                        piece: copy.piece,
                    })?;
            if copy.orientation == CopyOrientation::Reflected
                && piece.definition().mirroring != Mirroring::MirroredPairs
            {
                return Err(CutPlanError::UnexpectedReflection {
                    copy: copy.id,
                    piece: copy.piece,
                });
            }
            let overflow = CutPlanError::CountOverflow { piece: copy.piece };
            population.total = population
                .total
                .checked_add(1)
                .ok_or_else(|| overflow.clone())?;
            let oriented = match copy.orientation {
                CopyOrientation::Authored => &mut population.authored,
                CopyOrientation::Reflected => &mut population.reflected,
            };
            *oriented = oriented.checked_add(1).ok_or(overflow)?;
        }
        // Final quantity checks use stable Piece-id order; copy order never defines identity.
        for (piece, population) in patterns.values() {
            if population.total != piece.definition().quantity {
                return Err(CutPlanError::QuantityMismatch {
                    piece: piece.id(),
                    expected: piece.definition().quantity,
                    actual: population.total,
                });
            }
            if piece.definition().mirroring == Mirroring::MirroredPairs
                && population.authored != population.reflected
            {
                return Err(CutPlanError::UnbalancedPairs {
                    piece: piece.id(),
                    authored: population.authored,
                    reflected: population.reflected,
                });
            }
        }
        Ok(Self {
            copies: definitions
                .into_iter()
                .map(|definition| CutCopy { definition })
                .collect(),
        })
    }

    /// Physical copies in caller-authored order; identity is independent of that order.
    #[must_use]
    pub fn copies(&self) -> &[CutCopy] {
        &self.copies
    }

    /// Find a physical copy by persistent identity, never by ordinal.
    #[must_use]
    pub fn copy(&self, id: EntityId) -> Option<&CutCopy> {
        self.copies.iter().find(|copy| copy.id() == id)
    }

    /// Copies of a pattern Piece, preserving authored plan order.
    pub fn copies_for_piece(&self, piece: EntityId) -> impl Iterator<Item = &CutCopy> {
        self.copies.iter().filter(move |copy| copy.piece() == piece)
    }

    /// Copy orientation has not been applied to geometry by this structural plan.
    #[must_use]
    pub const fn geometric_validation(&self) -> GeometricValidation {
        GeometricValidation::DeferredToG2
    }
}
