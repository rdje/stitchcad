//! Actual canonical dynamic collections; no copied state, availability reads or registry lookup.
use super::{
    DesignResourceLimitError, DesignResourceLocation, DesignResourceScope, DesignResourceSink,
};
use crate::value::{
    CanonicalInputs, LengthDeclarationDefinition, LengthState, ScalarInputDeclarationDefinition,
    ScalarInputState,
};

fn location(field: &'static str, index: Option<usize>) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::Structure,
        field,
        index,
    }
}
// Fixed value payloads own no dynamic content. Removing Copy requires revisiting this accounting.
fn fixed_value<T: Copy>(_: &T) {}

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
    let (lengths, scalars) = inputs.resource_collections();
    sink.charge_entries(lengths.len(), location("canonical.lengths", None))?;
    for (index, declaration) in lengths.iter().enumerate() {
        let LengthDeclarationDefinition {
            id: _,
            source: _,
            state,
        } = declaration.definition();
        match state {
            LengthState::Known { value, evidence } => {
                fixed_value(value);
                sink.charge_entries(
                    evidence.len(),
                    location("canonical.lengths.evidence", Some(index)),
                )?;
            }
            LengthState::Assumed {
                value,
                assumption: _,
            }
            | LengthState::Preference {
                value,
                provenance: _,
            } => fixed_value(value),
            LengthState::Unknown { observation: _ } | LengthState::Derived { formula: _ } => {}
        }
    }
    sink.charge_entries(scalars.len(), location("canonical.scalars", None))?;
    for (index, declaration) in scalars.iter().enumerate() {
        let ScalarInputDeclarationDefinition {
            id: _,
            source: _,
            kind: _,
            state,
        } = declaration.definition();
        match state {
            ScalarInputState::Known { value, evidence } => {
                fixed_value(value);
                sink.charge_entries(
                    evidence.len(),
                    location("canonical.scalars.evidence", Some(index)),
                )?;
            }
            ScalarInputState::Assumed {
                value,
                assumption: _,
            }
            | ScalarInputState::Preference {
                value,
                provenance: _,
            } => fixed_value(value),
            ScalarInputState::Unknown { observation: _ }
            | ScalarInputState::Derived { formula: _ } => {}
        }
    }
    Ok(())
}
