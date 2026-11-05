//! Complete original Profile garment logical parameters borrowing their actual stable headers.
use super::{
    BoundDraftingProgramGarmentProfileLogicalHeaders,
    BoundDraftingProgramGarmentRootNumericSuppliers, DraftingProgramGarmentProfileLogicalHeader,
};
use crate::{
    construction::{
        BoundDraftingProfileLogicalSource, DraftingGarmentProvider, DraftingInitialInputProvider,
        DraftingProfileLogicalProvider, DraftingProgramSemanticInputs, DraftingRuleProvider,
    },
    design::{
        DesignMetadataHost, DesignReferenceHost, DesignResourceDemand, DesignResourceMeter,
        DesignResourceSource,
    },
};
use core::fmt;
mod error;
pub use error::{
    DraftingProgramGarmentProfileLogicalParameterError,
    DraftingProgramGarmentProfileLogicalParameterRefusal,
};
/// One genuine original garment/request/header parent and its original parameter/member/kind proof.
/// External Profile Derived references remain under that actual Profile source,without Root order.
pub struct DraftingProgramGarmentProfileLogicalParameter<
    't,
    'w,
    'a,
    H: DesignMetadataHost,
    S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
> {
    header_index: usize,
    header: &'t DraftingProgramGarmentProfileLogicalHeader<'w, 'a, H, S>,
    source: BoundDraftingProfileLogicalSource<'t, 'w, H, S>,
}
impl<
        't,
        'w,
        'a,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > DraftingProgramGarmentProfileLogicalParameter<'t, 'w, 'a, H, S>
{
    /// Original zero-based actual complete header-parent index.
    #[must_use]
    pub const fn header_index(&self) -> usize {
        self.header_index
    }
    /// Exact original requiring garment/request/header parent,not an equal-data replacement.
    #[must_use]
    pub const fn header(&self) -> &'t DraftingProgramGarmentProfileLogicalHeader<'w, 'a, H, S> {
        self.header
    }
    /// Actual original parameter metadata,Profile canonical member/kind/state and exact parent.
    #[must_use]
    pub const fn source(&self) -> &BoundDraftingProfileLogicalSource<'t, 'w, H, S> {
        &self.source
    }
}
impl<
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > fmt::Debug for DraftingProgramGarmentProfileLogicalParameter<'_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramGarmentProfileLogicalParameter")
            .field("header_index", &self.header_index)
            .finish_non_exhaustive()
    }
}
/// Complete actual logical Profile garment parameters borrowing stable original header storage
/// and the same unique Root/cursor through disjoint fields. No self-reference or body/value clone.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:&mut BoundDraftingProgramGarmentProfileLogicalParameters<'_,'_,'_,'_,'_,'_,'_,I,H,S>) {p.records.clear();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn duplicate<I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:BoundDraftingProgramGarmentProfileLogicalParameters<'_,'_,'_,'_,'_,'_,'_,I,H,S>) {let _=p.clone();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn escape<'t,'z,'q,'h,'v,'w,'a,I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:BoundDraftingProgramGarmentProfileLogicalParameters<'t,'z,'q,'h,'v,'w,'a,I,H,S>)->BoundDraftingProgramGarmentProfileLogicalParameters<'static,'z,'q,'h,'v,'w,'a,I,H,S> {p}
/// ```
pub struct BoundDraftingProgramGarmentProfileLogicalParameters<
    't,
    'z,
    'q,
    'h,
    'v,
    'w,
    'a,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
> {
    root: &'t mut BoundDraftingProgramGarmentRootNumericSuppliers<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
    headers: &'t [DraftingProgramGarmentProfileLogicalHeader<'w, 'a, H, S>],
    demand: Option<DesignResourceDemand>,
    records: Vec<DraftingProgramGarmentProfileLogicalParameter<'t, 'w, 'a, H, S>>,
}
impl<
        't,
        'z,
        'q,
        'h,
        'v,
        'w,
        'a,
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > BoundDraftingProgramGarmentProfileLogicalParameters<'t, 'z, 'q, 'h, 'v, 'w, 'a, I, H, S>
{
    /// Complete original Root garment/member/supplier/operation owners and same unique cursor.
    #[must_use]
    pub const fn root(
        &self,
    ) -> &BoundDraftingProgramGarmentRootNumericSuppliers<'z, 'q, 'h, 'v, 'w, 'a, I, H, S> {
        self.root
    }
    /// Complete stable original logical Profile header parents and original request fields.
    #[must_use]
    pub const fn headers(&self) -> &'t [DraftingProgramGarmentProfileLogicalHeader<'w, 'a, H, S>] {
        self.headers
    }
    /// Fresh full aggregate/proof demand before nonempty parameter work;None for N0.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.demand
    }
    /// Every original logical Profile garment parameter once,in actual header/garment order.
    #[must_use]
    pub fn records(&self) -> &[DraftingProgramGarmentProfileLogicalParameter<'t, 'w, 'a, H, S>] {
        &self.records
    }
    /// Zero for N0,otherwise N+1 fresh whole/parameter pass/result envelopes.
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
    > fmt::Debug
    for BoundDraftingProgramGarmentProfileLogicalParameters<'_, '_, '_, '_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProgramGarmentProfileLogicalParameters")
            .field("records", &self.records.len())
            .finish_non_exhaustive()
    }
}
/// Complete actual original parameters or original borrowed Root/headers/cursor and first cause.
pub type DraftingProgramGarmentProfileLogicalParametersResult<'t, 'z, 'q, 'h, 'v, 'w, 'a, I, H, S> =
    Result<
        BoundDraftingProgramGarmentProfileLogicalParameters<'t, 'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
        DraftingProgramGarmentProfileLogicalParameterError<'t, 'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
    >;
impl<
        'z,
        'q,
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
    BoundDraftingProgramGarmentProfileLogicalHeaders<
        'z,
        'q,
        'h,
        'v,
        'w,
        'a,
        DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        H,
        S,
    >
{
    /// Borrow actual stable headers and the same unique Root/cursor through disjoint fields.
    /// Reserve N+1 fresh whole/parameter passes for N>0 before complete aggregate/proof charging,
    /// complete N-record reservation and each existing parent-bound bind_logical_parameter query.
    /// Each child freshly charges Root/provider/request before exact original parameter/header/
    /// canonical/kind association. Actual domains/states and external Derived scope stay original;
    /// no Root position,selected value,source body or complete cohort is inferred or cloned.
    /// N0 adds no work,walk,allocation,query,reservation or demand claim.
    /// # Errors
    /// Complete arithmetic,atomic cursor shortage,fresh aggregate resources,allocator,then first
    /// original lending parameter/member/kind cause. Original complete Root/cursor and stable
    /// headers remain inspectable;private successful parameter prefixes drop. Dropping either
    /// result releases its borrow without resetting prior or newly reserved child work.
    pub fn bind_garment_profile_logical_parameters<'t>(
        &'t mut self,
    ) -> DraftingProgramGarmentProfileLogicalParametersResult<
        't,
        'z,
        'q,
        'h,
        'v,
        'w,
        'a,
        DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        H,
        S,
    > {
        use DraftingProgramGarmentProfileLogicalParameterRefusal as R;
        let Self {
            root,
            records: headers,
            demand: _,
        } = self;
        let headers: &'t [DraftingProgramGarmentProfileLogicalHeader<'w, 'a, H, S>] = headers;
        let count = headers.len();
        let passes = if count == 0 {
            Some(0)
        } else {
            count.checked_add(1)
        };
        let Some(passes) = passes else {
            return Err(DraftingProgramGarmentProfileLogicalParameterError::new(
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
            .inputs
            .root
            .logical
            .members
            .owner
            .selections
            .reserve_child_passes(passes)
        {
            return Err(DraftingProgramGarmentProfileLogicalParameterError::new(
                root,
                headers,
                None,
                R::Work(cause),
            ));
        }
        if count == 0 {
            return Ok(BoundDraftingProgramGarmentProfileLogicalParameters {
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
            .inputs
            .root
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
            .inputs
            .root
            .logical
            .members
            .owner
            .inputs()
            .charge_resources(&mut meter);
        let demand = match meter.finish() {
            Ok(d) => d,
            Err(cause) => {
                return Err(DraftingProgramGarmentProfileLogicalParameterError::new(
                    root,
                    headers,
                    None,
                    R::Resources(cause),
                ))
            }
        };
        if let Err(cause) = charged {
            return Err(DraftingProgramGarmentProfileLogicalParameterError::new(
                root,
                headers,
                None,
                R::Resources(cause),
            ));
        }
        let mut records = Vec::new();
        if let Err(cause) = records.try_reserve_exact(count) {
            return Err(DraftingProgramGarmentProfileLogicalParameterError::new(
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
                Ok(s) => s,
                Err(cause) => {
                    return Err(DraftingProgramGarmentProfileLogicalParameterError::new(
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
            records.push(DraftingProgramGarmentProfileLogicalParameter {
                header_index,
                header,
                source,
            });
        }
        Ok(BoundDraftingProgramGarmentProfileLogicalParameters {
            root,
            headers,
            demand: Some(demand),
            records,
        })
    }
}

mod numeric_headers;
pub use numeric_headers::{
    BoundDraftingProgramGarmentProfileNumericHeaders,
    BoundDraftingProgramGarmentProfileNumericParameters,
    DraftingProgramGarmentProfileNumericHeader, DraftingProgramGarmentProfileNumericHeaderError,
    DraftingProgramGarmentProfileNumericHeaderRefusal,
    DraftingProgramGarmentProfileNumericHeadersResult,
    DraftingProgramGarmentProfileNumericParameter,
    DraftingProgramGarmentProfileNumericParameterError,
    DraftingProgramGarmentProfileNumericParameterRefusal,
    DraftingProgramGarmentProfileNumericParametersResult,
    DraftingProgramGarmentProfileNumericRequest,
};
