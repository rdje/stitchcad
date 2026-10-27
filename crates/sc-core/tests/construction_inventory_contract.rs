//! Exact distinct-family ownership/order, before aggregate current-source validation.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
use sc_core::ontology::*;
use sc_units::Length;

fn id(value: u128) -> EntityId {
    EntityId::from_bits(value)
}

struct Fixture {
    pieces: Vec<Piece>,
    ledger: IdentityLedger,
    ids: DeterministicIdGenerator,
    plan: CutPlan,
    graph: SewingGraph,
}
fn fixture() -> Fixture {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let edges = ledger.declare_edges(&mut ids, 4).unwrap();
    let pieces = (0usize..2)
        .map(|index| {
            let edge = |offset| DirectedEdge {
                edge: *edges.get(index + offset).unwrap(),
                direction: Direction::Original,
            };
            Piece::new(
                PieceDefinition {
                    id: id(100 + u128::try_from(index).unwrap()),
                    boundary: vec![edge(0)],
                    holes: vec![],
                    construction_lines: vec![edge(2)],
                    quantity: 1,
                    mirroring: Mirroring::Single,
                    cut_on_fold: false,
                    fold_edges: vec![],
                    cutting_side: CuttingSide::Face,
                    material: MaterialAssignment::Unresolved("selection pending".into()),
                    layer_index: 0,
                    label: LabelText {
                        name: format!("source {index}"),
                        size: "M".into(),
                        fabric: "woven".into(),
                        colorway: "plain".into(),
                    },
                },
                &ledger,
            )
            .unwrap()
        })
        .collect::<Vec<_>>();
    let plan = CutPlan::new(
        vec![
            CutCopyDefinition {
                id: id(200),
                piece: id(100),
                orientation: CopyOrientation::Authored,
            },
            CutCopyDefinition {
                id: id(201),
                piece: id(101),
                orientation: CopyOrientation::Authored,
            },
        ],
        &pieces,
    )
    .unwrap();
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: id(300),
            spans: vec![SeamSpanDefinition {
                id: id(301),
                a: SeamSide {
                    copy: id(200),
                    range: EdgeRange::whole(*edges.first().unwrap()),
                },
                b: SeamSide {
                    copy: id(201),
                    range: EdgeRange::whole(*edges.get(1).unwrap()),
                },
                direction: SeamDirection::Opposite,
                ease: DeclaredEase {
                    amount: EaseAmount::Explicit(Length::ZERO),
                    distribution: EaseDistribution::Uniform,
                },
                stops: vec![],
            }],
        },
        &plan,
        &pieces,
        &[],
        &ledger,
    )
    .unwrap();
    Fixture {
        pieces,
        ledger,
        ids,
        plan,
        graph,
    }
}
impl Fixture {
    fn inventory(
        &self,
        dart_ids: &[u128],
        others: [u128; 5],
    ) -> Result<ConstructionInventory, ConstructionInventoryError> {
        let [tuck, pleat, gather, grain, allowance] = others;
        let piece = self.pieces.first().unwrap();
        let range = |edge| DirectedRange {
            range: EdgeRange::whole(edge),
            direction: Direction::Original,
        };
        let boundary = piece.definition().boundary.first().unwrap().edge;
        let internal = piece.definition().construction_lines.first().unwrap().edge;
        // These are declared uses, deliberately shared; the inventory owns only construction ids.
        let intake = IntakeAmount::Explicit {
            parameter: id(777),
            value: Length::ZERO,
        };
        let fold = |identity| FoldDefinition {
            id: id(identity),
            intake,
            fold_lines: vec![range(internal)],
            direction: range(boundary),
            closing_operation: id(778),
        };
        let darts = dart_ids
            .iter()
            .map(|identity| {
                Dart::new(
                    DartDefinition {
                        id: id(*identity),
                        intake,
                        apex: EdgeAnchor {
                            edge: internal,
                            param: Param::END,
                        },
                        first_leg: range(internal),
                        second_leg: range(boundary),
                        direction: range(internal),
                        closing_operation: id(778),
                    },
                    piece,
                    &self.ledger,
                )
                .unwrap()
            })
            .collect();
        ConstructionInventory::new(
            darts,
            vec![Tuck::new(fold(tuck), piece, &self.ledger).unwrap()],
            vec![Pleat::new(fold(pleat), piece, &self.ledger).unwrap()],
            vec![Gather::new(
                GatherDefinition {
                    id: id(gather),
                    graph: self.graph.id(),
                    span: id(301),
                    side: SeamSideId::A,
                    copy: id(200),
                    direction: range(internal),
                    closing_operation: id(778),
                },
                piece,
                &self.plan,
                &self.graph,
                &self.ledger,
            )
            .unwrap()],
            vec![Grainline::new(
                GrainlineDefinition {
                    id: id(grain),
                    arrow: range(internal),
                    alignment: GrainAlignment::ParallelTo(range(boundary)),
                    stripe: None,
                    plaid: None,
                },
                piece,
                &self.ledger,
            )
            .unwrap()],
            vec![SeamAllowance::new(
                SeamAllowanceDefinition {
                    id: id(allowance),
                    edge: boundary,
                    width: AllowanceWidth::Formula(id(777)),
                    corner: CornerTreatment::Envelope,
                    inclusion: ProfileParameterRef::new(id(779)),
                },
                piece,
                &self.ledger,
            )
            .unwrap()],
        )
    }
}

#[test]
fn authored_orders_and_distinct_typed_borrows_are_exact_without_numeric_sorting() {
    let f = fixture();
    let inventory = f
        .inventory(&[900, 100], [0, 1100, u128::MAX, 700, 800])
        .unwrap();
    let expected = [
        (ConstructionKind::Dart, 0, id(900)),
        (ConstructionKind::Dart, 1, id(100)),
        (ConstructionKind::Tuck, 0, id(0)),
        (ConstructionKind::Pleat, 0, id(1100)),
        (ConstructionKind::Gather, 0, id(u128::MAX)),
        (ConstructionKind::Grainline, 0, id(700)),
        (ConstructionKind::SeamAllowance, 0, id(800)),
    ];
    assert_eq!(
        inventory
            .entries()
            .map(|(position, value)| (position.kind, position.index, value.id()))
            .collect::<Vec<_>>(),
        expected
    );
    for (position, value) in inventory.entries() {
        assert_eq!(position.kind, value.kind());
        assert_eq!(value.piece(), id(100));
        match value {
            ConstructionRef::Dart(value) => assert!(std::ptr::eq(
                value,
                inventory.darts().get(position.index).unwrap()
            )),
            ConstructionRef::Tuck(value) => assert!(std::ptr::eq(
                value,
                inventory.tucks().get(position.index).unwrap()
            )),
            ConstructionRef::Pleat(value) => assert!(std::ptr::eq(
                value,
                inventory.pleats().get(position.index).unwrap()
            )),
            ConstructionRef::Gather(value) => assert!(std::ptr::eq(
                value,
                inventory.gathers().get(position.index).unwrap()
            )),
            ConstructionRef::Grainline(value) => assert!(std::ptr::eq(
                value,
                inventory.grainlines().get(position.index).unwrap()
            )),
            ConstructionRef::SeamAllowance(value) => assert!(std::ptr::eq(
                value,
                inventory.allowances().get(position.index).unwrap()
            )),
        }
    }
    let debug = format!("{inventory:?}");
    assert!(debug.contains("dart_count: 2") && debug.contains("allowance_count: 1"));
    assert!(
        !debug.contains("EntityId")
            && !debug.contains("Definition")
            && !debug.contains("ProfileParameterRef")
    );
}

#[test]
fn first_duplicate_locations_are_preserved_within_and_across_every_family() {
    let f = fixture();
    let position = |kind, index| ConstructionInventoryPosition { kind, index };
    assert_eq!(
        f.inventory(&[900, 100, 900], [500, 501, 502, 503, 504]),
        Err(ConstructionInventoryError::DuplicateIdentity {
            identity: id(900),
            first: position(ConstructionKind::Dart, 0),
            duplicate: position(ConstructionKind::Dart, 2),
        })
    );
    for (index, kind) in [
        ConstructionKind::Tuck,
        ConstructionKind::Pleat,
        ConstructionKind::Gather,
        ConstructionKind::Grainline,
        ConstructionKind::SeamAllowance,
    ]
    .into_iter()
    .enumerate()
    {
        let mut others = [500, 501, 502, 503, 504];
        *others.get_mut(index).unwrap() = 900;
        assert_eq!(
            f.inventory(&[900], others),
            Err(ConstructionInventoryError::DuplicateIdentity {
                identity: id(900),
                first: position(ConstructionKind::Dart, 0),
                duplicate: position(kind, 0),
            })
        );
    }
    assert_eq!(
        f.inventory(&[], [600, 600, 602, 603, 604]),
        Err(ConstructionInventoryError::DuplicateIdentity {
            identity: id(600),
            first: position(ConstructionKind::Tuck, 0),
            duplicate: position(ConstructionKind::Pleat, 0),
        })
    );
}

#[test]
fn references_remain_uses_and_explicit_empty_families_claim_no_readiness() {
    let f = fixture();
    // A construction id may equal a parameter/operation/profile use at this ownership-only stage.
    let inventory = f.inventory(&[777], [778, 779, 400, 401, 402]).unwrap();
    assert_eq!(inventory.entries().count(), 6);
    let empty = ConstructionInventory::new(vec![], vec![], vec![], vec![], vec![], vec![]).unwrap();
    assert_eq!(empty.entries().count(), 0);
    assert!(empty.darts().is_empty() && empty.tucks().is_empty() && empty.pleats().is_empty());
    assert!(
        empty.gathers().is_empty()
            && empty.grainlines().is_empty()
            && empty.allowances().is_empty()
    );
}

#[test]
fn owning_held_damage_never_rewrites_or_certifies_the_original_sources() {
    let mut f = fixture();
    let original = f.inventory(&[400], [401, 402, 403, 404, 405]).unwrap();
    let edge = original.darts().first().unwrap().definition().apex.edge;
    f.ledger.delete(&mut f.ids, edge).unwrap();
    assert!(original
        .darts()
        .first()
        .unwrap()
        .validate_current(f.pieces.first().unwrap(), &f.ledger)
        .is_err());
    let retained = ConstructionInventory::new(
        original.darts().to_vec(),
        original.tucks().to_vec(),
        original.pleats().to_vec(),
        original.gathers().to_vec(),
        original.grainlines().to_vec(),
        original.allowances().to_vec(),
    )
    .unwrap();
    assert_eq!(retained, original);
    assert_eq!(
        retained.darts().first().unwrap().definition().apex.edge,
        edge
    );
}
