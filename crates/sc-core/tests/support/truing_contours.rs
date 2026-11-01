//! Actual complete held source/direction/contour declarations for focused fold/lineage contracts.
#[path = "truing_piece.rs"]
pub mod piece;
use piece::{
    common::{
        domain::{q, roles::copies::id, with_domain_inventory},
        with_partitioned_outside,
    },
    rich,
};
use sc_core::{construction::*, ontology::*};
#[derive(Clone, Copy)]
pub enum FoldFixture {
    TargetBoundary,
    UnrelatedBoundary,
    TargetHole,
    NoFold,
}
pub fn with_rich_contours(
    side: SeamSideId,
    intent: TruingOutcomeIntent,
    fold: FoldFixture,
    run: impl FnOnce(TruingPieceContours<'_>),
) {
    with_domain_inventory(
        side,
        q(1, 4),
        q(3, 4),
        |f| {
            rich(f, side, true, matches!(fold, FoldFixture::TargetHole))
                .into_iter()
                .map(|p| {
                    if p.id()
                        != if side == SeamSideId::A {
                            id(100)
                        } else {
                            id(101)
                        }
                    {
                        return p;
                    }
                    let mut d = p.definition().clone();
                    match fold {
                        FoldFixture::UnrelatedBoundary => {
                            d.fold_edges =
                                vec![d.boundary.first().expect("actual other boundary").edge]
                        }
                        FoldFixture::NoFold => {
                            d.cut_on_fold = false;
                            d.fold_edges.clear();
                        }
                        _ => {}
                    }
                    Piece::new(d, &f.ledger).expect("actual original fold state")
                })
                .collect()
        },
        |roles| {
            let refit = intent == TruingOutcomeIntent::Refit;
            let outside = if refit {
                TruingOutsideRoles::new(
                    TruingOutsideIntent::Role(LocalTag::new(2)),
                    TruingOutsideIntent::Role(LocalTag::new(5)),
                )
            } else {
                TruingOutsideRoles::new(TruingOutsideIntent::Absent, TruingOutsideIntent::Absent)
            };
            with_partitioned_outside(
                roles,
                intent,
                outside,
                refit.then_some(q(1, 2)),
                |outside| {
                    let raw = TruingDirectionManifest::new(
                        id(1000),
                        outside
                            .evidence()
                            .locks()
                            .outcome()
                            .source_domain()
                            .edges()
                            .map(|e| e.candidate().candidate().tag())
                            .chain(outside.roles().map(|e| *e.role().tag()))
                            .map(|tag| {
                                TruingDirectionAssignment::new(
                                    tag,
                                    TruingParameterDirection::SameAsSource,
                                )
                            })
                            .collect(),
                    );
                    let directions = raw
                        .admit(&outside, TruingDirectionLimits::new(4, 4, 8))
                        .expect("explicit complete directions");
                    let source = directions
                        .admit_exact_piece_source(TruingPieceSourceLimits::new(4, 2, 8))
                        .expect("exact original Piece");
                    let contours = source
                        .declare_contours(TruingContourLimits::new(4, 2, 8, 16, 20))
                        .expect("complete original-order contours");
                    run(contours);
                },
            );
        },
    );
}
