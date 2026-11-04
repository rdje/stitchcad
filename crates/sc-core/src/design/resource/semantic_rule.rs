//! Exhaustive original rule declaration fields, separate from opaque body/source accounting.
use super::{
    fixed_value, DesignResourceLimitError, DesignResourceLocation, DesignResourceScope,
    DesignResourceSink,
};
use crate::recipe::{
    ButtonholePolicyRequirement, CanonicalNumericInputKind, SemanticRuleDeclaration,
    SemanticRuleDefinition, SemanticRuleInput, SemanticRuleInputKind, SemanticRuleSignature,
};

/// Charge the complete original signature vector before original role text in declaration order.
/// Caller supplies its actual containing scope; owner is the actual rule/policy identity. No
/// containing collection entry/index is invented. Constructor policy/shape grants no content credit.
/// Exhaustive owner/definition/variant/input views and fixed Copy guards expose future fields.
/// This accounts only this declaration; integrations must also walk actual bodies,unused records,
/// copies,caches and counters. It queries no source/value and grants no Root/Profile association.
/// # Errors
/// Original first cumulative sink refusal. Caller owns/finishes the meter; empty vectors preserve
/// any previously latched refusal. Role indices are original zero-based declaration positions.
pub fn charge_semantic_rule_resources(
    rule: &SemanticRuleDeclaration,
    sink: &mut dyn DesignResourceSink,
    scope: DesignResourceScope,
) -> Result<(), DesignResourceLimitError> {
    let (definition, limits, shape) = rule.resource_parts();
    fixed_value(&limits);
    fixed_value(&shape);
    let SemanticRuleDefinition { id, signature } = definition;
    fixed_value(id);
    let location = |field, index| DesignResourceLocation {
        scope,
        owner: Some(*id),
        field,
        index,
    };
    match signature {
        SemanticRuleSignature::Logical { result, inputs } => {
            fixed_value(result);
            sink.charge_entries(inputs.len(), location("semantic.rule.inputs", None))?;
            for (index, input) in inputs.iter().enumerate() {
                let SemanticRuleInput { role, kind } = input;
                match kind {
                    SemanticRuleInputKind::Logical(kind) => fixed_value(kind),
                    SemanticRuleInputKind::Numeric(kind) => numeric(kind),
                }
                sink.charge_text(
                    role.resource_text(),
                    location("semantic.rule.inputs.role", Some(index)),
                )?;
            }
        }
        SemanticRuleSignature::Buttonhole { inputs } => {
            sink.charge_entries(inputs.len(), location("semantic.rule.inputs", None))?;
            for (index, input) in inputs.iter().enumerate() {
                let ButtonholePolicyRequirement { role, kind } = input;
                numeric(kind);
                sink.charge_text(
                    role.resource_text(),
                    location("semantic.rule.inputs.role", Some(index)),
                )?;
            }
        }
    }
    Ok(())
}
fn numeric(kind: &CanonicalNumericInputKind) {
    match kind {
        CanonicalNumericInputKind::Length => {}
        CanonicalNumericInputKind::Scalar(kind) => fixed_value(kind),
    }
}
