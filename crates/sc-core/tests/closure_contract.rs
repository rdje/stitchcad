//! Closure instances: canonical physical placements, derived counts and explicit scope.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
use sc_core::ontology::{
    ButtonholeDerivation, ButtonholeValidation, Closure, ClosureDefinition, ClosureEnvelopeError,
    ClosureError, ClosureInstance, ClosureInventory, ClosureInventoryError,
    ClosureInventoryPosition, ClosureKind, ClosureLength, ClosurePlacementRole, CopyOrientation,
    CutCopyDefinition, CutPlan, CuttingSide, DeterministicIdGenerator, DirectedEdge, DirectedRange,
    Direction, EdgeAnchor, EdgeRange, EntityId, GeometricValidation, IdGenerator, IdentityLedger,
    LabelText, MaterialAssignment, Mirroring, NotionPlacement, NotionPlacementDefinition,
    NotionPlacementError, NotionPlacementInventory, NotionPlacementInventoryError, NotionSize,
    Param, Piece, PieceDefinition, ProfileBindingValidation, ProfileParameterRef, Rational,
};
use sc_units::{Count, Length};
fn t(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).unwrap()).unwrap()
}
struct Fixture {
    piece: Piece,
    ledger: IdentityLedger,
    ids: DeterministicIdGenerator,
    direction: DirectedRange,
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
            id: ids.next_id().expect("test ID capacity"),
            boundary: vec![directed(0)],
            holes: vec![],
            construction_lines: vec![directed(1), directed(2), directed(3)],
            quantity: 2,
            mirroring: Mirroring::MirroredPairs,
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
        direction: range(1),
    }
}

impl Fixture {
    fn plan(&self) -> CutPlan {
        CutPlan::new(
            vec![
                CutCopyDefinition {
                    id: EntityId::from_bits(600),
                    piece: self.piece.id(),
                    orientation: CopyOrientation::Authored,
                },
                CutCopyDefinition {
                    id: EntityId::from_bits(601),
                    piece: self.piece.id(),
                    orientation: CopyOrientation::Reflected,
                },
            ],
            std::slice::from_ref(&self.piece),
        )
        .unwrap()
    }
    fn placement_input(&self) -> NotionPlacementDefinition {
        NotionPlacementDefinition {
            id: EntityId::from_bits(700),
            copy: EntityId::from_bits(600),
            anchor: EdgeAnchor {
                edge: self.piece.definition().boundary.first().unwrap().edge,
                param: t(1, 4),
            },
            direction: self.direction,
        }
    }
    fn placement(
        &self,
        input: NotionPlacementDefinition,
    ) -> Result<NotionPlacement, NotionPlacementError> {
        NotionPlacement::new(input, &self.plan(), &self.piece, &self.ledger)
    }
}
impl Fixture {
    fn placements(&self) -> Vec<NotionPlacement> {
        let first = self.placement_input();
        let mut second = first;
        second.id = EntityId::from_bits(701);
        second.copy = EntityId::from_bits(601);
        second.anchor.param = t(3, 4);
        second.direction.direction = Direction::Reversed;
        vec![
            self.placement(first).unwrap(),
            self.placement(second).unwrap(),
        ]
    }
    fn closure_input(&self) -> ClosureDefinition {
        ClosureDefinition {
            id: EntityId::from_bits(800),
            kind: ClosureKind::CentredZipper {
                length: ClosureLength::Explicit {
                    parameter: EntityId::from_bits(801),
                    value: Length::from_micrometres(180_000).unwrap(),
                },
            },
            instances: vec![ClosureInstance {
                id: EntityId::from_bits(802),
                first: EntityId::from_bits(700),
                second: EntityId::from_bits(701),
            }],
        }
    }
    fn closure(
        &self,
        input: ClosureDefinition,
        placements: &[NotionPlacement],
    ) -> Result<Closure, ClosureError> {
        Closure::new(
            input,
            placements,
            &self.plan(),
            std::slice::from_ref(&self.piece),
            &self.ledger,
        )
    }
}
#[test]
fn fixture_shaped_zipper_and_hook_bar_keep_distinct_kinds_sizes_and_derived_counts() {
    let f = fixture();
    let placements = f.placements();
    let input = f.closure_input();
    let zipper = f.closure(input.clone(), &placements).unwrap();
    assert_eq!(zipper.id(), input.id);
    assert_eq!(zipper.definition(), &input);
    assert_eq!(zipper.count(), Count::new(1));
    assert_eq!(
        zipper.geometric_validation(),
        GeometricValidation::DeferredToG2
    );
    assert_eq!(zipper.profile_binding_validation(), None);
    let hook = NotionSize::Declaration(EntityId::from_bits(803));
    let bar = NotionSize::Profile(ProfileParameterRef::new(EntityId::from_bits(804)));
    let mut input = input;
    input.kind = ClosureKind::HookAndBar { hook, bar };
    let closure = f.closure(input.clone(), &placements).unwrap();
    assert_eq!(closure.definition(), &input);
    assert_eq!(closure.count(), Count::new(1));
    assert_eq!(
        closure.profile_binding_validation(),
        Some(ProfileBindingValidation::DeferredToG4)
    );
}
#[test]
fn positive_zipper_length_origins_never_default_or_accept_zero_as_a_real_zipper() {
    let f = fixture();
    let placements = f.placements();
    let mut input = f.closure_input();
    let parameter = EntityId::from_bits(805);
    for micrometres in [-1, 0] {
        let value = Length::from_micrometres(micrometres).unwrap();
        input.kind = ClosureKind::CentredZipper {
            length: ClosureLength::Explicit { parameter, value },
        };
        assert_eq!(
            f.closure(input.clone(), &placements),
            Err(ClosureError::NonPositiveLength(value))
        );
    }
    for length in [
        ClosureLength::Explicit {
            parameter,
            value: Length::from_micrometres(1).unwrap(),
        },
        ClosureLength::Formula(parameter),
        ClosureLength::Profile(ProfileParameterRef::new(parameter)),
    ] {
        input.kind = ClosureKind::CentredZipper { length };
        let closure = f.closure(input.clone(), &placements).unwrap();
        assert_eq!(closure.definition().kind, input.kind);
        assert_eq!(
            closure.profile_binding_validation(),
            if matches!(length, ClosureLength::Profile(_)) {
                Some(ProfileBindingValidation::DeferredToG4)
            } else {
                None
            }
        );
    }
    for (hook, bar) in [
        (
            NotionSize::Declaration(parameter),
            NotionSize::Declaration(parameter),
        ),
        (
            NotionSize::Profile(ProfileParameterRef::new(parameter)),
            NotionSize::Declaration(parameter),
        ),
    ] {
        input.kind = ClosureKind::HookAndBar { hook, bar };
        let closure = f.closure(input.clone(), &placements).unwrap();
        assert_eq!(closure.definition().kind, input.kind);
        assert_eq!(
            closure.profile_binding_validation(),
            if matches!(hook, NotionSize::Profile(_)) {
                Some(ProfileBindingValidation::DeferredToG4)
            } else {
                None
            }
        );
    }
}
#[test]
fn stable_instances_survive_reordering_and_count_derives_from_distinct_physical_pairs() {
    let f = fixture();
    let mut placements = f.placements();
    let mut input = f.closure_input();
    for id in [702, 703] {
        let mut p = f.placement_input();
        p.id = EntityId::from_bits(id);
        p.anchor.param = t(1, 3);
        if id == 703 {
            p.copy = EntityId::from_bits(601);
            p.anchor.param = t(2, 3)
        };
        placements.push(f.placement(p).unwrap());
    }
    let second = ClosureInstance {
        id: EntityId::from_bits(806),
        first: EntityId::from_bits(702),
        second: EntityId::from_bits(703),
    };
    input.instances.push(second);
    let first = f.closure(input.clone(), &placements).unwrap();
    input.instances.reverse();
    let reordered = f.closure(input, &placements).unwrap();
    assert_eq!(first.count(), Count::new(2));
    assert_eq!(reordered.count(), first.count());
    for role in [ClosurePlacementRole::First, ClosurePlacementRole::Second] {
        let a = first
            .placement(
                second.id,
                role,
                &placements,
                &f.plan(),
                std::slice::from_ref(&f.piece),
                &f.ledger,
            )
            .unwrap();
        let b = reordered
            .placement(
                second.id,
                role,
                &placements,
                &f.plan(),
                std::slice::from_ref(&f.piece),
                &f.ledger,
            )
            .unwrap();
        assert!(core::ptr::eq(a, b));
        assert_eq!(a.id(), second.placement(role));
    }
}
#[test]
fn empty_duplicate_instances_and_reused_component_placements_are_refused() {
    let f = fixture();
    let placements = f.placements();
    let mut input = f.closure_input();
    input.instances.clear();
    assert_eq!(
        f.closure(input, &placements),
        Err(ClosureError::NoInstances)
    );
    let mut input = f.closure_input();
    let instance = *input.instances.first().unwrap();
    input.instances.push(instance);
    assert_eq!(
        f.closure(input, &placements),
        Err(ClosureError::DuplicateInstance(instance.id))
    );
    let mut input = f.closure_input();
    input.instances.first_mut().unwrap().second = instance.first;
    assert_eq!(
        f.closure(input, &placements),
        Err(ClosureError::ReusedPlacement(instance.first))
    );
    let mut input = f.closure_input();
    input.instances.push(ClosureInstance {
        id: EntityId::from_bits(807),
        ..instance
    });
    assert_eq!(
        f.closure(input, &placements),
        Err(ClosureError::ReusedPlacement(instance.first))
    );
}
#[test]
fn ambiguous_context_ids_are_refused_instead_of_selecting_a_first_match() {
    let f = fixture();
    let mut placements = f.placements();
    let input = f.closure_input();
    let first = placements.first().unwrap().clone();
    placements.push(first.clone());
    assert_eq!(
        f.closure(input.clone(), &placements),
        Err(ClosureError::DuplicatePlacement(first.id()))
    );
    placements.pop();
    assert_eq!(
        Closure::new(
            input,
            &placements,
            &f.plan(),
            &[f.piece.clone(), f.piece.clone()],
            &f.ledger
        ),
        Err(ClosureError::DuplicatePiece(f.piece.id()))
    );
}
#[test]
fn missing_targets_name_the_stable_instance_role_placement_or_source_owner() {
    let f = fixture();
    let mut placements = f.placements();
    let input = f.closure_input();
    let instance = *input.instances.first().unwrap();
    let closure = f.closure(input.clone(), &placements).unwrap();
    let held = placements.remove(1);
    let expected = ClosureError::MissingPlacement {
        instance: instance.id,
        role: ClosurePlacementRole::Second,
        placement: held.id(),
    };
    assert_eq!(f.closure(input.clone(), &placements), Err(expected.clone()));
    assert_eq!(
        closure.validate_current(
            &placements,
            &f.plan(),
            std::slice::from_ref(&f.piece),
            &f.ledger
        ),
        Err(expected)
    );
    assert_eq!(
        Closure::new(input, &f.placements(), &f.plan(), &[], &f.ledger),
        Err(ClosureError::MissingPiece {
            placement: instance.first,
            piece: f.piece.id()
        })
    );
    assert_eq!(
        closure.placement(
            EntityId::from_bits(999),
            ClosurePlacementRole::First,
            &placements,
            &f.plan(),
            std::slice::from_ref(&f.piece),
            &f.ledger
        ),
        Err(ClosureError::MissingInstance(EntityId::from_bits(999)))
    );
}
#[test]
fn current_placement_repairs_are_revalidated_at_birth_all_target_checks_and_borrowed_queries() {
    let mut f = fixture();
    let placements = f.placements();
    let input = f.closure_input();
    let instance = *input.instances.first().unwrap();
    let closure = f.closure(input.clone(), &placements).unwrap();
    let source = placements
        .first()
        .unwrap()
        .definition()
        .direction
        .range
        .edge();
    let parts = f.ledger.split(&mut f.ids, source, t(1, 3)).unwrap();
    let tail = f.ledger.split(&mut f.ids, parts.second(), t(1, 2)).unwrap();
    f.ledger.delete(&mut f.ids, tail.first()).unwrap();
    let raw = placements.first().unwrap().direction_resolution(&f.ledger);
    assert!(raw.evidence().start().resolved().is_some());
    assert!(raw.evidence().end().resolved().is_some());
    let expected = ClosureError::InvalidPlacement {
        instance: instance.id,
        role: ClosurePlacementRole::First,
        placement: instance.first,
        error: Box::new(NotionPlacementError::UnresolvedDirection(Box::new(
            raw.evidence().clone(),
        ))),
    };
    assert_eq!(f.closure(input, &placements), Err(expected.clone()));
    assert_eq!(
        closure.validate_current(
            &placements,
            &f.plan(),
            std::slice::from_ref(&f.piece),
            &f.ledger
        ),
        Err(expected.clone())
    );
    assert_eq!(
        closure.placement(
            instance.id,
            ClosurePlacementRole::First,
            &placements,
            &f.plan(),
            std::slice::from_ref(&f.piece),
            &f.ledger
        ),
        Err(expected)
    );
}
#[test]
fn removed_physical_copy_refuses_closure_execution_even_if_both_placement_ids_remain() {
    let f = fixture();
    let placements = f.placements();
    let input = f.closure_input();
    let instance = *input.instances.first().unwrap();
    let closure = f.closure(input, &placements).unwrap();
    let mut copies = f
        .plan()
        .copies()
        .iter()
        .map(|c| *c.definition())
        .collect::<Vec<_>>();
    copies.first_mut().unwrap().id = EntityId::from_bits(602);
    let plan = CutPlan::new(copies, std::slice::from_ref(&f.piece)).unwrap();
    assert_eq!(
        closure.validate_current(
            &placements,
            &plan,
            std::slice::from_ref(&f.piece),
            &f.ledger
        ),
        Err(ClosureError::InvalidPlacement {
            instance: instance.id,
            role: ClosurePlacementRole::First,
            placement: instance.first,
            error: Box::new(NotionPlacementError::MissingCopy(EntityId::from_bits(600)))
        })
    );
}
#[test]
fn fly_scope_refusal_precedes_geometry_and_names_requested_closure_gap_and_gate() {
    let f = fixture();
    let mut input = f.closure_input();
    let trousers_gap = EntityId::from_bits(808);
    input.kind = ClosureKind::Fly { trousers_gap };
    input.instances.clear();
    let expected = ClosureEnvelopeError::FlyDeferred {
        closure: input.id,
        trousers_gap,
    };
    assert_eq!(input.require_in_scope(), Err(expected));
    assert_eq!(expected.diagnostic(), "env_fly");
    assert_eq!(expected.proving_gate(), "G7");
    let message = expected.to_string();
    assert!(message.contains(&input.id.to_string()));
    assert!(message.contains(&trousers_gap.to_string()));
    assert!(message.contains("env_fly"));
    assert_eq!(
        Closure::new(input, &[], &f.plan(), &[], &f.ledger),
        Err(ClosureError::OutsideEnvelope(expected))
    );
}
#[test]
fn current_targets_are_borrowed_and_cloned_input_cannot_mutate_kind_or_instances() {
    let f = fixture();
    let placements = f.placements();
    let input = f.closure_input();
    let closure = f.closure(input.clone(), &placements).unwrap();
    let instance = *input.instances.first().unwrap();
    let target = closure
        .placement(
            instance.id,
            ClosurePlacementRole::First,
            &placements,
            &f.plan(),
            std::slice::from_ref(&f.piece),
            &f.ledger,
        )
        .unwrap();
    assert!(core::ptr::eq(target, placements.first().unwrap()));
    let mut changed = input.clone();
    changed.kind = ClosureKind::HookAndBar {
        hook: NotionSize::Declaration(EntityId::from_bits(809)),
        bar: NotionSize::Declaration(EntityId::from_bits(810)),
    };
    changed.instances.first_mut().unwrap().first = instance.second;
    changed.instances.first_mut().unwrap().second = instance.first;
    let replacement = f.closure(changed, &placements).unwrap();
    assert_ne!(replacement.definition(), &input);
    assert_eq!(closure.definition(), &input);
}

fn button_kind(button: NotionSize, operation: EntityId) -> ClosureKind {
    ClosureKind::ButtonAndButtonhole {
        button,
        hole: ButtonholeDerivation { operation },
    }
}
#[test]
fn buttonhole_length_source_borrows_the_canonical_button_size_and_derivation_without_defaulting() {
    let f = fixture();
    let placements = f.placements();
    let mut input = f.closure_input();
    let size = EntityId::from_bits(820);
    let operation = EntityId::from_bits(821);
    for button in [
        NotionSize::Declaration(size),
        NotionSize::Profile(ProfileParameterRef::new(size)),
    ] {
        input.kind = button_kind(button, operation);
        let closure = f.closure(input.clone(), &placements).unwrap();
        let source = closure.buttonhole_length_source().unwrap();
        let ClosureKind::ButtonAndButtonhole {
            button: canonical,
            hole,
        } = &closure.definition().kind
        else {
            panic!("expected button pair")
        };
        assert!(core::ptr::eq(source.button_size(), canonical));
        assert!(core::ptr::eq(source.derivation(), hole));
        assert_eq!(source.closure(), closure.id());
        assert_eq!(*source.button_size(), button);
        assert_eq!(source.derivation().operation, operation);
        assert_eq!(source.validation(), ButtonholeValidation::DeferredToG3);
        assert_eq!(
            closure.geometric_validation(),
            GeometricValidation::DeferredToG2
        );
        assert_eq!(closure.count(), Count::new(1));
        assert_eq!(
            closure.profile_binding_validation(),
            if matches!(button, NotionSize::Profile(_)) {
                Some(ProfileBindingValidation::DeferredToG4)
            } else {
                None
            }
        );
    }
}
#[test]
fn replacing_button_size_or_operation_changes_the_single_observed_hole_source() {
    let f = fixture();
    let placements = f.placements();
    let mut input = f.closure_input();
    input.kind = button_kind(
        NotionSize::Declaration(EntityId::from_bits(822)),
        EntityId::from_bits(823),
    );
    let original = f.closure(input.clone(), &placements).unwrap();
    let original_source = original.buttonhole_length_source().unwrap();
    input.kind = button_kind(
        NotionSize::Profile(ProfileParameterRef::new(EntityId::from_bits(824))),
        EntityId::from_bits(825),
    );
    let replacement = f.closure(input.clone(), &placements).unwrap();
    let current_source = replacement.buttonhole_length_source().unwrap();
    assert_eq!(current_source.closure(), original_source.closure());
    assert_ne!(current_source.button_size(), original_source.button_size());
    assert_ne!(current_source.derivation(), original_source.derivation());
    assert_eq!(
        current_source.derivation().operation,
        EntityId::from_bits(825)
    );
    assert_eq!(
        *original_source.button_size(),
        NotionSize::Declaration(EntityId::from_bits(822))
    );
    assert_eq!(
        original_source.derivation().operation,
        EntityId::from_bits(823)
    );
    assert_eq!(replacement.definition(), &input);
}
#[test]
fn non_button_closures_do_not_offer_a_buttonhole_source_or_claim_a_derived_length() {
    let f = fixture();
    let placements = f.placements();
    let mut input = f.closure_input();
    assert!(f
        .closure(input.clone(), &placements)
        .unwrap()
        .buttonhole_length_source()
        .is_none());
    input.kind = ClosureKind::HookAndBar {
        hook: NotionSize::Declaration(EntityId::from_bits(826)),
        bar: NotionSize::Declaration(EntityId::from_bits(827)),
    };
    assert!(f
        .closure(input, &placements)
        .unwrap()
        .buttonhole_length_source()
        .is_none());
}
#[test]
fn button_pairs_use_existing_nonempty_count_reuse_and_missing_target_refusals() {
    let f = fixture();
    let placements = f.placements();
    let mut input = f.closure_input();
    input.kind = button_kind(
        NotionSize::Declaration(EntityId::from_bits(828)),
        EntityId::from_bits(829),
    );
    let instance = *input.instances.first().unwrap();
    let closure = f.closure(input.clone(), &placements).unwrap();
    for role in [ClosurePlacementRole::First, ClosurePlacementRole::Second] {
        let target = closure
            .placement(
                instance.id,
                role,
                &placements,
                &f.plan(),
                std::slice::from_ref(&f.piece),
                &f.ledger,
            )
            .unwrap();
        assert_eq!(target.id(), instance.placement(role));
    }
    let mut empty = input.clone();
    empty.instances.clear();
    assert_eq!(
        f.closure(empty, &placements),
        Err(ClosureError::NoInstances)
    );
    let mut reused = input.clone();
    reused.instances.first_mut().unwrap().second = instance.first;
    assert_eq!(
        f.closure(reused, &placements),
        Err(ClosureError::ReusedPlacement(instance.first))
    );
    assert_eq!(
        f.closure(input, std::slice::from_ref(placements.first().unwrap())),
        Err(ClosureError::MissingPlacement {
            instance: instance.id,
            role: ClosurePlacementRole::Second,
            placement: instance.second
        })
    );
}
#[test]
fn button_source_does_not_bypass_current_physical_placement_validation() {
    let mut f = fixture();
    let placements = f.placements();
    let mut input = f.closure_input();
    input.kind = button_kind(
        NotionSize::Declaration(EntityId::from_bits(830)),
        EntityId::from_bits(831),
    );
    let instance = *input.instances.first().unwrap();
    let closure = f.closure(input.clone(), &placements).unwrap();
    let source = placements
        .first()
        .unwrap()
        .definition()
        .direction
        .range
        .edge();
    let parts = f.ledger.split(&mut f.ids, source, t(1, 3)).unwrap();
    let tail = f.ledger.split(&mut f.ids, parts.second(), t(1, 2)).unwrap();
    f.ledger.delete(&mut f.ids, tail.first()).unwrap();
    let raw = placements.first().unwrap().direction_resolution(&f.ledger);
    let expected = ClosureError::InvalidPlacement {
        instance: instance.id,
        role: ClosurePlacementRole::First,
        placement: instance.first,
        error: Box::new(NotionPlacementError::UnresolvedDirection(Box::new(
            raw.evidence().clone(),
        ))),
    };
    assert!(closure.buttonhole_length_source().is_some());
    assert_eq!(f.closure(input, &placements), Err(expected.clone()));
    assert_eq!(
        closure.validate_current(
            &placements,
            &f.plan(),
            std::slice::from_ref(&f.piece),
            &f.ledger
        ),
        Err(expected)
    );
}

#[test]
fn closure_envelope_error_keeps_original_cause() {
    use std::error::Error;
    let f = fixture();
    let mut input = f.closure_input();
    input.kind = ClosureKind::Fly {
        trousers_gap: EntityId::from_bits(808),
    };
    let issue = f.closure(input, &[]).unwrap_err();
    let actual = match &issue {
        ClosureError::OutsideEnvelope(cause) => cause,
        _ => panic!("original envelope refusal"),
    };
    let cause = issue
        .source()
        .expect("original envelope cause must remain reachable");
    assert!(core::ptr::eq(
        cause.downcast_ref::<ClosureEnvelopeError>().unwrap(),
        actual
    ));
    assert_eq!(actual.diagnostic(), "env_fly");
}

#[test]
fn closure_placement_error_keeps_original_cause() {
    use std::error::Error;
    let mut f = fixture();
    let placements = f.placements();
    let input = f.closure_input();
    let closure = f.closure(input.clone(), &placements).unwrap();
    let edge = placements.first().unwrap().definition().anchor.edge;
    f.ledger.delete(&mut f.ids, edge).unwrap();
    let issue = closure
        .validate_current(
            &placements,
            &f.plan(),
            std::slice::from_ref(&f.piece),
            &f.ledger,
        )
        .unwrap_err();
    let actual = match &issue {
        ClosureError::InvalidPlacement { error, .. } => error.as_ref(),
        _ => panic!("original placement refusal"),
    };
    let cause = issue
        .source()
        .expect("original placement cause must remain reachable");
    assert!(core::ptr::eq(
        cause.downcast_ref::<NotionPlacementError>().unwrap(),
        actual
    ));
    assert!(!cause.is::<Box<NotionPlacementError>>());
    assert!(cause
        .source()
        .unwrap()
        .is::<sc_core::ontology::AnchorError>());
    assert_eq!(closure.definition(), &input);
}

#[test]
fn inventory_preserves_closures_order_shared_objects_counts_and_explicit_empty_scope() {
    let f = fixture();
    let placements = f.placements();
    let first = f.closure(f.closure_input(), &placements).unwrap();
    let mut input = f.closure_input();
    input.id = EntityId::from_bits(810);
    input.instances.first_mut().unwrap().id = EntityId::from_bits(812);
    input.kind = ClosureKind::HookAndBar {
        hook: NotionSize::Declaration(EntityId::from_bits(813)),
        bar: NotionSize::Profile(ProfileParameterRef::new(EntityId::from_bits(814))),
    };
    let second = f.closure(input, &placements).unwrap();
    // Reusing targets across distinct Closures keeps the existing per-Closure reuse rule.
    let inventory = ClosureInventory::new(vec![second.clone(), first.clone()]).unwrap();
    assert_eq!(inventory.closures(), &[second.clone(), first]);
    let borrowed = inventory.closure(second.id()).unwrap();
    assert!(core::ptr::eq(
        borrowed,
        inventory.closures().first().unwrap()
    ));
    assert_eq!(borrowed.count(), Count::new(1));
    assert_eq!(
        borrowed.profile_binding_validation(),
        Some(ProfileBindingValidation::DeferredToG4)
    );
    assert_eq!(
        borrowed.geometric_validation(),
        GeometricValidation::DeferredToG2
    );
    assert!(inventory.closure(EntityId::from_bits(999)).is_none());
    let notions = NotionPlacementInventory::new(placements).unwrap();
    inventory
        .validate_current(
            &notions,
            &f.plan(),
            std::slice::from_ref(&f.piece),
            &f.ledger,
        )
        .unwrap();
    let empty = ClosureInventory::new(vec![]).unwrap();
    assert!(empty.closures().is_empty());
    empty
        .validate_current(
            &NotionPlacementInventory::new(vec![]).unwrap(),
            &CutPlan::new(vec![], &[]).unwrap(),
            &[],
            &f.ledger,
        )
        .unwrap();
}

#[test]
fn inventory_censuses_closure_and_nested_instance_identities_with_original_positions() {
    use ClosureInventoryPosition::{Closure as Owner, Instance};
    let f = fixture();
    let placements = f.placements();
    let first = f.closure(f.closure_input(), &placements).unwrap();
    let original_instance = first.definition().instances.first().unwrap().id;
    assert_eq!(
        ClosureInventory::new(vec![first.clone(), first.clone()]),
        Err(ClosureInventoryError::DuplicateIdentity {
            identity: first.id(),
            first: Owner { index: 0 },
            duplicate: Owner { index: 1 }
        })
    );
    for (closure_id, instance_id, identity, earlier, duplicate) in [
        (
            810,
            original_instance,
            original_instance,
            Instance {
                closure: 0,
                index: 0,
            },
            Instance {
                closure: 1,
                index: 0,
            },
        ),
        (
            802,
            EntityId::from_bits(812),
            original_instance,
            Instance {
                closure: 0,
                index: 0,
            },
            Owner { index: 1 },
        ),
        (
            810,
            first.id(),
            first.id(),
            Owner { index: 0 },
            Instance {
                closure: 1,
                index: 0,
            },
        ),
    ] {
        let mut input = f.closure_input();
        input.id = EntityId::from_bits(closure_id);
        input.instances.first_mut().unwrap().id = instance_id;
        let second = f.closure(input, &placements).unwrap();
        assert_eq!(
            ClosureInventory::new(vec![first.clone(), second]),
            Err(ClosureInventoryError::DuplicateIdentity {
                identity,
                first: earlier,
                duplicate
            })
        );
    }
    let mut input = f.closure_input();
    input.id = original_instance;
    let self_alias = f.closure(input, &placements).unwrap();
    assert_eq!(
        ClosureInventory::new(vec![self_alias]),
        Err(ClosureInventoryError::DuplicateIdentity {
            identity: original_instance,
            first: Owner { index: 0 },
            duplicate: Instance {
                closure: 0,
                index: 0
            }
        })
    );
}

#[test]
fn inventory_checks_unused_notion_sources_even_with_no_closures_and_keeps_concrete_chain() {
    use std::error::Error;
    let mut f = fixture();
    let mut placements = f.placements();
    let closure = f.closure(f.closure_input(), &placements).unwrap();
    let mut input = f.placement_input();
    input.id = EntityId::from_bits(702);
    input.anchor.edge = f.piece.definition().construction_lines.get(1).unwrap().edge;
    let unused = f.placement(input).unwrap();
    f.ledger.delete(&mut f.ids, input.anchor.edge).unwrap();
    closure
        .validate_current(
            &placements,
            &f.plan(),
            std::slice::from_ref(&f.piece),
            &f.ledger,
        )
        .unwrap();
    placements.push(unused.clone());
    let notions = NotionPlacementInventory::new(placements).unwrap();
    for closures in [vec![closure.clone()], vec![]] {
        let inventory = ClosureInventory::new(closures).unwrap();
        let issue = inventory
            .validate_current(
                &notions,
                &f.plan(),
                std::slice::from_ref(&f.piece),
                &f.ledger,
            )
            .unwrap_err();
        let ClosureInventoryError::Notions(actual) = &issue else {
            panic!("whole notion cause")
        };
        assert!(
            matches!(actual.as_ref(), NotionPlacementInventoryError::Placement { index: 2, placement, .. } if *placement == unused.id())
        );
        let cause = issue.source().unwrap();
        assert!(core::ptr::eq(
            cause
                .downcast_ref::<NotionPlacementInventoryError>()
                .unwrap(),
            actual.as_ref()
        ));
        assert!(!cause.is::<Box<NotionPlacementInventoryError>>());
        assert!(cause.source().unwrap().is::<NotionPlacementError>());
        assert!(cause
            .source()
            .unwrap()
            .source()
            .unwrap()
            .is::<sc_core::ontology::AnchorError>());
    }
    assert_eq!(notions.placement(unused.id()).unwrap().definition(), &input);
}

#[test]
fn inventory_checks_later_closure_targets_and_retains_exact_instance_role_and_cause() {
    use std::error::Error;
    let f = fixture();
    let mut placements = f.placements();
    let first = f.closure(f.closure_input(), &placements).unwrap();
    for id in [702, 703] {
        let mut input = f.placement_input();
        input.id = EntityId::from_bits(id);
        placements.push(f.placement(input).unwrap());
    }
    let mut input = f.closure_input();
    input.id = EntityId::from_bits(810);
    input.instances = vec![ClosureInstance {
        id: EntityId::from_bits(812),
        first: EntityId::from_bits(702),
        second: EntityId::from_bits(703),
    }];
    let second = f.closure(input.clone(), &placements).unwrap();
    let inventory = ClosureInventory::new(vec![first.clone(), second.clone()]).unwrap();
    let removed = EntityId::from_bits(703);
    placements.retain(|placement| placement.id() != removed);
    let notions = NotionPlacementInventory::new(placements).unwrap();
    first
        .validate_current(
            notions.placements(),
            &f.plan(),
            std::slice::from_ref(&f.piece),
            &f.ledger,
        )
        .unwrap();
    let issue = inventory
        .validate_current(
            &notions,
            &f.plan(),
            std::slice::from_ref(&f.piece),
            &f.ledger,
        )
        .unwrap_err();
    let ClosureInventoryError::Closure {
        index,
        closure,
        issue: actual,
    } = &issue
    else {
        panic!("later Closure cause")
    };
    assert_eq!((*index, *closure), (1, second.id()));
    assert_eq!(
        actual.as_ref(),
        &ClosureError::MissingPlacement {
            instance: EntityId::from_bits(812),
            role: ClosurePlacementRole::Second,
            placement: removed
        }
    );
    assert!(core::ptr::eq(
        issue
            .source()
            .unwrap()
            .downcast_ref::<ClosureError>()
            .unwrap(),
        actual.as_ref()
    ));
    assert!(!issue.source().unwrap().is::<Box<ClosureError>>());
    assert_eq!(inventory.closure(second.id()).unwrap().definition(), &input);
    // Held target loss does not prevent constructing a new inspectable inventory.
    assert_eq!(
        ClosureInventory::new(inventory.closures().to_vec()).unwrap(),
        inventory
    );
}

#[test]
fn inventory_checks_complete_current_plan_even_without_closures_or_notions() {
    use sc_core::ontology::CutPlanError;
    use std::error::Error;
    let f = fixture();
    let mut definition = f.piece.definition().clone();
    definition.quantity = 4;
    let current = Piece::new(definition, &f.ledger).unwrap();
    let inventory = ClosureInventory::new(vec![]).unwrap();
    let notions = NotionPlacementInventory::new(vec![]).unwrap();
    let issue = inventory
        .validate_current(
            &notions,
            &f.plan(),
            std::slice::from_ref(&current),
            &f.ledger,
        )
        .unwrap_err();
    let ClosureInventoryError::Notions(actual) = &issue else {
        panic!("notion inventory cause")
    };
    let NotionPlacementInventoryError::CutPlan(plan) = actual.as_ref() else {
        panic!("complete plan cause")
    };
    assert_eq!(
        plan.as_ref(),
        &CutPlanError::QuantityMismatch {
            piece: current.id(),
            expected: 4,
            actual: 2
        }
    );
    assert!(core::ptr::eq(
        issue
            .source()
            .unwrap()
            .source()
            .unwrap()
            .downcast_ref::<CutPlanError>()
            .unwrap(),
        plan.as_ref()
    ));
}
