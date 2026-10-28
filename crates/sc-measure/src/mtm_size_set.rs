//! Owned external MTM sources; body/Ease values remain in their actual canonical inventory.
use crate::{
    EaseSetContext, EaseSetError, MeasurementInputs, MtmChart, MtmChartContext, MtmChartError,
    MtmSizeSetInputs, SizeInstantiationPath, SizeSetInputError, SizeSetMetadata,
    SizeSetMetadataError, SizeSetMetadataPosition, SizeSetReference,
};
use core::fmt;
use sc_core::{ontology::EntityId, value::CanonicalInputs};
use std::collections::BTreeMap;

/// Exact owned singleton common/chart sources; Ease sets remain in supplied measurement inputs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MtmSizeSetDefinition {
    /// Actual singleton membership, explicit axes/point and break metadata.
    pub metadata: SizeSetMetadata,
    /// Actual immutable custom body/Ease correspondence chart.
    pub chart: MtmChart,
}

/// Actual original ownership position across supplied inputs and the owned MTM set sources.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MtmSizeSetPosition {
    /// Actual common or supplied canonical/measurement/Ease position.
    Common(SizeSetMetadataPosition),
    /// The owned MTM chart.
    Chart,
}

/// Original current MTM source refusal, without synthesizing body-plus-Ease values.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MtmSizeSetError {
    /// Two owned source records claim one identity before current-reference validation.
    DuplicateIdentity {
        /// Repeated identity.
        identity: EntityId,
        /// First actual ownership position.
        first: MtmSizeSetPosition,
        /// First repeated position.
        duplicate: MtmSizeSetPosition,
    },
    /// Original actual common metadata/source refusal.
    Metadata(Box<SizeSetMetadataError>),
    /// Original actual Ease/table context refusal.
    Eases(Box<EaseSetError>),
    /// Original actual MTM membership/Ease-set context refusal.
    Context(Box<MtmChartError>),
    /// Original current composite/path refusal, including explicit singleton grading rejection.
    Inputs(Box<SizeSetInputError>),
}
impl fmt::Display for MtmSizeSetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity {
                identity,
                first,
                duplicate,
            } => write!(
                f,
                "MTM set identity {identity} repeats at {duplicate:?}, first supplied at {first:?}"
            ),
            Self::Metadata(issue) => write!(f, "MTM set metadata: {issue}"),
            Self::Eases(issue) => write!(f, "MTM set Ease context: {issue}"),
            Self::Context(issue) => write!(f, "MTM set chart context: {issue}"),
            Self::Inputs(issue) => write!(f, "MTM set inputs: {issue}"),
        }
    }
}
impl std::error::Error for MtmSizeSetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DuplicateIdentity { .. } => None,
            Self::Metadata(issue) => Some(issue.as_ref()),
            Self::Eases(issue) => Some(issue.as_ref()),
            Self::Context(issue) => Some(issue.as_ref()),
            Self::Inputs(issue) => Some(issue.as_ref()),
        }
    }
}

/// Private external singleton MTM set owner; Design holds its exact reference separately.
///
/// Actual current body/Ease/table data is borrowed through local contexts. No context/value cache,
/// body-plus-Ease arithmetic, implicit base or profile is stored. Partial mappings remain drafts.
///
/// ```compile_fail
/// fn edit(set: &mut sc_measure::MtmSizeSet) { set.definition.metadata = set.definition.metadata.clone(); }
/// ```
/// ```compile_fail
/// fn edit(set: &mut sc_measure::MtmSizeSet) { set.definition().chart = set.definition().chart.clone(); }
/// ```
/// ```compile_fail
/// fn escape(set: &sc_measure::MtmSizeSet, canonical: &sc_core::value::CanonicalInputs,
///     inputs: &sc_measure::MeasurementInputs) {
///     let _view = set.with_inputs(canonical, inputs, |view| view);
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct MtmSizeSet {
    definition: MtmSizeSetDefinition,
}
impl fmt::Debug for MtmSizeSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MtmSizeSet").finish_non_exhaustive()
    }
}
impl MtmSizeSet {
    pub(crate) fn resource_definition(&self) -> &MtmSizeSetDefinition {
        let Self { definition } = self;
        definition
    }
    /// Own exact sources after complete identity/current MTM validation, allowing partial drafts.
    ///
    /// # Errors
    /// Returns actual source refusal as described by validate_current; no replacement is selected.
    pub fn new(
        definition: MtmSizeSetDefinition,
        canonical: &CanonicalInputs,
        inputs: &MeasurementInputs,
    ) -> Result<Self, MtmSizeSetError> {
        let set = Self { definition };
        set.validate_current(canonical, inputs)?;
        Ok(set)
    }
    /// Original exact sources; shared access supplies no mutable owner.
    #[must_use]
    pub const fn definition(&self) -> &MtmSizeSetDefinition {
        &self.definition
    }
    /// Actual set identity and authored revision from canonical singleton membership.
    #[must_use]
    pub const fn reference(&self) -> SizeSetReference {
        self.definition.metadata.reference()
    }

    /// Actual owned common metadata then chart identity, without re-owning referenced Ease sources.
    #[must_use]
    pub fn identities(&self) -> Vec<(EntityId, MtmSizeSetPosition)> {
        self.definition
            .metadata
            .identities()
            .into_iter()
            .map(|(id, position)| (id, MtmSizeSetPosition::Common(position)))
            .chain(core::iter::once((
                self.definition.chart.id(),
                MtmSizeSetPosition::Chart,
            )))
            .collect()
    }

    /// Revalidate actual current source identities, singleton/chart bindings and shared contexts.
    ///
    /// # Errors
    /// Returns original current source causes without adding coverage or numerical computation.
    pub fn validate_current(
        &self,
        canonical: &CanonicalInputs,
        inputs: &MeasurementInputs,
    ) -> Result<(), MtmSizeSetError> {
        self.with_inputs(canonical, inputs, |_| ())
    }

    /// Require original structural path inputs; grade rules retain the actual MTM chart refusal.
    ///
    /// # Errors
    /// Returns original current/coverage errors or GradeRulesUnsupported through the Inputs cause.
    pub fn validate_inputs_for(
        &self,
        path: SizeInstantiationPath,
        canonical: &CanonicalInputs,
        inputs: &MeasurementInputs,
    ) -> Result<(), MtmSizeSetError> {
        self.with_inputs(canonical, inputs, |view| view.validate_inputs_for(path))?
            .map_err(|issue| MtmSizeSetError::Inputs(Box::new(issue)))
    }

    /// Check all actual owned/current sources, then visit a nonescaping borrowed MTM composite.
    ///
    /// Census order is supplied canonical/measurement/Ease records, common metadata, then chart.
    /// Contexts use the same actual membership, measurement context and logical Design table.
    /// The callback may return owned data; local contexts/views cannot escape. Callback results
    /// are not interpreted, and failure never invokes it.
    ///
    /// # Errors
    /// Returns first duplicate, then original common/Ease/MTM context or composite refusal.
    pub fn with_inputs<R>(
        &self,
        canonical: &CanonicalInputs,
        inputs: &MeasurementInputs,
        visit: impl for<'context> FnOnce(MtmSizeSetInputs<'context>) -> R,
    ) -> Result<R, MtmSizeSetError> {
        let mut seen = BTreeMap::new();
        for (identity, position) in inputs
            .identity_positions(canonical)
            .map(|(id, position)| {
                (
                    id,
                    MtmSizeSetPosition::Common(SizeSetMetadataPosition::Inputs(position)),
                )
            })
            .chain(self.identities())
        {
            if let Some(first) = seen.insert(identity, position) {
                return Err(MtmSizeSetError::DuplicateIdentity {
                    identity,
                    first,
                    duplicate: position,
                });
            }
        }
        self.definition
            .metadata
            .with_context(canonical, inputs, |break_context| {
                let eases = EaseSetContext::new(
                    &inputs.measurements().definition().tables,
                    &inputs.definition().eases,
                    break_context.measurements(),
                )
                .map_err(|issue| MtmSizeSetError::Eases(Box::new(issue)))?;
                let chart_context = MtmChartContext::new(
                    break_context.membership(),
                    &inputs.definition().ease_sets,
                    &eases,
                )
                .map_err(|issue| MtmSizeSetError::Context(Box::new(issue)))?;
                let view = MtmSizeSetInputs::new(
                    &self.definition.chart,
                    &chart_context,
                    &self.definition.metadata.definition().breaks,
                    break_context,
                )
                .map_err(|issue| MtmSizeSetError::Inputs(Box::new(issue)))?;
                Ok(visit(view))
            })
            .map_err(|issue| MtmSizeSetError::Metadata(Box::new(issue)))?
    }
}
