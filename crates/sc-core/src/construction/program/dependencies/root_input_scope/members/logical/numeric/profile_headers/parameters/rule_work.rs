//! Complete original rule/policy work admission after complete Root/Profile operation inputs.
use super::BoundDraftingProgramProfileParameters;
use crate::{
    construction::{
        AdmittedDraftingRuleWork, DraftingGarmentProvider, DraftingInitialInputProvider,
        DraftingProfileLogicalProvider, DraftingProgramSemanticInputs, DraftingRuleBindingLimits,
        DraftingRuleProvider, DraftingRuleSelection, DraftingSemanticField,
        DraftingSemanticSelectionLocation, DraftingSemanticSelectionRow,
    },
    design::{
        DesignMetadataHost, DesignReferenceHost, DesignResourceDemand, DesignResourceMeter,
        DesignResourceSource,
    },
    recipe::SemanticOperation,
};
use core::fmt;
mod error;
mod sources;
pub use error::{DraftingProgramRuleWorkError, DraftingProgramRuleWorkRefusal};
pub use sources::{
    BoundDraftingProgramGarmentProfileLogicalHeaders,
    BoundDraftingProgramGarmentProfileLogicalParameters,
    BoundDraftingProgramGarmentProfileNumericHeaders, BoundDraftingProgramGarmentRootInputMembers,
    BoundDraftingProgramGarmentRootLogicalSuppliers,
    BoundDraftingProgramGarmentRootNumericSuppliers, BoundDraftingProgramRuleSignatures,
    BoundDraftingProgramRuleSources, DraftingProgramGarmentProfileLogicalHeader,
    DraftingProgramGarmentProfileLogicalHeaderError,
    DraftingProgramGarmentProfileLogicalHeaderRefusal,
    DraftingProgramGarmentProfileLogicalHeadersResult,
    DraftingProgramGarmentProfileLogicalParameter,
    DraftingProgramGarmentProfileLogicalParameterError,
    DraftingProgramGarmentProfileLogicalParameterRefusal,
    DraftingProgramGarmentProfileLogicalParametersResult,
    DraftingProgramGarmentProfileLogicalRequest, DraftingProgramGarmentProfileNumericHeader,
    DraftingProgramGarmentProfileNumericHeaderError,
    DraftingProgramGarmentProfileNumericHeaderRefusal,
    DraftingProgramGarmentProfileNumericHeadersResult, DraftingProgramGarmentProfileNumericRequest,
    DraftingProgramGarmentRootInputMember, DraftingProgramGarmentRootInputMemberError,
    DraftingProgramGarmentRootInputMemberRefusal, DraftingProgramGarmentRootInputMembersResult,
    DraftingProgramGarmentRootInputSource, DraftingProgramGarmentRootLogicalSupplier,
    DraftingProgramGarmentRootLogicalSupplierError,
    DraftingProgramGarmentRootLogicalSupplierRefusal,
    DraftingProgramGarmentRootLogicalSuppliersResult,
    DraftingProgramGarmentRootNumericSupplierError,
    DraftingProgramGarmentRootNumericSupplierRefusal,
    DraftingProgramGarmentRootNumericSuppliersResult, DraftingProgramRuleSignature,
    DraftingProgramRuleSignatureError, DraftingProgramRuleSignatureRefusal,
    DraftingProgramRuleSignaturesResult, DraftingProgramRuleSource, DraftingProgramRuleSourceError,
    DraftingProgramRuleSourceRefusal, DraftingProgramRuleSourcesResult,
};

/// One actual original rule/policy field and its exact checked explicit origin row.
/// This is selection intent,not accepted rule or Profile source association.
#[derive(Clone, Copy)]
pub struct DraftingProgramRuleWorkRequest<'w, 'a> {
    operation: &'a SemanticOperation,
    ordinal: usize,
    position: usize,
    location: DraftingSemanticSelectionLocation,
    field: DraftingSemanticField<'a>,
    row_index: usize,
    origin: DraftingRuleSelection<'w>,
}
impl<'w, 'a> DraftingProgramRuleWorkRequest<'w, 'a> {
    /// Actual original Root semantic operation loan.
    #[must_use]
    pub const fn operation(self) -> &'a SemanticOperation {
        self.operation
    }
    /// Genuine semantic ordinal,independent of mixed position.
    #[must_use]
    pub const fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Genuine original mixed consumer position.
    #[must_use]
    pub const fn position(self) -> usize {
        self.position
    }
    /// Actual original rule/policy field coordinate.
    #[must_use]
    pub const fn location(self) -> DraftingSemanticSelectionLocation {
        self.location
    }
    /// Exact original rule/policy reference loan.
    #[must_use]
    pub const fn field(self) -> DraftingSemanticField<'a> {
        self.field
    }
    /// Zero-based original direct Rule row,not a projected or inferred selection.
    #[must_use]
    pub const fn row_index(self) -> usize {
        self.row_index
    }
    /// Explicit original Design intent or original requested Profile pin loan.
    #[must_use]
    pub const fn origin(self) -> DraftingRuleSelection<'w> {
        self.origin
    }
}
impl fmt::Debug for DraftingProgramRuleWorkRequest<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramRuleWorkRequest")
            .field("ordinal", &self.ordinal)
            .field("position", &self.position)
            .field("location", &self.location)
            .field("row_index", &self.row_index)
            .finish_non_exhaustive()
    }
}
/// One original rule/policy request and its freshly admitted original Root operation work parent.
pub struct DraftingProgramRuleWork<
    'w,
    'a,
    H: DesignMetadataHost,
    S: DraftingRuleProvider<H> + ?Sized,
> {
    request: DraftingProgramRuleWorkRequest<'w, 'a>,
    work: AdmittedDraftingRuleWork<'w, H, S>,
}
impl<'w, 'a, H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized>
    DraftingProgramRuleWork<'w, 'a, H, S>
{
    /// Original operation,field,position and checked selection witnesses.
    #[must_use]
    pub const fn request(&self) -> DraftingProgramRuleWorkRequest<'w, 'a> {
        self.request
    }
    /// Stable original work parent;later source binding must borrow this actual record.
    #[must_use]
    pub const fn work(&self) -> &AdmittedDraftingRuleWork<'w, H, S> {
        &self.work
    }
}
impl<H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramRuleWork<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramRuleWork")
            .field("request", &self.request)
            .finish_non_exhaustive()
    }
}
/// Complete original rule/policy work class retaining all original operation inputs and one cursor.
/// No rule/Profile source,signature,canonical output,body,value or whole cohort is accepted.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:&mut AdmittedDraftingProgramRuleWork<'_,'_,'_,'_,I,H,S>) {p.records.clear();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn duplicate<I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:AdmittedDraftingProgramRuleWork<'_,'_,'_,'_,I,H,S>) {let _=p.clone();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn escape<'h,'v,'w,'a,I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:AdmittedDraftingProgramRuleWork<'h,'v,'w,'a,I,H,S>)->AdmittedDraftingProgramRuleWork<'static,'v,'w,'a,I,H,S> {p}
/// ```
pub struct AdmittedDraftingProgramRuleWork<
    'h,
    'v,
    'w,
    'a,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
> {
    inputs: BoundDraftingProgramProfileParameters<'h, 'v, 'w, 'a, I, H, S>,
    demand: Option<DesignResourceDemand>,
    records: Vec<DraftingProgramRuleWork<'w, 'a, H, S>>,
}
impl<
        'h,
        'v,
        'w,
        'a,
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > AdmittedDraftingProgramRuleWork<'h, 'v, 'w, 'a, I, H, S>
{
    /// Complete original operation inputs and the same uniquely reserved finite cursor.
    #[must_use]
    pub const fn inputs(&self) -> &BoundDraftingProgramProfileParameters<'h, 'v, 'w, 'a, I, H, S> {
        &self.inputs
    }
    /// Fresh full aggregate/proof content before nonempty rule work;None for N0.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.demand
    }
    /// Every original rule/policy once,in actual mixed order without deduplication.
    #[must_use]
    pub fn records(&self) -> &[DraftingProgramRuleWork<'w, 'a, H, S>] {
        &self.records
    }
    /// Zero for N0,otherwise N+1 freshly reserved whole/child work passes.
    #[must_use]
    pub fn child_pass_count(&self) -> usize {
        if self.records.is_empty() {
            0
        } else {
            self.records.len() + 1
        }
    }
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > fmt::Debug for AdmittedDraftingProgramRuleWork<'_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AdmittedDraftingProgramRuleWork")
            .field("records", &self.records.len())
            .finish_non_exhaustive()
    }
}
/// Complete original work class or all original input/cursor owners and the first typed cause.
pub type DraftingProgramRuleWorkResult<'h, 'v, 'w, 'a, I, H, S> = Result<
    AdmittedDraftingProgramRuleWork<'h, 'v, 'w, 'a, I, H, S>,
    DraftingProgramRuleWorkError<'h, 'v, 'w, 'a, I, H, S>,
>;
fn is_rule(field: DraftingSemanticField<'_>) -> bool {
    matches!(
        field,
        DraftingSemanticField::LogicalRule(_) | DraftingSemanticField::ButtonholePolicy(_)
    )
}
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
    BoundDraftingProgramProfileParameters<
        'h,
        'v,
        'w,
        'a,
        DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        H,
        S,
    >
{
    /// Reserve complete N+1 whole/child passes for N>0 before fresh full aggregate/proof content,
    /// fallible N-record reservation and any original Root operation lookup. Reuse each original
    /// checked Rule row's explicit origin and existing prepare_rule_binding. Its content policy
    /// and finite matching ceiling come from this same admitted cursor;each child freshly charges
    /// content and derives its own required bound. Earlier demand supplies no acceptance credit.
    /// No Profile/rule/currentness/member/value query or signature/output association occurs.
    /// N0 does no new child work,walk,allocation or query and grants no new demand.
    /// # Errors
    /// Complete arithmetic,atomic cursor shortage,fresh aggregate resources,allocator,then first
    /// original child work cause. Original complete input/cursor owners survive;private successful
    /// work prefixes drop. Requested origins remain intent until actual source binding.
    pub fn admit_rule_work(
        self,
    ) -> DraftingProgramRuleWorkResult<
        'h,
        'v,
        'w,
        'a,
        DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        H,
        S,
    > {
        use DraftingProgramRuleWorkRefusal as R;
        let inputs = self.root.logical.members.owner.inputs();
        let program = inputs
            .geometry()
            .work()
            .inputs()
            .inputs()
            .order()
            .inputs()
            .roles()
            .positions()
            .work()
            .inputs()
            .program();
        let count = program.semantic_dependencies().try_fold(0usize, |n, use_| {
            n.checked_add(usize::from(is_rule(use_.field())))
        });
        let Some(count) = count else {
            return Err(DraftingProgramRuleWorkError::new(
                self,
                None,
                None,
                R::PopulationOverflow,
            ));
        };
        let passes = if count == 0 {
            Some(0)
        } else {
            count.checked_add(1)
        };
        let Some(passes) = passes else {
            return Err(DraftingProgramRuleWorkError::new(
                self,
                Some(count),
                None,
                R::PopulationOverflow,
            ));
        };
        if let Err(cause) = self
            .root
            .logical
            .members
            .owner
            .selections
            .reserve_child_passes(passes)
        {
            return Err(DraftingProgramRuleWorkError::new(
                self,
                Some(count),
                None,
                R::Work(cause),
            ));
        }
        if count == 0 {
            return Ok(AdmittedDraftingProgramRuleWork {
                inputs: self,
                demand: None,
                records: Vec::new(),
            });
        }
        let cursor = &self.root.logical.members.owner.selections;
        let content = cursor.limits().population().content();
        let limits = DraftingRuleBindingLimits::new(
            content,
            cursor.work().shape().per_pass_matching_bound(),
        );
        let mut meter = DesignResourceMeter::new(content);
        let charged = inputs.charge_resources(&mut meter);
        let demand = match meter.finish() {
            Ok(d) => d,
            Err(cause) => {
                return Err(DraftingProgramRuleWorkError::new(
                    self,
                    Some(count),
                    None,
                    R::Resources(cause),
                ))
            }
        };
        if let Err(cause) = charged {
            return Err(DraftingProgramRuleWorkError::new(
                self,
                Some(count),
                None,
                R::Resources(cause),
            ));
        }
        let mut records = Vec::new();
        if let Err(cause) = records.try_reserve_exact(count) {
            return Err(DraftingProgramRuleWorkError::new(
                self,
                Some(count),
                Some(demand),
                R::Reservation {
                    records: count,
                    cause,
                },
            ));
        }
        let rows = inputs.selections().inputs().rows();
        for use_ in program.semantic_dependencies() {
            if !is_rule(use_.field()) {
                continue;
            }
            let location = use_.selection_location();
            let row_index = self
                .root
                .logical
                .members
                .owner
                .selections
                .associations()
                .iter()
                .find(|a| a.location() == location)
                .and_then(|a| a.row_index())
                .unwrap_or_else(|| {
                    unreachable!("complete direct rule coverage retains the original Rule row")
                });
            let Some(DraftingSemanticSelectionRow::Rule { origin, .. }) = rows.get(row_index)
            else {
                unreachable!("complete checked Rule key retains the actual origin row")
            };
            let request = DraftingProgramRuleWorkRequest {
                operation: use_.operation(),
                ordinal: use_.ordinal(),
                position: use_.position(),
                location,
                field: use_.field(),
                row_index,
                origin: *origin,
            };
            let work = match self
                .root
                .logical
                .members
                .owner
                .scope
                .context()
                .prepare_rule_binding(
                    use_.operation().id(),
                    use_.operation().kind(),
                    *origin,
                    limits,
                ) {
                Ok(work) => work,
                Err(cause) => {
                    return Err(DraftingProgramRuleWorkError::new(
                        self,
                        Some(count),
                        Some(demand),
                        R::Child { request, cause },
                    ))
                }
            };
            records.push(DraftingProgramRuleWork { request, work });
        }
        Ok(AdmittedDraftingProgramRuleWork {
            inputs: self,
            demand: Some(demand),
            records,
        })
    }
}
