//! Both tuck and pleat share structural checks without claiming a physical fold shape.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
use sc_core::ontology::{
    CuttingSide, DeterministicIdGenerator, DirectedEdge, DirectedRange, DirectedRangeResolution,
    Direction, EdgeRange, EdgeRef, EntityId, FoldDefinition, FoldError, FoldReferenceRole,
    GeometricValidation, IdGenerator, IdentityLedger, IntakeAmount, IntakeValidation, LabelText,
    LocalTag, MaterialAssignment, Mirroring, Param, Piece, PieceDefinition, Pleat,
    ProfileBindingValidation, ProfileParameterRef, RangeIssue, Rational, Tuck,
};
use sc_units::Length;
fn t(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).unwrap()).unwrap()
}
struct Fixture {
    piece: Piece,
    ledger: IdentityLedger,
    ids: DeterministicIdGenerator,
    definition: FoldDefinition,
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
    Fixture {
        piece,
        ledger,
        ids,
        definition: FoldDefinition {
            id: EntityId::from_bits(100),
            intake: IntakeAmount::Explicit {
                parameter: EntityId::from_bits(101),
                value: Length::from_micrometres(40_000).unwrap(),
            },
            fold_lines: vec![range(1), range(2)],
            direction: range(3),
            closing_operation: EntityId::from_bits(102),
        },
    }
}

enum TestedFold {
    Tuck(Tuck),
    Pleat(Pleat),
}
impl TestedFold {
    fn definition(&self) -> &FoldDefinition {
        match self {
            Self::Tuck(v) => v.definition(),
            Self::Pleat(v) => v.definition(),
        }
    }
    fn evidence(
        &self,
        ledger: &IdentityLedger,
    ) -> Vec<(FoldReferenceRole, DirectedRangeResolution)> {
        match self {
            Self::Tuck(v) => v.reference_resolutions(ledger).collect(),
            Self::Pleat(v) => v.reference_resolutions(ledger).collect(),
        }
    }
}
impl Fixture {
    fn both(&self) -> [Result<TestedFold, FoldError>; 2] {
        [
            Tuck::new(self.definition.clone(), &self.piece, &self.ledger).map(TestedFold::Tuck),
            Pleat::new(self.definition.clone(), &self.piece, &self.ledger).map(TestedFold::Pleat),
        ]
    }
    fn set(&mut self, role: FoldReferenceRole, range: DirectedRange) {
        match role {
            FoldReferenceRole::FoldLine(index) => {
                *self.definition.fold_lines.get_mut(index).unwrap() = range
            }
            FoldReferenceRole::Direction => self.definition.direction = range,
        }
    }
}
#[test]
fn both_distinct_semantic_types_retain_intake_lines_direction_and_operation_without_physical_verdicts(
) {
    let f = fixture();
    let tuck = Tuck::new(f.definition.clone(), &f.piece, &f.ledger).unwrap();
    let pleat = Pleat::new(f.definition.clone(), &f.piece, &f.ledger).unwrap();
    assert_eq!(tuck.id(), f.definition.id);
    assert_eq!(pleat.id(), f.definition.id);
    assert_eq!(tuck.piece(), f.piece.id());
    assert_eq!(pleat.piece(), f.piece.id());
    assert_eq!(tuck.definition(), &f.definition);
    assert_eq!(pleat.definition(), &f.definition);
    assert_eq!(tuck.intake(), f.definition.intake);
    assert_eq!(pleat.intake(), f.definition.intake);
    assert_eq!(
        tuck.geometric_validation(),
        GeometricValidation::DeferredToG2
    );
    assert_eq!(
        pleat.geometric_validation(),
        GeometricValidation::DeferredToG2
    );
    assert_eq!(
        tuck.intake_validation(),
        IntakeValidation::DeferredToG2AndG3
    );
    assert_eq!(
        pleat.intake_validation(),
        IntakeValidation::DeferredToG2AndG3
    );
    assert_eq!(tuck.profile_binding_validation(), None);
    assert_eq!(pleat.profile_binding_validation(), None);
    let roles = vec![
        FoldReferenceRole::FoldLine(0),
        FoldReferenceRole::FoldLine(1),
        FoldReferenceRole::Direction,
    ];
    assert_eq!(
        tuck.references().map(|(role, _)| role).collect::<Vec<_>>(),
        roles
    );
    assert_eq!(
        pleat.references().map(|(role, _)| role).collect::<Vec<_>>(),
        roles
    );
}
#[test]
fn empty_or_duplicate_held_fold_intervals_are_refused_even_with_opposite_traversal() {
    let mut f = fixture();
    f.definition.fold_lines.clear();
    for result in f.both() {
        assert!(matches!(result, Err(FoldError::NoFoldLines)));
    }
    let line = f.definition.direction;
    f.definition.fold_lines = vec![
        line,
        DirectedRange {
            direction: Direction::Reversed,
            ..line
        },
    ];
    for result in f.both() {
        assert!(matches!(
            result,
            Err(FoldError::DuplicateFoldLine {
                first: 0,
                second: 1
            })
        ));
    }
}
#[test]
fn explicit_zero_and_symbolic_intake_are_retained_while_negative_intake_is_refused() {
    let mut f = fixture();
    let parameter = EntityId::from_bits(202);
    for intake in [
        IntakeAmount::Explicit {
            parameter,
            value: Length::ZERO,
        },
        IntakeAmount::Formula(parameter),
        IntakeAmount::Profile(ProfileParameterRef::new(parameter)),
    ] {
        f.definition.intake = intake;
        for result in f.both() {
            assert_eq!(result.unwrap().definition().intake, intake);
        }
        let expected = if matches!(intake, IntakeAmount::Profile(_)) {
            Some(ProfileBindingValidation::DeferredToG4)
        } else {
            None
        };
        assert_eq!(
            Tuck::new(f.definition.clone(), &f.piece, &f.ledger)
                .unwrap()
                .profile_binding_validation(),
            expected
        );
        assert_eq!(
            Pleat::new(f.definition.clone(), &f.piece, &f.ledger)
                .unwrap()
                .profile_binding_validation(),
            expected
        );
    }
    let negative = Length::from_micrometres(-1).unwrap();
    f.definition.intake = IntakeAmount::Explicit {
        parameter,
        value: negative,
    };
    for result in f.both() {
        assert!(matches!(result,Err(FoldError::NegativeIntake(value)) if value==negative));
    }
}
#[test]
fn every_line_and_direction_refuses_unknown_or_foreign_geometry_with_its_field_role() {
    for role in [
        FoldReferenceRole::FoldLine(0),
        FoldReferenceRole::FoldLine(1),
        FoldReferenceRole::Direction,
    ] {
        let mut f = fixture();
        let missing = EdgeRef::new(EntityId::from_bits(u128::MAX), LocalTag::FIRST);
        f.set(
            role,
            DirectedRange {
                range: EdgeRange::whole(missing),
                direction: Direction::Original,
            },
        );
        for result in f.both() {
            assert!(
                matches!(result,Err(FoldError::UnresolvedReference{role:actual,evidence}) if actual==role && evidence.repairs().next().unwrap().issue()==RangeIssue::UnknownSource)
            );
        }
        let foreign = *f
            .ledger
            .declare_edges(&mut f.ids, 1)
            .unwrap()
            .first()
            .unwrap();
        f.set(
            role,
            DirectedRange {
                range: EdgeRange::whole(foreign),
                direction: Direction::Original,
            },
        );
        for result in f.both() {
            assert!(
                matches!(result,Err(FoldError::OutsidePiece{role:actual,piece}) if actual==role && piece==f.piece.id())
            );
        }
    }
}
#[test]
fn full_positive_coverage_does_not_choose_a_split_endpoint_at_birth() {
    let mut f = fixture();
    let edge = f.definition.direction.range.edge();
    f.ledger.split(&mut f.ids, edge, t(1, 2)).unwrap();
    f.definition.direction.range = EdgeRange::new(edge, t(1, 2), Param::END).unwrap();
    for result in f.both() {
        assert!(
            matches!(result,Err(FoldError::UnresolvedReference{role:FoldReferenceRole::Direction,evidence}) if evidence.has_full_coverage() && evidence.start().resolved().is_none())
        );
    }
}
#[test]
fn foreign_middle_of_a_merged_fold_line_is_refused_despite_owned_endpoints_after_reversal() {
    let mut f = fixture();
    let left = f.definition.fold_lines.first().unwrap().range.edge();
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
    f.definition.fold_lines.first_mut().unwrap().range = EdgeRange::whole(merged);
    let evidence = f.ledger.resolve_range(EdgeRange::whole(merged));
    assert!(evidence.has_full_coverage());
    assert!(evidence.start().resolved().is_some());
    assert!(evidence.end().resolved().is_some());
    for result in f.both() {
        assert!(matches!(
            result,
            Err(FoldError::OutsidePiece {
                role: FoldReferenceRole::FoldLine(0),
                ..
            })
        ));
    }
    f.ledger.reverse(&mut f.ids, merged).unwrap();
    for result in f.both() {
        assert!(matches!(
            result,
            Err(FoldError::OutsidePiece {
                role: FoldReferenceRole::FoldLine(0),
                ..
            })
        ));
    }
}
#[test]
fn reversed_fold_queries_order_fragments_and_preserve_interior_repair_without_mutation() {
    let mut f = fixture();
    f.definition.fold_lines.first_mut().unwrap().direction = Direction::Reversed;
    let objects = f.both().map(Result::unwrap);
    let edge = f.definition.fold_lines.first().unwrap().range.edge();
    let first = f.ledger.split(&mut f.ids, edge, t(1, 3)).unwrap();
    let second = f.ledger.split(&mut f.ids, first.second(), t(1, 2)).unwrap();
    f.ledger.reverse(&mut f.ids, second.second()).unwrap();
    f.ledger.delete(&mut f.ids, second.first()).unwrap();
    for object in objects {
        let evidence = object.evidence(&f.ledger);
        let e = &evidence.first().unwrap().1;
        assert_eq!(e.portions().next().unwrap().direction, Direction::Original);
        assert_eq!(e.evidence().repairs().count(), 1);
        assert!(e.evidence().start().resolved().is_some());
        assert!(e.evidence().end().resolved().is_some());
        assert_eq!(object.definition(), &f.definition);
    }
}
#[test]
fn partial_distinct_intervals_and_shared_direction_are_explicit_intent_without_count_or_shape_claims(
) {
    let mut f = fixture();
    let line = *f.definition.fold_lines.first().unwrap();
    f.definition.fold_lines = vec![
        DirectedRange {
            range: EdgeRange::new(line.range.edge(), Param::START, t(1, 3)).unwrap(),
            ..line
        },
        DirectedRange {
            range: EdgeRange::new(line.range.edge(), t(2, 3), Param::END).unwrap(),
            ..line
        },
    ];
    f.definition.direction = *f.definition.fold_lines.first().unwrap();
    for result in f.both() {
        assert_eq!(result.unwrap().definition(), &f.definition);
    }
}
#[test]
fn replacement_input_cannot_mutate_either_validated_semantic_kind() {
    let f = fixture();
    let objects = f.both().map(Result::unwrap);
    let mut changed = f.definition.clone();
    changed.fold_lines.clear();
    changed.closing_operation = EntityId::from_bits(205);
    changed.direction.direction = Direction::Reversed;
    assert!(matches!(
        Tuck::new(changed.clone(), &f.piece, &f.ledger),
        Err(FoldError::NoFoldLines)
    ));
    assert!(matches!(
        Pleat::new(changed, &f.piece, &f.ledger),
        Err(FoldError::NoFoldLines)
    ));
    for object in objects {
        assert_eq!(object.definition(), &f.definition);
    }
}
