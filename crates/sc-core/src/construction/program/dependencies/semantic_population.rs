//! Fresh original consumer/request populations; no coverage, child work or source acceptance.
use super::DraftingProgramSemanticInputs;
use crate::{
    construction::{DraftingGarmentProvider, DraftingInitialInputProvider},
    design::{
        DesignReferenceHost, DesignResourceDemand, DesignResourceLimitError, DesignResourceLimits,
        DesignResourceMeter, DesignResourceSource,
    },
};
use core::fmt;

/// Explicit fresh content, original occurrence and direct association-work policy.
/// Child factories, nested geometry and final source result buffers require separate bounds.
/// ```compile_fail
/// let limits:sc_core::construction::DraftingProgramSemanticPopulationLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingProgramSemanticPopulationLimits {
    content: DesignResourceLimits,
    occurrences: usize,
    comparisons: usize,
    associations: usize,
}
impl DraftingProgramSemanticPopulationLimits {
    /// Select every allowance explicitly; zero direct populations permit an empty cohort.
    #[must_use]
    pub const fn new(
        content: DesignResourceLimits,
        occurrences: usize,
        selection_comparisons: usize,
        association_records: usize,
    ) -> Self {
        Self {
            content,
            occurrences,
            comparisons: selection_comparisons,
            associations: association_records,
        }
    }
    /// Complete fresh whole garment/programme/request/proof content allowance.
    #[must_use]
    pub const fn content(self) -> DesignResourceLimits {
        self.content
    }
    /// Maximum complete original garment plus semantic plus native occurrences, O.
    #[must_use]
    pub const fn occurrences(self) -> usize {
        self.occurrences
    }
    /// Maximum conservative 2*O*R fixed-key comparisons for both coverage directions.
    #[must_use]
    pub const fn selection_comparisons(self) -> usize {
        self.comparisons
    }
    /// Maximum O association slots plus R row-use slots; no child/source result credit.
    #[must_use]
    pub const fn association_records(self) -> usize {
        self.associations
    }
}
/// Actual complete original populations and conservative direct coverage/association bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingProgramSemanticPopulationShape {
    garment: usize,
    semantic: usize,
    native: usize,
    rows: usize,
    occurrences: usize,
    comparisons: usize,
    associations: usize,
}
impl DraftingProgramSemanticPopulationShape {
    /// Every original Pocket/Closure field occurrence, including repeated components/instances.
    #[must_use]
    pub const fn garment_occurrences(self) -> usize {
        self.garment
    }
    /// Every original ordered semantic header/input field occurrence.
    #[must_use]
    pub const fn semantic_occurrences(self) -> usize {
        self.semantic
    }
    /// Every original native target/direct Profile field in genuine mixed order.
    #[must_use]
    pub const fn native_occurrences(self) -> usize {
        self.native
    }
    /// Every original selection row, including invalid or unused coordinates.
    #[must_use]
    pub const fn selection_rows(self) -> usize {
        self.rows
    }
    /// O, the complete original garment/semantic/native occurrence population.
    #[must_use]
    pub const fn occurrence_count(self) -> usize {
        self.occurrences
    }
    /// Checked conservative 2*O*R fixed-key comparisons, excluding strings/child work.
    #[must_use]
    pub const fn selection_comparison_bound(self) -> usize {
        self.comparisons
    }
    /// Checked O+R direct association/row-use slots, excluding child/source buffers.
    #[must_use]
    pub const fn association_record_bound(self) -> usize {
        self.associations
    }
}
/// First original content, arithmetic or direct-population quota refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingProgramSemanticPopulationRefusal {
    /// Original whole-content field cause.
    Resources(DesignResourceLimitError),
    /// Complete original count/addition/multiplication does not fit usize.
    PopulationOverflow,
    /// Complete original O exceeds its selected policy.
    Occurrences {
        /// Selected maximum.
        allowed: usize,
        /// Complete original count.
        required: usize,
    },
    /// Complete conservative 2*O*R exceeds its selected policy.
    SelectionComparisons {
        /// Selected maximum.
        allowed: usize,
        /// Complete prospective bound.
        required: usize,
    },
    /// Complete O+R direct association/row-use slots exceed their selected policy.
    AssociationRecords {
        /// Selected maximum.
        allowed: usize,
        /// Complete prospective bound.
        required: usize,
    },
}
/// Exact original input owner and selected policy with the original first cause. I is the actual
/// owner type retained by the factory; public callers cannot construct an error or accepted prefix.
pub struct DraftingProgramSemanticPopulationError<'w, I: ?Sized> {
    details: Box<Failure<'w, I>>,
}
struct Failure<'w, I: ?Sized> {
    inputs: &'w I,
    limits: &'w DraftingProgramSemanticPopulationLimits,
    demand: Option<DesignResourceDemand>,
    shape: Option<DraftingProgramSemanticPopulationShape>,
    refusal: DraftingProgramSemanticPopulationRefusal,
}
impl<'w, I: ?Sized> DraftingProgramSemanticPopulationError<'w, I> {
    fn new(
        inputs: &'w I,
        limits: &'w DraftingProgramSemanticPopulationLimits,
        demand: Option<DesignResourceDemand>,
        shape: Option<DraftingProgramSemanticPopulationShape>,
        refusal: DraftingProgramSemanticPopulationRefusal,
    ) -> Self {
        Self {
            details: Box::new(Failure {
                inputs,
                limits,
                demand,
                shape,
                refusal,
            }),
        }
    }
    /// Exact actual joined owner, preserving every supplied source/proof/request loan.
    #[must_use]
    pub const fn inputs(&self) -> &'w I {
        self.details.inputs
    }
    /// Exact original explicit selected policy.
    #[must_use]
    pub const fn limits(&self) -> &'w DraftingProgramSemanticPopulationLimits {
        self.details.limits
    }
    /// Complete fresh content only after that entire stage passed; never a partial demand.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.details.demand
    }
    /// Complete counted shape only after all arithmetic passed; it grants no accepted quota.
    #[must_use]
    pub const fn shape(&self) -> Option<DraftingProgramSemanticPopulationShape> {
        self.details.shape
    }
    /// Original first field/arithmetic/quota cause.
    #[must_use]
    pub const fn refusal(&self) -> DraftingProgramSemanticPopulationRefusal {
        self.details.refusal
    }
}
impl<I: ?Sized> fmt::Debug for DraftingProgramSemanticPopulationError<'_, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramSemanticPopulationError")
            .field("refusal", &self.details.refusal)
            .finish_non_exhaustive()
    }
}
impl<I: ?Sized> fmt::Display for DraftingProgramSemanticPopulationError<'_, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "drafting semantic population: {:?}",
            self.details.refusal
        )
    }
}
impl<I: ?Sized> std::error::Error for DraftingProgramSemanticPopulationError<'_, I> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.details.refusal {
            DraftingProgramSemanticPopulationRefusal::Resources(cause) => Some(cause),
            _ => None,
        }
    }
}
/// Privately admitted fresh content and complete direct consumer/request population policy.
/// I remains the exact original joined owner type. This lends no source/coverage/child-work,
/// nested-geometry or final cohort acceptance. Core allocates no successful result vector.
/// ```compile_fail,E0616
/// fn alter<I:?Sized>(proof:&mut sc_core::construction::AdmittedDraftingProgramSemanticPopulation<'_,I>) {proof.inputs=todo!();}
/// ```
/// ```compile_fail
/// use sc_core::construction::AdmittedDraftingProgramSemanticPopulation;
/// fn escape<'w,I:?Sized>(proof:AdmittedDraftingProgramSemanticPopulation<'w,I>)
///     ->AdmittedDraftingProgramSemanticPopulation<'static,I> {proof}
/// ```
pub struct AdmittedDraftingProgramSemanticPopulation<'w, I: ?Sized> {
    inputs: &'w I,
    limits: &'w DraftingProgramSemanticPopulationLimits,
    demand: DesignResourceDemand,
    shape: DraftingProgramSemanticPopulationShape,
}
impl<'w, I: ?Sized> AdmittedDraftingProgramSemanticPopulation<'w, I> {
    /// Exact actual joined input owner; no copied source/request/proof collection.
    #[must_use]
    pub const fn inputs(&self) -> &'w I {
        self.inputs
    }
    /// Exact original selected direct-population policy.
    #[must_use]
    pub const fn limits(&self) -> &'w DraftingProgramSemanticPopulationLimits {
        self.limits
    }
    /// Complete freshly charged original whole content, without earlier admission credit.
    #[must_use]
    pub const fn demand(&self) -> DesignResourceDemand {
        self.demand
    }
    /// Actual original complete counts and direct association bounds.
    #[must_use]
    pub const fn shape(&self) -> DraftingProgramSemanticPopulationShape {
        self.shape
    }
}
impl<I: ?Sized> fmt::Debug for AdmittedDraftingProgramSemanticPopulation<'_, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AdmittedDraftingProgramSemanticPopulation")
            .field("shape", &self.shape)
            .finish_non_exhaustive()
    }
}
fn shape(
    garment: usize,
    semantic: usize,
    native: usize,
    rows: usize,
) -> Option<DraftingProgramSemanticPopulationShape> {
    let occurrences = garment.checked_add(semantic)?.checked_add(native)?;
    let cross = occurrences.checked_mul(rows)?;
    let comparisons = cross.checked_add(cross)?;
    let associations = occurrences.checked_add(rows)?;
    Some(DraftingProgramSemanticPopulationShape {
        garment,
        semantic,
        native,
        rows,
        occurrences,
        comparisons,
        associations,
    })
}
fn count(mut items: impl Iterator) -> Option<usize> {
    items.try_fold(0usize, |n, _| n.checked_add(1))
}
impl<
        's,
        'g,
        'k,
        'j,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignReferenceHost + DesignResourceSource,
        S: DraftingInitialInputProvider<H> + DraftingGarmentProvider<H> + ?Sized,
    > DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Freshly charge the complete original content,then stream all original garment/semantic/
    /// native fields and count every raw request row. Complete checked O,2*O*R,O+R arithmetic
    /// precedes occurrence,comparison and association-slot quotas in that order. No coverage scan,
    /// child/member/source query,value read,clone or successful result reservation occurs. A
    /// refused diagnostic is boxed; old demands and individually admitted scopes give no credit.
    /// Child-factory walks/matching,nested geometry and final source records need later explicit
    /// aggregate bounds. This direct population proof cannot approve an actual selection/source.
    /// ```
    /// use sc_core::{construction::*,design::*};
    /// fn populations<H:DesignReferenceHost+DesignResourceSource,
    ///     S:DraftingInitialInputProvider<H>+DraftingGarmentProvider<H>+?Sized>(
    ///     inputs:&DraftingProgramSemanticInputs<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_,H,S>,
    ///     limits:&DraftingProgramSemanticPopulationLimits)
    ///     ->Result<DraftingProgramSemanticPopulationShape,String> {
    ///     Ok(inputs.admit_consumer_population(limits).map_err(|e|e.to_string())?.shape())
    /// }
    /// ```
    /// # Errors
    /// Original first content field,complete count/arithmetic overflow or direct quota cause,
    /// preserving the actual original owner,selected policy and completed stage metadata only.
    pub fn admit_consumer_population<'w>(
        &'w self,
        limits: &'w DraftingProgramSemanticPopulationLimits,
    ) -> Result<
        AdmittedDraftingProgramSemanticPopulation<'w, Self>,
        DraftingProgramSemanticPopulationError<'w, Self>,
    > {
        use DraftingProgramSemanticPopulationRefusal as R;
        let refuse = |demand, shape, refusal| {
            DraftingProgramSemanticPopulationError::new(self, limits, demand, shape, refusal)
        };
        let mut meter = DesignResourceMeter::new(limits.content());
        let charged = self.charge_resources(&mut meter);
        let demand = meter
            .finish()
            .map_err(|cause| refuse(None, None, R::Resources(cause)))?;
        charged.map_err(|cause| refuse(None, None, R::Resources(cause)))?;
        let names = self
            .geometry()
            .work()
            .inputs()
            .inputs()
            .order()
            .inputs()
            .roles()
            .positions()
            .work()
            .inputs();
        let garment = count(
            self.garment()
                .source()
                .garment
                .pocket_closure_dependencies(),
        )
        .ok_or_else(|| refuse(Some(demand), None, R::PopulationOverflow))?;
        let semantic = count(names.program().semantic_dependencies())
            .ok_or_else(|| refuse(Some(demand), None, R::PopulationOverflow))?;
        let native = count(names.program().native_semantic_dependencies())
            .ok_or_else(|| refuse(Some(demand), None, R::PopulationOverflow))?;
        let rows = self.selections().inputs().rows().len();
        let shape = shape(garment, semantic, native, rows)
            .ok_or_else(|| refuse(Some(demand), None, R::PopulationOverflow))?;
        let refuse = |refusal| refuse(Some(demand), Some(shape), refusal);
        if shape.occurrences > limits.occurrences {
            return Err(refuse(R::Occurrences {
                allowed: limits.occurrences,
                required: shape.occurrences,
            }));
        }
        if shape.comparisons > limits.comparisons {
            return Err(refuse(R::SelectionComparisons {
                allowed: limits.comparisons,
                required: shape.comparisons,
            }));
        }
        if shape.associations > limits.associations {
            return Err(refuse(R::AssociationRecords {
                allowed: limits.associations,
                required: shape.associations,
            }));
        }
        Ok(AdmittedDraftingProgramSemanticPopulation {
            inputs: self,
            limits,
            demand,
            shape,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::shape;
    fn tuple(shape: super::DraftingProgramSemanticPopulationShape) -> (usize, usize, usize) {
        (
            shape.occurrence_count(),
            shape.selection_comparison_bound(),
            shape.association_record_bound(),
        )
    }
    #[test]
    fn semantic_population_checked_arithmetic_preserves_zero_products_and_refuses_overflow() {
        assert_eq!(shape(0, 0, 0, 0).map(tuple), Some((0, 0, 0)));
        assert_eq!(shape(2, 1, 2, 2).map(tuple), Some((5, 20, 7)));
        assert_eq!(
            shape(usize::MAX, 0, 0, 0).map(tuple),
            Some((usize::MAX, 0, usize::MAX))
        );
        assert!(shape(usize::MAX, 1, 0, 0).is_none());
        assert!(shape(usize::MAX, 0, 1, 0).is_none());
        assert!(shape(1, 0, 0, usize::MAX).is_none());
        assert!(shape(usize::MAX / 2 + 1, 0, 0, 1).is_none());
    }
}
