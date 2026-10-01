//! Semantic-anchor contracts; physical values deliberately remain a G4 obligation.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use sc_core::ontology::{
    CuttingSide, DeterministicIdGenerator, DirectedEdge, Direction, EdgeAnchor, EdgeRef, EntityId,
    IdGenerator, IdentityLedger, LabelText, LocalTag, MaterialAssignment, Mirroring, Notch,
    NotchDefinition, NotchDimensionBindings, NotchError, NotchProfileBindings, OrphaningEdit,
    Param, Piece, PieceDefinition, ProfileBindingValidation, ProfileParameterRef, Rational,
    Resolution, SplitSide,
};
use sc_units::Length;

fn t(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).unwrap()).unwrap()
}

fn bindings() -> NotchProfileBindings {
    let parameter = |id| ProfileParameterRef::new(EntityId::from_bits(id));
    NotchProfileBindings {
        style: parameter(101),
        sample: NotchDimensionBindings {
            depth: parameter(102),
            width: parameter(103),
        },
        production: NotchDimensionBindings {
            depth: parameter(104),
            width: parameter(105),
        },
        encoding: parameter(106),
    }
}

fn fixture() -> (Piece, IdentityLedger, DeterministicIdGenerator) {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let edges = ledger.declare_edges(&mut ids, 3).unwrap();
    let directed = |index| DirectedEdge {
        edge: *edges.get(index).unwrap(),
        direction: Direction::Original,
    };
    let piece = Piece::new(
        PieceDefinition {
            id: ids.next_id(),
            boundary: vec![directed(0)],
            holes: vec![vec![directed(1)]],
            construction_lines: vec![directed(2)],
            quantity: 1,
            mirroring: Mirroring::Single,
            cut_on_fold: false,
            fold_edges: vec![],
            cutting_side: CuttingSide::Face,
            material: MaterialAssignment::Unresolved("selection pending".into()),
            layer_index: 0,
            label: LabelText {
                name: "test piece".into(),
                size: "M".into(),
                fabric: "woven".into(),
                colorway: "plain".into(),
            },
        },
        &ledger,
    )
    .unwrap();
    (piece, ledger, ids)
}

fn definition(edge: EdgeRef, param: Param) -> NotchDefinition {
    NotchDefinition {
        id: EntityId::from_bits(200),
        anchor: EdgeAnchor { edge, param },
        representation: bindings(),
    }
}

#[test]
fn every_owned_edge_class_accepts_anchors_including_endpoints() {
    let (piece, ledger, _) = fixture();
    for directed in piece.edges() {
        for param in [Param::START, t(1, 3), Param::END] {
            let input = definition(directed.edge, param);
            let notch = Notch::new(input.clone(), &piece, &ledger).unwrap();
            assert_eq!(notch.id(), input.id);
            assert_eq!(notch.piece(), piece.id());
            assert_eq!(notch.definition(), &input);
            let resolved = notch.resolve(&ledger).resolved().unwrap();
            assert_eq!((resolved.edge(), resolved.param()), (directed.edge, param));
        }
    }
}

#[test]
fn symbolic_bindings_never_claim_a_profile_or_supply_physical_values() {
    let (piece, ledger, _) = fixture();
    let edge = piece.definition().boundary.first().unwrap().edge;
    let notch = Notch::new(definition(edge, t(1, 2)), &piece, &ledger).unwrap();
    let retained = notch.definition().representation;
    assert_eq!(retained, bindings());
    assert_ne!(retained.sample, retained.production);
    assert_eq!(retained.style.id(), EntityId::from_bits(101));
    assert_eq!(
        notch.profile_binding_validation(),
        ProfileBindingValidation::DeferredToG4
    );
    let mut input = notch.definition().clone();
    input.representation.production = input.representation.sample;
    // Sharing declarations is intentional and legal, rather than assuming different numeric values.
    let shared = Notch::new(input, &piece, &ledger).unwrap();
    assert_eq!(
        shared.definition().representation.sample,
        shared.definition().representation.production
    );
    assert_eq!(
        shared.profile_binding_validation(),
        ProfileBindingValidation::DeferredToG4
    );
    assert_eq!(notch.definition().representation, bindings());
}

#[test]
fn absent_and_live_foreign_edges_have_distinct_refusals() {
    let (piece, mut ledger, mut ids) = fixture();
    let absent = EdgeRef::new(EntityId::from_bits(u128::MAX), LocalTag::FIRST);
    assert_eq!(
        Notch::new(definition(absent, Param::START), &piece, &ledger),
        Err(NotchError::MissingEdge { edge: absent })
    );
    let foreign = *ledger.declare_edges(&mut ids, 1).unwrap().first().unwrap();
    let input = definition(foreign, t(1, 2));
    assert_eq!(
        Notch::new(input.clone(), &piece, &ledger),
        Err(NotchError::AnchorOutsidePiece {
            piece: piece.id(),
            anchor: input.anchor
        })
    );
}

#[test]
fn reversal_and_merge_recompute_without_changing_authored_anchor() {
    let (piece, mut ledger, mut ids) = fixture();
    let edge = piece.definition().boundary.first().unwrap().edge;
    let input = definition(edge, t(1, 3));
    let notch = Notch::new(input.clone(), &piece, &ledger).unwrap();
    ledger.reverse(&mut ids, edge).unwrap();
    let point = notch.resolve(&ledger).resolved().unwrap();
    assert_eq!(
        (point.param(), point.direction()),
        (t(2, 3), Direction::Reversed)
    );
    let other = *ledger.declare_edges(&mut ids, 1).unwrap().first().unwrap();
    let merged = ledger
        .merge(
            &mut ids,
            edge,
            Length::from_micrometres(3).unwrap(),
            other,
            Length::from_micrometres(7).unwrap(),
        )
        .unwrap();
    let point = notch.resolve(&ledger).resolved().unwrap();
    assert_eq!(
        (point.edge(), point.param(), point.direction()),
        (merged, t(1, 5), Direction::Reversed)
    );
    assert_eq!(notch.definition(), &input);
}

#[test]
fn split_point_requires_an_explicit_side_and_new_fragment_is_owned() {
    let (piece, mut ledger, mut ids) = fixture();
    let edge = piece.definition().boundary.first().unwrap().edge;
    let input = definition(edge, t(2, 5));
    let notch = Notch::new(input.clone(), &piece, &ledger).unwrap();
    let split = ledger.split(&mut ids, edge, t(2, 5)).unwrap();
    let resolution = notch.resolve(&ledger);
    assert!(matches!(resolution, Resolution::SplitPoint { .. }));
    assert!(resolution.resolved().is_none());
    let first = resolution
        .clone()
        .choose(SplitSide::First)
        .resolved()
        .unwrap();
    let second = resolution.choose(SplitSide::Second).resolved().unwrap();
    assert_eq!((first.edge(), first.param()), (split.first(), Param::END));
    assert_eq!(
        (second.edge(), second.param()),
        (split.second(), Param::START)
    );
    assert_eq!(notch.definition(), &input);
    assert!(Notch::new(definition(split.second(), t(1, 3)), &piece, &ledger).is_ok());
    // New objects cannot be born on a retired source, even though existing held anchors still resolve.
    assert_eq!(
        Notch::new(input, &piece, &ledger),
        Err(NotchError::MissingEdge { edge })
    );
}

#[test]
fn deletion_exposes_the_held_anchor_and_operation_without_reassignment() {
    let (piece, mut ledger, mut ids) = fixture();
    let edge = piece.definition().boundary.first().unwrap().edge;
    let input = definition(edge, t(1, 4));
    let notch = Notch::new(input.clone(), &piece, &ledger).unwrap();
    let operation = ledger.delete(&mut ids, edge).unwrap();
    let Resolution::Unresolved(task) = notch.resolve(&ledger) else {
        panic!("delete must retain a repair")
    };
    assert_eq!(task.reference(), edge);
    assert_eq!(task.param(), t(1, 4));
    assert_eq!(
        task.orphaned_by(),
        &OrphaningEdit::Deleted { operation, edge }
    );
    assert!(task.candidates().is_empty());
    assert_eq!(notch.definition(), &input);
}

#[test]
fn ownership_uses_surviving_partial_ranges_and_resolved_parameters() {
    let (piece, mut ledger, mut ids) = fixture();
    let held = piece.definition().boundary.first().unwrap().edge;
    let other = *ledger.declare_edges(&mut ids, 1).unwrap().first().unwrap();
    let merged = ledger
        .merge(
            &mut ids,
            held,
            Length::from_micrometres(3).unwrap(),
            other,
            Length::from_micrometres(7).unwrap(),
        )
        .unwrap();
    // The piece owns [0,3/10] of the merged edge, not the foreign remainder.
    assert!(Notch::new(definition(merged, t(1, 5)), &piece, &ledger).is_ok());
    let foreign = definition(merged, t(4, 5));
    assert_eq!(
        Notch::new(foreign.clone(), &piece, &ledger),
        Err(NotchError::AnchorOutsidePiece {
            piece: piece.id(),
            anchor: foreign.anchor
        })
    );
    ledger.reverse(&mut ids, merged).unwrap();
    // Both references are held in their original frame. Ownership must compare resolved t=4/5
    // with the resolved piece interval [7/10,1], not the authored t=1/5 with that interval.
    assert!(Notch::new(definition(merged, t(1, 5)), &piece, &ledger).is_ok());
    assert!(matches!(
        Notch::new(foreign, &piece, &ledger),
        Err(NotchError::AnchorOutsidePiece { .. })
    ));
}
