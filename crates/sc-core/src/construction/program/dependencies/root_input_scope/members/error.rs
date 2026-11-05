//! Every original Root owner/cursor and first typed cause,without a successful member prefix.
use super::DraftingProgramRootInputScope;
use crate::{
    construction::{
        DraftingContextSource, DraftingDesignSemanticLookupError, DraftingProgramSemanticWorkError,
        DraftingSemanticSelectionLocation,
    },
    design::{DesignMetadataHost, DesignResourceDemand, DesignResourceLimitError},
};
use core::fmt;
use std::collections::TryReserveError;
/// First original complete Root-member stage refusal.
pub enum DraftingProgramRootInputMemberRefusal<
    'v,
    'w,
    'a,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    /// Complete actual eligible Root-input count cannot fit usize,before child work.
    PopulationOverflow,
    /// Original unique child cursor refuses the complete N-pass/result reservation unchanged.
    Work(DraftingProgramSemanticWorkError<'w, I>),
    /// First fresh actual Root/provider content field cause,before allocation or member lookup.
    Resources(DesignResourceLimitError),
    /// Complete admitted record population could not be reserved.
    Reservation {
        /// Original complete record count.
        records: usize,
        /// Original allocator cause.
        cause: TryReserveError,
    },
    /// First actual original Root operation-input member refusal under its exact supplied scope.
    Lookup {
        /// Genuine original consumer field location.
        location: DraftingSemanticSelectionLocation,
        /// Original scope/request/lending cause.
        cause: DraftingDesignSemanticLookupError<'v, 'a, H, S>,
    },
}
impl<I: ?Sized, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingProgramRootInputMemberRefusal<'_, '_, '_, I, H, S>
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
            Self::Lookup { location, cause } => f
                .debug_struct("Lookup")
                .field("location", location)
                .field("cause", cause)
                .finish(),
        }
    }
}
/// Every original consumer/source/key owner and unique cursor,complete stage metadata and first
/// typed cause. Private partial member vectors are discarded;only the refused diagnostic boxes.
pub struct DraftingProgramRootInputMemberError<
    'v,
    'w,
    'a,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    details: Box<Failure<'v, 'w, 'a, I, H, S>>,
}
struct Failure<'v, 'w, 'a, I: ?Sized, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> {
    owner: DraftingProgramRootInputScope<'v, 'w, 'a, I, H, S>,
    count: Option<usize>,
    demand: Option<DesignResourceDemand>,
    refusal: DraftingProgramRootInputMemberRefusal<'v, 'w, 'a, I, H, S>,
}
impl<'v, 'w, 'a, I: ?Sized, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingProgramRootInputMemberError<'v, 'w, 'a, I, H, S>
{
    pub(super) fn new(
        owner: DraftingProgramRootInputScope<'v, 'w, 'a, I, H, S>,
        count: Option<usize>,
        demand: Option<DesignResourceDemand>,
        refusal: DraftingProgramRootInputMemberRefusal<'v, 'w, 'a, I, H, S>,
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
    pub const fn owner(&self) -> &DraftingProgramRootInputScope<'v, 'w, 'a, I, H, S> {
        &self.details.owner
    }
    /// Complete actual Root-input population,only after counting completed.
    #[must_use]
    pub const fn member_count(&self) -> Option<usize> {
        self.details.count
    }
    /// Complete fresh Root/provider demand only after the entire resource stage passed.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.details.demand
    }
    /// Original first typed cause and its actual original consumer/request/member witnesses.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingProgramRootInputMemberRefusal<'v, 'w, 'a, I, H, S> {
        &self.details.refusal
    }
}
impl<I: ?Sized, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingProgramRootInputMemberError<'_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramRootInputMemberError")
            .field("refusal", &self.details.refusal)
            .finish_non_exhaustive()
    }
}
impl<I: ?Sized, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Display
    for DraftingProgramRootInputMemberError<'_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "drafting Root operation inputs: {:?}",
            self.details.refusal
        )
    }
}
impl<I: ?Sized, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> std::error::Error
    for DraftingProgramRootInputMemberError<'_, '_, '_, I, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.details.refusal {
            DraftingProgramRootInputMemberRefusal::Resources(cause) => Some(cause),
            DraftingProgramRootInputMemberRefusal::Reservation { records: _, cause } => Some(cause),
            DraftingProgramRootInputMemberRefusal::Work(cause) => std::error::Error::source(cause),
            _ => None,
        }
    }
}
