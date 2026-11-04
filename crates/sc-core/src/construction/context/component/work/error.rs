//! Exact original component target/current parent and first content/work/member refusal.
use super::{
    AdmittedDraftingGarmentSource, DraftingComponentBindingLimits, DraftingComponentProvider,
};
use crate::{
    design::{DesignReferenceHost, DesignResourceLimitError},
    ontology::{CutCopy, PocketPieceRef},
};
use core::fmt;
/// First complete fresh resource/work or actual current copy/Piece association cause.
#[derive(Clone, Copy, Debug)]
pub enum DraftingComponentWorkRefusal<'a> {
    /// Original first complete actual garment/provider field refusal.
    Resources(DesignResourceLimitError),
    /// Complete E+8 or square cannot fit usize.
    PopulationOverflow,
    /// Complete conservative matching bound exceeds its explicit selected allowance.
    MatchingInspections {
        /// Exact selected maximum.
        allowed: usize,
        /// Complete conservative original requirement.
        required: usize,
    },
    /// No actual current CutPlan copy has the requested identity.
    MissingCopy,
    /// Actual current copy names another Piece;the exact original member remains loaned.
    CopyPiece {
        /// Original actual current CutPlan member,including its unchanged source/orientation.
        actual: &'a CutCopy,
    },
    /// No actual current Piece inventory member has the requested source identity.
    MissingPiece,
}
/// Exact original parent/target/policy with the first original content,work or member cause.
pub struct DraftingComponentWorkError<
    'g,
    'a,
    H: DesignReferenceHost,
    S: DraftingComponentProvider<H> + ?Sized,
> {
    details: Box<Failure<'g, 'a, H, S>>,
}
struct Failure<'g, 'a, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> {
    parent: &'g AdmittedDraftingGarmentSource<'a, H, S>,
    target: PocketPieceRef,
    limits: DraftingComponentBindingLimits,
    refusal: DraftingComponentWorkRefusal<'a>,
}
impl<'g, 'a, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized>
    DraftingComponentWorkError<'g, 'a, H, S>
{
    pub(super) fn new(
        parent: &'g AdmittedDraftingGarmentSource<'a, H, S>,
        target: PocketPieceRef,
        limits: DraftingComponentBindingLimits,
        refusal: DraftingComponentWorkRefusal<'a>,
    ) -> Self {
        Self {
            details: Box::new(Failure {
                parent,
                target,
                limits,
                refusal,
            }),
        }
    }
    /// Exact original admitted current garment/context/source parent.
    #[must_use]
    pub const fn parent(&self) -> &'g AdmittedDraftingGarmentSource<'a, H, S> {
        self.details.parent
    }
    /// Exact original explicit copy/expected-Piece request,including on missing membership.
    #[must_use]
    pub const fn target(&self) -> PocketPieceRef {
        self.details.target
    }
    /// Exact selected fresh complete content/work policy.
    #[must_use]
    pub const fn limits(&self) -> DraftingComponentBindingLimits {
        self.details.limits
    }
    /// Original first field/population/work or actual current member refusal.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingComponentWorkRefusal<'a> {
        &self.details.refusal
    }
}
impl<H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> fmt::Debug
    for DraftingComponentWorkError<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingComponentWorkError")
            .field("target", &self.details.target)
            .field("refusal", self.refusal())
            .finish_non_exhaustive()
    }
}
impl<H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> fmt::Display
    for DraftingComponentWorkError<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "drafting component work: {:?}", self.refusal())
    }
}
impl<H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> std::error::Error
    for DraftingComponentWorkError<'_, '_, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.refusal() {
            DraftingComponentWorkRefusal::Resources(cause) => Some(cause),
            _ => None,
        }
    }
}
