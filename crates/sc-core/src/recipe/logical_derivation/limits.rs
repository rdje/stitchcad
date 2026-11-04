//! Whole original named logical-operation inputs, before role validation or source binding.
use super::{LogicalDerivationDefinition, LogicalDerivationRefusal};
/// Closed complete constructor resource classes,without runtime or source-query credit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogicalDerivationResource {
    /// Complete original typed rule-input population.
    Inputs,
    /// Complete separately owned local role text bytes.
    RoleTextBytes,
    /// Conservative complete P*P role comparisons.
    ValidationInspections,
}
/// Explicit logical operation constructor allowances,without a default rule or selected result.
/// ```compile_fail
/// let limits:sc_core::recipe::LogicalDerivationLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LogicalDerivationLimits {
    inputs: usize,
    text: usize,
    inspections: usize,
}
impl LogicalDerivationLimits {
    /// Select all complete allowances explicitly; zero admits an explicit empty vector.
    #[must_use]
    pub const fn new(inputs: usize, role_text_bytes: usize, validation_inspections: usize) -> Self {
        Self {
            inputs,
            text: role_text_bytes,
            inspections: validation_inspections,
        }
    }
    /// Maximum complete original named rule inputs.
    #[must_use]
    pub const fn inputs(self) -> usize {
        self.inputs
    }
    /// Maximum separately owned local role bytes.
    #[must_use]
    pub const fn role_text_bytes(self) -> usize {
        self.text
    }
    /// Maximum conservative P*P role comparisons.
    #[must_use]
    pub const fn validation_inspections(self) -> usize {
        self.inspections
    }
}
/// Factory-derived complete original populations,not a cached aggregate content allowance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LogicalDerivationShape {
    inputs: usize,
    text: usize,
    inspections: usize,
}
impl LogicalDerivationShape {
    /// Complete original typed input population.
    #[must_use]
    pub const fn inputs(self) -> usize {
        self.inputs
    }
    /// Complete separately owned original role bytes.
    #[must_use]
    pub const fn role_text_bytes(self) -> usize {
        self.text
    }
    /// Conservative complete P*P comparisons,not string byte work or measured CPU time.
    #[must_use]
    pub const fn validation_inspections(self) -> usize {
        self.inspections
    }
    pub(super) fn admit(
        definition: &LogicalDerivationDefinition,
        limits: LogicalDerivationLimits,
    ) -> Result<Self, LogicalDerivationRefusal> {
        use LogicalDerivationRefusal as R;
        let inputs = definition.inputs.len();
        let inspections = inputs.checked_mul(inputs).ok_or(R::PopulationOverflow)?;
        let text = definition.inputs.iter().try_fold(0_usize, |total, input| {
            total
                .checked_add(input.role.resource_text().len())
                .ok_or(R::PopulationOverflow)
        })?;
        for (resource, allowed, required) in [
            (LogicalDerivationResource::Inputs, limits.inputs, inputs),
            (LogicalDerivationResource::RoleTextBytes, limits.text, text),
            (
                LogicalDerivationResource::ValidationInspections,
                limits.inspections,
                inspections,
            ),
        ] {
            if required > allowed {
                return Err(R::Limit {
                    resource,
                    allowed,
                    required,
                });
            }
        }
        Ok(Self {
            inputs,
            text,
            inspections,
        })
    }
}
