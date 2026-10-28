//! Actual structural composition and exact revision bits, without an invented source host.
use super::{limits, Error, Location, Meter, Scope, Unit};
use sc_core::{
    command::DesignRevision,
    design::{
        charge_design_structure_resources, DesignResourceSink, DesignStructure,
        DesignStructureDefinition,
    },
    ontology::{CutPlan, EntityId, IdentityLedger, PieceInventory},
    value::{
        CanonicalInputs, LengthDeclaration, LengthDeclarationDefinition, LengthState,
        ScalarInputDeclaration, ScalarInputDeclarationDefinition, ScalarInputKind,
        ScalarInputState,
    },
};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn fixture(revision: DesignRevision) -> DesignStructure {
    let (pieces, cut_plan, topology, _) = super::pieces::fixture();
    let inputs = CanonicalInputs::new(
        vec![LengthDeclaration::new(LengthDeclarationDefinition {
            id: id(200),
            source: id(201),
            state: LengthState::Derived { formula: id(202) },
        })
        .expect("actual derived length declaration")],
        vec![
            ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                id: id(203),
                source: id(201),
                kind: ScalarInputKind::Length,
                state: ScalarInputState::Unknown {
                    observation: id(204),
                },
            })
            .expect("actual unknown scalar declaration"),
        ],
    )
    .expect("actual canonical owner");
    DesignStructure::new(DesignStructureDefinition {
        id: id(50000),
        revision,
        inputs,
        pieces,
        cut_plan,
        topology,
    })
    .expect("actual complete structural owner")
}
fn location(field: &'static str, owner: Option<u128>, index: Option<usize>) -> Location {
    Location {
        scope: Scope::Structure,
        owner: owner.map(id),
        field,
        index,
    }
}
#[test]
fn actual_structure_composes_canonical_piece_copy_and_topology_resources_under_exact_revision_bits()
{
    let structure = fixture(DesignRevision::from_counter(u128::MAX).successor());
    let original = structure.clone();
    let mut meter = Meter::new(limits(41, 18, 129));
    charge_design_structure_resources(&structure, &mut meter)
        .expect("canonical2+Pieces22+copies2+topology15, text18, revision129");
    let demand = meter.finish().expect("exact actual structural scope");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (41, 18, 129)
    );
    assert_eq!(structure, original);
    assert_eq!(
        structure.revision().to_string(),
        "340282366920938463463374607431768211456"
    );
}
#[test]
fn structural_first_refusals_keep_revision_design_owner_and_nested_component_counts() {
    let structure = fixture(DesignRevision::from_counter(u128::MAX).successor());
    for (policy, expected) in [
        (
            limits(41, 18, 128),
            Error::CounterBits {
                maximum: 128,
                consumed: 0,
                requested: 129,
                location: location("design.revision", Some(50000), None),
            },
        ),
        (
            limits(25, 18, 129),
            Error::Count {
                unit: Unit::CollectionEntries,
                maximum: 25,
                consumed: 24,
                requested: 2,
                location: location("cut_plan.copies", None, None),
            },
        ),
        (
            limits(40, 18, 129),
            Error::Count {
                unit: Unit::CollectionEntries,
                maximum: 40,
                consumed: 34,
                requested: 7,
                location: location("topology.live_edges", None, None),
            },
        ),
        (
            limits(41, 17, 129),
            Error::Count {
                unit: Unit::TextBytes,
                maximum: 17,
                consumed: 16,
                requested: 2,
                location: location("pieces.label.colorway", Some(10001), Some(1)),
            },
        ),
    ] {
        let original = structure.clone();
        let mut meter = Meter::new(policy);
        assert_eq!(
            charge_design_structure_resources(&structure, &mut meter),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
        assert_eq!(structure, original);
    }
}
#[test]
fn actual_revision_magnitude_zero_machine_boundary_and_successors_remain_exact() {
    let beyond = DesignRevision::from_counter(u128::MAX).successor();
    for (revision, bits) in [
        (DesignRevision::initial(), 0),
        (DesignRevision::from_counter(1), 1),
        (DesignRevision::from_counter(2), 2),
        (DesignRevision::from_counter(3), 2),
        (DesignRevision::from_counter(u128::MAX), 128),
        (beyond.clone(), 129),
        (beyond.successor(), 129),
    ] {
        let original = revision.clone();
        let structure = fixture(revision);
        let mut meter = Meter::new(limits(41, 18, bits.max(1)));
        charge_design_structure_resources(&structure, &mut meter)
            .expect("exact represented magnitude");
        assert_eq!(
            meter.finish().expect("represented bits").counter_bits(),
            bits
        );
        assert_eq!(structure.revision(), &original);
    }
}
#[test]
fn structural_accounting_composes_with_prior_units_and_preserves_first_counter_refusal() {
    let structure = fixture(DesignRevision::from_counter(u128::MAX).successor());
    for maximum in [135, 136] {
        let mut meter = Meter::new(limits(44, 20, maximum));
        meter
            .charge_entries(3, location("earlier.entries", None, None))
            .expect("prior entries");
        meter
            .charge_text("é", location("earlier.text", None, None))
            .expect("prior UTF8 bytes");
        meter
            .charge_counter_bits(7, location("earlier.counter", None, None))
            .expect("prior represented counter");
        let result = charge_design_structure_resources(&structure, &mut meter);
        if maximum == 135 {
            let expected = Error::CounterBits {
                maximum,
                consumed: 7,
                requested: 129,
                location: location("design.revision", Some(50000), None),
            };
            assert_eq!(result, Err(expected));
            assert_eq!(meter.finish(), Err(expected));
        } else {
            result.expect("exact cumulative limits");
            let demand = meter.finish().expect("all original cumulative units");
            assert_eq!(
                (
                    demand.collection_entries(),
                    demand.text_bytes(),
                    demand.counter_bits()
                ),
                (44, 20, 136)
            );
        }
    }
}
#[test]
fn empty_actual_structure_has_zero_magnitude_and_cannot_clear_prior_error() {
    let empty = DesignStructure::new(DesignStructureDefinition {
        id: id(50000),
        revision: DesignRevision::initial(),
        inputs: CanonicalInputs::new(vec![], vec![]).expect("empty canonical inputs"),
        pieces: PieceInventory::new(vec![]).expect("empty Pieces"),
        cut_plan: CutPlan::new(vec![], &[]).expect("empty actual plan"),
        topology: IdentityLedger::new(),
    })
    .expect("actual structural draft");
    let mut meter = Meter::new(limits(1, 1, 1));
    charge_design_structure_resources(&empty, &mut meter).expect("empty structural fields");
    let demand = meter.finish().expect("zero represented demand");
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
    assert_eq!(
        charge_design_structure_resources(&empty, &mut refused),
        Err(first)
    );
    assert_eq!(
        charge_design_structure_resources(&fixture(DesignRevision::initial()), &mut refused),
        Err(first)
    );
    assert_eq!(refused.finish(), Err(first));
}
