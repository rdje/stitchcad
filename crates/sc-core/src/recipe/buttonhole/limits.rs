//! Complete original semantic policy populations, before local role validation.
use super::{ButtonholeOperationDefinition, ButtonholeOperationRefusal};
/// Closed constructor allowances; these do not cover Root content, source queries or runtime work.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonholeOperationResource {
    /// Complete original named dependency entries.
    PolicyInputs,
    /// Complete separately owned role text bytes.
    RoleTextBytes,
    /// Conservative complete P*P role comparison allowance.
    ValidationInspections,
}
/// Explicit complete operation constructor bounds. There is no default physical or admission policy.
/// ```compile_fail
/// let limits:sc_core::recipe::ButtonholeOperationLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonholeOperationLimits {
    inputs: usize,
    text: usize,
    inspections: usize,
}
impl ButtonholeOperationLimits {
    /// Select all complete allowances explicitly; zero admits an explicit empty input vector.
    #[must_use]
    pub const fn new(
        policy_inputs: usize,
        role_text_bytes: usize,
        validation_inspections: usize,
    ) -> Self {
        Self {
            inputs: policy_inputs,
            text: role_text_bytes,
            inspections: validation_inspections,
        }
    }
    /// Maximum complete original named policy dependencies.
    #[must_use]
    pub const fn policy_inputs(self) -> usize {
        self.inputs
    }
    /// Maximum original local role text bytes.
    #[must_use]
    pub const fn role_text_bytes(self) -> usize {
        self.text
    }
    /// Maximum conservative P*P original role comparisons.
    #[must_use]
    pub const fn validation_inspections(self) -> usize {
        self.inspections
    }
}
/// Factory-derived complete original populations, without cached-content admission credit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonholeOperationShape {
    inputs: usize,
    text: usize,
    inspections: usize,
}
impl ButtonholeOperationShape {
    /// Actual complete dependency population.
    #[must_use]
    pub const fn policy_inputs(self) -> usize {
        self.inputs
    }
    /// Complete actual original local role text bytes.
    #[must_use]
    pub const fn role_text_bytes(self) -> usize {
        self.text
    }
    /// Conservative complete P*P role comparison bound, not measured execution or string byte work.
    #[must_use]
    pub const fn validation_inspections(self) -> usize {
        self.inspections
    }
    pub(super) fn admit(
        definition: &ButtonholeOperationDefinition,
        limits: ButtonholeOperationLimits,
    ) -> Result<Self, ButtonholeOperationRefusal> {
        use ButtonholeOperationRefusal as R;
        let inputs = definition.inputs.len();
        let inspections = inputs.checked_mul(inputs).ok_or(R::PopulationOverflow)?;
        let text = definition.inputs.iter().try_fold(0_usize, |total, input| {
            total
                .checked_add(input.role.resource_text().len())
                .ok_or(R::PopulationOverflow)
        })?;
        for (resource, allowed, required) in [
            (
                ButtonholeOperationResource::PolicyInputs,
                limits.inputs,
                inputs,
            ),
            (
                ButtonholeOperationResource::RoleTextBytes,
                limits.text,
                text,
            ),
            (
                ButtonholeOperationResource::ValidationInspections,
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
