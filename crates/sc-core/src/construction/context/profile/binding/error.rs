//! Original content/provider/mismatch causes and complete original request/response witnesses.
use super::{DraftingContext, DraftingProfileProvider, DraftingProfileSource};
use crate::{
    design::{DesignMetadataHost, DesignResourceLimitError, DesignResourceLimits},
    ontology::{SelectedProfilePin, SelectedProfilePinField},
};
use core::fmt;

/// First returned-source association mismatch,after complete content and provider lookup.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingProfileSourceIssue {
    /// Returned Design owner differs,including equal-ID/equal-revision foreign roots.
    Root,
    /// First mismatching profile/schema/content label;labels are not normalized.
    SelectedPin(SelectedProfilePinField),
    /// Returned original snapshot record differs from the containing context snapshot.
    Snapshot,
}
/// Original first content,typed provider or actual source association refusal.
#[derive(Debug)]
pub enum DraftingProfileSourceRefusal<E> {
    /// First actual held/requested field excess before lookup.
    Resources(DesignResourceLimitError),
    /// Provider's original typed missing,retired,unsupported or current-source cause.
    Source(E),
    /// First Root,pin-field or snapshot mismatch,with returned records retained separately.
    Mismatch(DraftingProfileSourceIssue),
}
/// Actual context/request/policy,returned source when present,and original typed cause.
pub struct DraftingProfileSourceError<
    'a,
    H: DesignMetadataHost,
    S: DraftingProfileProvider<H> + ?Sized,
> {
    details: Box<Failure<'a, H, S>>,
}
struct Failure<'a, H: DesignMetadataHost, S: DraftingProfileProvider<H> + ?Sized> {
    context: DraftingContext<'a, H, S>,
    selected: &'a SelectedProfilePin,
    limits: DesignResourceLimits,
    returned: Option<DraftingProfileSource<'a, H, S::Snapshot, S::Profile>>,
    refusal: DraftingProfileSourceRefusal<S::Error>,
}
impl<'a, H: DesignMetadataHost, S: DraftingProfileProvider<H> + ?Sized>
    DraftingProfileSourceError<'a, H, S>
{
    pub(super) fn new(
        context: DraftingContext<'a, H, S>,
        selected: &'a SelectedProfilePin,
        limits: DesignResourceLimits,
        returned: Option<DraftingProfileSource<'a, H, S::Snapshot, S::Profile>>,
        refusal: DraftingProfileSourceRefusal<S::Error>,
    ) -> Self {
        Self {
            details: Box::new(Failure {
                context,
                selected,
                limits,
                returned,
                refusal,
            }),
        }
    }
    /// Original containing context and Root/provider/frame/snapshot loans.
    #[must_use]
    pub const fn context(&self) -> DraftingContext<'a, H, S> {
        self.details.context
    }
    /// Original requested pin,including its exact spellings.
    #[must_use]
    pub const fn selected(&self) -> &'a SelectedProfilePin {
        self.details.selected
    }
    /// Explicit original complete content allowance.
    #[must_use]
    pub const fn limits(&self) -> DesignResourceLimits {
        self.details.limits
    }
    /// Complete returned source,including foreign/mismatched records;None before a response.
    #[must_use]
    pub const fn returned(&self) -> Option<DraftingProfileSource<'a, H, S::Snapshot, S::Profile>> {
        self.details.returned
    }
    /// Original first content/provider/mismatch cause.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingProfileSourceRefusal<S::Error> {
        &self.details.refusal
    }
}
impl<H: DesignMetadataHost, S: DraftingProfileProvider<H> + ?Sized> fmt::Debug
    for DraftingProfileSourceError<'_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProfileSourceError")
            .finish_non_exhaustive()
    }
}
impl<H: DesignMetadataHost, S: DraftingProfileProvider<H> + ?Sized> fmt::Display
    for DraftingProfileSourceError<'_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("original selected Profile source refused")
    }
}
impl<H: DesignMetadataHost, S: DraftingProfileProvider<H> + ?Sized> std::error::Error
    for DraftingProfileSourceError<'_, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.refusal() {
            DraftingProfileSourceRefusal::Resources(cause) => Some(cause),
            DraftingProfileSourceRefusal::Source(cause) => Some(cause),
            DraftingProfileSourceRefusal::Mismatch(_) => None,
        }
    }
}
