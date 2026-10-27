//! Owned common metadata retains actual sources, complete identity scopes and temporary contexts.
#![allow(clippy::expect_used)]
#[path = "support/size_break_fixture.rs"]
mod support;
use core::cell::Cell;
use sc_core::value::CanonicalInputs;
use sc_measure::*;
use support::{id, Fixture};

fn canonical(f: &Fixture) -> CanonicalInputs {
    CanonicalInputs::new(f.lengths.clone(), f.scalars.clone())
        .expect("actual disjoint fixture inputs")
}
fn inputs(f: &Fixture, canonical: &CanonicalInputs, extra: Option<u128>) -> MeasurementInputs {
    let mut tables = vec![f.table.clone()];
    if let Some(identity) = extra {
        let records = MeasurementContext::new(&f.lengths, &f.landmarks, &f.procedures)
            .expect("actual records");
        let context =
            MeasurementTableContext::new(&f.measurements, &records).expect("actual metadata");
        let mut definition = f.table.definition().clone();
        definition.id = id(identity);
        tables.push(MeasurementTable::new(definition, &context).expect("unselected actual table"));
    }
    let measurements = MeasurementInventory::new(
        MeasurementInventoryDefinition {
            landmarks: f.landmarks.clone(),
            procedures: f.procedures.clone(),
            measurements: f.measurements.clone(),
            tables,
        },
        canonical,
    )
    .expect("actual metadata inventory");
    MeasurementInputs::new(
        MeasurementInputsDefinition {
            measurements,
            eases: vec![],
            ease_sets: vec![],
        },
        canonical,
    )
    .expect("actual measurement inputs")
}
fn definition(f: &Fixture, partial: bool) -> SizeSetMetadataDefinition {
    let breaks = f.with_context(|context| {
        let breaks = if partial {
            vec![]
        } else {
            vec![SizeBreak::new(f.definition(), context).expect("actual authored break")]
        };
        SizeBreaks::new(
            SizeBreaksDefinition {
                reference: f.membership.reference(),
                dimensions: vec![SizeBreakDimension {
                    axis: id(10),
                    dimension_table: f.table.id(),
                    dimension: MeasurementBinding::from(
                        f.measurements.first().expect("actual dimension"),
                    ),
                }],
                breaks,
            },
            context,
        )
        .expect("current authored inventory, possibly partial")
    });
    SizeSetMetadataDefinition {
        membership: f.membership.clone(),
        axes: f.axes.clone(),
        breaks,
        design_table: f.table.id(),
    }
}

#[test]
fn exact_metadata_order_and_actual_canonical_objects_survive_temporary_contexts() {
    let f = Fixture::new(MeasurementKind::Garment);
    let canonical = canonical(&f);
    let inputs = inputs(&f, &canonical, None);
    let definition = definition(&f, false);
    let original = definition.clone();
    let metadata =
        SizeSetMetadata::new(definition, &canonical, &inputs).expect("actual common sources");
    assert_eq!(metadata.definition(), &original);
    assert_eq!(metadata.reference(), f.membership.reference());
    use SizeSetMetadataPosition as P;
    assert_eq!(
        metadata.identities(),
        vec![
            (id(1), P::Set),
            (id(2), P::Member { index: 0 }),
            (id(3), P::Member { index: 1 }),
            (id(4), P::Member { index: 2 }),
            (id(10), P::Axis { index: 0 }),
            (id(11), P::AxisValue { axis: 0, index: 0 }),
            (id(12), P::AxisValue { axis: 0, index: 1 }),
            (id(13), P::AxisValue { axis: 0, index: 2 }),
            (id(700), P::Break { index: 0 })
        ]
    );
    let actual = metadata
        .with_context(&canonical, &inputs, |context| {
            assert!(core::ptr::eq(
                context.membership(),
                &metadata.definition().membership
            ));
            let record = metadata
                .definition()
                .breaks
                .definition()
                .breaks
                .first()
                .expect("authored break")
                .delta(context)
                .expect("actual canonical delta");
            core::ptr::eq(
                record,
                canonical
                    .length(id(100))
                    .expect("actual current declaration"),
            )
        })
        .expect("temporary actual context");
    assert!(actual);
    let debug = format!("{metadata:?}");
    assert!(debug.contains("member_count: 3"));
    assert!(!debug.contains("2XL"));
    assert_eq!(
        canonical
            .length(id(150))
            .expect("unavailable measurement")
            .state(),
        &sc_core::value::LengthState::Unknown {
            observation: id(802)
        }
    );
}

#[test]
fn partial_breaks_remain_a_draft_without_inventing_coverage() {
    let f = Fixture::new(MeasurementKind::Garment);
    let canonical = canonical(&f);
    let inputs = inputs(&f, &canonical, None);
    let metadata =
        SizeSetMetadata::new(definition(&f, true), &canonical, &inputs).expect("explicit draft");
    assert!(metadata.validate_current(&canonical, &inputs).is_ok());
    assert!(metadata
        .with_context(&canonical, &inputs, |context| metadata
            .definition()
            .breaks
            .require_grade_coverage(context)
            .is_err())
        .expect("actual draft context"));
    assert!(metadata.definition().breaks.definition().breaks.is_empty());
}

#[test]
fn all_common_owned_families_are_disjoint_from_unselected_measurement_tables() {
    let f = Fixture::new(MeasurementKind::Garment);
    let canonical = canonical(&f);
    use SizeSetMetadataPosition as P;
    for (identity, duplicate) in [
        (1, P::Set),
        (2, P::Member { index: 0 }),
        (10, P::Axis { index: 0 }),
        (11, P::AxisValue { axis: 0, index: 0 }),
        (700, P::Break { index: 0 }),
    ] {
        let inputs = inputs(&f, &canonical, Some(identity));
        assert_eq!(
            SizeSetMetadata::new(definition(&f, false), &canonical, &inputs)
                .expect_err("cross-family collision"),
            SizeSetMetadataError::DuplicateIdentity {
                identity: id(identity),
                first: P::Inputs(MeasurementInputsPosition::Metadata(
                    MeasurementInventoryPosition::Table { index: 1 }
                )),
                duplicate
            }
        );
    }
}

#[test]
fn missing_exact_design_table_is_not_replaced_by_equal_available_content() {
    let f = Fixture::new(MeasurementKind::Garment);
    let canonical = canonical(&f);
    let inputs = inputs(&f, &canonical, None);
    let mut definition = definition(&f, false);
    definition.design_table = id(9999);
    assert_eq!(
        SizeSetMetadata::new(definition, &canonical, &inputs).expect_err("missing exact table"),
        SizeSetMetadataError::MissingDesignTable { table: id(9999) }
    );
}

#[test]
fn replacement_canonical_context_rechecks_actual_delta_and_never_visits_on_failure() {
    let f = Fixture::new(MeasurementKind::Garment);
    let original = canonical(&f);
    let inputs = inputs(&f, &original, None);
    let metadata =
        SizeSetMetadata::new(definition(&f, false), &original, &inputs).expect("actual sources");
    let replacement = CanonicalInputs::new(
        original
            .lengths()
            .iter()
            .filter(|value| value.id() != id(100))
            .cloned()
            .collect(),
        vec![],
    )
    .expect("removed actual delta");
    let visited = Cell::new(false);
    let error = metadata
        .with_context(&replacement, &inputs, |_| visited.set(true))
        .expect_err("current missing delta");
    assert!(!visited.get());
    assert!(matches!(&error, SizeSetMetadataError::Breaks(_)));
    let mut cause = std::error::Error::source(&error);
    let mut actual_missing = false;
    let mut boxed_missing = false;
    while let Some(error) = cause {
        actual_missing |=
            error.downcast_ref::<SizeBreakError>() == Some(&SizeBreakError::MissingDelta(id(100)));
        boxed_missing |= error
            .downcast_ref::<Box<SizeBreakError>>()
            .is_some_and(|issue| issue.as_ref() == &SizeBreakError::MissingDelta(id(100)));
        cause = error.source();
    }
    assert!(
        actual_missing,
        "underlying typed cause missing; boxed payload seen={boxed_missing}"
    );
    assert!(metadata.validate_current(&original, &inputs).is_ok());
}
