//! Actual six-edit topology ledger/resource composition, without fake journal/model states.
use super::{limits, Error, Location, Meter, Scope, Unit};
use sc_core::{
    design::{charge_topology_resources, DesignResourceSink},
    ontology::{
        DeterministicIdGenerator, EntityId, IdentityLedger, OffsetInterval, Param, Rational,
        SplitSide,
    },
};
use sc_units::Length;
fn parameter(numerator: i64, denominator: i64) -> Param {
    Param::new(Rational::new(numerator, denominator).expect("exact valid ratio"))
        .expect("inside unit interval")
}
fn fixture() -> (IdentityLedger, EntityId, EntityId) {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let edges = ledger
        .declare_edges(&mut ids, 5)
        .expect("actual declared edges");
    let mut edges = edges.into_iter();
    let a = edges.next().expect("split source");
    let b = edges.next().expect("merge source");
    let c = edges.next().expect("offset source");
    let d = edges.next().expect("delete source");
    let e = edges.next().expect("unused but still live edge");
    let declared = ledger
        .journal()
        .first()
        .expect("actual declaration entry")
        .operation();
    let half = parameter(1, 2);
    ledger
        .register(EntityId::from_bits(50000), a, half)
        .expect("actual registered split-point reference");
    ledger
        .state_split_choice(EntityId::from_bits(50000), a, half, SplitSide::Second)
        .expect("actual retained choice");
    ledger
        .register(EntityId::from_bits(50001), d, parameter(1, 4))
        .expect("actual later orphaned reference");
    let fragments = ledger.split(&mut ids, a, half).expect("actual split");
    let merged = ledger
        .merge(
            &mut ids,
            fragments.first(),
            Length::from_micrometres(100).expect("positive length"),
            b,
            Length::from_micrometres(200).expect("positive length"),
        )
        .expect("actual merge");
    ledger
        .reverse(&mut ids, merged)
        .expect("actual direction edit");
    ledger
        .offset(
            &mut ids,
            c,
            &[
                OffsetInterval::new(Param::START, parameter(1, 4)).expect("first interval"),
                OffsetInterval::new(parameter(1, 4), parameter(3, 4)).expect("middle interval"),
                OffsetInterval::new(parameter(3, 4), Param::END).expect("last interval"),
            ],
        )
        .expect("actual three-fragment offset");
    let offset = ledger
        .journal()
        .last()
        .expect("actual offset entry")
        .operation();
    ledger.delete(&mut ids, d).expect("actual orphaning edit");
    assert!(!ledger.is_live(a));
    assert!(!ledger.is_live(d));
    assert!(ledger.is_live(e));
    (ledger, declared, offset)
}
fn location(field: &'static str, owner: Option<EntityId>, index: Option<usize>) -> Location {
    Location {
        scope: Scope::Structure,
        owner,
        field,
        index,
    }
}
fn refusal(ledger: &IdentityLedger, maximum: usize, expected: Error) {
    let saved = ledger.clone();
    let mut meter = Meter::new(limits(maximum, 1, 1));
    assert_eq!(charge_topology_resources(ledger, &mut meter), Err(expected));
    assert_eq!(meter.finish(), Err(expected));
    assert_eq!(ledger, &saved);
}
#[test]
fn all_actual_edit_variants_and_stored_indices_charge_exact_content_without_changing_history_or_choices(
) {
    let (ledger, _, _) = fixture();
    assert_eq!(ledger.journal().len(), 6);
    assert_eq!(ledger.live_edges().count(), 6);
    assert_eq!(ledger.registrations().count(), 2);
    assert!(ledger
        .registrations()
        .any(|entry| entry.split_choice() == Some(SplitSide::Second)));
    let saved = ledger.clone();
    let mut meter = Meter::new(limits(22, 1, 1));
    charge_topology_resources(&ledger, &mut meter).expect(
        "six journal entries, five declared edges, three fragments, six live, two registrations",
    );
    let demand = meter.finish().expect("exact actual component demand");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (22, 0, 0)
    );
    assert_eq!(ledger, saved);
}
#[test]
fn journal_and_nested_vector_first_limits_keep_actual_operation_owner_and_original_entry_index() {
    let (ledger, declared, offset) = fixture();
    refusal(
        &ledger,
        5,
        Error::Count {
            unit: Unit::CollectionEntries,
            maximum: 5,
            consumed: 0,
            requested: 6,
            location: location("topology.journal", None, None),
        },
    );
    refusal(
        &ledger,
        10,
        Error::Count {
            unit: Unit::CollectionEntries,
            maximum: 10,
            consumed: 6,
            requested: 5,
            location: location("topology.journal.declared.edges", Some(declared), Some(0)),
        },
    );
    refusal(
        &ledger,
        13,
        Error::Count {
            unit: Unit::CollectionEntries,
            maximum: 13,
            consumed: 11,
            requested: 3,
            location: location("topology.journal.offset.fragments", Some(offset), Some(4)),
        },
    );
}
#[test]
fn maintained_live_and_registered_reference_fields_refuse_their_exact_stored_lengths() {
    let (ledger, _, _) = fixture();
    refusal(
        &ledger,
        19,
        Error::Count {
            unit: Unit::CollectionEntries,
            maximum: 19,
            consumed: 14,
            requested: 6,
            location: location("topology.live_edges", None, None),
        },
    );
    refusal(
        &ledger,
        21,
        Error::Count {
            unit: Unit::CollectionEntries,
            maximum: 21,
            consumed: 20,
            requested: 2,
            location: location("topology.registrations", None, None),
        },
    );
}
#[test]
fn actual_empty_topology_scope_counts_zero_and_cannot_clear_a_prior_meter_refusal() {
    let ledger = IdentityLedger::new();
    let mut meter = Meter::new(limits(1, 1, 1));
    charge_topology_resources(&ledger, &mut meter).expect("actual empty topology");
    let demand = meter.finish().expect("empty actual component");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (0, 0, 0)
    );
    let mut meter = Meter::new(limits(1, 1, 1));
    let original = meter
        .charge_text("é", location("preceding", None, None))
        .expect_err("first earlier refusal");
    assert_eq!(
        charge_topology_resources(&ledger, &mut meter),
        Err(original)
    );
    assert_eq!(meter.finish(), Err(original));
}
