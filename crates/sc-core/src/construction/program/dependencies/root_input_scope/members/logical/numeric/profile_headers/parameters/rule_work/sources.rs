//! Complete original rule sources borrowing stable actual work parents and one input/cursor owner.
use super::{
    AdmittedDraftingProgramRuleWork, BoundDraftingProgramProfileParameters, DraftingProgramRuleWork,
};
use crate::{
    construction::{
        BoundDraftingRuleSource, DraftingGarmentProvider, DraftingInitialInputProvider,
        DraftingProfileLogicalProvider, DraftingProgramSemanticInputs, DraftingRuleProvider,
        DraftingRuleSelection,
    },
    design::{
        DesignMetadataHost, DesignReferenceHost, DesignResourceDemand, DesignResourceMeter,
        DesignResourceSource,
    },
};
use core::fmt;
mod error;
mod signatures;
pub use error::{DraftingProgramRuleSourceError, DraftingProgramRuleSourceRefusal};
pub use signatures::{
    BoundDraftingProgramGarmentProfileLogicalHeaders,
    BoundDraftingProgramGarmentProfileLogicalParameters,
    BoundDraftingProgramGarmentProfileNumericHeaders,
    BoundDraftingProgramGarmentProfileNumericParameters,
    BoundDraftingProgramGarmentRootInputMembers, BoundDraftingProgramGarmentRootLogicalSuppliers,
    BoundDraftingProgramGarmentRootNumericSuppliers, BoundDraftingProgramRuleSignatures,
    DraftingProgramGarmentProfileLogicalHeader, DraftingProgramGarmentProfileLogicalHeaderError,
    DraftingProgramGarmentProfileLogicalHeaderRefusal,
    DraftingProgramGarmentProfileLogicalHeadersResult,
    DraftingProgramGarmentProfileLogicalParameter,
    DraftingProgramGarmentProfileLogicalParameterError,
    DraftingProgramGarmentProfileLogicalParameterRefusal,
    DraftingProgramGarmentProfileLogicalParametersResult,
    DraftingProgramGarmentProfileLogicalRequest, DraftingProgramGarmentProfileNumericHeader,
    DraftingProgramGarmentProfileNumericHeaderError,
    DraftingProgramGarmentProfileNumericHeaderRefusal,
    DraftingProgramGarmentProfileNumericHeadersResult,
    DraftingProgramGarmentProfileNumericParameter,
    DraftingProgramGarmentProfileNumericParameterError,
    DraftingProgramGarmentProfileNumericParameterRefusal,
    DraftingProgramGarmentProfileNumericParametersResult,
    DraftingProgramGarmentProfileNumericRequest, DraftingProgramGarmentRootInputMember,
    DraftingProgramGarmentRootInputMemberError, DraftingProgramGarmentRootInputMemberRefusal,
    DraftingProgramGarmentRootInputMembersResult, DraftingProgramGarmentRootInputSource,
    DraftingProgramGarmentRootLogicalSupplier, DraftingProgramGarmentRootLogicalSupplierError,
    DraftingProgramGarmentRootLogicalSupplierRefusal,
    DraftingProgramGarmentRootLogicalSuppliersResult,
    DraftingProgramGarmentRootNumericSupplierError,
    DraftingProgramGarmentRootNumericSupplierRefusal,
    DraftingProgramGarmentRootNumericSuppliersResult, DraftingProgramRuleSignature,
    DraftingProgramRuleSignatureError, DraftingProgramRuleSignatureRefusal,
    DraftingProgramRuleSignaturesResult,
};
/// One actual original work parent and accepted original rule/policy declaration/body/source.
pub struct DraftingProgramRuleSource<
    'q,
    'w,
    'a,
    H: DesignMetadataHost,
    S: DraftingRuleProvider<H> + ?Sized,
> {
    work_index: usize,
    work: &'q DraftingProgramRuleWork<'w, 'a, H, S>,
    source: BoundDraftingRuleSource<'q, 'w, H, S>,
}
impl<'q, 'w, 'a, H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized>
    DraftingProgramRuleSource<'q, 'w, 'a, H, S>
{
    /// Original zero-based complete work-parent index.
    #[must_use]
    pub const fn work_index(&self) -> usize {
        self.work_index
    }
    /// Actual original consumer/request/work parent,not an equal-looking replacement.
    #[must_use]
    pub const fn work(&self) -> &'q DraftingProgramRuleWork<'w, 'a, H, S> {
        self.work
    }
    /// Actual accepted original source;signature binding must borrow this stable parent.
    #[must_use]
    pub const fn source(&self) -> &BoundDraftingRuleSource<'q, 'w, H, S> {
        &self.source
    }
}
impl<H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramRuleSource<'_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramRuleSource")
            .field("work_index", &self.work_index)
            .finish_non_exhaustive()
    }
}
/// Complete original rule source class borrowing stable work storage and the same unique inputs/
/// cursor through disjoint fields. This accepts no local signature,canonical output or body truth.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:&mut BoundDraftingProgramRuleSources<'_,'_,'_,'_,'_,I,H,S>) {p.records.clear();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn duplicate<I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:BoundDraftingProgramRuleSources<'_,'_,'_,'_,'_,I,H,S>) {let _=p.clone();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn escape<'q,'h,'v,'w,'a,I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:BoundDraftingProgramRuleSources<'q,'h,'v,'w,'a,I,H,S>)->BoundDraftingProgramRuleSources<'static,'h,'v,'w,'a,I,H,S> {p}
/// ```
pub struct BoundDraftingProgramRuleSources<
    'q,
    'h,
    'v,
    'w,
    'a,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
> {
    inputs: &'q mut BoundDraftingProgramProfileParameters<'h, 'v, 'w, 'a, I, H, S>,
    works: &'q [DraftingProgramRuleWork<'w, 'a, H, S>],
    demand: Option<DesignResourceDemand>,
    passes: usize,
    records: Vec<DraftingProgramRuleSource<'q, 'w, 'a, H, S>>,
}
impl<
        'q,
        'h,
        'v,
        'w,
        'a,
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > BoundDraftingProgramRuleSources<'q, 'h, 'v, 'w, 'a, I, H, S>
{
    /// Exact complete original Root/Profile operation inputs and the same unique cursor.
    #[must_use]
    pub const fn inputs(&self) -> &BoundDraftingProgramProfileParameters<'h, 'v, 'w, 'a, I, H, S> {
        self.inputs
    }
    /// Complete stable actual work parents with original request/selection/consumer loans.
    #[must_use]
    pub const fn works(&self) -> &'q [DraftingProgramRuleWork<'w, 'a, H, S>] {
        self.works
    }
    /// Every original rule source once,in genuine original mixed order.
    #[must_use]
    pub fn records(&self) -> &[DraftingProgramRuleSource<'q, 'w, 'a, H, S>] {
        &self.records
    }
    /// Complete fresh whole aggregate/proof demand before nonempty source work;None for N0.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.demand
    }
    /// Zero for N0,otherwise N+M+1 whole/work/Profile passes,M actual Profile-origin works.
    #[must_use]
    pub const fn child_pass_count(&self) -> usize {
        self.passes
    }
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > fmt::Debug for BoundDraftingProgramRuleSources<'_, '_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProgramRuleSources")
            .field("records", &self.records.len())
            .finish_non_exhaustive()
    }
}
/// Complete source class or original borrowed inputs/work parents/cursor and first typed cause.
pub type DraftingProgramRuleSourcesResult<'q, 'h, 'v, 'w, 'a, I, H, S> = Result<
    BoundDraftingProgramRuleSources<'q, 'h, 'v, 'w, 'a, I, H, S>,
    DraftingProgramRuleSourceError<'q, 'h, 'v, 'w, 'a, I, H, S>,
>;
impl<
        'h,
        'v,
        'w,
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
        S: DraftingInitialInputProvider<H>
            + DraftingGarmentProvider<H>
            + DraftingProfileLogicalProvider<H>
            + DraftingRuleProvider<H>
            + ?Sized,
    >
    AdmittedDraftingProgramRuleWork<
        'h,
        'v,
        'w,
        'a,
        DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        H,
        S,
    >
{
    /// Borrow actual stable work storage and the same unique inputs/cursor through disjoint fields.
    /// Reserve N+M+1 passes for N>0 before full fresh aggregate/proof content,fallible N-result
    /// reservation and any source query. Each original bind_source repeats fresh work;M Profile
    /// origins each require another fresh Profile walk. Explicit source association preserves
    /// actual Root/snapshot/origin/pin/inventory/body/declaration/ID/family loans and first causes.
    /// N0 adds no work,walk,allocation,query or new demand. No local signature/output is accepted.
    /// # Errors
    /// Complete checked pass arithmetic,atomic cursor shortage,fresh aggregate resources,allocator,
    /// then first original child source cause. Private successful prefixes drop;original complete
    /// inputs,cursor and stable work parents remain inspectable. Dropping the result releases its
    /// borrow without resetting prior or newly reserved work.
    pub fn bind_rule_sources<'q>(
        &'q mut self,
    ) -> DraftingProgramRuleSourcesResult<
        'q,
        'h,
        'v,
        'w,
        'a,
        DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        H,
        S,
    > {
        use DraftingProgramRuleSourceRefusal as R;
        let Self {
            inputs,
            records: works,
            demand: _,
        } = self;
        let works: &'q [DraftingProgramRuleWork<'w, 'a, H, S>] = works;
        let count = works.len();
        let passes = if count == 0 {
            Some(0)
        } else {
            works.iter().try_fold(1usize, |n, work| {
                n.checked_add(
                    if matches!(work.work().selection(), DraftingRuleSelection::Profile(_)) {
                        2
                    } else {
                        1
                    },
                )
            })
        };
        let Some(passes) = passes else {
            return Err(DraftingProgramRuleSourceError::new(
                inputs,
                works,
                None,
                R::PopulationOverflow,
            ));
        };
        if let Err(cause) = inputs
            .root
            .logical
            .members
            .owner
            .selections
            .reserve_child_passes(passes)
        {
            return Err(DraftingProgramRuleSourceError::new(
                inputs,
                works,
                None,
                R::Work(cause),
            ));
        }
        if count == 0 {
            return Ok(BoundDraftingProgramRuleSources {
                inputs,
                works,
                demand: None,
                passes,
                records: Vec::new(),
            });
        }
        let content = inputs
            .root
            .logical
            .members
            .owner
            .selections
            .limits()
            .population()
            .content();
        let mut meter = DesignResourceMeter::new(content);
        let charged = inputs
            .root
            .logical
            .members
            .owner
            .inputs()
            .charge_resources(&mut meter);
        let demand = match meter.finish() {
            Ok(d) => d,
            Err(cause) => {
                return Err(DraftingProgramRuleSourceError::new(
                    inputs,
                    works,
                    None,
                    R::Resources(cause),
                ))
            }
        };
        if let Err(cause) = charged {
            return Err(DraftingProgramRuleSourceError::new(
                inputs,
                works,
                None,
                R::Resources(cause),
            ));
        }
        let mut records = Vec::new();
        if let Err(cause) = records.try_reserve_exact(count) {
            return Err(DraftingProgramRuleSourceError::new(
                inputs,
                works,
                Some(demand),
                R::Reservation {
                    records: count,
                    cause,
                },
            ));
        }
        for (work_index, work) in works.iter().enumerate() {
            let source = match work.work().bind_source() {
                Ok(source) => source,
                Err(cause) => {
                    return Err(DraftingProgramRuleSourceError::new(
                        inputs,
                        works,
                        Some(demand),
                        R::Source { work_index, cause },
                    ))
                }
            };
            records.push(DraftingProgramRuleSource {
                work_index,
                work,
                source,
            });
        }
        Ok(BoundDraftingProgramRuleSources {
            inputs,
            works,
            demand: Some(demand),
            passes,
            records,
        })
    }
}
