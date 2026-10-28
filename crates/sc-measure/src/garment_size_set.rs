//! Owned external garment SizeSet sources, deriving borrowed views only within a visitor.
use crate::{
    GarmentSizeSetInputs, MeasurementInputs, SizeChart, SizeChartCollectionContext,
    SizeChartCollectionError, SizeChartContext, SizeChartError, SizeChartObservation,
    SizeInstantiationPath, SizeSetInputError, SizeSetMetadata, SizeSetMetadataError,
    SizeSetMetadataPosition, SizeSetReference,
};
use core::fmt;
use sc_core::{ontology::EntityId, value::CanonicalInputs};
use std::collections::BTreeMap;

/// Actual external garment set sources; no canonical scalar value/state is copied.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GarmentSizeSetDefinition {
    /// Exact membership, axes and break source owner.
    pub metadata: SizeSetMetadata,
    /// Complete actual observation inventory, including observations not selected by the chart.
    pub observations: Vec<SizeChartObservation>,
    /// Exact authored chart and its saved correspondence expectations; drafts may be partial.
    pub chart: SizeChart,
}

/// Original position in the complete actual source ownership census.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GarmentSizeSetPosition {
    /// Actual common SizeSet or supplied canonical/measurement/Ease position.
    Common(SizeSetMetadataPosition),
    /// Actual owned chart observation.
    Observation {
        /// Zero-based authored position.
        index: usize,
    },
    /// The owned garment chart.
    Chart,
}

/// Whole current garment source refusal, retaining original owner and concrete cause.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GarmentSizeSetError {
    /// Two actual source records claim one identity, before reference validation.
    DuplicateIdentity {
        /// Repeated identity.
        identity: EntityId,
        /// First original position.
        first: GarmentSizeSetPosition,
        /// First repeated original position.
        duplicate: GarmentSizeSetPosition,
    },
    /// Original common metadata/current-source refusal.
    Metadata(Box<SizeSetMetadataError>),
    /// Original actual garment observation target-context refusal.
    Context(Box<SizeChartError>),
    /// Original current refusal of an actual observation, even when not selected by this chart.
    Observation {
        /// Actual observation carrying the invalid reference.
        observation: EntityId,
        /// Original typed cause.
        issue: Box<SizeChartError>,
    },
    /// Original actual observation-catalog refusal.
    CollectionContext(Box<SizeChartCollectionError>),
    /// Original garment composite/current/path refusal.
    Inputs(Box<SizeSetInputError>),
}
impl fmt::Display for GarmentSizeSetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity { identity, first, duplicate } => write!(f, "garment set identity {identity} repeats at {duplicate:?}, first supplied at {first:?}"),
            Self::Metadata(issue) => write!(f, "garment set metadata: {issue}"),
            Self::Context(issue) => write!(f, "garment set observation context: {issue}"),
            Self::Observation { observation, issue } => write!(f, "garment set observation {observation}: {issue}"),
            Self::CollectionContext(issue) => write!(f, "garment set collection context: {issue}"),
            Self::Inputs(issue) => write!(f, "garment set inputs: {issue}"),
        }
    }
}
impl std::error::Error for GarmentSizeSetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DuplicateIdentity { .. } => None,
            Self::Metadata(issue) => Some(issue.as_ref()),
            Self::Context(issue) | Self::Observation { issue, .. } => Some(issue.as_ref()),
            Self::CollectionContext(issue) => Some(issue.as_ref()),
            Self::Inputs(issue) => Some(issue.as_ref()),
        }
    }
}

/// Immutable external garment SizeSet owner, separate from a Design's exact reference to it.
///
/// Every actual observation is checked, not just chart bindings. Borrowed contexts/views are local
/// to a callback; no self-reference, cached value or implicit path coverage is stored.
///
/// ```compile_fail
/// fn edit(set: &mut sc_measure::GarmentSizeSet) { set.definition.observations.clear(); }
/// ```
/// ```compile_fail
/// fn edit(set: &mut sc_measure::GarmentSizeSet) { set.definition().observations.clear(); }
/// ```
/// ```compile_fail
/// fn escape(set: &sc_measure::GarmentSizeSet, canonical: &sc_core::value::CanonicalInputs,
///     inputs: &sc_measure::MeasurementInputs) {
///     let _view = set.with_inputs(canonical, inputs, |view| view);
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct GarmentSizeSet {
    definition: GarmentSizeSetDefinition,
}
impl fmt::Debug for GarmentSizeSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GarmentSizeSet")
            .field("observation_count", &self.definition.observations.len())
            .finish_non_exhaustive()
    }
}
impl GarmentSizeSet {
    pub(crate) fn resource_definition(&self) -> &GarmentSizeSetDefinition {
        let Self { definition } = self;
        definition
    }
    /// Own exact sources after complete identity/current binding validation, allowing partial drafts.
    ///
    /// # Errors
    /// Returns the same actual source refusal as validate_current, without selecting replacements.
    pub fn new(
        definition: GarmentSizeSetDefinition,
        canonical: &CanonicalInputs,
        inputs: &MeasurementInputs,
    ) -> Result<Self, GarmentSizeSetError> {
        let set = Self { definition };
        set.validate_current(canonical, inputs)?;
        Ok(set)
    }
    /// Original source definition; shared access supplies no mutable owner.
    #[must_use]
    pub const fn definition(&self) -> &GarmentSizeSetDefinition {
        &self.definition
    }
    /// Actual external object identity and authored revision.
    #[must_use]
    pub const fn reference(&self) -> SizeSetReference {
        self.definition.metadata.reference()
    }

    /// Exact owned common metadata, observations and chart identities in original census order.
    #[must_use]
    pub fn identities(&self) -> Vec<(EntityId, GarmentSizeSetPosition)> {
        self.definition
            .metadata
            .identities()
            .into_iter()
            .map(|(id, position)| (id, GarmentSizeSetPosition::Common(position)))
            .chain(
                self.definition
                    .observations
                    .iter()
                    .enumerate()
                    .map(|(index, record)| {
                        (record.id(), GarmentSizeSetPosition::Observation { index })
                    }),
            )
            .chain(core::iter::once((
                self.definition.chart.id(),
                GarmentSizeSetPosition::Chart,
            )))
            .collect()
    }

    /// Recheck actual current source identities, every observation and the chart without path coverage.
    ///
    /// # Errors
    /// Returns original current causes as described by with_inputs.
    pub fn validate_current(
        &self,
        canonical: &CanonicalInputs,
        inputs: &MeasurementInputs,
    ) -> Result<(), GarmentSizeSetError> {
        self.with_inputs(canonical, inputs, |_| ())
    }

    /// Require original structural coverage for the explicit path, without numeric execution.
    ///
    /// # Errors
    /// Preserves current failures and actual missing chart/break cells through the Inputs cause.
    pub fn validate_inputs_for(
        &self,
        path: SizeInstantiationPath,
        canonical: &CanonicalInputs,
        inputs: &MeasurementInputs,
    ) -> Result<(), GarmentSizeSetError> {
        self.with_inputs(canonical, inputs, |view| view.validate_inputs_for(path))?
            .map_err(|issue| GarmentSizeSetError::Inputs(Box::new(issue)))
    }

    /// Validate whole owned/current sources, then visit their actual borrowed garment composite.
    ///
    /// Census order is supplied canonical/measurement/Ease records, common metadata, observations,
    /// chart. The callback cannot return local context/view borrows and never runs on failure.
    /// It may return owned data; this method does not interpret callback results.
    ///
    /// # Errors
    /// Returns first duplicate, then actual common metadata/context/observation/composite cause.
    pub fn with_inputs<R>(
        &self,
        canonical: &CanonicalInputs,
        inputs: &MeasurementInputs,
        visit: impl for<'context> FnOnce(GarmentSizeSetInputs<'context>) -> R,
    ) -> Result<R, GarmentSizeSetError> {
        let mut seen = BTreeMap::new();
        for (identity, position) in inputs
            .identity_positions(canonical)
            .map(|(id, position)| {
                (
                    id,
                    GarmentSizeSetPosition::Common(SizeSetMetadataPosition::Inputs(position)),
                )
            })
            .chain(self.identities())
        {
            if let Some(first) = seen.insert(identity, position) {
                return Err(GarmentSizeSetError::DuplicateIdentity {
                    identity,
                    first,
                    duplicate: position,
                });
            }
        }
        self.definition
            .metadata
            .with_context(canonical, inputs, |break_context| {
                let records = SizeChartContext::new(
                    break_context.membership(),
                    &inputs.measurements().definition().tables,
                    break_context.measurements(),
                )
                .map_err(|issue| GarmentSizeSetError::Context(Box::new(issue)))?;
                for observation in &self.definition.observations {
                    observation.validate_current(&records).map_err(|issue| {
                        GarmentSizeSetError::Observation {
                            observation: observation.id(),
                            issue: Box::new(issue),
                        }
                    })?;
                }
                let chart_context =
                    SizeChartCollectionContext::new(&self.definition.observations, &records)
                        .map_err(|issue| GarmentSizeSetError::CollectionContext(Box::new(issue)))?;
                let view = GarmentSizeSetInputs::new(
                    &self.definition.chart,
                    &chart_context,
                    &self.definition.metadata.definition().breaks,
                    break_context,
                )
                .map_err(|issue| GarmentSizeSetError::Inputs(Box::new(issue)))?;
                Ok(visit(view))
            })
            .map_err(|issue| GarmentSizeSetError::Metadata(Box::new(issue)))?
    }
}
