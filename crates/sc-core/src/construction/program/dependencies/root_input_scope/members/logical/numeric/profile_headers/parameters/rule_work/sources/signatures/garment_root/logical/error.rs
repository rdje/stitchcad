//! Every original Root owner/cursor and first typed cause,without a successful supplier prefix.
use super::BoundDraftingProgramGarmentRootInputMembers;
use crate::{
    construction::{
        DraftingDesignOperationSource, DraftingDesignSemanticLookupError,
        DraftingProfileLogicalProvider, DraftingProgramSemanticWorkError, DraftingRuleProvider,
    },
    design::{DesignMetadataHost, DesignResourceDemand, DesignResourceLimitError},
};
use core::fmt;
use std::collections::TryReserveError;
/// First original complete Root-garment-logical-supplier stage refusal.
pub enum DraftingProgramGarmentRootLogicalSupplierRefusal<
    'v,
    'w,
    'a,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
> {
    /// Complete actual eligible logical-Derived count cannot fit usize,before child work.
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
    /// First actual Root supplier lookup refusal,including original requested ID/kind and cause.
    Lookup {
        /// Original consumer record index in owner().records().
        member_index: usize,
        /// Original scope/request/lending cause.
        cause: DraftingDesignSemanticLookupError<'v, 'a, H, S>,
    },
    /// Actual LogicalDerivation output differs from the original canonical input ID or kind.
    Output {
        /// Original consumer record index.
        member_index: usize,
        /// Actual supplier and original output field witness.
        source: DraftingDesignOperationSource<'v, 'a, H, S>,
    },
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > fmt::Debug for DraftingProgramGarmentRootLogicalSupplierRefusal<'_, '_, '_, I, H, S>
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
            Self::Lookup {
                member_index,
                cause,
            } => f
                .debug_struct("Lookup")
                .field("member_index", member_index)
                .field("cause", cause)
                .finish(),
            Self::Output {
                member_index,
                source,
            } => f
                .debug_struct("Output")
                .field("member_index", member_index)
                .field("source", source)
                .finish(),
        }
    }
}
/// Every original consumer/source/key owner and unique cursor,complete stage metadata and first
/// typed cause. Private partial supplier vectors are discarded;only the refused diagnostic boxes.
pub struct DraftingProgramGarmentRootLogicalSupplierError<
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
    owner: BoundDraftingProgramGarmentRootInputMembers<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
    count: Option<usize>,
    demand: Option<DesignResourceDemand>,
    refusal: DraftingProgramGarmentRootLogicalSupplierRefusal<'v, 'w, 'a, I, H, S>,
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
    > DraftingProgramGarmentRootLogicalSupplierError<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>
{
    pub(super) fn new(
        owner: BoundDraftingProgramGarmentRootInputMembers<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
        count: Option<usize>,
        demand: Option<DesignResourceDemand>,
        refusal: DraftingProgramGarmentRootLogicalSupplierRefusal<'v, 'w, 'a, I, H, S>,
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
    ) -> &BoundDraftingProgramGarmentRootInputMembers<'z, 'q, 'h, 'v, 'w, 'a, I, H, S> {
        &self.details.owner
    }
    /// Complete actual logical-Derived population,only after counting completed.
    #[must_use]
    pub const fn supplier_count(&self) -> Option<usize> {
        self.details.count
    }
    /// Complete fresh Root/provider demand only after the entire resource stage passed.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.details.demand
    }
    /// Original first typed cause and its actual original consumer/request/supplier witnesses.
    #[must_use]
    pub const fn refusal(
        &self,
    ) -> &DraftingProgramGarmentRootLogicalSupplierRefusal<'v, 'w, 'a, I, H, S> {
        &self.details.refusal
    }
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > fmt::Debug
    for DraftingProgramGarmentRootLogicalSupplierError<'_, '_, '_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramGarmentRootLogicalSupplierError")
            .field("refusal", &self.details.refusal)
            .finish_non_exhaustive()
    }
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > fmt::Display
    for DraftingProgramGarmentRootLogicalSupplierError<'_, '_, '_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "drafting Root garment logical suppliers: {:?}",
            self.details.refusal
        )
    }
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > std::error::Error
    for DraftingProgramGarmentRootLogicalSupplierError<'_, '_, '_, '_, '_, '_, I, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.details.refusal {
            DraftingProgramGarmentRootLogicalSupplierRefusal::Resources(cause) => Some(cause),
            DraftingProgramGarmentRootLogicalSupplierRefusal::Reservation { records: _, cause } => {
                Some(cause)
            }
            DraftingProgramGarmentRootLogicalSupplierRefusal::Work(cause) => {
                std::error::Error::source(cause)
            }
            _ => None,
        }
    }
}
