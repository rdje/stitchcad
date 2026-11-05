//! Every original Root owner/cursor and first typed cause,without a successful numeric supplier prefix.
use super::super::BoundDraftingProgramRootLogicalSuppliers;
use crate::{
    construction::{DraftingContextSource, DraftingProgramSemanticWorkError},
    design::{DesignMetadataHost, DesignResourceDemand, DesignResourceLimitError},
};
use core::fmt;
use std::collections::TryReserveError;
/// First original complete Root-numeric-supplier stage refusal.
pub enum DraftingProgramRootNumericSupplierRefusal<'w, I: ?Sized> {
    /// Complete actual eligible numeric-Derived count cannot fit usize,before child work.
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
}
impl<I: ?Sized> fmt::Debug for DraftingProgramRootNumericSupplierRefusal<'_, I> {
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
        }
    }
}
/// Every original consumer/source/key owner and unique cursor,complete stage metadata and first
/// typed cause. Private partial numeric supplier vectors are discarded;only the refused diagnostic boxes.
pub struct DraftingProgramRootNumericSupplierError<
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
    owner: BoundDraftingProgramRootLogicalSuppliers<'v, 'w, 'a, I, H, S>,
    count: Option<usize>,
    demand: Option<DesignResourceDemand>,
    refusal: DraftingProgramRootNumericSupplierRefusal<'w, I>,
}
impl<'v, 'w, 'a, I: ?Sized, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingProgramRootNumericSupplierError<'v, 'w, 'a, I, H, S>
{
    pub(super) fn new(
        owner: BoundDraftingProgramRootLogicalSuppliers<'v, 'w, 'a, I, H, S>,
        count: Option<usize>,
        demand: Option<DesignResourceDemand>,
        refusal: DraftingProgramRootNumericSupplierRefusal<'w, I>,
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
    pub const fn owner(&self) -> &BoundDraftingProgramRootLogicalSuppliers<'v, 'w, 'a, I, H, S> {
        &self.details.owner
    }
    /// Complete actual numeric-Derived population,only after counting completed.
    #[must_use]
    pub const fn supplier_count(&self) -> Option<usize> {
        self.details.count
    }
    /// Complete fresh aggregate/proof demand only after the entire resource stage passed.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.details.demand
    }
    /// Original first typed cause and its actual original consumer/request/member witnesses.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingProgramRootNumericSupplierRefusal<'w, I> {
        &self.details.refusal
    }
}
impl<I: ?Sized, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingProgramRootNumericSupplierError<'_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramRootNumericSupplierError")
            .field("refusal", &self.details.refusal)
            .finish_non_exhaustive()
    }
}
impl<I: ?Sized, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Display
    for DraftingProgramRootNumericSupplierError<'_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "drafting Root numeric suppliers: {:?}",
            self.details.refusal
        )
    }
}
impl<I: ?Sized, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> std::error::Error
    for DraftingProgramRootNumericSupplierError<'_, '_, '_, I, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.details.refusal {
            DraftingProgramRootNumericSupplierRefusal::Resources(cause) => Some(cause),
            DraftingProgramRootNumericSupplierRefusal::Reservation { records: _, cause } => {
                Some(cause)
            }
            DraftingProgramRootNumericSupplierRefusal::Work(cause) => {
                std::error::Error::source(cause)
            }
            _ => None,
        }
    }
}
