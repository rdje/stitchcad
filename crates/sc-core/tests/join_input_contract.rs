//! Named two-Piece source associations;no fixture claims a joined geometric boundary.
#![allow(clippy::expect_used)]
use sc_core::{construction::*, ontology::*};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
#[path = "support/seam_edit_sources.rs"]
mod seam_sources;
use seam_sources::{first_span, fixture};
fn request(a: EntityId, b: EntityId) -> JoinPiecesAtSeam {
    JoinPiecesAtSeam::new(id(1000), id(301), a, b).expect("explicit distinct Piece sources")
}
#[test]
fn original_side_copy_and_piece_metadata_borrow_actual_owners_despite_inventory_order() {
    let f = fixture(false);
    let edit = request(id(100), id(101));
    let mut pieces = f.pieces.clone();
    pieces.reverse();
    let before = pieces.clone();
    let original = f.graph.clone();
    let bound = edit
        .bind_source(&f.graph, &f.plan, &pieces)
        .expect("actual two-sided association");
    assert!(std::ptr::eq(bound.edit(), &edit));
    assert!(std::ptr::eq(bound.graph(), &f.graph));
    assert!(std::ptr::eq(bound.plan(), &f.plan));
    assert!(std::ptr::eq(bound.pieces(), pieces.as_slice()));
    assert!(std::ptr::eq(bound.span(), first_span(&f.graph)));
    for side in [SeamSideId::A, SeamSideId::B] {
        let view = bound.side(side);
        assert!(std::ptr::eq(view.bound(), &bound));
        assert_eq!(view.side(), side);
        let source = match side {
            SeamSideId::A => &first_span(&f.graph).definition().a,
            SeamSideId::B => &first_span(&f.graph).definition().b,
        };
        assert!(std::ptr::eq(view.source_side(), source));
        assert!(std::ptr::eq(
            view.copy(),
            f.plan.copy(source.copy).expect("original copy")
        ));
        assert!(std::ptr::eq(
            view.piece(),
            pieces
                .iter()
                .find(|p| p.id() == edit.piece_id(side))
                .expect("actual provided Piece")
        ));
        assert_eq!(view.copy().piece(), view.piece().id());
        assert!(
            matches!(&view.piece().definition().material,MaterialAssignment::Unresolved(reason) if reason=="test material")
        );
        assert_eq!(view.piece().definition().label.size, "M");
        assert_eq!(view.source_copies().collect::<Vec<_>>(), vec![view.copy()]);
    }
    assert_eq!(edit.operation(), id(1000));
    assert_eq!(edit.span_id(), id(301));
    assert_eq!(bound.span().definition().direction, SeamDirection::Opposite);
    assert_eq!(bound.span().definition().stops.len(), 1);
    assert_eq!(
        bound.span().definition().ease.declaration().amount,
        EaseAmount::Profile(ProfileParameterRef::new(id(8000)))
    );
    assert_eq!(pieces, before);
    assert_eq!(f.graph, original);
}
#[test]
fn first_actual_refusal_keeps_original_inputs_and_never_selects_one_duplicate_named_piece() {
    use JoinBindingRefusal as R;
    let f = fixture(false);
    let a = f.pieces.first().expect("A");
    let b = f.pieces.get(1).expect("B");
    let empty = CutPlan::new(vec![], &[]).expect("empty plan");
    let a_only = std::slice::from_ref(a);
    let one = CutPlan::new(
        vec![*f.plan.copy(id(200)).expect("A copy").definition()],
        a_only,
    )
    .expect("plan for actual A subset");
    let duplicate_a = vec![b.clone(), a.clone(), a.clone()];
    let duplicate_b = vec![a.clone(), b.clone(), b.clone()];
    let absent = JoinPiecesAtSeam::new(id(1000), id(999), id(100), id(101)).expect("external span");
    let rows = [
        (absent, &empty, &[][..], R::MissingSpan),
        (
            request(id(100), id(101)),
            &empty,
            &[][..],
            R::MissingCopy {
                side: SeamSideId::A,
            },
        ),
        (
            request(id(999), id(101)),
            &f.plan,
            &[][..],
            R::Piece {
                side: SeamSideId::A,
            },
        ),
        (
            request(id(100), id(101)),
            &f.plan,
            &[][..],
            R::MissingPiece {
                side: SeamSideId::A,
            },
        ),
        (
            request(id(100), id(101)),
            &one,
            duplicate_a.as_slice(),
            R::DuplicatePiece {
                side: SeamSideId::A,
                first: 1,
                second: 2,
            },
        ),
        (
            request(id(100), id(101)),
            &one,
            a_only,
            R::MissingCopy {
                side: SeamSideId::B,
            },
        ),
        (
            request(id(100), id(999)),
            &f.plan,
            a_only,
            R::Piece {
                side: SeamSideId::B,
            },
        ),
        (
            request(id(100), id(101)),
            &f.plan,
            a_only,
            R::MissingPiece {
                side: SeamSideId::B,
            },
        ),
        (
            request(id(100), id(101)),
            &f.plan,
            duplicate_b.as_slice(),
            R::DuplicatePiece {
                side: SeamSideId::B,
                first: 1,
                second: 2,
            },
        ),
    ];
    for (edit, plan, pieces, cause) in rows {
        let before = f.graph.clone();
        let error = edit
            .bind_source(&f.graph, plan, pieces)
            .expect_err("no accepted prefix");
        assert!(std::ptr::eq(error.edit(), &edit));
        assert!(std::ptr::eq(error.graph(), &f.graph));
        assert!(std::ptr::eq(error.plan(), plan));
        assert!(std::ptr::eq(error.pieces(), pieces));
        assert_eq!(error.refusal(), cause);
        assert_eq!(f.graph, before);
    }
}
#[test]
fn explicit_distinct_piece_variant_refuses_local_cycles_without_rejecting_valid_shared_piece_graphs(
) {
    use JoinInputError as E;
    for (span, a, b, cause) in [
        (
            1000,
            1000,
            1000,
            E::SelfSpan {
                operation: id(1000),
            },
        ),
        (
            301,
            1000,
            1000,
            E::SelfPiece {
                operation: id(1000),
                side: SeamSideId::A,
            },
        ),
        (
            301,
            100,
            1000,
            E::SelfPiece {
                operation: id(1000),
                side: SeamSideId::B,
            },
        ),
        (
            301,
            100,
            100,
            E::SamePiece {
                operation: id(1000),
                piece: id(100),
            },
        ),
    ] {
        assert_eq!(
            JoinPiecesAtSeam::new(id(1000), id(span), id(a), id(b)),
            Err(cause)
        );
    }
    let f = fixture(true);
    assert_eq!(
        f.graph
            .validate_current(&f.plan, &f.pieces, &f.marks, &f.ledger),
        Ok(())
    );
    let edit = request(id(100), id(101));
    let error = edit
        .bind_source(&f.graph, &f.plan, &f.pieces)
        .expect_err("distinct requested B disagrees with actual shared Piece");
    assert_eq!(
        error.refusal(),
        JoinBindingRefusal::Piece {
            side: SeamSideId::B
        }
    );
    assert_eq!(first_span(&f.graph).definition().b.copy, id(201));
}
#[test]
fn source_association_retains_a_stale_original_span_without_certifying_currentness_or_join_feasibility(
) {
    let mut f = fixture(false);
    let edit = request(id(100), id(101));
    let original = first_span(&f.graph).definition().clone();
    f.ledger
        .delete(
            &mut DeterministicIdGenerator::from_start(10000),
            original.a.range.edge(),
        )
        .expect("delete source");
    assert!(matches!(
        f.graph
            .validate_current(&f.plan, &f.pieces, &f.marks, &f.ledger),
        Err(SewingError::UnresolvedRange {
            side: SeamSideId::A,
            ..
        })
    ));
    let bound = edit
        .bind_source(&f.graph, &f.plan, &f.pieces)
        .expect("identity association only");
    assert_eq!(bound.span().definition(), &original);
    assert_eq!(
        bound.side(SeamSideId::A).source_side().range,
        original.a.range
    );
    assert_eq!(
        bound.side(SeamSideId::A).piece().geometric_validation(),
        GeometricValidation::DeferredToG2
    );
    assert_eq!(
        bound.graph().geometric_validation(),
        GeometricValidation::DeferredToG2
    );
}
