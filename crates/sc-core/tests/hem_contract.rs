//! Hem contracts: authored intent and stable current Facing composition.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
use sc_core::ontology::{
    CuttingSide, DeterministicIdGenerator, DirectedEdge, DirectedRange, Direction, EdgeRange,
    EdgeRef, EntityId, Facing, GeometricValidation, Hem, HemDefinition, HemDepth, HemError,
    HemFoldType, HemMethod, IdGenerator, IdentityLedger, LabelText, LayerDefinition, LayerError,
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
