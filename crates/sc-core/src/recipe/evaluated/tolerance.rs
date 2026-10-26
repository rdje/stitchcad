//! Exact named comparison values; statement/within source context belongs to the evaluator.
use super::FormulaEvaluatedScalar as V;
use crate::recipe::{
    FormulaApproximationToleranceError, FormulaBuiltin, FormulaNumericError, FormulaToleranceValue,
};
use core::fmt;

/// Actual structural or approximation-class refusal before threshold comparison.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaToleranceComparisonRefusal {
    /// Wrong actual operand kinds, using existing numerical signature diagnostics.
    Dimension(FormulaNumericError),
    /// T1 was requested for contributed values, even if their numerical difference is zero.
    Approximation(FormulaApproximationToleranceError),
}
impl FormulaToleranceComparisonRefusal {
    /// Existing diagnostic family; no new language error token.
    #[must_use]
    pub const fn diagnostic_code(self) -> &'static str {
        match self {
            Self::Dimension(error) => error.diagnostic_code(),
            Self::Approximation(error) => error.diagnostic_code(),
        }
    }
}

/// Actual supplied operands and quantitative context beside the first comparison refusal.
/// No comparison label, source span or statement ordinal is invented by this value primitive.
#[derive(Clone, PartialEq, Eq)]
pub struct FormulaToleranceComparisonError<'a>(Box<ComparisonFailure<'a>>);

#[derive(Clone, PartialEq, Eq)]
struct ComparisonFailure<'a> {
    left: V,
    right: V,
    tolerance: FormulaToleranceValue<'a>,
    refusal: FormulaToleranceComparisonRefusal,
}
impl<'a> FormulaToleranceComparisonError<'a> {
    /// Actual original left operand, including its sources.
    #[must_use]
    pub fn left(&self) -> V {
        self.0.left
    }
    /// Actual original right operand, including its sources.
    #[must_use]
    pub fn right(&self) -> V {
        self.0.right
    }
    /// Actual supplied class/limit/derivation/provider/context, without substitution.
    #[must_use]
    pub fn tolerance(&self) -> FormulaToleranceValue<'a> {
        self.0.tolerance
    }
    /// First typed refusal; no threshold result is published after a class refusal.
    #[must_use]
    pub fn refusal(&self) -> FormulaToleranceComparisonRefusal {
        self.0.refusal
    }
    /// Existing diagnostic family, without an invented statement context.
    #[must_use]
    pub fn diagnostic_code(&self) -> &'static str {
        self.0.refusal.diagnostic_code()
    }
}
impl fmt::Debug for FormulaToleranceComparisonError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaToleranceComparisonError")
            .field("refusal", &self.0.refusal)
            .field("tolerance", &self.0.tolerance.name())
            .finish_non_exhaustive()
    }
}
impl fmt::Display for FormulaToleranceComparisonError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.refusal {
            FormulaToleranceComparisonRefusal::Dimension(error) => fmt::Display::fmt(&error, f),
            FormulaToleranceComparisonRefusal::Approximation(error) => fmt::Display::fmt(&error, f),
        }
    }
}
impl std::error::Error for FormulaToleranceComparisonError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.0.refusal {
            FormulaToleranceComparisonRefusal::Dimension(error) => Some(error),
            FormulaToleranceComparisonRefusal::Approximation(error) => Some(error),
        }
    }
}

/// Completed inclusive threshold comparison, retaining actual operands and quantitative context.
/// An unmet threshold is a valid false result here; assertion execution decides its own refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaToleranceComparison<'a> {
    left: V,
    right: V,
    tolerance: FormulaToleranceValue<'a>,
    holds: bool,
}
impl<'a> FormulaToleranceComparison<'a> {
    /// Actual original left operand, including its sources.
    #[must_use]
    pub const fn left(self) -> V {
        self.left
    }
    /// Actual original right operand, including its sources.
    #[must_use]
    pub const fn right(self) -> V {
        self.right
    }
    /// Actual supplied class/limit/derivation/provider/context.
    #[must_use]
    pub const fn tolerance(self) -> FormulaToleranceValue<'a> {
        self.tolerance
    }
    /// Whether the exact absolute difference is within the inclusive threshold.
    #[must_use]
    pub const fn holds(self) -> bool {
        self.holds
    }
    /// Boolean result with both evaluated operands' approximation sources retained.
    #[must_use]
    pub const fn value(self) -> V {
        V::from_boolean(self.holds).influenced_by(self.left.sources().union(self.right.sources()))
    }
}

impl V {
    /// Compare same-kind numeric operands at an actual supplied symbolic tolerance class.
    ///
    /// Numerical payloads and tolerance limits use their canonical internal integers/fractions.
    /// The absolute difference is a comparison intermediate, never another numeric expression.
    /// # Errors
    /// Returns actual operand-kind or inadmissible T1-source refusal, retaining supplied context.
    /// Missing-provider lookup must already have succeeded before this primitive is called.
    pub fn compare_tolerance<'a>(
        self,
        right: Self,
        tolerance: FormulaToleranceValue<'a>,
    ) -> Result<FormulaToleranceComparison<'a>, FormulaToleranceComparisonError<'a>> {
        let error = |refusal| {
            FormulaToleranceComparisonError(Box::new(ComparisonFailure {
                left: self,
                right,
                tolerance,
                refusal,
            }))
        };
        let left_value = self.numeric().ok_or_else(|| {
            error(FormulaToleranceComparisonRefusal::Dimension(
                FormulaNumericError::BuiltinInputKind {
                    builtin: FormulaBuiltin::Within,
                    argument: 0,
                    actual: self.kind(),
                },
            ))
        })?;
        if right.kind() != self.kind() {
            return Err(error(FormulaToleranceComparisonRefusal::Dimension(
                FormulaNumericError::BuiltinDimension {
                    builtin: FormulaBuiltin::Within,
                    argument: 1,
                    wanted: self.kind(),
                    actual: right.kind(),
                },
            )));
        }
        let right_value = right.numeric().ok_or_else(|| {
            error(FormulaToleranceComparisonRefusal::Dimension(
                FormulaNumericError::BuiltinInputKind {
                    builtin: FormulaBuiltin::Within,
                    argument: 1,
                    actual: right.kind(),
                },
            ))
        })?;
        self.sources()
            .union(right.sources())
            .check_tolerance(tolerance.name())
            .map_err(|refusal| error(FormulaToleranceComparisonRefusal::Approximation(refusal)))?;
        let holds = left_value
            .rational()
            .is_within(right_value.rational(), tolerance.value().rational());
        Ok(FormulaToleranceComparison {
            left: self,
            right,
            tolerance,
            holds,
        })
    }
}
