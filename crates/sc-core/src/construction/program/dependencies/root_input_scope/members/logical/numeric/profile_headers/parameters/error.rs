//! Actual borrowed Root/headers and first typed parameter refusal,without a successful prefix.
use super::{BoundDraftingProgramRootNumericSuppliers, DraftingProgramProfileInputHeader};
use crate::{
    construction::{
        DraftingProfileLogicalError, DraftingProfileLogicalProvider,
        DraftingProgramSemanticWorkError,
    },
    design::{DesignMetadataHost, DesignResourceDemand, DesignResourceLimitError},
};
use core::fmt;
use std::collections::TryReserveError;
/// First original complete Profile parameter-stage refusal.
pub enum DraftingProgramProfileParameterRefusal<
    'h,
    'w,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingProfileLogicalProvider<H> + ?Sized,
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
        cause: DraftingProfileLogicalError<'h, 'w, H, S>,
    },
}
impl<I: ?Sized, H: DesignMetadataHost, S: DraftingProfileLogicalProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramProfileParameterRefusal<'_, '_, I, H, S>
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
pub struct DraftingProgramProfileParameterError<
    'h,
    'v,
    'w,
    'a,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingProfileLogicalProvider<H> + ?Sized,
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
    S: DraftingProfileLogicalProvider<H> + ?Sized,
> {
    root: &'h mut BoundDraftingProgramRootNumericSuppliers<'v, 'w, 'a, I, H, S>,
    headers: &'h [DraftingProgramProfileInputHeader<'w, 'a, H, S>],
    demand: Option<DesignResourceDemand>,
    refusal: DraftingProgramProfileParameterRefusal<'h, 'w, I, H, S>,
}
impl<
        'h,
        'v,
        'w,
        'a,
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + ?Sized,
    > DraftingProgramProfileParameterError<'h, 'v, 'w, 'a, I, H, S>
{
    pub(super) fn new(
        root: &'h mut BoundDraftingProgramRootNumericSuppliers<'v, 'w, 'a, I, H, S>,
        headers: &'h [DraftingProgramProfileInputHeader<'w, 'a, H, S>],
        demand: Option<DesignResourceDemand>,
        refusal: DraftingProgramProfileParameterRefusal<'h, 'w, I, H, S>,
    ) -> Self {
        Self {
            details: Box::new(Failure {
                root,
                headers,
                demand,
                refusal,
            }),
        }
    }
    /// Exact original Root proof and unchanged or newly consumed unique reservation state.
    #[must_use]
    pub const fn root(&self) -> &BoundDraftingProgramRootNumericSuppliers<'v, 'w, 'a, I, H, S> {
        self.details.root
    }
    /// Actual complete original header-parent slice,including the first refused parameter's parent.
    #[must_use]
    pub const fn headers(&self) -> &'h [DraftingProgramProfileInputHeader<'w, 'a, H, S>] {
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
    pub const fn refusal(&self) -> &DraftingProgramProfileParameterRefusal<'h, 'w, I, H, S> {
        &self.details.refusal
    }
}
impl<I: ?Sized, H: DesignMetadataHost, S: DraftingProfileLogicalProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramProfileParameterError<'_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramProfileParameterError")
            .field("refusal", &self.details.refusal)
            .finish_non_exhaustive()
    }
}
impl<I: ?Sized, H: DesignMetadataHost, S: DraftingProfileLogicalProvider<H> + ?Sized> fmt::Display
    for DraftingProgramProfileParameterError<'_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "drafting Profile input parameters: {:?}",
            self.details.refusal
        )
    }
}
impl<I: ?Sized, H: DesignMetadataHost, S: DraftingProfileLogicalProvider<H> + ?Sized>
    std::error::Error for DraftingProgramProfileParameterError<'_, '_, '_, '_, I, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.details.refusal {
            DraftingProgramProfileParameterRefusal::Resources(cause) => Some(cause),
            DraftingProgramProfileParameterRefusal::Reservation { cause, .. } => Some(cause),
            DraftingProgramProfileParameterRefusal::Work(cause) => std::error::Error::source(cause),
            DraftingProgramProfileParameterRefusal::Parameter { cause, .. } => {
                std::error::Error::source(cause)
            }
            DraftingProgramProfileParameterRefusal::PopulationOverflow => None,
        }
    }
}
