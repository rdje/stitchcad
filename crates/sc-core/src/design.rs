//! Owned structural Design content. Recipe, metadata, garment registries and execution follow.
use core::fmt;
use std::collections::BTreeMap;

use crate::{
    command::DesignRevision,
    ontology::{CutPlan, CutPlanError, EntityId, IdentityLedger, PieceInventory},
    value::CanonicalInputs,
};

mod parameters;
pub use parameters::{
    DesignParameter, DesignParameterDefinition, DesignParameterError, DesignParameterTarget,
    DesignParameters,
};

/// Explicit structural input, with no inferred identity, revision, copy or topology registration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DesignStructureDefinition {
    /// Stable identity of the containing Design.
    pub id: EntityId,
    /// Exact revision represented by this state; constructing it does not commit a command.
    pub revision: DesignRevision,
    /// Original dedicated-length and general-scalar declarations.
    pub inputs: CanonicalInputs,
    /// Complete pair-consistent pattern Piece inventory.
    pub pieces: PieceInventory,
    /// Explicit physical copies, revalidated against these Pieces at construction.
    pub cut_plan: CutPlan,
    /// Exact topology history and explicit point registrations, including unresolved references.
    pub topology: IdentityLedger,
}

/// Position of an owned entity in structural content; referenced identities are not new owners.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DesignStructurePosition {
    /// Containing Design identity.
    Design,
    /// Dedicated length declaration in authored order.
    Length {
        /// Zero-based declaration position.
        index: usize,
    },
    /// General-scalar declaration in authored order.
    Scalar {
        /// Zero-based declaration position.
        index: usize,
    },
    /// Pattern Piece in authored order.
    Piece {
        /// Zero-based Piece position.
        index: usize,
    },
    /// Physical copy in authored order.
    CutCopy {
        /// Zero-based copy position.
        index: usize,
    },
    /// Topology operation in journal order, including operations whose edges were retired.
    Operation {
        /// Zero-based journal position.
        index: usize,
    },
}

/// Refusal to combine independently valid structural components into this owned state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DesignStructureError {
    /// One identity has two owners; first/duplicate positions retain the deterministic census order.
    DuplicateIdentity {
        /// Repeated entity identity.
        identity: EntityId,
        /// First owner.
        first: DesignStructurePosition,
        /// Repeated owner.
        duplicate: DesignStructurePosition,
    },
    /// Copy plan does not match the actual current Piece inventory.
    CutPlan(CutPlanError),
}
impl fmt::Display for DesignStructureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity { identity, first, duplicate } => write!(f, "structural identity {identity} repeats at {duplicate:?}, first supplied at {first:?}"),
            Self::CutPlan(error) => write!(f, "structural copy plan: {error}"),
        }
    }
}
impl std::error::Error for DesignStructureError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DuplicateIdentity { .. } => None,
            Self::CutPlan(error) => Some(error),
        }
    }
}

/// Immutable structural component of a Design, preserving current owned records and history.
///
/// This is not the complete Design aggregate or a readiness verdict. Historical Piece references
/// remain held references, including repairs after topology edits. No registration is added, and
/// geometry, source/evidence truth and recipe/garment readiness are not inferred.
///
/// ```compile_fail
/// fn erase_topology(state: &mut sc_core::design::DesignStructure) {
///     state.definition.topology = sc_core::ontology::IdentityLedger::new();
/// }
/// ```
/// ```compile_fail
/// fn change_topology(state: &mut sc_core::design::DesignStructure) {
///     let _mutable: &mut sc_core::ontology::IdentityLedger = &mut state.definition().topology;
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DesignStructure {
    definition: DesignStructureDefinition,
}

impl DesignStructure {
    /// Own the exact components after identity and actual Piece/copy consistency checks.
    ///
    /// The identity census runs Design, lengths, scalars, Pieces, copies, then journal operations,
    /// each in original order. Edge creators, source/evidence ids and registration owners are
    /// references, so their legitimate repeated uses are not treated as duplicate declarations.
    /// An explicitly empty structural draft is legal. No Piece is reconstructed against the current
    /// ledger: doing so would discard its historical references or refuse inspectable repair state.
    ///
    /// # Errors
    /// Returns the first owned identity collision, then any current Piece/copy-plan inconsistency.
    pub fn new(definition: DesignStructureDefinition) -> Result<Self, DesignStructureError> {
        let mut seen = BTreeMap::new();
        let mut claim = |identity, position| {
            if let Some(first) = seen.insert(identity, position) {
                Err(DesignStructureError::DuplicateIdentity {
                    identity,
                    first,
                    duplicate: position,
                })
            } else {
                Ok(())
            }
        };
        claim(definition.id, DesignStructurePosition::Design)?;
        for (index, value) in definition.inputs.lengths().iter().enumerate() {
            claim(value.id(), DesignStructurePosition::Length { index })?;
        }
        for (index, value) in definition.inputs.scalars().iter().enumerate() {
            claim(value.id(), DesignStructurePosition::Scalar { index })?;
        }
        for (index, value) in definition.pieces.pieces().iter().enumerate() {
            claim(value.id(), DesignStructurePosition::Piece { index })?;
        }
        for (index, value) in definition.cut_plan.copies().iter().enumerate() {
            claim(value.id(), DesignStructurePosition::CutCopy { index })?;
        }
        for (index, value) in definition.topology.journal().iter().enumerate() {
            claim(
                value.operation(),
                DesignStructurePosition::Operation { index },
            )?;
        }
        CutPlan::new(
            definition
                .cut_plan
                .copies()
                .iter()
                .map(|copy| *copy.definition())
                .collect(),
            definition.pieces.pieces(),
        )
        .map_err(DesignStructureError::CutPlan)?;
        Ok(Self { definition })
    }

    /// Exact owned structural input; this shared view grants no mutation.
    #[must_use]
    pub const fn definition(&self) -> &DesignStructureDefinition {
        &self.definition
    }
    /// Exact containing Design identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }
    /// Exact represented revision, without advancing it or checking a command precondition.
    #[must_use]
    pub const fn revision(&self) -> &DesignRevision {
        &self.definition.revision
    }
    /// Original owned canonical declarations.
    #[must_use]
    pub const fn inputs(&self) -> &CanonicalInputs {
        &self.definition.inputs
    }
    /// Original complete Piece inventory.
    #[must_use]
    pub const fn pieces(&self) -> &PieceInventory {
        &self.definition.pieces
    }
    /// Original authored physical-copy plan, validated against this inventory.
    #[must_use]
    pub const fn cut_plan(&self) -> &CutPlan {
        &self.definition.cut_plan
    }
    /// Original topology journal and registrations; shared access cannot edit either.
    #[must_use]
    pub const fn topology(&self) -> &IdentityLedger {
        &self.definition.topology
    }
}
