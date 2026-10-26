//! Existing numeric kernels with automatic retention of every evaluated operand's sources.
use super::FormulaEvaluatedScalar as V;
use crate::recipe::{
    FormulaBuiltin as B, FormulaKind as K, FormulaNumericError as E, FormulaNumericValue as N,
};

impl V {
    /// Correctly rounded inverse tangent, retaining the Ratio input and atan call source.
    /// # Errors
    /// Propagates the actual non-Ratio argument refusal, including Boolean and zero values.
    pub fn checked_atan(self) -> Result<Self, E> {
        let value = self
            .numeric_kind_argument(B::Atan, 0, K::Ratio)?
            .checked_atan()?;
        Ok(call_result(B::Atan, value, &[self]))
    }

    /// Principal atan2(self, x), retaining both operands and the atan2 call source.
    /// # Errors
    /// Checks homogeneous Length or Ratio kinds in argument order before the zero-vector domain.
    pub fn checked_atan2(self, x: Self) -> Result<Self, E> {
        if !matches!(self.kind(), K::Length | K::Ratio) {
            return Err(E::BuiltinInputKind {
                builtin: B::Atan2,
                argument: 0,
                actual: self.kind(),
            });
        }
        let y_value = self.numeric_argument(B::Atan2, 0)?;
        let x_value = x.homogeneous_argument(B::Atan2, 1, y_value.kind())?;
        let value = y_value.checked_atan2(x_value)?;
        Ok(call_result(B::Atan2, value, &[self, x]))
    }

    /// Correctly rounded sine, retaining the input and sin source even for exact axis results.
    /// # Errors
    /// Propagates the actual non-Angle argument refusal from the numeric value kernel.
    pub fn checked_sin(self) -> Result<Self, E> {
        let value = self.numeric_argument(B::Sin, 0)?.checked_sin()?;
        Ok(call_result(B::Sin, value, &[self]))
    }

    /// Correctly rounded cosine, retaining the input and cos source even for exact axis results.
    /// # Errors
    /// Propagates the actual non-Angle argument refusal from the numeric value kernel.
    pub fn checked_cos(self) -> Result<Self, E> {
        let value = self.numeric_argument(B::Cos, 0)?.checked_cos()?;
        Ok(call_result(B::Cos, value, &[self]))
    }

    /// Exact absolute value retaining all dependencies, without adding a rounding-call source.
    /// # Errors
    /// Returns the actual nonnumeric input kind.
    pub fn checked_abs(self) -> Result<Self, E> {
        Ok(call_result(
            B::Abs,
            self.numeric_argument(B::Abs, 0)?.abs(),
            &[self],
        ))
    }

    /// Correctly rounded square root, including a source for perfect or zero roots.
    /// # Errors
    /// Propagates the existing actual argument, negative-root, width or domain refusal.
    pub fn checked_sqrt(self) -> Result<Self, E> {
        let value = self.numeric_argument(B::Sqrt, 0)?.checked_sqrt()?;
        Ok(call_result(B::Sqrt, value, &[self]))
    }

    /// Correctly rounded hypotenuse, retaining both inputs and the hypot call itself.
    /// # Errors
    /// Returns the first non-Length argument or the existing rounded result refusal.
    pub fn checked_hypot(self, right: Self) -> Result<Self, E> {
        let left_value = self.numeric_kind_argument(B::Hypot, 0, K::Length)?;
        let right_value = right.numeric_kind_argument(B::Hypot, 1, K::Length)?;
        let value = left_value.checked_hypot(right_value)?;
        Ok(call_result(B::Hypot, value, &[self, right]))
    }

    /// Correctly rounded signed/full-turn arc length with both inputs and call source retained.
    /// # Errors
    /// Returns the first actual wrong argument kind or the existing numeric result refusal.
    pub fn checked_arc_length(self, radius: Self) -> Result<Self, E> {
        let angle = self.numeric_kind_argument(B::ArcLength, 0, K::Angle)?;
        let radius_value = radius.numeric_kind_argument(B::ArcLength, 1, K::Length)?;
        let value = angle.checked_arc_length(radius_value)?;
        Ok(call_result(B::ArcLength, value, &[self, radius]))
    }

    /// Exact minimum retaining every evaluated candidate's sources, including unselected values.
    /// # Errors
    /// Returns empty arity, nonnumeric first input or the first mismatched argument kind.
    pub fn checked_min(values: &[Self]) -> Result<Self, E> {
        extremum(values, B::Min)
    }

    /// Exact maximum retaining every evaluated candidate's sources, including unselected values.
    /// # Errors
    /// Returns empty arity, nonnumeric first input or the first mismatched argument kind.
    pub fn checked_max(values: &[Self]) -> Result<Self, E> {
        extremum(values, B::Max)
    }

    /// Exact inclusive clamp retaining the value and both evaluated bounds' sources.
    /// # Errors
    /// Returns the first actual kind refusal or the existing reversed-bound refusal.
    pub fn checked_clamp(self, low: Self, high: Self) -> Result<Self, E> {
        let value = self.numeric_argument(B::Clamp, 0)?;
        let low_value = low.homogeneous_argument(B::Clamp, 1, value.kind())?;
        let high_value = high.homogeneous_argument(B::Clamp, 2, value.kind())?;
        let result = value.checked_clamp(low_value, high_value)?;
        Ok(call_result(B::Clamp, result, &[self, low, high]))
    }

    /// Explicit quantization retaining both operands and round_to even when the value is unchanged.
    /// # Errors
    /// Returns actual kind, zero-step, reduced-width or completed-domain refusal.
    pub fn checked_round_to(self, step: Self) -> Result<Self, E> {
        let value = self.numeric_argument(B::RoundTo, 0)?;
        let step_value = step.homogeneous_argument(B::RoundTo, 1, value.kind())?;
        let result = value.checked_round_to(step_value)?;
        Ok(call_result(B::RoundTo, result, &[self, step]))
    }

    fn numeric_argument(self, builtin: B, argument: usize) -> Result<N, E> {
        self.numeric().ok_or(E::BuiltinInputKind {
            builtin,
            argument,
            actual: self.kind(),
        })
    }

    fn numeric_kind_argument(self, builtin: B, argument: usize, wanted: K) -> Result<N, E> {
        if self.kind() != wanted {
            return Err(E::BuiltinInputKind {
                builtin,
                argument,
                actual: self.kind(),
            });
        }
        self.numeric_argument(builtin, argument)
    }

    fn homogeneous_argument(self, builtin: B, argument: usize, wanted: K) -> Result<N, E> {
        if self.kind() != wanted {
            return Err(E::BuiltinDimension {
                builtin,
                argument,
                wanted,
                actual: self.kind(),
            });
        }
        self.numeric_argument(builtin, argument)
    }
}

fn call_result(builtin: B, value: N, arguments: &[V]) -> V {
    arguments
        .iter()
        .fold(V::from_numeric(value), |value, arg| {
            value.influenced_by(arg.sources())
        })
        .with_call_source(builtin)
}

fn extremum(values: &[V], builtin: B) -> Result<V, E> {
    let first = values
        .first()
        .ok_or(E::BuiltinArity { builtin, actual: 0 })?;
    let wanted = first.numeric_argument(builtin, 0)?.kind();
    let numeric = values
        .iter()
        .copied()
        .enumerate()
        .map(|(argument, value)| value.homogeneous_argument(builtin, argument, wanted))
        .collect::<Result<Vec<_>, _>>()?;
    let result = match builtin {
        B::Min => N::checked_min(&numeric)?,
        B::Max => N::checked_max(&numeric)?,
        _ => unreachable!("private extremum helper receives only min/max"),
    };
    Ok(call_result(builtin, result, values))
}
