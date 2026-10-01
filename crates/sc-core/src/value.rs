//! Canonical length-valued inputs: authored state/provenance, never a numeric unknown fallback.
use crate::ontology::EntityId;
use core::fmt;
use sc_units::Length;
use std::collections::BTreeSet;

/// Authored state of a length declaration; referenced records still require their owner validation.
/// Unknown and derived inputs cannot independently carry a numeric result.
/// ```compile_fail
/// use sc_core::{ontology::EntityId, value::LengthState};
/// use sc_units::Length;
/// let unknown = LengthState::Unknown {
///     observation: EntityId::from_bits(1), value: Length::ZERO,
/// };
/// ```
/// ```compile_fail
/// use sc_core::{ontology::EntityId, value::LengthState};
/// use sc_units::Length;
/// let derived = LengthState::Derived {
///     formula: EntityId::from_bits(1), value: Length::ZERO,
/// };
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LengthState {
    /// Claimed known value with scoped-evidence record references, not independent evidence proof.
    Known {
        /// Authored value in internal micrometres.
        value: Length,
        /// Required nonempty distinct evidence identities.
        evidence: Vec<EntityId>,
    },
    /// Value declared on a recorded human assumption, without claiming measured truth.
    Assumed {
        /// Authored value.
        value: Length,
        /// Assumption record; its existence and human actor are later registry checks.
        assumption: EntityId,
    },
    /// Observation is needed; this state has no authored numeric value.
    Unknown {
        /// Observation request needed to obtain this fact.
        observation: EntityId,
    },
    /// Explicit overridable preference, not a fallback for a separately unknown fact.
    Preference {
        /// Authored preference value.
        value: Length,
        /// Provenance of the preference.
        provenance: EntityId,
    },
    /// Formula is the sole result source; no authored or cached numeric result.
    Derived {
        /// Formula declaration; recipe validates dependencies and propagates input state.
        formula: EntityId,
    },
}
/// Editable canonical input; no metadata consumer should duplicate its value/state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LengthDeclarationDefinition {
    /// Stable declaration identity.
    pub id: EntityId,
    /// Source record identity, validated separately from the authored state's claim.
    pub source: EntityId,
    /// Exactly one declared state with its required provenance.
    pub state: LengthState,
}
/// Immutable canonical length declaration; no public mutation bypasses structural validation.
/// ```compile_fail
/// fn change_value(value: &mut sc_core::value::LengthDeclaration) {
///     value.definition.state = sc_core::value::LengthState::Unknown {
///         observation: sc_core::ontology::EntityId::from_bits(1),
///     };
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LengthDeclaration {
    definition: LengthDeclarationDefinition,
}
/// Required known-state evidence failed its structural inventory contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LengthDeclarationError {
    /// A claimed known value has no evidence record references.
    NoEvidence(EntityId),
    /// A known value repeats one evidence identity.
    DuplicateEvidence {
        /// Declaration whose inventory is ambiguous.
        declaration: EntityId,
        /// Repeated evidence record.
        evidence: EntityId,
    },
}
impl fmt::Display for LengthDeclarationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoEvidence(id) => write!(f, "length declaration {id} needs evidence references"),
            Self::DuplicateEvidence {
                declaration,
                evidence,
            } => write!(
                f,
                "length declaration {declaration} repeats evidence {evidence}"
            ),
        }
    }
}
impl std::error::Error for LengthDeclarationError {}
/// Numeric query cannot manufacture the value its source still needs to obtain or compute.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LengthValueError {
    /// Requires observation; never replaced by zero or preference.
    RequiresObservation {
        /// Unresolved declaration.
        declaration: EntityId,
        /// Needed observation request.
        observation: EntityId,
    },
    /// Requires recipe evaluation and state propagation; no independent result is stored here.
    RequiresEvaluation {
        /// Derived declaration.
        declaration: EntityId,
        /// Sole formula result source.
        formula: EntityId,
    },
}
impl fmt::Display for LengthValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequiresObservation {
                declaration,
                observation,
            } => write!(
                f,
                "length declaration {declaration} requires observation {observation}"
            ),
            Self::RequiresEvaluation {
                declaration,
                formula,
            } => write!(
                f,
                "length declaration {declaration} requires formula {formula} evaluation"
            ),
        }
    }
}
impl std::error::Error for LengthValueError {}
/// Record existence, scope, truth, human attribution and artifact policy are not proved by an id.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueProvenanceValidation {
    /// Design/recipe registries and G4 scoped evidence/policy must validate this authored claim.
    DeferredToDesignAndG4,
}
impl LengthDeclaration {
    /// Validate the authored state's structural evidence inventory; no source truth is inferred.
    /// # Errors
    /// Returns [`LengthDeclarationError`] for missing or duplicate known-state evidence references.
    pub fn new(definition: LengthDeclarationDefinition) -> Result<Self, LengthDeclarationError> {
        if let LengthState::Known { evidence, .. } = &definition.state {
            if evidence.is_empty() {
                return Err(LengthDeclarationError::NoEvidence(definition.id));
            }
            let mut seen = BTreeSet::new();
            for id in evidence {
                if !seen.insert(*id) {
                    return Err(LengthDeclarationError::DuplicateEvidence {
                        declaration: definition.id,
                        evidence: *id,
                    });
                }
            }
        }
        Ok(Self { definition })
    }
    /// Stable canonical declaration identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }
    /// Authored source identity, without asserting its record exists or is valid.
    #[must_use]
    pub const fn source(&self) -> EntityId {
        self.definition.source
    }
    /// Authored canonical input, exposed without mutation.
    #[must_use]
    pub const fn definition(&self) -> &LengthDeclarationDefinition {
        &self.definition
    }
    /// Borrow the canonical authored state/provenance, independent of numeric availability.
    #[must_use]
    pub const fn state(&self) -> &LengthState {
        &self.definition.state
    }
    /// Read a present authored value; this is not evaluated, source-verified or export-approved.
    /// # Errors
    /// Returns [`LengthValueError`] naming the observation/formula when no authored value exists.
    pub fn authored_value(&self) -> Result<Length, LengthValueError> {
        match &self.definition.state {
            LengthState::Known { value, .. }
            | LengthState::Assumed { value, .. }
            | LengthState::Preference { value, .. } => Ok(*value),
            LengthState::Unknown { observation } => Err(LengthValueError::RequiresObservation {
                declaration: self.id(),
                observation: *observation,
            }),
            LengthState::Derived { formula } => Err(LengthValueError::RequiresEvaluation {
                declaration: self.id(),
                formula: *formula,
            }),
        }
    }
    /// Separate structural content from deferred registry/evidence/policy proof.
    #[must_use]
    pub const fn provenance_validation(&self) -> ValueProvenanceValidation {
        ValueProvenanceValidation::DeferredToDesignAndG4
    }
}
