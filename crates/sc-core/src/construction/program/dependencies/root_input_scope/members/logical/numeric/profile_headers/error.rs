//! Every original Root owner/cursor and first typed cause,without a successful Profile header prefix.
use super::{super::BoundDraftingProgramRootNumericSuppliers, DraftingProgramProfileInputRequest};
use crate::{
    construction::{
        DraftingProfileProvider, DraftingProfileSourceError, DraftingProgramSemanticWorkError,
    },
    design::{DesignMetadataHost, DesignResourceDemand, DesignResourceLimitError},
};
use core::fmt;
use std::collections::TryReserveError;
/// First original complete Profile-header stage refusal.
pub enum DraftingProgramProfileHeaderRefusal<
    'w,
    'a,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingProfileProvider<H> + ?Sized,
> {
    /// Complete actual eligible Profile-input count cannot fit usize,before child work.
    PopulationOverflow,
    /// Original unique child cursor refuses the complete N-pass/result reservation unchanged.
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
        /// Exact original operation-input and direct-selection witnesses.
        request: DraftingProgramProfileInputRequest<'w, 'a>,
        /// Original child header factory cause and returned source witnesses.
        cause: DraftingProfileSourceError<'w, H, S>,
    },
}
impl<I: ?Sized, H: DesignMetadataHost, S: DraftingProfileProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramProfileHeaderRefusal<'_, '_, I, H, S>
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
pub struct DraftingProgramProfileHeaderError<
    'v,
    'w,
    'a,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingProfileProvider<H> + ?Sized,
> {
    details: Box<Failure<'v, 'w, 'a, I, H, S>>,
}
struct Failure<'v, 'w, 'a, I: ?Sized, H: DesignMetadataHost, S: DraftingProfileProvider<H> + ?Sized>
{
    owner: BoundDraftingProgramRootNumericSuppliers<'v, 'w, 'a, I, H, S>,
    count: Option<usize>,
    demand: Option<DesignResourceDemand>,
    refusal: DraftingProgramProfileHeaderRefusal<'w, 'a, I, H, S>,
}
impl<'v, 'w, 'a, I: ?Sized, H: DesignMetadataHost, S: DraftingProfileProvider<H> + ?Sized>
    DraftingProgramProfileHeaderError<'v, 'w, 'a, I, H, S>
{
    pub(super) fn new(
        owner: BoundDraftingProgramRootNumericSuppliers<'v, 'w, 'a, I, H, S>,
        count: Option<usize>,
        demand: Option<DesignResourceDemand>,
        refusal: DraftingProgramProfileHeaderRefusal<'w, 'a, I, H, S>,
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
    pub const fn owner(&self) -> &BoundDraftingProgramRootNumericSuppliers<'v, 'w, 'a, I, H, S> {
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
    pub const fn refusal(&self) -> &DraftingProgramProfileHeaderRefusal<'w, 'a, I, H, S> {
        &self.details.refusal
    }
}
impl<I: ?Sized, H: DesignMetadataHost, S: DraftingProfileProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramProfileHeaderError<'_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramProfileHeaderError")
            .field("refusal", &self.details.refusal)
            .finish_non_exhaustive()
    }
}
impl<I: ?Sized, H: DesignMetadataHost, S: DraftingProfileProvider<H> + ?Sized> fmt::Display
    for DraftingProgramProfileHeaderError<'_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "drafting Root Profile headers: {:?}",
            self.details.refusal
        )
    }
}
impl<I: ?Sized, H: DesignMetadataHost, S: DraftingProfileProvider<H> + ?Sized> std::error::Error
    for DraftingProgramProfileHeaderError<'_, '_, '_, I, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.details.refusal {
            DraftingProgramProfileHeaderRefusal::Resources(cause) => Some(cause),
            DraftingProgramProfileHeaderRefusal::Reservation { records: _, cause } => Some(cause),
            DraftingProgramProfileHeaderRefusal::Work(cause) => std::error::Error::source(cause),
            DraftingProgramProfileHeaderRefusal::Source { cause, .. } => {
                std::error::Error::source(cause)
            }
            _ => None,
        }
    }
}
