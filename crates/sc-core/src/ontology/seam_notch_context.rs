//! Actual borrowed source validation,original-notch context and separately admitted exact quantities.
use super::{
    CutPlan, EaseDistribution, EntityId, IdentityLedger, OriginalPointLimits, Piece,
    PointJournalAdmissionIssue, SeamEaseSource, SeamSideId, SewingLandmark,
};
use core::{fmt, num::NonZeroUsize};
mod context;
mod inputs;
pub use context::{OriginalNotchContext, OriginalNotchContextError, OriginalNotchContextIssue};
mod quantity;
pub use quantity::{
    NotchEaseAdmission, NotchEaseAmountError, NotchEaseLimits, NotchEaseWorkspaceError,
};
mod validation;
pub use validation::{
    NotchSourceRange, NotchSourceValidation, NotchSourceValidationError, NotchSourceValidationIssue,
};

/// Explicit input and per-query workspace limits for original semantic-notch context construction.
/// No default is supplied. Source dependency items count complete borrowed collections;original-point
/// and range-portion limits govern their later respective queries,not an aggregate allocator/CPU cap.
///
/// ```compile_fail
/// use sc_core::ontology::NotchContextLimits;
/// let limits: NotchContextLimits = Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NotchContextLimits {
    dependency_items: NonZeroUsize,
    original_point: OriginalPointLimits,
    range_portions: NonZeroUsize,
}
impl NotchContextLimits {
    /// Choose complete input,original-point and peak per-range query budgets explicitly.
    #[must_use]
    pub const fn new(
        dependency_items: NonZeroUsize,
        original_point: OriginalPointLimits,
        range_portions: NonZeroUsize,
    ) -> Self {
        Self {
            dependency_items,
            original_point,
            range_portions,
        }
    }
    /// Maximum aggregate dependency items,including unused Pieces,edges and registry entries.
    #[must_use]
    pub const fn dependency_items(self) -> NonZeroUsize {
        self.dependency_items
    }
    /// Complete journal/nested input and per-point inverse workspace limits.
    #[must_use]
    pub const fn original_point(self) -> OriginalPointLimits {
        self.original_point
    }
    /// Peak portions per later range query,including its active input and transient output.
    #[must_use]
    pub const fn range_portions(self) -> NonZeroUsize {
        self.range_portions
    }
}

/// Actual variable-length dependency collection,in schema traversal order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotchSourceCollection {
    /// Every original stop,including those not selected by the distribution.
    SourceStops,
    /// Every physical copy in the actual plan.
    CutCopies,
    /// Complete supplied Piece registry.
    Pieces,
    /// Original outer boundary edges.
    PieceBoundary,
    /// Original hole-loop collection.
    PieceHoles,
    /// All edges of one original hole loop.
    PieceHoleEdges,
    /// Original construction-line edges.
    PieceConstructionLines,
    /// Complete original fold-edge declarations.
    PieceFoldEdges,
    /// Complete supplied semantic registry,including unused Notch and Turn entries.
    Landmarks,
}

/// Actual non-notch intent or whole-input refusal,before indexes or derived resolution.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotchSourceAdmissionIssue {
    /// Actual original intent is uniform.
    Uniform,
    /// Actual original intent is weighted;the complete source remains borrowed by the caller.
    Weighted {
        /// Actual density side.
        side: SeamSideId,
        /// Actual complete region count;no region body is cloned.
        regions: usize,
    },
    /// Complete journal/nested admission refusal,with its original operation and collection.
    Journal(PointJournalAdmissionIssue),
    /// A whole next dependency collection exceeds the aggregate selected limit.
    Items {
        /// Actual collection.
        collection: NotchSourceCollection,
        /// Original source or Piece owner,when the collection has one.
        owner: Option<EntityId>,
        /// Authored Piece index,or hole index for PieceHoleEdges.
        index: Option<usize>,
        /// Previously admitted population.
        consumed: usize,
        /// Complete next population;never a selected subset.
        requested: usize,
        /// Actual selected dependency limit.
        limit: usize,
    },
    /// Adding the complete next population cannot be represented.
    CountOverflow {
        /// Actual collection.
        collection: NotchSourceCollection,
        /// Original source or Piece owner,when present.
        owner: Option<EntityId>,
        /// Original Piece or hole index,when present.
        index: Option<usize>,
        /// Previously admitted population.
        consumed: usize,
        /// Complete next request.
        requested: usize,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Failure {
    source_span: EntityId,
    issue: NotchSourceAdmissionIssue,
}

/// Fixed original source identity and actual admission cause;no cloned source or partial index.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NotchSourceAdmissionError(Box<Failure>);
impl NotchSourceAdmissionError {
    /// Actual original span,including a retained source that is no longer a live graph member.
    #[must_use]
    pub const fn source_span(&self) -> EntityId {
        self.0.source_span
    }
    /// Actual original intent or whole-input refusal.
    #[must_use]
    pub const fn issue(&self) -> &NotchSourceAdmissionIssue {
        &self.0.issue
    }
}
impl fmt::Display for NotchSourceAdmissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "notch ease source {} input admission: {:?}",
            self.source_span(),
            self.issue()
        )
    }
}
impl std::error::Error for NotchSourceAdmissionError {}

/// Sealed borrowed actual inputs after eligibility and complete population admission only.
/// No indexes,wide integers,range/point resolutions or owned dependency copies are constructed.
/// Success does not prove ownership,currentness,unique original positions or quantity availability.
/// Symbolic amount and profile declarations remain unchanged for their actual later resolvers.
///
/// The source and every supplied dependency remain immutably borrowed:
/// ```compile_fail
/// use sc_core::ontology::{CutPlan, IdentityLedger, NotchContextLimits, NotchSourceAdmission,
///     Piece, SeamEaseSource, SewingLandmark};
/// fn escape<'a>(source: &'a SeamEaseSource, plan: &'a CutPlan, pieces: &'a [Piece],
///     marks: &'a [SewingLandmark], ledger: &'a IdentityLedger,
///     limits: NotchContextLimits) -> NotchSourceAdmission<'static> {
///     source.admit_notch_context(plan, pieces, marks, ledger, limits).unwrap()
/// }
/// ```
#[derive(Clone, Copy)]
pub struct NotchSourceAdmission<'a> {
    source: &'a SeamEaseSource,
    plan: &'a CutPlan,
    pieces: &'a [Piece],
    landmarks: &'a [SewingLandmark],
    ledger: &'a IdentityLedger,
    side: SeamSideId,
    start: EntityId,
    end: EntityId,
    dependency_items: usize,
    limits: NotchContextLimits,
}
impl fmt::Debug for NotchSourceAdmission<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            source,
            plan,
            pieces,
            landmarks,
            ledger,
            side,
            start,
            end,
            dependency_items,
            limits,
        } = self;
        f.debug_struct("NotchSourceAdmission")
            .field("source_span", &source.span())
            .field("copies", &plan.copies().len())
            .field("pieces", &pieces.len())
            .field("landmarks", &landmarks.len())
            .field("journal_entries", &ledger.journal().len())
            .field("side", side)
            .field("start", start)
            .field("end", end)
            .field("dependency_items", dependency_items)
            .field("limits", limits)
            .finish()
    }
}
impl<'a> NotchSourceAdmission<'a> {
    /// Actual complete original source,never a child declaration or supplied coordinate.
    #[must_use]
    pub const fn source(self) -> &'a SeamEaseSource {
        self.source
    }
    /// Actual complete physical-copy plan.
    #[must_use]
    pub const fn plan(self) -> &'a CutPlan {
        self.plan
    }
    /// Complete Piece registry,including entries not selected by either side.
    #[must_use]
    pub const fn pieces(self) -> &'a [Piece] {
        self.pieces
    }
    /// Complete semantic registry in original supplied order.
    #[must_use]
    pub const fn landmarks(self) -> &'a [SewingLandmark] {
        self.landmarks
    }
    /// Actual immutable topology journal,not a reconstructed coordinate provider.
    #[must_use]
    pub const fn ledger(self) -> &'a IdentityLedger {
        self.ledger
    }
    /// Actual original declaration's named side.
    #[must_use]
    pub const fn distribution_side(self) -> SeamSideId {
        self.side
    }
    /// Actual start/end IDs,in named order;no registry lookup or coordinate ordering is implied.
    #[must_use]
    pub const fn notch_ids(self) -> (EntityId, EntityId) {
        (self.start, self.end)
    }
    /// Complete admitted dependency population;separate from journal/nested input populations.
    #[must_use]
    pub const fn dependency_item_count(self) -> usize {
        self.dependency_items
    }
    /// All actual explicitly selected input and later query budgets.
    #[must_use]
    pub const fn limits(self) -> NotchContextLimits {
        self.limits
    }
}
impl SeamEaseSource {
    /// Admit actual complete notch-source inputs before indexes or derived reference resolution.
    ///
    /// Order is original distribution eligibility,complete journal/nested inputs,then complete
    /// dependency collections in schema order. No amount evaluation or physical binding occurs.
    /// Piece label/Material text is borrowed without scanning or cloning;this is a dependency-item
    /// census,not a byte/allocator/CPU or complete Design-authority certificate.
    ///
    /// # Errors
    /// Returns actual non-notch intent,then original journal or first whole dependency refusal.
    pub fn admit_notch_context<'a>(
        &'a self,
        plan: &'a CutPlan,
        pieces: &'a [Piece],
        landmarks: &'a [SewingLandmark],
        ledger: &'a IdentityLedger,
        limits: NotchContextLimits,
    ) -> Result<NotchSourceAdmission<'a>, NotchSourceAdmissionError> {
        let failure = |issue| {
            NotchSourceAdmissionError(Box::new(Failure {
                source_span: self.span(),
                issue,
            }))
        };
        let (side, start, end) = match &self.declaration().distribution {
            EaseDistribution::Uniform => return Err(failure(NotchSourceAdmissionIssue::Uniform)),
            EaseDistribution::Weighted { side, regions } => {
                return Err(failure(NotchSourceAdmissionIssue::Weighted {
                    side: *side,
                    regions: regions.len(),
                }))
            }
            EaseDistribution::BetweenNotches { side, start, end } => (*side, *start, *end),
        };
        limits
            .original_point
            .current()
            .admit_journal(ledger)
            .map_err(|issue| failure(NotchSourceAdmissionIssue::Journal(issue)))?;
        let dependency_items =
            inputs::admit(self, plan, pieces, landmarks, limits.dependency_items.get())
                .map_err(failure)?;
        Ok(NotchSourceAdmission {
            source: self,
            plan,
            pieces,
            landmarks,
            ledger,
            side,
            start,
            end,
            dependency_items,
            limits,
        })
    }
}
