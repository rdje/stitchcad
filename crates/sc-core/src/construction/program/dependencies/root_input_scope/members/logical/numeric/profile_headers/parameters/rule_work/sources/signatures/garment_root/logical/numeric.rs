//! Complete original Root garment numeric Derived relations under their actual parameter targets.
use super::{
    super::DraftingProgramGarmentRootInputSource, BoundDraftingProgramGarmentRootLogicalSuppliers,
};
use crate::{
    construction::{
        DraftingGarmentProvider, DraftingInitialInputProvider, DraftingProfileLogicalProvider,
        DraftingProgramRootNumericSupplier, DraftingProgramSemanticInputs, DraftingRuleProvider,
    },
    design::{
        DesignMetadataHost, DesignReferenceHost, DesignResourceDemand, DesignResourceMeter,
        DesignResourceSource,
    },
    value::{CanonicalNumericInputMember, LengthState, ScalarInputState},
};
use core::fmt;
mod error;
pub use error::{
    DraftingProgramGarmentRootNumericSupplierError,
    DraftingProgramGarmentRootNumericSupplierRefusal,
};
/// Complete numeric Derived garment-input class retaining original members/logical suppliers and
/// the same unique cursor. Inline fields acquire no numeric formula or mixed supplier position.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:&mut BoundDraftingProgramGarmentRootNumericSuppliers<'_,'_,'_,'_,'_,'_,I,H,S>) {p.records.clear();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn duplicate<I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:BoundDraftingProgramGarmentRootNumericSuppliers<'_,'_,'_,'_,'_,'_,I,H,S>) {let _=p.clone();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn escape<'z,'q,'h,'v,'w,'a,I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:BoundDraftingProgramGarmentRootNumericSuppliers<'z,'q,'h,'v,'w,'a,I,H,S>)->BoundDraftingProgramGarmentRootNumericSuppliers<'static,'q,'h,'v,'w,'a,I,H,S> {p}
/// ```
pub struct BoundDraftingProgramGarmentRootNumericSuppliers<
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
    logical: BoundDraftingProgramGarmentRootLogicalSuppliers<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
    demand: Option<DesignResourceDemand>,
    records: Vec<DraftingProgramRootNumericSupplier<'w>>,
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
    > BoundDraftingProgramGarmentRootNumericSuppliers<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>
{
    /// Complete original garment members/logical suppliers/operation proofs and unique cursor.
    #[must_use]
    pub const fn logical(
        &self,
    ) -> &BoundDraftingProgramGarmentRootLogicalSuppliers<'z, 'q, 'h, 'v, 'w, 'a, I, H, S> {
        &self.logical
    }
    /// Fresh complete aggregate/proof demand for nonempty numeric work;None for N0.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.demand
    }
    /// Original garment numeric Derived consumers,without deduplicating targets or sources.
    /// member_index refers to logical().members().records(),not operation-input member storage.
    #[must_use]
    pub fn records(&self) -> &[DraftingProgramRootNumericSupplier<'w>] {
        &self.records
    }
    /// N complete whole/numeric/result envelopes;zero for N0.
    #[must_use]
    pub fn child_pass_count(&self) -> usize {
        self.records.len()
    }
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > fmt::Debug
    for BoundDraftingProgramGarmentRootNumericSuppliers<'_, '_, '_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProgramGarmentRootNumericSuppliers")
            .field("records", &self.records.len())
            .finish_non_exhaustive()
    }
}
/// Complete original numeric relations or original complete logical/member owners and first cause.
pub type DraftingProgramGarmentRootNumericSuppliersResult<'z, 'q, 'h, 'v, 'w, 'a, I, H, S> = Result<
    BoundDraftingProgramGarmentRootNumericSuppliers<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
    DraftingProgramGarmentRootNumericSupplierError<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
>;
fn derived<'a, H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized>(
    source: &DraftingProgramGarmentRootInputSource<'_, 'a, H, S>,
) -> Option<CanonicalNumericInputMember<'a>> {
    let DraftingProgramGarmentRootInputSource::FormulaLength { source, .. } = source else {
        return None;
    };
    let member = source.member();
    let derived = match member {
        CanonicalNumericInputMember::Length { declaration, .. } => {
            matches!(declaration.state(), LengthState::Derived { .. })
        }
        CanonicalNumericInputMember::Scalar { declaration, .. } => {
            matches!(declaration.state(), ScalarInputState::Derived { .. })
        }
    };
    derived.then_some(member)
}
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
    BoundDraftingProgramGarmentRootLogicalSuppliers<
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
    /// Reserve all N actual numeric Derived whole/result envelopes before fresh full aggregate/
    /// proof charging,fallible N-record reservation or original canonical/source joins. Match each
    /// actual canonical declaration by pointer to its original FormulaDerivedInput,checked source
    /// and exact already-held acyclic owner. Each original scan/source lookup fits (E+8)^2 and one
    /// complete walk fits N whole-content envelopes. Prior demand grants no credit.
    /// Existing exact prerequisite owners already certify source existence/result kind/acyclicity.
    /// Inline fields acquire no formula;non-Derived fields stay unchanged. No metadata/value clone,
    /// query,semantic/mixed supplier position or whole-cohort acceptance is introduced.
    /// N0 adds no walk,work,allocation,lookup,reservation or new demand claim.
    /// # Errors
    /// Complete arithmetic,atomic cursor shortage,fresh complete aggregate resources,then complete
    /// allocator cause. Original owners/cursor survive and private successful prefixes drop.
    pub fn bind_garment_root_numeric_suppliers(
        self,
    ) -> DraftingProgramGarmentRootNumericSuppliersResult<
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
        use DraftingProgramGarmentRootNumericSupplierRefusal as R;
        let count = self.members.records.iter().try_fold(0usize, |n, member| {
            n.checked_add(usize::from(derived(member.source()).is_some()))
        });
        let Some(count) = count else {
            return Err(DraftingProgramGarmentRootNumericSupplierError::new(
                self,
                None,
                None,
                R::PopulationOverflow,
            ));
        };
        if let Err(cause) = self
            .members
            .owner
            .inputs
            .root
            .logical
            .members
            .owner
            .selections
            .reserve_child_passes(count)
        {
            return Err(DraftingProgramGarmentRootNumericSupplierError::new(
                self,
                Some(count),
                None,
                R::Work(cause),
            ));
        }
        if count == 0 {
            return Ok(BoundDraftingProgramGarmentRootNumericSuppliers {
                logical: self,
                demand: None,
                records: Vec::new(),
            });
        }
        let inputs = self
            .members
            .owner
            .inputs
            .root
            .logical
            .members
            .owner
            .inputs();
        let content = self
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
        let charged = inputs.charge_resources(&mut meter);
        let demand = match meter.finish() {
            Ok(d) => d,
            Err(cause) => {
                return Err(DraftingProgramGarmentRootNumericSupplierError::new(
                    self,
                    Some(count),
                    None,
                    R::Resources(cause),
                ))
            }
        };
        if let Err(cause) = charged {
            return Err(DraftingProgramGarmentRootNumericSupplierError::new(
                self,
                Some(count),
                None,
                R::Resources(cause),
            ));
        }
        let mut records = Vec::new();
        if let Err(cause) = records.try_reserve_exact(count) {
            return Err(DraftingProgramGarmentRootNumericSupplierError::new(
                self,
                Some(count),
                Some(demand),
                R::Reservation {
                    records: count,
                    cause,
                },
            ));
        }
        let dependencies = inputs.geometry().work().inputs().inputs();
        let sources = dependencies.sources().proof();
        let source_inputs = sources.normalized().inputs();
        for (member_index, member) in self.members.records.iter().enumerate() {
            let Some(actual) = derived(member.source()) else {
                continue;
            };
            let input=source_inputs.derived_inputs().find(|input|match actual {CanonicalNumericInputMember::Length {declaration,..}=>input.length().is_some_and(|original|core::ptr::eq(original,declaration)),CanonicalNumericInputMember::Scalar {declaration,..}=>input.scalar().is_some_and(|original|core::ptr::eq(original,declaration))}).unwrap_or_else(||unreachable!("same actual Root garment member and complete immutable Derived/source owner"));
            let source = sources
                .source(input.source().id())
                .filter(|source| core::ptr::eq(source.normalized().source(), input.source()))
                .unwrap_or_else(|| {
                    unreachable!(
                        "complete original checked source proof retains this actual Derived source"
                    )
                });
            records.push(DraftingProgramRootNumericSupplier {
                member_index,
                input,
                source,
                graph: dependencies.graph(),
            });
        }
        Ok(BoundDraftingProgramGarmentRootNumericSuppliers {
            logical: self,
            demand: Some(demand),
            records,
        })
    }
}
