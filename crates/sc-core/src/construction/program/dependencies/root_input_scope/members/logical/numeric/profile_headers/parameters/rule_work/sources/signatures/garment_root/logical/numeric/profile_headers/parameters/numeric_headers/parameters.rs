//! Complete original Profile garment numeric parameters borrowing their actual stable headers.
use super::{
    BoundDraftingProgramGarmentProfileLogicalParameters,
    BoundDraftingProgramGarmentProfileNumericHeaders, DraftingProgramGarmentProfileNumericHeader,
};
use crate::{
    construction::{
        BoundDraftingProfileNumericSource, DraftingGarmentProvider, DraftingInitialInputProvider,
        DraftingProfileLogicalProvider, DraftingProfileNumericProvider,
        DraftingProgramSemanticInputs, DraftingRuleProvider,
    },
    design::{
        DesignMetadataHost, DesignReferenceHost, DesignResourceDemand, DesignResourceMeter,
        DesignResourceSource,
    },
};
use core::fmt;
mod error;
pub use error::{
    DraftingProgramGarmentProfileNumericParameterError,
    DraftingProgramGarmentProfileNumericParameterRefusal,
};
/// One genuine original garment/request/header parent and its original parameter/member/kind proof.
/// External Profile Derived references remain under that actual Profile source,without Root order.
pub struct DraftingProgramGarmentProfileNumericParameter<
    'u,
    'w,
    'a,
    H: DesignMetadataHost,
    S: DraftingProfileNumericProvider<H>
        + DraftingProfileLogicalProvider<H>
        + DraftingRuleProvider<H>
        + ?Sized,
> {
    header_index: usize,
    header: &'u DraftingProgramGarmentProfileNumericHeader<'w, 'a, H, S>,
    source: BoundDraftingProfileNumericSource<'u, 'w, H, S>,
}
impl<
        'u,
        'w,
        'a,
        H: DesignMetadataHost,
        S: DraftingProfileNumericProvider<H>
            + DraftingProfileLogicalProvider<H>
            + DraftingRuleProvider<H>
            + ?Sized,
    > DraftingProgramGarmentProfileNumericParameter<'u, 'w, 'a, H, S>
{
    /// Original zero-based actual complete header-parent index.
    #[must_use]
    pub const fn header_index(&self) -> usize {
        self.header_index
    }
    /// Exact original requiring garment/request/header parent,not an equal-data replacement.
    #[must_use]
    pub const fn header(&self) -> &'u DraftingProgramGarmentProfileNumericHeader<'w, 'a, H, S> {
        self.header
    }
    /// Actual original parameter metadata,Profile canonical member/kind/state and exact parent.
    #[must_use]
    pub const fn source(&self) -> &BoundDraftingProfileNumericSource<'u, 'w, H, S> {
        &self.source
    }
}
impl<
        H: DesignMetadataHost,
        S: DraftingProfileNumericProvider<H>
            + DraftingProfileLogicalProvider<H>
            + DraftingRuleProvider<H>
            + ?Sized,
    > fmt::Debug for DraftingProgramGarmentProfileNumericParameter<'_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramGarmentProfileNumericParameter")
            .field("header_index", &self.header_index)
            .finish_non_exhaustive()
    }
}
/// Complete actual numeric Profile garment parameters borrowing stable original header storage
/// and the same unique Root/cursor through disjoint fields. No self-reference or body/value clone.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<I:?Sized,H:DesignMetadataHost,S:DraftingProfileNumericProvider<H>+DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:&mut BoundDraftingProgramGarmentProfileNumericParameters<'_,'_,'_,'_,'_,'_,'_,'_,I,H,S>) {p.records.clear();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn duplicate<I:?Sized,H:DesignMetadataHost,S:DraftingProfileNumericProvider<H>+DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:BoundDraftingProgramGarmentProfileNumericParameters<'_,'_,'_,'_,'_,'_,'_,'_,I,H,S>) {let _=p.clone();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn escape<'u,'t,'z,'q,'h,'v,'w,'a,I:?Sized,H:DesignMetadataHost,S:DraftingProfileNumericProvider<H>+DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:BoundDraftingProgramGarmentProfileNumericParameters<'u,'t,'z,'q,'h,'v,'w,'a,I,H,S>)->BoundDraftingProgramGarmentProfileNumericParameters<'static,'t,'z,'q,'h,'v,'w,'a,I,H,S> {p}
/// ```
pub struct BoundDraftingProgramGarmentProfileNumericParameters<
    'u,
    't,
    'z,
    'q,
    'h,
    'v,
    'w,
    'a,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingProfileNumericProvider<H>
        + DraftingProfileLogicalProvider<H>
        + DraftingRuleProvider<H>
        + ?Sized,
> {
    logical: &'u mut BoundDraftingProgramGarmentProfileLogicalParameters<
        't,
        'z,
        'q,
        'h,
        'v,
        'w,
        'a,
        I,
        H,
        S,
    >,
    headers: &'u [DraftingProgramGarmentProfileNumericHeader<'w, 'a, H, S>],
    demand: Option<DesignResourceDemand>,
    records: Vec<DraftingProgramGarmentProfileNumericParameter<'u, 'w, 'a, H, S>>,
}
impl<
        'u,
        't,
        'z,
        'q,
        'h,
        'v,
        'w,
        'a,
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileNumericProvider<H>
            + DraftingProfileLogicalProvider<H>
            + DraftingRuleProvider<H>
            + ?Sized,
    > BoundDraftingProgramGarmentProfileNumericParameters<'u, 't, 'z, 'q, 'h, 'v, 'w, 'a, I, H, S>
{
    /// Complete original Root garment/member/supplier/operation owners and same unique cursor.
    #[must_use]
    pub const fn logical(
        &self,
    ) -> &BoundDraftingProgramGarmentProfileLogicalParameters<'t, 'z, 'q, 'h, 'v, 'w, 'a, I, H, S>
    {
        self.logical
    }
    /// Complete stable original numeric Profile header parents and original request fields.
    #[must_use]
    pub const fn headers(&self) -> &'u [DraftingProgramGarmentProfileNumericHeader<'w, 'a, H, S>] {
        self.headers
    }
    /// Fresh full aggregate/proof demand before nonempty parameter work;None for N0.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.demand
    }
    /// Every original numeric Profile garment parameter once,in actual header/garment order.
    #[must_use]
    pub fn records(&self) -> &[DraftingProgramGarmentProfileNumericParameter<'u, 'w, 'a, H, S>] {
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
        S: DraftingProfileNumericProvider<H>
            + DraftingProfileLogicalProvider<H>
            + DraftingRuleProvider<H>
            + ?Sized,
    > fmt::Debug
    for BoundDraftingProgramGarmentProfileNumericParameters<'_, '_, '_, '_, '_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProgramGarmentProfileNumericParameters")
            .field("records", &self.records.len())
            .finish_non_exhaustive()
    }
}
/// Complete actual original parameters or original borrowed Root/headers/cursor and first cause.
pub type DraftingProgramGarmentProfileNumericParametersResult<
    'u,
    't,
    'z,
    'q,
    'h,
    'v,
    'w,
    'a,
    I,
    H,
    S,
> = Result<
    BoundDraftingProgramGarmentProfileNumericParameters<'u, 't, 'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
    DraftingProgramGarmentProfileNumericParameterError<'u, 't, 'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
>;
impl<
        't,
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
            + DraftingProfileNumericProvider<H>
            + DraftingProfileLogicalProvider<H>
            + DraftingRuleProvider<H>
            + ?Sized,
    >
    BoundDraftingProgramGarmentProfileNumericHeaders<
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
    >
{
    /// Borrow actual stable headers and the same unique Root/cursor through disjoint fields.
    /// Reserve N+1 fresh whole/parameter passes for N>0 before complete aggregate/proof charging,
    /// complete N-record reservation and each existing parent-bound bind_numeric_parameter query.
    /// Each child freshly charges Root/provider/request before exact original parameter/header/
    /// canonical/kind association. Actual domains/states and external Derived scope stay original;
    /// no Root position,selected value,source body or complete cohort is inferred or cloned.
    /// N0 adds no work,walk,allocation,query,reservation or demand claim.
    /// # Errors
    /// Complete arithmetic,atomic cursor shortage,fresh aggregate resources,allocator,then first
    /// original lending parameter/member/kind cause. Original complete Root/cursor and stable
    /// headers remain inspectable;private successful parameter prefixes drop. Dropping either
    /// result releases its borrow without resetting prior or newly reserved child work.
    pub fn bind_garment_profile_numeric_parameters<'u>(
        &'u mut self,
    ) -> DraftingProgramGarmentProfileNumericParametersResult<
        'u,
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
        use DraftingProgramGarmentProfileNumericParameterRefusal as R;
        let Self {
            logical,
            records: headers,
            demand: _,
        } = self;
        let headers: &'u [DraftingProgramGarmentProfileNumericHeader<'w, 'a, H, S>] = headers;
        let count = headers.len();
        let passes = if count == 0 {
            Some(0)
        } else {
            count.checked_add(1)
        };
        let Some(passes) = passes else {
            return Err(DraftingProgramGarmentProfileNumericParameterError::new(
                logical,
                headers,
                None,
                R::PopulationOverflow,
            ));
        };
        if let Err(cause) = logical
            .root
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
            return Err(DraftingProgramGarmentProfileNumericParameterError::new(
                logical,
                headers,
                None,
                R::Work(cause),
            ));
        }
        if count == 0 {
            return Ok(BoundDraftingProgramGarmentProfileNumericParameters {
                logical,
                headers,
                demand: None,
                records: Vec::new(),
            });
        }
        let content = logical
            .root
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
        let charged = logical
            .root
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
                return Err(DraftingProgramGarmentProfileNumericParameterError::new(
                    logical,
                    headers,
                    None,
                    R::Resources(cause),
                ))
            }
        };
        if let Err(cause) = charged {
            return Err(DraftingProgramGarmentProfileNumericParameterError::new(
                logical,
                headers,
                None,
                R::Resources(cause),
            ));
        }
        let mut records = Vec::new();
        if let Err(cause) = records.try_reserve_exact(count) {
            return Err(DraftingProgramGarmentProfileNumericParameterError::new(
                logical,
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
            let source = match header.source().bind_numeric_parameter(
                request.parameter(),
                request.kind(),
                content,
            ) {
                Ok(s) => s,
                Err(cause) => {
                    return Err(DraftingProgramGarmentProfileNumericParameterError::new(
                        logical,
                        headers,
                        Some(demand),
                        R::Parameter {
                            header_index,
                            cause,
                        },
                    ))
                }
            };
            records.push(DraftingProgramGarmentProfileNumericParameter {
                header_index,
                header,
                source,
            });
        }
        Ok(BoundDraftingProgramGarmentProfileNumericParameters {
            logical,
            headers,
            demand: Some(demand),
            records,
        })
    }
}

impl<
        'u,
        't,
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
            + DraftingProfileNumericProvider<H>
            + DraftingProfileLogicalProvider<H>
            + DraftingRuleProvider<H>
            + crate::construction::DraftingComponentProvider<H>
            + ?Sized,
    >
    BoundDraftingProgramGarmentProfileNumericParameters<
        'u,
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
    >
{
    /// Consume the complete original garment input proof and reserve all N+1 Pocket passes before
    /// fresh aggregate charging,complete N-record reservation and each existing current-member
    /// factory. Every original served/component field keeps its actual Pocket/local position.
    /// N0 performs no walk,query,allocation or reservation. No source/geometry/value query runs.
    /// The generic parent type is this exact consumed proof;no body clone or self-reference occurs.
    /// # Errors
    /// Complete arithmetic,atomic work shortage,fresh aggregate resources,allocator,then first
    /// original child content/matching/current-copy/Piece cause. Both outcomes retain the entire
    /// prior proof and unique cursor;private accepted prefixes drop and reserved work is not reset.
    pub fn bind_pocket_component_work(
        self,
    ) -> crate::construction::DraftingProgramPocketWorkResult<
        's,
        'w,
        'a,
        Self,
        DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        H,
        S,
    > {
        let parent = self;
        let cursor = &mut parent
            .logical
            .root
            .logical
            .members
            .owner
            .inputs
            .root
            .logical
            .members
            .owner
            .selections;
        let inputs = cursor.inputs();
        let outcome = crate::construction::program::dependencies::pocket_work::prepare(
            cursor,
            inputs.garment(),
            |sink| inputs.charge_resources(sink),
        );
        outcome.finish(parent)
    }
}

// The authentic parent/input specialization stays readable at the public borrowing factory.
// Its target is the public generic source result;callers retain the same inferred original type.
type PocketSourcesResult<
    'x,
    'u,
    't,
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
    H,
    S,
> = crate::construction::DraftingProgramPocketSourcesResult<
    'x,
    's,
    'w,
    'a,
    BoundDraftingProgramGarmentProfileNumericParameters<
        'u,
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
    >,
    DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    H,
    S,
>;

impl<
        'u,
        't,
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
            + DraftingProfileNumericProvider<H>
            + DraftingProfileLogicalProvider<H>
            + DraftingRuleProvider<H>
            + crate::construction::DraftingComponentProvider<H>
            + ?Sized,
    >
    crate::construction::BoundDraftingProgramPocketWork<
        's,
        'a,
        BoundDraftingProgramGarmentProfileNumericParameters<
            'u,
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
        >,
        H,
        S,
    >
{
    /// Borrow actual stable current-member records and the same unique input/cursor through
    /// disjoint fields. Reserve N+1 fresh whole/source passes before aggregate charging,result
    /// reservation or original bind_source factories. N0 adds no walk,query,allocation or demand.
    /// Source loans preserve their actual work parents,frame/snapshot and captured recipe creator
    /// positions;no Root consumer position,body clone,selected value or physical proof is invented.
    /// # Errors
    /// First complete arithmetic,atomic cursor shortage,fresh resources,allocator or original
    /// work-bound source cause. All original work/input owners remain;private prefixes drop and
    /// dropping either result releases the borrow without refunding reserved work.
    pub fn bind_pocket_sources<'x>(
        &'x mut self,
    ) -> PocketSourcesResult<
        'x,
        'u,
        't,
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
        H,
        S,
    > {
        let (parent, work) = self.parts();
        let cursor = &mut parent
            .logical
            .root
            .logical
            .members
            .owner
            .inputs
            .root
            .logical
            .members
            .owner
            .selections;
        let inputs = cursor.inputs();
        let outcome = crate::construction::program::dependencies::pocket_work::sources::prepare(
            cursor,
            work,
            |sink| inputs.charge_resources(sink),
        );
        outcome.finish(parent, work)
    }
}

impl<
        'x,
        'u,
        't,
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
            + DraftingProfileNumericProvider<H>
            + DraftingProfileLogicalProvider<H>
            + DraftingRuleProvider<H>
            + crate::construction::DraftingComponentProvider<H>
            + ?Sized,
    >
    crate::construction::BoundDraftingProgramPocketSources<
        'x,
        's,
        'a,
        BoundDraftingProgramGarmentProfileNumericParameters<
            'u,
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
        >,
        H,
        S,
    >
{
    /// Consume the complete original Pocket source proof and reserve N+1 Closure relation passes
    /// before fresh aggregate charging,result reservation or existing original Buttonhole factories.
    /// N0 adds no walk,lookup,allocation or reservation. Original garment locations and canonical
    /// source/member loans remain unchanged;no physical length,placement or mixed order is proved.
    /// # Errors
    /// Whole arithmetic,atomic work shortage,complete resources,allocator,then first original
    /// Closure/member/button-origin cause. Both outcomes retain the prior proof and same cursor;
    /// accepted private prefixes drop and recovering the parent refunds no reserved work.
    pub fn bind_closure_buttonholes(
        mut self,
    ) -> crate::construction::DraftingProgramClosureButtonholeResult<
        's,
        'w,
        'a,
        Self,
        DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        H,
        S,
    > {
        let cursor = &mut self
            .parent_mut()
            .logical
            .root
            .logical
            .members
            .owner
            .inputs
            .root
            .logical
            .members
            .owner
            .selections;
        let inputs = cursor.inputs();
        let outcome = crate::construction::program::dependencies::closure_buttonholes::prepare(
            cursor,
            inputs.garment(),
            |sink| inputs.charge_resources(sink),
        );
        outcome.finish(self)
    }
}
