//! Actual canonical dynamic collections; no copied state, availability reads or registry lookup.
use super::{
    fixed_slice, fixed_value, DesignResourceLimitError, DesignResourceLocation,
    DesignResourceScope, DesignResourceSink,
};
use crate::ontology::EntityId;
use crate::value::{
    CanonicalInputs, LengthDeclaration, LengthDeclarationDefinition, LengthState,
    ScalarInputDeclarationDefinition, ScalarInputState,
};

fn location(
    field: &'static str,
    index: Option<usize>,
    owner: Option<EntityId>,
) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::Structure,
        owner,
        field,
        index,
    }
}
/// Charge actual canonical inventories/evidence vectors, in authored length-then-scalar order.
/// Each inventory is precharged before its members, then each Known evidence vector by exact length.
/// Shared evidence identities in different declarations still count as distinct logical vector entries.
/// Exhaustive owner/state patterns and Copy payload constraints guard the present dynamic-field scope.
/// This borrows only; Unknown/Derived states need no numeric availability. It grants no source truth,
/// whole-model accounting, authority or runtime admission capability. The caller owns/finishes the meter.
/// # Errors
/// Returns the original first resource refusal, with canonical field token and original member index.
pub fn charge_canonical_input_resources(
    inputs: &CanonicalInputs,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    charge_canonical_with_length(inputs, None, sink)
}
// Private prospective view: the command must first establish the exact actual target identity.
// Replace the borrowed declaration at its original slot; never charge old and new evidence together.
pub(super) fn charge_canonical_with_length(
    inputs: &CanonicalInputs,
    replacement: Option<&LengthDeclaration>,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let (lengths, scalars) = inputs.resource_collections();
    sink.charge_entries(lengths.len(), location("canonical.lengths", None, None))?;
    for (index, declaration) in lengths.iter().enumerate() {
        let declaration = replacement
            .filter(|candidate| candidate.id() == declaration.id())
            .unwrap_or(declaration);
        let LengthDeclarationDefinition { id, source, state } = declaration.definition();
        fixed_value(source);
        match state {
            LengthState::Known { value, evidence } => {
                fixed_value(value);
                fixed_slice(evidence);
                sink.charge_entries(
                    evidence.len(),
                    location("canonical.lengths.evidence", Some(index), Some(*id)),
                )?;
            }
            LengthState::Assumed { value, assumption } => {
                fixed_value(value);
                fixed_value(assumption);
            }
            LengthState::Preference { value, provenance } => {
                fixed_value(value);
                fixed_value(provenance);
            }
            LengthState::Unknown { observation } => fixed_value(observation),
            LengthState::Derived { formula } => fixed_value(formula),
        }
    }
    sink.charge_entries(scalars.len(), location("canonical.scalars", None, None))?;
    for (index, declaration) in scalars.iter().enumerate() {
        let ScalarInputDeclarationDefinition {
            id,
            source,
            kind,
            state,
        } = declaration.definition();
        fixed_value(source);
        fixed_value(kind);
        match state {
            ScalarInputState::Known { value, evidence } => {
                fixed_value(value);
                fixed_slice(evidence);
                sink.charge_entries(
                    evidence.len(),
                    location("canonical.scalars.evidence", Some(index), Some(*id)),
                )?;
            }
            ScalarInputState::Assumed { value, assumption } => {
                fixed_value(value);
                fixed_value(assumption);
            }
            ScalarInputState::Preference { value, provenance } => {
                fixed_value(value);
                fixed_value(provenance);
            }
            ScalarInputState::Unknown { observation } => fixed_value(observation),
            ScalarInputState::Derived { formula } => fixed_value(formula),
        }
    }
    Ok(())
}
