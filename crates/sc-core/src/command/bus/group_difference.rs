//! Net actual group differences; per-step reports define the permitted semantic footprint only.
use super::{
    difference, CommandDifferenceError, CommandGroupPreviewStep, CommandLengthChange,
    CommandMaterialChange, CommandPreview,
};
use crate::{
    design::{DesignGarment, DesignReferenceHost},
    ontology::{Notch, SewingLandmark},
};
use std::collections::BTreeSet;

/// Exact net metadata changes, omitting values restored to their original state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandGroupDifference {
    lengths: Vec<CommandLengthChange>,
    materials: Vec<CommandMaterialChange>,
    notches: Vec<Notch>,
}
impl CommandGroupDifference {
    /// Actual changed dedicated declarations in original canonical inventory order.
    #[must_use]
    pub fn lengths(&self) -> &[CommandLengthChange] {
        &self.lengths
    }
    /// Actual changed assignments in original Piece inventory order.
    #[must_use]
    pub fn materials(&self) -> &[CommandMaterialChange] {
        &self.materials
    }
    /// Actual appended semantic Notches in candidate append order, with tentative identities.
    #[must_use]
    pub fn notches(&self) -> &[Notch] {
        &self.notches
    }
}

pub(super) fn between<H: DesignReferenceHost + PartialEq>(
    before: &DesignGarment<H>,
    after: &DesignGarment<H>,
    steps: &[CommandGroupPreviewStep],
) -> Result<CommandGroupDifference, CommandDifferenceError> {
    difference::unchanged_noncanonical_context(before, after)?;
    let mut length_ids = BTreeSet::new();
    let mut piece_ids = BTreeSet::new();
    let mut notches = Vec::new();
    for step in steps.iter().filter(|step| step.replay_of().is_none()) {
        match step.report() {
            CommandPreview::Measurement(report) => {
                length_ids.extend(report.changes().iter().map(|c| c.after().id()))
            }
            CommandPreview::Material(report) => {
                piece_ids.extend(report.changes().iter().map(CommandMaterialChange::piece))
            }
            CommandPreview::Notch(report) => notches.push(report.notch().clone()),
        }
    }
    let mut lengths = Vec::new();
    let mut normalized = after.inputs().inputs().clone();
    for original in before.inputs().inputs().lengths() {
        let candidate = after
            .inputs()
            .inputs()
            .length(original.id())
            .ok_or(CommandDifferenceError::CanonicalInputs)?;
        if original != candidate {
            if !length_ids.contains(&original.id()) {
                return Err(CommandDifferenceError::CanonicalInputs);
            }
            lengths.push(CommandLengthChange {
                before: original.clone(),
                after: candidate.clone(),
            });
            normalized = normalized
                .with_length_declaration(original.clone())
                .map_err(|_| CommandDifferenceError::CanonicalInputs)?;
        }
    }
    // Full equality also checks dedicated inventory identity/order, general scalars and their order.
    if &normalized != before.inputs().inputs() {
        return Err(CommandDifferenceError::CanonicalInputs);
    }
    let originals = before.inputs().structure().pieces().pieces();
    let candidates = after.inputs().structure().pieces().pieces();
    if originals.len() != candidates.len() {
        return Err(CommandDifferenceError::Pieces);
    }
    let mut materials = Vec::new();
    for (original, candidate) in originals.iter().zip(candidates) {
        if original.id() != candidate.id() {
            return Err(CommandDifferenceError::Pieces);
        }
        if original.definition().material != candidate.definition().material {
            if !piece_ids.contains(&original.id()) {
                return Err(CommandDifferenceError::Pieces);
            }
            materials.push(CommandMaterialChange {
                piece: original.id(),
                before: original.definition().material.clone(),
                after: candidate.definition().material.clone(),
            });
        }
        if original
            .with_material_assignment(candidate.definition().material.clone())
            .as_ref()
            != Ok(candidate)
        {
            return Err(CommandDifferenceError::Pieces);
        }
    }
    let expected: Vec<_> = before
        .sewing()
        .landmarks()
        .iter()
        .cloned()
        .chain(notches.iter().cloned().map(SewingLandmark::Notch))
        .collect();
    if after.sewing().landmarks() != expected {
        return Err(CommandDifferenceError::Landmarks);
    }
    Ok(CommandGroupDifference {
        lengths,
        materials,
        notches,
    })
}
