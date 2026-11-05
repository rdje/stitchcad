//! Actual Root logical Derived suppliers,without numeric formula-source or rule acceptance.
use super::{BoundDraftingProgramRootInputMembers, DraftingProgramRootInputSource};
use crate::{
    construction::{
        DraftingContextSource, DraftingDesignOperationSource, DraftingGarmentProvider,
        DraftingInitialInputProvider, DraftingProgramSemanticInputs, DraftingProgramStepKind,
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
mod numeric;
pub use error::{
    DraftingProgramRootLogicalSupplierError, DraftingProgramRootLogicalSupplierRefusal,
};
pub use numeric::{
    AdmittedDraftingProgramRuleWork, BoundDraftingProgramGarmentProfileLogicalHeaders,
    BoundDraftingProgramGarmentProfileLogicalParameters,
    BoundDraftingProgramGarmentProfileNumericHeaders, BoundDraftingProgramGarmentRootInputMembers,
    BoundDraftingProgramGarmentRootLogicalSuppliers,
    BoundDraftingProgramGarmentRootNumericSuppliers, BoundDraftingProgramProfileHeaders,
    BoundDraftingProgramProfileParameters, BoundDraftingProgramRootNumericSuppliers,
    BoundDraftingProgramRuleSignatures, BoundDraftingProgramRuleSources,
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
    DraftingProgramGarmentProfileNumericHeadersResult, DraftingProgramGarmentProfileNumericRequest,
    DraftingProgramGarmentRootInputMember, DraftingProgramGarmentRootInputMemberError,
    DraftingProgramGarmentRootInputMemberRefusal, DraftingProgramGarmentRootInputMembersResult,
    DraftingProgramGarmentRootInputSource, DraftingProgramGarmentRootLogicalSupplier,
    DraftingProgramGarmentRootLogicalSupplierError,
    DraftingProgramGarmentRootLogicalSupplierRefusal,
    DraftingProgramGarmentRootLogicalSuppliersResult,
    DraftingProgramGarmentRootNumericSupplierError,
    DraftingProgramGarmentRootNumericSupplierRefusal,
    DraftingProgramGarmentRootNumericSuppliersResult, DraftingProgramOperationInput,
    DraftingProgramProfileHeaderError, DraftingProgramProfileHeaderRefusal,
    DraftingProgramProfileHeadersResult, DraftingProgramProfileInputHeader,
    DraftingProgramProfileInputParameter, DraftingProgramProfileInputRequest,
    DraftingProgramProfileParameterError, DraftingProgramProfileParameterRefusal,
    DraftingProgramProfileParametersResult, DraftingProgramRootInputBinding,
    DraftingProgramRootInputSupplier, DraftingProgramRootNumericSupplier,
    DraftingProgramRootNumericSupplierError, DraftingProgramRootNumericSupplierRefusal,
    DraftingProgramRootNumericSuppliersResult, DraftingProgramRuleSignature,
    DraftingProgramRuleSignatureError, DraftingProgramRuleSignatureRefusal,
    DraftingProgramRuleSignaturesResult, DraftingProgramRuleSource, DraftingProgramRuleSourceError,
    DraftingProgramRuleSourceRefusal, DraftingProgramRuleSourcesResult, DraftingProgramRuleWork,
    DraftingProgramRuleWorkError, DraftingProgramRuleWorkRefusal, DraftingProgramRuleWorkRequest,
    DraftingProgramRuleWorkResult,
};

/// One actual logical Derived consumer and its original Root supplier/output/order association.
/// Its containing owner retains the actual input record;repeated consumers are separate records.
pub struct DraftingProgramRootLogicalSupplier<
    'v,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    member_index: usize,
    source: DraftingDesignOperationSource<'v, 'a, H, S>,
    output: &'a LogicalInputTarget,
    ordinal: usize,
    position: usize,
}
impl<'v, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingProgramRootLogicalSupplier<'v, 'a, H, S>
{
    /// Original member-record index in the complete retained member owner.
    #[must_use]
    pub const fn member_index(&self) -> usize {
        self.member_index
    }
    /// Actual Root LogicalDerivation source,not a Profile operation sharing its ID.
    #[must_use]
    pub const fn source(&self) -> DraftingDesignOperationSource<'v, 'a, H, S> {
        self.source
    }
    /// Original supplier output field,joined to the exact canonical logical input declaration.
    #[must_use]
    pub const fn output(&self) -> &'a LogicalInputTarget {
        self.output
    }
    /// Genuine original one-based semantic supplier ordinal.
    #[must_use]
    pub const fn ordinal(&self) -> usize {
        self.ordinal
    }
    /// Genuine original mixed supplier position,strictly earlier than this consumer.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.position
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingProgramRootLogicalSupplier<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramRootLogicalSupplier")
            .field("member_index", &self.member_index)
            .field("ordinal", &self.ordinal)
            .field("position", &self.position)
            .finish_non_exhaustive()
    }
}
/// Complete logical Derived supplier class only. Numeric Derived,Profile/rule/signature and
/// whole-cohort acceptance remain separate. No values,domain selections or evidence are produced.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<I:?Sized,H:DesignMetadataHost,S:DraftingContextSource<H>>(p:&mut BoundDraftingProgramRootLogicalSuppliers<'_,'_,'_,I,H,S>) {p.records.clear();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn duplicate<I:?Sized,H:DesignMetadataHost,S:DraftingContextSource<H>>(p:BoundDraftingProgramRootLogicalSuppliers<'_,'_,'_,I,H,S>) {let _=p.clone();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn escape<'v,'w,'a,I:?Sized,H:DesignMetadataHost,S:DraftingContextSource<H>>(p:BoundDraftingProgramRootLogicalSuppliers<'v,'w,'a,I,H,S>)->BoundDraftingProgramRootLogicalSuppliers<'static,'w,'a,I,H,S> {p}
/// ```
pub struct BoundDraftingProgramRootLogicalSuppliers<
    'v,
    'w,
    'a,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    members: BoundDraftingProgramRootInputMembers<'v, 'w, 'a, I, H, S>,
    demand: Option<DesignResourceDemand>,
    records: Vec<DraftingProgramRootLogicalSupplier<'v, 'a, H, S>>,
}
impl<'v, 'w, 'a, I: ?Sized, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    BoundDraftingProgramRootLogicalSuppliers<'v, 'w, 'a, I, H, S>
{
    /// Complete original member/source/key owners and the same unique child cursor.
    #[must_use]
    pub const fn members(&self) -> &BoundDraftingProgramRootInputMembers<'v, 'w, 'a, I, H, S> {
        &self.members
    }
    /// Fresh complete source demand for nonempty work only;earlier demand supplies no credit.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.demand
    }
    /// Every actual logical Derived input once,in original member order,without deduplication.
    #[must_use]
    pub fn records(&self) -> &[DraftingProgramRootLogicalSupplier<'v, 'a, H, S>] {
        &self.records
    }
}
impl<I: ?Sized, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for BoundDraftingProgramRootLogicalSuppliers<'_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProgramRootLogicalSuppliers")
            .field("records", &self.records.len())
            .finish_non_exhaustive()
    }
}
/// Complete logical supplier class,or original member owner and first typed supplier refusal.
pub type DraftingProgramRootLogicalSuppliersResult<'v, 'w, 'a, I, H, S> = Result<
    BoundDraftingProgramRootLogicalSuppliers<'v, 'w, 'a, I, H, S>,
    DraftingProgramRootLogicalSupplierError<'v, 'w, 'a, I, H, S>,
>;

fn derived<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>(
    source: DraftingProgramRootInputSource<'_, '_, H, S>,
) -> Option<(EntityId, LogicalInputTarget)> {
    let DraftingProgramRootInputSource::Logical(source) = source else {
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
    BoundDraftingProgramRootInputMembers<
        'v,
        'w,
        'a,
        DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        H,
        S,
    >
{
    /// Reserve all logical Derived child passes/results before fresh exact Root/provider charging,
    /// fallible complete record reservation and original supplier lookup/output/order joins.
    /// Each of N passes covers one Root recipe scan and one genuine mixed-step scan within the
    /// original (E+8)^2 comparison envelope. One fresh source walk fits the N-pass content bound.
    /// N=0 performs no child reservation,walk,allocation or lookup and grants no new content proof.
    /// No numeric Derived formula source receives a semantic or synthetic mixed position here.
    /// # Errors
    /// Complete population overflow,atomic child reservation,first fresh resource cause,allocator,
    /// then first original supplier lookup,output mismatch or non-earlier genuine position.
    /// Refusal retains original members/cursor;no private successful supplier prefix escapes.
    pub fn bind_root_logical_suppliers(
        mut self,
    ) -> DraftingProgramRootLogicalSuppliersResult<
        'v,
        'w,
        'a,
        DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        H,
        S,
    > {
        use DraftingProgramRootLogicalSupplierRefusal as R;
        let count = self.records.iter().try_fold(0usize, |count, member| {
            count.checked_add(usize::from(derived(member.source()).is_some()))
        });
        let Some(count) = count else {
            return Err(DraftingProgramRootLogicalSupplierError::new(
                self,
                None,
                None,
                R::PopulationOverflow,
            ));
        };
        if let Err(cause) = self.owner.selections.reserve_child_passes(count) {
            return Err(DraftingProgramRootLogicalSupplierError::new(
                self,
                Some(count),
                None,
                R::Work(cause),
            ));
        }
        if count == 0 {
            return Ok(BoundDraftingProgramRootLogicalSuppliers {
                members: self,
                demand: None,
                records: Vec::new(),
            });
        }
        let mut meter =
            DesignResourceMeter::new(self.owner.selections.limits().population().content());
        let charged = self.owner.scope.charge_resources(&mut meter);
        let demand = match meter.finish() {
            Ok(demand) => demand,
            Err(cause) => {
                return Err(DraftingProgramRootLogicalSupplierError::new(
                    self,
                    Some(count),
                    None,
                    R::Resources(cause),
                ))
            }
        };
        if let Err(cause) = charged {
            return Err(DraftingProgramRootLogicalSupplierError::new(
                self,
                Some(count),
                None,
                R::Resources(cause),
            ));
        }
        let mut records = Vec::new();
        if let Err(cause) = records.try_reserve_exact(count) {
            return Err(DraftingProgramRootLogicalSupplierError::new(
                self,
                Some(count),
                Some(demand),
                R::Reservation {
                    records: count,
                    cause,
                },
            ));
        }
        let program = self
            .owner
            .inputs()
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
        for (member_index, member) in self.records.iter().copied().enumerate() {
            let Some((operation, target)) = derived(member.source()) else {
                continue;
            };
            let source = match self
                .owner
                .scope
                .operation_source(operation, SemanticOperationKind::LogicalDerivation)
            {
                Ok(source) => source,
                Err(cause) => {
                    return Err(DraftingProgramRootLogicalSupplierError::new(
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
                return Err(DraftingProgramRootLogicalSupplierError::new(
                    self,
                    Some(count),
                    Some(demand),
                    R::Output {
                        member_index,
                        source,
                    },
                ));
            }
            let supplier = program
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
            if supplier.1 >= member.position() {
                return Err(DraftingProgramRootLogicalSupplierError::new(
                    self,
                    Some(count),
                    Some(demand),
                    R::Order {
                        member_index,
                        source,
                        ordinal: supplier.0,
                        position: supplier.1,
                    },
                ));
            }
            records.push(DraftingProgramRootLogicalSupplier {
                member_index,
                source,
                output,
                ordinal: supplier.0,
                position: supplier.1,
            });
        }
        Ok(BoundDraftingProgramRootLogicalSuppliers {
            members: self,
            demand: Some(demand),
            records,
        })
    }
}
