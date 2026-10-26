//! Explicit reserved tolerance providers; no format/factory defaults or registry approval.
use super::{
    FormulaNumericValue, FormulaReservedContext, FormulaReservedName, FormulaToleranceName,
};
use crate::ontology::EntityId;
use core::fmt;
use sc_units::{Tolerance, ToleranceClass, UnitError};

mod size;
pub use size::{
    FormulaSizeContext, FormulaSizeContextError, FormulaSizeContextSource, FormulaSizeReadError,
    FormulaSizeValue,
};

/// Caller-supplied quantitative tolerance and its actual source-record identity.
/// The context validates the class; registry membership and derivation/evidence truth stay separate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaToleranceProvider<'a> {
    /// Actual format/receiver/profile source supplied by the host.
    pub source: EntityId,
    /// Original quantitative content, borrowed so it cannot mutate while the context uses it.
    pub tolerance: &'a Tolerance,
}

/// Editable explicit provider slots; absent optional values never acquire defaults.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FormulaToleranceContextDefinition<'a> {
    /// Optional actual execution-context label for diagnostics, without invented target identity.
    pub label: Option<&'a str>,
    /// Format quantum, requiring FormatQuantization class.
    pub format: Option<FormulaToleranceProvider<'a>>,
    /// Receiver comparison, requiring ImporterComparison class.
    pub importer: Option<FormulaToleranceProvider<'a>>,
    /// Factory tolerance, requiring PhysicalAcceptance class.
    pub physical: Option<FormulaToleranceProvider<'a>>,
}

/// Supplied quantitative content has the wrong class for its reserved-name slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaToleranceProviderError<'a> {
    /// Actual slot being populated.
    pub name: FormulaToleranceName,
    /// Class required by that slot.
    pub expected: ToleranceClass,
    /// Actual supplied source and quantitative content.
    pub provider: FormulaToleranceProvider<'a>,
}

impl fmt::Display for FormulaToleranceProviderError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} provider {} needs {}, found {}",
            self.name.token(),
            self.provider.source,
            self.expected.label(),
            self.provider.tolerance.class.label()
        )
    }
}
impl std::error::Error for FormulaToleranceProviderError<'_> {}

/// Immutable class-checked tolerance context; intrinsics retain the language's fixed defaults.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaToleranceContext<'a> {
    definition: FormulaToleranceContextDefinition<'a>,
}

impl Default for FormulaToleranceContext<'_> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<'a> FormulaToleranceContext<'a> {
    /// No external provider or label. Numerical/geometric intrinsics are still available.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            definition: FormulaToleranceContextDefinition {
                label: None,
                format: None,
                importer: None,
                physical: None,
            },
        }
    }

    /// Checks each present provider's actual class, in format/importer/physical order.
    /// # Errors
    /// Returns the first actual wrong-class provider. Intrinsic numerical/internal geometric
    /// tolerances cannot be replaced by these optional external slots.
    pub fn new(
        definition: FormulaToleranceContextDefinition<'a>,
    ) -> Result<Self, FormulaToleranceProviderError<'a>> {
        for (name, expected, provider) in [
            (
                FormulaToleranceName::Format,
                ToleranceClass::FormatQuantization,
                definition.format,
            ),
            (
                FormulaToleranceName::Importer,
                ToleranceClass::ImporterComparison,
                definition.importer,
            ),
            (
                FormulaToleranceName::Physical,
                ToleranceClass::PhysicalAcceptance,
                definition.physical,
            ),
        ] {
            if let Some(provider) = provider {
                if provider.tolerance.class != expected {
                    return Err(FormulaToleranceProviderError {
                        name,
                        expected,
                        provider,
                    });
                }
            }
        }
        Ok(Self { definition })
    }

    /// Original immutable label/provider slots; cloning a definition does not mutate this context.
    #[must_use]
    pub const fn definition(&self) -> &FormulaToleranceContextDefinition<'a> {
        &self.definition
    }

    /// Reads the actual named tolerance as an exact Length, retaining class/derivation and source.
    /// # Errors
    /// Missing optional providers return formula_tolerance_unbound with the actual supplied label.
    /// Unit errors while constructing intrinsic constants propagate rather than panic or default.
    pub fn read(
        self,
        name: FormulaToleranceName,
    ) -> Result<FormulaToleranceValue<'a>, FormulaToleranceReadError<'a>> {
        let (tolerance, source) = match name {
            FormulaToleranceName::Numerical | FormulaToleranceName::Geometric => {
                let intrinsic = if name == FormulaToleranceName::Numerical {
                    Tolerance::numerical()
                } else {
                    Tolerance::geometric_internal()
                };
                (
                    intrinsic
                        .map_err(|error| FormulaToleranceReadError::Intrinsic { name, error })?,
                    FormulaToleranceValueSource::Intrinsic,
                )
            }
            FormulaToleranceName::Format
            | FormulaToleranceName::Importer
            | FormulaToleranceName::Physical => {
                let provider = match name {
                    FormulaToleranceName::Format => self.definition.format,
                    FormulaToleranceName::Importer => self.definition.importer,
                    FormulaToleranceName::Physical => self.definition.physical,
                    _ => None,
                }
                .ok_or(FormulaToleranceReadError::MissingContext {
                    name,
                    context_label: self.definition.label,
                })?;
                (
                    *provider.tolerance,
                    FormulaToleranceValueSource::Provider(provider),
                )
            }
        };
        Ok(FormulaToleranceValue {
            name,
            tolerance,
            source,
            context_label: self.definition.label,
        })
    }
}

/// Whether a value came from the language contract or an actual external provider.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaToleranceValueSource<'a> {
    /// eps_num=1 or eps_geo=10 from the language's declared unit defaults.
    Intrinsic,
    /// Original supplied quantitative tolerance and source identity.
    Provider(FormulaToleranceProvider<'a>),
}

/// Exact named tolerance with its actual class, derivation, provider and optional context label.
/// No registry, physical acceptance or artifact authority is granted by reading it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaToleranceValue<'a> {
    name: FormulaToleranceName,
    tolerance: Tolerance,
    source: FormulaToleranceValueSource<'a>,
    context_label: Option<&'a str>,
}

impl<'a> FormulaToleranceValue<'a> {
    /// Actual reserved tolerance name, retaining the named comparison class role.
    #[must_use]
    pub const fn name(self) -> FormulaToleranceName {
        self.name
    }

    /// Exact completed Length in canonical micrometres, without input/binding rounding.
    #[must_use]
    pub fn value(self) -> FormulaNumericValue {
        self.tolerance.limit.into()
    }

    /// Actual quantitative class, limit and original requirement derivation.
    #[must_use]
    pub const fn tolerance(self) -> Tolerance {
        self.tolerance
    }

    /// Language intrinsic or the actual borrowed provider/source identity.
    #[must_use]
    pub const fn source(self) -> FormulaToleranceValueSource<'a> {
        self.source
    }

    /// Caller-supplied execution label, retaining None rather than guessing one.
    #[must_use]
    pub const fn context_label(self) -> Option<&'a str> {
        self.context_label
    }
}

/// Named context absence or an actual intrinsic unit-construction failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaToleranceReadError<'a> {
    /// No explicit provider supplies this optional class.
    MissingContext {
        /// Actual name that was read.
        name: FormulaToleranceName,
        /// Actual optional caller label, with no invented export/profile name.
        context_label: Option<&'a str>,
    },
    /// The units API refused construction of a language intrinsic.
    Intrinsic {
        /// Actual intrinsic being constructed.
        name: FormulaToleranceName,
        /// Original unit error, without a substituted value.
        error: UnitError,
    },
}

impl FormulaToleranceReadError<'_> {
    /// Actual requested tolerance name.
    #[must_use]
    pub const fn name(self) -> FormulaToleranceName {
        match self {
            Self::MissingContext { name, .. } | Self::Intrinsic { name, .. } => name,
        }
    }

    /// Provider category required by the language, distinct from an actual caller label.
    #[must_use]
    pub const fn required_context(self) -> FormulaReservedContext {
        FormulaReservedName::Tolerance(self.name()).required_context()
    }

    /// Stable diagnostic family for this read; no source location or ordinal is invented.
    #[must_use]
    pub const fn diagnostic_code(self) -> &'static str {
        match self {
            Self::MissingContext { .. } => "formula_tolerance_unbound",
            Self::Intrinsic { .. } => "formula_domain",
        }
    }
}

impl fmt::Display for FormulaToleranceReadError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingContext {
                name,
                context_label: Some(label),
            } => {
                write!(f, "formula_tolerance_unbound: {} in {label}", name.token())
            }
            Self::MissingContext {
                name,
                context_label: None,
            } => {
                write!(
                    f,
                    "formula_tolerance_unbound: {} needs {:?} context",
                    name.token(),
                    self.required_context()
                )
            }
            Self::Intrinsic { name, error } => {
                write!(f, "{}: {}: {error}", self.diagnostic_code(), name.token())
            }
        }
    }
}
impl std::error::Error for FormulaToleranceReadError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Intrinsic { error, .. } => Some(error),
            Self::MissingContext { .. } => None,
        }
    }
}
