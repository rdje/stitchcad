//! Validation inspects actual member identities and proposed labels without mutating the source.
use super::{SizeSetTransformation as Transformation, SizeSetTransformationError as Error, *};
use std::collections::{BTreeMap, BTreeSet};
impl Transformation {
    /// Recheck the exact source pin, local identities, evidence and authored operation.
    /// Unknown/Derived stay value-less; no profile constraints or formula execution is attempted.
    /// # Errors
    /// Returns the original current-input failure or exact structural conflict.
    pub fn validate_current(&self, inputs: SizeSetInputs<'_>) -> Result<(), Error> {
        inputs
            .validate_current()
            .map_err(|issue| Error::Inputs(Box::new(issue)))?;
        let definition = &self.definition;
        if definition.reference != inputs.reference() {
            return Err(Error::SourceMismatch {
                expected: definition.reference,
                actual: inputs.reference(),
            });
        }
        if definition.id == definition.source || inputs.contains_entity(definition.id) {
            return Err(Error::DuplicateIdentity(definition.id));
        }
        if let SizeSetTransformationState::Known { evidence, .. } = &definition.state {
            if evidence.is_empty() {
                return Err(Error::NoEvidence(definition.id));
            }
            let mut seen = BTreeSet::new();
            for identity in evidence {
                if !seen.insert(*identity) {
                    return Err(Error::DuplicateEvidence(*identity));
                }
            }
        }
        let Ok(value) = self.authored_value() else {
            return Ok(());
        };
        if value.kind() != definition.kind {
            return Err(Error::KindMismatch {
                expected: definition.kind,
                actual: value.kind(),
            });
        }
        match value {
            SizeSetTransformationValue::BaseSubstitution(member) => {
                inputs
                    .membership()
                    .member(*member)
                    .map_err(|_| Error::MissingMember(*member))?;
            }
            SizeSetTransformationValue::LabelMapping(mapping) => validate_labels(mapping, inputs)?,
        }
        Ok(())
    }
}
fn validate_labels(mapping: &[SizeLabelMapping], inputs: SizeSetInputs<'_>) -> Result<(), Error> {
    if mapping.is_empty() {
        return Err(Error::EmptyMapping);
    }
    let mut mapped = BTreeMap::new();
    for entry in mapping {
        inputs
            .membership()
            .member(entry.member)
            .map_err(|_| Error::MissingMember(entry.member))?;
        if mapped.insert(entry.member, &entry.label).is_some() {
            return Err(Error::DuplicateMember(entry.member));
        }
    }
    let mut labels = BTreeMap::new();
    for member in &inputs.membership().definition().members {
        let label = mapped.get(&member.id).copied().unwrap_or(&member.label);
        if let Some(first) = labels.insert(label.as_str(), member.id) {
            return Err(Error::DuplicateLabel {
                first,
                second: member.id,
                label: label.clone(),
            });
        }
    }
    Ok(())
}
