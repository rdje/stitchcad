//! Actual garment chart, membership/axes and break inputs from one shared current snapshot.
use super::{shared, SizeInstantiationPath, SizeSetInputError as Error};
use crate::{
    SizeAxes, SizeBreakContext, SizeBreaks, SizeChart, SizeChartCollectionContext,
    SizeChartCollectionError, SizeMembership, SizeSetReference,
};

/// Borrowed composite garment SizeSet inputs, retaining original records and every canonical state.
/// Draft chart/break inventories remain representable; explicit path validation requires coverage.
/// The chart and break contexts must share actual membership, measurement context and Design table
/// objects. Reconstruct this view for replacement snapshots. Global registry truth remains separate.
#[derive(Clone, Copy, Debug)]
pub struct GarmentSizeSetInputs<'a> {
    chart: &'a SizeChart,
    chart_context: &'a SizeChartCollectionContext<'a>,
    breaks: &'a SizeBreaks,
    break_context: &'a SizeBreakContext<'a>,
}
impl<'a> GarmentSizeSetInputs<'a> {
    /// Bind actual records after complete current metadata, shared snapshot and identity checks.
    /// # Errors
    /// Preserves chart/break refusals and rejects differing snapshots or cross-view identity aliases.
    pub fn new(
        chart: &'a SizeChart,
        chart_context: &'a SizeChartCollectionContext<'a>,
        breaks: &'a SizeBreaks,
        break_context: &'a SizeBreakContext<'a>,
    ) -> Result<Self, Error> {
        let result = Self {
            chart,
            chart_context,
            breaks,
            break_context,
        };
        result.validate_current()?;
        Ok(result)
    }

    /// Actual stable SizeSet identity and exact authored revision from the canonical membership.
    #[must_use]
    pub fn reference(self) -> SizeSetReference {
        self.membership().reference()
    }
    /// Original actual canonical membership, including authored order and base.
    #[must_use]
    pub const fn membership(self) -> &'a SizeMembership {
        self.break_context.membership()
    }
    /// Original complete authored axes/member points, without label/coordinate reconstruction.
    #[must_use]
    pub const fn axes(self) -> &'a SizeAxes {
        self.break_context.axes()
    }
    /// Original garment chart, without duplicated observations or numeric state.
    #[must_use]
    pub const fn chart(self) -> &'a SizeChart {
        self.chart
    }
    /// Original required/available break inventories, preserving authored order and absence.
    #[must_use]
    pub const fn breaks(self) -> &'a SizeBreaks {
        self.breaks
    }

    /// Recheck both actual inventories, exact shared snapshot references and cross-view identities.
    /// # Errors
    /// Returns original current metadata/source evidence before any path coverage result.
    pub fn validate_current(self) -> Result<(), Error> {
        self.chart
            .validate_current(self.chart_context)
            .map_err(|issue| Error::Chart(Box::new(issue)))?;
        self.breaks
            .validate_current(self.break_context)
            .map_err(|issue| Error::Breaks(Box::new(issue)))?;
        let records = self.chart_context.records();
        let design_table = records
            .table(self.chart.definition().design_table)
            .ok_or_else(|| {
                Error::Chart(Box::new(SizeChartCollectionError::MissingTable {
                    chart: self.chart.id(),
                    table: self.chart.definition().design_table,
                }))
            })?;
        shared::validate_snapshot(
            records.membership(),
            records.measurements(),
            design_table,
            self.break_context,
        )?;
        shared::validate_entities(
            self.chart.id(),
            self.chart_context.observation_ids(),
            records.tables(),
            self.breaks,
            self.break_context,
        )?;
        Ok(())
    }

    pub(crate) const fn chart_context(self) -> &'a SizeChartCollectionContext<'a> {
        self.chart_context
    }

    pub(crate) const fn break_context(self) -> &'a SizeBreakContext<'a> {
        self.break_context
    }

    pub(crate) fn contains_entity(self, identity: sc_core::ontology::EntityId) -> bool {
        identity == self.chart.id()
            || self.chart_context.contains(identity)
            || self.break_context.contains_entity(identity)
            || self
                .breaks
                .definition()
                .breaks
                .iter()
                .any(|value| value.definition().id == identity)
    }

    /// Require current structural chart coverage, and additionally complete break coverage for grading.
    /// Missing breaks do not block regeneration. Numeric availability, actual recipe/grade execution,
    /// geometry/equivalence, profile policy and release approval remain separate obligations.
    /// # Errors
    /// Preserves original incomplete chart/break cell or singleton-grading refusals as error sources.
    pub fn validate_inputs_for(self, path: SizeInstantiationPath) -> Result<(), Error> {
        self.validate_current()?;
        self.chart
            .validate_complete(self.chart_context)
            .map_err(|issue| Error::Chart(Box::new(issue)))?;
        if path == SizeInstantiationPath::GradeRules {
            self.breaks
                .require_grade_coverage(self.break_context)
                .map_err(|issue| Error::Breaks(Box::new(issue)))?;
        }
        Ok(())
    }
}
