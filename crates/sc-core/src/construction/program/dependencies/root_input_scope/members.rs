//! Complete actual Root operation-input membership,with original scope and one consuming budget.
use super::DraftingProgramRootInputScope;
use crate::{
    construction::{
        DraftingContextSource, DraftingDesignLogicalSource, DraftingDesignNumericSource,
        DraftingGarmentProvider, DraftingInitialInputProvider, DraftingProgramSemanticInputs,
        DraftingSemanticField, DraftingSemanticSelectionLocation,
    },
    design::{
        DesignMetadataHost, DesignReferenceHost, DesignResourceDemand, DesignResourceMeter,
        DesignResourceSource,
    },
    ontology::NotionSize,
    recipe::{LogicalDerivationInputSource, SemanticOperation},
    value::{CanonicalInputTarget, LogicalInputKind, LogicalInputTarget},
};
use core::fmt;
mod error;
mod logical;
pub use error::{DraftingProgramRootInputMemberError, DraftingProgramRootInputMemberRefusal};
pub use logical::{
    AdmittedDraftingProgramRuleWork, BoundDraftingProgramGarmentRootInputMembers,
    BoundDraftingProgramProfileHeaders, BoundDraftingProgramProfileParameters,
    BoundDraftingProgramRootLogicalSuppliers, BoundDraftingProgramRootNumericSuppliers,
    BoundDraftingProgramRuleSignatures, BoundDraftingProgramRuleSources,
    DraftingProgramGarmentRootInputMember, DraftingProgramGarmentRootInputMemberError,
    DraftingProgramGarmentRootInputMemberRefusal, DraftingProgramGarmentRootInputMembersResult,
    DraftingProgramGarmentRootInputSource, DraftingProgramOperationInput,
    DraftingProgramProfileHeaderError, DraftingProgramProfileHeaderRefusal,
    DraftingProgramProfileHeadersResult, DraftingProgramProfileInputHeader,
    DraftingProgramProfileInputParameter, DraftingProgramProfileInputRequest,
    DraftingProgramProfileParameterError, DraftingProgramProfileParameterRefusal,
    DraftingProgramProfileParametersResult, DraftingProgramRootInputBinding,
    DraftingProgramRootInputSupplier, DraftingProgramRootLogicalSupplier,
    DraftingProgramRootLogicalSupplierError, DraftingProgramRootLogicalSupplierRefusal,
    DraftingProgramRootLogicalSuppliersResult, DraftingProgramRootNumericSupplier,
    DraftingProgramRootNumericSupplierError, DraftingProgramRootNumericSupplierRefusal,
    DraftingProgramRootNumericSuppliersResult, DraftingProgramRuleSignature,
    DraftingProgramRuleSignatureError, DraftingProgramRuleSignatureRefusal,
    DraftingProgramRuleSignaturesResult, DraftingProgramRuleSource, DraftingProgramRuleSourceError,
    DraftingProgramRuleSourceRefusal, DraftingProgramRuleSourcesResult, DraftingProgramRuleWork,
    DraftingProgramRuleWorkError, DraftingProgramRuleWorkRefusal, DraftingProgramRuleWorkRequest,
    DraftingProgramRuleWorkResult,
};

/// Original kind-checked Root source view. It reads no value and accepts no Derived supplier/order.
pub enum DraftingProgramRootInputSource<
    'v,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    /// Actual original finite logical member under its exact fresh Root source scope.
    Logical(DraftingDesignLogicalSource<'v, 'a, H, S>),
    /// Actual original dedicated-length/general-scalar member under the same original scope.
    Numeric(DraftingDesignNumericSource<'v, 'a, H, S>),
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> Copy
    for DraftingProgramRootInputSource<'_, '_, H, S>
{
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> Clone
    for DraftingProgramRootInputSource<'_, '_, H, S>
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingProgramRootInputSource<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Logical(source) => f.debug_tuple("Logical").field(source).finish(),
            Self::Numeric(source) => f.debug_tuple("Numeric").field(source).finish(),
        }
    }
}
/// One original semantic operation-input field and its actual Root member. The containing private
/// layer retains the exact normalized/source/selection owners;this record supplies no whole proof.
pub struct DraftingProgramRootInputMember<
    'v,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    operation: &'a SemanticOperation,
    ordinal: usize,
    position: usize,
    location: DraftingSemanticSelectionLocation,
    field: DraftingSemanticField<'a>,
    source: DraftingProgramRootInputSource<'v, 'a, H, S>,
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> Copy
    for DraftingProgramRootInputMember<'_, '_, H, S>
{
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> Clone
    for DraftingProgramRootInputMember<'_, '_, H, S>
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<'v, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingProgramRootInputMember<'v, 'a, H, S>
{
    /// Exact original operation,not a replacement body sharing its ID.
    #[must_use]
    pub const fn operation(self) -> &'a SemanticOperation {
        self.operation
    }
    /// Original one-based semantic ordinal,not a mixed position.
    #[must_use]
    pub const fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Genuine original one-based mixed consumer position.
    #[must_use]
    pub const fn position(self) -> usize {
        self.position
    }
    /// Actual original field coordinates under the retained normalized owner.
    #[must_use]
    pub const fn location(self) -> DraftingSemanticSelectionLocation {
        self.location
    }
    /// Exact original header/named-input source field loan.
    #[must_use]
    pub const fn field(self) -> DraftingSemanticField<'a> {
        self.field
    }
    /// Actual original Root member/source association,without state/value copying.
    #[must_use]
    pub const fn source(self) -> DraftingProgramRootInputSource<'v, 'a, H, S> {
        self.source
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingProgramRootInputMember<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramRootInputMember")
            .field("ordinal", &self.ordinal)
            .field("position", &self.position)
            .field("field", &self.field)
            .field("source", &self.source)
            .finish()
    }
}
/// Complete Root-owned operation-input members only. Profile inputs/rule signatures/outputs,
/// Derived supplier/order,garment/native source fields and whole cohort remain separate stages.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<I:?Sized,H:DesignMetadataHost,S:DraftingContextSource<H>>(p:&mut BoundDraftingProgramRootInputMembers<'_,'_,'_,I,H,S>) {p.records.clear();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn escape<'v,'w,'a,I:?Sized,H:DesignMetadataHost,S:DraftingContextSource<H>>(p:BoundDraftingProgramRootInputMembers<'v,'w,'a,I,H,S>)->BoundDraftingProgramRootInputMembers<'static,'w,'a,I,H,S> {p}
/// ```
pub struct BoundDraftingProgramRootInputMembers<
    'v,
    'w,
    'a,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    owner: DraftingProgramRootInputScope<'v, 'w, 'a, I, H, S>,
    demand: Option<DesignResourceDemand>,
    records: Vec<DraftingProgramRootInputMember<'v, 'a, H, S>>,
}
impl<'v, 'w, 'a, I: ?Sized, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    BoundDraftingProgramRootInputMembers<'v, 'w, 'a, I, H, S>
{
    /// Exact original consumer/key/source owners and same remaining child cursor.
    #[must_use]
    pub const fn owner(&self) -> &DraftingProgramRootInputScope<'v, 'w, 'a, I, H, S> {
        &self.owner
    }
    /// Complete fresh Root/provider content for a nonempty Root-input stage. An actually empty
    /// stage performs no child walk or lookup and grants no new content proof,so returns None.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.demand
    }
    /// Every actual Root operation-input field once,in genuine mixed/local order.
    #[must_use]
    pub fn records(&self) -> &[DraftingProgramRootInputMember<'v, 'a, H, S>] {
        &self.records
    }
}
impl<I: ?Sized, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for BoundDraftingProgramRootInputMembers<'_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProgramRootInputMembers")
            .field("records", &self.records.len())
            .finish_non_exhaustive()
    }
}
/// Original exact source/consumer owners and complete Root-member layer,or first original refusal.
pub type DraftingProgramRootInputMembersResult<'v, 'w, 'a, I, H, S> = Result<
    BoundDraftingProgramRootInputMembers<'v, 'w, 'a, I, H, S>,
    DraftingProgramRootInputMemberError<'v, 'w, 'a, I, H, S>,
>;

#[derive(Clone, Copy)]
enum Request {
    Logical(LogicalInputTarget),
    Numeric(CanonicalInputTarget),
}
fn request(field: DraftingSemanticField<'_>) -> Option<Request> {
    match field {
        DraftingSemanticField::ButtonholeButton(button) => match button {
            NotionSize::Declaration(declaration) => Some(Request::Logical(LogicalInputTarget {
                declaration: *declaration,
                kind: LogicalInputKind::NotionSize,
            })),
            NotionSize::Profile(_) => None,
        },
        DraftingSemanticField::ButtonholeInput { index: _, input } => {
            Some(Request::Numeric(input.target))
        }
        DraftingSemanticField::LogicalInput { index: _, input } => match input.source {
            LogicalDerivationInputSource::Logical(target) => Some(Request::Logical(target)),
            LogicalDerivationInputSource::Numeric(target) => Some(Request::Numeric(target)),
            LogicalDerivationInputSource::Profile {
                parameter: _,
                kind: _,
            } => None,
        },
        DraftingSemanticField::ButtonholePolicy(_)
        | DraftingSemanticField::LogicalRule(_)
        | DraftingSemanticField::LogicalOutput(_) => None,
    }
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
    DraftingProgramRootInputScope<
        'v,
        'w,
        'a,
        DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        H,
        S,
    >
{
    /// Count every original Root-owned button/logical/numeric operation input,then reserve that
    /// complete N-pass/result population before child content,allocation or lookup. For N>0,
    /// freshly charge this exact Root/provider scope under the original whole content policy,
    /// without old scope-demand credit,then fallibly reserve N records and bind every actual
    /// original field through its existing Root source factory. One walk and N lookups fit the
    /// N*(whole-demand,(E+8)^2) envelope;each new source record consumes one reserved child slot.
    /// An actual N=0 stage performs no child pass,resource walk,record allocation or lookup.
    ///
    /// Original field/operation/ordinal/mixed-position/source loans survive. Unknown/Derived stay
    /// unchanged;this layer accepts no Derived supplier/order,Profile input,rule/output or value.
    /// No source/provider lookup is performed beyond the required resource walk. Private partial
    /// records drop on refusal;the original owners and updated/unchanged cursor remain inspectable.
    /// # Errors
    /// Complete count overflow,original child reservation,first resource cause,record reservation,
    /// then first original typed Root member lookup cause. No successful input prefix escapes.
    pub fn bind_root_input_members(
        mut self,
    ) -> DraftingProgramRootInputMembersResult<
        'v,
        'w,
        'a,
        DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        H,
        S,
    > {
        use DraftingProgramRootInputMemberRefusal as R;
        let program = self
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
        let count = program
            .semantic_dependencies()
            .try_fold(0usize, |count, use_| {
                count.checked_add(usize::from(request(use_.field()).is_some()))
            });
        let Some(count) = count else {
            return Err(DraftingProgramRootInputMemberError::new(
                self,
                None,
                None,
                R::PopulationOverflow,
            ));
        };
        if let Err(cause) = self.selections.reserve_child_passes(count) {
            return Err(DraftingProgramRootInputMemberError::new(
                self,
                Some(count),
                None,
                R::Work(cause),
            ));
        }
        if count == 0 {
            return Ok(BoundDraftingProgramRootInputMembers {
                owner: self,
                demand: None,
                records: Vec::new(),
            });
        }
        let mut meter = DesignResourceMeter::new(self.selections.limits().population().content());
        let charged = self.scope.charge_resources(&mut meter);
        let demand = match meter.finish() {
            Ok(demand) => demand,
            Err(cause) => {
                return Err(DraftingProgramRootInputMemberError::new(
                    self,
                    Some(count),
                    None,
                    R::Resources(cause),
                ))
            }
        };
        if let Err(cause) = charged {
            return Err(DraftingProgramRootInputMemberError::new(
                self,
                Some(count),
                None,
                R::Resources(cause),
            ));
        }
        let mut records = Vec::new();
        if let Err(cause) = records.try_reserve_exact(count) {
            return Err(DraftingProgramRootInputMemberError::new(
                self,
                Some(count),
                Some(demand),
                R::Reservation {
                    records: count,
                    cause,
                },
            ));
        }
        for use_ in program.semantic_dependencies() {
            let source = match request(use_.field()) {
                None => continue,
                Some(Request::Logical(target)) => match self.scope.logical_source(target) {
                    Ok(source) => DraftingProgramRootInputSource::Logical(source),
                    Err(cause) => {
                        return Err(DraftingProgramRootInputMemberError::new(
                            self,
                            Some(count),
                            Some(demand),
                            R::Lookup {
                                location: use_.selection_location(),
                                cause,
                            },
                        ))
                    }
                },
                Some(Request::Numeric(target)) => match self.scope.numeric_source(target) {
                    Ok(source) => DraftingProgramRootInputSource::Numeric(source),
                    Err(cause) => {
                        return Err(DraftingProgramRootInputMemberError::new(
                            self,
                            Some(count),
                            Some(demand),
                            R::Lookup {
                                location: use_.selection_location(),
                                cause,
                            },
                        ))
                    }
                },
            };
            records.push(DraftingProgramRootInputMember {
                operation: use_.operation(),
                ordinal: use_.ordinal(),
                position: use_.position(),
                location: use_.selection_location(),
                field: use_.field(),
                source,
            });
        }
        Ok(BoundDraftingProgramRootInputMembers {
            owner: self,
            demand: Some(demand),
            records,
        })
    }
}
