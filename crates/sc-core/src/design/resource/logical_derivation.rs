//! Actual standalone logical recipe content,without source/graph queries or cached-shape credit.
use super::{
    fixed_value, DesignResourceLimitError, DesignResourceLocation, DesignResourceScope,
    DesignResourceSink,
};
use crate::{
    recipe::{
        LogicalDerivationDefinition, LogicalDerivationInput, LogicalDerivationInputSource,
        LogicalDerivationOperation,
    },
    value::LogicalInputTarget,
};
/// Charge the complete actual typed-input vector,then original local role text in authored order.
/// No containing semantic recipe entry or position is invented. Output,rule and typed source refs
/// remain fixed Copy fields; constructor limits/shapes supply no content credit. Exhaustive original
/// owner/definition/input/source views expose future accounting obligations. This performs no member,
/// domain,rule signature,Profile pin,currentness or value query and grants no Root membership.
/// # Errors
/// Original first cumulative sink refusal with actual operation owner and original input position.
/// The caller owns/finishes the meter; an empty vector preserves a previously latched cause.
pub fn charge_logical_derivation_resources(
    operation: &LogicalDerivationOperation,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let (definition, limits, shape) = operation.resource_parts();
    fixed_value(&limits);
    fixed_value(&shape);
    let LogicalDerivationDefinition {
        id,
        output,
        rule,
        inputs,
    } = definition;
    fixed_value(id);
    fixed_value(rule);
    fixed_value(output);
    let LogicalInputTarget { declaration, kind } = output;
    fixed_value(declaration);
    fixed_value(kind);
    let location = |field, index| DesignResourceLocation {
        scope: DesignResourceScope::Inputs,
        owner: Some(*id),
        field,
        index,
    };
    sink.charge_entries(inputs.len(), location("semantic.logical.inputs", None))?;
    for (index, input) in inputs.iter().enumerate() {
        let LogicalDerivationInput { role, source } = input;
        match source {
            LogicalDerivationInputSource::Logical(target) => {
                fixed_value(target);
                let LogicalInputTarget { declaration, kind } = target;
                fixed_value(declaration);
                fixed_value(kind);
            }
            LogicalDerivationInputSource::Profile { parameter, kind } => {
                fixed_value(parameter);
                fixed_value(kind);
            }
            LogicalDerivationInputSource::Numeric(target) => fixed_value(target),
        }
        sink.charge_text(
            role.resource_text(),
            location("semantic.logical.inputs.role", Some(index)),
        )?;
    }
    Ok(())
}
