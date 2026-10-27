//! Immutable, pair-consistent Piece collections for the future privately owned Design aggregate.
//!
//! Individual Pieces validate local content. This collection additionally checks the authored
//! companion relation; it does not resolve materials, current topology, geometry or release policy.

use core::fmt;
use std::collections::BTreeMap;

use super::{EntityId, Handedness, Mirroring, Piece};

/// Pieces in authored order, with unique identities and complete reciprocal pair-member metadata.
///
/// Constructing a new value does not mutate an existing Design or authorize a command. The future
/// aggregate owns replacement and current-reference checks. Shared access grants no mutator:
///
/// ```compile_fail
/// fn erase_piece(inventory: &mut sc_core::ontology::PieceInventory) {
///     inventory.pieces.clear();
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PieceInventory {
    pieces: Vec<Piece>,
}

/// A refusal in the complete authored Piece collection, retaining the actual identities and values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceInventoryError {
    /// A Piece identity occurs twice; positions are zero-based in authored order.
    DuplicatePiece {
        /// Repeated identity.
        piece: EntityId,
        /// First occurrence.
        first: usize,
        /// First repeated occurrence.
        duplicate: usize,
    },
    /// A separate pair member names no Piece in this collection.
    MissingCompanion {
        /// Member carrying the reference.
        piece: EntityId,
        /// Absent companion identity.
        companion: EntityId,
    },
    /// The named companion does not declare separate pair-member metadata.
    CompanionNotPairMember {
        /// Member carrying the reference.
        piece: EntityId,
        /// Named companion.
        companion: EntityId,
        /// Companion's actual cut mode, without inference from its label or quantity.
        actual: Mirroring,
    },
    /// The named companion points to a different Piece.
    NonReciprocalCompanion {
        /// Member requiring the reciprocal reference.
        piece: EntityId,
        /// Named companion.
        companion: EntityId,
        /// Companion's actual declared partner.
        actual: EntityId,
    },
    /// Both separate members declare the same hand.
    SameHandedness {
        /// First checked member.
        piece: EntityId,
        /// Named companion.
        companion: EntityId,
        /// Actual shared hand.
        handedness: Handedness,
    },
    /// Separate members request different numbers of physical copies.
    UnequalQuantities {
        /// First checked member.
        piece: EntityId,
        /// Named companion.
        companion: EntityId,
        /// This member's authored quantity.
        quantity: u32,
        /// Companion's authored quantity.
        companion_quantity: u32,
    },
}

impl fmt::Display for PieceInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicatePiece { piece, first, duplicate } => write!(f, "piece {piece} repeats at position {duplicate}, first supplied at {first}"),
            Self::MissingCompanion { piece, companion } => write!(f, "piece {piece} names absent companion {companion}"),
            Self::CompanionNotPairMember { piece, companion, actual } => write!(f, "piece {piece} names companion {companion} with cut mode {actual:?}"),
            Self::NonReciprocalCompanion { piece, companion, actual } => write!(f, "piece {piece} names companion {companion}, which instead names {actual}"),
            Self::SameHandedness { piece, companion, handedness } => write!(f, "pieces {piece} and {companion} both declare {handedness:?} handedness"),
            Self::UnequalQuantities { piece, companion, quantity, companion_quantity } => write!(f, "piece {piece} requests {quantity} copies but companion {companion} requests {companion_quantity}"),
        }
    }
}

impl std::error::Error for PieceInventoryError {}

impl PieceInventory {
    /// Check the complete supplied collection without sorting, deduplicating or inventing Pieces.
    ///
    /// An explicitly empty inventory is legal. Uniqueness is checked before companion relations;
    /// relations are then checked in authored order. Labels never supply a hand or partner.
    ///
    /// # Errors
    /// Returns the first duplicate identity, then the first invalid companion relation. A named
    /// companion must be a reciprocal separate member with opposite handedness and equal quantity.
    pub fn new(pieces: Vec<Piece>) -> Result<Self, PieceInventoryError> {
        let mut indexed = BTreeMap::new();
        for (position, piece) in pieces.iter().enumerate() {
            if let Some((first, _)) = indexed.insert(piece.id(), (position, piece)) {
                return Err(PieceInventoryError::DuplicatePiece {
                    piece: piece.id(),
                    first,
                    duplicate: position,
                });
            }
        }
        for piece in &pieces {
            let Mirroring::PairMember {
                handedness,
                companion,
            } = piece.definition().mirroring
            else {
                continue;
            };
            let (_, other) =
                indexed
                    .get(&companion)
                    .ok_or(PieceInventoryError::MissingCompanion {
                        piece: piece.id(),
                        companion,
                    })?;
            let Mirroring::PairMember {
                handedness: other_hand,
                companion: reciprocal,
            } = other.definition().mirroring
            else {
                return Err(PieceInventoryError::CompanionNotPairMember {
                    piece: piece.id(),
                    companion,
                    actual: other.definition().mirroring,
                });
            };
            if reciprocal != piece.id() {
                return Err(PieceInventoryError::NonReciprocalCompanion {
                    piece: piece.id(),
                    companion,
                    actual: reciprocal,
                });
            }
            if handedness == other_hand {
                return Err(PieceInventoryError::SameHandedness {
                    piece: piece.id(),
                    companion,
                    handedness,
                });
            }
            if piece.definition().quantity != other.definition().quantity {
                return Err(PieceInventoryError::UnequalQuantities {
                    piece: piece.id(),
                    companion,
                    quantity: piece.definition().quantity,
                    companion_quantity: other.definition().quantity,
                });
            }
        }
        Ok(Self { pieces })
    }

    // Exhaustive borrowed resource view; new owned fields require an accounting update.
    pub(crate) fn resource_pieces(&self) -> &[Piece] {
        let Self { pieces } = self;
        pieces
    }

    /// Original immutable Pieces in caller-authored order.
    #[must_use]
    pub fn pieces(&self) -> &[Piece] {
        &self.pieces
    }

    /// Borrow the canonical member by identity, never by ordinal or label.
    #[must_use]
    pub fn piece(&self, id: EntityId) -> Option<&Piece> {
        self.pieces.iter().find(|piece| piece.id() == id)
    }
}
