//! Explicit owned external SizeSet family; neither labels nor member count select provenance.
use crate::{
    GarmentSizeSet, GarmentSizeSetError, GarmentSizeSetPosition, MeasurementInputs, MtmSizeSet,
    MtmSizeSetError, MtmSizeSetPosition, SizeInstantiationPath, SizeSetInputs, SizeSetMetadata,
    SizeSetReference,
};
use core::fmt;
use sc_core::{ontology::EntityId, value::CanonicalInputs};

/// Explicit actual owned family, without inferring MTM from a custom singleton garment chart.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SizeSet {
    /// Authored garment-chart sources, including possible singleton or partial drafts.
    Garment(GarmentSizeSet),
    /// Authored custom singleton body/Ease sources, with original grading refusal.
    Mtm(MtmSizeSet),
}
impl From<GarmentSizeSet> for SizeSet {
    fn from(value: GarmentSizeSet) -> Self {
        Self::Garment(value)
    }
}
impl From<MtmSizeSet> for SizeSet {
    fn from(value: MtmSizeSet) -> Self {
        Self::Mtm(value)
    }
}
/// Original owned source location, retaining the explicit actual family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SizeSetPosition {
    /// Actual garment source position.
    Garment(GarmentSizeSetPosition),
    /// Actual MTM source position.
    Mtm(MtmSizeSetPosition),
}
/// Original concrete family refusal, without retagging provenance or substituting another set.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SizeSetError {
    /// Original actual garment source cause.
    Garment(Box<GarmentSizeSetError>),
    /// Original actual MTM source cause.
    Mtm(Box<MtmSizeSetError>),
}
impl fmt::Display for SizeSetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Garment(issue) => write!(f, "garment SizeSet: {issue}"),
            Self::Mtm(issue) => write!(f, "MTM SizeSet: {issue}"),
        }
    }
}
impl std::error::Error for SizeSetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Garment(issue) => Some(issue.as_ref()),
            Self::Mtm(issue) => Some(issue.as_ref()),
        }
    }
}
impl SizeSet {
    /// Exact actual set identity/revision, independent of labels and containing Design revision.
    #[must_use]
    pub const fn reference(&self) -> SizeSetReference {
        match self {
            Self::Garment(set) => set.reference(),
            Self::Mtm(set) => set.reference(),
        }
    }
    /// Actual common source owner, borrowed without duplicating values or reconstructing membership.
    #[must_use]
    pub const fn metadata(&self) -> &SizeSetMetadata {
        match self {
            Self::Garment(set) => &set.definition().metadata,
            Self::Mtm(set) => &set.definition().metadata,
        }
    }
    /// Actual complete owned source identities in original family/authored order.
    #[must_use]
    pub fn identities(&self) -> Vec<(EntityId, SizeSetPosition)> {
        match self {
            Self::Garment(set) => set
                .identities()
                .into_iter()
                .map(|(id, position)| (id, SizeSetPosition::Garment(position)))
                .collect(),
            Self::Mtm(set) => set
                .identities()
                .into_iter()
                .map(|(id, position)| (id, SizeSetPosition::Mtm(position)))
                .collect(),
        }
    }
    /// Reuse the explicit actual family's complete current source checks, without coverage.
    ///
    /// # Errors
    /// Preserves the original actual family source cause.
    pub fn validate_current(
        &self,
        canonical: &CanonicalInputs,
        inputs: &MeasurementInputs,
    ) -> Result<(), SizeSetError> {
        match self {
            Self::Garment(set) => set
                .validate_current(canonical, inputs)
                .map_err(|issue| SizeSetError::Garment(Box::new(issue))),
            Self::Mtm(set) => set
                .validate_current(canonical, inputs)
                .map_err(|issue| SizeSetError::Mtm(Box::new(issue))),
        }
    }
    /// Require original structural path inputs without numeric evaluation or provenance inference.
    ///
    /// # Errors
    /// Preserves current/coverage causes and the actual MTM grading refusal.
    pub fn validate_inputs_for(
        &self,
        path: SizeInstantiationPath,
        canonical: &CanonicalInputs,
        inputs: &MeasurementInputs,
    ) -> Result<(), SizeSetError> {
        match self {
            Self::Garment(set) => set
                .validate_inputs_for(path, canonical, inputs)
                .map_err(|issue| SizeSetError::Garment(Box::new(issue))),
            Self::Mtm(set) => set
                .validate_inputs_for(path, canonical, inputs)
                .map_err(|issue| SizeSetError::Mtm(Box::new(issue))),
        }
    }
    /// Validate actual sources and visit the original explicit family through a nonescaping view.
    ///
    /// # Errors
    /// Returns the original family cause before invoking the visitor; callback results are uninterpreted.
    pub fn with_inputs<R>(
        &self,
        canonical: &CanonicalInputs,
        inputs: &MeasurementInputs,
        visit: impl for<'context> FnOnce(SizeSetInputs<'context>) -> R,
    ) -> Result<R, SizeSetError> {
        match self {
            Self::Garment(set) => set
                .with_inputs(canonical, inputs, |view| {
                    visit(SizeSetInputs::Garment(view))
                })
                .map_err(|issue| SizeSetError::Garment(Box::new(issue))),
            Self::Mtm(set) => set
                .with_inputs(canonical, inputs, |view| visit(SizeSetInputs::Mtm(view)))
                .map_err(|issue| SizeSetError::Mtm(Box::new(issue))),
        }
    }
}
