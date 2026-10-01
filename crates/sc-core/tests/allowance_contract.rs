//! Per-edge descriptor contracts; no fabricated offsets or profile policies.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
use sc_core::ontology::{
    AllowanceWidth, CornerTreatment, CuttingSide, DeterministicIdGenerator, DirectedEdge,
    Direction, EdgeRange, EdgeRef, EntityId, GeometricValidation, IdGenerator, IdentityLedger,
    LabelText, LocalTag, MaterialAssignment, Mirroring, Param, Piece, PieceDefinition,
    ProfileBindingValidation, ProfileParameterRef, RangeIssue, RangePortion, Rational,
    SeamAllowance, SeamAllowanceDefinition, SeamAllowanceError,
};
use sc_units::Length;
fn t(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).unwrap()).unwrap()
}
struct Fixture {
    piece: Piece,
    ledger: IdentityLedger,
    ids: DeterministicIdGenerator,
    definition: SeamAllowanceDefinition,
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
    Fixture {
        piece,
        ledger,
        ids,
        definition: SeamAllowanceDefinition {
            id: EntityId::from_bits(100),
            edge: *edges.first().unwrap(),
            width: AllowanceWidth::Explicit {
                parameter: EntityId::from_bits(101),
                value: Length::from_micrometres(10_000).unwrap(),
            },
            corner: CornerTreatment::Miter,
            inclusion: ProfileParameterRef::new(EntityId::from_bits(102)),
        },
    }
}

impl Fixture {
    fn allowance(&self) -> Result<SeamAllowance, SeamAllowanceError> {
        SeamAllowance::new(self.definition.clone(), &self.piece, &self.ledger)
    }
}
#[test]
fn authored_width_origin_corner_and_profile_inclusion_are_retained_without_physical_claims() {
    let f = fixture();
    let sa = f.allowance().unwrap();
    assert_eq!(sa.id(), f.definition.id);
    assert_eq!(sa.piece(), f.piece.id());
    assert_eq!(sa.definition(), &f.definition);
    assert_eq!(
        sa.profile_binding_validation(),
        ProfileBindingValidation::DeferredToG4
    );
    assert_eq!(sa.geometric_validation(), GeometricValidation::DeferredToG2);
    assert_eq!(
        sa.edge_resolution(&f.ledger),
        f.ledger.resolve_range(EdgeRange::whole(f.definition.edge))
    );
}
#[test]
fn all_five_corner_choices_are_explicit_and_no_choice_constructs_an_offset() {
    let mut f = fixture();
    for corner in [
        CornerTreatment::Miter,
        CornerTreatment::Slant,
        CornerTreatment::Envelope,
        CornerTreatment::Trim,
        CornerTreatment::Step,
    ] {
        f.definition.corner = corner;
        let sa = f.allowance().unwrap();
        assert_eq!(sa.definition().corner, corner);
        assert_eq!(sa.geometric_validation(), GeometricValidation::DeferredToG2);
    }
}
#[test]
fn explicit_width_accepts_zero_but_refuses_negative_without_treating_symbols_as_zero() {
    let mut f = fixture();
    let parameter = EntityId::from_bits(201);
    for micrometres in [0, 1, 30_000] {
        f.definition.width = AllowanceWidth::Explicit {
            parameter,
            value: Length::from_micrometres(micrometres).unwrap(),
        };
        assert_eq!(
            f.allowance().unwrap().definition().width,
            f.definition.width
        );
    }
    let negative = Length::from_micrometres(-1).unwrap();
    f.definition.width = AllowanceWidth::Explicit {
        parameter,
        value: negative,
    };
    assert_eq!(
        f.allowance(),
        Err(SeamAllowanceError::NegativeWidth(negative))
    );
    for width in [
        AllowanceWidth::Formula(parameter),
        AllowanceWidth::Profile(ProfileParameterRef::new(parameter)),
    ] {
        f.definition.width = width;
        let sa = f.allowance().unwrap();
        assert_eq!(sa.definition().width, width);
        assert_eq!(
            sa.profile_binding_validation(),
            ProfileBindingValidation::DeferredToG4
        );
    }
}
#[test]
fn unknown_and_live_foreign_edges_are_different_typed_refusals() {
    let mut f = fixture();
    f.definition.edge = EdgeRef::new(EntityId::from_bits(u128::MAX), LocalTag::FIRST);
    assert!(
        matches!(f.allowance(),Err(SeamAllowanceError::UnresolvedEdge(e)) if e.repairs().next().unwrap().issue()==RangeIssue::UnknownSource)
    );
    f.definition.edge = *f
        .ledger
        .declare_edges(&mut f.ids, 1)
        .unwrap()
        .first()
        .unwrap();
    assert_eq!(
        f.allowance(),
        Err(SeamAllowanceError::OutsidePiece(f.piece.id()))
    );
}
#[test]
fn split_reverse_and_merge_preserve_whole_source_edge_evidence_without_rewriting_descriptor() {
    let mut f = fixture();
    let sa = f.allowance().unwrap();
    let edge = f.definition.edge;
    let parts = f.ledger.split(&mut f.ids, edge, t(1, 4)).unwrap();
    assert_eq!(
        sa.edge_resolution(&f.ledger)
            .portions()
            .iter()
            .filter_map(|part| match part {
                RangePortion::Resolved(range) => Some(range),
                RangePortion::Unresolved(_) => None,
            })
            .count(),
        2
    );
    f.ledger.reverse(&mut f.ids, parts.first()).unwrap();
    let result = sa.edge_resolution(&f.ledger);
    let ranges = result
        .portions()
        .iter()
        .filter_map(|part| match part {
            RangePortion::Resolved(range) => Some(range),
            RangePortion::Unresolved(_) => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(ranges.first().unwrap().direction(), Direction::Reversed);
    assert_eq!(ranges.get(1).unwrap().direction(), Direction::Original);
    assert!(f.allowance().is_ok());
    let foreign = *f
        .ledger
        .declare_edges(&mut f.ids, 1)
        .unwrap()
        .first()
        .unwrap();
    let merged = f
        .ledger
        .merge(
            &mut f.ids,
            parts.second(),
            Length::from_micrometres(3).unwrap(),
            foreign,
            Length::from_micrometres(7).unwrap(),
        )
        .unwrap();
    f.ledger.reverse(&mut f.ids, merged).unwrap();
    assert!(f.allowance().is_ok());
    assert_eq!(sa.definition(), &f.definition);
    f.definition.edge = merged;
    assert_eq!(
        f.allowance(),
        Err(SeamAllowanceError::OutsidePiece(f.piece.id()))
    );
}
#[test]
fn deletion_of_middle_fragment_is_visible_even_with_live_original_endpoints() {
    let mut f = fixture();
    let sa = f.allowance().unwrap();
    let edge = f.definition.edge;
    let first = f.ledger.split(&mut f.ids, edge, t(1, 3)).unwrap();
    let second = f.ledger.split(&mut f.ids, first.second(), t(1, 2)).unwrap();
    f.ledger.delete(&mut f.ids, second.first()).unwrap();
    let evidence = sa.edge_resolution(&f.ledger);
    assert!(evidence.start().resolved().is_some());
    assert!(evidence.end().resolved().is_some());
    assert!(!evidence.has_full_coverage());
    assert_eq!(evidence.repairs().count(), 1);
    assert!(matches!(f.allowance(),Err(SeamAllowanceError::UnresolvedEdge(e)) if *e==evidence));
    assert!(matches!(
        evidence.portions().get(1).unwrap(),
        RangePortion::Unresolved(_)
    ));
}
#[test]
fn per_edge_descriptors_can_share_a_width_origin_without_caching_target_profile_policy() {
    let f = fixture();
    let first = f.allowance().unwrap();
    let mut input = f.definition.clone();
    input.id = EntityId::from_bits(202);
    input.edge = f
        .piece
        .definition()
        .construction_lines
        .first()
        .unwrap()
        .edge;
    let second = SeamAllowance::new(input.clone(), &f.piece, &f.ledger).unwrap();
    assert_ne!(first.id(), second.id());
    assert_ne!(first.definition().edge, second.definition().edge);
    assert_eq!(first.definition().width, second.definition().width);
    assert_eq!(first.definition().inclusion, second.definition().inclusion);
    input.inclusion = ProfileParameterRef::new(EntityId::from_bits(203));
    input.corner = CornerTreatment::Envelope;
    let replacement = SeamAllowance::new(input, &f.piece, &f.ledger).unwrap();
    assert_ne!(replacement.definition(), second.definition());
    assert_eq!(second.definition().inclusion, f.definition.inclusion);
    assert_eq!(
        replacement.profile_binding_validation(),
        ProfileBindingValidation::DeferredToG4
    );
}
#[test]
fn a_merged_edge_with_owned_endpoints_and_foreign_middle_is_not_an_owned_allowance_edge() {
    let mut f = fixture();
    let left = f.definition.edge;
    let right = f
        .piece
        .definition()
        .construction_lines
        .first()
        .unwrap()
        .edge;
    let middle = *f
        .ledger
        .declare_edges(&mut f.ids, 1)
        .unwrap()
        .first()
        .unwrap();
    let unit = Length::from_micrometres(1).unwrap();
    let first = f
        .ledger
        .merge(&mut f.ids, left, unit, middle, unit)
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
    f.definition.edge = merged;
    let evidence = f.ledger.resolve_range(EdgeRange::whole(merged));
    assert!(evidence.has_full_coverage());
    assert!(evidence.start().resolved().is_some());
    assert!(evidence.end().resolved().is_some());
    assert_eq!(
        f.allowance(),
        Err(SeamAllowanceError::OutsidePiece(f.piece.id()))
    );
    f.ledger.reverse(&mut f.ids, merged).unwrap();
    assert_eq!(
        f.allowance(),
        Err(SeamAllowanceError::OutsidePiece(f.piece.id()))
    );
}
