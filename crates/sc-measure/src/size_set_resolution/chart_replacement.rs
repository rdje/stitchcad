//! Typed candidate charts resolve in the original family's actual canonical metadata context.
use super::transformation::SizeSetTransformationError as Error;
use crate::{GarmentSizeSetInputs, MtmChart, MtmSizeSetInputs, SizeChart, SizeSetInputs};
use sc_core::ontology::EntityId;
use std::collections::BTreeSet;

/// Explicit chart-family/identity binding; no provenance is inferred from labels or member count.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SizeSetChartReplacement {
    /// Current garment chart in the source garment observation context.
    Garment(EntityId),
    /// Current body/Ease chart in the source MTM context.
    Mtm(EntityId),
}
impl SizeSetChartReplacement {
    /// Exact authored chart identity.
    #[must_use]
    pub const fn id(self) -> EntityId {
        match self {
            Self::Garment(id) | Self::Mtm(id) => id,
        }
    }
}
/// Actual candidate chart inventory, borrowed without copying observations or canonical values.
#[derive(Clone, Copy, Debug)]
pub enum SizeSetReplacementCharts<'a> {
    /// Explicit garment candidates; source must be the garment input family.
    Garment(&'a [&'a SizeChart]),
    /// Explicit body/Ease candidates; source must be the MTM input family.
    Mtm(&'a [&'a MtmChart]),
}
/// Immutable current replacement inventory. Each candidate uses the source's actual chart context,
/// original membership/axes/breaks and canonical metadata. Reconstruct for replacement snapshots.
#[derive(Debug)]
pub struct SizeSetChartReplacementContext<'a> {
    inputs: SizeSetInputs<'a>,
    charts: SizeSetReplacementCharts<'a>,
}
impl<'a> SizeSetChartReplacementContext<'a> {
    /// Validate all supplied candidate charts through actual source-family composite contracts.
    /// Partial candidates remain drafts; this constructor does not certify path completeness.
    /// # Errors
    /// Returns current source/candidate failures, a family mismatch or a duplicate/substituted identity.
    pub fn new(
        inputs: SizeSetInputs<'a>,
        charts: SizeSetReplacementCharts<'a>,
    ) -> Result<Self, Error> {
        inputs
            .validate_current()
            .map_err(|issue| Error::Inputs(Box::new(issue)))?;
        let result = Self { inputs, charts };
        let mut seen = BTreeSet::new();
        match charts {
            SizeSetReplacementCharts::Garment(values) => {
                if !matches!(inputs, SizeSetInputs::Garment(_)) {
                    return Err(Error::ChartFamilyMismatch);
                }
                for chart in values {
                    if !seen.insert(chart.id()) {
                        return Err(Error::DuplicateChart(chart.id()));
                    }
                    result.resolve(SizeSetChartReplacement::Garment(chart.id()))?;
                }
            }
            SizeSetReplacementCharts::Mtm(values) => {
                if !matches!(inputs, SizeSetInputs::Mtm(_)) {
                    return Err(Error::ChartFamilyMismatch);
                }
                for chart in values {
                    if !seen.insert(chart.id()) {
                        return Err(Error::DuplicateChart(chart.id()));
                    }
                    result.resolve(SizeSetChartReplacement::Mtm(chart.id()))?;
                }
            }
        }
        Ok(result)
    }
    /// Original source input view, without retargeting its chart or changing its path readiness.
    #[must_use]
    pub const fn inputs(&self) -> SizeSetInputs<'a> {
        self.inputs
    }
    pub(crate) fn contains_entity(&self, identity: EntityId) -> bool {
        self.inputs.contains_entity(identity)
            || match self.charts {
                SizeSetReplacementCharts::Garment(charts) => {
                    charts.iter().any(|chart| chart.id() == identity)
                }
                SizeSetReplacementCharts::Mtm(charts) => {
                    charts.iter().any(|chart| chart.id() == identity)
                }
            }
    }
    /// Revalidate and borrow a candidate composite using original membership/axes/breaks/metadata.
    /// Same-chart selection is explicit valid intent only for the original actual chart object.
    /// # Errors
    /// Returns missing/family/identity errors or the original candidate composite failure.
    pub fn resolve(&self, binding: SizeSetChartReplacement) -> Result<SizeSetInputs<'a>, Error> {
        self.inputs
            .validate_current()
            .map_err(|issue| Error::Inputs(Box::new(issue)))?;
        let result = match (self.inputs, self.charts, binding) {
            (
                SizeSetInputs::Garment(source),
                SizeSetReplacementCharts::Garment(charts),
                SizeSetChartReplacement::Garment(id),
            ) => {
                let chart = charts
                    .iter()
                    .copied()
                    .find(|chart| chart.id() == id)
                    .ok_or(Error::MissingChart(id))?;
                if id == source.chart().id() && !std::ptr::eq(chart, source.chart()) {
                    return Err(Error::DuplicateChart(id));
                }
                GarmentSizeSetInputs::new(
                    chart,
                    source.chart_context(),
                    source.breaks(),
                    source.break_context(),
                )
                .map(SizeSetInputs::from)
            }
            (
                SizeSetInputs::Mtm(source),
                SizeSetReplacementCharts::Mtm(charts),
                SizeSetChartReplacement::Mtm(id),
            ) => {
                let chart = charts
                    .iter()
                    .copied()
                    .find(|chart| chart.id() == id)
                    .ok_or(Error::MissingChart(id))?;
                if id == source.chart().id() && !std::ptr::eq(chart, source.chart()) {
                    return Err(Error::DuplicateChart(id));
                }
                MtmSizeSetInputs::new(
                    chart,
                    source.chart_context(),
                    source.breaks(),
                    source.break_context(),
                )
                .map(SizeSetInputs::from)
            }
            _ => return Err(Error::ChartFamilyMismatch),
        };
        result.map_err(|issue| Error::ReplacementChart {
            chart: binding.id(),
            issue: Box::new(issue),
        })
    }
}
