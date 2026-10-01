//! Served layer intent with explicit lining execution scope.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
use sc_core::ontology::{
    CuttingSide, DeterministicIdGenerator, DirectedEdge, DirectedRange, DirectedRangeResolution,
    Direction, EdgeRange, EdgeRef, EntityId, Facing, GeometricValidation, IdGenerator,
    IdentityLedger, Interfacing, LabelText, LayerDefinition, LayerEnvelopeError, LayerError,
    LayerKind, LayerOffsetRelationship, Lining, LocalTag, MaterialAssignment, Mirroring, Param,
    Piece, PieceDefinition, RangeIssue, Rational,
};
use sc_units::Length;
fn t(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).unwrap()).unwrap()
}
struct Fixture {
    piece: Piece,
    ledger: IdentityLedger,
    ids: DeterministicIdGenerator,
    definition: LayerDefinition,
}
fn fixture() -> Fixture {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let edges = ledger.declare_edges(&mut ids, 4).unwrap();
    let directed = |index| DirectedEdge {
        edge: *edges.get(index).unwrap(),
        direction: Direction::Original,
    };
    let piece = Piece::new(
        PieceDefinition {
            id: ids.next_id(),
            boundary: vec![directed(0)],
            holes: vec![],
            construction_lines: vec![directed(1), directed(2), directed(3)],
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
    let range = |index| DirectedRange {
        range: EdgeRange::whole(*edges.get(index).unwrap()),
        direction: Direction::Original,
    };
    let served_piece = piece.id();
    Fixture {
        piece,
        ledger,
        ids,
        definition: LayerDefinition {
            id: EntityId::from_bits(100),
            served_piece,
            offset: LayerOffsetRelationship {
                operation: EntityId::from_bits(101),
                sources: vec![range(1), range(2)],
            },
            material: MaterialAssignment::Unresolved("material evidence pending".into()),
        },
    }
}

enum TestedLayer {
    Facing(Facing),
    Lining(Lining),
    Interfacing(Interfacing),
}
impl TestedLayer {
    fn definition(&self) -> &LayerDefinition {
        match self {
            Self::Facing(v) => v.definition(),
            Self::Lining(v) => v.definition(),
            Self::Interfacing(v) => v.definition(),
        }
    }
    fn evidence(&self, ledger: &IdentityLedger) -> Vec<(usize, DirectedRangeResolution)> {
        match self {
            Self::Facing(v) => v.source_resolutions(ledger).collect(),
            Self::Lining(v) => v.source_resolutions(ledger).collect(),
            Self::Interfacing(v) => v.source_resolutions(ledger).collect(),
        }
    }
}
impl Fixture {
    fn all(&self) -> [Result<TestedLayer, LayerError>; 3] {
        [
            Facing::new(self.definition.clone(), &self.piece, &self.ledger)
                .map(TestedLayer::Facing),
            Lining::new(self.definition.clone(), &self.piece, &self.ledger)
                .map(TestedLayer::Lining),
            Interfacing::new(self.definition.clone(), &self.piece, &self.ledger)
                .map(TestedLayer::Interfacing),
        ]
    }
}
#[test]
fn all_distinct_layer_kinds_retain_served_piece_offset_operation_sources_and_material() {
    let f = fixture();
    let facing = Facing::new(f.definition.clone(), &f.piece, &f.ledger).unwrap();
    let lining = Lining::new(f.definition.clone(), &f.piece, &f.ledger).unwrap();
    let interfacing = Interfacing::new(f.definition.clone(), &f.piece, &f.ledger).unwrap();
    assert_eq!(facing.kind(), LayerKind::Facing);
    assert_eq!(lining.kind(), LayerKind::Lining);
    assert_eq!(interfacing.kind(), LayerKind::Interfacing);
    assert_eq!(facing.id(), f.definition.id);
    assert_eq!(lining.id(), f.definition.id);
    assert_eq!(interfacing.id(), f.definition.id);
    assert_eq!(facing.served_piece(), f.piece.id());
    assert_eq!(lining.served_piece(), f.piece.id());
    assert_eq!(interfacing.served_piece(), f.piece.id());
    assert_eq!(
        facing.geometric_validation(),
        GeometricValidation::DeferredToG2
    );
    assert_eq!(
        lining.geometric_validation(),
        GeometricValidation::DeferredToG2
    );
    assert_eq!(
        interfacing.geometric_validation(),
        GeometricValidation::DeferredToG2
    );
    for result in f.all() {
        assert_eq!(result.unwrap().definition(), &f.definition);
    }
    assert_eq!(facing.require_in_scope(), Ok(()));
    assert_eq!(interfacing.require_in_scope(), Ok(()));
}
#[test]
fn modelled_lining_refuses_v1_execution_with_named_diagnostic_piece_and_proving_gate() {
    let mut f = fixture();
    let lining = Lining::new(f.definition.clone(), &f.piece, &f.ledger).unwrap();
    let expected = LayerEnvelopeError::LiningDeferred {
        served_piece: f.piece.id(),
    };
    assert_eq!(lining.require_in_scope(), Err(expected));
    assert_eq!(expected.diagnostic(), "env_lining");
    assert_eq!(expected.proving_gate(), "G7");
    assert!(expected.to_string().contains(&f.piece.id().to_string()));
    assert!(expected.to_string().contains("env_lining"));
    f.definition.offset.sources.clear();
    assert_eq!(
        Lining::new(f.definition, &f.piece, &f.ledger),
        Err(LayerError::NoOffsetSources)
    );
}
#[test]
fn shared_material_reason_invariant_rejects_blank_without_defaulting_an_assignment() {
    let mut f = fixture();
    for reason in ["", " \t\n"] {
        f.definition.material = MaterialAssignment::Unresolved(reason.into());
        for result in f.all() {
            assert!(matches!(result, Err(LayerError::UnexplainedMaterial)));
        }
    }
    for material in [
        MaterialAssignment::Unresolved("testing evidence required".into()),
        MaterialAssignment::Assigned(EntityId::from_bits(999)),
    ] {
        f.definition.material = material.clone();
        for result in f.all() {
            assert_eq!(result.unwrap().definition().material, material);
        }
    }
}
#[test]
fn offset_sources_are_required_and_exact_duplicates_ignore_authored_traversal() {
    let mut f = fixture();
    f.definition.offset.sources.clear();
    for result in f.all() {
        assert!(matches!(result, Err(LayerError::NoOffsetSources)));
    }
    let line = DirectedRange {
        range: EdgeRange::whole(f.piece.definition().boundary.first().unwrap().edge),
        direction: Direction::Original,
    };
    f.definition.offset.sources = vec![
        line,
        DirectedRange {
            direction: Direction::Reversed,
            ..line
        },
    ];
    for result in f.all() {
        assert!(matches!(
            result,
            Err(LayerError::DuplicateOffsetSource {
                first: 0,
                second: 1
            })
        ));
    }
}
#[test]
fn wrong_served_piece_and_each_unknown_or_foreign_source_name_their_failed_scope() {
    let mut f = fixture();
    f.definition.served_piece = EntityId::from_bits(999);
    for result in f.all() {
        assert!(
            matches!(result,Err(LayerError::WrongServedPiece{held,provided}) if held==EntityId::from_bits(999) && provided==f.piece.id())
        );
    }
    for index in [0, 1] {
        let mut f = fixture();
        let missing = EdgeRef::new(EntityId::from_bits(u128::MAX), LocalTag::FIRST);
        f.definition.offset.sources.get_mut(index).unwrap().range = EdgeRange::whole(missing);
        for result in f.all() {
            assert!(
                matches!(result,Err(LayerError::UnresolvedSource{index:actual,evidence}) if actual==index && evidence.repairs().next().unwrap().issue()==RangeIssue::UnknownSource)
            );
        }
        let foreign = *f
            .ledger
            .declare_edges(&mut f.ids, 1)
            .unwrap()
            .first()
            .unwrap();
        f.definition.offset.sources.get_mut(index).unwrap().range = EdgeRange::whole(foreign);
        for result in f.all() {
            assert!(
                matches!(result,Err(LayerError::SourceOutsidePiece{index:actual,piece}) if actual==index && piece==f.piece.id())
            );
        }
    }
}
#[test]
fn complete_positive_intervals_do_not_choose_a_split_endpoint_at_birth() {
    let mut f = fixture();
    let edge = f.definition.offset.sources.first().unwrap().range.edge();
    f.ledger.split(&mut f.ids, edge, t(1, 2)).unwrap();
    f.definition.offset.sources.first_mut().unwrap().range =
        EdgeRange::new(edge, t(1, 2), Param::END).unwrap();
    for result in f.all() {
        assert!(
            matches!(result,Err(LayerError::UnresolvedSource{index:0,evidence}) if evidence.has_full_coverage() && evidence.start().resolved().is_none())
        );
    }
}
#[test]
fn foreign_middle_of_merged_offset_source_is_refused_despite_owned_ends_after_reversal() {
    let mut f = fixture();
    let left = f.definition.offset.sources.first().unwrap().range.edge();
    let right = f.piece.definition().boundary.first().unwrap().edge;
    let foreign = *f
        .ledger
        .declare_edges(&mut f.ids, 1)
        .unwrap()
        .first()
        .unwrap();
    let unit = Length::from_micrometres(1).unwrap();
    let first = f
        .ledger
        .merge(&mut f.ids, left, unit, foreign, unit)
        .unwrap();
    let merged = f
        .ledger
        .merge(
            &mut f.ids,
            first,
            Length::from_micrometres(2).unwrap(),
            right,
            unit,
        )
        .unwrap();
    f.definition.offset.sources.first_mut().unwrap().range = EdgeRange::whole(merged);
    let evidence = f.ledger.resolve_range(EdgeRange::whole(merged));
    assert!(evidence.has_full_coverage());
    assert!(evidence.start().resolved().is_some());
    assert!(evidence.end().resolved().is_some());
    for result in f.all() {
        assert!(matches!(
            result,
            Err(LayerError::SourceOutsidePiece { index: 0, .. })
        ));
    }
    f.ledger.reverse(&mut f.ids, merged).unwrap();
    for result in f.all() {
        assert!(matches!(
            result,
            Err(LayerError::SourceOutsidePiece { index: 0, .. })
        ));
    }
}
#[test]
fn directed_source_queries_keep_reversal_and_lost_interiors_in_authored_order() {
    let mut f = fixture();
    f.definition.offset.sources.first_mut().unwrap().direction = Direction::Reversed;
    let objects = f.all().map(Result::unwrap);
    let edge = f.definition.offset.sources.first().unwrap().range.edge();
    let first = f.ledger.split(&mut f.ids, edge, t(1, 3)).unwrap();
    let second = f.ledger.split(&mut f.ids, first.second(), t(1, 2)).unwrap();
    f.ledger.reverse(&mut f.ids, second.second()).unwrap();
    f.ledger.delete(&mut f.ids, second.first()).unwrap();
    for object in objects {
        let all = object.evidence(&f.ledger);
        assert_eq!(all.first().unwrap().0, 0);
        assert_eq!(all.get(1).unwrap().0, 1);
        let e = &all.first().unwrap().1;
        assert_eq!(e.portions().next().unwrap().direction, Direction::Original);
        assert_eq!(e.evidence().repairs().count(), 1);
        assert!(e.evidence().start().resolved().is_some());
        assert!(e.evidence().end().resolved().is_some());
        assert_eq!(object.definition(), &f.definition);
    }
}
#[test]
fn editing_offset_or_material_input_cannot_mutate_any_validated_layer_kind() {
    let f = fixture();
    let objects = f.all().map(Result::unwrap);
    let mut changed = f.definition.clone();
    changed.offset.operation = EntityId::from_bits(205);
    changed.material = MaterialAssignment::Assigned(EntityId::from_bits(206));
    changed.offset.sources.first_mut().unwrap().direction = Direction::Reversed;
    let replacement = Facing::new(changed, &f.piece, &f.ledger).unwrap();
    assert_ne!(replacement.definition(), &f.definition);
    for object in objects {
        assert_eq!(object.definition(), &f.definition);
    }
}
