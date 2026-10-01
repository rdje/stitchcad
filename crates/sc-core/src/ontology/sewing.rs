//! Copy-addressed sewing intent; geometric walking and realized ease remain G2/G3 obligations.
use super::anchor::validate_anchor;
use super::{
    AnchorError, CutPlan, CutPlanError, EdgeAnchor, EdgeRange, EntityId, GeometricValidation,
    IdentityLedger, Notch, Param, Piece, ProfileParameterRef, RangePortion, RangeResolution,
    Rational, Resolution, ResolvedRef,
};
use core::fmt;
use sc_units::Length;
use std::collections::{BTreeMap, BTreeSet};

/// One born-valid semantic sewing turn, without claiming a geometric corner.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TurnPoint {
    id: EntityId,
    piece: EntityId,
    anchor: EdgeAnchor,
}
impl TurnPoint {
    /// Validate an exact semantic anchor on the named piece.
    /// # Errors
    /// Returns [`AnchorError`] for missing, unresolved or foreign anchors.
    pub fn new(
        id: EntityId,
        anchor: EdgeAnchor,
        piece: &Piece,
        ledger: &IdentityLedger,
    ) -> Result<Self, AnchorError> {
        validate_anchor(anchor, piece, ledger)?;
        Ok(Self {
            id,
            piece: piece.id(),
            anchor,
        })
    }
    /// Stable turn-point identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.id
    }
    /// Source Piece identity.
    #[must_use]
    pub const fn piece(&self) -> EntityId {
        self.piece
    }
    /// Unchanged held anchor.
    #[must_use]
    pub const fn anchor(&self) -> EdgeAnchor {
        self.anchor
    }
    /// G1 validates semantic anchoring, not actual geometric turning.
    #[must_use]
    pub const fn geometric_validation(&self) -> GeometricValidation {
        GeometricValidation::DeferredToG2
    }

    /// Current resolution, including explicit choices and repairs.
    #[must_use]
    pub fn resolve(&self, ledger: &IdentityLedger) -> Resolution {
        ledger.resolve(self.anchor.edge, self.anchor.param)
    }
}

/// Semantic stop registry entries; physical notch geometry is not needed to resolve a stop.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SewingLandmark {
    /// A semantic matching notch.
    Notch(Notch),
    /// A semantic sewing turn point.
    Turn(TurnPoint),
}
impl SewingLandmark {
    /// Stable landmark identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        match self {
            Self::Notch(n) => n.id(),
            Self::Turn(t) => t.id(),
        }
    }
    /// Source pattern Piece identity.
    #[must_use]
    pub const fn piece(&self) -> EntityId {
        match self {
            Self::Notch(n) => n.piece(),
            Self::Turn(t) => t.piece(),
        }
    }
    /// Resolve the landmark without changing its held anchor.
    #[must_use]
    pub fn resolve(&self, ledger: &IdentityLedger) -> Resolution {
        match self {
            Self::Notch(n) => n.resolve(ledger),
            Self::Turn(t) => t.resolve(ledger),
        }
    }
}

/// Which side carries a declaration or diagnostic.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SeamSideId {
    /// First side.
    A,
    /// Second side.
    B,
}
/// One positive-length source-frame interval on one identified physical copy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SeamSide {
    /// Stable physical-copy identity, not a pattern Piece id.
    pub copy: EntityId,
    /// Authored interval in the source Piece's frame.
    pub range: EdgeRange,
}
/// Endpoint correspondence, independent of topological traversal and physical-copy reflection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeamDirection {
    /// A's lower endpoint meets B's lower endpoint.
    Same,
    /// A's lower endpoint meets B's upper endpoint.
    Opposite,
}
/// Authored A-minus-B differential; symbolic sources supply no value at G1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EaseAmount {
    /// Explicit signed length; positive means A is longer than B.
    Explicit(Length),
    /// Formula parameter expected to resolve to a signed length.
    Formula(EntityId),
    /// Target-profile declaration expected to resolve to a signed length.
    Profile(ProfileParameterRef),
}
/// One positive-weight region of normalized span arc-length progress.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WeightedEaseRegion {
    /// Lower bound, inclusive.
    pub from: Param,
    /// Upper bound, inclusive; must exceed `from`.
    pub to: Param,
    /// Relative density; strictly positive, not a physical stretch factor.
    pub weight: Rational,
}
/// Explicit ease-allocation intent; no default or geometric resampling is supplied here.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EaseDistribution {
    /// Distribute the declared differential uniformly.
    Uniform,
    /// Ordered, non-overlapping weighted parameter regions; omitted regions receive no allocation.
    Weighted {
        /// Side whose normalized arc-length progress locates the regions.
        side: SeamSideId,
        /// Nonempty positive-weight regions in ascending order.
        regions: Vec<WeightedEaseRegion>,
    },
    /// Allocate between two distinct semantic notch anchors on an explicit side.
    BetweenNotches {
        /// Side carrying both notch stops.
        side: SeamSideId,
        /// First named notch stop.
        start: EntityId,
        /// Second named notch stop.
        end: EntityId,
    },
}
/// Signed differential and its explicitly declared distribution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeclaredEase {
    /// A-minus-B length, or its symbolic source.
    pub amount: EaseAmount,
    /// Authored distribution.
    pub distribution: EaseDistribution,
}
/// A stop on an explicit side; that side supplies the physical-copy identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StopLandmark {
    /// Side carrying the stop.
    pub side: SeamSideId,
    /// Semantic landmark identity in the supplied registry.
    pub landmark: EntityId,
}
/// Editable span input, distinct from validated immutable [`SeamSpan`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeamSpanDefinition {
    /// Stable correspondence identity.
    pub id: EntityId,
    /// First copy and range.
    pub a: SeamSide,
    /// Second copy and range.
    pub b: SeamSide,
    /// Which endpoints correspond.
    pub direction: SeamDirection,
    /// Differential and allocation intent; never implicit stretch.
    pub ease: DeclaredEase,
    /// Semantic stops, in caller-authored order.
    pub stops: Vec<StopLandmark>,
}
impl SeamSpanDefinition {
    fn side(&self, id: SeamSideId) -> SeamSide {
        match id {
            SeamSideId::A => self.a,
            SeamSideId::B => self.b,
        }
    }
}
/// Immutable structurally validated correspondence, with geometric obligations deferred.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeamSpan {
    definition: SeamSpanDefinition,
}
impl SeamSpan {
    /// Stable correspondence identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }
    /// Shared authored content; editing a clone does not mutate the span.
    #[must_use]
    pub const fn definition(&self) -> &SeamSpanDefinition {
        &self.definition
    }
    /// Whole-interval and endpoint evidence after edits; neither is silently reassigned.
    #[must_use]
    pub fn resolve_side(&self, side: SeamSideId, ledger: &IdentityLedger) -> RangeResolution {
        ledger.resolve_range(self.definition.side(side).range)
    }
}
/// Editable graph input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SewingGraphDefinition {
    /// Stable graph identity.
    pub id: EntityId,
    /// All oriented correspondences, in authored order.
    pub spans: Vec<SeamSpanDefinition>,
}
/// Immutable first-class sewing graph, separate from any renderer.
/// ```compile_fail
/// fn erase_seams(graph: &mut sc_core::ontology::SewingGraph) {
///     graph.spans.clear();
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SewingGraph {
    id: EntityId,
    spans: Vec<SeamSpan>,
}
/// Visible deferred obligation for every declared seam differential/distribution.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EaseValidation {
    /// G2/G3 must consume resolved parameter values, walk lengths and check distribution.
    DeferredToG2AndG3,
}
/// Why a weighted distribution is structurally invalid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeightIssue {
    /// No region was supplied.
    Empty,
    /// Bounds are equal or descending.
    InvalidBounds,
    /// Density is zero or negative.
    NonPositive,
    /// Regions overlap or are not in ascending order.
    UnorderedOrOverlapping,
}
/// A typed sewing construction refusal, retaining resolution evidence where applicable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SewingError {
    /// The cut plan no longer matches the supplied Piece collection.
    InvalidCutPlan(CutPlanError),
    /// An entity id is repeated across graph, spans, landmarks, copies or Pieces.
    IdentityCollision {
        /// Repeated identity.
        id: EntityId,
    },
    /// A span side names no physical copy.
    MissingCopy {
        /// Correspondence.
        span: EntityId,
        /// A or B.
        side: SeamSideId,
        /// Missing physical copy.
        copy: EntityId,
    },
    /// A born range has a lost interval, unknown source or unresolved endpoint.
    UnresolvedRange {
        /// Correspondence.
        span: EntityId,
        /// A or B.
        side: SeamSideId,
        /// Exact held-range, interval and endpoint evidence.
        evidence: Box<RangeResolution>,
    },
    /// A resolved interval includes material outside its source Piece.
    RangeOutsidePiece {
        /// Correspondence.
        span: EntityId,
        /// A or B.
        side: SeamSideId,
        /// Source Piece.
        piece: EntityId,
    },
    /// Same-copy ranges overlap in their current positive-length interiors.
    OverlappingSelfSeam {
        /// Correspondence.
        span: EntityId,
        /// Physical copy.
        copy: EntityId,
    },
    /// A stop id is absent from the semantic landmark registry.
    MissingLandmark {
        /// Correspondence.
        span: EntityId,
        /// Authored stop.
        stop: StopLandmark,
    },
    /// A landmark is repeated on the same side.
    DuplicateStop {
        /// Correspondence.
        span: EntityId,
        /// Repeated stop.
        stop: StopLandmark,
    },
    /// A stop belongs to another Piece, or lies outside its side interval.
    StopOutsideSide {
        /// Correspondence.
        span: EntityId,
        /// Authored stop.
        stop: StopLandmark,
    },
    /// A born stop is ambiguous or orphaned after edits.
    UnresolvedStop {
        /// Correspondence.
        span: EntityId,
        /// Authored stop.
        stop: StopLandmark,
        /// Exact resolution requiring choice/repair.
        evidence: Box<Resolution>,
    },
    /// Invalid weighted ease regions.
    InvalidWeights {
        /// Correspondence.
        span: EntityId,
        /// Offending region index, or None for an empty population.
        region: Option<usize>,
        /// Structural issue.
        issue: WeightIssue,
    },
    /// Between-notches ease does not name two distinct, noncoincident notch stops on its side.
    InvalidEaseAnchors {
        /// Correspondence.
        span: EntityId,
        /// Side carrying the declared ease interval.
        side: SeamSideId,
        /// First landmark.
        start: EntityId,
        /// Second landmark.
        end: EntityId,
    },
}
impl fmt::Display for SewingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCutPlan(error) => write!(f, "sewing graph cut plan: {error}"),
            Self::IdentityCollision { id } => {
                write!(f, "sewing graph repeats entity identity {id}")
            }
            Self::MissingCopy { span, side, copy } => {
                write!(f, "span {span} side {side:?} names missing copy {copy}")
            }
            Self::UnresolvedRange { span, side, .. } => write!(
                f,
                "span {span} side {side:?} needs interval/endpoint repair or an explicit choice"
            ),
            Self::RangeOutsidePiece { span, side, piece } => write!(
                f,
                "span {span} side {side:?} includes material outside piece {piece}"
            ),
            Self::OverlappingSelfSeam { span, copy } => {
                write!(f, "span {span} overlaps material on physical copy {copy}")
            }
            Self::MissingLandmark { span, stop } => write!(
                f,
                "span {span} side {:?} names missing stop {}",
                stop.side, stop.landmark
            ),
            Self::DuplicateStop { span, stop } => write!(
                f,
                "span {span} side {:?} repeats stop {}",
                stop.side, stop.landmark
            ),
            Self::StopOutsideSide { span, stop } => write!(
                f,
                "span {span} stop {} lies outside side {:?}",
                stop.landmark, stop.side
            ),
            Self::UnresolvedStop { span, stop, .. } => write!(
                f,
                "span {span} stop {} needs repair or an explicit choice",
                stop.landmark
            ),
            Self::InvalidWeights {
                span,
                region,
                issue,
            } => write!(f, "span {span} weighted region {region:?}: {issue:?}"),
            Self::InvalidEaseAnchors {
                span,
                side,
                start,
                end,
            } => write!(
                f,
                "span {span} side {side:?} requires distinct notch stops {start} and {end}"
            ),
        }
    }
}
impl std::error::Error for SewingError {}

struct ValidatedSide {
    side: SeamSideId,
    piece: EntityId,
    range: RangeResolution,
}

impl SewingGraph {
    /// Validate copy addressing, owned complete ranges, disjoint self-seams, stops and ease domains.
    /// # Errors
    /// Returns [`SewingError`] for invalid structural references/content. Geometry, realized ease,
    /// global design identity, command revision and release checks are still separate obligations.
    pub fn new(
        definition: SewingGraphDefinition,
        plan: &CutPlan,
        pieces: &[Piece],
        landmarks: &[SewingLandmark],
        ledger: &IdentityLedger,
    ) -> Result<Self, SewingError> {
        CutPlan::new(
            plan.copies()
                .iter()
                .map(|copy| *copy.definition())
                .collect(),
            pieces,
        )
        .map_err(SewingError::InvalidCutPlan)?;
        let patterns = pieces
            .iter()
            .map(|piece| (piece.id(), piece))
            .collect::<BTreeMap<_, _>>();
        let mut identities = patterns
            .keys()
            .copied()
            .chain(plan.copies().iter().map(|copy| copy.id()))
            .collect::<BTreeSet<_>>();
        if !identities.insert(definition.id) {
            return Err(SewingError::IdentityCollision { id: definition.id });
        }
        let mut marks = BTreeMap::new();
        for landmark in landmarks {
            if !identities.insert(landmark.id()) {
                return Err(SewingError::IdentityCollision { id: landmark.id() });
            }
            marks.insert(landmark.id(), landmark);
        }
        for span in &definition.spans {
            if !identities.insert(span.id) {
                return Err(SewingError::IdentityCollision { id: span.id });
            }
            let mut resolutions = Vec::with_capacity(2);
            for side_id in [SeamSideId::A, SeamSideId::B] {
                let side = span.side(side_id);
                let copy = plan.copy(side.copy).ok_or(SewingError::MissingCopy {
                    span: span.id,
                    side: side_id,
                    copy: side.copy,
                })?;
                let resolved = ledger.resolve_range(side.range);
                if !resolved.has_full_coverage()
                    || resolved.start().resolved().is_none()
                    || resolved.end().resolved().is_none()
                {
                    return Err(SewingError::UnresolvedRange {
                        span: span.id,
                        side: side_id,
                        evidence: Box::new(resolved),
                    });
                }
                let piece = patterns.get(&copy.piece()).ok_or_else(|| {
                    SewingError::InvalidCutPlan(CutPlanError::MissingPiece {
                        copy: copy.id(),
                        piece: copy.piece(),
                    })
                })?;
                if !range_is_owned(&resolved, piece, ledger) {
                    return Err(SewingError::RangeOutsidePiece {
                        span: span.id,
                        side: side_id,
                        piece: piece.id(),
                    });
                }
                resolutions.push(ValidatedSide {
                    side: side_id,
                    piece: copy.piece(),
                    range: resolved,
                });
            }
            if span.a.copy == span.b.copy {
                // Exactly two sides were validated; zip avoids a panic-prone indexed invariant.
                if resolutions
                    .iter()
                    .take(1)
                    .zip(resolutions.iter().skip(1))
                    .any(|(a, b)| ranges_overlap(&a.range, &b.range))
                {
                    return Err(SewingError::OverlappingSelfSeam {
                        span: span.id,
                        copy: span.a.copy,
                    });
                }
            }
            let mut stops = BTreeSet::new();
            for stop in &span.stops {
                if !stops.insert(*stop) {
                    return Err(SewingError::DuplicateStop {
                        span: span.id,
                        stop: *stop,
                    });
                }
                let mark = marks
                    .get(&stop.landmark)
                    .ok_or(SewingError::MissingLandmark {
                        span: span.id,
                        stop: *stop,
                    })?;
                let resolution = mark.resolve(ledger);
                let point = resolution
                    .resolved()
                    .ok_or_else(|| SewingError::UnresolvedStop {
                        span: span.id,
                        stop: *stop,
                        evidence: Box::new(resolution.clone()),
                    })?;
                let belongs = resolutions.iter().any(|side| {
                    side.side == stop.side
                        && side.piece == mark.piece()
                        && range_contains_point(&side.range, point)
                });
                if !belongs {
                    return Err(SewingError::StopOutsideSide {
                        span: span.id,
                        stop: *stop,
                    });
                }
            }
            validate_distribution(span, &marks, ledger)?;
        }
        Ok(Self {
            id: definition.id,
            spans: definition
                .spans
                .into_iter()
                .map(|definition| SeamSpan { definition })
                .collect(),
        })
    }
    /// Stable first-class graph identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.id
    }
    /// Immutable correspondences in authored order.
    #[must_use]
    pub fn spans(&self) -> &[SeamSpan] {
        &self.spans
    }
    /// Missing physical targets after an explicit replacement; no transfer to another copy.
    pub fn missing_copies<'a>(&'a self, plan: &'a CutPlan) -> impl Iterator<Item = EntityId> + 'a {
        self.spans
            .iter()
            .flat_map(|span| [span.definition.a.copy, span.definition.b.copy])
            .collect::<BTreeSet<_>>()
            .into_iter()
            .filter(|id| plan.copy(*id).is_none())
    }
    /// Missing semantic landmarks after an explicit registry replacement.
    pub fn missing_landmarks<'a>(
        &'a self,
        landmarks: &'a [SewingLandmark],
    ) -> impl Iterator<Item = EntityId> + 'a {
        self.spans
            .iter()
            .flat_map(|span| span.definition.stops.iter().map(|stop| stop.landmark))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .filter(|id| !landmarks.iter().any(|mark| mark.id() == *id))
    }
    /// Geometric seam walking, actual turning and source transforms are unproved at G1.
    #[must_use]
    pub const fn geometric_validation(&self) -> GeometricValidation {
        GeometricValidation::DeferredToG2
    }
    /// Even explicit ease needs geometric comparison; symbolic parameters need resolution too.
    #[must_use]
    pub const fn ease_validation(&self) -> EaseValidation {
        EaseValidation::DeferredToG2AndG3
    }
}

fn range_contains_point(range: &RangeResolution, point: ResolvedRef) -> bool {
    range.portions().iter().any(|part| matches!(part, RangePortion::Resolved(part) if part.range().edge()==point.edge() && part.range().from()<=point.param() && point.param()<=part.range().to()))
}
fn ranges_overlap(a: &RangeResolution, b: &RangeResolution) -> bool {
    a.portions().iter().any(|a| {
        b.portions().iter().any(|b| match (a, b) {
            (RangePortion::Resolved(a), RangePortion::Resolved(b)) => {
                a.range().edge() == b.range().edge()
                    && a.range().from().max(b.range().from()) < a.range().to().min(b.range().to())
            }
            _ => false,
        })
    })
}
fn range_is_owned(range: &RangeResolution, piece: &Piece, ledger: &IdentityLedger) -> bool {
    let mut owned = Vec::new();
    for (_, resolution) in piece.range_resolutions(ledger) {
        for portion in resolution.portions() {
            if let RangePortion::Resolved(part) = portion {
                owned.push(part.range());
            }
        }
    }
    range.portions().iter().all(|part| {
        let RangePortion::Resolved(part) = part else {
            return false;
        };
        let requested = part.range();
        let mut intervals = owned
            .iter()
            .filter(|range| range.edge() == requested.edge())
            .copied()
            .collect::<Vec<_>>();
        intervals.sort_by_key(|range| range.from());
        let mut covered = requested.from();
        for interval in intervals {
            if interval.from() > covered {
                break;
            }
            covered = covered.max(interval.to());
            if covered >= requested.to() {
                return true;
            }
        }
        false
    })
}
fn validate_distribution(
    span: &SeamSpanDefinition,
    marks: &BTreeMap<EntityId, &SewingLandmark>,
    ledger: &IdentityLedger,
) -> Result<(), SewingError> {
    match &span.ease.distribution {
        EaseDistribution::Uniform => Ok(()),
        EaseDistribution::Weighted { regions, .. } => {
            if regions.is_empty() {
                return Err(SewingError::InvalidWeights {
                    span: span.id,
                    region: None,
                    issue: WeightIssue::Empty,
                });
            }
            let mut previous = None;
            for (index, region) in regions.iter().enumerate() {
                let issue = if region.from >= region.to {
                    Some(WeightIssue::InvalidBounds)
                } else if region.weight <= Rational::ZERO {
                    Some(WeightIssue::NonPositive)
                } else if previous.is_some_and(|end| region.from < end) {
                    Some(WeightIssue::UnorderedOrOverlapping)
                } else {
                    None
                };
                if let Some(issue) = issue {
                    return Err(SewingError::InvalidWeights {
                        span: span.id,
                        region: Some(index),
                        issue,
                    });
                }
                previous = Some(region.to);
            }
            Ok(())
        }
        EaseDistribution::BetweenNotches { side, start, end } => {
            let points = [start, end]
                .iter()
                .filter_map(|id| {
                    if !span.stops.contains(&StopLandmark {
                        side: *side,
                        landmark: **id,
                    }) {
                        return None;
                    }
                    let mark = marks.get(id)?;
                    if !matches!(mark, SewingLandmark::Notch(_)) {
                        return None;
                    }
                    mark.resolve(ledger).resolved()
                })
                .collect::<Vec<_>>();
            let distinct = points
                .iter()
                .take(1)
                .zip(points.iter().skip(1))
                .any(|(a, b)| a.edge() != b.edge() || a.param() != b.param());
            if start == end || points.len() != 2 || !distinct {
                return Err(SewingError::InvalidEaseAnchors {
                    span: span.id,
                    side: *side,
                    start: *start,
                    end: *end,
                });
            }
            Ok(())
        }
    }
}
