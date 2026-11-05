//! Complete original parent/request/response and original resource/provider/lending member causes.
use super::{
    BoundDraftingProfileSource, DraftingProfileNumericProvider, DraftingProfileNumericResponse,
};
use crate::{
    design::{DesignMetadataHost, DesignResourceLimitError, DesignResourceLimits},
    ontology::ProfileParameterRef,
    value::{CanonicalNumericInputError, ScalarInputKind},
};
use core::fmt;
/// First original returned-header/parameter/member association mismatch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingProfileNumericIssue {
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
}
/// Original first content,typed provider,actual association or lending canonical cause.
#[derive(Debug)]
pub enum DraftingProfileNumericRefusal<'a, E> {
    /// First complete held/requested content refusal before parameter lookup.
    Resources(DesignResourceLimitError),
    /// Original typed missing/unsupported/retired/current parameter source cause.
    Source(E),
    /// First original returned-header/parameter owner mismatch.
    Mismatch(DraftingProfileNumericIssue),
    /// Original actual Profile inventory missing/kind cause,including actual member loan.
    Canonical(CanonicalNumericInputError<'a>),
    /// Actual canonical member has another numeric kind after exact target membership succeeds.
    Kind {
        /// Actual original member kind;expected kind is retained by the containing error.
        actual: ScalarInputKind,
    },
}
/// Original selected parent,request/policy,returned source when present,and exact typed cause.
pub struct DraftingProfileNumericError<
    'b,
    'a,
    H: DesignMetadataHost,
    S: DraftingProfileNumericProvider<H> + ?Sized,
> {
    details: Box<Failure<'b, 'a, H, S>>,
}
struct Failure<'b, 'a, H: DesignMetadataHost, S: DraftingProfileNumericProvider<H> + ?Sized> {
    parent: &'b BoundDraftingProfileSource<'a, H, S>,
    parameter: ProfileParameterRef,
    kind: ScalarInputKind,
    limits: DesignResourceLimits,
    returned: Option<DraftingProfileNumericResponse<'a, H, S>>,
    refusal: DraftingProfileNumericRefusal<'a, S::Error>,
}
impl<'b, 'a, H: DesignMetadataHost, S: DraftingProfileNumericProvider<H> + ?Sized>
    DraftingProfileNumericError<'b, 'a, H, S>
{
    pub(super) fn new(
        parent: &'b BoundDraftingProfileSource<'a, H, S>,
        parameter: ProfileParameterRef,
        kind: ScalarInputKind,
        limits: DesignResourceLimits,
        returned: Option<DraftingProfileNumericResponse<'a, H, S>>,
        refusal: DraftingProfileNumericRefusal<'a, S::Error>,
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
    /// Original expected canonical numeric role.
    #[must_use]
    pub const fn kind(&self) -> ScalarInputKind {
        self.details.kind
    }
    /// Original explicit fresh complete content policy.
    #[must_use]
    pub const fn limits(&self) -> DesignResourceLimits {
        self.details.limits
    }
    /// Complete actual raw response,including detached/mismatched records;None before a response.
    #[must_use]
    pub const fn returned(&self) -> Option<DraftingProfileNumericResponse<'a, H, S>> {
        self.details.returned
    }
    /// Original first typed cause;canonical member witnesses retain their actual source lifetime.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingProfileNumericRefusal<'a, S::Error> {
        &self.details.refusal
    }
}
impl<H: DesignMetadataHost, S: DraftingProfileNumericProvider<H> + ?Sized> fmt::Debug
    for DraftingProfileNumericError<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProfileNumericError")
            .finish_non_exhaustive()
    }
}
impl<H: DesignMetadataHost, S: DraftingProfileNumericProvider<H> + ?Sized> fmt::Display
    for DraftingProfileNumericError<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("original Profile numeric parameter source refused")
    }
}
impl<H: DesignMetadataHost, S: DraftingProfileNumericProvider<H> + ?Sized> std::error::Error
    for DraftingProfileNumericError<'_, '_, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.refusal() {
            DraftingProfileNumericRefusal::Resources(e) => Some(e),
            DraftingProfileNumericRefusal::Source(e) => Some(e),
            DraftingProfileNumericRefusal::Mismatch(_)
            | DraftingProfileNumericRefusal::Canonical(_)
            | DraftingProfileNumericRefusal::Kind { .. } => None,
        }
    }
}
