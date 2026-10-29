//! Construct real bounded context under admitted C/P/M/E*B and pair-range populations.
use super::{
    CommandStopContextBoundary as Boundary, CommandStopContextError as Error,
    CommandStopContextIssue as Issue, CommandStopContextPopulation as Population,
    CommandStopContextRange as Location, CommandStopCurrentContext, CommandStopLandmarkEvidence,
    CommandStopOwnershipAdmission, CommandStopPiecePortion, PieceView,
};
use crate::{
    design::DesignReferenceHost,
    ontology::{
        CutCopy, CutPlanError, EdgeRange, EntityId, IdentityLedger, RangeCoverage, RangePortion,
        SeamSideId, SewingError, SewingLandmark, UniquePointResolution,
    },
};
use std::collections::BTreeMap;

struct Build<'a, H> {
    admission: &'a CommandStopOwnershipAdmission<'a, H>,
    owned_items: usize,
}
impl<'a, H: DesignReferenceHost> Build<'a, H> {
    fn target(&self) -> EntityId {
        self.admission.inputs().binding().span().id()
    }
    fn fail(&self, issue: Issue) -> Error {
        Error::new(self.target(), issue)
    }
    fn structural(&self, issue: SewingError) -> Error {
        self.fail(Issue::Structural(issue))
    }
    fn ledger(&self) -> &'a IdentityLedger {
        self.admission
            .inputs()
            .state()
            .inputs()
            .structure()
            .topology()
    }
    fn grow(
        &self,
        population: Population,
        owner: EntityId,
        retained: usize,
        limit: usize,
    ) -> Result<usize, Error> {
        retained
            .checked_add(1)
            .filter(|next| *next <= limit)
            .ok_or_else(|| {
                self.fail(Issue::DerivedItems {
                    population,
                    owner,
                    retained,
                    requested: 1,
                    limit,
                })
            })
    }
    fn coverage(&self, held: EdgeRange, reference: Location) -> Result<RangeCoverage, Error> {
        self.ledger()
            .resolve_range_coverage(held, self.admission.inputs().limits().range_limits())
            .map_err(|issue| self.fail(Issue::CoverageLimit { reference, issue }))
    }
    fn copies(&self) -> Result<BTreeMap<EntityId, &'a CutCopy>, Error> {
        let mut result = BTreeMap::new();
        for copy in self
            .admission
            .inputs()
            .state()
            .inputs()
            .structure()
            .cut_plan()
            .copies()
        {
            self.grow(
                Population::Copies,
                copy.id(),
                result.len(),
                self.admission.copy_count(),
            )?;
            if result.insert(copy.id(), copy).is_some() {
                return Err(self.structural(SewingError::IdentityCollision { id: copy.id() }));
            }
        }
        Ok(result)
    }
    fn pieces(&mut self) -> Result<BTreeMap<EntityId, PieceView<'a>>, Error> {
        let mut result = BTreeMap::new();
        for piece in self
            .admission
            .inputs()
            .state()
            .inputs()
            .structure()
            .pieces()
            .pieces()
        {
            let mut portions = Vec::new();
            for (source_edge_index, edge) in piece.edges().enumerate() {
                let coverage = self.coverage(
                    EdgeRange::whole(edge.edge),
                    Location::PieceEdge {
                        piece: piece.id(),
                        index: source_edge_index,
                    },
                )?;
                let (held, parts) = coverage.into_parts();
                for (portion_index, portion) in parts.into_iter().enumerate() {
                    let next = self.grow(
                        Population::OwnedPortions,
                        piece.id(),
                        self.owned_items,
                        self.admission.ownership_item_bound(),
                    )?;
                    portions.push(CommandStopPiecePortion {
                        held,
                        source_edge_index,
                        portion_index,
                        portion,
                    });
                    self.owned_items = next;
                }
            }
            // No heap scratch or source-order inference:original indices give deterministic ties.
            portions.sort_unstable_by_key(|part| {
                let (kind, range) = match &part.portion {
                    RangePortion::Resolved(current) => (1u8, current.range()),
                    RangePortion::Unresolved(_) => (0u8, part.held),
                };
                (
                    kind,
                    range.edge(),
                    range.from(),
                    range.to(),
                    part.source_edge_index,
                    part.portion_index,
                )
            });
            self.grow(
                Population::Pieces,
                piece.id(),
                result.len(),
                self.admission.piece_count(),
            )?;
            if result
                .insert(piece.id(), PieceView { piece, portions })
                .is_some()
            {
                return Err(self.structural(SewingError::IdentityCollision { id: piece.id() }));
            }
        }
        Ok(result)
    }
    fn marks(
        &self,
        pieces: &BTreeMap<EntityId, PieceView<'a>>,
    ) -> Result<BTreeMap<EntityId, CommandStopLandmarkEvidence<'a>>, Error> {
        let mut result = BTreeMap::new();
        for landmark in self.admission.inputs().state().landmarks() {
            let piece = pieces.get(&landmark.piece()).ok_or_else(|| {
                self.fail(Issue::MissingLandmarkPiece {
                    landmark: landmark.id(),
                    piece: landmark.piece(),
                })
            })?;
            let anchor = match landmark {
                SewingLandmark::Notch(notch) => notch.definition().anchor,
                SewingLandmark::Turn(turn) => turn.anchor(),
            };
            let resolution = self.ledger().resolve_unique_point(
                anchor.edge,
                anchor.param,
                self.admission.inputs().limits().current(),
            );
            let current = resolution.resolved().ok_or_else(|| {
                self.fail(Issue::LandmarkPoint {
                    landmark: landmark.id(),
                    resolution,
                })
            })?;
            if !piece.contains(current) {
                return Err(self.fail(Issue::LandmarkOutsidePiece {
                    landmark: landmark.id(),
                    piece: piece.piece.id(),
                    resolution,
                }));
            }
            self.grow(
                Population::Landmarks,
                landmark.id(),
                result.len(),
                self.admission.inputs().landmark_count(),
            )?;
            if result
                .insert(
                    landmark.id(),
                    CommandStopLandmarkEvidence {
                        landmark,
                        resolution,
                        current,
                    },
                )
                .is_some()
            {
                return Err(self.structural(SewingError::IdentityCollision { id: landmark.id() }));
            }
        }
        Ok(result)
    }
    fn boundaries(&self, side: SeamSideId) -> Result<[UniquePointResolution; 3], Error> {
        let binding = self.admission.inputs().binding();
        let definition = binding.span().definition();
        let range = match side {
            SeamSideId::A => definition.a.range,
            SeamSideId::B => definition.b.range,
        };
        let point = |boundary, param| {
            let resolution = self.ledger().resolve_unique_point(
                range.edge(),
                param,
                self.admission.inputs().limits().current(),
            );
            if resolution.resolved().is_none() {
                return Err(self.fail(Issue::Boundary {
                    side,
                    boundary,
                    resolution,
                }));
            }
            Ok(resolution)
        };
        Ok([
            point(Boundary::Start, range.from())?,
            point(Boundary::Cut, binding.request().cut(side).param)?,
            point(Boundary::End, range.to())?,
        ])
    }
    fn owned_coverage(
        &self,
        held: EdgeRange,
        reference: Location,
        piece: &PieceView<'a>,
    ) -> Result<RangeCoverage, Error> {
        let evidence = self.coverage(held, reference)?;
        if evidence.portions().is_empty() || !evidence.has_full_coverage() {
            return Err(self.fail(Issue::Coverage {
                reference,
                evidence,
            }));
        }
        if !evidence.portions().iter().all(|part| match part {
            RangePortion::Resolved(part) => piece.covers(part.range()),
            RangePortion::Unresolved(_) => false,
        }) {
            return Err(self.fail(Issue::OutsidePiece {
                reference,
                piece: piece.piece.id(),
                evidence,
            }));
        }
        Ok(evidence)
    }
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
pub(super) fn validate<'a, H: DesignReferenceHost>(
    admission: &'a CommandStopOwnershipAdmission<'a, H>,
) -> Result<CommandStopCurrentContext<'a, H>, Error> {
    let mut build = Build {
        admission,
        owned_items: 0,
    };
    let binding = admission.inputs().binding();
    let definition = binding.span().definition();
    let copies = build.copies()?;
    let get_copy = |side, copy| {
        copies.get(&copy).copied().ok_or_else(|| {
            build.structural(SewingError::MissingCopy {
                span: binding.span().id(),
                side,
                copy,
            })
        })
    };
    let a_copy = get_copy(SeamSideId::A, definition.a.copy)?;
    let b_copy = get_copy(SeamSideId::B, definition.b.copy)?;
    let pieces = build.pieces()?;
    let get_piece = |copy: &CutCopy| {
        pieces.get(&copy.piece()).ok_or_else(|| {
            build.structural(SewingError::InvalidCutPlan(CutPlanError::MissingPiece {
                copy: copy.id(),
                piece: copy.piece(),
            }))
        })
    };
    let a_piece = get_piece(a_copy)?;
    let b_piece = get_piece(b_copy)?;
    let marks = build.marks(&pieces)?;
    let [a_start, a_cut, a_end] = build.boundaries(SeamSideId::A)?;
    let [b_start, b_cut, b_end] = build.boundaries(SeamSideId::B)?;
    {
        let a = build.owned_coverage(
            definition.a.range,
            Location::TargetSide(SeamSideId::A),
            a_piece,
        )?;
        let b = build.owned_coverage(
            definition.b.range,
            Location::TargetSide(SeamSideId::B),
            b_piece,
        )?;
        if a_copy.id() == b_copy.id() && overlaps(&a, &b) {
            return Err(build.structural(SewingError::OverlappingSelfSeam {
                span: binding.span().id(),
                copy: a_copy.id(),
            }));
        }
    } // Release full-side buffers before four pair-side caches:at most4B retained/query items.
    let [first, second] = binding.paired_intervals();
    let first_a = build.owned_coverage(
        first.side(SeamSideId::A).range,
        Location::FirstPair(SeamSideId::A),
        a_piece,
    )?;
    let first_b = build.owned_coverage(
        first.side(SeamSideId::B).range,
        Location::FirstPair(SeamSideId::B),
        b_piece,
    )?;
    let second_a = build.owned_coverage(
        second.side(SeamSideId::A).range,
        Location::SecondPair(SeamSideId::A),
        a_piece,
    )?;
    let second_b = build.owned_coverage(
        second.side(SeamSideId::B).range,
        Location::SecondPair(SeamSideId::B),
        b_piece,
    )?;
    Ok(CommandStopCurrentContext {
        ownership: admission,
        copies,
        pieces,
        marks,
        points: [a_start, a_cut, a_end, b_start, b_cut, b_end],
        paired: [first_a, first_b, second_a, second_b],
        owned_items: build.owned_items,
    })
}
