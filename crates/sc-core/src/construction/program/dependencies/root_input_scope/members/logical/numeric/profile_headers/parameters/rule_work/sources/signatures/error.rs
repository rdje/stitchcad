//! Actual borrowed Root/sources and first typed signature refusal,without a successful prefix.
use super::{BoundDraftingProgramProfileParameters, DraftingProgramRuleSource};
use crate::{
    construction::{
        DraftingProfileLogicalProvider, DraftingProgramSemanticWorkError, DraftingRuleProvider,
        DraftingRuleSignatureError,
    },
    design::{DesignMetadataHost, DesignResourceDemand, DesignResourceLimitError},
};
use core::fmt;
use std::collections::TryReserveError;
/// First original complete rule signature-stage refusal.
pub enum DraftingProgramRuleSignatureRefusal<
    'z,
    'q,
    'w,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
> {
    /// Complete N+1 whole/signature pass arithmetic cannot fit usize.
    PopulationOverflow,
    /// Original cursor refuses the full complete stage without partial reservation.
    Work(DraftingProgramSemanticWorkError<'w, I>),
    /// First actual fresh aggregate/proof field cause,before result reservation or query.
    Resources(DesignResourceLimitError),
    /// Complete result population could not be reserved.
    Reservation {
        /// Actual complete signature result population.
        records: usize,
        /// Original allocator cause.
        cause: TryReserveError,
    },
    /// First actual original source-bound signature refusal.
    Signature {
        /// Original zero-based source index in the complete retained source slice.
        source_index: usize,
        /// Original parent/request/work/member/kind cause and actual returned source witnesses.
        cause: DraftingRuleSignatureError<'z, 'q, 'w, H, S>,
    },
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > fmt::Debug for DraftingProgramRuleSignatureRefusal<'_, '_, '_, I, H, S>
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
            Self::Signature {
                source_index,
                cause,
            } => f
                .debug_struct("Signature")
                .field("source_index", source_index)
                .field("cause", cause)
                .finish(),
        }
    }
}
/// Exact original borrowed Root/cursor and stable complete source parents,with the first typed
/// refusal and only a completed fresh demand. Private signature prefixes never escape.
pub struct DraftingProgramRuleSignatureError<
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
    details: Box<Failure<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>>,
}
struct Failure<
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
    inputs: &'z mut BoundDraftingProgramProfileParameters<'h, 'v, 'w, 'a, I, H, S>,
    sources: &'z [DraftingProgramRuleSource<'q, 'w, 'a, H, S>],
    demand: Option<DesignResourceDemand>,
    refusal: DraftingProgramRuleSignatureRefusal<'z, 'q, 'w, I, H, S>,
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
    > DraftingProgramRuleSignatureError<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>
{
    pub(super) fn new(
        inputs: &'z mut BoundDraftingProgramProfileParameters<'h, 'v, 'w, 'a, I, H, S>,
        sources: &'z [DraftingProgramRuleSource<'q, 'w, 'a, H, S>],
        demand: Option<DesignResourceDemand>,
        refusal: DraftingProgramRuleSignatureRefusal<'z, 'q, 'w, I, H, S>,
    ) -> Self {
        Self {
            details: Box::new(Failure {
                inputs,
                sources,
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
    pub const fn sources(&self) -> &'z [DraftingProgramRuleSource<'q, 'w, 'a, H, S>] {
        self.details.sources
    }
    /// Exact complete original source count,without deduplicating matching requests.
    #[must_use]
    pub fn signature_count(&self) -> usize {
        self.details.sources.len()
    }
    /// Complete fresh aggregate/proof demand only after the whole content stage passed.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.details.demand
    }
    /// Original first stage/source cause and every original lending child witness.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingProgramRuleSignatureRefusal<'z, 'q, 'w, I, H, S> {
        &self.details.refusal
    }
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > fmt::Debug for DraftingProgramRuleSignatureError<'_, '_, '_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramRuleSignatureError")
            .field("refusal", &self.details.refusal)
            .finish_non_exhaustive()
    }
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > fmt::Display for DraftingProgramRuleSignatureError<'_, '_, '_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "drafting program rule signatures: {:?}",
            self.details.refusal
        )
    }
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > std::error::Error for DraftingProgramRuleSignatureError<'_, '_, '_, '_, '_, '_, I, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.details.refusal {
            DraftingProgramRuleSignatureRefusal::Resources(cause) => Some(cause),
            DraftingProgramRuleSignatureRefusal::Reservation { cause, .. } => Some(cause),
            DraftingProgramRuleSignatureRefusal::Work(cause) => std::error::Error::source(cause),
            DraftingProgramRuleSignatureRefusal::Signature { cause, .. } => {
                std::error::Error::source(cause)
            }
            DraftingProgramRuleSignatureRefusal::PopulationOverflow => None,
        }
    }
}
