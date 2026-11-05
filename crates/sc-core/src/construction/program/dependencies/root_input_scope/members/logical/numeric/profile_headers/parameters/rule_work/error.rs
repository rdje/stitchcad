//! Every original Root owner/cursor and first typed cause,without a successful work prefix.
use super::{BoundDraftingProgramProfileParameters, DraftingProgramRuleWorkRequest};
use crate::{
    construction::{
        DraftingProfileLogicalProvider, DraftingProgramSemanticWorkError, DraftingRuleProvider,
        DraftingRuleWorkError,
    },
    design::{DesignMetadataHost, DesignResourceDemand, DesignResourceLimitError},
};
use core::fmt;
use std::collections::TryReserveError;
/// First original complete rule-work stage refusal.
pub enum DraftingProgramRuleWorkRefusal<
    'w,
    'a,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
> {
    /// Complete actual eligible rule/policy count cannot fit usize,before child work.
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
    /// First original child work cause,with the actual consumer field and original row/pin.
    Child {
        /// Exact original operation-input and direct-selection witnesses.
        request: DraftingProgramRuleWorkRequest<'w, 'a>,
        /// Original child work factory cause and operation lookup witnesses.
        cause: DraftingRuleWorkError<'w, H, S>,
    },
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > fmt::Debug for DraftingProgramRuleWorkRefusal<'_, '_, I, H, S>
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
            Self::Child { request, cause } => f
                .debug_struct("Child")
                .field("request", request)
                .field("cause", cause)
                .finish(),
        }
    }
}
/// Every original consumer/source/key owner and unique cursor,complete stage metadata and first
/// typed cause. Private partial rule-work vectors are discarded;only the refused diagnostic boxes.
pub struct DraftingProgramRuleWorkError<
    'h,
    'v,
    'w,
    'a,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
> {
    details: Box<Failure<'h, 'v, 'w, 'a, I, H, S>>,
}
struct Failure<
    'h,
    'v,
    'w,
    'a,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
> {
    owner: BoundDraftingProgramProfileParameters<'h, 'v, 'w, 'a, I, H, S>,
    count: Option<usize>,
    demand: Option<DesignResourceDemand>,
    refusal: DraftingProgramRuleWorkRefusal<'w, 'a, I, H, S>,
}
impl<
        'h,
        'v,
        'w,
        'a,
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > DraftingProgramRuleWorkError<'h, 'v, 'w, 'a, I, H, S>
{
    pub(super) fn new(
        owner: BoundDraftingProgramProfileParameters<'h, 'v, 'w, 'a, I, H, S>,
        count: Option<usize>,
        demand: Option<DesignResourceDemand>,
        refusal: DraftingProgramRuleWorkRefusal<'w, 'a, I, H, S>,
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
    pub const fn owner(&self) -> &BoundDraftingProgramProfileParameters<'h, 'v, 'w, 'a, I, H, S> {
        &self.details.owner
    }
    /// Complete actual rule/policy population,only after counting completed.
    #[must_use]
    pub const fn rule_count(&self) -> Option<usize> {
        self.details.count
    }
    /// Complete fresh aggregate/proof demand only after the entire resource stage passed.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.details.demand
    }
    /// Original first typed cause and its actual original consumer/request/member witnesses.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingProgramRuleWorkRefusal<'w, 'a, I, H, S> {
        &self.details.refusal
    }
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > fmt::Debug for DraftingProgramRuleWorkError<'_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramRuleWorkError")
            .field("refusal", &self.details.refusal)
            .finish_non_exhaustive()
    }
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > fmt::Display for DraftingProgramRuleWorkError<'_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "drafting program rule work: {:?}", self.details.refusal)
    }
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > std::error::Error for DraftingProgramRuleWorkError<'_, '_, '_, '_, I, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.details.refusal {
            DraftingProgramRuleWorkRefusal::Resources(cause) => Some(cause),
            DraftingProgramRuleWorkRefusal::Reservation { records: _, cause } => Some(cause),
            DraftingProgramRuleWorkRefusal::Work(cause) => std::error::Error::source(cause),
            DraftingProgramRuleWorkRefusal::Child { cause, .. } => std::error::Error::source(cause),
            _ => None,
        }
    }
}
