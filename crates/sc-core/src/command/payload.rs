//! Canonical mutation intents; concrete payloads determine their own command identity.
use super::{AddNotch, CloseDart, CommandId, CommandMutationContext};
use crate::{
    ontology::{EntityId, MaterialAssignment},
    value::LengthDeclaration,
};
use core::fmt;
use std::collections::BTreeMap;

/// Ordered, nonempty, duplicate-free target pieces; existence/currentness is checked at execution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PieceSelection(Vec<EntityId>);

/// Actual invalid target inventory, before any command can execute.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceSelectionError {
    /// No target piece was supplied.
    NoPieces,
    /// A piece occurs twice; both positions are zero-based in the original input order.
    DuplicatePiece {
        /// Actual repeated piece identity.
        piece: EntityId,
        /// First occurrence.
        first: usize,
        /// First repeated occurrence.
        duplicate: usize,
    },
}

impl fmt::Display for PieceSelectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoPieces => f.write_str("piece selection requires at least one piece"),
            Self::DuplicatePiece {
                piece,
                first,
                duplicate,
            } => write!(
                f,
                "piece {piece} repeats at position {duplicate}, first supplied at {first}"
            ),
        }
    }
}
impl std::error::Error for PieceSelectionError {}

impl PieceSelection {
    /// Validate inventory without sorting, silently deduplicating or looking up a registry.
    /// # Errors
    /// Returns NoPieces or the first actual DuplicatePiece in authored order.
    pub fn new(pieces: Vec<EntityId>) -> Result<Self, PieceSelectionError> {
        if pieces.is_empty() {
            return Err(PieceSelectionError::NoPieces);
        }
        let mut positions = BTreeMap::new();
        for (position, piece) in pieces.iter().copied().enumerate() {
            if let Some(first) = positions.insert(piece, position) {
                return Err(PieceSelectionError::DuplicatePiece {
                    piece,
                    first,
                    duplicate: position,
                });
            }
        }
        Ok(Self(pieces))
    }

    /// Exact validated input order and identities.
    #[must_use]
    pub fn pieces(&self) -> &[EntityId] {
        &self.0
    }
}

/// Replace the canonical value/state declaration associated with a named measurement.
/// Metadata binding, stable declaration identity and proposed source validation need the registry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChangeMeasurement {
    measurement: EntityId,
    replacement: LengthDeclaration,
}

impl ChangeMeasurement {
    /// Preserve the actual measurement target and structurally validated canonical replacement.
    /// No numeric value is extracted from Unknown or Derived state.
    #[must_use]
    pub const fn new(measurement: EntityId, replacement: LengthDeclaration) -> Self {
        Self {
            measurement,
            replacement,
        }
    }

    /// Measurement metadata identity, distinct from the canonical declaration identity.
    #[must_use]
    pub const fn measurement(&self) -> EntityId {
        self.measurement
    }

    /// Complete replacement, retaining its declaration/source/state/provenance once.
    #[must_use]
    pub const fn replacement(&self) -> &LengthDeclaration {
        &self.replacement
    }
}

/// Set the same explicit material intent on one or more pieces, without resolving material truth.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssignMaterial {
    pieces: PieceSelection,
    material: MaterialAssignment,
}

/// An unresolved material intent omitted its required nonblank reason.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssignMaterialError {
    reason: String,
}

impl AssignMaterialError {
    /// Exact refused reason, including its original whitespace.
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}
impl fmt::Display for AssignMaterialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AssignMaterial requires a nonblank unresolved reason, given {:?}",
            self.reason
        )
    }
}
impl std::error::Error for AssignMaterialError {}

impl AssignMaterial {
    /// Reuse the same local reason invariant as canonical Piece construction.
    /// # Errors
    /// Returns the original blank unresolved reason. Assigned material identity is not looked up.
    pub fn new(
        pieces: PieceSelection,
        material: MaterialAssignment,
    ) -> Result<Self, AssignMaterialError> {
        match &material {
            MaterialAssignment::Unresolved(reason) if !material.has_required_reason() => {
                Err(AssignMaterialError {
                    reason: reason.clone(),
                })
            }
            _ => Ok(Self { pieces, material }),
        }
    }

    /// Validated piece selection in original input order.
    #[must_use]
    pub const fn pieces(&self) -> &PieceSelection {
        &self.pieces
    }

    /// Exact assigned identity or explicit unresolved reason.
    #[must_use]
    pub const fn material(&self) -> &MaterialAssignment {
        &self.material
    }
}

/// Implemented canonical mutation payload schemas; this is not handler availability.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MutationPayload {
    /// Set a measurement's canonical length declaration.
    ChangeMeasurement(ChangeMeasurement),
    /// Set explicit material intent on the selected pieces.
    AssignMaterial(AssignMaterial),
    /// Execute the closure referenced by an actual canonical dart.
    CloseDart(CloseDart),
    /// Create a semantic notch using a bus-supplied fresh identity.
    AddNotch(AddNotch),
}

impl MutationPayload {
    /// Identity derived from the concrete variant; callers cannot substitute another command.
    #[must_use]
    pub const fn command_id(&self) -> CommandId {
        match self {
            Self::ChangeMeasurement(_) => CommandId::ChangeMeasurement,
            Self::AssignMaterial(_) => CommandId::AssignMaterial,
            Self::CloseDart(_) => CommandId::CloseDart,
            Self::AddNotch(_) => CommandId::AddNotch,
        }
    }
}

/// A concrete mutation intent with all mandatory request metadata, before execution validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandMutation {
    context: CommandMutationContext,
    payload: MutationPayload,
}

impl CommandMutation {
    /// Bind the supplied context and canonical intent without authenticating or executing it.
    #[must_use]
    pub const fn new(context: CommandMutationContext, payload: MutationPayload) -> Self {
        Self { context, payload }
    }

    /// Original immutable actor/design/revision/retry/group metadata.
    #[must_use]
    pub const fn context(&self) -> &CommandMutationContext {
        &self.context
    }

    /// Actual concrete typed payload.
    #[must_use]
    pub const fn payload(&self) -> &MutationPayload {
        &self.payload
    }

    /// Command identity tied to its actual payload, not another caller-supplied assertion.
    #[must_use]
    pub const fn command_id(&self) -> CommandId {
        self.payload.command_id()
    }
}
