//! Actual Piece binding, whole population admission and original field provenance.
#![allow(clippy::expect_used)]
use sc_core::{
    construction::{
        MirrorAxis, MirrorPieceEdgeLocation as Location, MirrorPieceEdgeSource as Source,
        MirrorPieceSourceLimits as Limits, MirrorPieceSourceRefusal as Refusal,
    },
    ontology::*,
};

#[path = "support/mirror_source.rs"]
mod common;
use common::{axis, fixture, id, input};

#[test]
fn both_axes_bind_actual_piece_and_every_original_role_direction_and_repeated_field() {
    let (_, piece) = fixture();
    let original = piece.clone();
    let d = piece.definition();
    let expected = vec![
        (
            Location::Boundary(0),
            Source::Directed(d.boundary.first().expect("first boundary")),
        ),
        (
            Location::Boundary(1),
            Source::Directed(d.boundary.get(1).expect("second boundary")),
        ),
        (
            Location::Hole { hole: 0, edge: 0 },
            Source::Directed(
                d.holes
                    .first()
                    .and_then(|h| h.first())
                    .expect("first hole edge"),
            ),
        ),
        (
            Location::Hole { hole: 1, edge: 0 },
            Source::Directed(
                d.holes
                    .get(1)
                    .and_then(|h| h.first())
                    .expect("second hole edge"),
            ),
        ),
        (
            Location::Construction(0),
            Source::Directed(d.construction_lines.first().expect("first construction")),
        ),
        (
            Location::Construction(1),
            Source::Directed(d.construction_lines.get(1).expect("second construction")),
        ),
        (
            Location::Construction(2),
            Source::Directed(d.construction_lines.get(2).expect("third construction")),
        ),
        (
            Location::Fold(0),
            Source::Fold(d.fold_edges.first().expect("fold declaration")),
        ),
    ];
    let limits = Limits::new(2, 8);
    for axis in [
        axis(),
        MirrorAxis::AlongRange(DirectedRange {
            range: EdgeRange::whole(d.boundary.get(1).expect("second boundary").edge),
            direction: Direction::Reversed,
        }),
    ] {
        let input = input(10, 100, axis);
        let bound = input
            .bind_piece(&piece, &limits)
            .expect("exact whole quotas");
        assert!(std::ptr::eq(bound.input(), &input));
        assert!(std::ptr::eq(bound.piece(), &piece));
        assert!(std::ptr::eq(bound.limits(), &limits));
        assert_eq!(*bound.input().axis(), axis);
        assert_eq!(bound.edge_count(), 8);
        assert_eq!(limits.holes(), 2);
        assert_eq!(limits.edges(), 8);
        for (view, (location, source)) in bound.edges().zip(&expected) {
            assert!(std::ptr::eq(view.copy(), &bound));
            assert_eq!(view.location(), *location);
            assert_eq!(view.source(), *source);
            assert!(std::ptr::eq(view.source().edge(), source.edge()));
            match (view.source(), source) {
                (Source::Directed(actual), Source::Directed(original)) => {
                    assert!(std::ptr::eq(actual, *original));
                }
                (Source::Fold(actual), Source::Fold(original)) => {
                    assert!(std::ptr::eq(actual, *original));
                }
                _ => assert_eq!(view.source(), *source),
            }
        }
        assert_eq!(bound.edges().count(), expected.len());
        assert_eq!(
            bound
                .edges()
                .rev()
                .map(|v| (v.location(), v.source()))
                .collect::<Vec<_>>(),
            expected.iter().copied().rev().collect::<Vec<_>>()
        );
    }
    assert_eq!(piece, original);
}

#[test]
fn refusal_keeps_actual_input_piece_and_policy_with_identity_before_quotas_or_sources() {
    let (_, piece) = fixture();
    let input = input(40, 99, axis());
    let limits = Limits::new(0, 0);
    let error = input
        .bind_piece(&piece, &limits)
        .expect_err("wrong actual Piece first");
    assert_eq!(
        error.refusal(),
        Refusal::PieceIdentity {
            expected: id(99),
            actual: id(100)
        }
    );
    assert!(std::ptr::eq(error.input(), &input));
    assert!(std::ptr::eq(error.piece(), &piece));
    assert!(std::ptr::eq(error.limits(), &limits));
}

#[test]
fn whole_holes_then_all_occurrences_precede_self_source_including_repeat_and_fold_cost() {
    let (_, piece) = fixture();
    let input = input(40, 100, axis());
    let holes = Limits::new(1, 0);
    assert_eq!(
        input
            .bind_piece(&piece, &holes)
            .expect_err("whole H first")
            .refusal(),
        Refusal::Holes {
            required: 2,
            allowed: 1
        }
    );
    for allowance in [0, 7] {
        let edges = Limits::new(2, allowance);
        assert_eq!(
            input
                .bind_piece(&piece, &edges)
                .expect_err("whole R before any source")
                .refusal(),
            Refusal::Edges {
                required: 8,
                allowed: allowance
            }
        );
    }
    let exact = Limits::new(2, 8);
    assert_eq!(
        input
            .bind_piece(&piece, &exact)
            .expect_err("source only after quotas")
            .refusal(),
        Refusal::SelfSource {
            operation: id(40),
            location: Location::Boundary(0),
            edge: piece
                .definition()
                .boundary
                .first()
                .expect("first boundary")
                .edge
        }
    );
}

#[test]
fn first_original_boundary_hole_and_construction_sources_keep_exact_locations_and_tags() {
    let (ledger, piece) = fixture();
    let limits = Limits::new(2, 8);
    for (operation, location, edge) in [
        (
            41,
            Location::Boundary(1),
            piece
                .definition()
                .boundary
                .get(1)
                .expect("second boundary")
                .edge,
        ),
        (
            42,
            Location::Hole { hole: 0, edge: 0 },
            piece
                .definition()
                .holes
                .first()
                .and_then(|h| h.first())
                .expect("first hole edge")
                .edge,
        ),
        (
            43,
            Location::Hole { hole: 1, edge: 0 },
            piece
                .definition()
                .holes
                .get(1)
                .and_then(|h| h.first())
                .expect("second hole edge")
                .edge,
        ),
    ] {
        let input = input(operation, 100, axis());
        assert_eq!(
            input
                .bind_piece(&piece, &limits)
                .expect_err("original source role")
                .refusal(),
            Refusal::SelfSource {
                operation: id(operation),
                location,
                edge
            }
        );
    }
    let mut definition = piece.definition().clone();
    let source = *definition
        .holes
        .first()
        .and_then(|h| h.first())
        .expect("first hole edge");
    definition.holes.clear();
    definition.construction_lines = vec![
        *definition.boundary.get(1).expect("second boundary"),
        source,
    ];
    let construction =
        Piece::new(definition, &ledger).expect("actual separate construction source");
    let input = input(42, 100, axis());
    assert_eq!(
        input
            .bind_piece(&construction, &limits)
            .expect_err("construction source")
            .refusal(),
        Refusal::SelfSource {
            operation: id(42),
            location: Location::Construction(1),
            edge: source.edge
        }
    );
}

#[test]
fn absent_optional_groups_need_no_fabricated_edge_or_positive_hole_allowance() {
    let (ledger, piece) = fixture();
    let mut definition = piece.definition().clone();
    definition.boundary.truncate(1);
    definition.holes.clear();
    definition.construction_lines.clear();
    definition.cut_on_fold = false;
    definition.fold_edges.clear();
    definition.quantity = 1;
    definition.mirroring = Mirroring::Single;
    let piece = Piece::new(definition, &ledger).expect("minimal structural source");
    let input = input(10, 100, axis());
    let limits = Limits::new(0, 1);
    let bound = input
        .bind_piece(&piece, &limits)
        .expect("zero optional population");
    assert_eq!(bound.edge_count(), 1);
    assert_eq!(bound.edges().count(), 1);
    assert_eq!(
        bound.edges().next().expect("actual boundary").location(),
        Location::Boundary(0)
    );
}
