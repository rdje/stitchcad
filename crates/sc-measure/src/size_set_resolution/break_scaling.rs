//! Explicit signed break/Ratio source bindings; no numeric application or copied authored states.
use super::transformation::SizeSetTransformationError as Error;
use crate::{SizeBreak, SizeSetInputs};
use sc_core::{
    ontology::EntityId,
    value::{LengthDeclaration, ScalarInputDeclaration, ScalarInputKind},
};

/// Explicit proposed scaling of one available authored break by one canonical Ratio declaration.
/// The transformation definition supplies the exact source SizeSet pin and its own state/evidence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SizeBreakScaling {
    /// Actual available signed break identity; its axis/POM/adjacent pair is never reconstructed.
    pub size_break: EntityId,
    /// Actual canonical general scalar identity, explicitly declared Ratio.
    pub factor: EntityId,
}
/// Original current records behind scaling intent. Reading values preserves their canonical refusals.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SizeBreakScalingInputs<'a> {
    /// Original authored signed break, with its exact axis/POM/adjacent pair binding.
    pub size_break: &'a SizeBreak,
    /// Original dedicated canonical Length delta, including its complete state and provenance.
    pub delta: &'a LengthDeclaration,
    /// Original canonical Ratio scalar; no copied value or independent state.
    pub factor: &'a ScalarInputDeclaration,
}
impl SizeBreakScaling {
    /// Resolve actual current break, dedicated Length delta and general Ratio scalar records.
    /// Zero, negative, Unknown and Derived factors remain intact; G4 owns constraints and execution.
    /// # Errors
    /// Returns original current input/break evidence or a missing/wrong-role/wrong-kind factor.
    pub fn inputs(self, inputs: SizeSetInputs<'_>) -> Result<SizeBreakScalingInputs<'_>, Error> {
        inputs
            .validate_current()
            .map_err(|issue| Error::Inputs(Box::new(issue)))?;
        let size_break = inputs
            .breaks()
            .definition()
            .breaks
            .iter()
            .find(|value| value.definition().id == self.size_break)
            .ok_or(Error::MissingBreak(self.size_break))?;
        let delta = size_break
            .delta(inputs.break_context())
            .map_err(|issue| Error::Break(Box::new(issue)))?;
        let factor = inputs
            .break_context()
            .sources()
            .scalar(self.factor)
            .ok_or_else(|| {
                if inputs.contains_entity(self.factor) {
                    Error::FactorRole(self.factor)
                } else {
                    Error::MissingFactor(self.factor)
                }
            })?;
        if factor.kind() != ScalarInputKind::Ratio {
            return Err(Error::FactorKind {
                factor: self.factor,
                actual: factor.kind(),
            });
        }
        Ok(SizeBreakScalingInputs {
            size_break,
            delta,
            factor,
        })
    }
}
