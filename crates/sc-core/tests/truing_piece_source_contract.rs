//! Exact held Piece fields and complete quotas;no source-resolution or geometry certificate.
#![allow(clippy::expect_used)]
#[path = "support/truing_piece.rs"]
mod fixture;
use common::{
    domain::{q, roles::copies::id, with_domain_inventory},
    with_bound_outside, with_outside,
};
use fixture::{common, rich};
use sc_core::{construction::*, ontology::*};

fn outside(intent: TruingOutcomeIntent) -> TruingOutsideRoles {
    if intent == TruingOutcomeIntent::Refit {
        TruingOutsideRoles::new(
            TruingOutsideIntent::Role(LocalTag::new(2)),
            TruingOutsideIntent::Role(LocalTag::new(5)),
        )
    } else {
        TruingOutsideRoles::new(TruingOutsideIntent::Absent, TruingOutsideIntent::Absent)
    }
}
fn directions(outside: &AdmittedTruingOutsideRoles<'_>) -> TruingDirectionManifest {
    TruingDirectionManifest::new(
        id(1000),
        outside
            .roles()
            .map(|r| *r.role().tag())
            .chain(
                outside
                    .evidence()
                    .locks()
                    .outcome()
                    .source_domain()
                    .edges()
                    .map(|e| e.candidate().candidate().tag()),
            )
            .map(|tag| TruingDirectionAssignment::new(tag, TruingParameterDirection::ReverseSource))
            .collect(),
    )
}
#[test]
fn every_original_field_and_repeated_use_remains_borrowed_in_forward_and_reverse_order() {
    use TruingPieceOccurrenceLocation as L;
    for side in [SeamSideId::A, SeamSideId::B] {
        for intent in [
            TruingOutcomeIntent::PreserveSource,
            TruingOutcomeIntent::Refit,
        ] {
            with_domain_inventory(
                side,
                q(1, 4),
                q(3, 4),
                |f| rich(f, side, true, false),
                |roles| {
                    with_bound_outside(roles, intent, outside(intent), |outside| {
                        let raw = directions(&outside);
                        let t = raw.assignments().len();
                        let directions = raw
                            .admit(&outside, TruingDirectionLimits::new(t, t, 2 * t))
                            .expect("explicit directions");
                        let limits = TruingPieceSourceLimits::new(t, 2, 8);
                        let bound = directions
                            .admit_exact_piece_source(limits)
                            .expect("whole exact Piece");
                        let piece = outside.evidence().locks().copies().selected_piece();
                        let snapshot = piece.definition().clone();
                        assert!(std::ptr::eq(bound.directions(), &directions));
                        assert!(std::ptr::eq(bound.piece(), piece));
                        assert_eq!(bound.limits(), limits);
                        assert_eq!(
                            (limits.outputs(), limits.holes(), limits.occurrences()),
                            (t, 2, 8)
                        );
                        assert_eq!(
                            (
                                bound.occurrence_count(),
                                bound.contour_matches(),
                                bound.fold_matches()
                            ),
                            (8, 3, 1)
                        );
                        assert_eq!(
                            bound.target(),
                            outside
                                .evidence()
                                .locks()
                                .source()
                                .source_side()
                                .range
                                .edge()
                        );
                        let expected = [
                            L::Boundary(0),
                            L::Boundary(1),
                            L::Hole { hole: 0, index: 0 },
                            L::Hole { hole: 1, index: 0 },
                            L::Construction(0),
                            L::Construction(1),
                            L::Construction(2),
                            L::Fold(0),
                        ];
                        for (i, field) in bound.occurrences().enumerate() {
                            assert!(std::ptr::eq(field.context(), &bound));
                            assert_eq!(
                                field.location(),
                                *expected.get(i).expect("original location")
                            );
                            assert_eq!(field.is_target(), [1, 4, 6, 7].contains(&i));
                            let d = piece.definition();
                            let same_original = match (field.location(), field.field()) {
                                (
                                    L::Boundary(i),
                                    TruingPieceOccurrenceField::Directed(original),
                                ) => std::ptr::eq(
                                    original,
                                    d.boundary.get(i).expect("original boundary"),
                                ),
                                (
                                    L::Hole { hole, index },
                                    TruingPieceOccurrenceField::Directed(original),
                                ) => std::ptr::eq(
                                    original,
                                    d.holes
                                        .get(hole)
                                        .and_then(|h| h.get(index))
                                        .expect("original hole"),
                                ),
                                (
                                    L::Construction(i),
                                    TruingPieceOccurrenceField::Directed(original),
                                ) => std::ptr::eq(
                                    original,
                                    d.construction_lines.get(i).expect("original construction"),
                                ),
                                (L::Fold(i), TruingPieceOccurrenceField::Fold(original)) => {
                                    std::ptr::eq(
                                        original,
                                        d.fold_edges.get(i).expect("original fold"),
                                    )
                                }
                                _ => false,
                            };
                            assert!(same_original, "incorrect original location/field kind");
                        }
                        assert_eq!(
                            bound
                                .occurrences()
                                .rev()
                                .map(|f| f.location())
                                .collect::<Vec<_>>(),
                            expected.into_iter().rev().collect::<Vec<_>>()
                        );
                        assert_eq!(bound.occurrences().filter(|f| f.is_target()).count(), 4);
                        assert!(format!("{bound:?}").contains("BoundExactTruingPieceSource"));
                        assert_eq!(piece.definition(), &snapshot);
                    });
                },
            );
        }
    }
}
#[test]
fn complete_unrelated_populations_precede_literal_membership_and_errors_retain_whole_parents() {
    use TruingPieceSourceRefusal as R;
    for exact in [false, true] {
        with_domain_inventory(
            SeamSideId::B,
            q(1, 4),
            q(3, 4),
            |f| rich(f, SeamSideId::B, exact, false),
            |roles| {
                with_bound_outside(
                    roles,
                    TruingOutcomeIntent::Refit,
                    outside(TruingOutcomeIntent::Refit),
                    |outside| {
                        let raw = directions(&outside);
                        let directions = raw
                            .admit(&outside, TruingDirectionLimits::new(3, 3, 6))
                            .expect("explicit directions");
                        for (limits, cause) in [
                            (
                                TruingPieceSourceLimits::new(2, 1, 7),
                                R::Outputs {
                                    required: 3,
                                    allowed: 2,
                                },
                            ),
                            (
                                TruingPieceSourceLimits::new(3, 1, 7),
                                R::Holes {
                                    required: 2,
                                    allowed: 1,
                                },
                            ),
                            (
                                TruingPieceSourceLimits::new(3, 2, 7),
                                R::Occurrences {
                                    required: 8,
                                    allowed: 7,
                                },
                            ),
                        ] {
                            let error = directions
                                .admit_exact_piece_source(limits)
                                .expect_err("whole quota before exact membership");
                            assert!(std::ptr::eq(error.directions(), &directions));
                            assert!(std::ptr::eq(
                                error.piece(),
                                outside.evidence().locks().copies().selected_piece()
                            ));
                            assert_eq!(error.limits(), limits);
                            assert_eq!(error.refusal(), cause);
                            assert!(error.to_string().contains("truing exact Piece source"));
                        }
                        let result = directions
                            .admit_exact_piece_source(TruingPieceSourceLimits::new(3, 2, 8));
                        if exact {
                            assert!(result.is_ok());
                        } else {
                            assert_eq!(
                                result
                                    .expect_err("different held edge;no global absence claim")
                                    .refusal(),
                                R::NoExactOccurrence
                            );
                        }
                    },
                );
            },
        );
    }
}
#[test]
fn empty_new_output_preservation_still_requires_and_lends_the_original_piece() {
    with_outside(
        SeamSideId::A,
        Param::START,
        Param::END,
        TruingOutcomeIntent::PreserveSource,
        outside(TruingOutcomeIntent::PreserveSource),
        |outside| {
            let raw = directions(&outside);
            let directions = raw
                .admit(&outside, TruingDirectionLimits::new(0, 0, 0))
                .expect("empty explicit preserve outputs");
            let bound = directions
                .admit_exact_piece_source(TruingPieceSourceLimits::new(0, 0, 1))
                .expect("actual original source");
            assert_eq!(
                (
                    bound.occurrence_count(),
                    bound.contour_matches(),
                    bound.fold_matches()
                ),
                (1, 1, 0)
            );
            assert_eq!(bound.occurrences().count(), 1);
        },
    );
}
