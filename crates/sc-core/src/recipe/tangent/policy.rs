use super::super::{FormulaNumericError, FormulaRational};
use core::fmt;

/// Explicit tangent execution quotas. Neither quota measures heap bytes or wall-clock time.
///
/// Zero is permitted. Exact endpoints need no series terms but still reserve156 folding bits.
/// There is deliberately no default policy.
///
/// ```compile_fail
/// use sc_core::recipe::FormulaTangentLimits;
/// let _ = FormulaTangentLimits::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaTangentLimits {
    maximum_integer_bits: usize,
    maximum_series_terms: usize,
}

impl FormulaTangentLimits {
    /// Selects the largest candidate-integer extent and cumulative charged-series allowance.
    #[must_use]
    pub const fn new(maximum_integer_bits: usize, maximum_series_terms: usize) -> Self {
        Self {
            maximum_integer_bits,
            maximum_series_terms,
        }
    }

    /// Maximum bit extent of any candidate integer, not the sum of live integer storage.
    #[must_use]
    pub const fn maximum_integer_bits(self) -> usize {
        self.maximum_integer_bits
    }

    /// Maximum cumulative number of charged pi and component-series terms.
    #[must_use]
    pub const fn maximum_series_terms(self) -> usize {
        self.maximum_series_terms
    }
}

/// The unit of a refused tangent reservation; these are execution resources, not language limits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaTangentResource {
    /// Largest candidate-integer bit extent.
    IntegerBits,
    /// Cumulative charged pi and component-series terms.
    SeriesTerms,
}

/// A refused next scheduled reservation, with the actual original Angle and immutable policy.
/// No approximate value or formula-language diagnostic is supplied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaTangentResourceError {
    pub(super) input: FormulaRational,
    pub(super) limits: FormulaTangentLimits,
    pub(super) unit: FormulaTangentResource,
    pub(super) precision: Option<usize>,
    pub(super) reserved_integer_bits: usize,
    pub(super) charged_series_terms: usize,
    pub(super) requested: u128,
}

impl FormulaTangentResourceError {
    /// Original signed Angle in microdegrees, before private reduction.
    #[must_use]
    pub const fn input(self) -> FormulaRational {
        self.input
    }

    /// The exact selected execution policy.
    #[must_use]
    pub const fn limits(self) -> FormulaTangentLimits {
        self.limits
    }

    /// Refused reservation unit; bit admission precedes work admission.
    #[must_use]
    pub const fn unit(self) -> FormulaTangentResource {
        self.unit
    }

    /// Prospective dyadic precision, or None for the fixed exact-fold reservation.
    #[must_use]
    pub const fn precision(self) -> Option<usize> {
        self.precision
    }

    /// Highest previously admitted integer reservation, not an observed allocation peak.
    #[must_use]
    pub const fn reserved_integer_bits(self) -> usize {
        self.reserved_integer_bits
    }

    /// Work already charged by completed admitted iterations. Failed admission is never charged.
    #[must_use]
    pub const fn charged_series_terms(self) -> usize {
        self.charged_series_terms
    }

    /// Selected quota in the refused unit.
    #[must_use]
    pub const fn maximum(self) -> usize {
        match self.unit {
            FormulaTangentResource::IntegerBits => self.limits.maximum_integer_bits,
            FormulaTangentResource::SeriesTerms => self.limits.maximum_series_terms,
        }
    }

    /// Next scheduled integer extent or additional term charge; not a mathematical minimum.
    #[must_use]
    pub const fn requested(self) -> u128 {
        self.requested
    }
}

impl fmt::Display for FormulaTangentResourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "tangent resource reservation refused: {self:?}")
    }
}
impl std::error::Error for FormulaTangentResourceError {}

/// Tangent's original mathematical failure or a separate execution-resource refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaTangentError {
    /// Original actual-kind or completed numerical cause, without a fabricated source location.
    Numeric(FormulaNumericError),
    /// The actual Angle is exactly an odd quarter turn, where tangent is undefined.
    Pole {
        /// Original signed Angle, including its complete turns.
        input: FormulaRational,
    },
    /// A selected execution quota cannot admit the next complete reservation.
    Resources(FormulaTangentResourceError),
}

impl FormulaTangentError {
    /// Existing mathematical diagnostic family, or None for execution-resource admission.
    #[must_use]
    pub const fn diagnostic_code(self) -> Option<&'static str> {
        match self {
            Self::Numeric(error) => Some(error.diagnostic_code()),
            Self::Pole { .. } => Some("formula_domain"),
            Self::Resources(_) => None,
        }
    }
}
impl fmt::Display for FormulaTangentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Numeric(error) => fmt::Display::fmt(error, f),
            Self::Pole { input } => write!(f, "formula_domain: tangent pole at {input:?}"),
            Self::Resources(error) => fmt::Display::fmt(error, f),
        }
    }
}
impl std::error::Error for FormulaTangentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Numeric(error) => Some(error),
            Self::Resources(error) => Some(error),
            Self::Pole { .. } => None,
        }
    }
}
