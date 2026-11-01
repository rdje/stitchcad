//! Composed shared-Piece declarations retain both physical copies;no fitting or publication.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/truing_outside.rs"]
mod common;
use common::{
    domain::{
        q,
        roles::copies::{id, with_copies},
    },
    with_partitioned_outside,
};
use sc_core::{construction::*, name::MachineToken, ontology::*};

#[test]
fn shared_piece_curve_proposals_retain_coupled_copies_stops_ease_and_whole_contours() {
    with_copies(true, |copies| {
        let piece_before = copies.selected_piece().clone();
        let span_before = copies.source().span().definition().clone();
        let catalogue = EdgeOutputRoleCatalog::new(
            id(1000),
            vec![EdgeOutputRoleDefinition::new(
                LocalTag::new(9),
                MachineToken::new("refit").expect("role key"),
            )],
            vec![LocalTag::new(9)],
        );
        let limits = EdgeOutputRoleCatalogLimits::new(1, 1, 5);
        let catalog = catalogue.admit(limits).expect("original catalogue");
        let roles = copies.bind_edge_roles(&catalog, limits).expect("roles");
        with_partitioned_outside(
            roles,
            TruingOutcomeIntent::Refit,
            TruingOutsideRoles::new(TruingOutsideIntent::Absent, TruingOutsideIntent::Absent),
            Some(q(1, 2)),
            |outside| {
                let direction_raw = TruingDirectionManifest::new(
                    id(1000),
                    vec![
                        TruingDirectionAssignment::new(
                            LocalTag::new(11),
                            TruingParameterDirection::SameAsSource,
                        ),
                        TruingDirectionAssignment::new(
                            LocalTag::new(9),
                            TruingParameterDirection::SameAsSource,
                        ),
                    ],
                );
                let directions = direction_raw
                    .admit(&outside, TruingDirectionLimits::new(2, 2, 4))
                    .expect("complete output directions");
                let source = directions
                    .admit_exact_piece_source(TruingPieceSourceLimits::new(2, 0, 2))
                    .expect("actual shared Piece");
                let contours = source
                    .declare_contours(TruingContourLimits::new(2, 0, 2, 3, 5))
                    .expect("both replacement fragments and untouched counterpart edge");
                let fold_request = TruingFoldRequirement::new(TruingFoldIntent::NoFold, id(7001));
                let fold = contours
                    .require_fold(&fold_request, TruingFoldLimits::new(2, 3))
                    .expect("explicit original no-fold state");
                let identity = TruingSourceIdentityIntent::RetireOriginal;
                let lineage = fold
                    .require_lineage(&identity, TruingLineageLimits::new(2, 3))
                    .expect("explicit whole-edge lineage requirement");
                let raw = TruingCurveManifest::new(
                    id(1000),
                    TruingCurveSelectionPolicy::FirstFeasibleCompleteAssignment,
                    directions
                        .assignments()
                        .map(|output| {
                            TruingCurveAlternatives::new(
                                *output.assignment().tag(),
                                vec![DirectedRange {
                                    range: *output.source().range(),
                                    direction: Direction::Original,
                                }],
                            )
                        })
                        .collect(),
                );
                let curves = raw
                    .admit(&lineage, TruingCurveLimits::new(2, 2, 2, 1, 4))
                    .expect("complete declared curve domain,not certified feasibility");
                let locks = curves.lineage().evidence().locks();
                assert!(locks.copies().same_pattern_piece());
                assert!(std::ptr::eq(
                    locks.copies().selected_piece(),
                    copies.selected_piece()
                ));
                assert!(std::ptr::eq(
                    locks.copies().selected_piece(),
                    locks.copies().counterpart_piece()
                ));
                assert!(std::ptr::eq(locks.source(), copies.source()));
                assert_eq!(
                    locks.copies().copy(SeamSideId::B).orientation(),
                    CopyOrientation::Reflected
                );
                assert_eq!(
                    locks
                        .affected_copies()
                        .map(|copy| copy.id())
                        .collect::<Vec<_>>(),
                    vec![id(201), id(200)]
                );
                assert_eq!(locks.endpoints().len(), 2);
                let stop = locks.stops().next().expect("original stop");
                assert!(std::ptr::eq(
                    stop.stop(),
                    copies
                        .source()
                        .span()
                        .definition()
                        .stops
                        .first()
                        .expect("actual stop")
                ));
                assert_eq!(
                    *stop.stop(),
                    *span_before.stops.first().expect("snapshot stop")
                );
                assert_eq!(curves.combination_count(), 1);
                assert_eq!(curves.groups().len(), 2);
                assert_eq!(contours.occurrence_count(), 3);
                let placed: Vec<_> = contours
                    .occurrences()
                    .map(|edge| *edge.directed())
                    .collect();
                let counterpart = copies.source().span().definition().b.range.edge();
                assert_eq!(
                    placed,
                    vec![
                        DirectedEdge {
                            edge: EdgeRef::new(id(1000), LocalTag::new(9)),
                            direction: Direction::Original
                        },
                        DirectedEdge {
                            edge: EdgeRef::new(id(1000), LocalTag::new(11)),
                            direction: Direction::Original
                        },
                        DirectedEdge {
                            edge: counterpart,
                            direction: Direction::Original
                        },
                    ]
                );
                assert_eq!(curves.lineage().retained_contours().count(), 1);
            },
        );
        assert_eq!(copies.selected_piece(), &piece_before);
        assert_eq!(copies.source().span().definition(), &span_before);
    });
}
