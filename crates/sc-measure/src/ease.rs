//! Authored body-to-garment intent over canonical current measurements and signed inputs.
use crate::{
    Measurement, MeasurementBinding, MeasurementError, MeasurementKind, MeasurementTableContext,
};
use core::fmt;
use sc_core::{
    ontology::EntityId,
    value::{LengthDeclaration, LengthValueError},
};
use sc_units::Length;

/// Ordered named fit classes, without invented physical thresholds or a default.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FitIntent {
    /// Close fit.
    Close,
    /// Semi-fitted intent.
    Semi,
    /// Loose fit.
    Loose,
}
/// Explicit authorization for compression; evidence scope/truth is a later registry obligation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompressionPermission {
    /// Negative ease is refused, including a later evaluated result.
    Forbidden,
    /// Compression has been explicitly declared for this mapping.
    Declared {
        /// Record owning the declaration; not inferred from fit class or value source.
        provenance: EntityId,
    },
}
/// Selected side of a body-to-garment mapping.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EaseSide {
    /// Body measurement that supplies the reference dimension.
    Body,
    /// Garment point of measure produced by the mapping.
    Garment,
}
impl EaseSide {
    const fn kind(self) -> MeasurementKind {
        match self {
            Self::Body => MeasurementKind::Body,
            Self::Garment => MeasurementKind::Garment,
        }
    }
}
/// Authored mapping; values and their uncertainty/source remain solely in a core declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EaseDefinition {
    /// Stable mapping identity.
    pub id: EntityId,
    /// Expected immutable body-measurement binding.
    pub body: MeasurementBinding,
    /// Expected immutable garment-POM binding.
    pub garment: MeasurementBinding,
    /// Canonical signed ease amount, distinct from the two measurement scalars.
    pub declaration: EntityId,
    /// Ordered named intent; no quantitative threshold is implied.
    pub fit: FitIntent,
    /// Correspondence and fit-intent provenance; record validity is checked by its owner.
    pub provenance: EntityId,
    /// Explicit compression declaration or refusal.
    pub compression: CompressionPermission,
}
/// Immutable, structurally validated individual Ease mapping.
/// ```compile_fail
/// fn change(ease: &mut sc_measure::Ease) { ease.definition.fit = sc_measure::FitIntent::Loose; }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ease {
    definition: EaseDefinition,
}
/// Current-reference, domain or signed-amount refusal, with scoped semantic identities.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EaseError {
    /// Mapping identity aliases a supplied semantic record.
    DuplicateIdentity(EntityId),
    /// Ease amount aliases a body/POM scalar instead of a distinct input.
    AliasedDeclaration(EntityId),
    /// An authored side is in the wrong measurement domain.
    WrongKind {
        /// Mapping whose authored binding is invalid.
        ease: EntityId,
        /// Selected side.
        side: EaseSide,
        /// Actual authored domain.
        actual: MeasurementKind,
    },
    /// Saved measurement identity is absent; matching peers never replace it.
    MissingMeasurement {
        /// Selected mapping.
        ease: EntityId,
        /// Selected side.
        side: EaseSide,
        /// Missing identity.
        measurement: EntityId,
    },
    /// Current metadata reassigns saved identity/token/domain/scalar expectations.
    ReassignedBinding {
        /// Selected mapping.
        ease: EntityId,
        /// Selected side.
        side: EaseSide,
        /// Authored expectation.
        expected: Box<MeasurementBinding>,
        /// Current metadata binding.
        actual: Box<MeasurementBinding>,
    },
    /// Current selected metadata fails its own required reference/domain checks.
    InvalidMeasurement {
        /// Selected mapping.
        ease: EntityId,
        /// Selected side.
        side: EaseSide,
        /// Exact underlying structural refusal.
        issue: MeasurementError,
    },
    /// Canonical ease amount is absent.
    MissingDeclaration {
        /// Selected mapping.
        ease: EntityId,
        /// Missing amount declaration.
        declaration: EntityId,
    },
    /// A negative amount lacks an explicit compression declaration.
    UndeclaredCompression {
        /// Selected mapping.
        ease: EntityId,
        /// Amount input whose result needs authorization.
        declaration: EntityId,
        /// Exact refused signed amount.
        amount: Length,
    },
    /// The canonical amount requires observation/evaluation; no fallback is supplied.
    UnavailableValue {
        /// Selected mapping.
        ease: EntityId,
        /// Exact underlying unresolved source.
        issue: LengthValueError,
    },
}
impl fmt::Display for EaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity(id) => write!(f, "ease reuses identity {id}"),
            Self::AliasedDeclaration(id) => write!(f, "ease amount aliases measurement declaration {id}"),
            Self::WrongKind { ease, side, actual } => write!(f, "ease {ease} {side:?} has wrong authored kind {actual:?}"),
            Self::MissingMeasurement { ease, side, measurement } => write!(f, "ease {ease} {side:?} measurement {measurement} is missing"),
            Self::ReassignedBinding { ease, side, expected, actual } => write!(f, "ease {ease} {side:?} binding reassigned: {expected:?} -> {actual:?}"),
            Self::InvalidMeasurement { ease, side, issue } => write!(f, "ease {ease} {side:?}: {issue}"),
            Self::MissingDeclaration { ease, declaration } => write!(f, "ease {ease} amount declaration {declaration} is missing"),
            Self::UndeclaredCompression { ease, declaration, amount } => write!(f, "ease {ease} declaration {declaration} needs compression authorization for {amount:?}"),
            Self::UnavailableValue { ease, issue } => write!(f, "ease {ease}: {issue}"),
        }
    }
}
impl std::error::Error for EaseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidMeasurement { issue, .. } => Some(issue),
            Self::UnavailableValue { issue, .. } => Some(issue),
            _ => None,
        }
    }
}
impl Ease {
    /// Validate authored domains, distinct amount and every current target; unresolved drafts legal.
    /// # Errors
    /// Returns [`EaseError`] for an invalid binding, alias, missing target or undeclared compression.
    pub fn new(
        definition: EaseDefinition,
        context: &MeasurementTableContext<'_>,
    ) -> Result<Self, EaseError> {
        for (side, binding) in [
            (EaseSide::Body, &definition.body),
            (EaseSide::Garment, &definition.garment),
        ] {
            if binding.kind != side.kind() {
                return Err(EaseError::WrongKind {
                    ease: definition.id,
                    side,
                    actual: binding.kind,
                });
            }
        }
        if definition.declaration == definition.body.declaration
            || definition.declaration == definition.garment.declaration
        {
            return Err(EaseError::AliasedDeclaration(definition.declaration));
        }
        let ease = Self { definition };
        ease.validate_current(context)?;
        Ok(ease)
    }
    /// Stable mapping identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }
    /// Borrow authored intent without bypassing construction validation.
    #[must_use]
    pub const fn definition(&self) -> &EaseDefinition {
        &self.definition
    }
    /// Validate both current bindings, metadata targets and present amount/permission.
    /// # Errors
    /// Returns [`EaseError`] for any current reference or compression failure.
    pub fn validate_current(&self, context: &MeasurementTableContext<'_>) -> Result<(), EaseError> {
        self.declaration(context).map(|_| ())
    }
    fn validate_identity(&self, context: &MeasurementTableContext<'_>) -> Result<(), EaseError> {
        if context.contains(self.id()) {
            return Err(EaseError::DuplicateIdentity(self.id()));
        }
        Ok(())
    }
    /// Borrow one selected current measurement, checking saved binding and all its metadata targets.
    /// This query does not certify the other side or the ease amount.
    /// # Errors
    /// Returns [`EaseError`] for collision, missing/reassigned or invalid selected metadata.
    pub fn measurement<'a>(
        &self,
        side: EaseSide,
        context: &MeasurementTableContext<'a>,
    ) -> Result<&'a Measurement, EaseError> {
        self.validate_identity(context)?;
        let expected = match side {
            EaseSide::Body => &self.definition.body,
            EaseSide::Garment => &self.definition.garment,
        };
        let measurement = context.current_measurement(expected.measurement).ok_or(
            EaseError::MissingMeasurement {
                ease: self.id(),
                side,
                measurement: expected.measurement,
            },
        )?;
        let actual = MeasurementBinding::from(measurement);
        if &actual != expected {
            return Err(EaseError::ReassignedBinding {
                ease: self.id(),
                side,
                expected: Box::new(expected.clone()),
                actual: Box::new(actual),
            });
        }
        measurement
            .validate_current(context.records())
            .map_err(|issue| EaseError::InvalidMeasurement {
                ease: self.id(),
                side,
                issue,
            })?;
        Ok(measurement)
    }
    /// Borrow the sole current ease value/state/source after validating both measurement bindings.
    /// Unknown/derived amounts remain inspectable; they do not acquire a numeric default.
    /// # Errors
    /// Returns [`EaseError`] for current references or present unauthorized negative amounts.
    pub fn declaration<'a>(
        &self,
        context: &MeasurementTableContext<'a>,
    ) -> Result<&'a LengthDeclaration, EaseError> {
        self.measurement(EaseSide::Body, context)?;
        self.measurement(EaseSide::Garment, context)?;
        let declaration = context
            .records()
            .current_declaration(self.definition.declaration)
            .ok_or(EaseError::MissingDeclaration {
                ease: self.id(),
                declaration: self.definition.declaration,
            })?;
        if let Ok(amount) = declaration.authored_value() {
            self.validate_amount(amount)?;
        }
        Ok(declaration)
    }
    /// Read only a present canonical authored value, with compression permission enforced.
    /// # Errors
    /// Returns [`EaseError`] for current references, compression or unresolved observation/evaluation.
    pub fn authored_value(
        &self,
        context: &MeasurementTableContext<'_>,
    ) -> Result<Length, EaseError> {
        self.declaration(context)?
            .authored_value()
            .map_err(|issue| EaseError::UnavailableValue {
                ease: self.id(),
                issue,
            })
    }
    /// Check compression permission for a signed amount, including an externally evaluated result.
    /// Does not prove evaluation provenance, current Design revision, fit or release eligibility.
    /// # Errors
    /// Returns [`EaseError::UndeclaredCompression`] when a negative amount is not authorized.
    pub fn validate_amount(&self, amount: Length) -> Result<(), EaseError> {
        if amount < Length::ZERO && self.definition.compression == CompressionPermission::Forbidden
        {
            return Err(EaseError::UndeclaredCompression {
                ease: self.id(),
                declaration: self.definition.declaration,
                amount,
            });
        }
        Ok(())
    }
}
