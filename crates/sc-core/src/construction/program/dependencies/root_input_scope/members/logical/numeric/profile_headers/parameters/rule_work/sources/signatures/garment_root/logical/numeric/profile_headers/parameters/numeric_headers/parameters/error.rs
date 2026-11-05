//! Actual borrowed Root/headers and first typed parameter refusal,without a successful prefix.
use super::{
    BoundDraftingProgramGarmentProfileLogicalParameters, DraftingProgramGarmentProfileNumericHeader,
};
use crate::{
    construction::{
        DraftingProfileLogicalProvider, DraftingProfileNumericError,
        DraftingProfileNumericProvider, DraftingProgramSemanticWorkError, DraftingRuleProvider,
    },
    design::{DesignMetadataHost, DesignResourceDemand, DesignResourceLimitError},
};
use core::fmt;
use std::collections::TryReserveError;
/// First original complete Profile parameter-stage refusal.
pub enum DraftingProgramGarmentProfileNumericParameterRefusal<
    'u,
    'w,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingProfileNumericProvider<H>
        + DraftingProfileLogicalProvider<H>
        + DraftingRuleProvider<H>
        + ?Sized,
> {
    /// Complete N+1 whole/parameter pass arithmetic cannot fit usize.
    PopulationOverflow,
    /// Original cursor refuses the full complete stage without partial reservation.
    Work(DraftingProgramSemanticWorkError<'w, I>),
    /// First actual fresh aggregate/proof field cause,before result reservation or query.
    Resources(DesignResourceLimitError),
    /// Complete result population could not be reserved.
    Reservation {
        /// Actual complete parameter result population.
        records: usize,
        /// Original allocator cause.
        cause: TryReserveError,
    },
    /// First actual original header-bound parameter refusal.
    Parameter {
        /// Original zero-based input/header index in the complete retained header slice.
        header_index: usize,
        /// Original parent/request/header/member/kind cause and actual returned source witnesses.
        cause: DraftingProfileNumericError<'u, 'w, H, S>,
    },
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileNumericProvider<H>
            + DraftingProfileLogicalProvider<H>
            + DraftingRuleProvider<H>
            + ?Sized,
    > fmt::Debug for DraftingProgramGarmentProfileNumericParameterRefusal<'_, '_, I, H, S>
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
            Self::Parameter {
                header_index,
                cause,
            } => f
                .debug_struct("Parameter")
                .field("header_index", header_index)
                .field("cause", cause)
                .finish(),
        }
    }
}
/// Exact original borrowed Root/cursor and stable complete header parents,with the first typed
/// refusal and only a completed fresh demand. Private parameter prefixes never escape.
pub struct DraftingProgramGarmentProfileNumericParameterError<
    'u,
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
    details: Box<Failure<'u, 't, 'z, 'q, 'h, 'v, 'w, 'a, I, H, S>>,
}
struct Failure<
    'u,
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
    logical: &'u mut BoundDraftingProgramGarmentProfileLogicalParameters<
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
    headers: &'u [DraftingProgramGarmentProfileNumericHeader<'w, 'a, H, S>],
    demand: Option<DesignResourceDemand>,
    refusal: DraftingProgramGarmentProfileNumericParameterRefusal<'u, 'w, I, H, S>,
}
impl<
        'u,
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
    > DraftingProgramGarmentProfileNumericParameterError<'u, 't, 'z, 'q, 'h, 'v, 'w, 'a, I, H, S>
{
    pub(super) fn new(
        logical: &'u mut BoundDraftingProgramGarmentProfileLogicalParameters<
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
        headers: &'u [DraftingProgramGarmentProfileNumericHeader<'w, 'a, H, S>],
        demand: Option<DesignResourceDemand>,
        refusal: DraftingProgramGarmentProfileNumericParameterRefusal<'u, 'w, I, H, S>,
    ) -> Self {
        Self {
            details: Box::new(Failure {
                logical,
                headers,
                demand,
                refusal,
            }),
        }
    }
    /// Exact original Root proof and unchanged or newly consumed unique reservation state.
    #[must_use]
    pub const fn logical(
        &self,
    ) -> &BoundDraftingProgramGarmentProfileLogicalParameters<'t, 'z, 'q, 'h, 'v, 'w, 'a, I, H, S>
    {
        self.details.logical
    }
    /// Actual complete original header-parent slice,including the first refused parameter's parent.
    #[must_use]
    pub const fn headers(&self) -> &'u [DraftingProgramGarmentProfileNumericHeader<'w, 'a, H, S>] {
        self.details.headers
    }
    /// Exact complete original parameter count,without deduplicating matching requests.
    #[must_use]
    pub fn parameter_count(&self) -> usize {
        self.details.headers.len()
    }
    /// Complete fresh aggregate/proof demand only after the whole content stage passed.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.details.demand
    }
    /// Original first stage/parameter cause and every original lending child witness.
    #[must_use]
    pub const fn refusal(
        &self,
    ) -> &DraftingProgramGarmentProfileNumericParameterRefusal<'u, 'w, I, H, S> {
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
    for DraftingProgramGarmentProfileNumericParameterError<'_, '_, '_, '_, '_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramGarmentProfileNumericParameterError")
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
    for DraftingProgramGarmentProfileNumericParameterError<'_, '_, '_, '_, '_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "drafting Profile garment numeric parameters: {:?}",
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
    for DraftingProgramGarmentProfileNumericParameterError<'_, '_, '_, '_, '_, '_, '_, '_, I, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.details.refusal {
            DraftingProgramGarmentProfileNumericParameterRefusal::Resources(cause) => Some(cause),
            DraftingProgramGarmentProfileNumericParameterRefusal::Reservation { cause, .. } => {
                Some(cause)
            }
            DraftingProgramGarmentProfileNumericParameterRefusal::Work(cause) => {
                std::error::Error::source(cause)
            }
            DraftingProgramGarmentProfileNumericParameterRefusal::Parameter { cause, .. } => {
                std::error::Error::source(cause)
            }
            DraftingProgramGarmentProfileNumericParameterRefusal::PopulationOverflow => None,
        }
    }
}
