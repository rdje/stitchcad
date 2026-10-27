//! Private composition of implemented structural/input components, before whole Design readiness.
use super::{
    DesignHostEntity, DesignMetadata, DesignMetadataError, DesignMetadataHost,
    DesignParameterError, DesignParameters, DesignStructure, DesignStructurePosition,
};
use crate::{
    command::DesignRevision,
    ontology::{EntityId, MaterialInventory, MaterialInventoryError},
    recipe::FormulaRecipeDocument,
    value::CanonicalInputs,
};
use core::fmt;
use std::collections::BTreeMap;

/// Exact implemented input components; this is not the complete Design schema.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DesignInputsDefinition<H> {
    /// Actual identity/revision, canonical inputs, Pieces, copies and topology owner.
    pub structure: DesignStructure,
    /// Actual named parameter metadata over the structure's canonical declarations.
    pub parameters: DesignParameters,
    /// Exact original recipe text and canonical syntax identity, without a static/execution proof.
    pub recipe: FormulaRecipeDocument,
    /// Actual Material inventory and canonical property targets.
    pub materials: MaterialInventory,
    /// Actual checked higher metadata owner; canonical declarations remain in structure only.
    pub sources: DesignMetadata<H>,
}

/// Actual original location in the complete implemented-component ownership census.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DesignInputsPosition {
    /// Actual structural identity, including canonical records and retired creating operations.
    Structure(DesignStructurePosition),
    /// Actual parameter metadata.
    Parameter {
        /// Zero-based authored parameter position.
        index: usize,
    },
    /// Actual Material metadata.
    Material {
        /// Zero-based authored Material position.
        index: usize,
    },
    /// Actual higher owned metadata category/position.
    Source(DesignHostEntity),
}

/// Actual input-composition refusal with original typed current-reference causes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DesignInputsError<E> {
    /// Independently valid components claim the same actual owned identity.
    DuplicateIdentity {
        /// Repeated identity.
        identity: EntityId,
        /// First actual ownership position.
        first: DesignInputsPosition,
        /// First repeated original position.
        duplicate: DesignInputsPosition,
    },
    /// Original parameter/current-canonical target refusal.
    Parameters(Box<DesignParameterError>),
    /// Original actual Material/current-property/Piece-assignment refusal.
    Materials(Box<MaterialInventoryError>),
    /// Original higher metadata/current-canonical refusal, preserving the concrete host cause.
    Sources(Box<DesignMetadataError<E>>),
}
impl<E: fmt::Display> fmt::Display for DesignInputsError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity {identity,first,duplicate}=>write!(f,"Design input identity {identity} repeats at {duplicate:?}, first supplied at {first:?}"),
            Self::Parameters(issue)=>write!(f,"Design input parameters: {issue}"),
            Self::Materials(issue)=>write!(f,"Design input Materials: {issue}"),
            Self::Sources(issue)=>write!(f,"Design input sources: {issue}"),
        }
    }
}
impl<E: std::error::Error + 'static> std::error::Error for DesignInputsError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DuplicateIdentity { .. } => None,
            Self::Parameters(issue) => Some(issue.as_ref()),
            Self::Materials(issue) => Some(issue.as_ref()),
            Self::Sources(issue) => Some(issue.as_ref()),
        }
    }
}

/// Immutable owner of the implemented structural/input components over one canonical inventory.
///
/// Selected source references, flat formula-name binding, sewing/garment state and readiness remain
/// separate work. Recipe admission is syntax/input ownership only. Held Piece topology references
/// remain inspectable after edits; no Piece is reconstructed and no value/history is generated.
///
/// ```compile_fail
/// fn edit<H: sc_core::design::DesignMetadataHost>(state: &mut sc_core::design::DesignInputs<H>) {
///     state.definition.recipe = state.definition.recipe.clone();
/// }
/// ```
/// ```compile_fail
/// fn edit<H: sc_core::design::DesignMetadataHost>(state: &mut sc_core::design::DesignInputs<H>) {
///     let _: &mut sc_core::value::CanonicalInputs = state.inputs();
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct DesignInputs<H> {
    definition: DesignInputsDefinition<H>,
}
impl<H> fmt::Debug for DesignInputs<H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignInputs").finish_non_exhaustive()
    }
}
impl<H: DesignMetadataHost> DesignInputs<H> {
    /// Own exact implemented components after complete id and current target/assignment checks.
    ///
    /// # Errors
    /// Returns first owned-id collision, then parameters, Materials/Pieces and source current causes.
    pub fn new(definition: DesignInputsDefinition<H>) -> Result<Self, DesignInputsError<H::Error>> {
        let inputs = Self { definition };
        inputs.validate_current()?;
        Ok(inputs)
    }
    /// Exact original construction data; shared access grants no mutator.
    #[must_use]
    pub const fn definition(&self) -> &DesignInputsDefinition<H> {
        &self.definition
    }
    /// Actual containing Design identity, without allocating a replacement.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.structure().id()
    }
    /// Exact actual Design revision, without advancing a command counter.
    #[must_use]
    pub const fn revision(&self) -> &DesignRevision {
        self.structure().revision()
    }
    /// Original owned structural component, including held topology repair state.
    #[must_use]
    pub const fn structure(&self) -> &DesignStructure {
        &self.definition.structure
    }
    /// The single canonical declaration owner in this state.
    #[must_use]
    pub const fn inputs(&self) -> &CanonicalInputs {
        self.structure().inputs()
    }
    /// Original exact parameter registry.
    #[must_use]
    pub const fn parameters(&self) -> &DesignParameters {
        &self.definition.parameters
    }
    /// Original exact authored recipe and canonical syntax identity.
    #[must_use]
    pub const fn recipe(&self) -> &FormulaRecipeDocument {
        &self.definition.recipe
    }
    /// Original actual Material inventory, without physical/evidence certification.
    #[must_use]
    pub const fn materials(&self) -> &MaterialInventory {
        &self.definition.materials
    }
    /// Original actual metadata host; shared access cannot replace source records.
    #[must_use]
    pub const fn sources(&self) -> &DesignMetadata<H> {
        &self.definition.sources
    }
    /// Complete actual ownership positions: structure, parameters, Materials, then higher metadata.
    /// Referenced source/evidence/formula/assignment ids remain uses, not additional owners.
    #[must_use]
    pub fn identities(&self) -> Vec<(EntityId, DesignInputsPosition)> {
        self.structure()
            .identity_positions()
            .map(|(id, position)| (id, DesignInputsPosition::Structure(position)))
            .chain(
                self.parameters()
                    .parameters()
                    .iter()
                    .enumerate()
                    .map(|(index, parameter)| {
                        (parameter.id(), DesignInputsPosition::Parameter { index })
                    }),
            )
            .chain(
                self.materials()
                    .materials()
                    .iter()
                    .enumerate()
                    .map(|(index, material)| {
                        (material.id(), DesignInputsPosition::Material { index })
                    }),
            )
            .chain(
                self.sources()
                    .entities()
                    .iter()
                    .map(|entity| (entity.id, DesignInputsPosition::Source(*entity))),
            )
            .collect()
    }
    /// Recheck whole implemented ownership and actual current links against this state's canonical inputs.
    ///
    /// Identity checks precede parameters, complete Material/current Piece assignments, then complete
    /// higher sources. Original uncertain states and explicit unresolved assignments remain intact.
    /// No namespace/kind/evaluation, geometry, physical or release readiness follows acceptance.
    ///
    /// # Errors
    /// Returns first duplicate then original parameter, Material/assignment or current host refusal.
    pub fn validate_current(&self) -> Result<(), DesignInputsError<H::Error>> {
        let mut seen = BTreeMap::new();
        for (identity, position) in self.identities() {
            if let Some(first) = seen.insert(identity, position) {
                return Err(DesignInputsError::DuplicateIdentity {
                    identity,
                    first,
                    duplicate: position,
                });
            }
        }
        self.parameters()
            .validate_current(self.inputs())
            .map_err(|issue| DesignInputsError::Parameters(Box::new(issue)))?;
        self.materials()
            .assignments(self.structure().pieces(), self.inputs())
            .map_err(|issue| DesignInputsError::Materials(Box::new(issue)))?;
        self.sources()
            .validate_current(self.inputs())
            .map_err(|issue| DesignInputsError::Sources(Box::new(issue)))?;
        Ok(())
    }
}
