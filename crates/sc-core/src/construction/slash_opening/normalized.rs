//! Complete one-document admission before any normalization reparse.
use super::SlashSpreadOpening;
use crate::{
    construction::arguments::{measure, PopulationRefusal},
    recipe::{FormulaExpressionDocumentError, FormulaNormalizedExpression},
};
use core::fmt;

/// Explicit complete node and conservative `1 + 2N` formula normalization allowances.
/// Original input/parser/checker/runtime/allocator/CPU and whole geometry usage are separate.
/// ```compile_fail
/// let limits:sc_core::construction::SlashSpreadNormalizationLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlashSpreadNormalizationLimits {
    nodes: usize,
    retained_records: usize,
}
impl SlashSpreadNormalizationLimits {
    /// Select explicit complete node and retained-record allowances; no hidden defaults.
    #[must_use]
    pub const fn new(nodes: usize, retained_records: usize) -> Self {
        Self {
            nodes,
            retained_records,
        }
    }
    /// Maximum actual complete formula nodes,including all branches.
    #[must_use]
    pub const fn nodes(self) -> usize {
        self.nodes
    }
    /// Maximum conservative normalized formula records,not whole-operation resource usage.
    #[must_use]
    pub const fn retained_records(self) -> usize {
        self.retained_records
    }
}
/// Complete population/quota or original document refusal; no normalized prefix escapes.
#[derive(Debug)]
pub enum SlashSpreadNormalizationRefusal {
    /// Complete population arithmetic is not representable.
    PopulationOverflow,
    /// Actual node population exceeds the explicit quota,checked first.
    Nodes {
        /// Selected allowance.
        allowed: usize,
        /// Complete actual population.
        required: usize,
    },
    /// Conservative complete records exceed the explicit quota,before reparse.
    RetainedRecords {
        /// Selected allowance.
        allowed: usize,
        /// Complete conservative population.
        required: usize,
    },
    /// Original document normalization error,without type erasure.
    Document(Box<FormulaExpressionDocumentError>),
}
/// Original opening,policy and typed refusal; mode comes from that actual owner.
pub struct SlashSpreadNormalizationError<'a> {
    opening: &'a SlashSpreadOpening,
    limits: SlashSpreadNormalizationLimits,
    refusal: SlashSpreadNormalizationRefusal,
}
impl<'a> SlashSpreadNormalizationError<'a> {
    /// Actual original explicit opening and document.
    #[must_use]
    pub const fn opening(&self) -> &'a SlashSpreadOpening {
        self.opening
    }
    /// Original explicit complete normalization policy.
    #[must_use]
    pub const fn limits(&self) -> SlashSpreadNormalizationLimits {
        self.limits
    }
    /// Complete original typed cause.
    #[must_use]
    pub const fn refusal(&self) -> &SlashSpreadNormalizationRefusal {
        &self.refusal
    }
}
impl fmt::Debug for SlashSpreadNormalizationError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SlashSpreadNormalizationError")
            .field("mode", &self.opening.mode())
            .field("limits", &self.limits)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for SlashSpreadNormalizationError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("slash/spread opening normalization refused")
    }
}
impl std::error::Error for SlashSpreadNormalizationError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.refusal {
            SlashSpreadNormalizationRefusal::Document(error) => Some(error.as_ref()),
            _ => None,
        }
    }
}
/// Actual normalized amount arena and original explicit mode,without a value or geometry.
/// ```compile_fail
/// fn escape(opening:&sc_core::construction::SlashSpreadOpening)
///     ->sc_core::construction::NormalizedSlashSpreadOpening<'static> {
///     opening.normalized(sc_core::construction::SlashSpreadNormalizationLimits::new(1,3)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct NormalizedSlashSpreadOpening<'a> {
    opening: &'a SlashSpreadOpening,
    limits: SlashSpreadNormalizationLimits,
    nodes: usize,
    retained_records: usize,
    expression: FormulaNormalizedExpression<'a>,
}
impl<'a> NormalizedSlashSpreadOpening<'a> {
    pub(crate) fn resource_expressions(&self) -> [&FormulaNormalizedExpression<'a>; 1] {
        use crate::design::fixed_value;
        let Self {
            opening,
            limits,
            nodes,
            retained_records,
            expression,
        } = self;
        fixed_value(opening);
        fixed_value(limits);
        fixed_value(nodes);
        fixed_value(retained_records);
        [expression]
    }

    /// Actual explicit authored owner and original document.
    #[must_use]
    pub const fn opening(&self) -> &'a SlashSpreadOpening {
        self.opening
    }
    /// Explicit admitted normalization policy.
    #[must_use]
    pub const fn limits(&self) -> SlashSpreadNormalizationLimits {
        self.limits
    }
    /// Complete actual amount-expression node population.
    #[must_use]
    pub const fn node_count(&self) -> usize {
        self.nodes
    }
    /// Complete conservative admitted normalized records.
    #[must_use]
    pub const fn retained_records(&self) -> usize {
        self.retained_records
    }
    /// Actual normalized arena borrowing the original document text.
    #[must_use]
    pub const fn expression(&self) -> &FormulaNormalizedExpression<'a> {
        &self.expression
    }
}
impl SlashSpreadOpening {
    /// Admit `1 + 2N` records:one amount owner,N nodes and at most N call-argument indices.
    /// Complete actual counts and explicit quotas precede any reparse. This excludes earlier
    /// owned input,parser/checker scratch and whole-operation/geometry/runtime resources.
    /// # Errors
    /// Original complete population/quota or actual document cause,without an accepted prefix.
    pub fn normalized(
        &self,
        limits: SlashSpreadNormalizationLimits,
    ) -> Result<NormalizedSlashSpreadOpening<'_>, SlashSpreadNormalizationError<'_>> {
        use SlashSpreadNormalizationRefusal as R;
        let refuse = |refusal| SlashSpreadNormalizationError {
            opening: self,
            limits,
            refusal,
        };
        let (nodes, records) = measure([self.document()], limits.nodes, limits.retained_records)
            .map_err(|cause| {
                refuse(match cause {
                    PopulationRefusal::Overflow => R::PopulationOverflow,
                    PopulationRefusal::Nodes { allowed, required } => {
                        R::Nodes { allowed, required }
                    }
                    PopulationRefusal::RetainedRecords { allowed, required } => {
                        R::RetainedRecords { allowed, required }
                    }
                })
            })?;
        let expression = self
            .document()
            .normalized()
            .map_err(|error| refuse(R::Document(Box::new(error))))?;
        Ok(NormalizedSlashSpreadOpening {
            opening: self,
            limits,
            nodes,
            retained_records: records,
            expression,
        })
    }
}
