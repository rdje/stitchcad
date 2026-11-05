//! Complete original Root-owned Pocket/Closure input members,without invented consumer positions.
use super::BoundDraftingProgramRuleSignatures;
use crate::{
    construction::{
        DraftingDesignLogicalSource, DraftingDesignNumericSource, DraftingGarmentProvider,
        DraftingGarmentSemanticField, DraftingGarmentSemanticUse, DraftingInitialInputProvider,
        DraftingProfileLogicalProvider, DraftingProgramSemanticInputs, DraftingRuleProvider,
    },
    design::{
        DesignMetadataHost, DesignParameter, DesignReferenceHost, DesignResourceDemand,
        DesignResourceMeter, DesignResourceSource,
    },
    ontology::{ClosureLength, EntityId, NotionSize, PocketOpening},
    recipe::{FormulaDeclaration, FormulaKind},
    value::{LogicalInputKind, LogicalInputTarget},
};
use core::fmt;
use sc_units::Length;
mod error;
mod logical;
pub use error::{
    DraftingProgramGarmentRootInputMemberError, DraftingProgramGarmentRootInputMemberRefusal,
};
pub use logical::{
    BoundDraftingProgramGarmentProfileLogicalHeaders,
    BoundDraftingProgramGarmentProfileLogicalParameters,
    BoundDraftingProgramGarmentRootLogicalSuppliers,
    BoundDraftingProgramGarmentRootNumericSuppliers, DraftingProgramGarmentProfileLogicalHeader,
    DraftingProgramGarmentProfileLogicalHeaderError,
    DraftingProgramGarmentProfileLogicalHeaderRefusal,
    DraftingProgramGarmentProfileLogicalHeadersResult,
    DraftingProgramGarmentProfileLogicalParameter,
    DraftingProgramGarmentProfileLogicalParameterError,
    DraftingProgramGarmentProfileLogicalParameterRefusal,
    DraftingProgramGarmentProfileLogicalParametersResult,
    DraftingProgramGarmentProfileLogicalRequest, DraftingProgramGarmentRootLogicalSupplier,
    DraftingProgramGarmentRootLogicalSupplierError,
    DraftingProgramGarmentRootLogicalSupplierRefusal,
    DraftingProgramGarmentRootLogicalSuppliersResult,
    DraftingProgramGarmentRootNumericSupplierError,
    DraftingProgramGarmentRootNumericSupplierRefusal,
    DraftingProgramGarmentRootNumericSuppliersResult,
};
/// Exact original logical member,inline length intent,or parameter-to-canonical numeric member.
/// Public variants are views;only the private complete owner certifies coverage and admission.
pub enum DraftingProgramGarmentRootInputSource<
    'v,
    'a,
    H: DesignMetadataHost,
    S: DraftingRuleProvider<H> + ?Sized,
> {
    /// Original expected-kind Root logical member and its actual domain/state/provenance.
    Logical(DraftingDesignLogicalSource<'v, 'a, H, S>),
    /// Original positive inline Closure intent;the metadata ID is not a canonical declaration ID.
    ExplicitLength {
        /// Original authored metadata identity,without a global identity join.
        parameter: &'a EntityId,
        /// Original authored field,not a computed/canonical value read.
        value: &'a Length,
    },
    /// Original actual Design parameter,Length-kind metadata and exact canonical numeric target.
    FormulaLength {
        /// Actual original parameter record,not a same-number canonical declaration.
        parameter: &'a DesignParameter,
        /// Original current parameter metadata under its actual canonical target.
        metadata: FormulaDeclaration<'a>,
        /// Actual original dedicated/general Length member and unchanged state/provenance.
        source: DraftingDesignNumericSource<'v, 'a, H, S>,
    },
}
impl<H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramGarmentRootInputSource<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Logical(_) => "Logical",
            Self::ExplicitLength { .. } => "ExplicitLength",
            Self::FormulaLength { .. } => "FormulaLength",
        })
    }
}
/// One genuine original garment/member/field/location and its Root-owned input association.
/// Root consumers have no semantic ordinal,mixed position or fabricated end-of-program sentinel.
pub struct DraftingProgramGarmentRootInputMember<
    'v,
    'a,
    H: DesignMetadataHost,
    S: DraftingRuleProvider<H> + ?Sized,
> {
    use_: DraftingGarmentSemanticUse<'a, H>,
    source: DraftingProgramGarmentRootInputSource<'v, 'a, H, S>,
}
impl<'v, 'a, H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized>
    DraftingProgramGarmentRootInputMember<'v, 'a, H, S>
{
    /// Actual original garment/owner/index/field and its authentic Root location.
    #[must_use]
    pub const fn occurrence(&self) -> DraftingGarmentSemanticUse<'a, H> {
        self.use_
    }
    /// Actual original typed input source,without supplier,value or physical acceptance.
    #[must_use]
    pub const fn source(&self) -> &DraftingProgramGarmentRootInputSource<'v, 'a, H, S> {
        &self.source
    }
}
impl<H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramGarmentRootInputMember<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramGarmentRootInputMember")
            .field("occurrence", &self.use_)
            .field("source", &self.source)
            .finish()
    }
}
/// Complete actual Root-owned Pocket/Closure input class with original operation proofs/cursor.
/// Profile fields,suppliers,served/components,Closure relations and whole cohort remain separate.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:&mut BoundDraftingProgramGarmentRootInputMembers<'_,'_,'_,'_,'_,'_,I,H,S>) {p.records.clear();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn duplicate<I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:BoundDraftingProgramGarmentRootInputMembers<'_,'_,'_,'_,'_,'_,I,H,S>) {let _=p.clone();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn escape<'z,'q,'h,'v,'w,'a,I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:BoundDraftingProgramGarmentRootInputMembers<'z,'q,'h,'v,'w,'a,I,H,S>)->BoundDraftingProgramGarmentRootInputMembers<'static,'q,'h,'v,'w,'a,I,H,S> {p}
/// ```
pub struct BoundDraftingProgramGarmentRootInputMembers<
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
    owner: BoundDraftingProgramRuleSignatures<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
    demand: Option<DesignResourceDemand>,
    records: Vec<DraftingProgramGarmentRootInputMember<'v, 'a, H, S>>,
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
    > BoundDraftingProgramGarmentRootInputMembers<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>
{
    /// Complete original operation inputs/rule signatures and the same unique finite cursor.
    #[must_use]
    pub const fn owner(
        &self,
    ) -> &BoundDraftingProgramRuleSignatures<'z, 'q, 'h, 'v, 'w, 'a, I, H, S> {
        &self.owner
    }
    /// Fresh complete aggregate/proof demand before nonempty membership;None for N0.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.demand
    }
    /// Every original Root-owned Pocket/Closure input once,in actual garment/local order.
    #[must_use]
    pub fn records(&self) -> &[DraftingProgramGarmentRootInputMember<'v, 'a, H, S>] {
        &self.records
    }
    /// N fresh whole/member pass/result envelopes;zero for N0.
    #[must_use]
    pub fn child_pass_count(&self) -> usize {
        self.records.len()
    }
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > fmt::Debug for BoundDraftingProgramGarmentRootInputMembers<'_, '_, '_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProgramGarmentRootInputMembers")
            .field("records", &self.records.len())
            .finish_non_exhaustive()
    }
}
/// Complete original members or exact original operation/cursor owners and first typed cause.
pub type DraftingProgramGarmentRootInputMembersResult<'z, 'q, 'h, 'v, 'w, 'a, I, H, S> = Result<
    BoundDraftingProgramGarmentRootInputMembers<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
    DraftingProgramGarmentRootInputMemberError<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
>;
enum Request<'a> {
    Logical(LogicalInputTarget),
    Explicit(&'a EntityId, &'a Length),
    Formula(&'a EntityId),
}
fn request(field: DraftingGarmentSemanticField<'_>) -> Option<Request<'_>> {
    match field {
        DraftingGarmentSemanticField::PocketOpening(PocketOpening::Declaration(declaration)) => {
            Some(Request::Logical(LogicalInputTarget {
                declaration: *declaration,
                kind: LogicalInputKind::PocketOpening,
            }))
        }
        DraftingGarmentSemanticField::ClosureSize {
            source: NotionSize::Declaration(declaration),
            ..
        } => Some(Request::Logical(LogicalInputTarget {
            declaration: *declaration,
            kind: LogicalInputKind::NotionSize,
        })),
        DraftingGarmentSemanticField::ClosureLength(ClosureLength::Explicit {
            parameter,
            value,
        }) => Some(Request::Explicit(parameter, value)),
        DraftingGarmentSemanticField::ClosureLength(ClosureLength::Formula(parameter)) => {
            Some(Request::Formula(parameter))
        }
        DraftingGarmentSemanticField::PocketOpening(PocketOpening::Profile(_))
        | DraftingGarmentSemanticField::ClosureSize {
            source: NotionSize::Profile(_),
            ..
        }
        | DraftingGarmentSemanticField::ClosureLength(ClosureLength::Profile(_))
        | DraftingGarmentSemanticField::PocketServed(_)
        | DraftingGarmentSemanticField::PocketPosition(_)
        | DraftingGarmentSemanticField::PocketOrientation(_)
        | DraftingGarmentSemanticField::PocketComponent { .. }
        | DraftingGarmentSemanticField::ClosureButtonhole(_)
        | DraftingGarmentSemanticField::ClosureInstance { .. }
        | DraftingGarmentSemanticField::ClosureUnsupportedFly(_) => None,
    }
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
    BoundDraftingProgramRuleSignatures<
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
    /// Count all actual Root-owned Pocket opening,Closure size and inline/formula zipper fields;
    /// reserve N complete whole/member/result envelopes before fresh complete aggregate/proof
    /// content,fallible N-record reservation or any member/parameter lookup. One whole walk plus
    /// each original bounded member/parameter join fits N*(whole-demand,(E+8)^2) for N>0.
    /// Reuse actual Design parameter-to-canonical metadata semantics for formula zipper lengths;
    /// inline fields retain original intent without inventing canonical records. No source/value
    /// query,supplier/consumer position,Profile source,physical relation or whole cohort is accepted.
    /// N0 adds no work,walk,allocation,lookup or demand claim.
    /// # Errors
    /// Complete count arithmetic,atomic cursor shortage,fresh aggregate resources,allocator,then
    /// first original member or parameter/kind cause. Private successful prefixes drop;original
    /// operation/cursor owners and actual garment/field/metadata witnesses remain inspectable.
    pub fn bind_garment_root_input_members(
        self,
    ) -> DraftingProgramGarmentRootInputMembersResult<
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
        use DraftingProgramGarmentRootInputMemberRefusal as R;
        let inputs = self.inputs.root.logical.members.owner.inputs();
        let garment = inputs.garment().source().garment;
        let count = garment
            .pocket_closure_dependencies()
            .try_fold(0usize, |n, use_| {
                n.checked_add(usize::from(request(use_.field()).is_some()))
            });
        let Some(count) = count else {
            return Err(DraftingProgramGarmentRootInputMemberError::new(
                self,
                None,
                None,
                R::PopulationOverflow,
            ));
        };
        if let Err(cause) = self
            .inputs
            .root
            .logical
            .members
            .owner
            .selections
            .reserve_child_passes(count)
        {
            return Err(DraftingProgramGarmentRootInputMemberError::new(
                self,
                Some(count),
                None,
                R::Work(cause),
            ));
        }
        if count == 0 {
            return Ok(BoundDraftingProgramGarmentRootInputMembers {
                owner: self,
                demand: None,
                records: Vec::new(),
            });
        }
        let content = self
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
                return Err(DraftingProgramGarmentRootInputMemberError::new(
                    self,
                    Some(count),
                    None,
                    R::Resources(cause),
                ))
            }
        };
        if let Err(cause) = charged {
            return Err(DraftingProgramGarmentRootInputMemberError::new(
                self,
                Some(count),
                None,
                R::Resources(cause),
            ));
        }
        let mut records = Vec::new();
        if let Err(cause) = records.try_reserve_exact(count) {
            return Err(DraftingProgramGarmentRootInputMemberError::new(
                self,
                Some(count),
                Some(demand),
                R::Reservation {
                    records: count,
                    cause,
                },
            ));
        }
        let scope = self.inputs.root.logical.members.owner.scope;
        for use_ in garment.pocket_closure_dependencies() {
            let source = match request(use_.field()) {
                None => continue,
                Some(Request::Logical(target)) => match scope.logical_source(target) {
                    Ok(source) => DraftingProgramGarmentRootInputSource::Logical(source),
                    Err(cause) => {
                        return Err(DraftingProgramGarmentRootInputMemberError::new(
                            self,
                            Some(count),
                            Some(demand),
                            R::Lookup { use_, cause },
                        ))
                    }
                },
                Some(Request::Explicit(parameter, value)) => {
                    DraftingProgramGarmentRootInputSource::ExplicitLength { parameter, value }
                }
                Some(Request::Formula(identity)) => {
                    let root = scope.context().scope().root();
                    let Some(parameter) = root.parameters().parameter(*identity) else {
                        return Err(DraftingProgramGarmentRootInputMemberError::new(
                            self,
                            Some(count),
                            Some(demand),
                            R::MissingParameter {
                                use_,
                                identity: *identity,
                            },
                        ));
                    };
                    let metadata = match parameter.declaration(root.inputs()) {
                        Ok(d) => d,
                        Err(cause) => {
                            return Err(DraftingProgramGarmentRootInputMemberError::new(
                                self,
                                Some(count),
                                Some(demand),
                                R::Parameter {
                                    use_,
                                    parameter,
                                    cause,
                                },
                            ))
                        }
                    };
                    if metadata.kind() != FormulaKind::Length {
                        return Err(DraftingProgramGarmentRootInputMemberError::new(
                            self,
                            Some(count),
                            Some(demand),
                            R::FormulaKind {
                                use_,
                                parameter,
                                metadata,
                            },
                        ));
                    }
                    let source = match scope.numeric_source(parameter.definition().target) {
                        Ok(s) => s,
                        Err(cause) => {
                            return Err(DraftingProgramGarmentRootInputMemberError::new(
                                self,
                                Some(count),
                                Some(demand),
                                R::Lookup { use_, cause },
                            ))
                        }
                    };
                    DraftingProgramGarmentRootInputSource::FormulaLength {
                        parameter,
                        metadata,
                        source,
                    }
                }
            };
            records.push(DraftingProgramGarmentRootInputMember { use_, source });
        }
        Ok(BoundDraftingProgramGarmentRootInputMembers {
            owner: self,
            demand: Some(demand),
            records,
        })
    }
}
