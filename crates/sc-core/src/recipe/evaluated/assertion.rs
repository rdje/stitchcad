//! Actual named assertion outcome; no statement identity, namespace or replay execution is implied.
use super::FormulaToleranceComparison;
use crate::{name::MachineToken, recipe::FormulaEvaluatedScalar};
use core::fmt;

/// A named assertion whose original admitted comparison holds.
/// Private construction prevents a false comparison from being represented as satisfied.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FormulaSatisfiedAssertion<'name, 'context> {
    name: &'name MachineToken,
    comparison: FormulaToleranceComparison<'context>,
}
impl<'name, 'context> FormulaSatisfiedAssertion<'name, 'context> {
    /// Actual caller-supplied name, borrowed without inventing a recipe ordinal or source span.
    #[must_use]
    pub const fn name(self) -> &'name MachineToken {
        self.name
    }
    /// Original true comparison, including actual operands/context/sources.
    #[must_use]
    pub const fn comparison(self) -> FormulaToleranceComparison<'context> {
        self.comparison
    }
}
impl fmt::Debug for FormulaSatisfiedAssertion<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaSatisfiedAssertion")
            .field("tolerance", &self.comparison.tolerance().name())
            .finish_non_exhaustive()
    }
}

/// Named false assertion retaining the actual operands, quantitative class and full context.
/// The private boxed comparison keeps the Result error small without dropping its original data.
#[derive(Clone, PartialEq, Eq)]
pub struct FormulaAssertionFailure<'name, 'context> {
    name: &'name MachineToken,
    comparison: Box<FormulaToleranceComparison<'context>>,
}
impl<'name, 'context> FormulaAssertionFailure<'name, 'context> {
    /// Actual caller-supplied assertion name, not a substituted generic label.
    #[must_use]
    pub const fn name(&self) -> &'name MachineToken {
        self.name
    }
    /// Original false comparison, with both sourced values and actual quantitative context.
    #[must_use]
    pub fn comparison(&self) -> FormulaToleranceComparison<'context> {
        *self.comparison
    }
    /// Normative assertion refusal family; this is not an accepted Boolean false result.
    #[must_use]
    pub const fn diagnostic_code(&self) -> &'static str {
        "formula_assertion"
    }
}
impl fmt::Debug for FormulaAssertionFailure<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaAssertionFailure")
            .field("token", &self.diagnostic_code())
            .field("tolerance", &self.comparison.tolerance().name())
            .finish_non_exhaustive()
    }
}
impl fmt::Display for FormulaAssertionFailure<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: assert {}: ", self.diagnostic_code(), self.name)?;
        display_scalar(f, self.comparison.left())?;
        f.write_str(" != ")?;
        display_scalar(f, self.comparison.right())?;
        write!(f, " at {} (", self.comparison.tolerance().name().token())?;
        display_scalar(
            f,
            FormulaEvaluatedScalar::from_numeric(self.comparison.tolerance().value()),
        )?;
        f.write_str(")")
    }
}
impl std::error::Error for FormulaAssertionFailure<'_, '_> {}

impl<'context> FormulaToleranceComparison<'context> {
    /// Require this comparison to hold for the actual supplied assertion name.
    ///
    /// This publishes no bindings or geometry, checks no name scope, and invents no statement
    /// index/canonical expression. Earlier dimension/class/context failures precede this API.
    /// # Errors
    /// Returns formula_assertion with the original name and full false comparison on failure.
    pub fn assert_named<'name>(
        self,
        name: &'name MachineToken,
    ) -> Result<FormulaSatisfiedAssertion<'name, 'context>, FormulaAssertionFailure<'name, 'context>>
    {
        if self.holds() {
            Ok(FormulaSatisfiedAssertion {
                name,
                comparison: self,
            })
        } else {
            Err(FormulaAssertionFailure {
                name,
                comparison: Box::new(self),
            })
        }
    }
}

fn display_scalar(f: &mut fmt::Formatter<'_>, value: FormulaEvaluatedScalar) -> fmt::Result {
    write!(f, "{}:", value.kind().token())?;
    if let Some(numeric) = value.numeric() {
        let rational = numeric.rational();
        if rational.is_negative() {
            f.write_str("-")?;
        }
        write!(f, "{}", rational.numerator_magnitude())?;
        if rational.denominator() != 1 {
            write!(f, "/{}", rational.denominator())?;
        }
        Ok(())
    } else if let Some(boolean) = value.boolean() {
        write!(f, "{boolean}")
    } else {
        // The sealed scalar contains exactly numeric or Boolean; there is no fallback value.
        Err(fmt::Error)
    }
}
