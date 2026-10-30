//! Exact immutable source snapshots and held-buffer accounting before Design-root embedding.
#![allow(clippy::expect_used)]
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::{
    design::{
        charge_formula_source_resources, DesignResourceCount as Unit,
        DesignResourceLimitError as Error, DesignResourceLimits as Limits,
        DesignResourceLocation as Location, DesignResourceMeter as Meter,
        DesignResourceScope as Scope, DesignResourceSink,
    },
    ontology::EntityId,
    recipe::{FormulaExpressionDocument, FormulaSource, FormulaSourceLimits, FormulaSources},
};
const FIRST: &str = " 2.5 cm ";
const SECOND: &str = "tan(90 deg)";
const FIRST_CANONICAL: &str = "length:25000";
const SECOND_CANONICAL: &str = "(tan angle:90000000)";
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn registry(first: &str, reverse: bool) -> FormulaSources {
    let mut rows = vec![
        FormulaSource::new(
            id(9002),
            FormulaExpressionDocument::new(first.into()).expect("actual original literal"),
        ),
        FormulaSource::new(
            id(9001),
            FormulaExpressionDocument::new(SECOND.into()).expect("actual unevaluated pole"),
        ),
    ];
    if reverse {
        rows.reverse();
    }
    FormulaSources::new(rows, FormulaSourceLimits::new(2, 100, 100))
        .expect("actual complete source collection")
}
fn limits(entries: usize, text: usize, bits: u64) -> Limits {
    Limits::new(
        NonZeroUsize::new(entries).expect("positive entries"),
        NonZeroUsize::new(text).expect("positive text"),
        NonZeroU64::new(bits).expect("positive bits"),
    )
}
fn location(field: &'static str, owner: Option<u128>, index: Option<usize>) -> Location {
    Location {
        scope: Scope::Inputs,
        owner: owner.map(id),
        field,
        index,
    }
}
#[test]
fn snapshots_preserve_exact_original_order_buffers_factory_counts_and_private_lookup_behavior() {
    let original = registry(FIRST, false);
    let snapshot = original.clone();
    assert_eq!(snapshot, original);
    assert_ne!(snapshot.sources().as_ptr(), original.sources().as_ptr());
    assert_eq!(
        (
            FIRST.len(),
            FIRST_CANONICAL.len(),
            SECOND.len(),
            SECOND_CANONICAL.len()
        ),
        (8, 12, 11, 20)
    );
    for ((source, copy), expected) in original.sources().iter().zip(snapshot.sources()).zip([
        (9002, FIRST, FIRST_CANONICAL, 1),
        (9001, SECOND, SECOND_CANONICAL, 2),
    ]) {
        assert_eq!(source.id(), id(expected.0));
        assert_eq!(copy.id(), source.id());
        assert_eq!(source.expression().source(), expected.1);
        assert_eq!(source.expression().canonical().as_str(), expected.2);
        assert_eq!(source.expression().node_count(), expected.3);
        assert_eq!(copy.expression().node_count(), expected.3);
        assert_eq!(source.expression(), copy.expression());
        assert_ne!(
            source.expression().source().as_ptr(),
            copy.expression().source().as_ptr()
        );
        assert_ne!(
            source.expression().canonical().as_str().as_ptr(),
            copy.expression().canonical().as_str().as_ptr()
        );
        assert!(std::ptr::eq(
            snapshot
                .source(copy.id())
                .expect("clone-owned exact lookup"),
            copy
        ));
        assert!(!std::ptr::eq(
            original.source(copy.id()).expect("original lookup"),
            copy
        ));
    }
    let alternate = registry("25 mm", false);
    assert_eq!(
        original
            .source(id(9002))
            .expect("original")
            .expression()
            .canonical(),
        alternate
            .source(id(9002))
            .expect("alias")
            .expression()
            .canonical()
    );
    assert_ne!(
        original, alternate,
        "canonical aliases cannot replace original source spelling"
    );
    assert_ne!(
        original,
        registry(FIRST, true),
        "authored order remains part of an exact snapshot"
    );
    let mut meter = Meter::new(limits(4, 51, 1));
    charge_formula_source_resources(&original, &mut meter)
        .expect("both vectors/all original and canonical buffers");
    let demand = meter.finish().expect("exact actual held content");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (4, 51, 0)
    );
    assert_eq!(
        original, snapshot,
        "accounting never normalizes/evaluates the unavailable tangent pole"
    );
}
#[test]
fn exact_vector_and_text_boundaries_preserve_first_actual_location_and_original_source_order() {
    let original = registry(FIRST, false);
    let saved = original.clone();
    for (entries, text, unit, maximum, consumed, requested, field, owner, index) in [
        (
            1,
            51,
            Unit::CollectionEntries,
            1,
            0,
            2,
            "formula_sources",
            None,
            None,
        ),
        (
            3,
            51,
            Unit::CollectionEntries,
            3,
            2,
            2,
            "formula_sources.index",
            None,
            None,
        ),
        (
            4,
            7,
            Unit::TextBytes,
            7,
            0,
            8,
            "formula_sources.source",
            Some(9002),
            Some(0),
        ),
        (
            4,
            19,
            Unit::TextBytes,
            19,
            8,
            12,
            "formula_sources.canonical",
            Some(9002),
            Some(0),
        ),
        (
            4,
            50,
            Unit::TextBytes,
            50,
            31,
            20,
            "formula_sources.canonical",
            Some(9001),
            Some(1),
        ),
    ] {
        let mut meter = Meter::new(limits(entries, text, 1));
        let expected = Error::Count {
            unit,
            maximum,
            consumed,
            requested,
            location: location(field, owner, index),
        };
        assert_eq!(
            charge_formula_source_resources(&original, &mut meter),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
        assert_eq!(original, saved);
    }
}
#[test]
fn cumulative_and_empty_source_scopes_charge_actual_content_and_preserve_prior_sink_failures() {
    let sources = registry(FIRST, false);
    let mut meter = Meter::new(limits(6, 53, 1));
    meter
        .charge_entries(2, location("prior", None, None))
        .expect("prior entries");
    meter
        .charge_text("é", location("prior", None, None))
        .expect("two actual preceding bytes");
    meter
        .charge_counter_bits(1, location("prior", None, None))
        .expect("prior bit");
    charge_formula_source_resources(&sources, &mut meter).expect("same cumulative sink");
    let demand = meter.finish().expect("exact combined content");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (6, 53, 1)
    );
    let empty = FormulaSources::new(vec![], FormulaSourceLimits::new(0, 0, 0))
        .expect("explicit actual empty registry");
    assert_eq!(empty, empty.clone());
    let mut meter = Meter::new(limits(1, 1, 1));
    charge_formula_source_resources(&empty, &mut meter).expect("zero empty source/index entries");
    let demand = meter.finish().expect("no dummy canonical expression");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (0, 0, 0)
    );
    let mut meter = Meter::new(limits(1, 1, 1));
    let prior = meter
        .charge_text("é", location("prior", None, None))
        .expect_err("actual preceding failure");
    assert_eq!(
        charge_formula_source_resources(&empty, &mut meter),
        Err(prior)
    );
    assert_eq!(meter.finish(), Err(prior));
}
