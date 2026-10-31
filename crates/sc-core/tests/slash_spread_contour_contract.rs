//! Structural contour/source contracts only;one-edge cyclic declarations still require G2 geometry.
#![allow(clippy::expect_used)]
use sc_core::{
    construction::*, name::MachineToken, ontology::*, recipe::FormulaExpressionDocument,
};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
struct Fixture {
    ledger: IdentityLedger,
    piece: Piece,
    first: DirectedEdge,
    second: DirectedEdge,
}
fn fixture() -> Fixture {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let edges = ledger
        .declare_edges(&mut ids, 2)
        .expect("actual source identities");
    let first = DirectedEdge {
        edge: *edges.first().expect("boundary"),
        direction: Direction::Original,
    };
    let second = DirectedEdge {
        edge: *edges.get(1).expect("construction"),
        direction: Direction::Original,
    };
    let piece = Piece::new(
        PieceDefinition {
            id: id(100),
            boundary: vec![first],
            holes: vec![],
            construction_lines: vec![second],
            quantity: 1,
            mirroring: Mirroring::Single,
            cut_on_fold: false,
            fold_edges: vec![],
            cutting_side: CuttingSide::Face,
            material: MaterialAssignment::Unresolved("explicit pending material".into()),
            layer_index: 0,
            label: LabelText {
                name: "source".into(),
                size: "M".into(),
                fabric: "woven".into(),
                colorway: "plain".into(),
            },
        },
        &ledger,
    )
    .expect("structural source Piece");
    Fixture {
        ledger,
        piece,
        first,
        second,
    }
}
fn intent(f: &Fixture) -> SlashSpreadOutputIntent {
    let input = SlashSpreadConstruction::rotation(
        id(10),
        id(100),
        DirectedRange {
            range: EdgeRange::whole(f.first.edge),
            direction: Direction::Original,
        },
        SlashSpreadSide::Left,
        PointRef::new(id(60), LocalTag::FIRST),
        FormulaExpressionDocument::new("10 deg".into()).expect("angle"),
    )
    .expect("spatial inputs");
    SlashSpreadOutputIntent::new(
        input,
        SlashSpreadBoundaryPolicy::StraightBridges,
        SlashSpreadFullnessIntent::Added,
    )
}
fn catalog() -> EdgeOutputRoleCatalog {
    EdgeOutputRoleCatalog::new(
        id(10),
        [(9, "main"), (2, "aux")]
            .into_iter()
            .map(|(tag, key)| {
                EdgeOutputRoleDefinition::new(
                    LocalTag::new(tag),
                    MachineToken::new(key).expect("key"),
                )
            })
            .collect(),
        vec![LocalTag::new(2), LocalTag::new(9)],
    )
}
fn catalogue_limits() -> EdgeOutputRoleCatalogLimits {
    EdgeOutputRoleCatalogLimits::new(2, 2, 7)
}
fn manifest(f: &Fixture) -> SlashSpreadEdgeManifest {
    SlashSpreadEdgeManifest::new(
        id(10),
        vec![
            SlashSpreadEdgeCandidate::new(
                LocalTag::new(9),
                SlashSpreadEdgeOrigin::Source {
                    range: DirectedRange {
                        range: EdgeRange::whole(f.first.edge),
                        direction: Direction::Original,
                    },
                    motion: SlashSpreadSourceMotion::Moving,
                },
            ),
            SlashSpreadEdgeCandidate::new(
                LocalTag::new(2),
                SlashSpreadEdgeOrigin::StraightBridge {
                    end: SlashSpreadCutEnd::Upper,
                    direction: SlashSpreadBridgeDirection::MovingToStationary,
                },
            ),
        ],
    )
}
fn output(tag: u32, direction: Direction) -> SlashSpreadContourEdge {
    SlashSpreadContourEdge::Output {
        tag: LocalTag::new(tag),
        direction,
    }
}
fn contours(f: &Fixture) -> SlashSpreadContours {
    SlashSpreadContours::new(
        vec![
            SlashSpreadContourEdge::Retained(f.first),
            output(9, Direction::Reversed),
        ],
        vec![vec![output(2, Direction::Original)]],
        vec![
            output(9, Direction::Original),
            output(9, Direction::Reversed),
            SlashSpreadContourEdge::Retained(f.second),
        ],
    )
}
fn limits() -> SlashSpreadContourLimits {
    SlashSpreadContourLimits::new(2, 2, 0, 1, 6)
}
#[test]
fn authentic_mixed_uses_keep_original_identities_directions_positions_and_complete_source() {
    use SlashSpreadContourLocation as L;
    let f = fixture();
    let intent = intent(&f);
    let raw_catalog = catalog();
    let catalog = raw_catalog.admit(catalogue_limits()).expect("catalogue");
    let bound = intent
        .bind_edge_roles(&catalog, catalogue_limits())
        .expect("operation");
    let raw_edges = manifest(&f);
    let edges = raw_edges
        .admit(
            &bound,
            SlashSpreadEdgeManifestLimits::new(2, catalogue_limits()),
        )
        .expect("candidate metadata");
    let raw = contours(&f);
    let original = raw.clone();
    let source = f.piece.clone();
    let policy = limits();
    let admitted = raw
        .admit(&edges, &f.piece, &policy)
        .expect("structural uses");
    assert!(std::ptr::eq(admitted.contours(), &raw));
    assert!(std::ptr::eq(admitted.edges(), &edges));
    assert!(std::ptr::eq(admitted.source_piece(), &f.piece));
    assert_eq!(admitted.limits(), &policy);
    assert!(std::ptr::eq(admitted.limits(), &policy));
    assert_eq!(
        (
            limits().candidates(),
            limits().source_edges(),
            limits().source_holes(),
            limits().holes(),
            limits().uses()
        ),
        (2, 2, 0, 1, 6)
    );
    assert_eq!(admitted.workspace_entries(), 15);
    assert_eq!(admitted.uses().len(), 6);
    let positions = [
        L::Boundary(0),
        L::Boundary(1),
        L::Hole { hole: 0, edge: 0 },
        L::Construction(0),
        L::Construction(1),
        L::Construction(2),
    ];
    let fields = [
        raw.boundary().first().expect("first boundary field"),
        raw.boundary().get(1).expect("second boundary field"),
        raw.holes()
            .first()
            .expect("first hole")
            .first()
            .expect("first hole field"),
        raw.construction_lines()
            .first()
            .expect("first construction field"),
        raw.construction_lines()
            .get(1)
            .expect("second construction field"),
        raw.construction_lines()
            .get(2)
            .expect("third construction field"),
    ];
    for ((usage, location), field) in admitted.uses().zip(positions).zip(fields) {
        assert!(std::ptr::eq(usage.contours(), &admitted));
        assert_eq!(usage.location(), location);
        assert!(std::ptr::eq(usage.declared(), field));
        assert_eq!(usage.direction(), field.direction());
        match field {
            SlashSpreadContourEdge::Output { tag, .. } => {
                let actual = usage.output().expect("authentic candidate");
                assert!(std::ptr::eq(actual.admitted(), &edges));
                assert_eq!(actual.candidate().tag(), *tag);
                assert_eq!(usage.edge(), EdgeRef::new(id(10), *tag));
                assert!(usage.retained().is_none());
            }
            SlashSpreadContourEdge::Retained(directed) => {
                assert!(std::ptr::eq(
                    usage.retained().expect("actual retained field"),
                    directed
                ));
                assert_eq!(usage.edge(), directed.edge);
                assert!(usage.output().is_none());
            }
        }
    }
    assert_eq!(
        admitted
            .uses()
            .rev()
            .map(|u| u.location())
            .collect::<Vec<_>>(),
        positions.into_iter().rev().collect::<Vec<_>>()
    );
    assert_eq!(raw, original);
    assert_eq!(f.piece, source);
}
#[test]
fn complete_quotas_owner_and_original_structural_refusals_preserve_every_input() {
    use SlashSpreadContourLocation as L;
    use SlashSpreadContourRefusal as R;
    let f = fixture();
    let intent = intent(&f);
    let raw_catalog = catalog();
    let catalog = raw_catalog.admit(catalogue_limits()).expect("catalogue");
    let bound = intent
        .bind_edge_roles(&catalog, catalogue_limits())
        .expect("operation");
    let raw_edges = manifest(&f);
    let edges = raw_edges
        .admit(
            &bound,
            SlashSpreadEdgeManifestLimits::new(2, catalogue_limits()),
        )
        .expect("edges");
    let mut wrong = f.piece.definition().clone();
    wrong.id = id(999);
    let wrong = Piece::new(wrong, &f.ledger).expect("different actual Piece");
    let mut hole = f.piece.definition().clone();
    hole.holes = vec![vec![f.second]];
    let hole = Piece::new(hole, &f.ledger).expect("source hole with construction occurrence");
    let mut ledger = f.ledger.clone();
    let own = ledger
        .declare_tagged_edges(id(10), &[LocalTag::FIRST])
        .expect("actual wrong-position source identity");
    let mut own_source = f.piece.definition().clone();
    own_source.boundary = vec![DirectedEdge {
        edge: *own.first().expect("own edge"),
        direction: Direction::Original,
    }];
    let own_source =
        Piece::new(own_source, &ledger).expect("birth-valid but self-dependent source");
    let make =
        |boundary, holes, construction| SlashSpreadContours::new(boundary, holes, construction);
    let lone = || make(vec![output(9, Direction::Original)], vec![], vec![]);
    let foreign = DirectedEdge {
        edge: EdgeRef::new(id(555), LocalTag::FIRST),
        direction: Direction::Original,
    };
    let self_edge = DirectedEdge {
        edge: EdgeRef::new(id(10), LocalTag::new(9)),
        direction: Direction::Original,
    };
    let rows = [
        (
            &wrong,
            contours(&f),
            SlashSpreadContourLimits::new(0, 0, 0, 0, 0),
            R::Holes {
                required: 1,
                allowed: 0,
            },
        ),
        (
            &hole,
            contours(&f),
            limits(),
            R::SourceHoles {
                required: 1,
                allowed: 0,
            },
        ),
        (
            &f.piece,
            contours(&f),
            SlashSpreadContourLimits::new(1, 0, 0, 1, 0),
            R::Candidates {
                required: 2,
                allowed: 1,
            },
        ),
        (
            &f.piece,
            contours(&f),
            SlashSpreadContourLimits::new(2, 1, 0, 1, 0),
            R::SourceEdges {
                required: 2,
                allowed: 1,
            },
        ),
        (
            &f.piece,
            contours(&f),
            SlashSpreadContourLimits::new(2, 2, 0, 1, 5),
            R::Uses {
                required: 6,
                allowed: 5,
            },
        ),
        (
            &wrong,
            contours(&f),
            limits(),
            R::SourcePiece {
                required: id(100),
                provided: id(999),
            },
        ),
        (
            &own_source,
            make(
                vec![],
                vec![],
                vec![
                    output(9, Direction::Original),
                    output(2, Direction::Original),
                ],
            ),
            limits(),
            R::EmptyBoundary,
        ),
        (
            &own_source,
            contours(&f),
            limits(),
            R::SelfSource { source_edge: 0 },
        ),
        (
            &f.piece,
            make(vec![output(99, Direction::Original)], vec![], vec![]),
            limits(),
            R::MissingOutput {
                location: L::Boundary(0),
                tag: LocalTag::new(99),
            },
        ),
        (
            &f.piece,
            make(
                vec![SlashSpreadContourEdge::Retained(self_edge)],
                vec![],
                vec![],
            ),
            limits(),
            R::SelfRetained {
                location: L::Boundary(0),
            },
        ),
        (
            &f.piece,
            make(
                vec![SlashSpreadContourEdge::Retained(foreign)],
                vec![],
                vec![],
            ),
            limits(),
            R::ForeignRetained {
                location: L::Boundary(0),
                edge: foreign.edge,
            },
        ),
        (
            &f.piece,
            make(
                vec![output(9, Direction::Original)],
                vec![vec![]],
                vec![output(2, Direction::Original)],
            ),
            limits(),
            R::EmptyHole { hole: 0 },
        ),
        (
            &f.piece,
            make(
                vec![output(9, Direction::Original)],
                vec![vec![output(9, Direction::Reversed)]],
                vec![output(2, Direction::Original)],
            ),
            limits(),
            R::RepeatedCut {
                first: L::Boundary(0),
                second: L::Hole { hole: 0, edge: 0 },
            },
        ),
        (
            &f.piece,
            make(
                vec![SlashSpreadContourEdge::Retained(f.first)],
                vec![vec![SlashSpreadContourEdge::Retained(f.first)]],
                vec![],
            ),
            limits(),
            R::RepeatedCut {
                first: L::Boundary(0),
                second: L::Hole { hole: 0, edge: 0 },
            },
        ),
        (
            &f.piece,
            lone(),
            limits(),
            R::UnusedCandidate {
                candidate: 1,
                tag: LocalTag::new(2),
            },
        ),
        (
            &f.piece,
            make(
                vec![SlashSpreadContourEdge::Retained(f.first)],
                vec![],
                vec![],
            ),
            limits(),
            R::UnusedCandidate {
                candidate: 0,
                tag: LocalTag::new(9),
            },
        ),
    ];
    for (source, raw, limits, cause) in rows {
        let original = raw.clone();
        let original_source = source.clone();
        let error = raw
            .admit(&edges, source, &limits)
            .expect_err("whole refusal");
        assert!(std::ptr::eq(error.contours(), &raw));
        assert!(std::ptr::eq(error.edges(), &edges));
        assert!(std::ptr::eq(error.source_piece(), source));
        assert_eq!(error.limits(), &limits);
        assert!(std::ptr::eq(error.limits(), &limits));
        assert_eq!(error.refusal(), cause);
        let repeated = if matches!(cause, R::RepeatedCut { .. }) {
            Some(
                match raw.boundary().first().expect("first repeated cut field") {
                    SlashSpreadContourEdge::Output { tag, .. } => EdgeRef::new(id(10), *tag),
                    SlashSpreadContourEdge::Retained(directed) => directed.edge,
                },
            )
        } else {
            None
        };
        assert_eq!(error.repeated_edge(), repeated);
        assert_eq!(raw, original);
        assert_eq!(*source, original_source);
        assert!(format!("{error}").contains("contour admission"));
        assert!(format!("{error:?}").contains("SlashSpreadContourError"));
    }
}
#[test]
fn construction_occurrences_can_repeat_cut_edges_without_dropping_any_explicit_output() {
    let f = fixture();
    let intent = intent(&f);
    let raw_catalog = catalog();
    let catalog = raw_catalog.admit(catalogue_limits()).expect("catalogue");
    let bound = intent
        .bind_edge_roles(&catalog, catalogue_limits())
        .expect("operation");
    let raw_edges = manifest(&f);
    let edges = raw_edges
        .admit(
            &bound,
            SlashSpreadEdgeManifestLimits::new(2, catalogue_limits()),
        )
        .expect("edges");
    let raw = SlashSpreadContours::new(
        vec![output(9, Direction::Original)],
        vec![],
        vec![
            output(9, Direction::Reversed),
            output(2, Direction::Original),
            output(2, Direction::Reversed),
            SlashSpreadContourEdge::Retained(f.first),
            SlashSpreadContourEdge::Retained(f.first),
        ],
    );
    let policy = limits();
    let admitted = raw
        .admit(&edges, &f.piece, &policy)
        .expect("construction repetitions remain explicit");
    assert_eq!(admitted.uses().len(), 6);
    assert_eq!(
        admitted
            .uses()
            .filter_map(|u| u.output())
            .map(|u| u.candidate().tag().get())
            .collect::<Vec<_>>(),
        vec![9, 9, 2, 2]
    );
    assert_eq!(admitted.uses().filter_map(|u| u.retained()).count(), 2);
}
