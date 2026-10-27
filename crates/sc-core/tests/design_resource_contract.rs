//! Logical resource accounting, without an invented model traversal or runtime admission result.
#![allow(clippy::expect_used)]
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::design::{
    DesignResourceCount as Unit, DesignResourceLimitError as Error, DesignResourceLimits as Limits,
    DesignResourceLocation as Location, DesignResourceMeter as Meter, DesignResourceScope as Scope,
    DesignResourceSink,
};
fn limits(entries: usize, text: usize, bits: u64) -> Limits {
    Limits::new(
        NonZeroUsize::new(entries).expect("explicit positive entry bound"),
        NonZeroUsize::new(text).expect("explicit positive text bound"),
        NonZeroU64::new(bits).expect("explicit positive bit bound"),
    )
}
fn location(field: &'static str, index: Option<usize>) -> Location {
    Location {
        scope: Scope::SourceHost,
        field,
        index,
    }
}

#[test]
fn cumulative_collection_utf8_and_counter_units_retain_exact_bounds_and_empty_charges() {
    let policy = limits(3, 5, 129);
    let mut meter = Meter::new(policy);
    let sink: &mut dyn DesignResourceSink = &mut meter;
    sink.charge_entries(2, location("records", None))
        .expect("first collection");
    sink.charge_entries(1, location("references", Some(0)))
        .expect("nested reference");
    sink.charge_text("é", location("label", Some(0)))
        .expect("two UTF-8 bytes");
    sink.charge_text("猫", location("label", Some(1)))
        .expect("three UTF-8 bytes");
    sink.charge_counter_bits(128, location("revision", None))
        .expect("first actual magnitude count");
    sink.charge_counter_bits(1, location("nested.revision", Some(0)))
        .expect("one further bit");
    sink.charge_entries(0, location("empty", None))
        .expect("empty collection");
    sink.charge_text("", location("empty.label", None))
        .expect("empty text");
    sink.charge_counter_bits(0, location("zero.revision", None))
        .expect("zero magnitude");
    let demand = meter.finish().expect("exact independent maxima");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (3, 5, 129)
    );
    assert_eq!(policy.collection_entries().get(), 3);
    assert_eq!(policy.text_bytes().get(), 5);
    assert_eq!(policy.counter_bits().get(), 129);
}

#[test]
fn collection_refusal_keeps_original_location_and_cannot_be_cleared_by_ignored_or_zero_charges() {
    let mut meter = Meter::new(limits(3, 10, 100));
    meter
        .charge_entries(2, location("first", Some(4)))
        .expect("preceding actual charge");
    let first = Error::Count {
        unit: Unit::CollectionEntries,
        maximum: 3,
        consumed: 2,
        requested: 2,
        location: location("nested.references", Some(7)),
    };
    assert_eq!(
        meter.charge_entries(2, location("nested.references", Some(7))),
        Err(first)
    );
    assert_eq!(
        meter.charge_text("later", location("later", None)),
        Err(first)
    );
    assert_eq!(
        meter.charge_counter_bits(0, location("zero", None)),
        Err(first)
    );
    assert_eq!(meter.charge_entries(0, location("empty", None)), Err(first));
    assert_eq!(
        meter.finish(),
        Err(first),
        "finish refuses even if a producer ignored the first error"
    );
}

#[test]
fn utf8_text_refusal_keeps_byte_counts_without_trimming_or_character_counting() {
    let mut meter = Meter::new(limits(1, 4, 1));
    meter
        .charge_text(" é", location("first", Some(0)))
        .expect("space plus two-byte code point");
    let first = Error::Count {
        unit: Unit::TextBytes,
        maximum: 4,
        consumed: 3,
        requested: 3,
        location: location("second", Some(1)),
    };
    assert_eq!(
        meter.charge_text("猫", location("second", Some(1))),
        Err(first)
    );
    assert_eq!(meter.charge_text("", location("empty", None)), Err(first));
    assert_eq!(meter.finish(), Err(first));
}

#[test]
fn counter_refusal_is_first_failure_even_when_later_collection_also_exceeds_its_maximum() {
    let mut meter = Meter::new(limits(1, 1, 129));
    meter
        .charge_counter_bits(128, location("first.revision", None))
        .expect("preceding bits");
    let first = Error::CounterBits {
        maximum: 129,
        consumed: 128,
        requested: 2,
        location: location("second.revision", Some(2)),
    };
    assert_eq!(
        meter.charge_counter_bits(2, location("second.revision", Some(2))),
        Err(first)
    );
    assert_eq!(
        meter.charge_entries(usize::MAX, location("later", None)),
        Err(first)
    );
    assert_eq!(meter.finish(), Err(first));
}

#[test]
fn platform_maxima_admit_exact_counts_and_refuse_successors_without_overflow_or_clipping() {
    let mut entries = Meter::new(limits(usize::MAX, 1, 1));
    entries
        .charge_entries(usize::MAX - 1, location("first", None))
        .expect("exact first component");
    entries
        .charge_entries(1, location("last", None))
        .expect("exact maximum");
    let issue = Error::Count {
        unit: Unit::CollectionEntries,
        maximum: usize::MAX,
        consumed: usize::MAX,
        requested: 1,
        location: location("overflow", Some(0)),
    };
    assert_eq!(
        entries.charge_entries(1, location("overflow", Some(0))),
        Err(issue)
    );
    assert_eq!(entries.finish(), Err(issue));
    let mut counters = Meter::new(limits(1, 1, u64::MAX));
    counters
        .charge_counter_bits(u64::MAX - 1, location("first", None))
        .expect("exact first bits");
    counters
        .charge_counter_bits(1, location("last", None))
        .expect("exact maximum bits");
    let issue = Error::CounterBits {
        maximum: u64::MAX,
        consumed: u64::MAX,
        requested: 1,
        location: location("overflow", Some(1)),
    };
    assert_eq!(
        counters.charge_counter_bits(1, location("overflow", Some(1))),
        Err(issue)
    );
    assert_eq!(counters.finish(), Err(issue));
}

#[test]
fn empty_meter_finishes_as_zero_data_without_claiming_that_any_model_was_traversed() {
    let demand = Meter::new(limits(1, 1, 1))
        .finish()
        .expect("empty declared demand");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (0, 0, 0)
    );
}
