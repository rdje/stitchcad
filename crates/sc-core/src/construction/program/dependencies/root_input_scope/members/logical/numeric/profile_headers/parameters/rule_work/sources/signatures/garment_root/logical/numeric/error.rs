//! Every original Root owner/cursor and first typed cause,without a successful numeric supplier prefix.
use super::BoundDraftingProgramGarmentRootLogicalSuppliers;
use crate::{
    construction::{
        DraftingProfileLogicalProvider, DraftingProgramSemanticWorkError, DraftingRuleProvider,
    },
    design::{DesignMetadataHost, DesignResourceDemand, DesignResourceLimitError},
};
use core::fmt;
use std::collections::TryReserveError;
/// First original complete Root-garment-numeric-supplier stage refusal.
pub enum DraftingProgramGarmentRootNumericSupplierRefusal<'w, I: ?Sized> {
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
impl<I: ?Sized> fmt::Debug for DraftingProgramGarmentRootNumericSupplierRefusal<'_, I> {
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
pub struct DraftingProgramGarmentRootNumericSupplierError<
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
    owner: BoundDraftingProgramGarmentRootLogicalSuppliers<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
    count: Option<usize>,
    demand: Option<DesignResourceDemand>,
    refusal: DraftingProgramGarmentRootNumericSupplierRefusal<'w, I>,
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
    > DraftingProgramGarmentRootNumericSupplierError<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>
{
    pub(super) fn new(
        owner: BoundDraftingProgramGarmentRootLogicalSuppliers<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
        count: Option<usize>,
        demand: Option<DesignResourceDemand>,
        refusal: DraftingProgramGarmentRootNumericSupplierRefusal<'w, I>,
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
    ) -> &BoundDraftingProgramGarmentRootLogicalSuppliers<'z, 'q, 'h, 'v, 'w, 'a, I, H, S> {
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
    pub const fn refusal(&self) -> &DraftingProgramGarmentRootNumericSupplierRefusal<'w, I> {
        &self.details.refusal
    }
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > fmt::Debug
    for DraftingProgramGarmentRootNumericSupplierError<'_, '_, '_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramGarmentRootNumericSupplierError")
            .field("refusal", &self.details.refusal)
            .finish_non_exhaustive()
    }
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > fmt::Display
    for DraftingProgramGarmentRootNumericSupplierError<'_, '_, '_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "drafting Root garment numeric suppliers: {:?}",
            self.details.refusal
        )
    }
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > std::error::Error
    for DraftingProgramGarmentRootNumericSupplierError<'_, '_, '_, '_, '_, '_, I, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.details.refusal {
            DraftingProgramGarmentRootNumericSupplierRefusal::Resources(cause) => Some(cause),
            DraftingProgramGarmentRootNumericSupplierRefusal::Reservation { records: _, cause } => {
                Some(cause)
            }
            DraftingProgramGarmentRootNumericSupplierRefusal::Work(cause) => {
                std::error::Error::source(cause)
            }
            _ => None,
        }
    }
}
