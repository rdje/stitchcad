//! Actual standalone logical content, without cached-demand credit or source/value lookup.
use super::{
    fixed_slice, fixed_value, DesignResourceLimitError, DesignResourceLocation,
    DesignResourceScope, DesignResourceSink,
};
use crate::{
    ontology::EntityId,
    value::{
        LogicalInputDeclaration, LogicalInputDeclarationDefinition, LogicalInputState,
        LogicalInputs,
    },
};
fn at(
    field: &'static str,
    owner: Option<EntityId>,
    index: Option<usize>,
) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::Structure,
        owner,
        field,
        index,
    }
}
/// Charge the complete actual standalone logical vector,then each original record in authored
/// order. Precharge every collection before visiting its fields. State selections and equal domain
/// members are separately owned text; repeated evidence in separate records is not deduplicated.
/// Fixed constructor allowances/shapes are checked as Copy payloads,never used as demand credit.
/// No validation,member lookup,cloning,normalization,source,currentness or numeric query occurs.
/// Canonical Design accounting composes this same walk. Standalone use supplies no containing
/// Root membership,whole-source admission,allocator capacity or runtime proof.
///
/// ```
/// use sc_core::{design::*,value::LogicalInputs};
/// fn logical_content(inputs:&LogicalInputs,limits:DesignResourceLimits)
///     ->Result<DesignResourceDemand,DesignResourceLimitError> {
///     let mut meter=DesignResourceMeter::new(limits);
///     charge_logical_input_resources(inputs,&mut meter)?;
///     meter.finish()
/// }
/// ```
/// # Errors
/// Original first sink refusal,retaining original field/owner/index. Empty scopes preserve a
/// previously latched failure;the caller owns and finishes the cumulative meter.
pub fn charge_logical_input_resources(
    inputs: &LogicalInputs,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let (declarations, limits, shape) = inputs.resource_parts();
    fixed_value(&limits);
    fixed_value(&shape);
    sink.charge_entries(declarations.len(), at("canonical.logicals", None, None))?;
    for (index, declaration) in declarations.iter().enumerate() {
        charge_declaration(declaration, Some(index), sink)?;
    }
    Ok(())
}
/// Charge one actual logical declaration snapshot without inventing an inventory membership index.
/// Domain-member text retains its original domain position;other fields have no invented index.
/// All dynamic definition/state fields count and constructor proof fields remain fixed metadata.
/// No source validation,selection computation or cached shape total supplies accounting credit.
/// # Errors
/// Original first sink refusal,with actual declaration identity and original field/member index.
pub fn charge_logical_declaration_resources(
    declaration: &LogicalInputDeclaration,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    charge_declaration(declaration, None, sink)
}
fn charge_declaration(
    declaration: &LogicalInputDeclaration,
    index: Option<usize>,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let (definition, limits, shape) = declaration.resource_parts();
    fixed_value(&limits);
    fixed_value(&shape);
    let LogicalInputDeclarationDefinition {
        id,
        source,
        kind,
        domain,
        state,
    } = definition;
    fixed_value(id);
    fixed_value(source);
    fixed_value(kind);
    sink.charge_entries(
        domain.len(),
        at("canonical.logicals.domain", Some(*id), index),
    )?;
    for (member, token) in domain.iter().enumerate() {
        sink.charge_text(
            token.resource_text(),
            at("canonical.logicals.domain.member", Some(*id), Some(member)),
        )?;
    }
    match state {
        LogicalInputState::Known { value, evidence } => {
            sink.charge_text(
                value.resource_text(),
                at("canonical.logicals.value", Some(*id), index),
            )?;
            fixed_slice(evidence);
            sink.charge_entries(
                evidence.len(),
                at("canonical.logicals.evidence", Some(*id), index),
            )?;
        }
        LogicalInputState::Assumed { value, assumption } => {
            sink.charge_text(
                value.resource_text(),
                at("canonical.logicals.value", Some(*id), index),
            )?;
            fixed_value(assumption);
        }
        LogicalInputState::Choice { value, decision } => {
            sink.charge_text(
                value.resource_text(),
                at("canonical.logicals.value", Some(*id), index),
            )?;
            fixed_value(decision);
        }
        LogicalInputState::Preference { value, provenance } => {
            sink.charge_text(
                value.resource_text(),
                at("canonical.logicals.value", Some(*id), index),
            )?;
            fixed_value(provenance);
        }
        LogicalInputState::Unknown { observation } => fixed_value(observation),
        LogicalInputState::Derived { operation } => fixed_value(operation),
    }
    Ok(())
}
