//! Actual bound canonical measurement replacement; recipe evaluation is a separate dependency.
use super::{CommandPreview, CommandPreviewError, PreparationBase, PreparedCommand};
use crate::{
    command::{ChangeMeasurement, DesignRevision},
    design::{DesignGarment, DesignInputs, DesignResourceSource, DesignSewing, DesignStructure},
    ontology::EntityId,
    value::LengthDeclaration,
};

/// One actual changed canonical length declaration, with original source/state/provenance intact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandLengthChange {
    pub(super) before: LengthDeclaration,
    pub(super) after: LengthDeclaration,
}
impl CommandLengthChange {
    /// Original canonical declaration; its identity is stable across replacement.
    #[must_use]
    pub const fn before(&self) -> &LengthDeclaration {
        &self.before
    }
    /// Actual candidate declaration, retaining its exact supplied source and authored state.
    #[must_use]
    pub const fn after(&self) -> &LengthDeclaration {
        &self.after
    }
}

/// Metadata-only result of actual bound ChangeMeasurement candidate validation; no evaluated value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandMeasurementPreview {
    design: EntityId,
    revision: DesignRevision,
    measurement: EntityId,
    changes: Vec<CommandLengthChange>,
}
impl CommandMeasurementPreview {
    pub(super) fn resource_parts(
        &self,
    ) -> (EntityId, &DesignRevision, EntityId, &[CommandLengthChange]) {
        let Self {
            design,
            revision,
            measurement,
            changes,
        } = self;
        (*design, revision, *measurement, changes)
    }
    /// Actual inspected Design identity.
    #[must_use]
    pub const fn design(&self) -> EntityId {
        self.design
    }
    /// Exact unchanged base revision.
    #[must_use]
    pub const fn revision(&self) -> &DesignRevision {
        &self.revision
    }
    /// Original resolved Measurement metadata identity, distinct from its declaration identity.
    #[must_use]
    pub const fn measurement(&self) -> EntityId {
        self.measurement
    }
    /// Zero changes for an exact no-op, otherwise the one actual canonical declaration replacement.
    #[must_use]
    pub fn changes(&self) -> &[CommandLengthChange] {
        &self.changes
    }
}

impl<H: DesignResourceSource + Clone + PartialEq> PreparationBase<'_, H> {
    pub(super) fn prepare_measurement(
        &self,
        replacement: &ChangeMeasurement,
    ) -> Result<PreparedCommand<H>, CommandPreviewError<H::Error>> {
        let inputs = self.state.inputs();
        let bound = inputs
            .sources()
            .measurement_declaration(replacement.measurement(), inputs.inputs())
            .map_err(|e| CommandPreviewError::MeasurementBinding(Box::new(e)))?;
        if bound.id() != replacement.replacement().id() {
            return Err(CommandPreviewError::MeasurementDeclarationMismatch {
                measurement: replacement.measurement(),
                expected: bound.id(),
                requested: replacement.replacement().id(),
            });
        }
        super::operation::admit_length_replacement(
            self.state,
            self.limits,
            replacement.replacement(),
        )?;
        let mut garment = self.state.definition().clone();
        let mut sewing = garment.sewing.into_definition();
        let mut inputs = sewing.inputs.into_definition();
        let mut structure = inputs.structure.into_definition();
        structure.inputs = structure
            .inputs
            .with_length_declaration(replacement.replacement().clone())
            .map_err(|e| CommandPreviewError::CanonicalInputs(Box::new(e)))?;
        inputs.structure = DesignStructure::new(structure)
            .map_err(|e| CommandPreviewError::Structure(Box::new(e)))?;
        sewing.inputs =
            DesignInputs::new(inputs).map_err(|e| CommandPreviewError::Inputs(Box::new(e)))?;
        garment.sewing =
            DesignSewing::new(sewing).map_err(|e| CommandPreviewError::Sewing(Box::new(e)))?;
        let candidate =
            DesignGarment::new(garment).map_err(|e| CommandPreviewError::Candidate(Box::new(e)))?;
        candidate
            .validate_current()
            .map_err(|e| CommandPreviewError::Candidate(Box::new(e)))?;
        super::difference::unchanged_noncanonical_context(self.state, &candidate)
            .map_err(CommandPreviewError::Difference)?;
        if self.state.inputs().structure().pieces() != candidate.inputs().structure().pieces() {
            return Err(CommandPreviewError::Difference(
                super::CommandDifferenceError::Pieces,
            ));
        }
        if self.state.sewing().landmarks() != candidate.sewing().landmarks() {
            return Err(CommandPreviewError::Difference(
                super::CommandDifferenceError::Landmarks,
            ));
        }
        let changes = super::difference::length_changes(
            self.state.inputs().inputs(),
            candidate.inputs().inputs(),
            replacement,
        )
        .map_err(CommandPreviewError::Difference)?;
        let report = CommandMeasurementPreview {
            design: candidate.inputs().id(),
            revision: candidate.inputs().revision().clone(),
            measurement: replacement.measurement(),
            changes,
        };
        Ok(PreparedCommand {
            state: candidate,
            report: CommandPreview::Measurement(report),
        })
    }
}
