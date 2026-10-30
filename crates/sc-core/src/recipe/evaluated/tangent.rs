//! Actual tangent execution retaining operand contributions and explicit native policy metadata.
use super::FormulaEvaluatedScalar;
use crate::recipe::{
    FormulaBuiltin, FormulaExecutedTangent, FormulaNumericError, FormulaTangentError,
    FormulaTangentExecutionError, FormulaTangentLimits,
};
use core::fmt;

/// A successful tangent call with its actual sourced operand and certified native execution.
#[derive(Debug)]
pub struct FormulaEvaluatedTangent {
    input: FormulaEvaluatedScalar,
    execution: FormulaExecutedTangent,
}
impl FormulaEvaluatedTangent {
    /// Original supplied operand with every executed input contribution preserved.
    #[must_use]
    pub const fn input(&self) -> FormulaEvaluatedScalar {
        self.input
    }

    /// Actual certified result,original native plan,explicit policy and execution counters.
    #[must_use]
    pub const fn execution(&self) -> &FormulaExecutedTangent {
        &self.execution
    }

    /// Certified Ratio with input contributions and this successful Tan call,even for zero/unit.
    #[must_use]
    pub const fn value(&self) -> FormulaEvaluatedScalar {
        FormulaEvaluatedScalar::from_numeric(self.execution.value())
            .influenced_by(self.input.sources())
            .with_call_source(FormulaBuiltin::Tan)
    }
}

#[derive(Debug)]
enum Failure {
    Preparation(FormulaTangentError),
    Execution(FormulaTangentExecutionError),
}

/// An attempted tangent call retaining its actual sourced operand,policy and original cause.
/// No successful result or new successful-call contribution is fabricated by failure.
#[derive(Debug)]
pub struct FormulaEvaluatedTangentError {
    input: FormulaEvaluatedScalar,
    limits: FormulaTangentLimits,
    failure: Failure,
}
impl FormulaEvaluatedTangentError {
    /// Original supplied operand,including Boolean false and all existing input contributions.
    #[must_use]
    pub const fn input(&self) -> FormulaEvaluatedScalar {
        self.input
    }
    /// Original explicit execution policy,including for wrong-kind preparation failures.
    #[must_use]
    pub const fn limits(&self) -> FormulaTangentLimits {
        self.limits
    }
    /// Actual attempted operation;this is not a successful-result source contribution.
    #[must_use]
    pub const fn builtin(&self) -> FormulaBuiltin {
        FormulaBuiltin::Tan
    }
    /// Original preparation cause,or None if the actual native plan reached execution.
    #[must_use]
    pub const fn preparation_error(&self) -> Option<FormulaTangentError> {
        match self.failure {
            Failure::Preparation(error) => Some(error),
            Failure::Execution(_) => None,
        }
    }
    /// Original native execution error with original plan and progress,without cloning it.
    #[must_use]
    pub const fn execution_error(&self) -> Option<&FormulaTangentExecutionError> {
        match &self.failure {
            Failure::Execution(error) => Some(error),
            Failure::Preparation(_) => None,
        }
    }
    /// Original mathematical diagnostic,or None for execution-resource admission.
    #[must_use]
    pub const fn diagnostic_code(&self) -> Option<&'static str> {
        match &self.failure {
            Failure::Preparation(error) => error.diagnostic_code(),
            Failure::Execution(error) => error.diagnostic_code(),
        }
    }
}
impl fmt::Display for FormulaEvaluatedTangentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.failure {
            Failure::Preparation(error) => fmt::Display::fmt(error, f),
            Failure::Execution(error) => fmt::Display::fmt(error, f),
        }
    }
}
impl std::error::Error for FormulaEvaluatedTangentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.failure {
            Failure::Preparation(error) => Some(error),
            Failure::Execution(error) => Some(error),
        }
    }
}

impl FormulaEvaluatedScalar {
    /// Executes tangent with explicit quotas,retaining the actual input and successful Tan source.
    ///
    /// No caller-supplied value or contribution set substitutes for the native computation.
    /// Exact zero/unit values still retain Tan;binding/arithmetic retain the resulting sources.
    ///
    /// # Errors
    /// Retains the original sourced input,explicit limits and actual preparation/execution cause.
    /// Wrong kind precedes resource admission. Failure exposes no successful output or invented
    /// contribution;previously executed input contributions remain available on the original input.
    ///
    /// ```
    /// use sc_core::recipe::{FormulaBuiltin,FormulaEvaluatedScalar,FormulaKind,
    ///     FormulaNumericValue,FormulaRational,FormulaTangentLimits,FormulaToleranceName};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let input=FormulaEvaluatedScalar::from_numeric(
    ///     FormulaNumericValue::new(FormulaKind::Angle,FormulaRational::ZERO)?);
    /// let result=input.checked_tan(FormulaTangentLimits::new(156,0))?;
    /// assert!(result.value().sources().contains(FormulaBuiltin::Tan));
    /// assert!(result.value().sources().check_tolerance(FormulaToleranceName::Numerical).is_err());
    /// assert_eq!(result.execution().charged_series_terms(),0);
    /// # Ok(())
    /// # }
    /// ```
    // Keep original input/policy/native causes inline;refusing resources must not allocate a box.
    #[allow(clippy::result_large_err)]
    pub fn checked_tan(
        self,
        limits: FormulaTangentLimits,
    ) -> Result<FormulaEvaluatedTangent, FormulaEvaluatedTangentError> {
        let fail = |failure| FormulaEvaluatedTangentError {
            input: self,
            limits,
            failure,
        };
        let numeric = self.numeric().ok_or_else(|| {
            fail(Failure::Preparation(FormulaTangentError::Numeric(
                FormulaNumericError::BuiltinInputKind {
                    builtin: FormulaBuiltin::Tan,
                    argument: 0,
                    actual: self.kind(),
                },
            )))
        })?;
        let plan = numeric
            .prepare_tangent(limits)
            .map_err(|error| fail(Failure::Preparation(error)))?;
        let execution = plan
            .evaluate()
            .map_err(|error| fail(Failure::Execution(error)))?;
        Ok(FormulaEvaluatedTangent {
            input: self,
            execution,
        })
    }
}
