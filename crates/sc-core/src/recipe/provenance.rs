//! Immutable call-source metadata for executed approximation dependencies.
//!
//! This does not evaluate calls or certify that a caller selected the executed dependencies.
use super::{FormulaBuiltin, FormulaToleranceName};
use core::fmt;

// Lexical machine-token order is stable independently of the signature table's grouping.
const SOURCES: [FormulaBuiltin; 11] = [
    FormulaBuiltin::ArcLength,
    FormulaBuiltin::Atan,
    FormulaBuiltin::Atan2,
    FormulaBuiltin::Cos,
    FormulaBuiltin::Dir,
    FormulaBuiltin::Dist,
    FormulaBuiltin::Hypot,
    FormulaBuiltin::RoundTo,
    FormulaBuiltin::Sin,
    FormulaBuiltin::Sqrt,
    FormulaBuiltin::Tan,
];

/// The distinct approximation-producing calls in a value's executed dependencies.
///
/// Sources describe how a value was obtained, even when a particular call returns an exact
/// integer, cancels later, or survives a binding. Input/binding rounding alone adds no source.
/// The evaluator must merge all executed operands and the selected conditional's condition,
/// excluding untaken branches. Point coordinates must retain their own sets.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct FormulaApproximationSources(u16);

impl FormulaApproximationSources {
    /// No approximation-producing dependency; no claim about value or source-record truth.
    pub const EMPTY: Self = Self(0);

    /// Contribution introduced by this successful executed call, excluding its input sources.
    /// A nonproducing built-in returns an empty set; its input sources still need merging.
    #[must_use]
    pub const fn for_call(call: FormulaBuiltin) -> Self {
        Self(source_bit(call))
    }

    /// Whether the supplied dependencies contain no approximation-producing call.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Whether this exact source is present. Nonproducing calls always return false.
    #[must_use]
    pub const fn contains(self, call: FormulaBuiltin) -> bool {
        self.0 & source_bit(call) != 0
    }

    /// Immutable deduplicated union; operand order and repeated occurrences do not affect it.
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Add the call's own contribution after merging its executed inputs.
    #[must_use]
    pub const fn with_call(self, call: FormulaBuiltin) -> Self {
        self.union(Self::for_call(call))
    }

    /// Distinct existing built-ins in lexical machine-token order; no invented source names.
    pub fn iter(self) -> impl Iterator<Item = FormulaBuiltin> {
        SOURCES.into_iter().filter(move |call| self.contains(*call))
    }

    /// Apply only the symbolic T1 rule, independently of the tolerance's numeric limit.
    ///
    /// This does not supply a missing provider, compare values, or establish class suitability.
    /// An enclosing evaluator must attach the actual comparison/name/statement context.
    ///
    /// # Errors
    /// Returns the actual requested class and contributing calls when T1 is inadmissible.
    pub fn check_tolerance(
        self,
        tolerance: FormulaToleranceName,
    ) -> Result<(), FormulaApproximationToleranceError> {
        if tolerance == FormulaToleranceName::Numerical && !self.is_empty() {
            Err(FormulaApproximationToleranceError {
                tolerance,
                sources: self,
            })
        } else {
            Ok(())
        }
    }
}

impl fmt::Debug for FormulaApproximationSources {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set()
            .entries(self.iter().map(FormulaBuiltin::token))
            .finish()
    }
}

/// Source-only class refusal; enclosing comparison context is deliberately not fabricated.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaApproximationToleranceError {
    tolerance: FormulaToleranceName,
    sources: FormulaApproximationSources,
}
impl FormulaApproximationToleranceError {
    /// Actual symbolic class requested by the caller.
    #[must_use]
    pub const fn tolerance(self) -> FormulaToleranceName {
        self.tolerance
    }

    /// Complete supplied executed contribution set, in deterministic order when inspected.
    #[must_use]
    pub const fn sources(self) -> FormulaApproximationSources {
        self.sources
    }

    /// Existing numerical-domain diagnostic family.
    #[must_use]
    pub const fn diagnostic_code(self) -> &'static str {
        "formula_domain"
    }
}
impl fmt::Display for FormulaApproximationToleranceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} requires T2 or looser for contributions {:?}",
            self.diagnostic_code(),
            self.tolerance.token(),
            self.sources
        )
    }
}
impl std::error::Error for FormulaApproximationToleranceError {}

const fn source_bit(call: FormulaBuiltin) -> u16 {
    use FormulaBuiltin as B;
    match call {
        B::ArcLength => 1 << 0,
        B::Atan => 1 << 1,
        B::Atan2 => 1 << 2,
        B::Cos => 1 << 3,
        B::Dir => 1 << 4,
        B::Dist => 1 << 5,
        B::Hypot => 1 << 6,
        B::RoundTo => 1 << 7,
        B::Sin => 1 << 8,
        B::Sqrt => 1 << 9,
        B::Tan => 1 << 10,
        B::Abs
        | B::Min
        | B::Max
        | B::Clamp
        | B::If
        | B::Within
        | B::X
        | B::Y
        | B::Len
        | B::ParamAt
        | B::PointAt => 0,
    }
}
