//! Pocket intent binds physical composition without duplicating pattern geometry.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
use sc_core::ontology::{
    AnchorError, CopyOrientation, CutCopyDefinition, CutPlan, CuttingSide,
    DeterministicIdGenerator, DirectedEdge, DirectedRange, Direction, EdgeAnchor, EdgeRange,
    EdgeRef, EntityId, GeometricValidation, IdGenerator, IdentityLedger, LabelText, LocalTag,
    MaterialAssignment, Mirroring, Param, Piece, PieceDefinition, Pocket, PocketDefinition,
    PocketError, PocketOpening, PocketOpeningValidation, PocketPieceRef, ProfileBindingValidation,
    ProfileParameterRef, Rational, Resolution,
};
use sc_units::Length;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn t(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).unwrap()).unwrap()
}
struct Fixture {
    pieces: Vec<Piece>,
    ledger: IdentityLedger,
    ids: DeterministicIdGenerator,
}
fn fixture() -> Fixture {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let mut pieces = Vec::new();
    for name in ["served", "pocket bag"] {
        let edges = ledger.declare_edges(&mut ids, 4).unwrap();
        let directed = |index| DirectedEdge {
            edge: *edges.get(index).unwrap(),
            direction: Direction::Original,
        };
        pieces.push(
            Piece::new(
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
                        name: name.into(),
                        size: "M".into(),
                        fabric: "woven".into(),
                        colorway: "plain".into(),
                    },
                },
                &ledger,
            )
            .unwrap(),
        );
    }
    Fixture {
        pieces,
        ledger,
        ids,
    }
}
impl Fixture {
    fn plan(&self) -> CutPlan {
        let copies = self
            .pieces
            .iter()
            .enumerate()
            .flat_map(|(index, piece)| {
                [CopyOrientation::Authored, CopyOrientation::Reflected]
                    .into_iter()
                    .enumerate()
                    .map(move |(side, orientation)| CutCopyDefinition {
                        id: id(1000 + index as u128 * 100 + side as u128),
                        piece: piece.id(),
                        orientation,
                    })
            })
            .collect();
        CutPlan::new(copies, &self.pieces).unwrap()
    }
    fn input(&self) -> PocketDefinition {
        let served = self.pieces.first().unwrap();
        let component = self.pieces.get(1).unwrap();
        PocketDefinition {
            id: id(2000),
            served: PocketPieceRef {
                copy: id(1000),
                piece: served.id(),
            },
            position: EdgeAnchor {
                edge: served.definition().boundary.first().unwrap().edge,
                param: t(1, 4),
            },
            orientation: DirectedRange {
                range: EdgeRange::whole(
                    served.definition().construction_lines.first().unwrap().edge,
                ),
                direction: Direction::Reversed,
            },
            opening: PocketOpening::Declaration(id(3000)),
            components: vec![
                PocketPieceRef {
                    copy: id(1100),
                    piece: component.id(),
                },
                PocketPieceRef {
                    copy: id(1101),
                    piece: component.id(),
                },
            ],
        }
    }
    fn pocket(&self, input: PocketDefinition) -> Result<Pocket, PocketError> {
        Pocket::new(input, &self.plan(), &self.pieces, &self.ledger)
    }
}
#[test]
fn physical_components_borrow_canonical_pattern_metadata_and_retain_all_deferred_proofs() {
    let f = fixture();
    let input = f.input();
    let pocket = f.pocket(input.clone()).unwrap();
    let plan = f.plan();
    assert_eq!(pocket.id(), input.id);
    assert_eq!(pocket.definition(), &input);
    for copy in [id(1100), id(1101)] {
        assert!(std::ptr::eq(
            pocket.component(copy, &plan, &f.pieces).unwrap(),
            f.pieces.get(1).unwrap()
        ));
    }
    assert_ne!(
        input.components.first().unwrap().copy,
        input.components.get(1).unwrap().copy
    );
    assert_eq!(
        plan.copy(id(1101)).unwrap().orientation(),
        CopyOrientation::Reflected
    );
    assert_eq!(
        pocket
            .orientation_resolution(&f.ledger)
            .portions()
            .next()
            .unwrap()
            .direction,
        Direction::Reversed
    );
    assert_eq!(
        pocket.geometric_validation(),
        GeometricValidation::DeferredToG2
    );
    assert_eq!(
        pocket.opening_validation(),
        PocketOpeningValidation::DeferredToG3
    );
    assert_eq!(pocket.profile_binding_validation(), None);
    let mut profile = input;
    profile.opening = PocketOpening::Profile(ProfileParameterRef::new(id(3001)));
    let bound = f.pocket(profile).unwrap();
    assert_eq!(
        bound.profile_binding_validation(),
        Some(ProfileBindingValidation::DeferredToG4)
    );
    assert_eq!(
        bound.opening_validation(),
        PocketOpeningValidation::DeferredToG3
    );
    pocket
        .validate_current(&plan, &f.pieces, &f.ledger)
        .unwrap();
}
#[test]
fn composition_is_nonempty_and_unique_by_physical_copy_even_with_changed_source() {
    let f = fixture();
    let mut input = f.input();
    input.components.clear();
    assert_eq!(f.pocket(input), Err(PocketError::NoComponents));
    let mut input = f.input();
    *input.components.get_mut(1).unwrap() = PocketPieceRef {
        piece: input.served.piece,
        ..*input.components.first().unwrap()
    };
    assert_eq!(
        f.pocket(input),
        Err(PocketError::DuplicateComponent(id(1100)))
    );
}
#[test]
fn served_copy_may_be_an_explicit_component_without_inferred_construction_approval() {
    let f = fixture();
    let mut input = f.input();
    input.components.push(input.served);
    let pocket = f.pocket(input).unwrap();
    assert!(std::ptr::eq(
        pocket.component(id(1000), &f.plan(), &f.pieces).unwrap(),
        f.pieces.first().unwrap()
    ));
    assert_eq!(
        pocket.opening_validation(),
        PocketOpeningValidation::DeferredToG3
    );
}
#[test]
fn all_explicit_target_failures_are_named_and_registry_ambiguity_refuses() {
    let f = fixture();
    let plan = f.plan();
    for served in [true, false] {
        let mut input = f.input();
        let target = if served {
            &mut input.served
        } else {
            input.components.first_mut().unwrap()
        };
        target.copy = id(9999);
        assert_eq!(
            Pocket::new(input, &plan, &f.pieces, &f.ledger),
            Err(PocketError::MissingCopy(id(9999)))
        );
        let mut input = f.input();
        let target = if served {
            &mut input.served
        } else {
            input.components.first_mut().unwrap()
        };
        let actual = target.piece;
        target.piece = id(9999);
        let copy = target.copy;
        assert_eq!(
            Pocket::new(input, &plan, &f.pieces, &f.ledger),
            Err(PocketError::WrongCopyPiece {
                copy,
                expected: id(9999),
                actual
            })
        );
    }
    let input = f.input();
    assert_eq!(
        Pocket::new(input.clone(), &plan, f.pieces.get(..1).unwrap(), &f.ledger),
        Err(PocketError::MissingPiece {
            copy: id(1100),
            piece: input.components.first().unwrap().piece
        })
    );
    assert_eq!(
        Pocket::new(input.clone(), &plan, f.pieces.get(1..).unwrap(), &f.ledger),
        Err(PocketError::MissingPiece {
            copy: id(1000),
            piece: input.served.piece
        })
    );
    let mut duplicate = f.pieces.clone();
    duplicate.push(f.pieces.get(1).unwrap().clone());
    assert_eq!(
        Pocket::new(input, &plan, &duplicate, &f.ledger),
        Err(PocketError::DuplicatePiece(f.pieces.get(1).unwrap().id()))
    );
}
#[test]
fn current_copy_removal_and_reassignment_cannot_retarget_to_other_copies_or_pieces() {
    let f = fixture();
    let pocket = f.pocket(f.input()).unwrap();
    let mut copies = f
        .plan()
        .copies()
        .iter()
        .map(|c| *c.definition())
        .collect::<Vec<_>>();
    copies.reverse();
    let reordered = CutPlan::new(copies.clone(), &f.pieces).unwrap();
    pocket
        .validate_current(&reordered, &f.pieces, &f.ledger)
        .unwrap();
    copies.iter_mut().find(|c| c.id == id(1100)).unwrap().id = id(1102);
    let replaced = CutPlan::new(copies, &f.pieces).unwrap();
    assert_eq!(
        pocket.validate_current(&replaced, &f.pieces, &f.ledger),
        Err(PocketError::MissingCopy(id(1100)))
    );
    assert_eq!(
        pocket.component(id(1100), &replaced, &f.pieces),
        Err(PocketError::MissingCopy(id(1100)))
    );
    let reassigned = CutPlan::new(
        f.plan()
            .copies()
            .iter()
            .map(|c| CutCopyDefinition {
                piece: if c.piece() == f.pieces.first().unwrap().id() {
                    f.pieces.get(1).unwrap().id()
                } else {
                    f.pieces.first().unwrap().id()
                },
                ..*c.definition()
            })
            .collect(),
        &f.pieces,
    )
    .unwrap();
    assert_eq!(
        pocket.validate_current(&reassigned, &f.pieces, &f.ledger),
        Err(PocketError::WrongCopyPiece {
            copy: id(1000),
            expected: f.pieces.first().unwrap().id(),
            actual: f.pieces.get(1).unwrap().id()
        })
    );
    assert_eq!(
        pocket.component(id(1100), &reassigned, &f.pieces),
        Err(PocketError::WrongCopyPiece {
            copy: id(1100),
            expected: f.pieces.get(1).unwrap().id(),
            actual: f.pieces.first().unwrap().id()
        })
    );
    assert_eq!(
        pocket.component(id(9999), &f.plan(), &f.pieces),
        Err(PocketError::MissingComponent(id(9999)))
    );
}
#[test]
fn birth_position_is_live_owned_and_current_position_preserves_historical_choices() {
    let mut f = fixture();
    let input = f.input();
    let pocket = f.pocket(input.clone()).unwrap();
    let mut at_split = input.clone();
    at_split.position.param = t(1, 2);
    let ambiguous = f.pocket(at_split).unwrap();
    let mut missing = input.clone();
    missing.position.edge = EdgeRef::new(id(u128::MAX), LocalTag::FIRST);
    assert!(
        matches!(f.pocket(missing),Err(PocketError::Position(e)) if matches!(*e,AnchorError::MissingEdge{..}))
    );
    let mut foreign = input.clone();
    foreign.position.edge = f
        .pieces
        .get(1)
        .unwrap()
        .definition()
        .boundary
        .first()
        .unwrap()
        .edge;
    assert!(
        matches!(f.pocket(foreign),Err(PocketError::Position(e)) if matches!(*e,AnchorError::AnchorOutsidePiece{..}))
    );
    let parts = f
        .ledger
        .split(&mut f.ids, input.position.edge, t(1, 2))
        .unwrap();
    assert!(
        matches!(f.pocket(input.clone()),Err(PocketError::Position(e)) if matches!(*e,AnchorError::MissingEdge{..}))
    );
    pocket
        .validate_current(&f.plan(), &f.pieces, &f.ledger)
        .unwrap();
    assert!(
        matches!(ambiguous.validate_current(&f.plan(),&f.pieces,&f.ledger),Err(PocketError::Position(e)) if matches!(*e,AnchorError::CurrentUnresolved{ref resolution,..} if matches!(**resolution,Resolution::SplitPoint{..})))
    );
    f.ledger.reverse(&mut f.ids, parts.first()).unwrap();
    pocket
        .validate_current(&f.plan(), &f.pieces, &f.ledger)
        .unwrap();
    f.ledger.delete(&mut f.ids, parts.first()).unwrap();
    assert!(
        matches!(pocket.validate_current(&f.plan(),&f.pieces,&f.ledger),Err(PocketError::Position(e)) if matches!(*e,AnchorError::CurrentUnresolved{ref resolution,..} if **resolution==pocket.position_resolution(&f.ledger)))
    );
    assert_eq!(pocket.definition(), &input);
}
#[test]
fn orientation_requires_full_owned_intervals_and_unique_endpoints() {
    let mut f = fixture();
    let mut input = f.input();
    input.orientation.range = EdgeRange::whole(EdgeRef::new(id(u128::MAX), LocalTag::FIRST));
    assert!(matches!(
        f.pocket(input),
        Err(PocketError::UnresolvedOrientation(_))
    ));
    let mut input = f.input();
    input.orientation.range = EdgeRange::whole(
        f.pieces
            .get(1)
            .unwrap()
            .definition()
            .boundary
            .first()
            .unwrap()
            .edge,
    );
    assert_eq!(
        f.pocket(input),
        Err(PocketError::OrientationOutsidePiece(
            f.pieces.first().unwrap().id()
        ))
    );
    let mut input = f.input();
    let edge = input.orientation.range.edge();
    f.ledger.split(&mut f.ids, edge, t(1, 2)).unwrap();
    input.orientation.range = EdgeRange::new(edge, t(1, 2), Param::END).unwrap();
    assert!(
        matches!(f.pocket(input),Err(PocketError::UnresolvedOrientation(e)) if e.has_full_coverage() && e.start().resolved().is_none())
    );
}
#[test]
fn orientation_cannot_hide_foreign_middle_geometry_behind_owned_live_endpoints() {
    let mut f = fixture();
    let mut input = f.input();
    let unit = Length::from_micrometres(1).unwrap();
    let foreign = f
        .pieces
        .get(1)
        .unwrap()
        .definition()
        .boundary
        .first()
        .unwrap()
        .edge;
    let right = f
        .pieces
        .first()
        .unwrap()
        .definition()
        .construction_lines
        .get(1)
        .unwrap()
        .edge;
    let first = f
        .ledger
        .merge(
            &mut f.ids,
            input.orientation.range.edge(),
            unit,
            foreign,
            unit,
        )
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
    input.orientation.range = EdgeRange::whole(merged);
    let e = f.ledger.resolve_range(input.orientation.range);
    assert!(e.has_full_coverage());
    assert!(e.start().resolved().is_some());
    assert!(e.end().resolved().is_some());
    assert_eq!(
        f.pocket(input.clone()),
        Err(PocketError::OrientationOutsidePiece(
            f.pieces.first().unwrap().id()
        ))
    );
    f.ledger.reverse(&mut f.ids, merged).unwrap();
    assert_eq!(
        f.pocket(input),
        Err(PocketError::OrientationOutsidePiece(
            f.pieces.first().unwrap().id()
        ))
    );
}
#[test]
fn current_orientation_retains_interior_loss_and_definition_cannot_be_mutated_by_cloning() {
    let mut f = fixture();
    let input = f.input();
    let pocket = f.pocket(input.clone()).unwrap();
    let mut changed = input.clone();
    changed.components.reverse();
    changed.opening = PocketOpening::Declaration(id(3002));
    let replacement = f.pocket(changed).unwrap();
    assert_eq!(pocket.definition(), &input);
    assert_ne!(replacement.definition(), &input);
    let first = f
        .ledger
        .split(&mut f.ids, input.orientation.range.edge(), t(1, 3))
        .unwrap();
    let tail = f.ledger.split(&mut f.ids, first.second(), t(1, 2)).unwrap();
    f.ledger.reverse(&mut f.ids, tail.second()).unwrap();
    f.ledger.delete(&mut f.ids, tail.first()).unwrap();
    let raw = pocket.orientation_resolution(&f.ledger);
    assert_eq!(
        raw.portions().next().unwrap().direction,
        Direction::Original
    );
    assert!(raw.evidence().start().resolved().is_some());
    assert!(raw.evidence().end().resolved().is_some());
    assert!(!raw.evidence().has_full_coverage());
    assert!(
        matches!(pocket.validate_current(&f.plan(),&f.pieces,&f.ledger),Err(PocketError::UnresolvedOrientation(e)) if *e==*raw.evidence())
    );
}
#[test]
fn component_queries_preserve_current_metadata_and_visible_contour_repairs_without_certifying_release(
) {
    let mut f = fixture();
    let pocket = f.pocket(f.input()).unwrap();
    let plan = f.plan();
    let mut definition = f.pieces.get(1).unwrap().definition().clone();
    definition.label.name = "current pocket bag".into();
    *f.pieces.get_mut(1).unwrap() = Piece::new(definition, &f.ledger).unwrap();
    assert_eq!(
        pocket
            .component(id(1100), &plan, &f.pieces)
            .unwrap()
            .definition()
            .label
            .name,
        "current pocket bag"
    );
    let edge = f
        .pieces
        .get(1)
        .unwrap()
        .definition()
        .boundary
        .first()
        .unwrap()
        .edge;
    let parts = f.ledger.split(&mut f.ids, edge, t(1, 3)).unwrap();
    let tail = f.ledger.split(&mut f.ids, parts.second(), t(1, 2)).unwrap();
    f.ledger.delete(&mut f.ids, tail.first()).unwrap();
    let component = pocket.component(id(1100), &plan, &f.pieces).unwrap();
    assert!(component
        .range_resolutions(&f.ledger)
        .any(|(_, e)| !e.has_full_coverage()
            && e.start().resolved().is_some()
            && e.end().resolved().is_some()));
    pocket
        .validate_current(&plan, &f.pieces, &f.ledger)
        .unwrap(); // target/placement validation; Design must consume component contour evidence.
    assert_eq!(
        pocket.geometric_validation(),
        GeometricValidation::DeferredToG2
    );
}
#[test]
fn current_position_ownership_uses_piece_content_instead_of_identity_only() {
    let mut f = fixture();
    let pocket = f.pocket(f.input()).unwrap();
    let plan = f.plan();
    let mut input = f.pieces.first().unwrap().definition().clone();
    input.boundary = vec![input.construction_lines.remove(1)];
    *f.pieces.first_mut().unwrap() = Piece::new(input, &f.ledger).unwrap();
    assert!(
        matches!(pocket.validate_current(&plan,&f.pieces,&f.ledger),Err(PocketError::Position(e)) if matches!(*e,AnchorError::AnchorOutsidePiece{..}))
    );
}

#[test]
fn pocket_position_error_keeps_original_anchor_cause() {
    use std::error::Error;
    let mut f = fixture();
    let input = f.input();
    let pocket = f.pocket(input.clone()).unwrap();
    f.ledger.delete(&mut f.ids, input.position.edge).unwrap();
    let issue = pocket
        .validate_current(&f.plan(), &f.pieces, &f.ledger)
        .unwrap_err();
    let actual = match &issue {
        PocketError::Position(anchor) => anchor.as_ref(),
        _ => panic!("original position error"),
    };
    assert!(matches!(actual, AnchorError::CurrentUnresolved { .. }));
    let cause = issue
        .source()
        .expect("original AnchorError must remain in standard error chain");
    assert!(core::ptr::eq(
        cause
            .downcast_ref::<AnchorError>()
            .expect("concrete anchor cause"),
        actual
    ));
    assert!(!cause.is::<Box<AnchorError>>());
    assert_eq!(pocket.definition(), &input);
}

#[test]
fn inventory_keeps_authored_objects_exact_ids_and_explicit_empty_scope() {
    use sc_core::ontology::{PocketInventory, PocketInventoryError};
    let f = fixture();
    let first = f.pocket(f.input()).unwrap();
    let mut second_input = f.input();
    second_input.id = id(2001);
    second_input.components.reverse();
    second_input.opening = PocketOpening::Profile(ProfileParameterRef::new(id(3001)));
    let second = f.pocket(second_input).unwrap();
    let original = vec![second, first.clone()];
    let inventory = PocketInventory::new(original.clone()).unwrap();
    assert_eq!(inventory.pockets(), original);
    assert!(core::ptr::eq(
        inventory.pocket(id(2000)).unwrap(),
        inventory.pockets().get(1).unwrap()
    ));
    assert!(inventory.pocket(id(9999)).is_none());
    assert_eq!(
        inventory.validate_current(&f.plan(), &f.pieces, &f.ledger),
        Ok(())
    );
    assert_eq!(
        inventory.pockets().first().unwrap().opening_validation(),
        PocketOpeningValidation::DeferredToG3
    );
    assert_eq!(
        inventory
            .pockets()
            .first()
            .unwrap()
            .profile_binding_validation(),
        Some(ProfileBindingValidation::DeferredToG4)
    );
    assert_eq!(
        PocketInventory::new(vec![first.clone(), first]),
        Err(PocketInventoryError::DuplicateIdentity {
            identity: id(2000),
            first: 0,
            duplicate: 1
        })
    );
    let empty = PocketInventory::new(vec![]).unwrap();
    assert!(empty.pockets().is_empty());
    assert_eq!(
        empty.validate_current(&f.plan(), &f.pieces, &f.ledger),
        Ok(())
    );
}

#[test]
fn inventory_rechecks_complete_current_copies_against_actual_component_quantities() {
    use sc_core::ontology::{PocketInventory, PocketInventoryError};
    use std::error::Error;
    let f = fixture();
    let pocket = f.pocket(f.input()).unwrap();
    let old_plan = f.plan();
    let pieces = f
        .pieces
        .iter()
        .map(|piece| {
            let mut definition = piece.definition().clone();
            definition.quantity = 4;
            Piece::new(definition, &f.ledger).unwrap()
        })
        .collect::<Vec<_>>();
    assert_eq!(
        pocket.validate_current(&old_plan, &pieces, &f.ledger),
        Ok(())
    );
    let expected = CutPlan::new(
        old_plan
            .copies()
            .iter()
            .map(|copy| *copy.definition())
            .collect(),
        &pieces,
    )
    .unwrap_err();
    let inventory = PocketInventory::new(vec![pocket]).unwrap();
    let issue = inventory
        .validate_current(&old_plan, &pieces, &f.ledger)
        .unwrap_err();
    assert_eq!(
        issue,
        PocketInventoryError::CutPlan(Box::new(expected.clone()))
    );
    assert_eq!(
        issue
            .source()
            .unwrap()
            .downcast_ref::<sc_core::ontology::CutPlanError>(),
        Some(&expected)
    );
}

#[test]
fn inventory_blocks_component_interior_loss_even_when_placement_and_component_endpoints_stay_valid()
{
    use sc_core::ontology::{PocketInventory, PocketInventoryError};
    for kind in ["boundary", "hole", "construction"] {
        let mut f = fixture();
        if kind == "hole" {
            let mut changed = f.pieces.get(1).unwrap().definition().clone();
            let mut hole = *changed.construction_lines.first().unwrap();
            changed
                .construction_lines
                .retain(|item| item.edge != hole.edge);
            hole.direction = Direction::Reversed;
            changed.holes.push(vec![hole]);
            *f.pieces.get_mut(1).unwrap() = Piece::new(changed, &f.ledger).unwrap();
        }
        let input = f.input();
        let component = f.pieces.get(1).unwrap();
        let component_id = component.id();
        let (held, edge_index) = match kind {
            "boundary" => (*component.definition().boundary.first().unwrap(), 0),
            "hole" => (
                *component
                    .definition()
                    .holes
                    .first()
                    .unwrap()
                    .first()
                    .unwrap(),
                1,
            ),
            _ => (
                *component.definition().construction_lines.last().unwrap(),
                3,
            ),
        };
        let pocket = f.pocket(input.clone()).unwrap();
        let original = pocket.clone();
        let first = f.ledger.split(&mut f.ids, held.edge, t(1, 3)).unwrap();
        let second = f.ledger.split(&mut f.ids, first.second(), t(1, 2)).unwrap();
        f.ledger.delete(&mut f.ids, second.first()).unwrap();
        assert_eq!(
            pocket.validate_current(&f.plan(), &f.pieces, &f.ledger),
            Ok(())
        );
        let inventory = PocketInventory::new(vec![pocket]).unwrap();
        let issue = inventory
            .validate_current(&f.plan(), &f.pieces, &f.ledger)
            .unwrap_err();
        let PocketInventoryError::ComponentRange {
            pocket,
            component,
            copy,
            piece,
            edge,
            direction,
            evidence,
            endpoints,
        } = issue
        else {
            panic!("complete actual component interval refusal");
        };
        assert_eq!(
            (pocket, component, copy, piece, edge, direction),
            (
                id(2000),
                0,
                id(1100),
                component_id,
                edge_index,
                held.direction
            )
        );
        assert_eq!(evidence.held(), EdgeRange::whole(held.edge));
        assert!(evidence.start().resolved().is_some() && evidence.end().resolved().is_some());
        assert!(endpoints.iter().all(|answer| answer.resolved().is_some()));
        assert_eq!(evidence.portions().len(), 3);
        assert_eq!(evidence.repairs().count(), 1);
        assert_eq!(inventory.pockets(), &[original]);
    }
}

#[test]
fn inventory_preserves_the_exact_nested_pocket_and_anchor_error_chain_after_current_damage() {
    use sc_core::ontology::{PocketInventory, PocketInventoryError};
    use std::error::Error;
    let mut f = fixture();
    let first_input = f.input();
    let first = f.pocket(first_input).unwrap();
    let mut input = f.input();
    input.id = id(2001);
    input.position.edge = f
        .pieces
        .first()
        .unwrap()
        .definition()
        .construction_lines
        .get(1)
        .unwrap()
        .edge;
    let pocket = f.pocket(input.clone()).unwrap();
    let inventory = PocketInventory::new(vec![first, pocket]).unwrap();
    f.ledger.delete(&mut f.ids, input.position.edge).unwrap();
    assert_eq!(
        inventory
            .pockets()
            .first()
            .unwrap()
            .validate_current(&f.plan(), &f.pieces, &f.ledger),
        Ok(())
    );
    let issue = inventory
        .validate_current(&f.plan(), &f.pieces, &f.ledger)
        .unwrap_err();
    assert!(
        matches!(&issue,PocketInventoryError::Pocket { index:1, pocket, issue } if *pocket==id(2001) && matches!(issue.as_ref(),PocketError::Position(_)))
    );
    let pocket_cause = issue
        .source()
        .unwrap()
        .downcast_ref::<PocketError>()
        .unwrap();
    assert!(!issue.source().unwrap().is::<Box<PocketError>>());
    assert!(pocket_cause.source().unwrap().is::<AnchorError>());
    assert_eq!(inventory.pockets().get(1).unwrap().definition(), &input);
}

#[test]
fn inventory_component_endpoints_use_only_the_exact_piece_saved_choice_and_keep_raw_evidence() {
    use sc_core::ontology::{PocketInventory, PocketInventoryError, SplitSide};
    for matching_owner in [false, true] {
        let mut f = fixture();
        let pocket = f.pocket(f.input()).unwrap();
        let component = f.pieces.get(1).unwrap();
        let component_id = component.id();
        let held = component.definition().boundary.first().unwrap().edge;
        let owner = if matching_owner {
            component_id
        } else {
            id(99999)
        };
        f.ledger.register(owner, held, Param::END).unwrap();
        let peer = *f
            .ledger
            .declare_edges(&mut f.ids, 1)
            .unwrap()
            .first()
            .unwrap();
        let length = Length::from_micrometres(1000).unwrap();
        let merged = f
            .ledger
            .merge(&mut f.ids, held, length, peer, length)
            .unwrap();
        let split = f.ledger.split(&mut f.ids, merged, t(1, 2)).unwrap();
        f.ledger
            .state_split_choice(owner, held, Param::END, SplitSide::Second)
            .unwrap();
        assert!(f
            .ledger
            .resolve_registered(owner, held, Param::END)
            .unwrap()
            .resolved()
            .is_some());
        let raw = f.ledger.resolve_range(EdgeRange::whole(held));
        assert!(raw.has_full_coverage() && matches!(raw.end(), Resolution::SplitPoint { .. }));
        let inventory = PocketInventory::new(vec![pocket]).unwrap();
        if matching_owner {
            assert_eq!(
                inventory.validate_current(&f.plan(), &f.pieces, &f.ledger),
                Ok(())
            );
            f.ledger.delete(&mut f.ids, split.second()).unwrap();
            let issue = inventory
                .validate_current(&f.plan(), &f.pieces, &f.ledger)
                .unwrap_err();
            assert!(
                matches!(&issue,PocketInventoryError::ComponentRange { evidence,endpoints,.. } if evidence.has_full_coverage() && matches!(endpoints.get(1),Some(Resolution::Unresolved(_))))
            );
        } else {
            let issue = inventory
                .validate_current(&f.plan(), &f.pieces, &f.ledger)
                .unwrap_err();
            assert!(
                matches!(&issue,PocketInventoryError::ComponentRange { evidence,endpoints,.. } if matches!(evidence.end(),Resolution::SplitPoint { .. }) && matches!(endpoints.get(1),Some(Resolution::SplitPoint { .. })))
            );
        }
        assert_eq!(
            inventory
                .pockets()
                .first()
                .unwrap()
                .definition()
                .components
                .first()
                .unwrap()
                .piece,
            component_id
        );
    }
}
