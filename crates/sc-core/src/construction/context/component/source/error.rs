//! Exact original work/returned source witnesses and first complete association refusal.
use super::{
    Admission, AdmittedDraftingComponentWork, DraftingComponentBindingShape,
    DraftingComponentProvider, DraftingComponentResponse,
};
use crate::{
    construction::DraftingComponentWorkError,
    design::{DesignReferenceHost, DesignResourceDemand},
    ontology::SeamSideId,
};
use core::fmt;
/// First actual current header or complete captured recipe association mismatch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingComponentSourceIssue {
    /// Origin substitutes another actual Piece,even with equal identity/content.
    Piece,
    /// Component header substitutes another original Cartesian frame record.
    Frame,
    /// Component header substitutes another original containing snapshot record.
    Snapshot,
    /// Original recipe context uses another frame;no implicit transform is inferred.
    RecipeFrame,
    /// Genuine creator declaration names a different result Piece.
    CreatorPiece,
    /// A native source Piece is not the actual captured Root member.
    RecipePiece {
        /// None for Mirror;canonical A/B source for Join.
        side: Option<SeamSideId>,
    },
    /// Join's actual inventory/source slice is not the captured Root's original Piece inventory.
    RecipeInventory,
    /// Join's actual original CutPlan differs from its captured Root's plan.
    RecipeCutPlan,
    /// No actual native program member is the creator's exact original input loan.
    MissingCreator,
    /// The creator's exact original input occurs more than once in the mixed sequence.
    RepeatedCreator {
        /// First genuine one-based mixed position.
        first: usize,
        /// Second genuine one-based mixed position,without ordinal reconstruction.
        second: usize,
    },
}
/// Original first fresh work/provider cause or exact complete association mismatch.
pub enum DraftingComponentSourceRefusal<
    'g,
    'a,
    H: DesignReferenceHost,
    S: DraftingComponentProvider<H> + ?Sized,
> {
    /// Original fresh content/work/current-member refusal;no component query occurred.
    Work(DraftingComponentWorkError<'g, 'a, H, S>),
    /// Original typed component provider cause.
    Source(S::Error),
    /// First actual original header/recipe/creator mismatch.
    Mismatch(DraftingComponentSourceIssue),
}
impl<H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> fmt::Debug
    for DraftingComponentSourceRefusal<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Work(cause) => f.debug_tuple("Work").field(cause).finish(),
            Self::Source(cause) => f.debug_tuple("Source").field(cause).finish(),
            Self::Mismatch(issue) => f.debug_tuple("Mismatch").field(issue).finish(),
        }
    }
}
/// Boxed original work,fresh admission,complete raw response and first cause;no accepted prefix.
pub struct DraftingComponentSourceError<
    'w,
    'g,
    'a,
    H: DesignReferenceHost,
    S: DraftingComponentProvider<H> + ?Sized,
> {
    details: Box<Failure<'w, 'g, 'a, H, S>>,
}
struct Failure<'w, 'g, 'a, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> {
    work: &'w AdmittedDraftingComponentWork<'g, 'a, H, S>,
    fresh: Option<Admission>,
    returned: Option<DraftingComponentResponse<'a, H, S>>,
    refusal: DraftingComponentSourceRefusal<'g, 'a, H, S>,
}
impl<'w, 'g, 'a, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized>
    DraftingComponentSourceError<'w, 'g, 'a, H, S>
{
    pub(super) fn new(
        work: &'w AdmittedDraftingComponentWork<'g, 'a, H, S>,
        fresh: Option<Admission>,
        returned: Option<DraftingComponentResponse<'a, H, S>>,
        refusal: DraftingComponentSourceRefusal<'g, 'a, H, S>,
    ) -> Self {
        Self {
            details: Box::new(Failure {
                work,
                fresh,
                returned,
                refusal,
            }),
        }
    }
    /// Exact original work parent,including actual current copy/Piece and selected policy.
    #[must_use]
    pub const fn work(&self) -> &'w AdmittedDraftingComponentWork<'g, 'a, H, S> {
        self.details.work
    }
    /// Complete fresh demand when repeated work admission succeeded.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        match self.details.fresh {
            Some((demand, _)) => Some(demand),
            None => None,
        }
    }
    /// Complete fresh matching shape when repeated work admission succeeded.
    #[must_use]
    pub const fn shape(&self) -> Option<DraftingComponentBindingShape> {
        match self.details.fresh {
            Some((_, shape)) => Some(shape),
            None => None,
        }
    }
    /// Complete original returned response,including mismatched fields,when query succeeded.
    #[must_use]
    pub const fn returned(&self) -> Option<DraftingComponentResponse<'a, H, S>> {
        self.details.returned
    }
    /// Original first fresh work/provider cause or complete association mismatch.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingComponentSourceRefusal<'g, 'a, H, S> {
        &self.details.refusal
    }
}
impl<H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> fmt::Debug
    for DraftingComponentSourceError<'_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingComponentSourceError")
            .field("refusal", self.refusal())
            .finish_non_exhaustive()
    }
}
impl<H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> fmt::Display
    for DraftingComponentSourceError<'_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "drafting component source: {:?}", self.refusal())
    }
}
impl<H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> std::error::Error
    for DraftingComponentSourceError<'_, '_, '_, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.refusal() {
            DraftingComponentSourceRefusal::Work(cause) => std::error::Error::source(cause),
            DraftingComponentSourceRefusal::Source(cause) => Some(cause),
            DraftingComponentSourceRefusal::Mismatch(_) => None,
        }
    }
}
