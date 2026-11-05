//! Complete original Profile logical parameters borrowing stable original accepted header parents.
use super::{BoundDraftingProgramProfileHeaders, DraftingProgramProfileInputHeader};
use crate::{
    construction::{
        BoundDraftingProfileLogicalSource, BoundDraftingProgramRootNumericSuppliers,
        DraftingGarmentProvider, DraftingInitialInputProvider, DraftingProfileLogicalProvider,
        DraftingProgramSemanticInputs,
    },
    design::{
        DesignMetadataHost, DesignReferenceHost, DesignResourceDemand, DesignResourceMeter,
        DesignResourceSource,
    },
};
use core::fmt;
mod compose;
mod error;
mod rule_work;
pub use compose::DraftingProgramOperationInput;
pub use error::{DraftingProgramProfileParameterError, DraftingProgramProfileParameterRefusal};
pub use rule_work::{
    AdmittedDraftingProgramRuleWork, BoundDraftingProgramGarmentRootInputMembers,
    BoundDraftingProgramGarmentRootLogicalSuppliers,
    BoundDraftingProgramGarmentRootNumericSuppliers, BoundDraftingProgramRuleSignatures,
    BoundDraftingProgramRuleSources, DraftingProgramGarmentRootInputMember,
    DraftingProgramGarmentRootInputMemberError, DraftingProgramGarmentRootInputMemberRefusal,
    DraftingProgramGarmentRootInputMembersResult, DraftingProgramGarmentRootInputSource,
    DraftingProgramGarmentRootLogicalSupplier, DraftingProgramGarmentRootLogicalSupplierError,
    DraftingProgramGarmentRootLogicalSupplierRefusal,
    DraftingProgramGarmentRootLogicalSuppliersResult,
    DraftingProgramGarmentRootNumericSupplierError,
    DraftingProgramGarmentRootNumericSupplierRefusal,
    DraftingProgramGarmentRootNumericSuppliersResult, DraftingProgramRuleSignature,
    DraftingProgramRuleSignatureError, DraftingProgramRuleSignatureRefusal,
    DraftingProgramRuleSignaturesResult, DraftingProgramRuleSource, DraftingProgramRuleSourceError,
    DraftingProgramRuleSourceRefusal, DraftingProgramRuleSourcesResult, DraftingProgramRuleWork,
    DraftingProgramRuleWorkError, DraftingProgramRuleWorkRefusal, DraftingProgramRuleWorkRequest,
    DraftingProgramRuleWorkResult,
};

/// One actual original input/header parent and its complete original parameter/member/kind proof.
/// Profile-owned states and external Derived operation references remain under that Profile source.
pub struct DraftingProgramProfileInputParameter<
    'h,
    'w,
    'a,
    H: DesignMetadataHost,
    S: DraftingProfileLogicalProvider<H> + ?Sized,
> {
    header_index: usize,
    header: &'h DraftingProgramProfileInputHeader<'w, 'a, H, S>,
    source: BoundDraftingProfileLogicalSource<'h, 'w, H, S>,
}
impl<'h, 'w, 'a, H: DesignMetadataHost, S: DraftingProfileLogicalProvider<H> + ?Sized>
    DraftingProgramProfileInputParameter<'h, 'w, 'a, H, S>
{
    /// Original header/consumer position in the complete header owner,without deduplication.
    #[must_use]
    pub const fn header_index(&self) -> usize {
        self.header_index
    }
    /// Exact original input/request/header parent,not another header over equal data.
    #[must_use]
    pub const fn header(&self) -> &'h DraftingProgramProfileInputHeader<'w, 'a, H, S> {
        self.header
    }
    /// Actual original parameter metadata,canonical member/kind/state and exact source parent.
    #[must_use]
    pub const fn source(&self) -> &BoundDraftingProfileLogicalSource<'h, 'w, H, S> {
        &self.source
    }
}
impl<H: DesignMetadataHost, S: DraftingProfileLogicalProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramProfileInputParameter<'_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramProfileInputParameter")
            .field("header_index", &self.header_index)
            .finish_non_exhaustive()
    }
}
/// Complete actual Profile operation-input parameter class. Stable original header storage stays
/// with its caller-owned parent;this proof borrows that storage and the same unique Root/cursor
/// through disjoint fields. No self-reference,source body clone or external Derived Root position.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>>(p:&mut BoundDraftingProgramProfileParameters<'_,'_,'_,'_,I,H,S>) {p.records.clear();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn duplicate<I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>>(p:BoundDraftingProgramProfileParameters<'_,'_,'_,'_,I,H,S>) {let _=p.clone();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn escape<'h,'v,'w,'a,I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>>(p:BoundDraftingProgramProfileParameters<'h,'v,'w,'a,I,H,S>)->BoundDraftingProgramProfileParameters<'static,'v,'w,'a,I,H,S> {p}
/// ```
pub struct BoundDraftingProgramProfileParameters<
    'h,
    'v,
    'w,
    'a,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingProfileLogicalProvider<H> + ?Sized,
> {
    root: &'h mut BoundDraftingProgramRootNumericSuppliers<'v, 'w, 'a, I, H, S>,
    headers: &'h [DraftingProgramProfileInputHeader<'w, 'a, H, S>],
    demand: Option<DesignResourceDemand>,
    records: Vec<DraftingProgramProfileInputParameter<'h, 'w, 'a, H, S>>,
}
impl<
        'h,
        'v,
        'w,
        'a,
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + ?Sized,
    > BoundDraftingProgramProfileParameters<'h, 'v, 'w, 'a, I, H, S>
{
    /// Exact original Root members/suppliers/keys and the same uniquely borrowed finite cursor.
    #[must_use]
    pub const fn root(&self) -> &BoundDraftingProgramRootNumericSuppliers<'v, 'w, 'a, I, H, S> {
        self.root
    }
    /// Every actual accepted original input/header parent in stable caller-owned storage.
    #[must_use]
    pub const fn headers(&self) -> &'h [DraftingProgramProfileInputHeader<'w, 'a, H, S>] {
        self.headers
    }
    /// Complete freshly measured original aggregate/proof content before nonempty parameter work.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.demand
    }
    /// Every original Profile input once,with its actual original parent and canonical association.
    #[must_use]
    pub fn records(&self) -> &[DraftingProgramProfileInputParameter<'h, 'w, 'a, H, S>] {
        &self.records
    }
    /// Zero for N0,otherwise N+1 fresh whole/parameter passes already reserved on the same cursor.
    #[must_use]
    pub fn child_pass_count(&self) -> usize {
        if self.records.is_empty() {
            0
        } else {
            self.records.len() + 1
        }
    }
}
impl<I: ?Sized, H: DesignMetadataHost, S: DraftingProfileLogicalProvider<H> + ?Sized> fmt::Debug
    for BoundDraftingProgramProfileParameters<'_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProgramProfileParameters")
            .field("records", &self.records.len())
            .finish_non_exhaustive()
    }
}
/// Complete actual parameters or original borrowed Root/headers/cursor and first typed cause.
pub type DraftingProgramProfileParametersResult<'h, 'v, 'w, 'a, I, H, S> = Result<
    BoundDraftingProgramProfileParameters<'h, 'v, 'w, 'a, I, H, S>,
    DraftingProgramProfileParameterError<'h, 'v, 'w, 'a, I, H, S>,
>;
impl<
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
            + ?Sized,
    >
    BoundDraftingProgramProfileHeaders<
        'v,
        'w,
        'a,
        DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        H,
        S,
    >
{
    /// Borrow stable actual header storage and the unique original cursor through disjoint fields.
    /// Reserve all N+1 whole/parameter passes for N>0 before fresh complete aggregate/proof content,
    /// fallible N-result reservation and N original parent-bound parameter queries. Each existing
    /// child factory freshly walks Root/provider/request pin before exact header/metadata/member/
    /// kind checks. Source metadata/member/state remain original loans;no selected value is read.
    /// N0 performs no reservation,content walk,allocation or query and grants no new demand.
    /// # Errors
    /// Complete arithmetic,atomic cursor reservation,first fresh aggregate content,allocator,then
    /// first original child parameter cause. Original Root/cursor and complete stable headers
    /// remain inspectable;all private successful parameter prefixes drop on refusal. Dropping the
    /// result releases its borrow without resetting any prior or newly reserved child work.
    pub fn bind_profile_input_parameters<'h>(
        &'h mut self,
    ) -> DraftingProgramProfileParametersResult<
        'h,
        'v,
        'w,
        'a,
        DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        H,
        S,
    > {
        use DraftingProgramProfileParameterRefusal as R;
        let Self {
            root,
            records: headers,
            demand: _,
        } = self;
        let headers: &'h [DraftingProgramProfileInputHeader<'w, 'a, H, S>] = headers;
        let count = headers.len();
        let passes = if count == 0 {
            Some(0)
        } else {
            count.checked_add(1)
        };
        let Some(passes) = passes else {
            return Err(DraftingProgramProfileParameterError::new(
                root,
                headers,
                None,
                R::PopulationOverflow,
            ));
        };
        if let Err(cause) = root
            .logical
            .members
            .owner
            .selections
            .reserve_child_passes(passes)
        {
            return Err(DraftingProgramProfileParameterError::new(
                root,
                headers,
                None,
                R::Work(cause),
            ));
        }
        if count == 0 {
            return Ok(BoundDraftingProgramProfileParameters {
                root,
                headers,
                demand: None,
                records: Vec::new(),
            });
        }
        let content = root
            .logical
            .members
            .owner
            .selections
            .limits()
            .population()
            .content();
        let mut meter = DesignResourceMeter::new(content);
        let charged = root
            .logical
            .members
            .owner
            .inputs()
            .charge_resources(&mut meter);
        let demand = match meter.finish() {
            Ok(demand) => demand,
            Err(cause) => {
                return Err(DraftingProgramProfileParameterError::new(
                    root,
                    headers,
                    None,
                    R::Resources(cause),
                ))
            }
        };
        if let Err(cause) = charged {
            return Err(DraftingProgramProfileParameterError::new(
                root,
                headers,
                None,
                R::Resources(cause),
            ));
        }
        let mut records = Vec::new();
        if let Err(cause) = records.try_reserve_exact(count) {
            return Err(DraftingProgramProfileParameterError::new(
                root,
                headers,
                Some(demand),
                R::Reservation {
                    records: count,
                    cause,
                },
            ));
        }
        for (header_index, header) in headers.iter().enumerate() {
            let request = header.request();
            let source = match header.source().bind_logical_parameter(
                request.parameter(),
                request.kind(),
                content,
            ) {
                Ok(source) => source,
                Err(cause) => {
                    return Err(DraftingProgramProfileParameterError::new(
                        root,
                        headers,
                        Some(demand),
                        R::Parameter {
                            header_index,
                            cause,
                        },
                    ))
                }
            };
            records.push(DraftingProgramProfileInputParameter {
                header_index,
                header,
                source,
            });
        }
        Ok(BoundDraftingProgramProfileParameters {
            root,
            headers,
            demand: Some(demand),
            records,
        })
    }
}
