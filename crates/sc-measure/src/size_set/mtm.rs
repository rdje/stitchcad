//! Custom singleton SizeSet inputs retain canonical body/Ease mapping and never imply a garment value.
use super::{shared, SizeInstantiationPath, SizeSetInputError as Error};
use crate::{
    EaseSetError, EaseSide, MtmChart, MtmChartContext, MtmChartError, SizeAxes, SizeBreakContext,
    SizeBreaks, SizeMembership, SizeSetReference,
};

/// Borrowed composite MTM SizeSet inputs with an explicit authored axis and original body/Ease sources.
/// Contexts must share the actual membership, measurement context and logical garment Design table.
/// Partial chart/break inventories remain drafts; regeneration separately requires complete mappings.
#[derive(Clone, Copy, Debug)]
pub struct MtmSizeSetInputs<'a> {
    chart: &'a MtmChart,
    chart_context: &'a MtmChartContext<'a>,
    breaks: &'a SizeBreaks,
    break_context: &'a SizeBreakContext<'a>,
}
impl<'a> MtmSizeSetInputs<'a> {
    /// Compose actual MTM/body/Ease records after current metadata, snapshot and identity checks.
    /// # Errors
    /// Preserves original MTM/break errors and refuses differing snapshots or cross-view aliases.
    pub fn new(
        chart: &'a MtmChart,
        chart_context: &'a MtmChartContext<'a>,
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
    /// Exact stable SizeSet identity/revision supplied by its original canonical membership.
    #[must_use]
    pub fn reference(self) -> SizeSetReference {
        self.membership().reference()
    }
    /// Actual custom singleton membership with its explicit base and authored label.
    #[must_use]
    pub const fn membership(self) -> &'a SizeMembership {
        self.break_context.membership()
    }
    /// Actual authored singleton axis/member coordinates, without an implicit axis default.
    #[must_use]
    pub const fn axes(self) -> &'a SizeAxes {
        self.break_context.axes()
    }
    /// Original body/Ease chart and its saved correspondence provenance.
    #[must_use]
    pub const fn chart(self) -> &'a MtmChart {
        self.chart
    }
    /// Original required/available break inventory; singleton grading is still unsupported.
    #[must_use]
    pub const fn breaks(self) -> &'a SizeBreaks {
        self.breaks
    }

    /// Validate actual current MTM, break, shared snapshot and cross-view identity metadata.
    /// Does not add body and Ease or read any unavailable canonical number.
    /// # Errors
    /// Returns original target/reference evidence, differing snapshots or conflicting identities.
    pub fn validate_current(self) -> Result<(), Error> {
        self.chart
            .validate_current(self.chart_context)
            .map_err(|issue| Error::Mtm(Box::new(issue)))?;
        self.breaks
            .validate_current(self.break_context)
            .map_err(|issue| Error::Breaks(Box::new(issue)))?;
        let records = self.chart_context.eases();
        let set = self
            .chart
            .ease_set(self.chart_context)
            .map_err(|issue| Error::Mtm(Box::new(issue)))?;
        let table_id = set.definition().garment_table;
        let table = records.current_table(table_id).ok_or_else(|| {
            Error::Mtm(Box::new(MtmChartError::InvalidEaseSet {
                chart: self.chart.id(),
                ease_set: set.id(),
                issue: Box::new(EaseSetError::MissingTable {
                    set: set.id(),
                    side: EaseSide::Garment,
                    table: table_id,
                }),
            }))
        })?;
        shared::validate_snapshot(
            self.chart_context.membership(),
            records.measurements(),
            table,
            self.break_context,
        )?;
        shared::validate_entities(
            self.chart.id(),
            self.chart_context.set_ids().chain(records.ease_ids()),
            records.tables(),
            self.breaks,
            self.break_context,
        )?;
        Ok(())
    }
    /// Original shared body/Ease context for explicit quantity queries in this view's lifetime.
    #[must_use]
    pub const fn chart_context(self) -> &'a MtmChartContext<'a> {
        self.chart_context
    }

    /// Original shared break context for explicit declaration/coverage queries in this view's lifetime.
    #[must_use]
    pub const fn break_context(self) -> &'a SizeBreakContext<'a> {
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

    /// Require complete body/Ease mappings for regeneration; always retain MTM's typed grading refusal.
    /// Numeric body-plus-Ease execution, physical fit, profile policy and approval remain separate.
    /// # Errors
    /// Preserves current metadata/coverage errors or the original GradeRulesUnsupported chart/member refusal.
    pub fn validate_inputs_for(self, path: SizeInstantiationPath) -> Result<(), Error> {
        self.validate_current()?;
        let result = match path {
            SizeInstantiationPath::Regeneration => self.chart.validate_complete(self.chart_context),
            SizeInstantiationPath::GradeRules => self.chart.validate_grade_rule_input(),
        };
        result.map_err(|issue| Error::Mtm(Box::new(issue)))
    }
}
