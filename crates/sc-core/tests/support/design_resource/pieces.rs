//! Actual held Piece/copy-plan resources; no geometry or registry substitutes.
use super::{limits, Error, Location, Meter, Scope, Unit};
use sc_core::{
    design::{charge_cut_plan_resources, charge_piece_inventory_resources, DesignResourceSink},
    ontology::{
        CopyOrientation, CutCopyDefinition, CutPlan, CuttingSide, DeterministicIdGenerator,
        DirectedEdge, Direction, EntityId, IdentityLedger, LabelText, MaterialAssignment,
        Mirroring, Param, Piece, PieceDefinition, PieceInventory, Rational,
    },
};

pub(super) fn fixture() -> (
    PieceInventory,
    CutPlan,
    IdentityLedger,
    DeterministicIdGenerator,
) {
    let mut ids = DeterministicIdGenerator::new();
    let mut ledger = IdentityLedger::new();
    let edges = ledger
        .declare_edges(&mut ids, 7)
        .expect("actual declared edge identities");
    let directed: Vec<_> = edges
        .iter()
        .copied()
        .map(|edge| DirectedEdge {
            edge,
            direction: Direction::Original,
        })
        .collect();
    let fold = *edges.first().expect("actual boundary fold");
    let first = PieceDefinition {
        id: EntityId::from_bits(10000),
        boundary: directed.iter().take(3).copied().collect(),
        holes: vec![
            directed.iter().skip(3).take(1).copied().collect(),
            directed.iter().skip(4).take(2).copied().collect(),
        ],
        construction_lines: directed.iter().skip(6).copied().collect(),
        quantity: 1,
        mirroring: Mirroring::Single,
        cut_on_fold: true,
        fold_edges: vec![fold],
        cutting_side: CuttingSide::Face,
        material: MaterialAssignment::Unresolved(" é ".into()),
        layer_index: 0,
        label: LabelText {
            name: "A".into(),
            size: "M".into(),
            fabric: "猫".into(),
            colorway: "é".into(),
        },
    };
    let mut second = first.clone();
    second.id = EntityId::from_bits(10001);
    second.label.name = "B".into();
    second.material = MaterialAssignment::Assigned(EntityId::from_bits(99999));
    let pieces = PieceInventory::new(vec![
        Piece::new(first, &ledger).expect("actual structurally born Piece"),
        Piece::new(second, &ledger)
            .expect("actual second Piece, assigned registry validation remains separate"),
    ])
    .expect("actual authored inventory");
    let copies = pieces
        .pieces()
        .iter()
        .zip(11000_u128..)
        .map(|(piece, id)| CutCopyDefinition {
            id: EntityId::from_bits(id),
            piece: piece.id(),
            orientation: CopyOrientation::Authored,
        })
        .collect();
    let plan =
        CutPlan::new(copies, pieces.pieces()).expect("one explicit actual physical copy per Piece");
    (pieces, plan, ledger, ids)
}
fn location(field: &'static str, owner: Option<u128>, index: Option<usize>) -> Location {
    Location {
        scope: Scope::Structure,
        owner: owner.map(EntityId::from_bits),
        field,
        index,
    }
}

#[test]
fn actual_piece_and_cut_plan_fields_count_exact_collections_utf8_and_shared_logical_text_without_changes(
) {
    let (pieces, plan, _, _) = fixture();
    let original_pieces = pieces.clone();
    let original_plan = plan.clone();
    let mut meter = Meter::new(limits(24, 18, 1));
    charge_piece_inventory_resources(&pieces, &mut meter)
        .expect("two actual Pieces and all held fields");
    charge_cut_plan_resources(&plan, &mut meter).expect("two actual copies");
    let demand = meter.finish().expect("exact combined component demand");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (24, 18, 0)
    );
    assert_eq!(pieces, original_pieces);
    assert_eq!(plan, original_plan);
}
#[test]
fn nested_hole_refusal_retains_second_actual_piece_owner_and_second_original_hole_index() {
    let (pieces, _, _, _) = fixture();
    let mut meter = Meter::new(limits(18, 18, 1));
    let original = Error::Count {
        unit: Unit::CollectionEntries,
        maximum: 18,
        consumed: 18,
        requested: 2,
        location: location("pieces.holes.edges", Some(10001), Some(1)),
    };
    assert_eq!(
        charge_piece_inventory_resources(&pieces, &mut meter),
        Err(original)
    );
    assert_eq!(meter.finish(), Err(original));
}
#[test]
fn exact_utf8_label_refusal_keeps_original_piece_owner_and_does_not_trim_material_reason() {
    let (pieces, _, _, _) = fixture();
    let mut meter = Meter::new(limits(24, 17, 1));
    let original = Error::Count {
        unit: Unit::TextBytes,
        maximum: 17,
        consumed: 16,
        requested: 2,
        location: location("pieces.label.colorway", Some(10001), Some(1)),
    };
    assert_eq!(
        charge_piece_inventory_resources(&pieces, &mut meter),
        Err(original)
    );
    assert_eq!(meter.finish(), Err(original));
}
#[test]
fn held_reference_accounting_after_real_split_does_not_rebirth_repair_or_recopy_piece_or_plan() {
    let (pieces, plan, mut ledger, mut ids) = fixture();
    let piece = pieces.pieces().first().expect("actual first Piece");
    let old = piece
        .definition()
        .boundary
        .first()
        .expect("actual held boundary")
        .edge;
    ledger
        .split(
            &mut ids,
            old,
            Param::new(Rational::new(1, 2).expect("exact half")).expect("valid parameter"),
        )
        .expect("actual later topology edit");
    assert!(!ledger.is_live(old));
    Piece::new(piece.definition().clone(), &ledger)
        .expect_err("rebirth would incorrectly reject retired held source");
    let saved_pieces = pieces.clone();
    let saved_plan = plan.clone();
    let saved_ledger = ledger.clone();
    let mut meter = Meter::new(limits(24, 18, 1));
    charge_piece_inventory_resources(&pieces, &mut meter)
        .expect("accounting inspects held Piece without rebirth");
    charge_cut_plan_resources(&plan, &mut meter).expect("original physical-copy values");
    assert_eq!(
        meter
            .finish()
            .expect("same actual content")
            .collection_entries(),
        24
    );
    assert_eq!(pieces, saved_pieces);
    assert_eq!(plan, saved_plan);
    assert_eq!(ledger, saved_ledger);
    let mut limited = Meter::new(limits(23, 18, 1));
    charge_piece_inventory_resources(&pieces, &mut limited).expect("22 Piece entries");
    let original = Error::Count {
        unit: Unit::CollectionEntries,
        maximum: 23,
        consumed: 22,
        requested: 2,
        location: location("cut_plan.copies", None, None),
    };
    assert_eq!(
        charge_cut_plan_resources(&plan, &mut limited),
        Err(original)
    );
    assert_eq!(limited.finish(), Err(original));
}
#[test]
fn actual_empty_piece_and_copy_scopes_preserve_the_same_prior_refusal() {
    let pieces = PieceInventory::new(vec![]).expect("explicit empty actual inventory");
    let plan = CutPlan::new(vec![], pieces.pieces()).expect("explicit empty actual plan");
    let mut meter = Meter::new(limits(1, 1, 1));
    charge_piece_inventory_resources(&pieces, &mut meter).expect("empty Piece scope");
    charge_cut_plan_resources(&plan, &mut meter).expect("empty copy scope");
    let demand = meter.finish().expect("empty actual components");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (0, 0, 0)
    );
    let mut refused = Meter::new(limits(1, 1, 1));
    let original = refused
        .charge_text("é", location("preceding", Some(10000), None))
        .expect_err("actual first prior charge refusal");
    assert_eq!(
        charge_piece_inventory_resources(&pieces, &mut refused),
        Err(original)
    );
    assert_eq!(
        charge_cut_plan_resources(&plan, &mut refused),
        Err(original)
    );
    assert_eq!(refused.finish(), Err(original));
}
