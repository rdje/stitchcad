//! Actual bounded source checks. No legacy point/range resolution or synthetic graph is used.
use super::{
    failure, NotchSourceAdmission, NotchSourceRange as Location, NotchSourceValidation,
    NotchSourceValidationError as Error, NotchSourceValidationIssue as Issue,
};
use crate::ontology::{
    AnchorError, CutPlan, CutPlanError, EdgeRange, EntityId, Notch, Piece, RangeCoverage,
    RangeCoverageLimits, RangePortion, ResolvedRef, SeamSideId, SewingError, SewingLandmark,
    StopLandmark,
};
use std::collections::{BTreeMap, BTreeSet};

struct PieceView<'a> {
    piece: &'a Piece,
    owned: Vec<EdgeRange>,
}
#[derive(Clone, Copy)]
struct MarkView<'a> {
    mark: &'a SewingLandmark,
    current: ResolvedRef,
}
#[derive(Clone, Copy)]
struct Context<'a> {
    admission: NotchSourceAdmission<'a>,
    interval_bound: usize,
    interval_count: usize,
    index_bound: usize,
    index_count: usize,
}
impl<'a> Context<'a> {
    fn fail(&self, issue: Issue) -> Error {
        failure(self.admission.source().span(), issue)
    }
    fn structural(&self, issue: SewingError) -> Error {
        self.fail(Issue::Structural(Box::new(issue)))
    }
    fn index(&mut self, owner: EntityId) -> Result<(), Error> {
        let next = self
            .index_count
            .checked_add(1)
            .filter(|total| *total <= self.index_bound)
            .ok_or_else(|| {
                self.fail(Issue::DerivedIndexItems {
                    owner,
                    retained: self.index_count,
                    requested: 1,
                    limit: self.index_bound,
                })
            })?;
        self.index_count = next;
        Ok(())
    }
    fn coverage(&self, range: EdgeRange, reference: Location) -> Result<RangeCoverage, Error> {
        let limits = self.admission.limits();
        self.admission
            .ledger()
            .resolve_range_coverage(
                range,
                RangeCoverageLimits::new(
                    limits.original_point().current(),
                    limits.range_portions(),
                ),
            )
            .map_err(|issue| self.fail(Issue::CoverageLimit { reference, issue }))
    }
    fn pieces(&mut self) -> Result<BTreeMap<EntityId, PieceView<'a>>, Error> {
        let source = self.admission;
        let mut pieces = BTreeMap::new();
        for piece in source.pieces() {
            let mut owned = Vec::new();
            for (index, edge) in piece.edges().enumerate() {
                let reference = Location::PieceEdge {
                    piece: piece.id(),
                    index,
                };
                let coverage = self.coverage(EdgeRange::whole(edge.edge), reference)?;
                for part in coverage.portions() {
                    let RangePortion::Resolved(part) = part else {
                        continue;
                    };
                    let next = self
                        .interval_count
                        .checked_add(1)
                        .filter(|total| *total <= self.interval_bound)
                        .ok_or_else(|| {
                            self.fail(Issue::DerivedItems {
                                reference,
                                retained: self.interval_count,
                                requested: 1,
                                limit: self.interval_bound,
                            })
                        })?;
                    owned.push(part.range());
                    self.interval_count = next;
                }
            }
            owned.sort_unstable_by_key(|range| (range.edge(), range.from(), range.to()));
            self.index(piece.id())?;
            pieces.insert(piece.id(), PieceView { piece, owned });
        }
        Ok(pieces)
    }
    fn marks(
        &mut self,
        pieces: &BTreeMap<EntityId, PieceView<'a>>,
    ) -> Result<BTreeMap<EntityId, MarkView<'a>>, Error> {
        let source = self.admission;
        let mut marks = BTreeMap::new();
        for mark in source.landmarks() {
            let piece = pieces.get(&mark.piece()).ok_or_else(|| {
                self.fail(Issue::MissingLandmarkPiece {
                    landmark: mark.id(),
                    piece: mark.piece(),
                })
            })?;
            let anchor = match mark {
                SewingLandmark::Notch(notch) => notch.definition().anchor,
                SewingLandmark::Turn(turn) => turn.anchor(),
            };
            let result = source.ledger().resolve_unique_point(
                anchor.edge,
                anchor.param,
                source.limits().original_point().current(),
            );
            let current = result.resolved().ok_or_else(|| {
                self.fail(Issue::LandmarkPoint {
                    landmark: mark.id(),
                    resolution: Box::new(result),
                })
            })?;
            if !contains(&piece.owned, current) {
                return Err(self.fail(Issue::LandmarkOutsidePiece {
                    landmark: mark.id(),
                    issue: Box::new(AnchorError::AnchorOutsidePiece {
                        piece: piece.piece.id(),
                        anchor,
                    }),
                }));
            }
            self.index(mark.id())?;
            marks.insert(mark.id(), MarkView { mark, current });
        }
        Ok(marks)
    }
    fn side(
        &self,
        side: SeamSideId,
        pieces: &BTreeMap<EntityId, PieceView<'_>>,
    ) -> Result<Side, Error> {
        let source = self.admission.source();
        let held = source.side(side);
        let copy = self.admission.plan().copy(held.copy).ok_or_else(|| {
            self.structural(SewingError::MissingCopy {
                span: source.span(),
                side,
                copy: held.copy,
            })
        })?;
        for (upper, param) in [(false, held.range.from()), (true, held.range.to())] {
            let result = self.admission.ledger().resolve_unique_point(
                held.range.edge(),
                param,
                self.admission.limits().original_point().current(),
            );
            if result.resolved().is_none() {
                return Err(self.fail(Issue::Endpoint {
                    side,
                    upper,
                    resolution: Box::new(result),
                }));
            }
        }
        let coverage = self.coverage(held.range, Location::Side(side))?;
        if !coverage.has_full_coverage() {
            return Err(self.fail(Issue::Coverage {
                side,
                evidence: coverage,
            }));
        }
        let piece = pieces.get(&copy.piece()).ok_or_else(|| {
            self.structural(SewingError::InvalidCutPlan(CutPlanError::MissingPiece {
                copy: copy.id(),
                piece: copy.piece(),
            }))
        })?;
        if !coverage.portions().iter().all(|part| match part {
            RangePortion::Resolved(part) => covers(&piece.owned, part.range()),
            RangePortion::Unresolved(_) => false,
        }) {
            return Err(self.structural(SewingError::RangeOutsidePiece {
                span: source.span(),
                side,
                piece: copy.piece(),
            }));
        }
        Ok(Side {
            piece: copy.piece(),
            coverage,
        })
    }
}
struct Side {
    piece: EntityId,
    coverage: RangeCoverage,
}
fn contains(owned: &[EdgeRange], point: ResolvedRef) -> bool {
    owned.iter().any(|range| {
        range.edge() == point.edge() && range.from() <= point.param() && point.param() <= range.to()
    })
}
fn covers(owned: &[EdgeRange], requested: EdgeRange) -> bool {
    let mut covered = requested.from();
    for range in owned
        .iter()
        .filter(|range| range.edge() == requested.edge())
    {
        if range.from() > covered {
            break;
        }
        covered = covered.max(range.to());
        if covered >= requested.to() {
            return true;
        }
    }
    false
}
fn overlaps(a: &RangeCoverage, b: &RangeCoverage) -> bool {
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
fn identities(context: &mut Context<'_>) -> Result<BTreeSet<EntityId>, Error> {
    let source = context.admission;
    let mut seen = BTreeSet::new();
    let ids = source
        .pieces()
        .iter()
        .map(Piece::id)
        .chain(source.plan().copies().iter().map(|copy| copy.id()))
        .chain(core::iter::once(source.source().span()))
        .chain(source.landmarks().iter().map(SewingLandmark::id));
    for id in ids {
        if seen.contains(&id) {
            return Err(failure(
                source.source().span(),
                Issue::Structural(Box::new(SewingError::IdentityCollision { id })),
            ));
        }
        context.index(id)?;
        seen.insert(id);
    }
    Ok(seen)
}
fn stops(
    context: &mut Context<'_>,
    a: &Side,
    b: &Side,
    marks: &BTreeMap<EntityId, MarkView<'_>>,
) -> Result<BTreeSet<StopLandmark>, Error> {
    let source = context.admission.source();
    let mut seen = BTreeSet::new();
    for stop in source.stops() {
        if seen.contains(stop) {
            return Err(context.structural(SewingError::DuplicateStop {
                span: source.span(),
                stop: *stop,
            }));
        }
        context.index(stop.landmark)?;
        seen.insert(*stop);
        let mark = marks.get(&stop.landmark).ok_or_else(|| {
            context.structural(SewingError::MissingLandmark {
                span: source.span(),
                stop: *stop,
            })
        })?;
        let side = match stop.side {
            SeamSideId::A => a,
            SeamSideId::B => b,
        };
        if side.piece != mark.mark.piece()
            || !side.coverage.portions().iter().any(|part| match part {
                RangePortion::Resolved(part) => {
                    contains(core::slice::from_ref(&part.range()), mark.current)
                }
                RangePortion::Unresolved(_) => false,
            })
        {
            return Err(context.structural(SewingError::StopOutsideSide {
                span: source.span(),
                stop: *stop,
            }));
        }
    }
    Ok(seen)
}
fn distribution<'a>(
    context: &Context<'a>,
    marks: &BTreeMap<EntityId, MarkView<'a>>,
    stops: &BTreeSet<StopLandmark>,
) -> Result<(&'a Notch, &'a Notch), Error> {
    let admission = context.admission;
    let (start, end) = admission.notch_ids();
    let side = admission.distribution_side();
    let get = |id| {
        let MarkView { mark, current } = *marks.get(&id)?;
        if !stops.contains(&StopLandmark { side, landmark: id }) {
            return None;
        }
        match mark {
            SewingLandmark::Notch(notch) => Some((notch, current)),
            SewingLandmark::Turn(_) => None,
        }
    };
    let pair = get(start).zip(get(end)).filter(|((_, a), (_, b))| {
        start != end && (a.edge() != b.edge() || a.param() != b.param())
    });
    let Some(((first, _), (second, _))) = pair else {
        return Err(context.structural(SewingError::InvalidEaseAnchors {
            span: admission.source().span(),
            side,
            start,
            end,
        }));
    };
    Ok((first, second))
}
pub(super) fn validate(
    admission: NotchSourceAdmission<'_>,
) -> Result<NotchSourceValidation<'_>, Error> {
    let count = admission.dependency_item_count();
    let portions = admission.limits().range_portions().get();
    let overflow = || {
        failure(
            admission.source().span(),
            Issue::DerivedBoundOverflow {
                dependency_items: count,
                range_portions: portions,
            },
        )
    };
    let interval_bound = count.checked_mul(portions).ok_or_else(overflow)?;
    let index_bound = count
        .checked_mul(3)
        .and_then(|value| value.checked_add(1))
        .ok_or_else(overflow)?;
    // Public complete CutPlan validation preserves its original cause;fixed copy definitions only.
    CutPlan::new(
        admission
            .plan()
            .copies()
            .iter()
            .map(|copy| *copy.definition())
            .collect(),
        admission.pieces(),
    )
    .map_err(|issue| {
        failure(
            admission.source().span(),
            Issue::Structural(Box::new(SewingError::InvalidCutPlan(issue))),
        )
    })?;
    let mut context = Context {
        admission,
        interval_bound,
        interval_count: 0,
        index_bound,
        index_count: 0,
    };
    let _identities = identities(&mut context)?;
    let pieces = context.pieces()?;
    let marks = context.marks(&pieces)?;
    let a = context.side(SeamSideId::A, &pieces)?;
    let b = context.side(SeamSideId::B, &pieces)?;
    if admission.source().side(SeamSideId::A).copy == admission.source().side(SeamSideId::B).copy
        && overlaps(&a.coverage, &b.coverage)
    {
        return Err(context.structural(SewingError::OverlappingSelfSeam {
            span: admission.source().span(),
            copy: admission.source().side(SeamSideId::A).copy,
        }));
    }
    let stops = stops(&mut context, &a, &b, &marks)?;
    let (start_notch, end_notch) = distribution(&context, &marks, &stops)?;
    // All local index insertions and ownership-cache appends were checked before allocation.
    // The larger linear reservation also covers the public CutPlan's temporary fixed-copy arrays.
    Ok(NotchSourceValidation {
        admission,
        interval_item_bound: interval_bound,
        index_item_bound: index_bound,
        interval_item_count: context.interval_count,
        index_item_count: context.index_count,
        start_notch,
        end_notch,
    })
}
