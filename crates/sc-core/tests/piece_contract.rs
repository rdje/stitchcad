//! Contract tests for immutable, structurally validated pieces (`G1-SLICE.3c.1`).
//!
//! The declaration journal supplies an independent census of live edges. Tests mutate exactly one
//! required property at a time and match the diagnostic's payload, rather than merely accepting any
//! refusal. No test claims to prove winding or endpoint closure in the absence of a geometry kernel.

use sc_core::ontology::{
    CuttingSide, DeterministicIdGenerator, DirectedEdge, Direction, EdgeRef, EntityId,
    GeometricValidation, IdGenerator, IdentityLedger, LabelField, LabelText, LocalTag,
    LoopLocation, MaterialAssignment, Mirroring, Param, Piece, PieceDefinition, PieceError,
    Rational, ReleaseReadiness, Resolution,
};

#[allow(clippy::expect_used)]
fn fixture() -> (PieceDefinition, IdentityLedger, DeterministicIdGenerator) {
    let mut ids = DeterministicIdGenerator::new();
    let mut ledger = IdentityLedger::new();
    let edges = ledger
        .declare_edges(&mut ids, 8)
        .expect("bounded declaration");
    let directed = edges
        .iter()
        .copied()
        .map(|edge| DirectedEdge {
            edge,
            direction: Direction::Original,
        })
        .collect::<Vec<_>>();
    let definition = PieceDefinition {
        id: ids.next_id(),
        boundary: directed.iter().take(4).copied().collect(),
        holes: vec![directed.iter().skip(4).take(3).copied().collect()],
        construction_lines: directed.iter().skip(7).copied().collect(),
        quantity: 2,
        mirroring: Mirroring::MirroredPairs,
        cut_on_fold: false,
        fold_edges: vec![],
        cutting_side: CuttingSide::Face,
        material: MaterialAssignment::Unresolved("awaiting fabric selection".into()),
        layer_index: 0,
        label: LabelText {
            name: "skirt front".into(),
            size: "M".into(),
            fabric: "unassigned woven fabric".into(),
            colorway: "undetermined".into(),
        },
    };
    (definition, ledger, ids)
}

#[test]
fn every_required_field_is_retained_and_geometry_is_visibly_deferred() {
    let (definition, ledger, _) = fixture();
    let piece = Piece::new(definition.clone(), &ledger).unwrap();
    assert_eq!(piece.definition(), &definition);
    assert_eq!(piece.id(), definition.id);
    assert_eq!(
        piece.geometric_validation(),
        GeometricValidation::DeferredToG2
    );
    let label = piece.printed_label();
    assert_eq!(label.text, &definition.label);
    assert_eq!(label.quantity, 2);
    assert_eq!(label.mirroring, Mirroring::MirroredPairs);
    assert!(!label.cut_on_fold);
    assert_eq!(
        piece.edges().map(|item| item.edge).collect::<Vec<_>>(),
        ledger.live_edges().collect::<Vec<_>>()
    );
}

#[test]
fn empty_boundary_and_each_empty_hole_name_the_exact_loop() {
    let (mut definition, ledger, _) = fixture();
    let mut empty_boundary = definition.clone();
    empty_boundary.boundary.clear();
    assert_eq!(
        Piece::new(empty_boundary, &ledger),
        Err(PieceError::EmptyLoop {
            location: LoopLocation::Boundary
        })
    );
    definition.holes.push(vec![]);
    assert_eq!(
        Piece::new(definition, &ledger),
        Err(PieceError::EmptyLoop {
            location: LoopLocation::Hole(1)
        })
    );
}

#[test]
fn repeated_closing_edge_and_edge_shared_by_cut_loops_are_refused() {
    let (definition, ledger, _) = fixture();
    let first = *definition.boundary.first().unwrap();
    let mut repeated = definition.clone();
    repeated.boundary.push(DirectedEdge {
        direction: Direction::Reversed,
        ..first
    });
    assert_eq!(
        Piece::new(repeated, &ledger),
        Err(PieceError::RepeatedCutEdge {
            location: LoopLocation::Boundary,
            edge: first.edge
        })
    );
    let mut shared = definition;
    shared.holes.push(vec![first]);
    assert_eq!(
        Piece::new(shared, &ledger),
        Err(PieceError::RepeatedCutEdge {
            location: LoopLocation::Hole(1),
            edge: first.edge
        })
    );
}

#[test]
fn an_absent_edge_is_refused_in_every_reference_position() {
    let (definition, ledger, _) = fixture();
    let missing = DirectedEdge {
        edge: EdgeRef::new(EntityId::from_bits(u128::MAX), LocalTag::FIRST),
        direction: Direction::Original,
    };
    let mut boundary = definition.clone();
    boundary.boundary.push(missing);
    let mut hole = definition.clone();
    hole.holes.push(vec![missing]);
    let mut construction = definition;
    construction.construction_lines.push(missing);
    for input in [boundary, hole, construction] {
        assert_eq!(
            Piece::new(input, &ledger),
            Err(PieceError::MissingEdge { edge: missing.edge })
        );
    }
}

#[test]
fn cut_quantity_and_mirroring_cannot_make_an_empty_or_incomplete_pair() {
    let (mut definition, ledger, _) = fixture();
    definition.quantity = 0;
    assert_eq!(
        Piece::new(definition.clone(), &ledger),
        Err(PieceError::ZeroQuantity)
    );
    definition.quantity = 3;
    assert_eq!(
        Piece::new(definition.clone(), &ledger),
        Err(PieceError::UnpairedQuantity { quantity: 3 })
    );
    definition.mirroring = Mirroring::Single;
    definition.quantity = u32::MAX;
    assert_eq!(
        Piece::new(definition, &ledger)
            .unwrap()
            .printed_label()
            .quantity,
        u32::MAX
    );
}

#[test]
fn cut_on_fold_requires_exactly_one_outer_boundary_edge() {
    let (definition, ledger, _) = fixture();
    let boundary = definition.boundary.first().unwrap().edge;
    let hole = definition.holes.first().unwrap().first().unwrap().edge;
    for (cut_on_fold, edges, expected) in [
        (
            false,
            vec![boundary],
            PieceError::FoldCount {
                expected: 0,
                actual: 1,
            },
        ),
        (
            true,
            vec![],
            PieceError::FoldCount {
                expected: 1,
                actual: 0,
            },
        ),
        (
            true,
            vec![boundary, boundary],
            PieceError::FoldCount {
                expected: 1,
                actual: 2,
            },
        ),
        (
            true,
            vec![hole],
            PieceError::FoldNotOnBoundary { edge: hole },
        ),
    ] {
        let mut input = definition.clone();
        input.cut_on_fold = cut_on_fold;
        input.fold_edges = edges;
        assert_eq!(Piece::new(input, &ledger), Err(expected));
    }
    let mut valid = definition;
    valid.cut_on_fold = true;
    valid.fold_edges = vec![boundary];
    assert!(
        Piece::new(valid, &ledger)
            .unwrap()
            .printed_label()
            .cut_on_fold
    );
}

#[test]
fn every_print_text_field_must_be_nonblank() {
    let (definition, ledger, _) = fixture();
    for field in [
        LabelField::Name,
        LabelField::Size,
        LabelField::Fabric,
        LabelField::Colorway,
    ] {
        for blank in ["", " \n\t", "\u{2003}"] {
            let mut input = definition.clone();
            let text = match field {
                LabelField::Name => &mut input.label.name,
                LabelField::Size => &mut input.label.size,
                LabelField::Fabric => &mut input.label.fabric,
                LabelField::Colorway => &mut input.label.colorway,
            };
            *text = blank.into();
            assert_eq!(
                Piece::new(input, &ledger),
                Err(PieceError::IncompleteLabel { field })
            );
        }
    }
}

#[test]
fn unresolved_material_is_explicit_and_requires_an_explanation() {
    let (mut definition, ledger, _) = fixture();
    definition.material = MaterialAssignment::Unresolved(" \t".into());
    assert_eq!(
        Piece::new(definition.clone(), &ledger),
        Err(PieceError::UnexplainedMaterial)
    );
    definition.material = MaterialAssignment::Assigned(EntityId::from_bits(55));
    assert!(Piece::new(definition, &ledger).is_ok());
}

#[test]
fn modifying_a_definition_copy_does_not_mutate_a_valid_piece() {
    let (definition, ledger, _) = fixture();
    let piece = Piece::new(definition.clone(), &ledger).unwrap();
    let mut copy = piece.definition().clone();
    copy.boundary.clear();
    assert_eq!(piece.definition(), &definition);
    assert!(matches!(
        Piece::new(copy, &ledger),
        Err(PieceError::EmptyLoop { .. })
    ));
}

#[test]
fn edits_leave_authored_content_intact_and_endpoint_queries_show_repairs() {
    let (definition, mut ledger, mut ids) = fixture();
    let piece = Piece::new(definition.clone(), &ledger).unwrap();
    let edge = definition.boundary.first().unwrap().edge;
    for (edge, param) in piece.endpoint_references() {
        ledger.register(piece.id(), edge, param).unwrap();
    }
    ledger.reverse(&mut ids, edge).unwrap();
    assert_eq!(piece.definition(), &definition);
    let reversed = piece
        .endpoint_resolutions(&ledger)
        .find(|(e, t, _)| *e == edge && *t == Param::START)
        .unwrap()
        .2;
    let resolved = reversed.resolved().unwrap();
    assert_eq!(resolved.param(), Param::END);
    assert_eq!(resolved.direction(), Direction::Reversed);
    ledger.delete(&mut ids, edge).unwrap();
    assert_eq!(piece.definition(), &definition);
    for (held, _, resolution) in piece.endpoint_resolutions(&ledger) {
        assert_eq!(
            matches!(resolution, Resolution::Unresolved(_)),
            held == edge
        );
    }
    assert_eq!(ledger.open_repairs().len(), 2);
    assert_eq!(
        Piece::new(definition, &ledger),
        Err(PieceError::MissingEdge { edge })
    );
}

#[test]
fn cyclic_order_has_no_geometric_proof_and_directions_remain_authored() {
    let (mut definition, ledger, _) = fixture();
    definition.boundary.reverse();
    for item in &mut definition.boundary {
        item.direction = Direction::Reversed;
    }
    let piece = Piece::new(definition, &ledger).unwrap();
    assert_eq!(
        piece.geometric_validation(),
        GeometricValidation::DeferredToG2
    );
    assert_eq!(piece.endpoint_references().next().unwrap().1, Param::END);
}

#[test]
fn endpoint_inventory_cannot_certify_the_interior_of_an_edge_range() {
    // D55: a point-reference release verdict must not become a range-integrity verdict.
    let (definition, mut ledger, mut ids) = fixture();
    let piece = Piece::new(definition, &ledger).unwrap();
    let edge = piece.definition().boundary.first().unwrap().edge;
    for (edge, param) in piece.endpoint_references() {
        ledger.register(piece.id(), edge, param).unwrap();
    }
    let first = ledger
        .split(
            &mut ids,
            edge,
            Param::new(Rational::new(1, 3).unwrap()).unwrap(),
        )
        .unwrap();
    let second = ledger
        .split(
            &mut ids,
            first.second(),
            Param::new(Rational::new(1, 2).unwrap()).unwrap(),
        )
        .unwrap();
    ledger.delete(&mut ids, second.first()).unwrap();
    assert!(piece
        .endpoint_resolutions(&ledger)
        .all(|(_, _, state)| state.resolved().is_some()));
    assert_eq!(ledger.release_readiness(), ReleaseReadiness::Releasable);
    assert_eq!(
        piece.geometric_validation(),
        GeometricValidation::DeferredToG2
    );
    let whole = piece
        .range_resolutions(&ledger)
        .find(|(item, _)| item.edge == edge)
        .unwrap()
        .1;
    assert!(!whole.has_full_coverage());
    assert_eq!(whole.repairs().count(), 1);
    // G1-SLICE.3c.2a supplies range evidence; G2 must still certify full contours.
}
