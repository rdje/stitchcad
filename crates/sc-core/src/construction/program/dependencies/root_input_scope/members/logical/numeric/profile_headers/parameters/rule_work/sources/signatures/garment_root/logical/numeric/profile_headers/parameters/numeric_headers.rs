//! Complete original numeric Profile garment headers under exact original direct selection pins.
use super::BoundDraftingProgramGarmentProfileLogicalParameters;
use crate::{
    construction::{
        BoundDraftingProfileSource, DraftingGarmentProvider, DraftingGarmentSemanticField,
        DraftingGarmentSemanticUse, DraftingInitialInputProvider, DraftingProfileLogicalProvider,
        DraftingProfileNumericProvider, DraftingProgramSemanticInputs, DraftingRuleProvider,
        DraftingSemanticSelectionRow,
    },
    design::{
        DesignMetadataHost, DesignReferenceHost, DesignResourceDemand, DesignResourceMeter,
        DesignResourceSource,
    },
    ontology::{ClosureLength, ProfileParameterRef, SelectedProfilePin},
    value::ScalarInputKind,
};
use core::fmt;
mod error;
pub use error::{
    DraftingProgramGarmentProfileNumericHeaderError,
    DraftingProgramGarmentProfileNumericHeaderRefusal,
};
/// One actual original Profile numeric input field and its exact checked direct row/pin intent.
pub struct DraftingProgramGarmentProfileNumericRequest<'w, 'a, H: DesignMetadataHost> {
    occurrence: DraftingGarmentSemanticUse<'a, H>,
    row_index: usize,
    selected: &'w SelectedProfilePin,
    parameter: ProfileParameterRef,
    kind: ScalarInputKind,
}
impl<H: DesignMetadataHost> Copy for DraftingProgramGarmentProfileNumericRequest<'_, '_, H> {}
impl<H: DesignMetadataHost> Clone for DraftingProgramGarmentProfileNumericRequest<'_, '_, H> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'w, 'a, H: DesignMetadataHost> DraftingProgramGarmentProfileNumericRequest<'w, 'a, H> {
    /// Exact original garment/owner/index/field and genuine Root location,without a mixed position.
    #[must_use]
    pub const fn occurrence(self) -> DraftingGarmentSemanticUse<'a, H> {
        self.occurrence
    }
    /// Original zero-based checked direct Profile row,independent of occurrence order.
    #[must_use]
    pub const fn row_index(self) -> usize {
        self.row_index
    }
    /// Original requested pin loan,not a substituted provider pin or copied string record.
    #[must_use]
    pub const fn selected(self) -> &'w SelectedProfilePin {
        self.selected
    }
    /// Original requested parameter ID,not a canonical declaration or membership proof.
    #[must_use]
    pub const fn parameter(self) -> ProfileParameterRef {
        self.parameter
    }
    /// Actual numeric field's expected kind;source header acceptance does not bind the member.
    #[must_use]
    pub const fn kind(self) -> ScalarInputKind {
        self.kind
    }
}
impl<H: DesignMetadataHost> fmt::Debug for DraftingProgramGarmentProfileNumericRequest<'_, '_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramGarmentProfileNumericRequest")
            .field("occurrence", &self.occurrence)
            .field("row_index", &self.row_index)
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}
/// One exact original garment input/request and its stable accepted original Profile header parent.
pub struct DraftingProgramGarmentProfileNumericHeader<
    'w,
    'a,
    H: DesignMetadataHost,
    S: DraftingRuleProvider<H> + ?Sized,
> {
    request: DraftingProgramGarmentProfileNumericRequest<'w, 'a, H>,
    source: BoundDraftingProfileSource<'w, H, S>,
}
impl<'w, 'a, H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized>
    DraftingProgramGarmentProfileNumericHeader<'w, 'a, H, S>
{
    /// Original field/request/row/pin intent without cloned text or source data.
    #[must_use]
    pub const fn request(&self) -> DraftingProgramGarmentProfileNumericRequest<'w, 'a, H> {
        self.request
    }
    /// Actual accepted original Root/pin-label/snapshot/Profile header,not parameter membership.
    #[must_use]
    pub const fn source(&self) -> &BoundDraftingProfileSource<'w, H, S> {
        &self.source
    }
}

impl<H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramGarmentProfileNumericHeader<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramGarmentProfileNumericHeader")
            .field("request", &self.request)
            .finish_non_exhaustive()
    }
}
/// Complete actual Profile zipper header class,with all prior original logical/Root input owners.
/// Numeric parameter/member,external Derived and whole cohort acceptance stay separate.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<I:?Sized,H:DesignMetadataHost,S:DraftingProfileNumericProvider<H>+DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:&mut BoundDraftingProgramGarmentProfileNumericHeaders<'_,'_,'_,'_,'_,'_,'_,I,H,S>) {p.records.clear();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn duplicate<I:?Sized,H:DesignMetadataHost,S:DraftingProfileNumericProvider<H>+DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:BoundDraftingProgramGarmentProfileNumericHeaders<'_,'_,'_,'_,'_,'_,'_,I,H,S>) {let _=p.clone();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn escape<'t,'z,'q,'h,'v,'w,'a,I:?Sized,H:DesignMetadataHost,S:DraftingProfileNumericProvider<H>+DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:BoundDraftingProgramGarmentProfileNumericHeaders<'t,'z,'q,'h,'v,'w,'a,I,H,S>)->BoundDraftingProgramGarmentProfileNumericHeaders<'t,'static,'q,'h,'v,'w,'a,I,H,S> {p}
/// ```
pub struct BoundDraftingProgramGarmentProfileNumericHeaders<
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
    logical:
        BoundDraftingProgramGarmentProfileLogicalParameters<'t, 'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
    demand: Option<DesignResourceDemand>,
    records: Vec<DraftingProgramGarmentProfileNumericHeader<'w, 'a, H, S>>,
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
        S: DraftingProfileNumericProvider<H>
            + DraftingProfileLogicalProvider<H>
            + DraftingRuleProvider<H>
            + ?Sized,
    > BoundDraftingProgramGarmentProfileNumericHeaders<'t, 'z, 'q, 'h, 'v, 'w, 'a, I, H, S>
{
    /// Complete original logical Profile parameter/header and Root owners with the same unique cursor.
    #[must_use]
    pub const fn logical(
        &self,
    ) -> &BoundDraftingProgramGarmentProfileLogicalParameters<'t, 'z, 'q, 'h, 'v, 'w, 'a, I, H, S>
    {
        &self.logical
    }
    /// Fresh complete aggregate/proof demand before nonempty header work;None for N0.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.demand
    }
    /// Every actual numeric Profile garment-input header once,in original garment/local order.
    #[must_use]
    pub fn records(&self) -> &[DraftingProgramGarmentProfileNumericHeader<'w, 'a, H, S>] {
        &self.records
    }
    /// Zero for N0,otherwise N+1 fresh whole/header pass/result envelopes.
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
    for BoundDraftingProgramGarmentProfileNumericHeaders<'_, '_, '_, '_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProgramGarmentProfileNumericHeaders")
            .field("records", &self.records.len())
            .finish_non_exhaustive()
    }
}
/// Complete original numeric Profile headers or original complete logical/Root owners and first cause.
pub type DraftingProgramGarmentProfileNumericHeadersResult<'t, 'z, 'q, 'h, 'v, 'w, 'a, I, H, S> =
    Result<
        BoundDraftingProgramGarmentProfileNumericHeaders<'t, 'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
        DraftingProgramGarmentProfileNumericHeaderError<'t, 'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
    >;
fn request(
    field: DraftingGarmentSemanticField<'_>,
) -> Option<(ProfileParameterRef, ScalarInputKind)> {
    match field {
        DraftingGarmentSemanticField::ClosureLength(ClosureLength::Profile(p)) => {
            Some((*p, ScalarInputKind::Length))
        }
        DraftingGarmentSemanticField::PocketOpening(_)
        | DraftingGarmentSemanticField::ClosureSize { .. }
        | DraftingGarmentSemanticField::ClosureLength(_)
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
    BoundDraftingProgramGarmentProfileLogicalParameters<
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
    /// Count every actual Profile zipper-length field and atomically reserve
    /// N+1 whole/header passes for N>0 before fresh full aggregate/proof charging,complete N-record
    /// reservation or query. Reuse each exact checked direct row/pin and existing bind_profile_source,
    /// which freshly charges Root/provider/request before actual Root/pin-label/snapshot checks.
    /// Retain stable actual parents for later original numeric metadata/member loans. Prior complete
    /// logical/Root input proofs survive;no numeric member/value/external Derived or whole acceptance occurs.
    /// N0 adds no walk,work,allocation,query,reservation or new demand claim.
    /// # Errors
    /// Complete arithmetic,atomic cursor shortage,fresh aggregate resources,allocator,then first
    /// original child header cause. Private successful prefixes drop;original Root/cursor and actual
    /// requiring garment/field/row/pin/source witnesses remain inspectable.
    pub fn bind_garment_profile_numeric_headers(
        self,
    ) -> DraftingProgramGarmentProfileNumericHeadersResult<
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
        use DraftingProgramGarmentProfileNumericHeaderRefusal as R;
        let inputs = self
            .root
            .logical
            .members
            .owner
            .inputs
            .root
            .logical
            .members
            .owner
            .inputs();
        let garment = inputs.garment().source().garment;
        let count = garment
            .pocket_closure_dependencies()
            .try_fold(0usize, |n, u| {
                n.checked_add(usize::from(request(u.field()).is_some()))
            });
        let Some(count) = count else {
            return Err(DraftingProgramGarmentProfileNumericHeaderError::new(
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
            return Err(DraftingProgramGarmentProfileNumericHeaderError::new(
                self,
                Some(count),
                None,
                R::PopulationOverflow,
            ));
        };
        if let Err(cause) = self
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
            return Err(DraftingProgramGarmentProfileNumericHeaderError::new(
                self,
                Some(count),
                None,
                R::Work(cause),
            ));
        }
        if count == 0 {
            return Ok(BoundDraftingProgramGarmentProfileNumericHeaders {
                logical: self,
                demand: None,
                records: Vec::new(),
            });
        }
        let content = self
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
        let charged = inputs.charge_resources(&mut meter);
        let demand = match meter.finish() {
            Ok(d) => d,
            Err(cause) => {
                return Err(DraftingProgramGarmentProfileNumericHeaderError::new(
                    self,
                    Some(count),
                    None,
                    R::Resources(cause),
                ))
            }
        };
        if let Err(cause) = charged {
            return Err(DraftingProgramGarmentProfileNumericHeaderError::new(
                self,
                Some(count),
                None,
                R::Resources(cause),
            ));
        }
        let mut records = Vec::new();
        if let Err(cause) = records.try_reserve_exact(count) {
            return Err(DraftingProgramGarmentProfileNumericHeaderError::new(
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
        for occurrence in garment.pocket_closure_dependencies() {
            let Some((parameter, kind)) = request(occurrence.field()) else {
                continue;
            };
            let location = occurrence.selection_location();
            let row_index = self
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
                .associations()
                .iter()
                .find(|a| a.location() == location)
                .and_then(|a| a.row_index())
                .unwrap_or_else(|| {
                    unreachable!(
                        "complete direct Profile numeric garment coverage retains the actual row"
                    )
                });
            let Some(DraftingSemanticSelectionRow::Profile { selected, .. }) = rows.get(row_index)
            else {
                unreachable!("complete checked Profile key retains the actual original Profile row")
            };
            let request = DraftingProgramGarmentProfileNumericRequest {
                occurrence,
                row_index,
                selected,
                parameter,
                kind,
            };
            let source = match self
                .root
                .logical
                .members
                .owner
                .inputs
                .root
                .logical
                .members
                .owner
                .scope
                .context()
                .bind_profile_source(selected, content)
            {
                Ok(s) => s,
                Err(cause) => {
                    return Err(DraftingProgramGarmentProfileNumericHeaderError::new(
                        self,
                        Some(count),
                        Some(demand),
                        R::Source { request, cause },
                    ))
                }
            };
            records.push(DraftingProgramGarmentProfileNumericHeader { request, source });
        }
        Ok(BoundDraftingProgramGarmentProfileNumericHeaders {
            logical: self,
            demand: Some(demand),
            records,
        })
    }
}

mod parameters;
pub use parameters::{
    BoundDraftingProgramGarmentProfileNumericParameters,
    DraftingProgramGarmentProfileNumericParameter,
    DraftingProgramGarmentProfileNumericParameterError,
    DraftingProgramGarmentProfileNumericParameterRefusal,
    DraftingProgramGarmentProfileNumericParametersResult,
};
