//! Actual layer/Hem content preserves modelled envelopes and held damaged sources.
use super::{limits, Error, Location, Meter, Scope, Unit};
use sc_core::{
    design::{
        charge_hem_inventory_resources, charge_layer_inventory_resources, DesignResourceSink,
    },
    ontology::{
        CuttingSide, DeterministicIdGenerator, DirectedEdge, DirectedRange, Direction, EdgeRange,
        EntityId, Facing, Hem, HemDefinition, HemDepth, HemFoldType, HemInventory, HemMethod,
        IdentityLedger, Interfacing, LabelText, LayerDefinition, LayerInventory,
        LayerOffsetRelationship, Lining, MaterialAssignment, Mirroring, Piece, PieceDefinition,
        ProfileParameterRef,
    },
};
use sc_units::Length;
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
struct Fixture {
    layers: LayerInventory,
    hems: HemInventory,
    piece: Piece,
    ledger: IdentityLedger,
    ids: DeterministicIdGenerator,
}
fn fixture() -> Fixture {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let edges = ledger
        .declare_edges(&mut ids, 2)
        .expect("actual owned sources");
    let boundary = *edges.first().expect("actual boundary");
    let internal = *edges.last().expect("actual construction edge");
    let piece = Piece::new(
        PieceDefinition {
            id: id(100),
            boundary: vec![DirectedEdge {
                edge: boundary,
                direction: Direction::Original,
            }],
            holes: vec![],
            construction_lines: vec![DirectedEdge {
                edge: internal,
                direction: Direction::Original,
            }],
            quantity: 1,
            mirroring: Mirroring::Single,
            cut_on_fold: false,
            fold_edges: vec![],
            cutting_side: CuttingSide::Face,
            material: MaterialAssignment::Unresolved("selection pending".into()),
            layer_index: 0,
            label: LabelText {
                name: "served".into(),
                size: "M".into(),
                fabric: "woven".into(),
                colorway: "plain".into(),
            },
        },
        &ledger,
    )
    .expect("actual born served Piece");
    let source = |edge| DirectedRange {
        range: EdgeRange::whole(edge),
        direction: Direction::Reversed,
    };
    let definition = |identity, sources, material| LayerDefinition {
        id: id(identity),
        served_piece: piece.id(),
        offset: LayerOffsetRelationship {
            operation: id(900 + identity),
            sources,
        },
        material,
    };
    let first = Facing::new(
        definition(
            300,
            vec![source(boundary)],
            MaterialAssignment::Unresolved(" é ".into()),
        ),
        &piece,
        &ledger,
    )
    .expect("actual Facing reason4 bytes");
    let second = Facing::new(
        definition(
            301,
            vec![source(boundary), source(internal)],
            MaterialAssignment::Assigned(id(999)),
        ),
        &piece,
        &ledger,
    )
    .expect("actual assigned Facing; registry remains separate");
    let lining = Lining::new(
        definition(
            302,
            vec![source(boundary), source(internal)],
            MaterialAssignment::Unresolved("猫".into()),
        ),
        &piece,
        &ledger,
    )
    .expect("actual modelled Lining");
    let interfacing = Interfacing::new(
        definition(
            303,
            vec![source(internal)],
            MaterialAssignment::Unresolved("é".into()),
        ),
        &piece,
        &ledger,
    )
    .expect("actual Interfacing reason2 bytes");
    let hem = |identity, depth, fold_type, method, facing| {
        Hem::new(
            HemDefinition {
                id: id(identity),
                edge: boundary,
                depth,
                fold_type,
                method,
            },
            &piece,
            &ledger,
            facing,
        )
        .expect("actual authored Hem")
    };
    let hems = HemInventory::new(vec![
        hem(
            400,
            HemDepth::Explicit {
                parameter: id(950),
                value: Length::ZERO,
            },
            HemFoldType::Declaration(id(951)),
            HemMethod::Turned,
            None,
        ),
        hem(
            401,
            HemDepth::Formula(id(952)),
            HemFoldType::Profile(ProfileParameterRef::new(id(953))),
            HemMethod::Faced { facing: first.id() },
            Some(&first),
        ),
        hem(
            402,
            HemDepth::Profile(ProfileParameterRef::new(id(954))),
            HemFoldType::Declaration(id(955)),
            HemMethod::Faced {
                facing: second.id(),
            },
            Some(&second),
        ),
    ])
    .expect("actual Hem inventory");
    let layers = LayerInventory::new(vec![first, second], vec![lining], vec![interfacing])
        .expect("actual three layer families");
    Fixture {
        layers,
        hems,
        piece,
        ledger,
        ids,
    }
}
fn charge(f: &Fixture, meter: &mut Meter) -> Result<(), Error> {
    charge_layer_inventory_resources(&f.layers, meter)?;
    charge_hem_inventory_resources(&f.hems, meter)
}
fn location(field: &'static str, owner: Option<u128>, index: Option<usize>) -> Location {
    Location {
        scope: Scope::Garment,
        owner: owner.map(id),
        field,
        index,
    }
}
#[test]
fn actual_layer_sources_reason_bytes_and_hems_count_once_without_changing_lining_scope() {
    let f = fixture();
    let original = (f.layers.clone(), f.hems.clone());
    assert!(f.layers.require_in_scope().is_err());
    let mut meter = Meter::new(limits(13, 9, 1));
    charge(&f, &mut meter).expect("4 layers,6 sources,3 Hems,9 reason bytes");
    let demand = meter.finish().expect("exact represented content");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (13, 9, 0)
    );
    assert!(f.layers.require_in_scope().is_err());
    assert_eq!((f.layers, f.hems), original);
}
#[test]
fn every_layer_family_nested_sources_and_hem_vector_refuse_with_original_counts_and_owner_index() {
    let f = fixture();
    for (maximum, consumed, requested, field, owner, index) in [
        (1, 0, 2, "layers.facings", None, None),
        (4, 3, 2, "layers.offset.sources", Some(301), Some(1)),
        (5, 5, 1, "layers.linings", None, None),
        (7, 6, 2, "layers.offset.sources", Some(302), Some(0)),
        (8, 8, 1, "layers.interfacings", None, None),
        (9, 9, 1, "layers.offset.sources", Some(303), Some(0)),
        (12, 10, 3, "hems", None, None),
    ] {
        let original = (f.layers.clone(), f.hems.clone());
        let expected = Error::Count {
            unit: Unit::CollectionEntries,
            maximum,
            consumed,
            requested,
            location: location(field, owner, index),
        };
        let mut meter = Meter::new(limits(maximum, 9, 1));
        assert_eq!(charge(&f, &mut meter), Err(expected));
        assert_eq!(meter.finish(), Err(expected));
        assert_eq!((&f.layers, &f.hems), (&original.0, &original.1));
    }
}
#[test]
fn layer_material_reasons_count_exact_utf8_without_trimming_or_assigned_lookup() {
    let f = fixture();
    for (maximum, consumed, requested, owner, index) in
        [(3, 0, 4, 300, 0), (6, 4, 3, 302, 0), (8, 7, 2, 303, 0)]
    {
        let mut meter = Meter::new(limits(13, maximum, 1));
        let expected = Error::Count {
            unit: Unit::TextBytes,
            maximum,
            consumed,
            requested,
            location: location("layers.material.reason", Some(owner), Some(index)),
        };
        assert_eq!(charge(&f, &mut meter), Err(expected));
        assert_eq!(meter.finish(), Err(expected));
    }
}
#[test]
fn accounting_actual_layers_and_hems_after_deletion_preserves_held_sources_without_repair() {
    let mut f = fixture();
    let edge = f
        .piece
        .definition()
        .boundary
        .first()
        .expect("held boundary")
        .edge;
    f.ledger.delete(&mut f.ids, edge).expect("actual deletion");
    assert!(f
        .layers
        .validate_current(std::slice::from_ref(&f.piece), &f.ledger)
        .is_err());
    let original = (f.layers.clone(), f.hems.clone(), f.ledger.clone());
    let mut meter = Meter::new(limits(13, 9, 1));
    charge(&f, &mut meter).expect("held accounting needs no current source validation");
    assert_eq!(
        meter
            .finish()
            .expect("held resource demand")
            .collection_entries(),
        13
    );
    assert_eq!((f.layers, f.hems, f.ledger), original);
}
#[test]
fn empty_layers_and_hems_are_zero_and_both_producers_keep_prior_refusal() {
    let layers = LayerInventory::new(vec![], vec![], vec![]).expect("empty modelled layers");
    let hems = HemInventory::new(vec![]).expect("empty Hems");
    let mut meter = Meter::new(limits(1, 1, 1));
    charge_layer_inventory_resources(&layers, &mut meter).expect("empty layer families");
    charge_hem_inventory_resources(&hems, &mut meter).expect("empty Hems");
    let demand = meter.finish().expect("empty actual scope");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (0, 0, 0)
    );
    let mut refused = Meter::new(limits(1, 1, 1));
    let first = Error::Count {
        unit: Unit::CollectionEntries,
        maximum: 1,
        consumed: 0,
        requested: 2,
        location: location("original", None, None),
    };
    assert_eq!(
        refused.charge_entries(2, location("original", None, None)),
        Err(first)
    );
    let f = fixture();
    assert_eq!(
        charge_layer_inventory_resources(&f.layers, &mut refused),
        Err(first)
    );
    assert_eq!(
        charge_hem_inventory_resources(&f.hems, &mut refused),
        Err(first)
    );
    assert_eq!(
        charge_layer_inventory_resources(&layers, &mut refused),
        Err(first)
    );
    assert_eq!(
        charge_hem_inventory_resources(&hems, &mut refused),
        Err(first)
    );
    assert_eq!(refused.finish(), Err(first));
}
