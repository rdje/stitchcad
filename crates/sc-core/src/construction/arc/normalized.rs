//! Complete actual three-document normalization admission before any reparse.
use super::{CenterArcArgument, CenterArcConstruction};
use crate::{
    construction::arguments::{measure, PopulationRefusal},
    recipe::{FormulaExpressionDocumentError, FormulaNormalizedExpression},
};
use core::fmt;

/// Explicit complete formula-node and `3 + 2N` normalized retained-record allowances.
/// Earlier owned documents/center metadata and parser/checker/runtime/allocator/CPU are separate.
/// ```compile_fail
/// let limits:sc_core::construction::CenterArcNormalizationLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CenterArcNormalizationLimits {
    nodes: usize,
    retained_records: usize,
}
impl CenterArcNormalizationLimits {
    /// Select complete N nodes and conservative `3 + 2N` formula normalization records.
    #[must_use]
    pub const fn new(nodes: usize, retained_records: usize) -> Self {
        Self {
            nodes,
            retained_records,
        }
    }
    /// Maximum complete nodes across radius/start/sweep, including every branch.
    #[must_use]
    pub const fn nodes(self) -> usize {
        self.nodes
    }
    /// Maximum conservative normalized formula records, not whole arc/model resource usage.
    #[must_use]
    pub const fn retained_records(self) -> usize {
        self.retained_records
    }
}
/// Complete normalization population/quota or original scalar-document refusal.
#[derive(Debug)]
pub enum CenterArcNormalizationRefusal {
    /// Complete count or conservative retained-record arithmetic cannot be represented.
    PopulationOverflow,
    /// Complete actual formula-node quota, checked first.
    Nodes {
        /// Explicit selected allowance.
        allowed: usize,
        /// Complete actual formula-node population.
        required: usize,
    },
    /// Complete conservative normalized-record quota, before any reparse.
    RetainedRecords {
        /// Explicit selected allowance.
        allowed: usize,
        /// Complete prospective formula normalization records.
        required: usize,
    },
    /// Original scalar document failure with actual field role.
    Document {
        /// Actual radius/start/sweep field.
        argument: CenterArcArgument,
        /// Original semantic document error, compacted without type erasure.
        error: Box<FormulaExpressionDocumentError>,
    },
}
/// Original arc declaration, selected normalization policy and complete refusal; no accepted prefix.
pub struct CenterArcNormalizationError<'a> {
    arc: &'a CenterArcConstruction,
    limits: CenterArcNormalizationLimits,
    refusal: CenterArcNormalizationRefusal,
}
impl<'a> CenterArcNormalizationError<'a> {
    /// Actual original center-based arc and all its documents.
    #[must_use]
    pub const fn arc(&self) -> &'a CenterArcConstruction {
        self.arc
    }
    /// Original explicit complete normalization policy.
    #[must_use]
    pub const fn limits(&self) -> CenterArcNormalizationLimits {
        self.limits
    }
    /// Original complete typed cause, with a real argument role when available.
    #[must_use]
    pub const fn refusal(&self) -> &CenterArcNormalizationRefusal {
        &self.refusal
    }
}
impl fmt::Debug for CenterArcNormalizationError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CenterArcNormalizationError")
            .field("operation", &self.arc.operation())
            .field("limits", &self.limits)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for CenterArcNormalizationError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("center arc normalization refused")
    }
}
impl std::error::Error for CenterArcNormalizationError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.refusal {
            CenterArcNormalizationRefusal::Document { error, .. } => Some(error.as_ref()),
            _ => None,
        }
    }
}
/// Complete actual radius/start/sweep normalized arenas borrowing original document text.
/// ```compile_fail
/// fn escape(arc:&sc_core::construction::CenterArcConstruction)
///     ->sc_core::construction::NormalizedCenterArcConstruction<'static> {
///     arc.normalized(sc_core::construction::CenterArcNormalizationLimits::new(8,19)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct NormalizedCenterArcConstruction<'a> {
    arc: &'a CenterArcConstruction,
    limits: CenterArcNormalizationLimits,
    nodes: usize,
    retained_records: usize,
    radius: FormulaNormalizedExpression<'a>,
    start_angle: FormulaNormalizedExpression<'a>,
    sweep_angle: FormulaNormalizedExpression<'a>,
}
impl<'a> NormalizedCenterArcConstruction<'a> {
    /// Actual authored center-based variant,center/output roles and original documents.
    #[must_use]
    pub const fn arc(&self) -> &'a CenterArcConstruction {
        self.arc
    }
    /// Original explicitly admitted normalization policy.
    #[must_use]
    pub const fn limits(&self) -> CenterArcNormalizationLimits {
        self.limits
    }
    /// Complete actual nodes across all three scalar arguments.
    #[must_use]
    pub const fn node_count(&self) -> usize {
        self.nodes
    }
    /// Complete conservative admitted normalized formula records.
    #[must_use]
    pub const fn retained_records(&self) -> usize {
        self.retained_records
    }
    /// Original selected scalar argument's actual normalized arena.
    #[must_use]
    pub const fn argument(&self, argument: CenterArcArgument) -> &FormulaNormalizedExpression<'a> {
        match argument {
            CenterArcArgument::Radius => &self.radius,
            CenterArcArgument::StartAngle => &self.start_angle,
            CenterArcArgument::SweepAngle => &self.sweep_angle,
        }
    }
}
impl CenterArcConstruction {
    /// Admit all formula populations before normalizing radius,then start,then sweep.
    /// Shape is `3 + 2N`:three argument owners,N normalized nodes and at most N call-argument
    /// indices. This excludes earlier owned inputs,parser/checker scratch and geometry/runtime.
    /// # Errors
    /// Complete arithmetic/nodes/retained quotas precede any document factory. Original document
    /// failures retain actual argument roles and causes, with no accepted normalized prefix.
    pub fn normalized(
        &self,
        limits: CenterArcNormalizationLimits,
    ) -> Result<NormalizedCenterArcConstruction<'_>, CenterArcNormalizationError<'_>> {
        use CenterArcNormalizationRefusal as R;
        let refuse = |refusal| CenterArcNormalizationError {
            arc: self,
            limits,
            refusal,
        };
        let (nodes, records) = measure(
            [&self.radius, &self.start_angle, &self.sweep_angle],
            limits.nodes,
            limits.retained_records,
        )
        .map_err(|cause| {
            refuse(match cause {
                PopulationRefusal::Overflow => R::PopulationOverflow,
                PopulationRefusal::Nodes { allowed, required } => R::Nodes { allowed, required },
                PopulationRefusal::RetainedRecords { allowed, required } => {
                    R::RetainedRecords { allowed, required }
                }
            })
        })?;
        let normalize = |argument| {
            self.argument(argument).normalized().map_err(|error| {
                refuse(R::Document {
                    argument,
                    error: Box::new(error),
                })
            })
        };
        let radius = normalize(CenterArcArgument::Radius)?;
        let start_angle = normalize(CenterArcArgument::StartAngle)?;
        let sweep_angle = normalize(CenterArcArgument::SweepAngle)?;
        Ok(NormalizedCenterArcConstruction {
            arc: self,
            limits,
            nodes,
            retained_records: records,
            radius,
            start_angle,
            sweep_angle,
        })
    }
}
