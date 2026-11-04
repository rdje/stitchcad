//! Original requested IDs/kinds and actual lending inventory causes.
use super::{DraftingContextSource, DraftingDesignSemanticScope};
use crate::{
    design::DesignMetadataHost,
    ontology::EntityId,
    recipe::{SemanticOperationKind, SemanticOperationLookupError},
    value::{LogicalInputLookupError, LogicalInputTarget},
};
use core::fmt;

/// Original typed request,not a formula name,source ordinal or caller resource claim.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingDesignSemanticRequest {
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
    pub(super) scope: &'s DraftingDesignSemanticScope<'a, H, S>,
    pub(super) request: DraftingDesignSemanticRequest,
    pub(super) cause: DraftingDesignSemanticLookupCause<'a>,
}
impl<'s, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingDesignSemanticLookupError<'s, 'a, H, S>
{
    /// Exact original fresh source scope and its actual context/policy.
    #[must_use]
    pub const fn scope(&self) -> &'s DraftingDesignSemanticScope<'a, H, S> {
        self.scope
    }
    /// Exact original requested identity/semantic kind.
    #[must_use]
    pub const fn request(&self) -> DraftingDesignSemanticRequest {
        self.request
    }
    /// Original typed missing/kind cause;borrowed member witnesses remain inspectable.
    #[must_use]
    pub const fn cause(&self) -> DraftingDesignSemanticLookupCause<'a> {
        self.cause
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingDesignSemanticLookupError<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingDesignSemanticLookupError")
            .field("request", &self.request)
            .field("cause", &self.cause)
            .finish_non_exhaustive()
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Display
    for DraftingDesignSemanticLookupError<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.cause {
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
