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
/// Charge actual canonical content in authored length, scalar, then finite logical order.
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
    let (lengths, scalars, logicals) = inputs.resource_collections();
    sink.charge_entries(lengths.len(), location("canonical.lengths", None, None))?;
    for (index, declaration) in lengths.iter().enumerate() {
        let declaration = replacement
            .filter(|candidate| candidate.id() == declaration.id())
            .unwrap_or(declaration);
        charge_length_declaration_resources(
            declaration,
            location(
                "canonical.lengths.evidence",
                Some(index),
                Some(declaration.id()),
            ),
            sink,
        )?;
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
    super::charge_logical_input_resources(logicals, sink)
}

// Shared actual declaration snapshot accounting. The caller supplies its static original containing
// scope/location; no model/source lookup, numerical availability or ownership reconstruction occurs.
pub(crate) fn charge_length_declaration_resources(
    declaration: &LengthDeclaration,
    location: DesignResourceLocation,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    sink.charge_entries(0, location)?;
    let LengthDeclarationDefinition { id, source, state } = declaration.resource_definition();
    fixed_value(id);
    fixed_value(source);
    match state {
        LengthState::Known { value, evidence } => {
            fixed_value(value);
            fixed_slice(evidence);
            sink.charge_entries(evidence.len(), location)?;
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
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        design::{DesignResourceCount, DesignResourceLimits, DesignResourceMeter},
        name::MachineToken,
        value::*,
    };
    use core::num::{NonZeroU64, NonZeroUsize};
    #[test]
    fn prospective_length_charges_original_logical_content_after_only_replacement_evidence(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let id = EntityId::from_bits;
        let original = LengthDeclaration::new(LengthDeclarationDefinition {
            id: id(10),
            source: id(90),
            state: LengthState::Unknown {
                observation: id(91),
            },
        })?;
        let replacement = LengthDeclaration::new(LengthDeclarationDefinition {
            id: id(10),
            source: id(90),
            state: LengthState::Known {
                value: sc_units::Length::ZERO,
                evidence: vec![id(92), id(93)],
            },
        })?;
        let logical = LogicalInputDeclaration::new(
            LogicalInputDeclarationDefinition {
                id: id(20),
                source: id(94),
                kind: LogicalInputKind::NotionSize,
                domain: vec![MachineToken::new("small")?],
                state: LogicalInputState::Choice {
                    value: MachineToken::new("small")?,
                    decision: id(95),
                },
            },
            LogicalInputLimits::new(1, 0, 10, 2),
        )?;
        let inputs = CanonicalInputs::with_logicals(
            vec![original],
            vec![],
            LogicalInputs::new(vec![logical], LogicalInputInventoryLimits::new(1, 1))?,
        )?;
        let limits = |text| DesignResourceLimits::new(NonZeroUsize::MAX, text, NonZeroU64::MAX);
        let mut actual = DesignResourceMeter::new(limits(NonZeroUsize::MAX));
        charge_canonical_input_resources(&inputs, &mut actual)?;
        let actual = actual.finish()?;
        assert_eq!((actual.collection_entries(), actual.text_bytes()), (3, 10));
        let mut candidate = DesignResourceMeter::new(limits(NonZeroUsize::MAX));
        charge_canonical_with_length(&inputs, Some(&replacement), &mut candidate)?;
        let candidate = candidate.finish()?;
        assert_eq!(
            (candidate.collection_entries(), candidate.text_bytes()),
            (5, 10)
        );
        let mut short =
            DesignResourceMeter::new(limits(NonZeroUsize::new(9).ok_or("positive text")?));
        let expected = DesignResourceLimitError::Count {
            unit: DesignResourceCount::TextBytes,
            maximum: 9,
            consumed: 5,
            requested: 5,
            location: location("canonical.logicals.value", Some(0), Some(id(20))),
        };
        assert_eq!(
            charge_canonical_with_length(&inputs, Some(&replacement), &mut short),
            Err(expected)
        );
        assert_eq!(short.finish(), Err(expected));
        assert!(matches!(
            inputs.length(id(10)).map(LengthDeclaration::state),
            Some(LengthState::Unknown { .. })
        ));
        Ok(())
    }
}
