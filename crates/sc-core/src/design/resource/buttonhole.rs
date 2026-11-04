//! Complete original standalone semantic operation fields, without cached-shape credit.
use super::{
    fixed_value, DesignResourceLimitError, DesignResourceLocation, DesignResourceScope,
    DesignResourceSink,
};
use crate::recipe::{ButtonholeOperation, ButtonholeOperationDefinition, ButtonholePolicyInput};
/// Charge an actual standalone Buttonhole operation's complete policy-input vector,then every
/// original local role token in authored order. No containing recipe entry/index is invented.
/// Button origin,policy reference and numeric targets remain fixed Copy fields; constructor
/// limits/shapes provide no content credit. Exhaustive owner/definition/input views expose new fields.
/// This creates no meter,cloned input,source lookup,value,Root membership or physical result.
/// # Errors
/// Original first cumulative sink refusal with the actual operation owner and policy-input index.
/// The caller owns/finishes the meter; an empty vector still retains any previously latched cause.
pub fn charge_buttonhole_operation_resources(
    operation: &ButtonholeOperation,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let (definition, limits, shape) = operation.resource_parts();
    fixed_value(&limits);
    fixed_value(&shape);
    let ButtonholeOperationDefinition {
        id,
        button,
        policy,
        inputs,
    } = definition;
    fixed_value(id);
    fixed_value(button);
    fixed_value(policy);
    let location = |field, index| DesignResourceLocation {
        scope: DesignResourceScope::Inputs,
        owner: Some(*id),
        field,
        index,
    };
    sink.charge_entries(inputs.len(), location("semantic.buttonhole.inputs", None))?;
    for (index, input) in inputs.iter().enumerate() {
        let ButtonholePolicyInput { role, target } = input;
        fixed_value(target);
        sink.charge_text(
            role.resource_text(),
            location("semantic.buttonhole.inputs.role", Some(index)),
        )?;
    }
    Ok(())
}
