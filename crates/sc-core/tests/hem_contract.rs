//! Hem contracts: authored intent and stable current Facing composition.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
use sc_core::ontology::{
    CuttingSide, DeterministicIdGenerator, DirectedEdge, DirectedRange, Direction, EdgeRange,
    EdgeRef, EntityId, Facing, GeometricValidation, Hem, HemDefinition, HemDepth, HemError,
    HemFoldType, HemInventory, HemInventoryError, HemMethod, IdGenerator, IdentityLedger,
    Interfacing, LabelText, LayerDefinition, LayerError, LayerInventory, LayerInventoryError,
    LayerOffsetRelationship, LocalTag, MaterialAssignment, Mirroring, Param, Piece,
    PieceDefinition, ProfileBindingValidation, ProfileParameterRef, RangeIssue, RangePortion,
    Rational,
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
            id: ids.next_id().expect("test ID capacity"),
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

impl Fixture {
    fn facing(&self) -> Facing {
        Facing::new(self.definition.clone(), &self.piece, &self.ledger).unwrap()
    }
    fn hem_definition(&self) -> HemDefinition {
        HemDefinition {
            id: EntityId::from_bits(200),
            edge: self.piece.definition().boundary.first().unwrap().edge,
            depth: HemDepth::Explicit {
                parameter: EntityId::from_bits(201),
                value: Length::from_micrometres(30_000).unwrap(),
            },
            fold_type: HemFoldType::Declaration(EntityId::from_bits(202)),
            method: HemMethod::Turned,
        }
    }
    fn hem(&self, definition: HemDefinition, facing: Option<&Facing>) -> Result<Hem, HemError> {
        Hem::new(definition, &self.piece, &self.ledger, facing)
    }
}
#[test]
fn turned_fixture_shaped_depth_retains_fold_binding_and_no_physical_approval() {
    let f = fixture();
    let input = f.hem_definition();
    let hem = f.hem(input.clone(), None).unwrap();
    assert_eq!(hem.id(), input.id);
    assert_eq!(hem.piece(), f.piece.id());
    assert_eq!(hem.definition(), &input);
    assert_eq!(
        hem.geometric_validation(),
        GeometricValidation::DeferredToG2
    );
    assert_eq!(hem.profile_binding_validation(), None);
    assert_eq!(hem.facing(&f.piece, &f.ledger, None).unwrap(), None);
    assert_eq!(
        hem.edge_resolution(&f.ledger),
        f.ledger.resolve_range(EdgeRange::whole(input.edge))
    );
}
#[test]
fn depth_origins_retain_authored_zero_and_symbols_without_defaulting() {
    let f = fixture();
    let mut input = f.hem_definition();
    let parameter = EntityId::from_bits(203);
    for value in [0, 1, 30_000] {
        input.depth = HemDepth::Explicit {
            parameter,
            value: Length::from_micrometres(value).unwrap(),
        };
        assert_eq!(f.hem(input.clone(), None).unwrap().definition(), &input);
    }
    let value = Length::from_micrometres(-1).unwrap();
    input.depth = HemDepth::Explicit { parameter, value };
    assert_eq!(
        f.hem(input.clone(), None),
        Err(HemError::NegativeDepth(value))
    );
    for depth in [
        HemDepth::Formula(parameter),
        HemDepth::Profile(ProfileParameterRef::new(parameter)),
    ] {
        input.depth = depth;
        let hem = f.hem(input.clone(), None).unwrap();
        assert_eq!(hem.definition().depth, depth);
        assert_eq!(
            hem.profile_binding_validation(),
            if matches!(depth, HemDepth::Profile(_)) {
                Some(ProfileBindingValidation::DeferredToG4)
            } else {
                None
            }
        );
    }
    input.fold_type = HemFoldType::Profile(ProfileParameterRef::new(EntityId::from_bits(204)));
    input.depth = HemDepth::Formula(parameter);
    let hem = f.hem(input.clone(), None).unwrap();
    assert_eq!(hem.definition().fold_type, input.fold_type);
    assert_eq!(
        hem.profile_binding_validation(),
        Some(ProfileBindingValidation::DeferredToG4)
    );
}
#[test]
fn faced_method_borrows_one_canonical_target_without_duplicating_its_material_or_sources() {
    let f = fixture();
    let facing = f.facing();
    let mut input = f.hem_definition();
    input.method = HemMethod::Faced {
        facing: facing.id(),
    };
    let hem = f.hem(input.clone(), Some(&facing)).unwrap();
    let current = hem
        .facing(&f.piece, &f.ledger, Some(&facing))
        .unwrap()
        .unwrap();
    assert!(core::ptr::eq(current, &facing));
    assert_eq!(hem.definition(), &input);
    let mut changed = f.definition.clone();
    changed.material = MaterialAssignment::Assigned(EntityId::from_bits(205));
    let replaced = Facing::new(changed, &f.piece, &f.ledger).unwrap();
    assert_eq!(
        hem.facing(&f.piece, &f.ledger, Some(&replaced))
            .unwrap()
            .unwrap()
            .definition()
            .material,
        replaced.definition().material
    );
    assert_eq!(hem.definition(), &input);
}
#[test]
fn missing_and_replaced_facing_id_are_refused_at_birth_and_current_query() {
    let f = fixture();
    let facing = f.facing();
    let mut input = f.hem_definition();
    input.method = HemMethod::Faced {
        facing: facing.id(),
    };
    assert_eq!(
        f.hem(input.clone(), None),
        Err(HemError::MissingFacing(facing.id()))
    );
    let hem = f.hem(input.clone(), Some(&facing)).unwrap();
    assert_eq!(
        hem.facing(&f.piece, &f.ledger, None),
        Err(HemError::MissingFacing(facing.id()))
    );
    let mut changed = f.definition.clone();
    changed.id = EntityId::from_bits(206);
    let replacement = Facing::new(changed, &f.piece, &f.ledger).unwrap();
    let expected = HemError::WrongFacing {
        held: facing.id(),
        provided: replacement.id(),
    };
    assert_eq!(
        f.hem(input.clone(), Some(&replacement)),
        Err(expected.clone())
    );
    assert_eq!(
        hem.facing(&f.piece, &f.ledger, Some(&replacement)),
        Err(expected)
    );
}
#[test]
fn reassigned_facing_and_wrong_query_owner_are_refused_even_when_sources_exist() {
    let f = fixture();
    let facing = f.facing();
    let mut input = f.hem_definition();
    input.method = HemMethod::Faced {
        facing: facing.id(),
    };
    let hem = f.hem(input.clone(), Some(&facing)).unwrap();
    let mut piece_input = f.piece.definition().clone();
    piece_input.id = EntityId::from_bits(207);
    let other = Piece::new(piece_input, &f.ledger).unwrap();
    let mut changed = f.definition.clone();
    changed.served_piece = other.id();
    let reassigned = Facing::new(changed, &other, &f.ledger).unwrap();
    let expected = HemError::FacingOutsidePiece {
        facing: facing.id(),
        expected: f.piece.id(),
        served: other.id(),
    };
    assert_eq!(f.hem(input, Some(&reassigned)), Err(expected.clone()));
    assert_eq!(
        hem.facing(&f.piece, &f.ledger, Some(&reassigned)),
        Err(expected)
    );
    assert_eq!(
        hem.facing(&other, &f.ledger, Some(&facing)),
        Err(HemError::WrongPiece {
            held: f.piece.id(),
            provided: other.id()
        })
    );
}
#[test]
fn old_facing_is_revalidated_against_current_ledger_instead_of_reusing_birth_approval() {
    let mut f = fixture();
    let facing = f.facing();
    let mut input = f.hem_definition();
    input.method = HemMethod::Faced {
        facing: facing.id(),
    };
    let hem = f.hem(input.clone(), Some(&facing)).unwrap();
    let source = facing
        .definition()
        .offset
        .sources
        .first()
        .unwrap()
        .range
        .edge();
    let parts = f.ledger.split(&mut f.ids, source, t(1, 3)).unwrap();
    let tail = f.ledger.split(&mut f.ids, parts.second(), t(1, 2)).unwrap();
    f.ledger.delete(&mut f.ids, tail.first()).unwrap();
    let evidence = facing.source_resolutions(&f.ledger).next().unwrap().1;
    assert!(evidence.evidence().start().resolved().is_some());
    assert!(evidence.evidence().end().resolved().is_some());
    assert!(!evidence.evidence().has_full_coverage());
    let expected = HemError::InvalidFacing {
        facing: facing.id(),
        error: LayerError::UnresolvedSource {
            index: 0,
            evidence: Box::new(evidence.evidence().clone()),
        },
    };
    assert_eq!(
        hem.facing(&f.piece, &f.ledger, Some(&facing)),
        Err(expected.clone())
    );
    assert_eq!(f.hem(input, Some(&facing)), Err(expected));
}
#[test]
fn unknown_and_foreign_finish_edges_have_typed_evidence_without_replacement() {
    let mut f = fixture();
    let mut input = f.hem_definition();
    input.edge = EdgeRef::new(EntityId::from_bits(u128::MAX), LocalTag::FIRST);
    assert!(
        matches!(f.hem(input.clone(),None),Err(HemError::UnresolvedEdge(e)) if e.repairs().next().unwrap().issue()==RangeIssue::UnknownSource)
    );
    input.edge = *f
        .ledger
        .declare_edges(&mut f.ids, 1)
        .unwrap()
        .first()
        .unwrap();
    assert_eq!(
        f.hem(input, None),
        Err(HemError::OutsidePiece(f.piece.id()))
    );
}
#[test]
fn foreign_middle_of_finish_edge_is_refused_despite_owned_ends_after_reversal() {
    let mut f = fixture();
    let mut input = f.hem_definition();
    let right = f.definition.offset.sources.first().unwrap().range.edge();
    let foreign = *f
        .ledger
        .declare_edges(&mut f.ids, 1)
        .unwrap()
        .first()
        .unwrap();
    let unit = Length::from_micrometres(1).unwrap();
    let first = f
        .ledger
        .merge(&mut f.ids, input.edge, unit, foreign, unit)
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
    input.edge = merged;
    let evidence = f.ledger.resolve_range(EdgeRange::whole(merged));
    assert!(evidence.has_full_coverage());
    assert!(evidence.start().resolved().is_some());
    assert!(evidence.end().resolved().is_some());
    assert_eq!(
        f.hem(input.clone(), None),
        Err(HemError::OutsidePiece(f.piece.id()))
    );
    f.ledger.reverse(&mut f.ids, merged).unwrap();
    assert_eq!(
        f.hem(input, None),
        Err(HemError::OutsidePiece(f.piece.id()))
    );
}
#[test]
fn finish_queries_preserve_fragment_direction_and_interior_repairs_without_mutation() {
    let mut f = fixture();
    let input = f.hem_definition();
    let hem = f.hem(input.clone(), None).unwrap();
    let first = f.ledger.split(&mut f.ids, input.edge, t(1, 3)).unwrap();
    let tail = f.ledger.split(&mut f.ids, first.second(), t(1, 2)).unwrap();
    f.ledger.reverse(&mut f.ids, first.first()).unwrap();
    assert!(
        matches!(hem.edge_resolution(&f.ledger).portions().first().unwrap(),RangePortion::Resolved(r) if r.direction()==Direction::Reversed)
    );
    assert!(f.hem(input.clone(), None).is_ok());
    f.ledger.delete(&mut f.ids, tail.first()).unwrap();
    let evidence = hem.edge_resolution(&f.ledger);
    assert!(evidence.start().resolved().is_some());
    assert!(evidence.end().resolved().is_some());
    assert!(!evidence.has_full_coverage());
    assert_eq!(evidence.repairs().count(), 1);
    assert_eq!(
        f.hem(input.clone(), None),
        Err(HemError::UnresolvedEdge(Box::new(evidence)))
    );
    assert_eq!(hem.definition(), &input);
}
#[test]
fn editing_cloned_depth_fold_method_and_edge_cannot_mutate_validated_content() {
    let f = fixture();
    let input = f.hem_definition();
    let hem = f.hem(input.clone(), None).unwrap();
    let facing = f.facing();
    let mut changed = input.clone();
    changed.depth = HemDepth::Formula(EntityId::from_bits(208));
    changed.fold_type = HemFoldType::Profile(ProfileParameterRef::new(EntityId::from_bits(209)));
    changed.method = HemMethod::Faced {
        facing: facing.id(),
    };
    changed.edge = f.definition.offset.sources.first().unwrap().range.edge();
    let replacement = f.hem(changed, Some(&facing)).unwrap();
    assert_ne!(replacement.definition(), &input);
    assert_eq!(hem.definition(), &input);
}

#[test]
fn hem_invalid_facing_error_keeps_original_layer_cause() {
    use std::error::Error;
    let mut f = fixture();
    let facing = f.facing();
    let mut input = f.hem_definition();
    input.method = HemMethod::Faced {
        facing: facing.id(),
    };
    let hem = f.hem(input, Some(&facing)).unwrap();
    let source = facing
        .definition()
        .offset
        .sources
        .first()
        .unwrap()
        .range
        .edge();
    f.ledger.delete(&mut f.ids, source).unwrap();
    let issue = hem.facing(&f.piece, &f.ledger, Some(&facing)).unwrap_err();
    let HemError::InvalidFacing {
        facing: held,
        error: actual,
    } = &issue
    else {
        panic!("original current Facing layer error")
    };
    assert_eq!(*held, facing.id());
    assert!(matches!(
        actual,
        LayerError::UnresolvedSource { index: 0, .. }
    ));
    let cause = issue
        .source()
        .expect("original LayerError must remain in standard error chain");
    assert!(core::ptr::eq(
        cause
            .downcast_ref::<LayerError>()
            .expect("concrete layer cause"),
        actual
    ));
    assert!(!cause.is::<Box<LayerError>>());
}

#[test]
fn current_validation_does_not_let_healthy_facing_approve_a_damaged_finish_edge() {
    let mut f = fixture();
    let facing = f.facing();
    let mut input = f.hem_definition();
    input.method = HemMethod::Faced {
        facing: facing.id(),
    };
    let hem = f.hem(input.clone(), Some(&facing)).unwrap();
    let parts = f.ledger.split(&mut f.ids, input.edge, t(1, 3)).unwrap();
    let tail = f.ledger.split(&mut f.ids, parts.second(), t(1, 2)).unwrap();
    f.ledger.delete(&mut f.ids, tail.first()).unwrap();
    assert!(core::ptr::eq(
        hem.facing(&f.piece, &f.ledger, Some(&facing))
            .unwrap()
            .unwrap(),
        &facing
    ));
    let raw = hem.edge_resolution(&f.ledger);
    assert!(raw.start().resolved().is_some());
    assert!(raw.end().resolved().is_some());
    assert!(!raw.has_full_coverage());
    assert_eq!(
        hem.validate_current(&f.piece, &f.ledger, Some(&facing)),
        Err(HemError::UnresolvedEdge(Box::new(raw)))
    );
    assert_eq!(hem.definition(), &input);
}

#[test]
fn current_validation_follows_historical_finish_and_checks_original_owner_current_content() {
    let mut f = fixture();
    let input = f.hem_definition();
    let hem = f.hem(input.clone(), None).unwrap();
    let mut foreign = f.piece.definition().clone();
    foreign.id = EntityId::from_bits(900);
    let foreign = Piece::new(foreign, &f.ledger).unwrap();
    let parts = f.ledger.split(&mut f.ids, input.edge, t(1, 2)).unwrap();
    f.ledger.reverse(&mut f.ids, parts.first()).unwrap();
    hem.validate_current(&f.piece, &f.ledger, None).unwrap();
    assert_eq!(
        hem.validate_current(&foreign, &f.ledger, None),
        Err(HemError::WrongPiece {
            held: f.piece.id(),
            provided: foreign.id()
        })
    );
    let mut changed = f.piece.definition().clone();
    changed.boundary = vec![changed.construction_lines.remove(0)];
    let current = Piece::new(changed, &f.ledger).unwrap();
    assert!(hem.edge_resolution(&f.ledger).has_full_coverage());
    assert_eq!(
        hem.validate_current(&current, &f.ledger, None),
        Err(HemError::OutsidePiece(f.piece.id()))
    );
    assert_eq!(hem.definition(), &input);
}

#[test]
fn inventory_preserves_original_hem_order_identity_shared_objects_and_empty_scope() {
    let f = fixture();
    let facing = f.facing();
    let first = f.hem(f.hem_definition(), None).unwrap();
    let mut input = f.hem_definition();
    input.id = EntityId::from_bits(210);
    input.depth = HemDepth::Explicit {
        parameter: EntityId::from_bits(211),
        value: Length::from_micrometres(30000).unwrap(),
    };
    input.method = HemMethod::Faced {
        facing: facing.id(),
    };
    let second = f.hem(input, Some(&facing)).unwrap();
    let inventory = HemInventory::new(vec![second.clone(), first.clone()]).unwrap();
    assert_eq!(inventory.hems(), &[second.clone(), first.clone()]);
    assert!(core::ptr::eq(
        inventory.hem(second.id()).unwrap(),
        inventory.hems().first().unwrap()
    ));
    assert!(inventory.hem(EntityId::from_bits(999)).is_none());
    assert_eq!(
        HemInventory::new(vec![first.clone(), second.clone(), first.clone()]),
        Err(HemInventoryError::DuplicateIdentity {
            identity: first.id(),
            first: 0,
            duplicate: 2
        })
    );
    let layers = LayerInventory::new(vec![facing], vec![], vec![]).unwrap();
    inventory
        .validate_current(&layers, std::slice::from_ref(&f.piece), &f.ledger)
        .unwrap();
    assert_eq!(
        second.geometric_validation(),
        GeometricValidation::DeferredToG2
    );
    let empty = HemInventory::new(vec![]).unwrap();
    assert!(empty.hems().is_empty());
    empty
        .validate_current(
            &LayerInventory::new(vec![], vec![], vec![]).unwrap(),
            &[],
            &f.ledger,
        )
        .unwrap();
}

#[test]
fn inventory_keeps_original_hem_owner_and_refuses_ambiguous_current_pieces_even_when_empty() {
    let f = fixture();
    let hem = f.hem(f.hem_definition(), None).unwrap();
    let inventory = HemInventory::new(vec![hem.clone()]).unwrap();
    let layers = LayerInventory::new(vec![], vec![], vec![]).unwrap();
    let mut input = f.piece.definition().clone();
    input.id = EntityId::from_bits(901);
    let other = Piece::new(input, &f.ledger).unwrap();
    assert_eq!(
        inventory.validate_current(&layers, std::slice::from_ref(&other), &f.ledger),
        Err(HemInventoryError::MissingPiece {
            index: 0,
            hem: hem.id(),
            piece: f.piece.id()
        })
    );
    for collection in [&inventory, &HemInventory::new(vec![]).unwrap()] {
        assert_eq!(
            collection.validate_current(&layers, &[f.piece.clone(), f.piece.clone()], &f.ledger),
            Err(HemInventoryError::Layers(Box::new(
                LayerInventoryError::DuplicatePiece(f.piece.id())
            )))
        );
    }
}

#[test]
fn inventory_facing_joins_never_substitute_peers_other_layer_kinds_or_reassigned_owners() {
    let f = fixture();
    let facing = f.facing();
    let mut input = f.hem_definition();
    input.method = HemMethod::Faced {
        facing: facing.id(),
    };
    let hem = f.hem(input, Some(&facing)).unwrap();
    let inventory = HemInventory::new(vec![hem.clone()]).unwrap();
    let mut input = f.definition.clone();
    input.id = EntityId::from_bits(101);
    let peer = Facing::new(input, &f.piece, &f.ledger).unwrap();
    let same_id_other_kind = Interfacing::new(f.definition.clone(), &f.piece, &f.ledger).unwrap();
    for layers in [
        LayerInventory::new(vec![peer], vec![], vec![]).unwrap(),
        LayerInventory::new(vec![], vec![], vec![same_id_other_kind]).unwrap(),
    ] {
        assert_eq!(
            inventory.validate_current(&layers, std::slice::from_ref(&f.piece), &f.ledger),
            Err(HemInventoryError::Hem {
                index: 0,
                hem: hem.id(),
                issue: Box::new(HemError::MissingFacing(facing.id()))
            })
        );
    }
    let mut input = f.piece.definition().clone();
    input.id = EntityId::from_bits(901);
    let other = Piece::new(input, &f.ledger).unwrap();
    let mut input = f.definition.clone();
    input.served_piece = other.id();
    let reassigned = Facing::new(input, &other, &f.ledger).unwrap();
    let layers = LayerInventory::new(vec![reassigned], vec![], vec![]).unwrap();
    assert_eq!(
        inventory.validate_current(&layers, &[f.piece.clone(), other.clone()], &f.ledger),
        Err(HemInventoryError::Hem {
            index: 0,
            hem: hem.id(),
            issue: Box::new(HemError::FacingOutsidePiece {
                facing: facing.id(),
                expected: f.piece.id(),
                served: other.id()
            })
        })
    );
}

#[test]
fn inventory_checks_unused_layer_sources_even_without_hems_and_keeps_concrete_cause_chain() {
    use std::error::Error;
    let mut f = fixture();
    let unused = f.facing();
    let hem = f.hem(f.hem_definition(), None).unwrap();
    let hems = HemInventory::new(vec![hem.clone()]).unwrap();
    let source = f.definition.offset.sources.first().unwrap().range.edge();
    f.ledger.delete(&mut f.ids, source).unwrap();
    hem.validate_current(&f.piece, &f.ledger, None).unwrap();
    let layers = LayerInventory::new(vec![unused.clone()], vec![], vec![]).unwrap();
    for inventory in [&hems, &HemInventory::new(vec![]).unwrap()] {
        let issue = inventory
            .validate_current(&layers, std::slice::from_ref(&f.piece), &f.ledger)
            .unwrap_err();
        let HemInventoryError::Layers(actual) = &issue else {
            panic!("whole current layer cause")
        };
        let LayerInventoryError::Layer {
            layer,
            issue: original,
            ..
        } = actual.as_ref()
        else {
            panic!("original layer cause")
        };
        assert_eq!(*layer, unused.id());
        assert!(matches!(
            original.as_ref(),
            LayerError::UnresolvedSource { index: 0, .. }
        ));
        let cause = issue.source().unwrap();
        assert!(core::ptr::eq(
            cause.downcast_ref::<LayerInventoryError>().unwrap(),
            actual.as_ref()
        ));
        assert!(!cause.is::<Box<LayerInventoryError>>());
        assert!(core::ptr::eq(
            cause
                .source()
                .unwrap()
                .downcast_ref::<LayerError>()
                .unwrap(),
            original.as_ref()
        ));
    }
}

#[test]
fn inventory_checks_later_hem_full_finish_despite_live_endpoints_and_retains_held_damage() {
    use std::error::Error;
    let mut f = fixture();
    let first = f.hem(f.hem_definition(), None).unwrap();
    let edge = *f
        .ledger
        .declare_edges(&mut f.ids, 1)
        .unwrap()
        .first()
        .unwrap();
    let mut input = f.piece.definition().clone();
    input.id = EntityId::from_bits(901);
    input.boundary = vec![DirectedEdge {
        edge,
        direction: Direction::Original,
    }];
    input.construction_lines.clear();
    let other = Piece::new(input, &f.ledger).unwrap();
    let mut input = f.hem_definition();
    input.id = EntityId::from_bits(210);
    input.edge = edge;
    input.depth = HemDepth::Explicit {
        parameter: EntityId::from_bits(211),
        value: Length::from_micrometres(30000).unwrap(),
    };
    let later = Hem::new(input.clone(), &other, &f.ledger, None).unwrap();
    let parts = f.ledger.split(&mut f.ids, edge, t(1, 3)).unwrap();
    let tail = f.ledger.split(&mut f.ids, parts.second(), t(1, 2)).unwrap();
    f.ledger.delete(&mut f.ids, tail.first()).unwrap();
    let inventory = HemInventory::new(vec![first.clone(), later.clone()]).unwrap();
    first.validate_current(&f.piece, &f.ledger, None).unwrap();
    let raw = later.edge_resolution(&f.ledger);
    assert!(raw.start().resolved().is_some());
    assert!(raw.end().resolved().is_some());
    assert!(!raw.has_full_coverage());
    let layers = LayerInventory::new(vec![], vec![], vec![]).unwrap();
    let issue = inventory
        .validate_current(&layers, &[f.piece.clone(), other], &f.ledger)
        .unwrap_err();
    let HemInventoryError::Hem {
        index,
        hem,
        issue: actual,
    } = &issue
    else {
        panic!("later original Hem cause")
    };
    assert_eq!((*index, *hem), (1, later.id()));
    assert_eq!(actual.as_ref(), &HemError::UnresolvedEdge(Box::new(raw)));
    assert!(core::ptr::eq(
        issue.source().unwrap().downcast_ref::<HemError>().unwrap(),
        actual.as_ref()
    ));
    assert!(!issue.source().unwrap().is::<Box<HemError>>());
    assert_eq!(inventory.hem(later.id()).unwrap().definition(), &input);
}
