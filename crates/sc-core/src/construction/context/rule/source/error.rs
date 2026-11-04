//! Original work/Profile/provider causes and exact original expected/returned source witnesses.
use super::{
    Admission, AdmittedDraftingRuleWork, DraftingRuleBindingShape, DraftingRuleProvider,
    DraftingRuleResponse, Profile,
};
use crate::{
    construction::{DraftingProfileSourceError, DraftingRuleWorkError},
    design::{DesignMetadataHost, DesignResourceDemand},
};
use core::fmt;
/// First original rule-source header,origin or identity/family mismatch;witnesses stay in the error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingRuleSourceIssue {
    /// Returned source belongs to another actual containing Root.
    Root,
    /// Returned source holds a different original snapshot record.
    Snapshot,
    /// Returned Design/Profile origin differs from explicit requested origin.
    Origin,
    /// Profile origin substitutes an equal-data or foreign original selected pin.
    SelectedPin,
    /// Profile origin substitutes another canonical inventory.
    Inventory,
    /// Profile origin substitutes another original Profile payload record.
    Profile,
    /// Actual declaration identity differs from the operation's original rule/policy ID.
    Rule,
    /// Actual declaration family differs from the original operation's family.
    Kind,
}
/// Original first fresh work/Profile/provider cause or actual source mismatch.
pub enum DraftingRuleSourceRefusal<'a, H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> {
    /// Original complete fresh content/work/member refusal,without a source query.
    Work(DraftingRuleWorkError<'a, H, S>),
    /// Original selected Profile admission cause,without a rule query.
    Profile(DraftingProfileSourceError<'a, H, S>),
    /// Original typed readonly rule provider cause.
    Source(S::Error),
    /// First actual original source association mismatch.
    Mismatch(DraftingRuleSourceIssue),
}
impl<H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> fmt::Debug
    for DraftingRuleSourceRefusal<'_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Work(cause) => f.debug_tuple("Work").field(cause).finish(),
            Self::Profile(cause) => f.debug_tuple("Profile").field(cause).finish(),
            Self::Source(cause) => f.debug_tuple("Source").field(cause).finish(),
            Self::Mismatch(issue) => f.debug_tuple("Mismatch").field(issue).finish(),
        }
    }
}
/// Exact original work parent,fresh admission,expected Profile/returned rule and first refusal.
pub struct DraftingRuleSourceError<
    'w,
    'a,
    H: DesignMetadataHost,
    S: DraftingRuleProvider<H> + ?Sized,
> {
    details: Box<Failure<'w, 'a, H, S>>,
}
struct Failure<'w, 'a, H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> {
    work: &'w AdmittedDraftingRuleWork<'a, H, S>,
    fresh: Option<Admission>,
    profile: Option<Profile<'a, H, S>>,
    returned: Option<DraftingRuleResponse<'a, H, S>>,
    refusal: DraftingRuleSourceRefusal<'a, H, S>,
}
impl<'w, 'a, H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized>
    DraftingRuleSourceError<'w, 'a, H, S>
{
    pub(super) fn new(
        work: &'w AdmittedDraftingRuleWork<'a, H, S>,
        fresh: Option<Admission>,
        profile: Option<Profile<'a, H, S>>,
        returned: Option<DraftingRuleResponse<'a, H, S>>,
        refusal: DraftingRuleSourceRefusal<'a, H, S>,
    ) -> Self {
        Self {
            details: Box::new(Failure {
                work,
                fresh,
                profile,
                returned,
                refusal,
            }),
        }
    }
    /// Exact original work parent,including actual operation and original requested selection.
    #[must_use]
    pub const fn work(&self) -> &'w AdmittedDraftingRuleWork<'a, H, S> {
        self.details.work
    }
    /// Fresh complete demand when repeated work admission succeeded.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        match self.details.fresh {
            Some((demand, _)) => Some(demand),
            None => None,
        }
    }
    /// Fresh prospective shape when repeated work admission succeeded.
    #[must_use]
    pub const fn shape(&self) -> Option<DraftingRuleBindingShape> {
        match self.details.fresh {
            Some((_, shape)) => Some(shape),
            None => None,
        }
    }
    /// Original accepted selected Profile header when Profile admission succeeded.
    #[must_use]
    pub const fn profile_source(
        &self,
    ) -> Option<crate::construction::DraftingProfileSource<'a, H, S::Snapshot, S::Profile>> {
        self.details.profile
    }
    /// Original complete returned rule source,including mismatched fields,when lookup succeeded.
    #[must_use]
    pub const fn returned(&self) -> Option<DraftingRuleResponse<'a, H, S>> {
        self.details.returned
    }
    /// Original typed nested cause or exact first association mismatch.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingRuleSourceRefusal<'a, H, S> {
        &self.details.refusal
    }
}
impl<H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> fmt::Debug
    for DraftingRuleSourceError<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingRuleSourceError")
            .field("refusal", self.refusal())
            .finish_non_exhaustive()
    }
}
impl<H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> fmt::Display
    for DraftingRuleSourceError<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "drafting rule source: {:?}", self.refusal())
    }
}
impl<H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> std::error::Error
    for DraftingRuleSourceError<'_, '_, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.refusal() {
            DraftingRuleSourceRefusal::Work(cause) => std::error::Error::source(cause),
            DraftingRuleSourceRefusal::Profile(cause) => std::error::Error::source(cause),
            DraftingRuleSourceRefusal::Source(cause) => Some(cause),
            DraftingRuleSourceRefusal::Mismatch(_) => None,
        }
    }
}
