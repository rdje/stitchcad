//! Complete original logical Profile garment headers under exact original direct selection pins.
use super::BoundDraftingProgramGarmentRootNumericSuppliers;
use crate::{
    construction::{
        BoundDraftingProfileSource, DraftingGarmentProvider, DraftingGarmentSemanticField,
        DraftingGarmentSemanticUse, DraftingInitialInputProvider, DraftingProfileLogicalProvider,
        DraftingProgramSemanticInputs, DraftingRuleProvider, DraftingSemanticSelectionRow,
    },
    design::{
        DesignMetadataHost, DesignReferenceHost, DesignResourceDemand, DesignResourceMeter,
        DesignResourceSource,
    },
    ontology::{NotionSize, PocketOpening, ProfileParameterRef, SelectedProfilePin},
    value::LogicalInputKind,
};
use core::fmt;
mod error;
mod parameters;
pub use error::{
    DraftingProgramGarmentProfileLogicalHeaderError,
    DraftingProgramGarmentProfileLogicalHeaderRefusal,
};
pub use parameters::{
    BoundDraftingProgramGarmentProfileLogicalParameters,
    BoundDraftingProgramGarmentProfileNumericHeaders,
    DraftingProgramGarmentProfileLogicalParameter,
    DraftingProgramGarmentProfileLogicalParameterError,
    DraftingProgramGarmentProfileLogicalParameterRefusal,
    DraftingProgramGarmentProfileLogicalParametersResult,
    DraftingProgramGarmentProfileNumericHeader, DraftingProgramGarmentProfileNumericHeaderError,
    DraftingProgramGarmentProfileNumericHeaderRefusal,
    DraftingProgramGarmentProfileNumericHeadersResult, DraftingProgramGarmentProfileNumericRequest,
};
/// One actual original Profile logical input field and its exact checked direct row/pin intent.
pub struct DraftingProgramGarmentProfileLogicalRequest<'w, 'a, H: DesignMetadataHost> {
    occurrence: DraftingGarmentSemanticUse<'a, H>,
    row_index: usize,
    selected: &'w SelectedProfilePin,
    parameter: ProfileParameterRef,
    kind: LogicalInputKind,
}
impl<H: DesignMetadataHost> Copy for DraftingProgramGarmentProfileLogicalRequest<'_, '_, H> {}
impl<H: DesignMetadataHost> Clone for DraftingProgramGarmentProfileLogicalRequest<'_, '_, H> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'w, 'a, H: DesignMetadataHost> DraftingProgramGarmentProfileLogicalRequest<'w, 'a, H> {
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
    /// Actual logical field's expected kind;source header acceptance does not bind the member.
    #[must_use]
    pub const fn kind(self) -> LogicalInputKind {
        self.kind
    }
}
impl<H: DesignMetadataHost> fmt::Debug for DraftingProgramGarmentProfileLogicalRequest<'_, '_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramGarmentProfileLogicalRequest")
            .field("occurrence", &self.occurrence)
            .field("row_index", &self.row_index)
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}
/// One exact original garment input/request and its stable accepted original Profile header parent.
pub struct DraftingProgramGarmentProfileLogicalHeader<
    'w,
    'a,
    H: DesignMetadataHost,
    S: DraftingRuleProvider<H> + ?Sized,
> {
    request: DraftingProgramGarmentProfileLogicalRequest<'w, 'a, H>,
    source: BoundDraftingProfileSource<'w, H, S>,
}
impl<'w, 'a, H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized>
    DraftingProgramGarmentProfileLogicalHeader<'w, 'a, H, S>
{
    /// Original field/request/row/pin intent without cloned text or source data.
    #[must_use]
    pub const fn request(&self) -> DraftingProgramGarmentProfileLogicalRequest<'w, 'a, H> {
        self.request
    }
    /// Actual accepted original Root/pin-label/snapshot/Profile header,not parameter membership.
    #[must_use]
    pub const fn source(&self) -> &BoundDraftingProfileSource<'w, H, S> {
        &self.source
    }
}
impl<H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramGarmentProfileLogicalHeader<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramGarmentProfileLogicalHeader")
            .field("request", &self.request)
            .finish_non_exhaustive()
    }
}
/// Complete actual Profile opening/notion-size header class,with all original Root supplier owners.
/// Parameter/member,external Derived,numeric Profile length and whole cohort acceptance stay separate.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:&mut BoundDraftingProgramGarmentProfileLogicalHeaders<'_,'_,'_,'_,'_,'_,I,H,S>) {p.records.clear();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn duplicate<I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:BoundDraftingProgramGarmentProfileLogicalHeaders<'_,'_,'_,'_,'_,'_,I,H,S>) {let _=p.clone();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn escape<'z,'q,'h,'v,'w,'a,I:?Sized,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>+DraftingRuleProvider<H>>(p:BoundDraftingProgramGarmentProfileLogicalHeaders<'z,'q,'h,'v,'w,'a,I,H,S>)->BoundDraftingProgramGarmentProfileLogicalHeaders<'static,'q,'h,'v,'w,'a,I,H,S> {p}
/// ```
pub struct BoundDraftingProgramGarmentProfileLogicalHeaders<
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
    root: BoundDraftingProgramGarmentRootNumericSuppliers<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
    demand: Option<DesignResourceDemand>,
    records: Vec<DraftingProgramGarmentProfileLogicalHeader<'w, 'a, H, S>>,
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
    > BoundDraftingProgramGarmentProfileLogicalHeaders<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>
{
    /// Complete original Root member/supplier/operation owners and same unique finite cursor.
    #[must_use]
    pub const fn root(
        &self,
    ) -> &BoundDraftingProgramGarmentRootNumericSuppliers<'z, 'q, 'h, 'v, 'w, 'a, I, H, S> {
        &self.root
    }
    /// Fresh complete aggregate/proof demand before nonempty header work;None for N0.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.demand
    }
    /// Every actual logical Profile garment-input header once,in original garment/local order.
    #[must_use]
    pub fn records(&self) -> &[DraftingProgramGarmentProfileLogicalHeader<'w, 'a, H, S>] {
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
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > fmt::Debug
    for BoundDraftingProgramGarmentProfileLogicalHeaders<'_, '_, '_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProgramGarmentProfileLogicalHeaders")
            .field("records", &self.records.len())
            .finish_non_exhaustive()
    }
}
/// Complete original logical Profile headers or original complete Root/cursor and first cause.
pub type DraftingProgramGarmentProfileLogicalHeadersResult<'z, 'q, 'h, 'v, 'w, 'a, I, H, S> =
    Result<
        BoundDraftingProgramGarmentProfileLogicalHeaders<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
        DraftingProgramGarmentProfileLogicalHeaderError<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
    >;
fn request(
    field: DraftingGarmentSemanticField<'_>,
) -> Option<(ProfileParameterRef, LogicalInputKind)> {
    match field {
        DraftingGarmentSemanticField::PocketOpening(PocketOpening::Profile(p)) => {
            Some((*p, LogicalInputKind::PocketOpening))
        }
        DraftingGarmentSemanticField::ClosureSize {
            source: NotionSize::Profile(p),
            ..
        } => Some((*p, LogicalInputKind::NotionSize)),
        DraftingGarmentSemanticField::PocketOpening(PocketOpening::Declaration(_))
        | DraftingGarmentSemanticField::ClosureSize {
            source: NotionSize::Declaration(_),
            ..
        }
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
    BoundDraftingProgramGarmentRootNumericSuppliers<
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
    /// Count every actual Profile Pocket opening/Closure notion-size field and atomically reserve
    /// N+1 whole/header passes for N>0 before fresh full aggregate/proof charging,complete N-record
    /// reservation or query. Reuse each exact checked direct row/pin and existing bind_profile_source,
    /// which freshly charges Root/provider/request before actual Root/pin-label/snapshot checks.
    /// Retain stable actual parents for later parameter loans. Profile numeric lengths remain outside
    /// this logical class;no parameter/member/value/external Derived or whole acceptance occurs.
    /// N0 adds no walk,work,allocation,query,reservation or new demand claim.
    /// # Errors
    /// Complete arithmetic,atomic cursor shortage,fresh aggregate resources,allocator,then first
    /// original child header cause. Private successful prefixes drop;original Root/cursor and actual
    /// requiring garment/field/row/pin/source witnesses remain inspectable.
    pub fn bind_garment_profile_logical_headers(
        self,
    ) -> DraftingProgramGarmentProfileLogicalHeadersResult<
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
        use DraftingProgramGarmentProfileLogicalHeaderRefusal as R;
        let inputs = self
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
            return Err(DraftingProgramGarmentProfileLogicalHeaderError::new(
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
            return Err(DraftingProgramGarmentProfileLogicalHeaderError::new(
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
            .inputs
            .root
            .logical
            .members
            .owner
            .selections
            .reserve_child_passes(passes)
        {
            return Err(DraftingProgramGarmentProfileLogicalHeaderError::new(
                self,
                Some(count),
                None,
                R::Work(cause),
            ));
        }
        if count == 0 {
            return Ok(BoundDraftingProgramGarmentProfileLogicalHeaders {
                root: self,
                demand: None,
                records: Vec::new(),
            });
        }
        let content = self
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
                return Err(DraftingProgramGarmentProfileLogicalHeaderError::new(
                    self,
                    Some(count),
                    None,
                    R::Resources(cause),
                ))
            }
        };
        if let Err(cause) = charged {
            return Err(DraftingProgramGarmentProfileLogicalHeaderError::new(
                self,
                Some(count),
                None,
                R::Resources(cause),
            ));
        }
        let mut records = Vec::new();
        if let Err(cause) = records.try_reserve_exact(count) {
            return Err(DraftingProgramGarmentProfileLogicalHeaderError::new(
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
                        "complete direct Profile logical garment coverage retains the actual row"
                    )
                });
            let Some(DraftingSemanticSelectionRow::Profile { selected, .. }) = rows.get(row_index)
            else {
                unreachable!("complete checked Profile key retains the actual original Profile row")
            };
            let request = DraftingProgramGarmentProfileLogicalRequest {
                occurrence,
                row_index,
                selected,
                parameter,
                kind,
            };
            let source = match self
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
                    return Err(DraftingProgramGarmentProfileLogicalHeaderError::new(
                        self,
                        Some(count),
                        Some(demand),
                        R::Source { request, cause },
                    ))
                }
            };
            records.push(DraftingProgramGarmentProfileLogicalHeader { request, source });
        }
        Ok(BoundDraftingProgramGarmentProfileLogicalHeaders {
            root: self,
            demand: Some(demand),
            records,
        })
    }
}
