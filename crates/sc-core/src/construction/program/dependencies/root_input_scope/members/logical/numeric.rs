//! Actual numeric Derived source/checked/acyclic associations,with no synthetic mixed positions.
use super::BoundDraftingProgramRootLogicalSuppliers;
use crate::{
    construction::{
        DraftingContextSource, DraftingGarmentProvider, DraftingInitialInputProvider,
        DraftingProgramRootInputSource, DraftingProgramSemanticInputs,
    },
    design::{
        DesignMetadataHost, DesignReferenceHost, DesignResourceDemand, DesignResourceMeter,
        DesignResourceSource,
    },
    recipe::{FormulaAcyclicSourceDependencies, FormulaCheckedSource, FormulaDerivedInput},
    value::{CanonicalNumericInputMember, LengthState, ScalarInputState},
};
use core::fmt;
mod compose;
mod error;
mod profile_headers;
pub use compose::{DraftingProgramRootInputBinding, DraftingProgramRootInputSupplier};
pub use error::{
    DraftingProgramRootNumericSupplierError, DraftingProgramRootNumericSupplierRefusal,
};
pub use profile_headers::{
    BoundDraftingProgramProfileHeaders, BoundDraftingProgramProfileParameters,
    DraftingProgramOperationInput, DraftingProgramProfileHeaderError,
    DraftingProgramProfileHeaderRefusal, DraftingProgramProfileHeadersResult,
    DraftingProgramProfileInputHeader, DraftingProgramProfileInputParameter,
    DraftingProgramProfileInputRequest, DraftingProgramProfileParameterError,
    DraftingProgramProfileParameterRefusal, DraftingProgramProfileParametersResult,
};

/// One original numeric Derived consumer and its actual source/checked/acyclic owners.
/// Canonical family position remains in input();no mixed supplier position is invented.
#[derive(Clone, Copy)]
pub struct DraftingProgramRootNumericSupplier<'w> {
    member_index: usize,
    input: FormulaDerivedInput<'w>,
    source: &'w FormulaCheckedSource<'w>,
    graph: &'w FormulaAcyclicSourceDependencies<'w>,
}
impl<'w> DraftingProgramRootNumericSupplier<'w> {
    /// Actual original consumer member index,including repeated references separately.
    #[must_use]
    pub const fn member_index(self) -> usize {
        self.member_index
    }
    /// Exact original canonical declaration,state,source and canonical family position.
    #[must_use]
    pub const fn input(self) -> FormulaDerivedInput<'w> {
        self.input
    }
    /// Actual original checked expression proof for input().source().
    #[must_use]
    pub const fn source(self) -> &'w FormulaCheckedSource<'w> {
        self.source
    }
    /// Exact already-held acyclic owner over that complete original checked-source proof.
    #[must_use]
    pub const fn graph(self) -> &'w FormulaAcyclicSourceDependencies<'w> {
        self.graph
    }
}
impl fmt::Debug for DraftingProgramRootNumericSupplier<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramRootNumericSupplier")
            .field("member_index", &self.member_index)
            .finish_non_exhaustive()
    }
}
/// Complete numeric Derived supplier class after complete logical supplier binding. Original
/// members and the same finite cursor survive;Profile/rule/cohort/execution remain separate.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<I:?Sized,H:DesignMetadataHost,S:DraftingContextSource<H>>(p:&mut BoundDraftingProgramRootNumericSuppliers<'_,'_,'_,I,H,S>) {p.records.clear();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn duplicate<I:?Sized,H:DesignMetadataHost,S:DraftingContextSource<H>>(p:BoundDraftingProgramRootNumericSuppliers<'_,'_,'_,I,H,S>) {let _=p.clone();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn escape<'v,'w,'a,I:?Sized,H:DesignMetadataHost,S:DraftingContextSource<H>>(p:BoundDraftingProgramRootNumericSuppliers<'v,'w,'a,I,H,S>)->BoundDraftingProgramRootNumericSuppliers<'v,'static,'a,I,H,S> {p}
/// ```
pub struct BoundDraftingProgramRootNumericSuppliers<
    'v,
    'w,
    'a,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    logical: BoundDraftingProgramRootLogicalSuppliers<'v, 'w, 'a, I, H, S>,
    demand: Option<DesignResourceDemand>,
    records: Vec<DraftingProgramRootNumericSupplier<'w>>,
}
impl<'v, 'w, 'a, I: ?Sized, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    BoundDraftingProgramRootNumericSuppliers<'v, 'w, 'a, I, H, S>
{
    /// Complete original logical suppliers,members,key/source owners and unique child cursor.
    #[must_use]
    pub const fn logical(&self) -> &BoundDraftingProgramRootLogicalSuppliers<'v, 'w, 'a, I, H, S> {
        &self.logical
    }
    /// Complete freshly measured aggregate content/proofs for nonempty numeric work only.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.demand
    }
    /// Every numeric Derived input in genuine member order,without deduplicating targets/sources.
    #[must_use]
    pub fn records(&self) -> &[DraftingProgramRootNumericSupplier<'w>] {
        &self.records
    }
}
impl<I: ?Sized, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for BoundDraftingProgramRootNumericSuppliers<'_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProgramRootNumericSuppliers")
            .field("records", &self.records.len())
            .finish_non_exhaustive()
    }
}
/// Complete numeric supplier class,or the complete original logical owner and first refusal.
pub type DraftingProgramRootNumericSuppliersResult<'v, 'w, 'a, I, H, S> = Result<
    BoundDraftingProgramRootNumericSuppliers<'v, 'w, 'a, I, H, S>,
    DraftingProgramRootNumericSupplierError<'v, 'w, 'a, I, H, S>,
>;
fn derived<'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>(
    source: DraftingProgramRootInputSource<'_, 'a, H, S>,
) -> Option<CanonicalNumericInputMember<'a>> {
    let DraftingProgramRootInputSource::Numeric(source) = source else {
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
        S: DraftingInitialInputProvider<H> + DraftingGarmentProvider<H> + ?Sized,
    >
    BoundDraftingProgramRootLogicalSuppliers<
        'v,
        'w,
        'a,
        DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        H,
        S,
    >
{
    /// Count/reserve all N numeric Derived passes/results before freshly charging complete original
    /// aggregate inputs and held checked/order/acyclic records. Previous demand supplies no credit.
    /// Only after complete content success,fallibly reserve N relations and match exact actual
    /// canonical declarations to original FormulaDerivedInput/source proof/held acyclic owners.
    /// Each N-pass envelope bounds a native Derived iterator scan,its source lookups and one checked
    /// proof lookup by (E+8)^2. Matching reads no authored numeric value or formula-only metadata.
    /// N0 performs no child reservation,content walk,allocation or lookup and grants no fresh demand.
    /// # Errors
    /// Complete population overflow,atomic remaining cursor reservation,first actual fresh content
    /// cause,then full result allocator cause. Original source existence/kind/acyclic failures have
    /// already refused in their exact prerequisite owners;no successful supplier prefix escapes.
    pub fn bind_root_numeric_suppliers(
        mut self,
    ) -> DraftingProgramRootNumericSuppliersResult<
        'v,
        'w,
        'a,
        DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        H,
        S,
    > {
        use DraftingProgramRootNumericSupplierRefusal as R;
        let count = self
            .members
            .records
            .iter()
            .try_fold(0usize, |count, member| {
                count.checked_add(usize::from(derived(member.source()).is_some()))
            });
        let Some(count) = count else {
            return Err(DraftingProgramRootNumericSupplierError::new(
                self,
                None,
                None,
                R::PopulationOverflow,
            ));
        };
        if let Err(cause) = self.members.owner.selections.reserve_child_passes(count) {
            return Err(DraftingProgramRootNumericSupplierError::new(
                self,
                Some(count),
                None,
                R::Work(cause),
            ));
        }
        if count == 0 {
            return Ok(BoundDraftingProgramRootNumericSuppliers {
                logical: self,
                demand: None,
                records: Vec::new(),
            });
        }
        let mut meter = DesignResourceMeter::new(
            self.members
                .owner
                .selections
                .limits()
                .population()
                .content(),
        );
        let charged = self.members.owner.inputs().charge_resources(&mut meter);
        let demand = match meter.finish() {
            Ok(demand) => demand,
            Err(cause) => {
                return Err(DraftingProgramRootNumericSupplierError::new(
                    self,
                    Some(count),
                    None,
                    R::Resources(cause),
                ))
            }
        };
        if let Err(cause) = charged {
            return Err(DraftingProgramRootNumericSupplierError::new(
                self,
                Some(count),
                None,
                R::Resources(cause),
            ));
        }
        let mut records = Vec::new();
        if let Err(cause) = records.try_reserve_exact(count) {
            return Err(DraftingProgramRootNumericSupplierError::new(
                self,
                Some(count),
                Some(demand),
                R::Reservation {
                    records: count,
                    cause,
                },
            ));
        }
        let dependencies = self
            .members
            .owner
            .inputs()
            .geometry()
            .work()
            .inputs()
            .inputs();
        let sources = dependencies.sources().proof();
        let source_inputs = sources.normalized().inputs();
        for (member_index, member) in self.members.records.iter().copied().enumerate() {
            let Some(actual) = derived(member.source()) else {
                continue;
            };
            let input = source_inputs
                .derived_inputs()
                .find(|input| match actual {
                    CanonicalNumericInputMember::Length { declaration, .. } => input
                        .length()
                        .is_some_and(|original| core::ptr::eq(original, declaration)),
                    CanonicalNumericInputMember::Scalar { declaration, .. } => input
                        .scalar()
                        .is_some_and(|original| core::ptr::eq(original, declaration)),
                })
                .unwrap_or_else(|| {
                    unreachable!(
                        "same Root member and complete immutable numeric Derived/source owner"
                    )
                });
            let source = sources
                .source(input.source().id())
                .filter(|source| core::ptr::eq(source.normalized().source(), input.source()))
                .unwrap_or_else(|| {
                    unreachable!(
                        "complete original checked source proof retains actual Derived source"
                    )
                });
            records.push(DraftingProgramRootNumericSupplier {
                member_index,
                input,
                source,
                graph: dependencies.graph(),
            });
        }
        Ok(BoundDraftingProgramRootNumericSuppliers {
            logical: self,
            demand: Some(demand),
            records,
        })
    }
}
