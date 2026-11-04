//! Fresh complete local content/work and duplicate-key admission;no actual source/coverage proof.
use super::{DraftingSemanticSelectionInputs, DraftingSemanticSelectionKey};
use crate::design::{
    fixed_value, DesignResourceDemand, DesignResourceLimitError, DesignResourceLimits,
    DesignResourceMeter, DesignResourceScope, DesignResourceSink,
};
use crate::ontology::EntityId;
use core::fmt;
mod error;
pub use error::{DraftingSemanticSelectionAdmissionError, DraftingSemanticSelectionRefusal};
/// Explicit fresh local content and conservative fixed-key comparison policy,without defaults.
/// ```compile_fail
/// let limits:sc_core::construction::DraftingSemanticSelectionLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingSemanticSelectionLimits {
    content: DesignResourceLimits,
    comparisons: usize,
}
impl DraftingSemanticSelectionLimits {
    /// Select complete fresh local record content and conservative N² key comparisons explicitly.
    #[must_use]
    pub const fn new(content: DesignResourceLimits, key_comparisons: usize) -> Self {
        Self {
            content,
            comparisons: key_comparisons,
        }
    }
    /// Complete actual local row/pin content allowance,without Root/provider/body credit.
    #[must_use]
    pub const fn content(self) -> DesignResourceLimits {
        self.content
    }
    /// Conservative fixed-key comparison maximum;pin strings are not compared.
    #[must_use]
    pub const fn key_comparisons(self) -> usize {
        self.comparisons
    }
}
/// Actual original local row population and checked conservative duplicate comparison envelope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingSemanticSelectionShape {
    rows: usize,
    comparisons: usize,
}
impl DraftingSemanticSelectionShape {
    /// Complete actual original row count,including invalid/extra requests.
    #[must_use]
    pub const fn row_count(self) -> usize {
        self.rows
    }
    /// Checked conservative N² fixed-key comparisons,without CPU/allocator/string-work credit.
    #[must_use]
    pub const fn key_comparison_bound(self) -> usize {
        self.comparisons
    }
}
/// Complete local content/work and unique structural keys over the actual original request owner.
/// This admits no actual coordinate/source need,member,Profile body,order,value or whole scope.
/// ```compile_fail,E0616
/// fn alter(proof:&mut sc_core::construction::AdmittedDraftingSemanticSelections<'_,'_>) {proof.inputs=todo!();}
/// ```
/// Local admission cannot outlive the actual raw owner:
/// ```compile_fail
/// use sc_core::construction::*;
/// fn escape<'r,'a>(inputs:&'r DraftingSemanticSelectionInputs<'a>,limits:DraftingSemanticSelectionLimits)
///     ->Result<AdmittedDraftingSemanticSelections<'static,'a>,String> {inputs.admit(limits).map_err(|e|e.to_string())}
/// ```
pub struct AdmittedDraftingSemanticSelections<'r, 'a> {
    inputs: &'r DraftingSemanticSelectionInputs<'a>,
    limits: DraftingSemanticSelectionLimits,
    demand: DesignResourceDemand,
    shape: DraftingSemanticSelectionShape,
}
impl<'r, 'a> AdmittedDraftingSemanticSelections<'r, 'a> {
    /// Exact actual raw owner and every original row/pin/origin,without a copied table.
    #[must_use]
    pub const fn inputs(&self) -> &'r DraftingSemanticSelectionInputs<'a> {
        self.inputs
    }
    /// Exact selected explicit fresh local policy.
    #[must_use]
    pub const fn limits(&self) -> DraftingSemanticSelectionLimits {
        self.limits
    }
    /// Complete fresh local content demand;not later whole-source budget credit.
    #[must_use]
    pub const fn demand(&self) -> DesignResourceDemand {
        self.demand
    }
    /// Actual complete row count and checked conservative local comparison envelope.
    #[must_use]
    pub const fn shape(&self) -> DraftingSemanticSelectionShape {
        self.shape
    }
    /// Freshly append every original record/pin field. Fixed proof metadata and stored demand
    /// give no credit;no duplicate scan,lookup,source query or actual-scope coverage occurs.
    /// # Errors
    /// Original first cumulative field cause or earlier sink latch.
    pub fn charge_record_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
        scope: DesignResourceScope,
        owner: Option<EntityId>,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            inputs,
            limits,
            demand,
            shape,
        } = self;
        fixed_value(limits);
        fixed_value(demand);
        fixed_value(shape);
        inputs.charge_record_resources(sink, scope, owner)
    }
}
impl fmt::Debug for AdmittedDraftingSemanticSelections<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AdmittedDraftingSemanticSelections")
            .field("shape", &self.shape)
            .finish_non_exhaustive()
    }
}
impl<'a> DraftingSemanticSelectionInputs<'a> {
    /// Freshly walk complete local records in an unattached CommandState/None owner scope,then
    /// check N² arithmetic/work before any duplicate-key scan. Preserve original row order and
    /// actual first duplicate key/indices;different pins/origins do not excuse a duplicate key.
    /// Invalid/missing/extra requested coordinates remain data for later actual-scope coverage.
    /// No query,source/body/value selection,result map or success allocation occurs.
    /// ```
    /// use sc_core::construction::*;
    /// fn local<'r,'a>(inputs:&'r DraftingSemanticSelectionInputs<'a>,limits:DraftingSemanticSelectionLimits)
    ///     ->Result<AdmittedDraftingSemanticSelections<'r,'a>,DraftingSemanticSelectionAdmissionError<'r,'a>> {
    ///     inputs.admit(limits)
    /// }
    /// ```
    /// # Errors
    /// Original first content,comparison arithmetic/quota or duplicate-key cause,with the raw owner.
    pub fn admit(
        &self,
        limits: DraftingSemanticSelectionLimits,
    ) -> Result<
        AdmittedDraftingSemanticSelections<'_, 'a>,
        DraftingSemanticSelectionAdmissionError<'_, 'a>,
    > {
        let mut meter = DesignResourceMeter::new(limits.content());
        let charged =
            self.charge_record_resources(&mut meter, DesignResourceScope::CommandState, None);
        let demand = meter.finish().map_err(|cause| {
            DraftingSemanticSelectionAdmissionError::new(
                self,
                limits,
                None,
                None,
                DraftingSemanticSelectionRefusal::Resources(cause),
            )
        })?;
        charged.map_err(|cause| {
            DraftingSemanticSelectionAdmissionError::new(
                self,
                limits,
                None,
                None,
                DraftingSemanticSelectionRefusal::Resources(cause),
            )
        })?;
        let rows = self.rows().len();
        let comparisons = bound(rows).ok_or_else(|| {
            DraftingSemanticSelectionAdmissionError::new(
                self,
                limits,
                Some(demand),
                None,
                DraftingSemanticSelectionRefusal::PopulationOverflow,
            )
        })?;
        let shape = DraftingSemanticSelectionShape { rows, comparisons };
        let refuse = |refusal| {
            DraftingSemanticSelectionAdmissionError::new(
                self,
                limits,
                Some(demand),
                Some(shape),
                refusal,
            )
        };
        if comparisons > limits.key_comparisons() {
            return Err(refuse(DraftingSemanticSelectionRefusal::KeyComparisons {
                allowed: limits.key_comparisons(),
                required: comparisons,
            }));
        }
        for (duplicate, row) in self.rows().iter().enumerate() {
            let key: DraftingSemanticSelectionKey = row.key();
            if let Some(first) = self
                .rows()
                .iter()
                .take(duplicate)
                .position(|earlier| earlier.key() == key)
            {
                return Err(refuse(DraftingSemanticSelectionRefusal::DuplicateKey {
                    first,
                    duplicate,
                    key,
                }));
            }
        }
        Ok(AdmittedDraftingSemanticSelections {
            inputs: self,
            limits,
            demand,
            shape,
        })
    }
}
fn bound(rows: usize) -> Option<usize> {
    rows.checked_mul(rows)
}
#[cfg(test)]
mod tests {
    use super::bound;
    #[test]
    fn selection_comparison_bound_refuses_square_overflow() {
        assert_eq!(bound(0), Some(0));
        assert_eq!(bound(4), Some(16));
        assert_eq!(bound(usize::MAX), None);
        assert_eq!(bound(usize::MAX / 2 + 1), None);
    }
}
