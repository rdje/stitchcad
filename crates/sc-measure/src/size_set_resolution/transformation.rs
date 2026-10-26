//! Authored, state-bearing transformation intent over actual current SizeSet inputs; no application.
use crate::{SizeLabel, SizeSetInputError, SizeSetInputs, SizeSetReference};
use core::fmt;
use sc_core::{ontology::EntityId, value::ValueProvenanceValidation};

mod validate;

/// Explicit transformation domain; labels never select an operation implicitly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SizeSetTransformationKind {
    /// Rename explicitly selected existing members, preserving member identities and order.
    LabelMapping,
    /// Select an existing member as the proposed base.
    BaseSubstitution,
}
/// One authored member-to-designation correspondence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeLabelMapping {
    /// Existing source member identity, independent of its label or ordinal.
    pub member: EntityId,
    /// Exact proposed designation; no numeric or token interpretation.
    pub label: SizeLabel,
}
/// Typed authored payload; it records intent without modifying source membership or chart.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SizeSetTransformationValue {
    /// Explicit nonempty partial or complete mapping in authored order.
    LabelMapping(Vec<SizeLabelMapping>),
    /// Explicit existing member identity, even when already the base.
    BaseSubstitution(EntityId),
}
impl SizeSetTransformationValue {
    /// Exact operation domain, independent of label spelling and numeric magnitude.
    #[must_use]
    pub const fn kind(&self) -> SizeSetTransformationKind {
        match self {
            Self::LabelMapping(_) => SizeSetTransformationKind::LabelMapping,
            Self::BaseSubstitution(_) => SizeSetTransformationKind::BaseSubstitution,
        }
    }
}
/// Authored transformation claim; observation and evaluation states cannot contain a cached payload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SizeSetTransformationState {
    /// Claimed known payload, with nonempty distinct evidence identities.
    Known {
        /// Proposed operation.
        value: SizeSetTransformationValue,
        /// Scoped evidence references; their existence and truth are checked elsewhere.
        evidence: Vec<EntityId>,
    },
    /// Proposed operation on an explicit human assumption.
    Assumed {
        /// Proposed operation.
        value: SizeSetTransformationValue,
        /// Original assumption record.
        assumption: EntityId,
    },
    /// Observation is needed; no authored operation value.
    Unknown {
        /// Original observation request.
        observation: EntityId,
    },
    /// Explicit overridable proposed operation.
    Preference {
        /// Proposed operation.
        value: SizeSetTransformationValue,
        /// Original preference provenance.
        provenance: EntityId,
    },
    /// Formula is the sole result source; no independent result is stored.
    Derived {
        /// Original formula declaration; its type and execution remain recipe obligations.
        formula: EntityId,
    },
}
/// Editable transformation declaration, separate from the immutable source SizeSet.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeSetTransformationDefinition {
    /// Stable transformation identity, distinct from supplied input records and source provenance.
    pub id: EntityId,
    /// Exact source SizeSet identity/revision.
    pub reference: SizeSetReference,
    /// Authored provenance record; no truth or registry existence inferred from its identity.
    pub source: EntityId,
    /// Explicit operation domain, available even without an authored value.
    pub kind: SizeSetTransformationKind,
    /// Original state and evidence/source references.
    pub state: SizeSetTransformationState,
}
/// Local structural refusal, preserving actual current input failures and conflicting identities.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SizeSetTransformationError {
    /// Original current input failure.
    Inputs(Box<SizeSetInputError>),
    /// Declaration names another source identity or revision.
    SourceMismatch {
        /// Authored source pin.
        expected: SizeSetReference,
        /// Actual supplied input pin.
        actual: SizeSetReference,
    },
    /// Transformation identity aliases a supplied source record or its own source provenance.
    DuplicateIdentity(EntityId),
    /// Known-state evidence inventory is empty.
    NoEvidence(EntityId),
    /// Known-state evidence repeats an identity.
    DuplicateEvidence(EntityId),
    /// Authored payload differs from the explicitly declared domain.
    KindMismatch {
        /// Declared domain.
        expected: SizeSetTransformationKind,
        /// Actual payload domain.
        actual: SizeSetTransformationKind,
    },
    /// Label mapping has no authored entries.
    EmptyMapping,
    /// Label mapping repeats a member identity.
    DuplicateMember(EntityId),
    /// Mapping or proposed base names no actual source member.
    MissingMember(EntityId),
    /// Proposed mapped and unchanged labels would collide.
    DuplicateLabel {
        /// Earlier member in original membership order.
        first: EntityId,
        /// Later member in original membership order.
        second: EntityId,
        /// Exact conflicting designation.
        label: SizeLabel,
    },
}
impl fmt::Display for SizeSetTransformationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "size set transformation: {self:?}")
    }
}
impl std::error::Error for SizeSetTransformationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Inputs(issue) => Some(issue),
            _ => None,
        }
    }
}
/// Reading intent cannot manufacture observation or formula results.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SizeSetTransformationReadError {
    /// This declaration requires the named observation.
    RequiresObservation {
        /// Unresolved transformation.
        transformation: EntityId,
        /// Original request.
        observation: EntityId,
    },
    /// This declaration requires evaluation of its sole result source.
    RequiresEvaluation {
        /// Derived transformation.
        transformation: EntityId,
        /// Original formula.
        formula: EntityId,
    },
}
impl fmt::Display for SizeSetTransformationReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "size set transformation value: {self:?}")
    }
}
impl std::error::Error for SizeSetTransformationReadError {}
/// Immutable locally validated declaration, not an applied transformation or approval.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeSetTransformation {
    definition: SizeSetTransformationDefinition,
}
impl SizeSetTransformation {
    /// Validate declaration identity, source pin, evidence inventory and actual authored payload.
    /// # Errors
    /// Returns current input, source/identity/evidence, domain or member/label errors without defaults.
    pub fn new(
        definition: SizeSetTransformationDefinition,
        inputs: SizeSetInputs<'_>,
    ) -> Result<Self, SizeSetTransformationError> {
        let result = Self { definition };
        result.validate_current(inputs)?;
        Ok(result)
    }
    /// Original declaration and authored order, without unchecked mutation.
    #[must_use]
    pub const fn definition(&self) -> &SizeSetTransformationDefinition {
        &self.definition
    }
    /// Evidence truth, scope, human actor and registry binding remain separate obligations.
    #[must_use]
    pub const fn provenance_validation(&self) -> ValueProvenanceValidation {
        ValueProvenanceValidation::DeferredToDesignAndG4
    }
    /// Read authored intent without applying it or upgrading its state.
    /// # Errors
    /// Returns the original required observation or formula for a value-less state.
    pub fn authored_value(
        &self,
    ) -> Result<&SizeSetTransformationValue, SizeSetTransformationReadError> {
        match &self.definition.state {
            SizeSetTransformationState::Known { value, .. }
            | SizeSetTransformationState::Assumed { value, .. }
            | SizeSetTransformationState::Preference { value, .. } => Ok(value),
            SizeSetTransformationState::Unknown { observation } => {
                Err(SizeSetTransformationReadError::RequiresObservation {
                    transformation: self.definition.id,
                    observation: *observation,
                })
            }
            SizeSetTransformationState::Derived { formula } => {
                Err(SizeSetTransformationReadError::RequiresEvaluation {
                    transformation: self.definition.id,
                    formula: *formula,
                })
            }
        }
    }
}
