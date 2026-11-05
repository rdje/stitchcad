//! Every original logical Profile/Root owner/cursor and first typed cause,without a successful Profile header prefix.
use super::{
    BoundDraftingProgramGarmentProfileLogicalParameters,
    DraftingProgramGarmentProfileNumericRequest,
};
use crate::{
    construction::{
        DraftingProfileLogicalProvider, DraftingProfileNumericProvider, DraftingProfileSourceError,
        DraftingProgramSemanticWorkError, DraftingRuleProvider,
    },
    design::{DesignMetadataHost, DesignResourceDemand, DesignResourceLimitError},
};
use core::fmt;
use std::collections::TryReserveError;
/// First original complete Profile-header stage refusal.
pub enum DraftingProgramGarmentProfileNumericHeaderRefusal<
    'w,
    'a,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingProfileNumericProvider<H>
        + DraftingProfileLogicalProvider<H>
        + DraftingRuleProvider<H>
        + ?Sized,
> {
    /// Complete actual eligible Profile-input count cannot fit usize,before child work.
    PopulationOverflow,
    /// Original unique child cursor refuses the complete N+1-pass/result reservation unchanged.
    Work(DraftingProgramSemanticWorkError<'w, I>),
    /// First fresh actual aggregate/proof content field cause,before allocation or member lookup.
    Resources(DesignResourceLimitError),
    /// Complete admitted record population could not be reserved.
    Reservation {
        /// Original complete record count.
        records: usize,
        /// Original allocator cause.
        cause: TryReserveError,
    },
    /// First original header source cause,with the actual consumer field and original row/pin.
    Source {
        /// Exact original garment-input and direct-selection witnesses.
        request: DraftingProgramGarmentProfileNumericRequest<'w, 'a, H>,
        /// Original child header factory cause and returned source witnesses.
        cause: DraftingProfileSourceError<'w, H, S>,
    },
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileNumericProvider<H>
            + DraftingProfileLogicalProvider<H>
            + DraftingRuleProvider<H>
            + ?Sized,
    > fmt::Debug for DraftingProgramGarmentProfileNumericHeaderRefusal<'_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PopulationOverflow => f.write_str("PopulationOverflow"),
            Self::Work(cause) => f.debug_tuple("Work").field(cause).finish(),
            Self::Resources(cause) => f.debug_tuple("Resources").field(cause).finish(),
            Self::Reservation { records, cause } => f
                .debug_struct("Reservation")
                .field("records", records)
                .field("cause", cause)
                .finish(),
            Self::Source { request, cause } => f
                .debug_struct("Source")
                .field("request", request)
                .field("cause", cause)
                .finish(),
        }
    }
}
/// Every original consumer/source/key owner and unique cursor,complete stage metadata and first
/// typed cause. Private partial Profile header vectors are discarded;only the refused diagnostic boxes.
pub struct DraftingProgramGarmentProfileNumericHeaderError<
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
    details: Box<Failure<'t, 'z, 'q, 'h, 'v, 'w, 'a, I, H, S>>,
}
struct Failure<
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
    owner: BoundDraftingProgramGarmentProfileLogicalParameters<'t, 'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
    count: Option<usize>,
    demand: Option<DesignResourceDemand>,
    refusal: DraftingProgramGarmentProfileNumericHeaderRefusal<'w, 'a, I, H, S>,
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
    > DraftingProgramGarmentProfileNumericHeaderError<'t, 'z, 'q, 'h, 'v, 'w, 'a, I, H, S>
{
    pub(super) fn new(
        owner: BoundDraftingProgramGarmentProfileLogicalParameters<
            't,
            'z,
            'q,
            'h,
            'v,
            'w,
            'a,
            I,
            H,
            S,
        >,
        count: Option<usize>,
        demand: Option<DesignResourceDemand>,
        refusal: DraftingProgramGarmentProfileNumericHeaderRefusal<'w, 'a, I, H, S>,
    ) -> Self {
        Self {
            details: Box::new(Failure {
                owner,
                count,
                demand,
                refusal,
            }),
        }
    }
    /// Exact original consumer/key/source owners and unchanged or consumed reservation state.
    #[must_use]
    pub const fn owner(
        &self,
    ) -> &BoundDraftingProgramGarmentProfileLogicalParameters<'t, 'z, 'q, 'h, 'v, 'w, 'a, I, H, S>
    {
        &self.details.owner
    }
    /// Complete actual Profile-input population,only after counting completed.
    #[must_use]
    pub const fn header_count(&self) -> Option<usize> {
        self.details.count
    }
    /// Complete fresh aggregate/proof demand only after the entire resource stage passed.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.details.demand
    }
    /// Original first typed cause and its actual original consumer/request/member witnesses.
    #[must_use]
    pub const fn refusal(
        &self,
    ) -> &DraftingProgramGarmentProfileNumericHeaderRefusal<'w, 'a, I, H, S> {
        &self.details.refusal
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
    for DraftingProgramGarmentProfileNumericHeaderError<'_, '_, '_, '_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramGarmentProfileNumericHeaderError")
            .field("refusal", &self.details.refusal)
            .finish_non_exhaustive()
    }
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileNumericProvider<H>
            + DraftingProfileLogicalProvider<H>
            + DraftingRuleProvider<H>
            + ?Sized,
    > fmt::Display
    for DraftingProgramGarmentProfileNumericHeaderError<'_, '_, '_, '_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "drafting garment Profile numeric headers: {:?}",
            self.details.refusal
        )
    }
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileNumericProvider<H>
            + DraftingProfileLogicalProvider<H>
            + DraftingRuleProvider<H>
            + ?Sized,
    > std::error::Error
    for DraftingProgramGarmentProfileNumericHeaderError<'_, '_, '_, '_, '_, '_, '_, I, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.details.refusal {
            DraftingProgramGarmentProfileNumericHeaderRefusal::Resources(cause) => Some(cause),
            DraftingProgramGarmentProfileNumericHeaderRefusal::Reservation {
                records: _,
                cause,
            } => Some(cause),
            DraftingProgramGarmentProfileNumericHeaderRefusal::Work(cause) => {
                std::error::Error::source(cause)
            }
            DraftingProgramGarmentProfileNumericHeaderRefusal::Source { cause, .. } => {
                std::error::Error::source(cause)
            }
            _ => None,
        }
    }
}
