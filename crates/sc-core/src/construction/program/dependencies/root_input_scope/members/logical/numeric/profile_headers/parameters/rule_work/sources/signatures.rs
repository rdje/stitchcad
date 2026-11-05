//! Complete local rule signatures and actual Root outputs under stable original source parents.
use super::{
    BoundDraftingProgramProfileParameters, BoundDraftingProgramRuleSources,
    DraftingProgramRuleSource,
};
use crate::{
    construction::{
        BoundDraftingRuleSignature, DraftingGarmentProvider, DraftingInitialInputProvider,
        DraftingProfileLogicalProvider, DraftingProgramSemanticInputs, DraftingRuleProvider,
    },
    design::{
        DesignMetadataHost, DesignReferenceHost, DesignResourceDemand, DesignResourceMeter,
        DesignResourceSource,
    },
};
use core::fmt;
mod error;
mod garment_root;
pub use error::{DraftingProgramRuleSignatureError, DraftingProgramRuleSignatureRefusal};
pub use garment_root::{
    BoundDraftingProgramGarmentProfileLogicalHeaders, BoundDraftingProgramGarmentRootInputMembers,
    BoundDraftingProgramGarmentRootLogicalSuppliers,
    BoundDraftingProgramGarmentRootNumericSuppliers, DraftingProgramGarmentProfileLogicalHeader,
    DraftingProgramGarmentProfileLogicalHeaderError,
    DraftingProgramGarmentProfileLogicalHeaderRefusal,
    DraftingProgramGarmentProfileLogicalHeadersResult, DraftingProgramGarmentProfileLogicalRequest,
    DraftingProgramGarmentRootInputMember, DraftingProgramGarmentRootInputMemberError,
    DraftingProgramGarmentRootInputMemberRefusal, DraftingProgramGarmentRootInputMembersResult,
    DraftingProgramGarmentRootInputSource, DraftingProgramGarmentRootLogicalSupplier,
    DraftingProgramGarmentRootLogicalSupplierError,
    DraftingProgramGarmentRootLogicalSupplierRefusal,
    DraftingProgramGarmentRootLogicalSuppliersResult,
    DraftingProgramGarmentRootNumericSupplierError,
    DraftingProgramGarmentRootNumericSupplierRefusal,
    DraftingProgramGarmentRootNumericSuppliersResult,
};
/// One original source parent and its complete named signature/actual canonical output relation.
pub struct DraftingProgramRuleSignature<
    'z,
    'q,
    'w,
    'a,
    H: DesignMetadataHost,
    S: DraftingRuleProvider<H> + ?Sized,
> {
    source_index: usize,
    source: &'z DraftingProgramRuleSource<'q, 'w, 'a, H, S>,
    signature: BoundDraftingRuleSignature<'z, 'q, 'w, H, S>,
}
impl<'z, 'q, 'w, 'a, H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized>
    DraftingProgramRuleSignature<'z, 'q, 'w, 'a, H, S>
{
    /// Original zero-based complete source-parent index.
    #[must_use]
    pub const fn source_index(&self) -> usize {
        self.source_index
    }
    /// Exact original source/work/request/consumer parent.
    #[must_use]
    pub const fn source(&self) -> &'z DraftingProgramRuleSource<'q, 'w, 'a, H, S> {
        self.source
    }
    /// Original named matches and actual Root logical Derived output,or no authored policy output.
    #[must_use]
    pub const fn signature(&self) -> &BoundDraftingRuleSignature<'z, 'q, 'w, H, S> {
        &self.signature
    }
}
impl<H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramRuleSignature<'_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramRuleSignature")
            .field("source_index", &self.source_index)
            .finish_non_exhaustive()
    }
}
/// Complete original local signatures/outputs with complete input proofs and one cursor. Original
/// source storage stays caller-owned;no self-reference,body clone,value or whole cohort acceptance.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:&mut BoundDraftingProgramRuleSignatures<'_,'_,'_,'_,'_,'_,I,H,S>) {p.records.clear();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn duplicate<I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:BoundDraftingProgramRuleSignatures<'_,'_,'_,'_,'_,'_,I,H,S>) {let _=p.clone();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn escape<'z,'q,'h,'v,'w,'a,I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:BoundDraftingProgramRuleSignatures<'z,'q,'h,'v,'w,'a,I,H,S>)->BoundDraftingProgramRuleSignatures<'static,'q,'h,'v,'w,'a,I,H,S> {p}
/// ```
pub struct BoundDraftingProgramRuleSignatures<
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
    inputs: &'z mut BoundDraftingProgramProfileParameters<'h, 'v, 'w, 'a, I, H, S>,
    sources: &'z [DraftingProgramRuleSource<'q, 'w, 'a, H, S>],
    demand: Option<DesignResourceDemand>,
    records: Vec<DraftingProgramRuleSignature<'z, 'q, 'w, 'a, H, S>>,
}
impl<
        'z,
        'q,
        'h,
        'v,
        'w,
        'a,
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > BoundDraftingProgramRuleSignatures<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>
{
    /// Complete original Root/Profile operation inputs and the same unique finite cursor.
    #[must_use]
    pub const fn inputs(&self) -> &BoundDraftingProgramProfileParameters<'h, 'v, 'w, 'a, I, H, S> {
        self.inputs
    }
    /// Complete stable actual source/work/request parents,in original mixed order.
    #[must_use]
    pub const fn sources(&self) -> &'z [DraftingProgramRuleSource<'q, 'w, 'a, H, S>] {
        self.sources
    }
    /// Every original signature/output relation once,without an accepted prefix on refusal.
    #[must_use]
    pub fn records(&self) -> &[DraftingProgramRuleSignature<'z, 'q, 'w, 'a, H, S>] {
        &self.records
    }
    /// Fresh complete aggregate/proof demand before nonempty signature work;None for N0.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.demand
    }
    /// Zero for N0,otherwise N+1 reserved whole/signature passes.
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
    > fmt::Debug for BoundDraftingProgramRuleSignatures<'_, '_, '_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProgramRuleSignatures")
            .field("records", &self.records.len())
            .finish_non_exhaustive()
    }
}
/// Complete original signatures/outputs or all original borrowed parents/cursor and first cause.
pub type DraftingProgramRuleSignaturesResult<'z, 'q, 'h, 'v, 'w, 'a, I, H, S> = Result<
    BoundDraftingProgramRuleSignatures<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
    DraftingProgramRuleSignatureError<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
>;
impl<
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
    BoundDraftingProgramRuleSources<
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
    /// Borrow stable actual source parents and unique complete inputs/cursor through disjoint
    /// fields. For N>0,reserve N+1 whole/signature passes before fresh full aggregate/proof content,
    /// full N-record reservation and original local signature/output joins. Each existing child
    /// bind_signature freshly admits work before original roles/kinds and actual Root Derived output.
    /// No Profile/rule/currentness/value query,body clone or physical result is introduced.
    /// N0 adds no work,walk,allocation,query or new demand claim.
    /// # Errors
    /// Complete pass arithmetic,atomic cursor shortage,fresh aggregate resources,allocator,then
    /// first original child signature/output cause with its original source parent. Successful
    /// private prefixes drop;complete actual input/source/cursor loans remain inspectable. Dropping
    /// a result releases its borrow without resetting prior or newly reserved work.
    pub fn bind_rule_signatures<'z>(
        &'z mut self,
    ) -> DraftingProgramRuleSignaturesResult<
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
        use DraftingProgramRuleSignatureRefusal as R;
        let Self {
            inputs,
            records: sources,
            works: _,
            demand: _,
            passes: _,
        } = self;
        let sources: &'z [DraftingProgramRuleSource<'q, 'w, 'a, H, S>] = sources;
        let count = sources.len();
        let passes = if count == 0 {
            Some(0)
        } else {
            count.checked_add(1)
        };
        let Some(passes) = passes else {
            return Err(DraftingProgramRuleSignatureError::new(
                inputs,
                sources,
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
            return Err(DraftingProgramRuleSignatureError::new(
                inputs,
                sources,
                None,
                R::Work(cause),
            ));
        }
        if count == 0 {
            return Ok(BoundDraftingProgramRuleSignatures {
                inputs,
                sources,
                demand: None,
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
                return Err(DraftingProgramRuleSignatureError::new(
                    inputs,
                    sources,
                    None,
                    R::Resources(cause),
                ))
            }
        };
        if let Err(cause) = charged {
            return Err(DraftingProgramRuleSignatureError::new(
                inputs,
                sources,
                None,
                R::Resources(cause),
            ));
        }
        let mut records = Vec::new();
        if let Err(cause) = records.try_reserve_exact(count) {
            return Err(DraftingProgramRuleSignatureError::new(
                inputs,
                sources,
                Some(demand),
                R::Reservation {
                    records: count,
                    cause,
                },
            ));
        }
        for (source_index, source) in sources.iter().enumerate() {
            let signature = match source.source().bind_signature() {
                Ok(s) => s,
                Err(cause) => {
                    return Err(DraftingProgramRuleSignatureError::new(
                        inputs,
                        sources,
                        Some(demand),
                        R::Signature {
                            source_index,
                            cause,
                        },
                    ))
                }
            };
            records.push(DraftingProgramRuleSignature {
                source_index,
                source,
                signature,
            });
        }
        Ok(BoundDraftingProgramRuleSignatures {
            inputs,
            sources,
            demand: Some(demand),
            records,
        })
    }
}
