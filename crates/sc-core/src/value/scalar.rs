//! Canonical scalar input content, without registry, recipe execution or evidence-truth authority.
use super::{first_duplicate, ValueProvenanceValidation};
use crate::ontology::EntityId;
use core::fmt;
use sc_units::Length;

/// Declared scalar kind, available even when observation or evaluation is required.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScalarInputKind {
    /// Signed micrometres, within the Length scalar domain.
    Length,
    /// Signed raw microdegrees, preserving complete turns.
    Angle,
    /// Computed square micrometres; only a Derived declaration is permitted.
    Area,
    /// Signed parts per million.
    Ratio,
    /// Nonnegative signed-i64 count.
    Count,
    /// Boolean content, distinct from numeric zero/one.
    Boolean,
}

impl ScalarInputKind {
    /// Complete scalar declaration-kind inventory.
    pub const ALL: [Self; 6] = [
        Self::Length,
        Self::Angle,
        Self::Area,
        Self::Ratio,
        Self::Count,
        Self::Boolean,
    ];

    /// Stable kind token shared with formula annotations; no kind is inferred from magnitude.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Length => "length",
            Self::Angle => "angle",
            Self::Area => "area",
            Self::Ratio => "ratio",
            Self::Count => "count",
            Self::Boolean => "boolean",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum ScalarValue {
    Length(Length),
    Angle(i64),
    Ratio(i64),
    Count(i64),
    Boolean(bool),
}

/// Validated authored scalar in canonical internal units. Area has no authored-value constructor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ScalarInputValue(ScalarValue);

/// Authored count is outside its nonnegative signed-i64 domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScalarInputValueError {
    /// Actual negative count; no clamp or unsigned reinterpretation is applied.
    NegativeCount(i64),
}

impl fmt::Display for ScalarInputValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self::NegativeCount(value) = self;
        write!(f, "scalar input count {value} is below zero")
    }
}
impl std::error::Error for ScalarInputValueError {}

impl From<Length> for ScalarInputValue {
    fn from(value: Length) -> Self {
        Self(ScalarValue::Length(value))
    }
}

impl ScalarInputValue {
    /// Raw signed microdegrees; unlike an entity direction this does not normalize full turns.
    #[must_use]
    pub const fn from_angle_microdegrees(value: i64) -> Self {
        Self(ScalarValue::Angle(value))
    }

    /// Signed parts per million, including zero and values outside the zero-to-one interval.
    #[must_use]
    pub const fn from_ratio_ppm(value: i64) -> Self {
        Self(ScalarValue::Ratio(value))
    }

    /// Nonnegative canonical count without rounding.
    /// # Errors
    /// Returns the actual negative value; the i64 argument supplies the signed storage bound.
    pub const fn from_count(value: i64) -> Result<Self, ScalarInputValueError> {
        if value < 0 {
            Err(ScalarInputValueError::NegativeCount(value))
        } else {
            Ok(Self(ScalarValue::Count(value)))
        }
    }

    /// Boolean content without numeric coercion.
    #[must_use]
    pub const fn from_boolean(value: bool) -> Self {
        Self(ScalarValue::Boolean(value))
    }

    /// Actual authored kind; an Area value cannot be independently authored here.
    #[must_use]
    pub const fn kind(self) -> ScalarInputKind {
        match self.0 {
            ScalarValue::Length(_) => ScalarInputKind::Length,
            ScalarValue::Angle(_) => ScalarInputKind::Angle,
            ScalarValue::Ratio(_) => ScalarInputKind::Ratio,
            ScalarValue::Count(_) => ScalarInputKind::Count,
            ScalarValue::Boolean(_) => ScalarInputKind::Boolean,
        }
    }

    /// Canonical signed numeric integer, or None for Boolean; zero is a present value.
    #[must_use]
    pub const fn numeric_integer(self) -> Option<i64> {
        match self.0 {
            ScalarValue::Length(value) => Some(value.as_micrometres()),
            ScalarValue::Angle(value) | ScalarValue::Ratio(value) | ScalarValue::Count(value) => {
                Some(value)
            }
            ScalarValue::Boolean(_) => None,
        }
    }

    /// Actual Boolean, or None for a numeric kind; false is a present value.
    #[must_use]
    pub const fn boolean(self) -> Option<bool> {
        match self.0 {
            ScalarValue::Boolean(value) => Some(value),
            _ => None,
        }
    }
}

/// Closed authored-state label, without copying a value or provenance inventory.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScalarInputStateTag {
    /// Claimed fact with scoped-evidence identities.
    Known,
    /// Recorded human assumption.
    Assumed,
    /// Observation still required.
    Unknown,
    /// Explicit overridable preference.
    Preference,
    /// Sole formula source, without an authored result.
    Derived,
}

/// Canonical scalar state and its required provenance. Unknown/Derived have no value field.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScalarInputState {
    /// Claimed known value; construction requires nonempty distinct evidence references.
    Known {
        /// Authored canonical value.
        value: ScalarInputValue,
        /// Scoped-evidence identities; existence and truth remain separate validation.
        evidence: Vec<EntityId>,
    },
    /// Explicitly assumed authored value.
    Assumed {
        /// Authored canonical value.
        value: ScalarInputValue,
        /// Human-assumption record identity.
        assumption: EntityId,
    },
    /// Requires observation, without numeric or Boolean fallback.
    Unknown {
        /// Required observation-request identity.
        observation: EntityId,
    },
    /// Explicit preference, without substitution over a different unknown fact.
    Preference {
        /// Authored canonical value.
        value: ScalarInputValue,
        /// Preference-provenance identity.
        provenance: EntityId,
    },
    /// Formula is the sole result source, including for computed Area.
    Derived {
        /// Formula declaration identity.
        formula: EntityId,
    },
}

impl ScalarInputState {
    /// Authored-state label, without resolving values or referenced records.
    #[must_use]
    pub const fn tag(&self) -> ScalarInputStateTag {
        match self {
            Self::Known { .. } => ScalarInputStateTag::Known,
            Self::Assumed { .. } => ScalarInputStateTag::Assumed,
            Self::Unknown { .. } => ScalarInputStateTag::Unknown,
            Self::Preference { .. } => ScalarInputStateTag::Preference,
            Self::Derived { .. } => ScalarInputStateTag::Derived,
        }
    }
}

/// Editable canonical scalar definition; a metadata locator must not duplicate its content.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScalarInputDeclarationDefinition {
    /// Stable canonical declaration identity.
    pub id: EntityId,
    /// Source record identity.
    pub source: EntityId,
    /// Declared kind, available independently of numeric availability.
    pub kind: ScalarInputKind,
    /// Exactly one authored state with required provenance.
    pub state: ScalarInputState,
}

/// Immutable structurally validated scalar declaration. Registry/evidence truth remain deferred.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScalarInputDeclaration {
    definition: ScalarInputDeclarationDefinition,
}

/// Structural refusal with actual declaration and kind/state/evidence witnesses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScalarInputDeclarationError {
    /// Area is a computed product, without an independently authored input state.
    AreaMustBeDerived {
        /// Invalid declaration identity.
        declaration: EntityId,
        /// Actual non-derived state.
        state: ScalarInputStateTag,
    },
    /// Present authored value disagrees with the declared kind.
    KindMismatch {
        /// Invalid declaration identity.
        declaration: EntityId,
        /// Declared kind.
        declared: ScalarInputKind,
        /// Actual authored kind.
        actual: ScalarInputKind,
    },
    /// A Known claim has no evidence references.
    NoEvidence(EntityId),
    /// The authored evidence inventory repeats an identity.
    DuplicateEvidence {
        /// Invalid declaration identity.
        declaration: EntityId,
        /// First repeated identity in authored order.
        evidence: EntityId,
    },
}

impl fmt::Display for ScalarInputDeclarationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AreaMustBeDerived { declaration, state } => write!(
                f,
                "scalar declaration {declaration} area must be derived, found {state:?}"
            ),
            Self::KindMismatch {
                declaration,
                declared,
                actual,
            } => write!(
                f,
                "scalar declaration {declaration} declares {} but authors {}",
                declared.token(),
                actual.token()
            ),
            Self::NoEvidence(declaration) => write!(
                f,
                "scalar declaration {declaration} needs evidence references"
            ),
            Self::DuplicateEvidence {
                declaration,
                evidence,
            } => write!(
                f,
                "scalar declaration {declaration} repeats evidence {evidence}"
            ),
        }
    }
}
impl std::error::Error for ScalarInputDeclarationError {}

/// Canonical value needs observation or evaluation; no independently stored result is available.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScalarInputReadError {
    /// Required observation, without a replacement zero or false.
    RequiresObservation {
        /// Unresolved canonical declaration.
        declaration: EntityId,
        /// Actual observation request.
        observation: EntityId,
    },
    /// Required formula evaluation and state propagation.
    RequiresEvaluation {
        /// Derived canonical declaration.
        declaration: EntityId,
        /// Sole formula result source.
        formula: EntityId,
    },
}

impl fmt::Display for ScalarInputReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequiresObservation {
                declaration,
                observation,
            } => write!(
                f,
                "scalar declaration {declaration} requires observation {observation}"
            ),
            Self::RequiresEvaluation {
                declaration,
                formula,
            } => write!(
                f,
                "scalar declaration {declaration} requires formula {formula} evaluation"
            ),
        }
    }
}
impl std::error::Error for ScalarInputReadError {}

impl ScalarInputDeclaration {
    /// Validates kind/state and Known evidence structure, without fetching referenced records.
    /// # Errors
    /// Refuses non-derived Area first, then a present value's kind mismatch, then missing/duplicate
    /// Known evidence. Authored inventory order and the original definition are preserved.
    pub fn new(
        definition: ScalarInputDeclarationDefinition,
    ) -> Result<Self, ScalarInputDeclarationError> {
        if definition.kind == ScalarInputKind::Area
            && !matches!(definition.state, ScalarInputState::Derived { .. })
        {
            return Err(ScalarInputDeclarationError::AreaMustBeDerived {
                declaration: definition.id,
                state: definition.state.tag(),
            });
        }
        if let ScalarInputState::Known { value, .. }
        | ScalarInputState::Assumed { value, .. }
        | ScalarInputState::Preference { value, .. } = &definition.state
        {
            if definition.kind != value.kind() {
                return Err(ScalarInputDeclarationError::KindMismatch {
                    declaration: definition.id,
                    declared: definition.kind,
                    actual: value.kind(),
                });
            }
        }
        if let ScalarInputState::Known { evidence, .. } = &definition.state {
            if evidence.is_empty() {
                return Err(ScalarInputDeclarationError::NoEvidence(definition.id));
            }
            if let Some(evidence) = first_duplicate(evidence) {
                return Err(ScalarInputDeclarationError::DuplicateEvidence {
                    declaration: definition.id,
                    evidence,
                });
            }
        }
        Ok(Self { definition })
    }

    /// Stable canonical declaration identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }

    /// Authored source identity; carrying it does not establish record existence or truth.
    #[must_use]
    pub const fn source(&self) -> EntityId {
        self.definition.source
    }

    /// Declared kind, without reading or evaluating a value.
    #[must_use]
    pub const fn kind(&self) -> ScalarInputKind {
        self.definition.kind
    }

    /// Complete immutable definition, including original provenance inventory.
    #[must_use]
    pub const fn definition(&self) -> &ScalarInputDeclarationDefinition {
        &self.definition
    }

    /// Complete borrowed authored state.
    #[must_use]
    pub const fn state(&self) -> &ScalarInputState {
        &self.definition.state
    }

    /// Reads only a present authored value; does not execute Derived formulas or publish bindings.
    /// # Errors
    /// Unknown and Derived retain their actual declaration/observation/formula identities.
    pub fn authored_value(&self) -> Result<ScalarInputValue, ScalarInputReadError> {
        match self.state() {
            ScalarInputState::Known { value, .. }
            | ScalarInputState::Assumed { value, .. }
            | ScalarInputState::Preference { value, .. } => Ok(*value),
            ScalarInputState::Unknown { observation } => {
                Err(ScalarInputReadError::RequiresObservation {
                    declaration: self.id(),
                    observation: *observation,
                })
            }
            ScalarInputState::Derived { formula } => {
                Err(ScalarInputReadError::RequiresEvaluation {
                    declaration: self.id(),
                    formula: *formula,
                })
            }
        }
    }

    /// Source/evidence/uncertainty-policy validation remains owned by Design/G4.
    #[must_use]
    pub const fn provenance_validation(&self) -> ValueProvenanceValidation {
        ValueProvenanceValidation::DeferredToDesignAndG4
    }
}
