//! Actual borrowed Root/works and first typed source refusal,without a successful prefix.
use super::{BoundDraftingProgramProfileParameters, DraftingProgramRuleWork};
use crate::{
    construction::{
        DraftingProfileLogicalProvider, DraftingProgramSemanticWorkError, DraftingRuleProvider,
        DraftingRuleSourceError,
    },
    design::{DesignMetadataHost, DesignResourceDemand, DesignResourceLimitError},
};
use core::fmt;
use std::collections::TryReserveError;
/// First original complete rule source-stage refusal.
pub enum DraftingProgramRuleSourceRefusal<
    'q,
    'w,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
> {
    /// Complete N+M+1 whole/work/Profile pass arithmetic cannot fit usize.
    PopulationOverflow,
    /// Original cursor refuses the full complete stage without partial reservation.
    Work(DraftingProgramSemanticWorkError<'w, I>),
    /// First actual fresh aggregate/proof field cause,before result reservation or query.
    Resources(DesignResourceLimitError),
    /// Complete result population could not be reserved.
    Reservation {
        /// Actual complete source result population.
        records: usize,
        /// Original allocator cause.
        cause: TryReserveError,
    },
    /// First actual original work-bound source refusal.
    Source {
        /// Original zero-based input/work index in the complete retained work slice.
        work_index: usize,
        /// Original parent/request/work/member/kind cause and actual returned source witnesses.
        cause: DraftingRuleSourceError<'q, 'w, H, S>,
    },
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > fmt::Debug for DraftingProgramRuleSourceRefusal<'_, '_, I, H, S>
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
            Self::Source { work_index, cause } => f
                .debug_struct("Source")
                .field("work_index", work_index)
                .field("cause", cause)
                .finish(),
        }
    }
}
/// Exact original borrowed Root/cursor and stable complete work parents,with the first typed
/// refusal and only a completed fresh demand. Private source prefixes never escape.
pub struct DraftingProgramRuleSourceError<
    'q,
    'h,
    'v,
    'w,
    'a,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
> {
    details: Box<Failure<'q, 'h, 'v, 'w, 'a, I, H, S>>,
}
struct Failure<
    'q,
    'h,
    'v,
    'w,
    'a,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
> {
    inputs: &'q mut BoundDraftingProgramProfileParameters<'h, 'v, 'w, 'a, I, H, S>,
    works: &'q [DraftingProgramRuleWork<'w, 'a, H, S>],
    demand: Option<DesignResourceDemand>,
    refusal: DraftingProgramRuleSourceRefusal<'q, 'w, I, H, S>,
}
impl<
        'q,
        'h,
        'v,
        'w,
        'a,
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > DraftingProgramRuleSourceError<'q, 'h, 'v, 'w, 'a, I, H, S>
{
    pub(super) fn new(
        inputs: &'q mut BoundDraftingProgramProfileParameters<'h, 'v, 'w, 'a, I, H, S>,
        works: &'q [DraftingProgramRuleWork<'w, 'a, H, S>],
        demand: Option<DesignResourceDemand>,
        refusal: DraftingProgramRuleSourceRefusal<'q, 'w, I, H, S>,
    ) -> Self {
        Self {
            details: Box::new(Failure {
                inputs,
                works,
                demand,
                refusal,
            }),
        }
    }
    /// Exact original Root proof and unchanged or newly consumed unique reservation state.
    #[must_use]
    pub const fn inputs(&self) -> &BoundDraftingProgramProfileParameters<'h, 'v, 'w, 'a, I, H, S> {
        self.details.inputs
    }
    /// Actual complete original work-parent slice,including the first refused source's parent.
    #[must_use]
    pub const fn works(&self) -> &'q [DraftingProgramRuleWork<'w, 'a, H, S>] {
        self.details.works
    }
    /// Exact complete original source count,without deduplicating matching requests.
    #[must_use]
    pub fn source_count(&self) -> usize {
        self.details.works.len()
    }
    /// Complete fresh aggregate/proof demand only after the whole content stage passed.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.details.demand
    }
    /// Original first stage/source cause and every original lending child witness.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingProgramRuleSourceRefusal<'q, 'w, I, H, S> {
        &self.details.refusal
    }
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > fmt::Debug for DraftingProgramRuleSourceError<'_, '_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramRuleSourceError")
            .field("refusal", &self.details.refusal)
            .finish_non_exhaustive()
    }
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > fmt::Display for DraftingProgramRuleSourceError<'_, '_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "drafting program rule sources: {:?}",
            self.details.refusal
        )
    }
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > std::error::Error for DraftingProgramRuleSourceError<'_, '_, '_, '_, '_, I, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.details.refusal {
            DraftingProgramRuleSourceRefusal::Resources(cause) => Some(cause),
            DraftingProgramRuleSourceRefusal::Reservation { cause, .. } => Some(cause),
            DraftingProgramRuleSourceRefusal::Work(cause) => std::error::Error::source(cause),
            DraftingProgramRuleSourceRefusal::Source { cause, .. } => {
                std::error::Error::source(cause)
            }
            DraftingProgramRuleSourceRefusal::PopulationOverflow => None,
        }
    }
}
