//! Structural dart contracts; no executed closing operation or conserved-length claim.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
use sc_core::ontology::{
    AnchorError, CuttingSide, Dart, DartDefinition, DartError, DartReferenceRole,
    DeterministicIdGenerator, DirectedEdge, DirectedRange, Direction, EdgeAnchor, EdgeRange,
    EdgeRef, EntityId, GeometricValidation, IdGenerator, IdentityLedger, IntakeAmount,
    IntakeValidation, LabelText, LocalTag, MaterialAssignment, Mirroring, Param, Piece,
    PieceDefinition, ProfileBindingValidation, ProfileParameterRef, RangeIssue, Rational,
    Resolution,
};
use sc_units::Length;
fn t(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).unwrap()).unwrap()
}
struct Fixture {
    piece: Piece,
    ledger: IdentityLedger,
    ids: DeterministicIdGenerator,
    definition: DartDefinition,
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
        definition: DartDefinition {
            id: EntityId::from_bits(100),
            intake: IntakeAmount::Explicit {
                parameter: EntityId::from_bits(101),
                value: Length::from_micrometres(40_000).unwrap(),
            },
            apex: EdgeAnchor {
                edge: range(1).range.edge(),
                param: Param::END,
            },
            first_leg: range(1),
            second_leg: range(2),
            direction: range(3),
            closing_operation: EntityId::from_bits(102),
        },
    }
}

impl Fixture {
    fn dart(&self) -> Result<Dart, DartError> {
        Dart::new(self.definition.clone(), &self.piece, &self.ledger)
    }
    fn set(&mut self, role: DartReferenceRole, range: DirectedRange) {
        match role {
            DartReferenceRole::FirstLeg => self.definition.first_leg = range,
            DartReferenceRole::SecondLeg => self.definition.second_leg = range,
            DartReferenceRole::Direction => self.definition.direction = range,
        }
    }
}
#[test]
fn interior_apex_owned_legs_intake_origin_and_operation_identity_are_immutable_content() {
    let f = fixture();
    let dart = f.dart().unwrap();
    assert_eq!(dart.id(), f.definition.id);
    assert_eq!(dart.piece(), f.piece.id());
    assert_eq!(dart.definition(), &f.definition);
    assert_eq!(dart.intake(), f.definition.intake);
    assert_ne!(
        dart.definition().apex.edge,
        f.piece.definition().boundary.first().unwrap().edge
    );
    assert_eq!(
        dart.references().map(|(role, _)| role).collect::<Vec<_>>(),
        vec![
            DartReferenceRole::FirstLeg,
            DartReferenceRole::SecondLeg,
            DartReferenceRole::Direction
        ]
    );
    assert_eq!(
        dart.intake_validation(),
        IntakeValidation::DeferredToG2AndG3
    );
    assert_eq!(
        dart.geometric_validation(),
        GeometricValidation::DeferredToG2
    );
    assert_eq!(dart.profile_binding_validation(), None);
}
#[test]
fn negative_intake_is_refused_zero_is_authored_and_symbolic_values_are_not_defaulted() {
    let mut f = fixture();
    let parameter = EntityId::from_bits(202);
    for value in [0, 1, 40_000] {
        f.definition.intake = IntakeAmount::Explicit {
            parameter,
            value: Length::from_micrometres(value).unwrap(),
        };
        assert_eq!(f.dart().unwrap().intake(), f.definition.intake);
    }
    let negative = Length::from_micrometres(-1).unwrap();
    f.definition.intake = IntakeAmount::Explicit {
        parameter,
        value: negative,
    };
    assert_eq!(f.dart(), Err(DartError::NegativeIntake(negative)));
    for intake in [
        IntakeAmount::Formula(parameter),
        IntakeAmount::Profile(ProfileParameterRef::new(parameter)),
    ] {
        f.definition.intake = intake;
        let dart = f.dart().unwrap();
        assert_eq!(dart.intake(), intake);
        assert_eq!(
            dart.profile_binding_validation(),
            if matches!(intake, IntakeAmount::Profile(_)) {
                Some(ProfileBindingValidation::DeferredToG4)
            } else {
                None
            }
        );
        assert_eq!(
            dart.intake_validation(),
            IntakeValidation::DeferredToG2AndG3
        );
    }
}
#[test]
fn identical_held_leg_intervals_are_refused_even_with_opposite_direction() {
    let mut f = fixture();
    f.definition.second_leg = f.definition.first_leg;
    assert_eq!(f.dart(), Err(DartError::IdenticalLegs));
    f.definition.second_leg.direction = Direction::Reversed;
    assert_eq!(f.dart(), Err(DartError::IdenticalLegs));
}
#[test]
fn apex_unknown_foreign_and_post_split_choice_are_explicit() {
    let mut f = fixture();
    let missing = EdgeRef::new(EntityId::from_bits(u128::MAX), LocalTag::FIRST);
    f.definition.apex.edge = missing;
    assert_eq!(
        f.dart(),
        Err(DartError::Apex(AnchorError::MissingEdge { edge: missing }))
    );
    let foreign = *f
        .ledger
        .declare_edges(&mut f.ids, 1)
        .unwrap()
        .first()
        .unwrap();
    f.definition.apex.edge = foreign;
    assert!(matches!(
        f.dart(),
        Err(DartError::Apex(AnchorError::AnchorOutsidePiece { .. }))
    ));
    f.definition.apex.edge = f.definition.first_leg.range.edge();
    f.definition.apex.param = t(1, 2);
    let dart = f.dart().unwrap();
    f.ledger
        .split(&mut f.ids, f.definition.apex.edge, t(1, 2))
        .unwrap();
    assert!(matches!(
        dart.apex_resolution(&f.ledger),
        Resolution::SplitPoint { .. }
    ));
    assert_eq!(dart.definition(), &f.definition);
}
#[test]
fn each_directed_field_refuses_unknown_and_live_foreign_ranges() {
    for role in [
        DartReferenceRole::FirstLeg,
        DartReferenceRole::SecondLeg,
        DartReferenceRole::Direction,
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
        assert!(
            matches!(f.dart(),Err(DartError::UnresolvedReference{role:actual,evidence}) if actual==role && evidence.repairs().next().unwrap().issue()==RangeIssue::UnknownSource)
        );
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
        assert_eq!(
            f.dart(),
            Err(DartError::OutsidePiece {
                role,
                piece: f.piece.id()
            })
        );
    }
}
#[test]
fn a_split_endpoint_choice_is_not_inferred_from_positive_interval_coverage() {
    let mut f = fixture();
    let edge = f.definition.direction.range.edge();
    f.ledger.split(&mut f.ids, edge, t(1, 2)).unwrap();
    f.definition.direction.range = EdgeRange::new(edge, t(1, 2), Param::END).unwrap();
    assert!(
        matches!(f.dart(),Err(DartError::UnresolvedReference{role:DartReferenceRole::Direction,evidence}) if evidence.has_full_coverage() && evidence.start().resolved().is_none())
    );
}
#[test]
fn foreign_middle_of_merged_direction_is_refused_despite_owned_endpoints_after_reversal() {
    let mut f = fixture();
    let left = f.definition.direction.range.edge();
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
    f.definition.direction.range = EdgeRange::whole(merged);
    let evidence = f.ledger.resolve_range(f.definition.direction.range);
    assert!(evidence.has_full_coverage());
    assert!(evidence.start().resolved().is_some());
    assert!(evidence.end().resolved().is_some());
    assert_eq!(
        f.dart(),
        Err(DartError::OutsidePiece {
            role: DartReferenceRole::Direction,
            piece: f.piece.id()
        })
    );
    f.ledger.reverse(&mut f.ids, merged).unwrap();
    assert!(matches!(
        f.dart(),
        Err(DartError::OutsidePiece {
            role: DartReferenceRole::Direction,
            ..
        })
    ));
}
#[test]
fn directed_queries_preserve_reversal_and_interior_deletion_evidence() {
    let mut f = fixture();
    f.definition.second_leg.direction = Direction::Reversed;
    let dart = f.dart().unwrap();
    let edge = f.definition.second_leg.range.edge();
    let first = f.ledger.split(&mut f.ids, edge, t(1, 3)).unwrap();
    let second = f.ledger.split(&mut f.ids, first.second(), t(1, 2)).unwrap();
    f.ledger.reverse(&mut f.ids, second.second()).unwrap();
    f.ledger.delete(&mut f.ids, second.first()).unwrap();
    let (_, e) = dart
        .reference_resolutions(&f.ledger)
        .find(|(role, _)| *role == DartReferenceRole::SecondLeg)
        .unwrap();
    assert!(e.evidence().start().resolved().is_some());
    assert!(e.evidence().end().resolved().is_some());
    assert_eq!(e.evidence().repairs().count(), 1);
    assert_eq!(e.portions().next().unwrap().direction, Direction::Original);
    assert_eq!(dart.definition(), &f.definition);
}
#[test]
fn editable_input_and_replacement_cannot_change_a_validated_dart() {
    let f = fixture();
    let dart = f.dart().unwrap();
    let mut edited = dart.definition().clone();
    edited.direction.direction = Direction::Reversed;
    edited.closing_operation = EntityId::from_bits(205);
    edited.apex.param = Param::START;
    let replacement = Dart::new(edited, &f.piece, &f.ledger).unwrap();
    assert_ne!(replacement, dart);
    assert_eq!(dart.definition(), &f.definition);
    assert_eq!(
        replacement.intake_validation(),
        IntakeValidation::DeferredToG2AndG3
    );
}
