//! Physical notion placements: stable copy domains, current references and no retargeting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
use sc_core::ontology::{
    AnchorError, CopyOrientation, CutCopyDefinition, CutPlan, CuttingSide,
    DeterministicIdGenerator, DirectedEdge, DirectedRange, Direction, EdgeAnchor, EdgeRange,
    EdgeRef, EntityId, GeometricValidation, IdGenerator, IdentityLedger, LabelText, LocalTag,
    MaterialAssignment, Mirroring, NotionPlacement, NotionPlacementDefinition,
    NotionPlacementError, Param, Piece, PieceDefinition, RangeIssue, Rational, Resolution,
};
use sc_units::Length;
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
            id: ids.next_id(),
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
#[test]
fn two_copies_can_have_distinct_notion_placements_without_pattern_geometry_duplication() {
    let f = fixture();
    let plan = f.plan();
    let first = f.placement(f.placement_input()).unwrap();
    let mut input = f.placement_input();
    input.id = EntityId::from_bits(701);
    input.copy = EntityId::from_bits(601);
    input.anchor.param = t(3, 4);
    input.direction.direction = Direction::Reversed;
    let second = f.placement(input).unwrap();
    assert_ne!(first.id(), second.id());
    assert_ne!(first.copy(), second.copy());
    assert_eq!(first.piece(), second.piece());
    assert_eq!(second.definition(), &input);
    assert_eq!(
        plan.copy(first.copy()).unwrap().orientation(),
        CopyOrientation::Authored
    );
    assert_eq!(
        plan.copy(second.copy()).unwrap().orientation(),
        CopyOrientation::Reflected
    );
    assert_eq!(
        second
            .direction_resolution(&f.ledger)
            .portions()
            .next()
            .unwrap()
            .direction,
        Direction::Reversed
    );
    assert_eq!(
        second.geometric_validation(),
        GeometricValidation::DeferredToG2
    );
    first.validate_current(&plan, &f.piece, &f.ledger).unwrap();
    second.validate_current(&plan, &f.piece, &f.ledger).unwrap();
}
#[test]
fn plan_reordering_preserves_identity_and_removed_copy_cannot_retarget_to_its_peer() {
    let f = fixture();
    let placement = f.placement(f.placement_input()).unwrap();
    let mut copies = f
        .plan()
        .copies()
        .iter()
        .map(|c| *c.definition())
        .collect::<Vec<_>>();
    copies.reverse();
    let reordered = CutPlan::new(copies.clone(), std::slice::from_ref(&f.piece)).unwrap();
    placement
        .validate_current(&reordered, &f.piece, &f.ledger)
        .unwrap();
    copies
        .iter_mut()
        .find(|c| c.id == placement.copy())
        .unwrap()
        .id = EntityId::from_bits(602);
    let replaced = CutPlan::new(copies, std::slice::from_ref(&f.piece)).unwrap();
    assert!(replaced.copy(EntityId::from_bits(601)).is_some());
    assert_eq!(
        placement.validate_current(&replaced, &f.piece, &f.ledger),
        Err(NotionPlacementError::MissingCopy(placement.copy()))
    );
    let mut missing = f.placement_input();
    missing.copy = EntityId::from_bits(999);
    assert_eq!(
        f.placement(missing),
        Err(NotionPlacementError::MissingCopy(missing.copy))
    );
}
#[test]
fn copy_source_reassignment_and_wrong_provided_piece_are_typed_refusals() {
    let f = fixture();
    let placement = f.placement(f.placement_input()).unwrap();
    let mut input = f.piece.definition().clone();
    input.id = EntityId::from_bits(900);
    let other = Piece::new(input, &f.ledger).unwrap();
    let copies = f
        .plan()
        .copies()
        .iter()
        .map(|c| CutCopyDefinition {
            piece: other.id(),
            ..*c.definition()
        })
        .collect();
    let reassigned = CutPlan::new(copies, std::slice::from_ref(&other)).unwrap();
    let expected = NotionPlacementError::CopyOutsidePiece {
        copy: placement.copy(),
        expected: f.piece.id(),
        actual: other.id(),
    };
    assert_eq!(
        placement.validate_current(&reassigned, &f.piece, &f.ledger),
        Err(expected.clone())
    );
    assert_eq!(
        NotionPlacement::new(*placement.definition(), &reassigned, &f.piece, &f.ledger),
        Err(expected)
    );
    assert_eq!(
        placement.validate_current(&reassigned, &other, &f.ledger),
        Err(NotionPlacementError::WrongPiece {
            held: f.piece.id(),
            provided: other.id()
        })
    );
}
#[test]
fn birth_requires_live_owned_unique_anchors_with_no_implicit_split_choice() {
    let mut f = fixture();
    let mut input = f.placement_input();
    let missing = EdgeRef::new(EntityId::from_bits(u128::MAX), LocalTag::FIRST);
    input.anchor.edge = missing;
    assert_eq!(
        f.placement(input),
        Err(NotionPlacementError::Anchor(AnchorError::MissingEdge {
            edge: missing
        }))
    );
    let foreign = *f
        .ledger
        .declare_edges(&mut f.ids, 1)
        .unwrap()
        .first()
        .unwrap();
    input.anchor.edge = foreign;
    assert_eq!(
        f.placement(input),
        Err(NotionPlacementError::Anchor(
            AnchorError::AnchorOutsidePiece {
                piece: f.piece.id(),
                anchor: input.anchor
            }
        ))
    );
    input = f.placement_input();
    let source = input.anchor.edge;
    f.ledger.split(&mut f.ids, source, t(1, 2)).unwrap();
    input.anchor.param = t(1, 2);
    assert_eq!(
        f.placement(input),
        Err(NotionPlacementError::Anchor(AnchorError::MissingEdge {
            edge: source
        }))
    );
}
#[test]
fn current_validation_follows_historical_anchors_but_reports_current_choice_and_repairs() {
    let mut f = fixture();
    let input = f.placement_input();
    let original = f.placement(input).unwrap();
    let mut at_split = input;
    at_split.id = EntityId::from_bits(702);
    at_split.anchor.param = t(1, 2);
    let ambiguous = f.placement(at_split).unwrap();
    let plan = f.plan();
    let parts = f
        .ledger
        .split(&mut f.ids, input.anchor.edge, t(1, 2))
        .unwrap();
    assert!(!f.ledger.is_live(input.anchor.edge));
    original
        .validate_current(&plan, &f.piece, &f.ledger)
        .unwrap();
    assert!(original.anchor_resolution(&f.ledger).resolved().is_some());
    assert!(
        matches!(ambiguous.validate_current(&plan,&f.piece,&f.ledger),Err(NotionPlacementError::Anchor(AnchorError::CurrentUnresolved{resolution,..})) if matches!(*resolution,Resolution::SplitPoint{..}))
    );
    f.ledger.reverse(&mut f.ids, parts.first()).unwrap();
    original
        .validate_current(&plan, &f.piece, &f.ledger)
        .unwrap();
    f.ledger.delete(&mut f.ids, parts.first()).unwrap();
    assert!(
        matches!(original.validate_current(&plan,&f.piece,&f.ledger),Err(NotionPlacementError::Anchor(AnchorError::CurrentUnresolved{resolution,..})) if *resolution==original.anchor_resolution(&f.ledger))
    );
    assert_eq!(original.definition(), &input);
}
#[test]
fn direction_requires_full_owned_intervals_and_unique_endpoints() {
    let mut f = fixture();
    let mut input = f.placement_input();
    input.direction.range = EdgeRange::whole(EdgeRef::new(
        EntityId::from_bits(u128::MAX),
        LocalTag::FIRST,
    ));
    assert!(
        matches!(f.placement(input),Err(NotionPlacementError::UnresolvedDirection(e)) if e.repairs().next().unwrap().issue()==RangeIssue::UnknownSource)
    );
    let foreign = *f
        .ledger
        .declare_edges(&mut f.ids, 1)
        .unwrap()
        .first()
        .unwrap();
    input.direction.range = EdgeRange::whole(foreign);
    assert_eq!(
        f.placement(input),
        Err(NotionPlacementError::DirectionOutsidePiece(f.piece.id()))
    );
    input = f.placement_input();
    let source = input.direction.range.edge();
    f.ledger.split(&mut f.ids, source, t(1, 2)).unwrap();
    input.direction.range = EdgeRange::new(source, t(1, 2), Param::END).unwrap();
    assert!(
        matches!(f.placement(input),Err(NotionPlacementError::UnresolvedDirection(e)) if e.has_full_coverage() && e.start().resolved().is_none())
    );
}
#[test]
fn foreign_middle_of_direction_is_refused_despite_owned_ends_after_reversal() {
    let mut f = fixture();
    let mut input = f.placement_input();
    let right = f.piece.definition().construction_lines.get(1).unwrap().edge;
    let foreign = *f
        .ledger
        .declare_edges(&mut f.ids, 1)
        .unwrap()
        .first()
        .unwrap();
    let unit = Length::from_micrometres(1).unwrap();
    let first = f
        .ledger
        .merge(
            &mut f.ids,
            input.direction.range.edge(),
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
    input.direction.range = EdgeRange::whole(merged);
    let e = f.ledger.resolve_range(input.direction.range);
    assert!(e.has_full_coverage());
    assert!(e.start().resolved().is_some());
    assert!(e.end().resolved().is_some());
    assert_eq!(
        f.placement(input),
        Err(NotionPlacementError::DirectionOutsidePiece(f.piece.id()))
    );
    f.ledger.reverse(&mut f.ids, merged).unwrap();
    assert_eq!(
        f.placement(input),
        Err(NotionPlacementError::DirectionOutsidePiece(f.piece.id()))
    );
}
#[test]
fn raw_direction_retains_authored_order_and_repairs_with_live_endpoints() {
    let mut f = fixture();
    let mut input = f.placement_input();
    input.direction.direction = Direction::Reversed;
    let placement = f.placement(input).unwrap();
    let first = f
        .ledger
        .split(&mut f.ids, input.direction.range.edge(), t(1, 3))
        .unwrap();
    let tail = f.ledger.split(&mut f.ids, first.second(), t(1, 2)).unwrap();
    f.ledger.reverse(&mut f.ids, tail.second()).unwrap();
    f.ledger.delete(&mut f.ids, tail.first()).unwrap();
    let e = placement.direction_resolution(&f.ledger);
    assert_eq!(e.portions().next().unwrap().direction, Direction::Original);
    assert!(e.evidence().start().resolved().is_some());
    assert!(e.evidence().end().resolved().is_some());
    assert!(!e.evidence().has_full_coverage());
    assert_eq!(e.evidence().repairs().count(), 1);
    assert!(
        matches!(placement.validate_current(&f.plan(),&f.piece,&f.ledger),Err(NotionPlacementError::UnresolvedDirection(raw)) if *raw==*e.evidence())
    );
    assert_eq!(placement.definition(), &input);
}
#[test]
fn cloned_input_cannot_mutate_validated_copy_anchor_or_orientation() {
    let f = fixture();
    let input = f.placement_input();
    let placement = f.placement(input).unwrap();
    let mut changed = input;
    changed.id = EntityId::from_bits(703);
    changed.copy = EntityId::from_bits(601);
    changed.anchor.param = t(3, 4);
    changed.direction.direction = Direction::Reversed;
    let replacement = f.placement(changed).unwrap();
    assert_eq!(placement.definition(), &input);
    assert_ne!(replacement.definition(), placement.definition());
}
#[test]
fn current_anchor_ownership_is_checked_against_updated_piece_content_not_its_id_alone() {
    let f = fixture();
    let placement = f.placement(f.placement_input()).unwrap();
    let mut changed = f.piece.definition().clone();
    changed.boundary = vec![changed.construction_lines.remove(1)];
    let current = Piece::new(changed, &f.ledger).unwrap();
    assert_eq!(current.id(), f.piece.id());
    assert!(placement.anchor_resolution(&f.ledger).resolved().is_some());
    assert_eq!(
        placement.validate_current(&f.plan(), &current, &f.ledger),
        Err(NotionPlacementError::Anchor(
            AnchorError::AnchorOutsidePiece {
                piece: current.id(),
                anchor: placement.definition().anchor,
            }
        ))
    );
}
