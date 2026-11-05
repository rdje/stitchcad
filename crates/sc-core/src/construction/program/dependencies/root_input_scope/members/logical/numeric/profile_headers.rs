//! Complete original Profile operation-input headers before borrowing parameter/member proofs.
use super::BoundDraftingProgramRootNumericSuppliers;
use crate::{
    construction::{
        BoundDraftingProfileSource, DraftingGarmentProvider, DraftingInitialInputProvider,
        DraftingProfileProvider, DraftingProgramSemanticInputs, DraftingSemanticField,
        DraftingSemanticSelectionLocation, DraftingSemanticSelectionRow,
    },
    design::{
        DesignMetadataHost, DesignReferenceHost, DesignResourceDemand, DesignResourceMeter,
        DesignResourceSource,
    },
    ontology::{NotionSize, ProfileParameterRef, SelectedProfilePin},
    recipe::{LogicalDerivationInputSource, SemanticOperation},
    value::LogicalInputKind,
};
use core::fmt;
mod error;
mod parameters;
pub use error::{DraftingProgramProfileHeaderError, DraftingProgramProfileHeaderRefusal};
pub use parameters::{
    AdmittedDraftingProgramRuleWork, BoundDraftingProgramGarmentProfileLogicalHeaders,
    BoundDraftingProgramGarmentProfileLogicalParameters,
    BoundDraftingProgramGarmentProfileNumericHeaders,
    BoundDraftingProgramGarmentProfileNumericParameters,
    BoundDraftingProgramGarmentRootInputMembers, BoundDraftingProgramGarmentRootLogicalSuppliers,
    BoundDraftingProgramGarmentRootNumericSuppliers, BoundDraftingProgramProfileParameters,
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
    DraftingProgramGarmentProfileNumericHeadersResult,
    DraftingProgramGarmentProfileNumericParameter,
    DraftingProgramGarmentProfileNumericParameterError,
    DraftingProgramGarmentProfileNumericParameterRefusal,
    DraftingProgramGarmentProfileNumericParametersResult,
    DraftingProgramGarmentProfileNumericRequest, DraftingProgramGarmentRootInputMember,
    DraftingProgramGarmentRootInputMemberError, DraftingProgramGarmentRootInputMemberRefusal,
    DraftingProgramGarmentRootInputMembersResult, DraftingProgramGarmentRootInputSource,
    DraftingProgramGarmentRootLogicalSupplier, DraftingProgramGarmentRootLogicalSupplierError,
    DraftingProgramGarmentRootLogicalSupplierRefusal,
    DraftingProgramGarmentRootLogicalSuppliersResult,
    DraftingProgramGarmentRootNumericSupplierError,
    DraftingProgramGarmentRootNumericSupplierRefusal,
    DraftingProgramGarmentRootNumericSuppliersResult, DraftingProgramOperationInput,
    DraftingProgramProfileInputParameter, DraftingProgramProfileParameterError,
    DraftingProgramProfileParameterRefusal, DraftingProgramProfileParametersResult,
    DraftingProgramRuleSignature, DraftingProgramRuleSignatureError,
    DraftingProgramRuleSignatureRefusal, DraftingProgramRuleSignaturesResult,
    DraftingProgramRuleSource, DraftingProgramRuleSourceError, DraftingProgramRuleSourceRefusal,
    DraftingProgramRuleSourcesResult, DraftingProgramRuleWork, DraftingProgramRuleWorkError,
    DraftingProgramRuleWorkRefusal, DraftingProgramRuleWorkRequest, DraftingProgramRuleWorkResult,
};

/// Actual original Profile operation-input request and exact already-checked selection row.
/// This preserves syntax/selection intent,not parameter membership or selected values.
#[derive(Clone, Copy)]
pub struct DraftingProgramProfileInputRequest<'w, 'a> {
    operation: &'a SemanticOperation,
    ordinal: usize,
    position: usize,
    location: DraftingSemanticSelectionLocation,
    field: DraftingSemanticField<'a>,
    row_index: usize,
    selected: &'w SelectedProfilePin,
    parameter: ProfileParameterRef,
    kind: LogicalInputKind,
}
impl<'w, 'a> DraftingProgramProfileInputRequest<'w, 'a> {
    /// Original Root semantic operation loan.
    #[must_use]
    pub const fn operation(self) -> &'a SemanticOperation {
        self.operation
    }
    /// Genuine original semantic ordinal,independent of mixed position.
    #[must_use]
    pub const fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Genuine original mixed consumer position.
    #[must_use]
    pub const fn position(self) -> usize {
        self.position
    }
    /// Original operation-input field coordinate under complete direct coverage.
    #[must_use]
    pub const fn location(self) -> DraftingSemanticSelectionLocation {
        self.location
    }
    /// Exact original typed field/parameter reference loan.
    #[must_use]
    pub const fn field(self) -> DraftingSemanticField<'a> {
        self.field
    }
    /// Original zero-based direct Profile request row.
    #[must_use]
    pub const fn row_index(self) -> usize {
        self.row_index
    }
    /// Original requested pin loan;it does not replace the provider's actual source pin.
    #[must_use]
    pub const fn selected(self) -> &'w SelectedProfilePin {
        self.selected
    }
    /// Original requested parameter ID,not a canonical declaration ID or membership proof.
    #[must_use]
    pub const fn parameter(self) -> ProfileParameterRef {
        self.parameter
    }
    /// Actual field's expected logical kind;membership/kind binding follows separately.
    #[must_use]
    pub const fn kind(self) -> LogicalInputKind {
        self.kind
    }
}
impl fmt::Debug for DraftingProgramProfileInputRequest<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramProfileInputRequest")
            .field("ordinal", &self.ordinal)
            .field("position", &self.position)
            .field("location", &self.location)
            .field("row_index", &self.row_index)
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}
/// One exact original input request and its actual accepted selected-Profile header parent.
/// This owns stable header storage;it accepts no Profile logical parameter or external recipe.
pub struct DraftingProgramProfileInputHeader<
    'w,
    'a,
    H: DesignMetadataHost,
    S: DraftingProfileProvider<H> + ?Sized,
> {
    request: DraftingProgramProfileInputRequest<'w, 'a>,
    source: BoundDraftingProfileSource<'w, H, S>,
}
impl<'w, 'a, H: DesignMetadataHost, S: DraftingProfileProvider<H> + ?Sized>
    DraftingProgramProfileInputHeader<'w, 'a, H, S>
{
    /// Actual original field/request/selection loans without copied pin text.
    #[must_use]
    pub const fn request(&self) -> DraftingProgramProfileInputRequest<'w, 'a> {
        self.request
    }
    /// Exact accepted Root/pin-label/snapshot/Profile source parent. Parameter binding must borrow it.
    #[must_use]
    pub const fn source(&self) -> &BoundDraftingProfileSource<'w, H, S> {
        &self.source
    }
}
impl<H: DesignMetadataHost, S: DraftingProfileProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramProfileInputHeader<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramProfileInputHeader")
            .field("request", &self.request)
            .finish_non_exhaustive()
    }
}
/// Complete actual Profile operation-input header class,retaining original Root proof and cursor.
/// Profile parameter/member,external Derived recipe,rule/family/cohort and execution stay separate.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<I:?Sized,H:DesignMetadataHost,S:DraftingProfileProvider<H>>(p:&mut BoundDraftingProgramProfileHeaders<'_,'_,'_,I,H,S>) {p.records.clear();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn duplicate<I:?Sized,H:DesignMetadataHost,S:DraftingProfileProvider<H>>(p:BoundDraftingProgramProfileHeaders<'_,'_,'_,I,H,S>) {let _=p.clone();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn escape<'v,'w,'a,I:?Sized,H:DesignMetadataHost,S:DraftingProfileProvider<H>>(p:BoundDraftingProgramProfileHeaders<'v,'w,'a,I,H,S>)->BoundDraftingProgramProfileHeaders<'v,'static,'a,I,H,S> {p}
/// ```
pub struct BoundDraftingProgramProfileHeaders<
    'v,
    'w,
    'a,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingProfileProvider<H> + ?Sized,
> {
    root: BoundDraftingProgramRootNumericSuppliers<'v, 'w, 'a, I, H, S>,
    demand: Option<DesignResourceDemand>,
    records: Vec<DraftingProgramProfileInputHeader<'w, 'a, H, S>>,
}
impl<'v, 'w, 'a, I: ?Sized, H: DesignMetadataHost, S: DraftingProfileProvider<H> + ?Sized>
    BoundDraftingProgramProfileHeaders<'v, 'w, 'a, I, H, S>
{
    /// Complete original Root member/supplier/source/key owners and the same unique finite cursor.
    #[must_use]
    pub const fn root(&self) -> &BoundDraftingProgramRootNumericSuppliers<'v, 'w, 'a, I, H, S> {
        &self.root
    }
    /// Fresh whole aggregate/proof demand before header work,or None for N0.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.demand
    }
    /// Every original Profile operation-input header,in genuine mixed/local order without deduplication.
    #[must_use]
    pub fn records(&self) -> &[DraftingProgramProfileInputHeader<'w, 'a, H, S>] {
        &self.records
    }
    /// Reserved stage passes:zero for N0,otherwise N+1 for whole admission plus N child headers.
    #[must_use]
    pub fn child_pass_count(&self) -> usize {
        if self.records.is_empty() {
            0
        } else {
            self.records.len() + 1
        }
    }
}
impl<I: ?Sized, H: DesignMetadataHost, S: DraftingProfileProvider<H> + ?Sized> fmt::Debug
    for BoundDraftingProgramProfileHeaders<'_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProgramProfileHeaders")
            .field("records", &self.records.len())
            .finish_non_exhaustive()
    }
}
/// Complete original Profile input headers,or original Root proof/cursor and first typed refusal.
pub type DraftingProgramProfileHeadersResult<'v, 'w, 'a, I, H, S> = Result<
    BoundDraftingProgramProfileHeaders<'v, 'w, 'a, I, H, S>,
    DraftingProgramProfileHeaderError<'v, 'w, 'a, I, H, S>,
>;
fn request(field: DraftingSemanticField<'_>) -> Option<(ProfileParameterRef, LogicalInputKind)> {
    match field {
        DraftingSemanticField::ButtonholeButton(NotionSize::Profile(parameter)) => {
            Some((*parameter, LogicalInputKind::NotionSize))
        }
        DraftingSemanticField::LogicalInput { input, .. } => match input.source {
            LogicalDerivationInputSource::Profile { parameter, kind } => Some((parameter, kind)),
            LogicalDerivationInputSource::Logical(_) | LogicalDerivationInputSource::Numeric(_) => {
                None
            }
        },
        DraftingSemanticField::ButtonholeButton(NotionSize::Declaration(_))
        | DraftingSemanticField::ButtonholePolicy(_)
        | DraftingSemanticField::ButtonholeInput { .. }
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
        S: DraftingInitialInputProvider<H>
            + DraftingGarmentProvider<H>
            + DraftingProfileProvider<H>
            + ?Sized,
    >
    BoundDraftingProgramRootNumericSuppliers<
        'v,
        'w,
        'a,
        DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        H,
        S,
    >
{
    /// Count every actual Profile-owned button/logical input and reserve N+1 passes/results for
    /// N>0 before fresh full aggregate/proof admission,fallible N-record reservation and any query.
    /// Each child reuses its original checked direct row/pin and existing fresh header factory.
    /// The extra pass covers whole admission;each of N child passes covers one fresh Root/provider/
    /// request walk and its bounded header join. No previous demand supplies content/work credit.
    /// N0 performs no new reservation,content walk,allocation or query. No Profile parameter/member,
    /// external Derived order,rule origin or whole semantic acceptance follows from header success.
    /// # Errors
    /// Complete arithmetic,atomic remaining cursor,first fresh aggregate content,allocator,then
    /// first actual child header cause with original operation/field/row/pin witnesses. Private
    /// successful header prefixes drop;the original Root proof/cursor and first cause survive.
    pub fn bind_profile_input_headers(
        mut self,
    ) -> DraftingProgramProfileHeadersResult<
        'v,
        'w,
        'a,
        DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        H,
        S,
    > {
        use DraftingProgramProfileHeaderRefusal as R;
        let inputs = self.logical.members.owner.inputs();
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
        let count = program
            .semantic_dependencies()
            .try_fold(0usize, |count, use_| {
                count.checked_add(usize::from(request(use_.field()).is_some()))
            });
        let Some(count) = count else {
            return Err(DraftingProgramProfileHeaderError::new(
                self,
                None,
                None,
                R::PopulationOverflow,
            ));
        };
        let passes = if count == 0 {
            Some(0)
        } else {
            count.checked_add(1)
        };
        let Some(passes) = passes else {
            return Err(DraftingProgramProfileHeaderError::new(
                self,
                Some(count),
                None,
                R::PopulationOverflow,
            ));
        };
        if let Err(cause) = self
            .logical
            .members
            .owner
            .selections
            .reserve_child_passes(passes)
        {
            return Err(DraftingProgramProfileHeaderError::new(
                self,
                Some(count),
                None,
                R::Work(cause),
            ));
        }
        if count == 0 {
            return Ok(BoundDraftingProgramProfileHeaders {
                root: self,
                demand: None,
                records: Vec::new(),
            });
        }
        let content = self
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
            Ok(demand) => demand,
            Err(cause) => {
                return Err(DraftingProgramProfileHeaderError::new(
                    self,
                    Some(count),
                    None,
                    R::Resources(cause),
                ))
            }
        };
        if let Err(cause) = charged {
            return Err(DraftingProgramProfileHeaderError::new(
                self,
                Some(count),
                None,
                R::Resources(cause),
            ));
        }
        let mut records = Vec::new();
        if let Err(cause) = records.try_reserve_exact(count) {
            return Err(DraftingProgramProfileHeaderError::new(
                self,
                Some(count),
                Some(demand),
                R::Reservation {
                    records: count,
                    cause,
                },
            ));
        }
        let rows = inputs.selections().inputs().rows();
        for use_ in program.semantic_dependencies() {
            let Some((parameter, kind)) = request(use_.field()) else {
                continue;
            };
            let location = use_.selection_location();
            let row_index = self
                .logical
                .members
                .owner
                .selections
                .associations()
                .iter()
                .find(|association| association.location() == location)
                .and_then(|association| association.row_index())
                .unwrap_or_else(|| {
                    unreachable!("complete direct Profile input coverage retains its original row")
                });
            let Some(DraftingSemanticSelectionRow::Profile { selected, .. }) = rows.get(row_index)
            else {
                unreachable!(
                    "complete checked Profile key retains the actual Profile selection row"
                )
            };
            let request = DraftingProgramProfileInputRequest {
                operation: use_.operation(),
                ordinal: use_.ordinal(),
                position: use_.position(),
                location,
                field: use_.field(),
                row_index,
                selected,
                parameter,
                kind,
            };
            let source = match self
                .logical
                .members
                .owner
                .scope
                .context()
                .bind_profile_source(selected, content)
            {
                Ok(source) => source,
                Err(cause) => {
                    return Err(DraftingProgramProfileHeaderError::new(
                        self,
                        Some(count),
                        Some(demand),
                        R::Source { request, cause },
                    ))
                }
            };
            records.push(DraftingProgramProfileInputHeader { request, source });
        }
        Ok(BoundDraftingProgramProfileHeaders {
            root: self,
            demand: Some(demand),
            records,
        })
    }
}
