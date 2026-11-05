//! Complete original Root garment logical suppliers,without fabricated Root consumer order.
use super::{BoundDraftingProgramGarmentRootInputMembers, DraftingProgramGarmentRootInputSource};
use crate::{
    construction::{
        DraftingDesignOperationSource, DraftingGarmentProvider, DraftingInitialInputProvider,
        DraftingProfileLogicalProvider, DraftingProgramSemanticInputs, DraftingProgramStepKind,
        DraftingRuleProvider,
    },
    design::{
        DesignMetadataHost, DesignReferenceHost, DesignResourceDemand, DesignResourceMeter,
        DesignResourceSource,
    },
    ontology::EntityId,
    recipe::{SemanticOperation, SemanticOperationKind},
    value::{LogicalInputState, LogicalInputTarget},
};
use core::fmt;
mod error;
pub use error::{
    DraftingProgramGarmentRootLogicalSupplierError,
    DraftingProgramGarmentRootLogicalSupplierRefusal,
};
/// One original Root garment logical Derived member and its actual original supplier/output.
pub struct DraftingProgramGarmentRootLogicalSupplier<
    'v,
    'a,
    H: DesignMetadataHost,
    S: DraftingRuleProvider<H> + ?Sized,
> {
    member_index: usize,
    source: DraftingDesignOperationSource<'v, 'a, H, S>,
    output: &'a LogicalInputTarget,
    ordinal: usize,
    position: usize,
    signature_index: usize,
}
impl<'v, 'a, H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized>
    DraftingProgramGarmentRootLogicalSupplier<'v, 'a, H, S>
{
    /// Original consumer record index in the complete Root garment member class.
    #[must_use]
    pub const fn member_index(&self) -> usize {
        self.member_index
    }
    /// Actual original Root LogicalDerivation source,not an external Profile operation.
    #[must_use]
    pub const fn source(&self) -> DraftingDesignOperationSource<'v, 'a, H, S> {
        self.source
    }
    /// Original supplier output field,matched to the actual canonical member ID/kind.
    #[must_use]
    pub const fn output(&self) -> &'a LogicalInputTarget {
        self.output
    }
    /// Genuine original one-based semantic supplier ordinal.
    #[must_use]
    pub const fn ordinal(&self) -> usize {
        self.ordinal
    }
    /// Genuine original mixed supplier position. A Root consumer has no position to compare.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.position
    }
    /// Actual supplier's original complete signature record index in members().owner().records().
    #[must_use]
    pub const fn signature_index(&self) -> usize {
        self.signature_index
    }
}
impl<H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramGarmentRootLogicalSupplier<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramGarmentRootLogicalSupplier")
            .field("member_index", &self.member_index)
            .field("ordinal", &self.ordinal)
            .field("position", &self.position)
            .finish_non_exhaustive()
    }
}
/// Complete logical Derived supplier class with original garment members and operation proofs.
/// Root locations remain unordered;numeric/Profile/physical and whole cohort acceptance stay separate.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:&mut BoundDraftingProgramGarmentRootLogicalSuppliers<'_,'_,'_,'_,'_,'_,I,H,S>) {p.records.clear();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn duplicate<I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:BoundDraftingProgramGarmentRootLogicalSuppliers<'_,'_,'_,'_,'_,'_,I,H,S>) {let _=p.clone();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn escape<'z,'q,'h,'v,'w,'a,I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:BoundDraftingProgramGarmentRootLogicalSuppliers<'z,'q,'h,'v,'w,'a,I,H,S>)->BoundDraftingProgramGarmentRootLogicalSuppliers<'static,'q,'h,'v,'w,'a,I,H,S> {p}
/// ```
pub struct BoundDraftingProgramGarmentRootLogicalSuppliers<
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
    members: BoundDraftingProgramGarmentRootInputMembers<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
    demand: Option<DesignResourceDemand>,
    records: Vec<DraftingProgramGarmentRootLogicalSupplier<'v, 'a, H, S>>,
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
    > BoundDraftingProgramGarmentRootLogicalSuppliers<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>
{
    /// Complete original Root members/operation signatures and the same unique finite cursor.
    #[must_use]
    pub const fn members(
        &self,
    ) -> &BoundDraftingProgramGarmentRootInputMembers<'z, 'q, 'h, 'v, 'w, 'a, I, H, S> {
        &self.members
    }
    /// Fresh complete aggregate/proof demand for nonempty supplier work;None for N0.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.demand
    }
    /// Every original logical Derived consumer once,in original member order without deduplication.
    #[must_use]
    pub fn records(&self) -> &[DraftingProgramGarmentRootLogicalSupplier<'v, 'a, H, S>] {
        &self.records
    }
    /// N complete whole/member/supplier-result envelopes;zero for N0.
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
    for BoundDraftingProgramGarmentRootLogicalSuppliers<'_, '_, '_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProgramGarmentRootLogicalSuppliers")
            .field("records", &self.records.len())
            .finish_non_exhaustive()
    }
}
/// Complete original suppliers or exact original members/cursor and first typed refusal.
pub type DraftingProgramGarmentRootLogicalSuppliersResult<'z, 'q, 'h, 'v, 'w, 'a, I, H, S> = Result<
    BoundDraftingProgramGarmentRootLogicalSuppliers<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
    DraftingProgramGarmentRootLogicalSupplierError<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
>;
fn derived<H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized>(
    source: &DraftingProgramGarmentRootInputSource<'_, '_, H, S>,
) -> Option<(EntityId, LogicalInputTarget)> {
    let DraftingProgramGarmentRootInputSource::Logical(source) = source else {
        return None;
    };
    let declaration = source.declaration().definition();
    let LogicalInputState::Derived { operation } = declaration.state else {
        return None;
    };
    Some((
        operation,
        LogicalInputTarget {
            declaration: declaration.id,
            kind: declaration.kind,
        },
    ))
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
    BoundDraftingProgramGarmentRootInputMembers<
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
    /// Count every original logical Derived member and reserve N complete fresh whole/supplier/
    /// result envelopes before full aggregate/proof charging,complete N-record reservation or
    /// supplier lookup. Each original Root supplier must be LogicalDerivation and output the exact
    /// member ID/kind;retain its genuine original ordinal/mixed position and complete signature
    /// index. Root consumers have no mixed position,so no synthetic consumer order is imposed.
    /// One full walk plus all original recipe/mixed/signature joins fit N*(whole-demand,(E+8)^2).
    /// No Profile/rule/currentness/value query or domain/body/evidence acceptance occurs.
    /// N0 adds no walk,work,allocation,lookup or demand claim.
    /// # Errors
    /// Complete arithmetic,atomic cursor shortage,fresh aggregate resources,allocator,then first
    /// original supplier family/missing or output mismatch. Private successful prefixes drop;
    /// original Root members/cursor and actual source/output witnesses remain inspectable.
    pub fn bind_garment_root_logical_suppliers(
        self,
    ) -> DraftingProgramGarmentRootLogicalSuppliersResult<
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
        use DraftingProgramGarmentRootLogicalSupplierRefusal as R;
        let count = self.records.iter().try_fold(0usize, |n, member| {
            n.checked_add(usize::from(derived(member.source()).is_some()))
        });
        let Some(count) = count else {
            return Err(DraftingProgramGarmentRootLogicalSupplierError::new(
                self,
                None,
                None,
                R::PopulationOverflow,
            ));
        };
        if let Err(cause) = self
            .owner
            .inputs
            .root
            .logical
            .members
            .owner
            .selections
            .reserve_child_passes(count)
        {
            return Err(DraftingProgramGarmentRootLogicalSupplierError::new(
                self,
                Some(count),
                None,
                R::Work(cause),
            ));
        }
        if count == 0 {
            return Ok(BoundDraftingProgramGarmentRootLogicalSuppliers {
                members: self,
                demand: None,
                records: Vec::new(),
            });
        }
        let inputs = self.owner.inputs.root.logical.members.owner.inputs();
        let content = self
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
                return Err(DraftingProgramGarmentRootLogicalSupplierError::new(
                    self,
                    Some(count),
                    None,
                    R::Resources(cause),
                ))
            }
        };
        if let Err(cause) = charged {
            return Err(DraftingProgramGarmentRootLogicalSupplierError::new(
                self,
                Some(count),
                None,
                R::Resources(cause),
            ));
        }
        let mut records = Vec::new();
        if let Err(cause) = records.try_reserve_exact(count) {
            return Err(DraftingProgramGarmentRootLogicalSupplierError::new(
                self,
                Some(count),
                Some(demand),
                R::Reservation {
                    records: count,
                    cause,
                },
            ));
        }
        let scope = self.owner.inputs.root.logical.members.owner.scope;
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
        for (member_index, member) in self.records.iter().enumerate() {
            let Some((operation, target)) = derived(member.source()) else {
                continue;
            };
            let source =
                match scope.operation_source(operation, SemanticOperationKind::LogicalDerivation) {
                    Ok(s) => s,
                    Err(cause) => {
                        return Err(DraftingProgramGarmentRootLogicalSupplierError::new(
                            self,
                            Some(count),
                            Some(demand),
                            R::Lookup {
                                member_index,
                                cause,
                            },
                        ))
                    }
                };
            let SemanticOperation::LogicalDerivation(operation) = source.operation() else {
                unreachable!("actual kind-checked Root logical supplier")
            };
            let output = &operation.definition().output;
            if *output != target {
                return Err(DraftingProgramGarmentRootLogicalSupplierError::new(
                    self,
                    Some(count),
                    Some(demand),
                    R::Output {
                        member_index,
                        source,
                    },
                ));
            }
            let (ordinal, position) = program
                .steps()
                .find_map(|step| match step.kind() {
                    DraftingProgramStepKind::Semantic { ordinal, operation }
                        if core::ptr::eq(operation, source.operation()) =>
                    {
                        Some((ordinal, step.position()))
                    }
                    _ => None,
                })
                .unwrap_or_else(|| {
                    unreachable!(
                        "complete original semantic coverage retains the actual Root supplier"
                    )
                });
            let signature_index = self
                .owner
                .records
                .iter()
                .position(|signature| {
                    core::ptr::eq(
                        signature.source().work().request().operation(),
                        source.operation(),
                    )
                })
                .unwrap_or_else(|| {
                    unreachable!(
                        "complete original signature coverage retains this actual supplier"
                    )
                });
            records.push(DraftingProgramGarmentRootLogicalSupplier {
                member_index,
                source,
                output,
                ordinal,
                position,
                signature_index,
            });
        }
        Ok(BoundDraftingProgramGarmentRootLogicalSuppliers {
            members: self,
            demand: Some(demand),
            records,
        })
    }
}
