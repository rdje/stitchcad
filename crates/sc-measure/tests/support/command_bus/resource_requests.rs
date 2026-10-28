//! Actual immutable request objects, including unsupported fixed intents and exact retry payload copies.
use super::group_commit::{group, mutation};
use super::resource_admission::{counts, reset};
use super::*;
use sc_core::{
    command::{
        charge_command_mutation_group_resources, AddNotch, AssignMaterial, ChangeMeasurement,
        CloseDart, CommandMutationGroup, CommandOperationLimits, PieceSelection, SplitSeamSpan,
    },
    design::{
        DesignResourceCount as Unit, DesignResourceLimitError as Error, DesignResourceLimits,
        DesignResourceLocation as Location, DesignResourceMeter as Meter,
        DesignResourceScope as Scope, DesignResourceSink,
    },
    ontology::{EdgeAnchor, MaterialAssignment, Param},
    value::{LengthDeclaration, LengthState},
};
use std::num::{NonZeroU64, NonZeroUsize};
fn limits(entries: usize, text: usize, bits: u64) -> DesignResourceLimits {
    DesignResourceLimits::new(
        NonZeroUsize::new(entries).expect("positive entry budget"),
        NonZeroUsize::new(text).expect("positive byte budget"),
        NonZeroU64::new(bits).expect("positive bit budget"),
    )
}
fn location(field: &'static str, index: Option<usize>) -> Location {
    Location {
        scope: Scope::CommandState,
        owner: Some(id(808000)),
        field,
        index,
    }
}
fn fixture() -> (CommandMutationGroup, [Rc<Cell<usize>>; 3]) {
    let owner = super::resource_measurement::fixture();
    let counters = owner.inputs().sources().host().telemetry();
    let canonical = owner
        .inputs()
        .sources()
        .measurement_declaration(id(1), owner.inputs().inputs())
        .expect("actual bound Known2 replacement")
        .clone();
    let pieces = owner.inputs().structure().pieces().pieces();
    let first = pieces.first().expect("actual first Piece");
    let second = pieces.get(1).expect("actual second Piece");
    let anchor = EdgeAnchor {
        edge: first.edges().next().expect("actual first edge").edge,
        param: Param::START,
    };
    let span = owner
        .sewing()
        .graph()
        .spans()
        .first()
        .expect("actual authored sewing span")
        .id();
    let dart = owner
        .constructions()
        .darts()
        .first()
        .expect("actual authored Dart")
        .id();
    let revision = DesignRevision::from_counter(3);
    let design = owner.inputs().id();
    let measurement = mutation(
        design,
        revision.clone(),
        808000,
        808001,
        MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(1), canonical)),
    );
    let request = group(
        808000,
        vec![
            measurement.clone(),
            mutation(
                design,
                revision.clone(),
                808000,
                808002,
                MutationPayload::AssignMaterial(
                    AssignMaterial::new(
                        PieceSelection::new(vec![second.id(), first.id()])
                            .expect("actual reversed ordered selection"),
                        MaterialAssignment::Unresolved(" é ".into()),
                    )
                    .expect("actual nonblank UTF-8 reason"),
                ),
            ),
            mutation(
                design,
                revision.clone(),
                808000,
                808003,
                MutationPayload::AddNotch(AddNotch::new(
                    first.id(),
                    anchor,
                    notch_preview::bindings(),
                )),
            ),
            mutation(
                design,
                revision.clone(),
                808000,
                808004,
                MutationPayload::SplitSeamSpan(SplitSeamSpan::new(span, anchor, anchor)),
            ),
            mutation(
                design,
                revision,
                808000,
                808005,
                MutationPayload::CloseDart(CloseDart::new(dart)),
            ),
            measurement,
        ],
    );
    reset(&counters);
    (request, counters)
}
#[test]
fn exact_owned_group_counts_each_context_and_alias_but_reason_only_once_per_payload() {
    let (request, counters) = fixture();
    let original = request.clone();
    let mut meter = Meter::new(limits(12, 4, 14));
    charge_command_mutation_group_resources(&request, &mut meter)
        .expect("six members + evidence2 twice + selection2");
    let demand = meter.finish().expect("actual complete owned request data");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (12, 4, 14)
    );
    let operation = CommandOperationLimits::new(
        NonZeroUsize::MAX,
        NonZeroUsize::MAX,
        NonZeroUsize::MAX,
        NonZeroUsize::MAX,
    )
    .check_group(&request)
    .expect("separate declared work demand");
    assert_eq!(
        operation.reason_bytes(),
        8,
        "proposed Piece copies differ from one owned request string"
    );
    assert_eq!(
        operation.evidence_references(),
        4,
        "alias owns another actual evidence payload"
    );
    assert_eq!(request, original);
    assert_eq!(
        counts(&counters),
        [0, 0, 0],
        "no model/source/auth callbacks"
    );
}
#[test]
fn request_first_refusals_keep_group_owner_schema_order_and_actual_member_positions() {
    let (request, counters) = fixture();
    for (policy, expected) in [
        (
            limits(12, 4, 1),
            Error::CounterBits {
                maximum: 1,
                consumed: 0,
                requested: 2,
                location: location("requests.group.context.revision", None),
            },
        ),
        (
            limits(5, 4, 14),
            Error::Count {
                unit: Unit::CollectionEntries,
                maximum: 5,
                consumed: 0,
                requested: 6,
                location: location("requests.mutations", None),
            },
        ),
        (
            limits(7, 4, 14),
            Error::Count {
                unit: Unit::CollectionEntries,
                maximum: 7,
                consumed: 6,
                requested: 2,
                location: location("requests.mutations.measurement.evidence", Some(0)),
            },
        ),
        (
            limits(9, 4, 14),
            Error::Count {
                unit: Unit::CollectionEntries,
                maximum: 9,
                consumed: 8,
                requested: 2,
                location: location("requests.mutations.material.pieces", Some(1)),
            },
        ),
        (
            limits(11, 4, 14),
            Error::Count {
                unit: Unit::CollectionEntries,
                maximum: 11,
                consumed: 10,
                requested: 2,
                location: location("requests.mutations.measurement.evidence", Some(5)),
            },
        ),
        (
            limits(12, 3, 14),
            Error::Count {
                unit: Unit::TextBytes,
                maximum: 3,
                consumed: 0,
                requested: 4,
                location: location("requests.mutations.material.reason", Some(1)),
            },
        ),
        (
            limits(12, 4, 13),
            Error::CounterBits {
                maximum: 13,
                consumed: 12,
                requested: 2,
                location: location("requests.mutations.context.revision", Some(5)),
            },
        ),
    ] {
        let original = request.clone();
        let mut meter = Meter::new(policy);
        assert_eq!(
            charge_command_mutation_group_resources(&request, &mut meter),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
        assert_eq!(request, original);
    }
    assert_eq!(counts(&counters), [0, 0, 0]);
}
#[test]
fn exact_request_data_composes_prior_units_and_never_clears_a_latched_error() {
    let (request, counters) = fixture();
    let prior = location("earlier", None);
    let mut meter = Meter::new(limits(14, 6, 15));
    meter.charge_entries(2, prior).expect("prior entries");
    meter.charge_text("é", prior).expect("prior bytes");
    meter.charge_counter_bits(1, prior).expect("prior bits");
    charge_command_mutation_group_resources(&request, &mut meter).expect("same caller sink");
    let demand = meter.finish().expect("full prefix+request demand");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (14, 6, 15)
    );
    let mut refused = Meter::new(limits(1, 1, 1));
    let first = refused
        .charge_entries(2, prior)
        .expect_err("original refusal");
    assert_eq!(
        charge_command_mutation_group_resources(&request, &mut refused),
        Err(first)
    );
    assert_eq!(refused.finish(), Err(first));
    assert_eq!(counts(&counters), [0, 0, 0]);
}
#[test]
fn fixed_intents_and_unknown_replacement_add_no_inferred_text_evidence_or_counter_bits() {
    let (request, counters) = fixture();
    let mut members = Vec::new();
    for (index, original) in request.mutations().iter().skip(2).take(3).enumerate() {
        members.push(mutation(
            request.context().design(),
            DesignRevision::initial(),
            808010,
            808011 + index as u128,
            original.payload().clone(),
        ));
    }
    let fixed = group(808010, members);
    let mut meter = Meter::new(limits(3, 1, 1));
    charge_command_mutation_group_resources(&fixed, &mut meter)
        .expect("only fixed intents at zero revision");
    let demand = meter.finish().expect("three fixed payload entries");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (3, 0, 0)
    );
    let original = match request
        .mutations()
        .first()
        .expect("actual first request")
        .payload()
    {
        MutationPayload::ChangeMeasurement(original) => Some(original),
        _ => None,
    }
    .expect("expected actual Measurement payload");
    let mut definition = original.replacement().definition().clone();
    definition.state = LengthState::Unknown {
        observation: id(808099),
    };
    let unknown = group(
        808000,
        vec![mutation(
            request.context().design(),
            DesignRevision::initial(),
            808000,
            808001,
            MutationPayload::ChangeMeasurement(ChangeMeasurement::new(
                original.measurement(),
                LengthDeclaration::new(definition).expect("actual explicit unknown snapshot"),
            )),
        )],
    );
    let mut meter = Meter::new(limits(1, 1, 1));
    charge_command_mutation_group_resources(&unknown, &mut meter)
        .expect("unknown has no owned evidence");
    let demand = meter.finish().expect("one actual request entry only");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (1, 0, 0)
    );
    assert_eq!(counts(&counters), [0, 0, 0]);
}
