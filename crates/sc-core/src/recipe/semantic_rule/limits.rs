//! Complete original signature bounds, without body/source/query or cached accounting credit.
use super::{SemanticRuleRefusal, SemanticRuleSignature};
/// Explicit whole constructor resource classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemanticRuleResource {
    /// Complete named input requirement population.
    Inputs,
    /// Complete separately owned local role text bytes.
    RoleTextBytes,
    /// Conservative complete P*P role comparisons.
    ValidationInspections,
}
/// Explicit complete signature allowances; no implicit policy or default rule.
/// ```compile_fail
/// let limits:sc_core::recipe::SemanticRuleLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SemanticRuleLimits {
    inputs: usize,
    text: usize,
    inspections: usize,
}
impl SemanticRuleLimits {
    /// Select every complete allowance; all-zero explicitly admits an empty signature.
    #[must_use]
    pub const fn new(inputs: usize, role_text_bytes: usize, validation_inspections: usize) -> Self {
        Self {
            inputs,
            text: role_text_bytes,
            inspections: validation_inspections,
        }
    }
    /// Maximum complete named requirement count.
    #[must_use]
    pub const fn inputs(self) -> usize {
        self.inputs
    }
    /// Maximum complete original local role bytes.
    #[must_use]
    pub const fn role_text_bytes(self) -> usize {
        self.text
    }
    /// Maximum conservative complete P*P comparisons.
    #[must_use]
    pub const fn validation_inspections(self) -> usize {
        self.inspections
    }
}
/// Constructor-derived complete populations; no whole-source or runtime grant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SemanticRuleShape {
    inputs: usize,
    text: usize,
    inspections: usize,
}
impl SemanticRuleShape {
    /// Complete original named requirement population.
    #[must_use]
    pub const fn inputs(self) -> usize {
        self.inputs
    }
    /// Complete original separately owned role bytes.
    #[must_use]
    pub const fn role_text_bytes(self) -> usize {
        self.text
    }
    /// Conservative complete P*P comparisons, not measured CPU or string byte work.
    #[must_use]
    pub const fn validation_inspections(self) -> usize {
        self.inspections
    }
    pub(super) fn admit(
        signature: &SemanticRuleSignature,
        limits: SemanticRuleLimits,
    ) -> Result<Self, SemanticRuleRefusal> {
        use SemanticRuleRefusal as R;
        let inputs = match signature {
            SemanticRuleSignature::Logical { inputs, result: _ } => inputs.len(),
            SemanticRuleSignature::Buttonhole { inputs } => inputs.len(),
        };
        let inspections = inputs.checked_mul(inputs).ok_or(R::PopulationOverflow)?;
        let text = signature.roles().try_fold(0_usize, |total, role| {
            total
                .checked_add(role.resource_text().len())
                .ok_or(R::PopulationOverflow)
        })?;
        for (resource, allowed, required) in [
            (SemanticRuleResource::Inputs, limits.inputs, inputs),
            (SemanticRuleResource::RoleTextBytes, limits.text, text),
            (
                SemanticRuleResource::ValidationInspections,
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
