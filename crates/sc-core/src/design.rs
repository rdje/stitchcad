//! Owned Design structural/input components; selected input names; semantic state and execution follow.
use core::fmt;
use std::collections::BTreeMap;

use crate::{
    command::DesignRevision,
    ontology::{CutPlan, CutPlanError, EntityId, IdentityLedger, PieceInventory},
    value::CanonicalInputs,
};
use sc_units::Count;

/// Exact external SizeSet reference authored by a Design; construction performs no registry lookup.
/// This reference owns neither member labels nor a chart and implies no instantiation readiness.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DesignSizeSetReference {
    id: EntityId,
    revision: Count,
}
impl DesignSizeSetReference {
    /// Retain the explicit identity/revision, including zero and the terminal Count revision.
    #[must_use]
    pub const fn new(id: EntityId, revision: Count) -> Self {
        Self { id, revision }
    }
    /// Exact external set identity, without resolving a label or selecting a replacement.
    #[must_use]
    pub const fn id(self) -> EntityId {
        self.id
    }
    /// Exact authored set revision, independent of the containing Design revision.
    #[must_use]
    pub const fn revision(self) -> Count {
        self.revision
    }
}

mod closures;
mod constructions;
mod garment;
mod gathers;
mod hems;
mod host;
mod inputs;
mod layers;
mod parameters;
mod sewing;
mod sewing_state;
mod topology;
pub use closures::{
    DesignClosureDeclaration, DesignClosureDeclarationError, DesignClosureDeclarationRole,
    DesignClosureDeclarationSource,
};
pub use constructions::{
    DesignConstructionDeclaration, DesignConstructionDeclarationError,
    DesignConstructionDeclarationRole, DesignConstructionDeclarationSource,
};
pub use garment::{
    DesignGarment, DesignGarmentDefinition, DesignGarmentError, DesignGarmentPosition,
};
pub use gathers::{DesignGatherDeclaration, DesignGatherDeclarationError};
pub use hems::{
    DesignHemDeclaration, DesignHemDeclarationError, DesignHemDeclarationRole,
    DesignHemDeclarationSource,
};
pub use host::{
    DesignFormulaHost, DesignHostEntity, DesignHostEntityKind, DesignMetadata, DesignMetadataError,
    DesignMetadataHost, DesignReferenceHost, DesignSizeSetHost, DesignSourceReferences,
};
pub use inputs::{
    DesignFormulaError, DesignInputs, DesignInputsDefinition, DesignInputsError,
    DesignInputsPosition,
};
pub use layers::DesignLayerOffsetDeclaration;
pub use parameters::{
    DesignParameter, DesignParameterDefinition, DesignParameterError, DesignParameterTarget,
    DesignParameters,
};
pub use sewing::{DesignSewingAmount, DesignSewingAmountSource, DesignSewingDeclarationError};
pub use sewing_state::{
    DesignSewing, DesignSewingDefinition, DesignSewingError, DesignSewingPosition,
};
pub use topology::{DesignPieceTopology, DesignTopologyReadiness};

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
        let structure = Self { definition };
        let mut seen = BTreeMap::new();
        for (identity, position) in structure.identity_positions() {
            if let Some(first) = seen.insert(identity, position) {
                return Err(DesignStructureError::DuplicateIdentity {
                    identity,
                    first,
                    duplicate: position,
                });
            }
        }
        let definition = structure.definition();
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
        Ok(structure)
    }

    // Canonical structural census reused by containing immutable owners without cloning records.
    pub(crate) fn identity_positions(
        &self,
    ) -> impl Iterator<Item = (EntityId, DesignStructurePosition)> + '_ {
        core::iter::once((self.id(), DesignStructurePosition::Design))
            .chain(
                self.inputs()
                    .lengths()
                    .iter()
                    .enumerate()
                    .map(|(index, value)| (value.id(), DesignStructurePosition::Length { index })),
            )
            .chain(
                self.inputs()
                    .scalars()
                    .iter()
                    .enumerate()
                    .map(|(index, value)| (value.id(), DesignStructurePosition::Scalar { index })),
            )
            .chain(
                self.pieces()
                    .pieces()
                    .iter()
                    .enumerate()
                    .map(|(index, value)| (value.id(), DesignStructurePosition::Piece { index })),
            )
            .chain(
                self.cut_plan()
                    .copies()
                    .iter()
                    .enumerate()
                    .map(|(index, value)| (value.id(), DesignStructurePosition::CutCopy { index })),
            )
            .chain(
                self.topology()
                    .journal()
                    .iter()
                    .enumerate()
                    .map(|(index, value)| {
                        (
                            value.operation(),
                            DesignStructurePosition::Operation { index },
                        )
                    }),
            )
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
