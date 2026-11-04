//! Complete original parent/request/response and original resource/provider/lending member causes.
use super::{
    BoundDraftingProfileSource, DraftingProfileLogicalProvider, DraftingProfileLogicalResponse,
};
use crate::{
    design::{DesignMetadataHost, DesignResourceLimitError, DesignResourceLimits},
    ontology::ProfileParameterRef,
    value::{LogicalInputKind, LogicalInputLookupError},
};
use core::fmt;
/// First original returned-header/parameter/member association mismatch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingProfileLogicalIssue {
    /// Returned actual Design owner differs.
    Root,
    /// Returned source uses another pin owner,even with identical labels.
    SelectedPin,
    /// Returned source uses another canonical inventory,even with equal definitions.
    Inventory,
    /// Returned original Profile payload record differs.
    Profile,
    /// Returned snapshot record differs.
    Snapshot,
    /// Original returned metadata field names another parameter.
    Parameter,
    /// Returned declaration is detached from the actual canonical member,even with equal fields.
    Declaration,
}
/// Original first content,typed provider,actual association or lending canonical cause.
#[derive(Debug)]
pub enum DraftingProfileLogicalRefusal<'a, E> {
    /// First complete held/requested content refusal before parameter lookup.
    Resources(DesignResourceLimitError),
    /// Original typed missing/unsupported/retired/current parameter source cause.
    Source(E),
    /// First original returned-header/parameter/declaration owner mismatch.
    Mismatch(DraftingProfileLogicalIssue),
    /// Original actual Profile inventory missing/kind cause,including actual member loan.
    Canonical(LogicalInputLookupError<'a>),
}
/// Original selected parent,request/policy,returned source when present,and exact typed cause.
pub struct DraftingProfileLogicalError<
    'b,
    'a,
    H: DesignMetadataHost,
    S: DraftingProfileLogicalProvider<H> + ?Sized,
> {
    details: Box<Failure<'b, 'a, H, S>>,
}
struct Failure<'b, 'a, H: DesignMetadataHost, S: DraftingProfileLogicalProvider<H> + ?Sized> {
    parent: &'b BoundDraftingProfileSource<'a, H, S>,
    parameter: ProfileParameterRef,
    kind: LogicalInputKind,
    limits: DesignResourceLimits,
    returned: Option<DraftingProfileLogicalResponse<'a, H, S>>,
    refusal: DraftingProfileLogicalRefusal<'a, S::Error>,
}
impl<'b, 'a, H: DesignMetadataHost, S: DraftingProfileLogicalProvider<H> + ?Sized>
    DraftingProfileLogicalError<'b, 'a, H, S>
{
    pub(super) fn new(
        parent: &'b BoundDraftingProfileSource<'a, H, S>,
        parameter: ProfileParameterRef,
        kind: LogicalInputKind,
        limits: DesignResourceLimits,
        returned: Option<DraftingProfileLogicalResponse<'a, H, S>>,
        refusal: DraftingProfileLogicalRefusal<'a, S::Error>,
    ) -> Self {
        Self {
            details: Box::new(Failure {
                parent,
                parameter,
                kind,
                limits,
                returned,
                refusal,
            }),
        }
    }
    /// Exact original selected Profile parent and its original Root/request/source loans.
    #[must_use]
    pub const fn parent(&self) -> &'b BoundDraftingProfileSource<'a, H, S> {
        self.details.parent
    }
    /// Original requested parameter identity.
    #[must_use]
    pub const fn parameter(&self) -> ProfileParameterRef {
        self.details.parameter
    }
    /// Original expected canonical logical role.
    #[must_use]
    pub const fn kind(&self) -> LogicalInputKind {
        self.details.kind
    }
    /// Original explicit fresh complete content policy.
    #[must_use]
    pub const fn limits(&self) -> DesignResourceLimits {
        self.details.limits
    }
    /// Complete actual raw response,including detached/mismatched records;None before a response.
    #[must_use]
    pub const fn returned(&self) -> Option<DraftingProfileLogicalResponse<'a, H, S>> {
        self.details.returned
    }
    /// Original first typed cause;canonical member witnesses retain their actual source lifetime.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingProfileLogicalRefusal<'a, S::Error> {
        &self.details.refusal
    }
}
impl<H: DesignMetadataHost, S: DraftingProfileLogicalProvider<H> + ?Sized> fmt::Debug
    for DraftingProfileLogicalError<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProfileLogicalError")
            .finish_non_exhaustive()
    }
}
impl<H: DesignMetadataHost, S: DraftingProfileLogicalProvider<H> + ?Sized> fmt::Display
    for DraftingProfileLogicalError<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("original Profile logical parameter source refused")
    }
}
impl<H: DesignMetadataHost, S: DraftingProfileLogicalProvider<H> + ?Sized> std::error::Error
    for DraftingProfileLogicalError<'_, '_, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.refusal() {
            DraftingProfileLogicalRefusal::Resources(e) => Some(e),
            DraftingProfileLogicalRefusal::Source(e) => Some(e),
            DraftingProfileLogicalRefusal::Mismatch(_)
            | DraftingProfileLogicalRefusal::Canonical(_) => None,
        }
    }
}
