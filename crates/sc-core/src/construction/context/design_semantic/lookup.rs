//! Original requested IDs/kinds and actual lending inventory causes.
use super::{DraftingContextSource, DraftingDesignSemanticScope};
use crate::{
    design::DesignMetadataHost,
    ontology::EntityId,
    recipe::{SemanticOperationKind, SemanticOperationLookupError},
    value::{
        CanonicalInputTarget, CanonicalNumericInputError, LogicalInputLookupError,
        LogicalInputTarget,
    },
};
use core::fmt;

/// Original typed request,not a formula name,source ordinal or caller resource claim.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingDesignSemanticRequest {
    /// Original dedicated-length/general-scalar target,without fabricated formula metadata.
    Numeric(CanonicalInputTarget),
    /// Original logical declaration and expected semantic role.
    Logical(LogicalInputTarget),
    /// Original semantic operation identity and expected family.
    Operation {
        /// Actual original requested operation ID.
        operation: EntityId,
        /// Original expected semantic family.
        kind: SemanticOperationKind,
    },
}
/// Original inventory refusal,including actual borrowed member on a kind mismatch.
#[derive(Clone, Copy, Debug)]
pub enum DraftingDesignSemanticLookupCause<'a> {
    /// Actual original canonical numeric target/family/kind refusal with inventory/member loans.
    Numeric(CanonicalNumericInputError<'a>),
    /// Actual canonical logical inventory cause,without state/domain copying.
    Logical(LogicalInputLookupError<'a>),
    /// Actual Root semantic recipe cause,without body copying.
    Operation(SemanticOperationLookupError<'a>),
}
/// Original source scope,request and typed lending cause;no accepted member/prefix on failure.
pub struct DraftingDesignSemanticLookupError<
    's,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    details: Box<LookupFailure<'s, 'a, H, S>>,
}
struct LookupFailure<'s, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> {
    scope: &'s DraftingDesignSemanticScope<'a, H, S>,
    request: DraftingDesignSemanticRequest,
    cause: DraftingDesignSemanticLookupCause<'a>,
}
impl<'s, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingDesignSemanticLookupError<'s, 'a, H, S>
{
    pub(super) fn new(
        scope: &'s DraftingDesignSemanticScope<'a, H, S>,
        request: DraftingDesignSemanticRequest,
        cause: DraftingDesignSemanticLookupCause<'a>,
    ) -> Self {
        Self {
            details: Box::new(LookupFailure {
                scope,
                request,
                cause,
            }),
        }
    }
    /// Exact original fresh source scope and its actual context/policy.
    #[must_use]
    pub const fn scope(&self) -> &'s DraftingDesignSemanticScope<'a, H, S> {
        self.details.scope
    }
    /// Exact original requested identity/semantic kind.
    #[must_use]
    pub const fn request(&self) -> DraftingDesignSemanticRequest {
        self.details.request
    }
    /// Original typed missing/kind cause;borrowed member witnesses remain inspectable.
    #[must_use]
    pub const fn cause(&self) -> DraftingDesignSemanticLookupCause<'a> {
        self.details.cause
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingDesignSemanticLookupError<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingDesignSemanticLookupError")
            .field("request", &self.details.request)
            .field("cause", &self.details.cause)
            .finish_non_exhaustive()
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Display
    for DraftingDesignSemanticLookupError<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.details.cause {
            DraftingDesignSemanticLookupCause::Numeric(cause) => {
                write!(f, "Design numeric source: {cause}")
            }
            DraftingDesignSemanticLookupCause::Logical(cause) => {
                write!(f, "Design logical source: {cause}")
            }
            DraftingDesignSemanticLookupCause::Operation(cause) => {
                write!(f, "Design operation source: {cause}")
            }
        }
    }
}
// Lending member witnesses are exposed by cause(),not erased into a 'static error chain.
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> std::error::Error
    for DraftingDesignSemanticLookupError<'_, '_, H, S>
{
}
