//! Original ordered semantic owner and complete actual member content,in one cumulative sink.
use super::{
    charge_buttonhole_operation_resources, charge_logical_derivation_resources, fixed_value,
    DesignResourceLimitError, DesignResourceLocation, DesignResourceScope, DesignResourceSink,
};
use crate::recipe::{SemanticOperation, SemanticRecipe};
/// Precharge the actual complete semantic-operation vector,then each original member's complete
/// owned content in authored order. No enum-valued scalar,native step,Root membership or containing
/// mixed position is invented. Exhaustive owner/variant/member views guard future held fields.
/// Constructor limits/shapes remain fixed Copy metadata,never aggregate content credit. No cloned
/// model,rule/policy/member lookup,source/value,currentness or graph validation occurs.
/// # Errors
/// Original first cumulative sink refusal,with semantic.operations for the complete inventory and
/// each original member's actual own field/owner/input position. The caller owns/finishes the meter.
/// An empty recipe preserves the same earlier latched failure rather than exposing zero-demand success.
pub fn charge_semantic_recipe_resources(
    recipe: &SemanticRecipe,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let (operations, limits, shape) = recipe.resource_parts();
    fixed_value(&limits);
    fixed_value(&shape);
    sink.charge_entries(
        operations.len(),
        DesignResourceLocation {
            scope: DesignResourceScope::Inputs,
            owner: None,
            field: "semantic.operations",
            index: None,
        },
    )?;
    for operation in operations {
        match operation {
            SemanticOperation::Buttonhole(p) => charge_buttonhole_operation_resources(p, sink)?,
            SemanticOperation::LogicalDerivation(p) => {
                charge_logical_derivation_resources(p, sink)?
            }
        }
    }
    Ok(())
}
