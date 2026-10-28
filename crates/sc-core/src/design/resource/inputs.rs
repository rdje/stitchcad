//! Actual held input metadata, without rebinding, recipe reparse or property availability reads.
use super::{
    fixed_value, DesignResourceLimitError, DesignResourceLocation, DesignResourceScope,
    DesignResourceSink,
};
use crate::{
    design::{DesignParameterDefinition, DesignParameters},
    ontology::{EntityId, MaterialDefinition, MaterialInventory},
    recipe::FormulaRecipeDocument,
};
fn location(
    field: &'static str,
    owner: Option<EntityId>,
    index: Option<usize>,
) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::Inputs,
        owner,
        field,
        index,
    }
}
/// Charge actual parameter collection/token text, without target lookup or numerical availability.
/// Exhaustive private owner/definition/token views preserve future owned-field obligations.
/// # Errors
/// Returns the original first refusal, with actual parameter identity and original authored index.
pub fn charge_parameter_resources(
    parameters: &DesignParameters,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let parameters = parameters.resource_parameters();
    sink.charge_entries(parameters.len(), location("parameters", None, None))?;
    for (index, parameter) in parameters.iter().enumerate() {
        let DesignParameterDefinition { id, token, target } = parameter.resource_definition();
        fixed_value(target);
        sink.charge_text(
            token.resource_text(),
            location("parameters.token", Some(*id), Some(index)),
        )?;
    }
    Ok(())
}
/// Charge the actual retained authored and canonical recipe strings, without reparsing/regeneration.
/// Both logical fields count even when their backing/text could coincide. The document is unchanged.
/// # Errors
/// Returns the original first refusal for recipe.source or recipe.canonical, preserving prior failure.
pub fn charge_recipe_document_resources(
    document: &FormulaRecipeDocument,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let (source, canonical) = document.resource_texts();
    sink.charge_text(source, location("recipe.source", None, None))?;
    sink.charge_text(canonical, location("recipe.canonical", None, None))
}
/// Charge the actual Material collection/name/optional composition, without canonical property lookup.
/// Targets/shrinkage remain fixed Copy metadata, not a second value/evidence cache or applied scale.
/// No source truth, physics, whole-model admission or publication authority follows from accounting.
/// # Errors
/// Returns the original first refusal, retaining actual Material identity/field/original authored index.
pub fn charge_material_inventory_resources(
    inventory: &MaterialInventory,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let materials = inventory.resource_materials();
    sink.charge_entries(materials.len(), location("materials", None, None))?;
    for (index, material) in materials.iter().enumerate() {
        let MaterialDefinition {
            id,
            source,
            name,
            composition,
            width,
            nap,
            directional_print,
            striped,
            checked,
            shrinkage,
        } = material.resource_definition();
        fixed_value(source);
        for target in [width, nap, directional_print, striped, checked] {
            fixed_value(target);
        }
        fixed_value(shrinkage);
        sink.charge_text(name, location("materials.name", Some(*id), Some(index)))?;
        if let Some(composition) = composition {
            sink.charge_text(
                composition,
                location("materials.composition", Some(*id), Some(index)),
            )?;
        }
    }
    Ok(())
}
