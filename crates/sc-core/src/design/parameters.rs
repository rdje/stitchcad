//! Owned named Design parameters; canonical values remain singly owned by CanonicalInputs.
use core::fmt;
use std::collections::BTreeMap;

use crate::{
    name::MachineToken,
    ontology::EntityId,
    recipe::{
        FormulaDeclaration, FormulaInputOrigin, FormulaScalarInput, FormulaScalarInputAdapterRule,
        FormulaScalarInputOrigin,
    },
    value::{CanonicalInputs, ScalarInputKind},
};

/// Pinned canonical target representation; a general Length scalar never becomes a dedicated length.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DesignParameterTarget {
    /// Existing dedicated length declaration.
    Length(EntityId),
    /// Existing general scalar declaration with an explicit expected kind.
    Scalar {
        /// Stable target identity.
        declaration: EntityId,
        /// Kind expected by this metadata binding, including when no authored value exists.
        kind: ScalarInputKind,
    },
}

/// Editable named parameter metadata; it owns no independent numeric value or uncertainty state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DesignParameterDefinition {
    /// Stable metadata identity, distinct from canonical declaration identities.
    pub id: EntityId,
    /// Authored exact machine token; no localized label is inferred.
    pub token: MachineToken,
    /// Exact canonical target family, identity and scalar annotation.
    pub target: DesignParameterTarget,
}

/// Parameter identity/name/current-target refusal, without invented replacement or source truth.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DesignParameterError {
    /// One metadata identity occurs twice, with original zero-based positions.
    DuplicateIdentity {
        /// Repeated metadata id.
        identity: EntityId,
        /// First occurrence.
        first: usize,
        /// Repeated occurrence.
        duplicate: usize,
    },
    /// Parameter metadata aliases a canonical length or general-scalar declaration identity.
    CanonicalIdentityCollision {
        /// Aliased identity.
        identity: EntityId,
        /// Original parameter position.
        index: usize,
    },
    /// Two parameter tokens are equal, including equal targets with distinct metadata identities.
    AmbiguousName {
        /// First authored position.
        first: usize,
        /// Repeated authored position; callers can explicitly inspect its token.
        duplicate: usize,
    },
    /// Parameter metadata attempts to bind a language-owned reserved input token.
    ReservedName {
        /// Metadata carrying the token; the token is available through its definition.
        parameter: EntityId,
    },
    /// Dedicated target is absent; no general Length scalar substitutes for it.
    MissingLength {
        /// Parameter carrying the binding.
        parameter: EntityId,
        /// Absent dedicated length id.
        declaration: EntityId,
    },
    /// General-scalar target is absent; no dedicated length substitutes for it.
    MissingScalar {
        /// Parameter carrying the binding.
        parameter: EntityId,
        /// Absent general-scalar id.
        declaration: EntityId,
    },
    /// Existing general-scalar adapter rejects the actual current target/kind match.
    ScalarTarget {
        /// Parameter carrying the binding.
        parameter: EntityId,
        /// Original typed adapter refusal with actual expected/current values.
        rule: FormulaScalarInputAdapterRule,
    },
}
impl fmt::Display for DesignParameterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Design parameter: {self:?}")
    }
}
impl std::error::Error for DesignParameterError {}

/// Immutable parameter metadata constructed only as part of a complete validated registry.
#[derive(Clone, PartialEq, Eq)]
pub struct DesignParameter {
    definition: DesignParameterDefinition,
}
impl fmt::Debug for DesignParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignParameter")
            .field("id", &self.id())
            .field("target", &self.definition.target)
            .finish_non_exhaustive()
    }
}
impl DesignParameter {
    /// Stable metadata identity; this is not its canonical value declaration id.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }

    /// Original metadata; a shared borrow cannot rebind its name or target.
    #[must_use]
    pub const fn definition(&self) -> &DesignParameterDefinition {
        &self.definition
    }

    /// Resolve the exact held target against current canonical inputs, without reading a value.
    ///
    /// Dedicated lengths retain an actual borrowed record. General metadata retains its pinned
    /// identity/kind after the existing scalar adapter checks the current canonical object. Source,
    /// state and provenance remain on that object, with no extra cache or inferred source identity.
    /// This checks only the selected target; the registry's complete current check also rechecks
    /// metadata identity disjointness after canonical inventory replacement.
    ///
    /// # Errors
    /// Returns the missing target family/id or actual current scalar-kind mismatch.
    pub fn declaration<'a>(
        &'a self,
        inputs: &'a CanonicalInputs,
    ) -> Result<FormulaDeclaration<'a>, DesignParameterError> {
        let parameter = self.id();
        match self.definition.target {
            DesignParameterTarget::Length(declaration) => {
                let canonical =
                    inputs
                        .length(declaration)
                        .ok_or(DesignParameterError::MissingLength {
                            parameter,
                            declaration,
                        })?;
                Ok(FormulaDeclaration::length_input(
                    &self.definition.token,
                    FormulaInputOrigin::Parameter,
                    parameter,
                    canonical,
                ))
            }
            DesignParameterTarget::Scalar { declaration, kind } => {
                let canonical =
                    inputs
                        .scalar(declaration)
                        .ok_or(DesignParameterError::MissingScalar {
                            parameter,
                            declaration,
                        })?;
                let metadata = FormulaDeclaration::input(
                    &self.definition.token,
                    FormulaScalarInputOrigin::Parameter,
                    parameter,
                    declaration,
                    kind.into(),
                );
                FormulaScalarInput::from_declaration(metadata, canonical)
                    .map(|matched| matched.metadata())
                    .map_err(|error| DesignParameterError::ScalarTarget {
                        parameter,
                        rule: error.rule(),
                    })
            }
        }
    }
}

/// Immutable ordered Design parameters with unique metadata ids/tokens and pinned canonical targets.
///
/// This registry validates its supplied canonical context, not other Design entities, selected
/// Measurement/Ease/Profile/Material scopes, source/evidence facts or release readiness.
///
/// ```compile_fail
/// fn clear(parameters: &mut sc_core::design::DesignParameters) { parameters.parameters.clear(); }
/// ```
/// ```compile_fail
/// fn rebind(parameters: &mut sc_core::design::DesignParameters) {
///     parameters.parameters()[0].definition().target =
///         sc_core::design::DesignParameterTarget::Length(sc_core::ontology::EntityId::from_bits(1));
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct DesignParameters {
    parameters: Vec<DesignParameter>,
}
impl fmt::Debug for DesignParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignParameters")
            .field("parameter_count", &self.parameters.len())
            .finish_non_exhaustive()
    }
}
impl DesignParameters {
    /// Validate complete authored metadata before publishing any immutable parameter.
    ///
    /// Identity/name checks visit authored order before target resolution. Reserved inputs cannot
    /// be rebound. Empty drafts and distinct names sharing one declaration are legal. No target,
    /// value, token or source is generated, and no cross-domain flat namespace is claimed here.
    ///
    /// # Errors
    /// Returns the first metadata identity/name refusal, then the first current target refusal.
    pub fn new(
        definitions: Vec<DesignParameterDefinition>,
        inputs: &CanonicalInputs,
    ) -> Result<Self, DesignParameterError> {
        let mut ids = BTreeMap::new();
        let mut names = BTreeMap::new();
        for (index, definition) in definitions.iter().enumerate() {
            if let Some(first) = ids.insert(definition.id, index) {
                return Err(DesignParameterError::DuplicateIdentity {
                    identity: definition.id,
                    first,
                    duplicate: index,
                });
            }
            if inputs.length(definition.id).is_some() || inputs.scalar(definition.id).is_some() {
                return Err(DesignParameterError::CanonicalIdentityCollision {
                    identity: definition.id,
                    index,
                });
            }
            if definition.token.is_reserved_input_name() {
                return Err(DesignParameterError::ReservedName {
                    parameter: definition.id,
                });
            }
            if let Some(first) = names.insert(definition.token.as_str(), index) {
                return Err(DesignParameterError::AmbiguousName {
                    first,
                    duplicate: index,
                });
            }
        }
        let parameters = Self {
            parameters: definitions
                .into_iter()
                .map(|definition| DesignParameter { definition })
                .collect(),
        };
        parameters.validate_current(inputs)?;
        Ok(parameters)
    }

    /// Revalidate all pinned targets against the actual supplied current canonical records.
    ///
    /// # Errors
    /// Returns the first missing-family/id or changed-kind refusal in authored parameter order.
    /// Canonical identity disjointness is checked again after same-id canonical inventory replacement.
    pub fn validate_current(&self, inputs: &CanonicalInputs) -> Result<(), DesignParameterError> {
        for (index, parameter) in self.parameters.iter().enumerate() {
            if inputs.length(parameter.id()).is_some() || inputs.scalar(parameter.id()).is_some() {
                return Err(DesignParameterError::CanonicalIdentityCollision {
                    identity: parameter.id(),
                    index,
                });
            }
            parameter.declaration(inputs)?;
        }
        Ok(())
    }

    /// Original immutable parameter order.
    #[must_use]
    pub fn parameters(&self) -> &[DesignParameter] {
        &self.parameters
    }

    /// Borrow canonical metadata by identity, without interpreting its label or position.
    #[must_use]
    pub fn parameter(&self, identity: EntityId) -> Option<&DesignParameter> {
        self.parameters
            .iter()
            .find(|parameter| parameter.id() == identity)
    }
}
