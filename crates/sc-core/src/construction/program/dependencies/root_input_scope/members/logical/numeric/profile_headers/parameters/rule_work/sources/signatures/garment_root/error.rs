//! Original complete operation/cursor owners and first actual garment/member/metadata refusal.
use super::BoundDraftingProgramRuleSignatures;
use crate::{
    construction::{
        DraftingDesignSemanticLookupError, DraftingGarmentSemanticUse,
        DraftingProfileLogicalProvider, DraftingProgramSemanticWorkError, DraftingRuleProvider,
    },
    design::{
        DesignMetadataHost, DesignParameter, DesignParameterError, DesignResourceDemand,
        DesignResourceLimitError,
    },
    ontology::EntityId,
    recipe::FormulaDeclaration,
};
use core::fmt;
use std::collections::TryReserveError;
/// First complete Root-owned Pocket/Closure input-stage refusal,with original witnesses.
pub enum DraftingProgramGarmentRootInputMemberRefusal<
    'v,
    'w,
    'a,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
> {
    /// Complete eligible original input count cannot fit usize.
    PopulationOverflow,
    /// Original cursor refuses complete stage unchanged.
    Work(DraftingProgramSemanticWorkError<'w, I>),
    /// First fresh complete aggregate/proof field cause,before allocation or member lookup.
    Resources(DesignResourceLimitError),
    /// Complete result population could not be reserved.
    Reservation {
        /// Actual complete record count.
        records: usize,
        /// Original allocator cause.
        cause: TryReserveError,
    },
    /// First original logical/numeric lookup cause and actual garment field.
    Lookup {
        /// Exact actual garment/member/field/location.
        use_: DraftingGarmentSemanticUse<'a, H>,
        /// Original supplied scope/request and lending cause.
        cause: DraftingDesignSemanticLookupError<'v, 'a, H, S>,
    },
    /// Formula zipper origin names no actual Design parameter metadata.
    MissingParameter {
        /// Exact original requiring garment field.
        use_: DraftingGarmentSemanticUse<'a, H>,
        /// Original metadata identity,not a canonical declaration ID.
        identity: EntityId,
    },
    /// Original current parameter-to-canonical declaration cause.
    Parameter {
        /// Exact original requiring garment field.
        use_: DraftingGarmentSemanticUse<'a, H>,
        /// Actual original parameter record.
        parameter: &'a DesignParameter,
        /// Original metadata/target cause.
        cause: DesignParameterError,
    },
    /// Actual current formula parameter metadata is not Length-kind.
    FormulaKind {
        /// Exact original requiring garment field.
        use_: DraftingGarmentSemanticUse<'a, H>,
        /// Actual original parameter record.
        parameter: &'a DesignParameter,
        /// Original current metadata and actual kind,without a value read.
        metadata: FormulaDeclaration<'a>,
    },
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > fmt::Debug for DraftingProgramGarmentRootInputMemberRefusal<'_, '_, '_, I, H, S>
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
            Self::Lookup { use_, cause } => f
                .debug_struct("Lookup")
                .field("occurrence", use_)
                .field("cause", cause)
                .finish(),
            Self::MissingParameter { use_, identity } => f
                .debug_struct("MissingParameter")
                .field("occurrence", use_)
                .field("identity", identity)
                .finish(),
            Self::Parameter {
                use_,
                parameter,
                cause,
            } => f
                .debug_struct("Parameter")
                .field("occurrence", use_)
                .field("parameter", &parameter.id())
                .field("cause", cause)
                .finish(),
            Self::FormulaKind {
                use_,
                parameter,
                metadata,
            } => f
                .debug_struct("FormulaKind")
                .field("occurrence", use_)
                .field("parameter", &parameter.id())
                .field("kind", &metadata.kind())
                .finish(),
        }
    }
}
/// Exact original complete operation/cursor owners,count,demand and first typed cause. Only the
/// refused diagnostic boxes;successful private member prefixes drop and cannot be inspected.
pub struct DraftingProgramGarmentRootInputMemberError<
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
    owner: BoundDraftingProgramRuleSignatures<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
    count: Option<usize>,
    demand: Option<DesignResourceDemand>,
    refusal: DraftingProgramGarmentRootInputMemberRefusal<'v, 'w, 'a, I, H, S>,
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
    > DraftingProgramGarmentRootInputMemberError<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>
{
    pub(super) fn new(
        owner: BoundDraftingProgramRuleSignatures<'z, 'q, 'h, 'v, 'w, 'a, I, H, S>,
        count: Option<usize>,
        demand: Option<DesignResourceDemand>,
        refusal: DraftingProgramGarmentRootInputMemberRefusal<'v, 'w, 'a, I, H, S>,
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
    /// Original complete operation inputs/rule signatures and unchanged or consumed unique cursor.
    #[must_use]
    pub const fn owner(
        &self,
    ) -> &BoundDraftingProgramRuleSignatures<'z, 'q, 'h, 'v, 'w, 'a, I, H, S> {
        &self.details.owner
    }
    /// Complete original Root-owned input population after successful counting.
    #[must_use]
    pub const fn member_count(&self) -> Option<usize> {
        self.details.count
    }
    /// Fresh complete aggregate/proof demand only after the entire resource stage passed.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.details.demand
    }
    /// Original first typed cause and all actual garment/scope/member/metadata witnesses.
    #[must_use]
    pub const fn refusal(
        &self,
    ) -> &DraftingProgramGarmentRootInputMemberRefusal<'v, 'w, 'a, I, H, S> {
        &self.details.refusal
    }
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > fmt::Debug for DraftingProgramGarmentRootInputMemberError<'_, '_, '_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramGarmentRootInputMemberError")
            .field("refusal", &self.details.refusal)
            .finish_non_exhaustive()
    }
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > fmt::Display for DraftingProgramGarmentRootInputMemberError<'_, '_, '_, '_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "drafting garment Root input members: {:?}",
            self.details.refusal
        )
    }
}
impl<
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + DraftingRuleProvider<H> + ?Sized,
    > std::error::Error
    for DraftingProgramGarmentRootInputMemberError<'_, '_, '_, '_, '_, '_, I, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.details.refusal {
            DraftingProgramGarmentRootInputMemberRefusal::Resources(cause) => Some(cause),
            DraftingProgramGarmentRootInputMemberRefusal::Reservation { cause, .. } => Some(cause),
            DraftingProgramGarmentRootInputMemberRefusal::Work(cause) => {
                std::error::Error::source(cause)
            }
            DraftingProgramGarmentRootInputMemberRefusal::Lookup { cause, .. } => {
                std::error::Error::source(cause)
            }
            DraftingProgramGarmentRootInputMemberRefusal::Parameter { cause, .. } => Some(cause),
            _ => None,
        }
    }
}
