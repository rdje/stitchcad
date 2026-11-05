//! Fresh complete content and prospective geometry work, before creator scans or source lookups.
use super::BoundDraftingProgramCanonicalInputs;
use crate::{
    construction::DraftingInitialInputProvider,
    design::{
        DesignMetadataHost, DesignResourceDemand, DesignResourceLimitError, DesignResourceLimits,
        DesignResourceMeter, DesignResourceSource,
    },
};
use core::fmt;

/// Explicit fresh content, occurrence, retained-record and creator-inspection allowances.
/// Existing proof buffers retain their original policies. These are logical populations, not
/// allocator bytes/capacity, elapsed CPU, scratch or execution allowances.
/// ```compile_fail
/// let policy:sc_core::construction::DraftingProgramGeometryLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingProgramGeometryLimits {
    content: DesignResourceLimits,
    occurrences: usize,
    records: usize,
    inspections: usize,
}
impl DraftingProgramGeometryLimits {
    /// Select all allowances explicitly; zero geometry work permits an empty occurrence stream.
    #[must_use]
    pub const fn new(
        content: DesignResourceLimits,
        occurrences: usize,
        records: usize,
        inspections: usize,
    ) -> Self {
        Self {
            content,
            occurrences,
            records,
            inspections,
        }
    }
    /// Complete fresh original programme/name/provider-held content allowance.
    #[must_use]
    pub const fn content(self) -> DesignResourceLimits {
        self.content
    }
    /// Maximum original formula plus native geometry occurrences, without deduplication.
    #[must_use]
    pub const fn occurrences(self) -> usize {
        self.occurrences
    }
    /// Maximum E request slots plus E supplier associations.
    #[must_use]
    pub const fn retained_records(self) -> usize {
        self.records
    }
    /// Conservative maximum E*S inspections, counting every original mixed slot per occurrence.
    #[must_use]
    pub const fn creator_inspections(self) -> usize {
        self.inspections
    }
}
/// Factory-derived complete original geometry populations and conservative prospective bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingProgramGeometryShape {
    formulas: usize,
    native: usize,
    steps: usize,
    occurrences: usize,
    records: usize,
    inspections: usize,
}
impl DraftingProgramGeometryShape {
    /// Original geometry-name uses in every Root source and complete mixed canonical stream.
    #[must_use]
    pub const fn formula_count(self) -> usize {
        self.formulas
    }
    /// Original direct native geometry fields, including all repeated selectors.
    #[must_use]
    pub const fn native_count(self) -> usize {
        self.native
    }
    /// Every original mixed slot, including scalar slots inspected by creator traversal.
    #[must_use]
    pub const fn step_count(self) -> usize {
        self.steps
    }
    /// E, the complete original formula plus native geometry occurrences.
    #[must_use]
    pub const fn occurrence_count(self) -> usize {
        self.occurrences
    }
    /// Complete additional E+E logical request and supplier-association record bound.
    #[must_use]
    pub const fn retained_record_bound(self) -> usize {
        self.records
    }
    /// Complete conservative E*S creator-slot inspection bound, not measured CPU.
    #[must_use]
    pub const fn creator_inspection_bound(self) -> usize {
        self.inspections
    }
}
/// First fresh content or complete prospective population refusal, without a formula diagnostic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingProgramGeometryWorkRefusal {
    /// Original first complete raw/normalized/name/provider-held field refusal.
    Resources(DesignResourceLimitError),
    /// Complete original count, addition or multiplication does not fit usize.
    PopulationOverflow,
    /// Complete E exceeds its selected allowance.
    Occurrences {
        /// Selected maximum.
        allowed: usize,
        /// Complete original population.
        required: usize,
    },
    /// Complete E+E exceeds its selected allowance.
    RetainedRecords {
        /// Selected maximum.
        allowed: usize,
        /// Complete conservative bound.
        required: usize,
    },
    /// Complete E*S exceeds its selected allowance.
    CreatorInspections {
        /// Selected maximum.
        allowed: usize,
        /// Complete conservative bound.
        required: usize,
    },
}
/// First refusal with the original complete canonical owner and exact selected geometry policy.
/// Only the semantic diagnostic is boxed; no request/source result vector is allocated.
pub struct DraftingProgramGeometryWorkError<
    'g,
    'k,
    'j,
    'i,
    'r,
    'b,
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingInitialInputProvider<H> + ?Sized,
> {
    inputs: &'g BoundDraftingProgramCanonicalInputs<'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    limits: &'g DraftingProgramGeometryLimits,
    refusal: Box<DraftingProgramGeometryWorkRefusal>,
}
impl<
        'g,
        'k,
        'j,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > DraftingProgramGeometryWorkError<'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Exact actual complete canonical owner; no successful geometry prefix exists.
    #[must_use]
    pub const fn inputs(
        &self,
    ) -> &'g BoundDraftingProgramCanonicalInputs<'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S> {
        self.inputs
    }
    /// Exact selected complete geometry policy.
    #[must_use]
    pub const fn limits(&self) -> &'g DraftingProgramGeometryLimits {
        self.limits
    }
    /// Original first field/population/quota refusal.
    #[must_use]
    pub fn refusal(&self) -> &DraftingProgramGeometryWorkRefusal {
        &self.refusal
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramGeometryWorkError<'_, '_, '_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramGeometryWorkError")
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Display
    for DraftingProgramGeometryWorkError<'_, '_, '_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "drafting geometry work: {:?}", self.refusal)
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> std::error::Error
    for DraftingProgramGeometryWorkError<'_, '_, '_, '_, '_, '_, '_, '_, '_, H, S>
{
}

/// Privately admitted complete fresh content and prospective geometry work. It binds no supplier,
/// frame, snapshot or semantic target and certifies no geometry, values or whole programme.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<H:DesignMetadataHost,S:DraftingInitialInputProvider<H>>(
///     work:&mut AdmittedDraftingProgramGeometryWork<'_, '_, '_, '_, '_, '_, '_, '_, '_,H,S>) {work.shape=todo!();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn escape<'g,'k,'j,'i,'r,'b,'n,'p,'a,H:DesignMetadataHost,S:DraftingInitialInputProvider<H>>(
///     work:AdmittedDraftingProgramGeometryWork<'g,'k,'j,'i,'r,'b,'n,'p,'a,H,S>)
///     ->AdmittedDraftingProgramGeometryWork<'static,'k,'j,'i,'r,'b,'n,'p,'a,H,S> {work}
/// ```
pub struct AdmittedDraftingProgramGeometryWork<
    'g,
    'k,
    'j,
    'i,
    'r,
    'b,
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingInitialInputProvider<H> + ?Sized,
> {
    inputs: &'g BoundDraftingProgramCanonicalInputs<'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    limits: &'g DraftingProgramGeometryLimits,
    demand: DesignResourceDemand,
    shape: DraftingProgramGeometryShape,
}
impl<
        'g,
        'k,
        'j,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > AdmittedDraftingProgramGeometryWork<'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Original actual complete canonical dependency owner.
    #[must_use]
    pub const fn inputs(
        &self,
    ) -> &'g BoundDraftingProgramCanonicalInputs<'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S> {
        self.inputs
    }
    /// Original exact selected policy.
    #[must_use]
    pub const fn limits(&self) -> &'g DraftingProgramGeometryLimits {
        self.limits
    }
    /// Complete freshly charged original content; previous demands supply no credit.
    #[must_use]
    pub const fn demand(&self) -> DesignResourceDemand {
        self.demand
    }
    /// Actual complete populations and conservative prospective work.
    #[must_use]
    pub const fn shape(&self) -> DraftingProgramGeometryShape {
        self.shape
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Debug
    for AdmittedDraftingProgramGeometryWork<'_, '_, '_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AdmittedDraftingProgramGeometryWork")
            .field("shape", &self.shape)
            .finish_non_exhaustive()
    }
}
fn count(mut items: impl Iterator) -> Result<usize, DraftingProgramGeometryWorkRefusal> {
    items.try_fold(0usize, |n, _| {
        n.checked_add(1)
            .ok_or(DraftingProgramGeometryWorkRefusal::PopulationOverflow)
    })
}
impl<
        'k,
        'j,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignResourceSource,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > BoundDraftingProgramCanonicalInputs<'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Freshly charge all original programme/name/provider-held content in one meter, then count
    /// every Root/mixed formula and native geometry use without classifying creators. Complete
    /// checked E=F+N, records=E+E and inspections=E*S arithmetic precedes E,records,inspection quotas.
    /// No geometry request/source result buffer, creator scan, source lookup or value read occurs.
    /// A refused semantic diagnostic is boxed; core creates no work/result buffer. Earlier proof
    /// buffers retain their independently selected source/order/graph/canonical policies.
    ///
    /// ```
    /// use sc_core::{construction::*,design::DesignResourceSource};
    /// fn geometry_work<H:DesignResourceSource,S:DraftingInitialInputProvider<H> + ?Sized>(
    ///     inputs:&BoundDraftingProgramCanonicalInputs<'_, '_, '_, '_, '_, '_, '_, '_,H,S>,
    ///     limits:&DraftingProgramGeometryLimits)->Result<DraftingProgramGeometryShape,String> {
    ///     Ok(inputs.admit_geometry_work(limits).map_err(|e|e.to_string())?.shape())
    /// }
    /// ```
    /// # Errors
    /// Original content refusal, complete population overflow, then occurrence/record/inspection
    /// quotas in that order, retaining actual canonical owner and selected policy on every refusal.
    pub fn admit_geometry_work<'g>(
        &'g self,
        limits: &'g DraftingProgramGeometryLimits,
    ) -> Result<
        AdmittedDraftingProgramGeometryWork<'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        DraftingProgramGeometryWorkError<'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    > {
        use DraftingProgramGeometryWorkRefusal as R;
        let refuse = |refusal| DraftingProgramGeometryWorkError {
            inputs: self,
            limits,
            refusal: Box::new(refusal),
        };
        let names = self
            .inputs()
            .order()
            .inputs()
            .roles()
            .positions()
            .work()
            .inputs();
        let mut meter = DesignResourceMeter::new(limits.content());
        names
            .charge_resources(&mut meter)
            .map_err(|e| refuse(R::Resources(e)))?;
        let demand = meter.finish().map_err(|e| refuse(R::Resources(e)))?;
        let formulas = count(self.formula_geometry_dependencies()).map_err(refuse)?;
        let native = count(self.native_geometry_dependencies()).map_err(refuse)?;
        let steps = names.program().operations().len();
        let occurrences = formulas
            .checked_add(native)
            .ok_or_else(|| refuse(R::PopulationOverflow))?;
        let records = occurrences
            .checked_add(occurrences)
            .ok_or_else(|| refuse(R::PopulationOverflow))?;
        let inspections = occurrences
            .checked_mul(steps)
            .ok_or_else(|| refuse(R::PopulationOverflow))?;
        if occurrences > limits.occurrences() {
            return Err(refuse(R::Occurrences {
                allowed: limits.occurrences(),
                required: occurrences,
            }));
        }
        if records > limits.retained_records() {
            return Err(refuse(R::RetainedRecords {
                allowed: limits.retained_records(),
                required: records,
            }));
        }
        if inspections > limits.creator_inspections() {
            return Err(refuse(R::CreatorInspections {
                allowed: limits.creator_inspections(),
                required: inspections,
            }));
        }
        Ok(AdmittedDraftingProgramGeometryWork {
            inputs: self,
            limits,
            demand,
            shape: DraftingProgramGeometryShape {
                formulas,
                native,
                steps,
                occurrences,
                records,
                inspections,
            },
        })
    }
}

mod retained_resources;
