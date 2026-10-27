//! Composite garment inputs use one actual snapshot and preserve chart/break path distinctions.
#![allow(clippy::expect_used, clippy::indexing_slicing)]
#[path = "support/size_break_fixture.rs"]
mod support;
use sc_core::value::{LengthState, LengthValueError};
use sc_measure::*;
use std::error::Error;
use support::{declaration, id, Fixture};
#[derive(Default)]
struct Options {
    complete_breaks: bool,
    partial_chart: bool,
    snapshot: Option<SizeSetSnapshot>,
    chart_alias: Option<u128>,
    observation_alias: Option<u128>,
}
fn with_parts<T>(
    f: &Fixture,
    options: Options,
    run: impl FnOnce(
        &SizeChart,
        &SizeChartCollectionContext<'_>,
        &SizeBreaks,
        &SizeBreakContext<'_>,
    ) -> T,
) -> T {
    let records = MeasurementContext::new(&f.lengths, &f.landmarks, &f.procedures)
        .expect("canonical records");
    let metadata =
        MeasurementTableContext::new(&f.measurements, &records).expect("shared metadata");
    let other_metadata =
        MeasurementTableContext::new(&f.measurements, &records).expect("separate wrapper");
    let other_membership = f.membership.clone();
    let other_table = f.table.clone();
    let refs = f.lengths.iter().collect::<Vec<_>>();
    let scalars = f.scalars.iter().collect::<Vec<_>>();
    let sources = SizeAxisSourceContext::new(&refs, &scalars).expect("shared numeric records");
    let break_context =
        SizeBreakContext::new(&f.membership, &f.axes, &sources, &f.table, &metadata)
            .expect("actual break context");
    let chart_records = SizeChartContext::new(
        if options.snapshot == Some(SizeSetSnapshot::Membership) {
            &other_membership
        } else {
            &f.membership
        },
        std::slice::from_ref(if options.snapshot == Some(SizeSetSnapshot::DesignTable) {
            &other_table
        } else {
            &f.table
        }),
        if options.snapshot == Some(SizeSetSnapshot::Measurements) {
            &other_metadata
        } else {
            &metadata
        },
    )
    .expect("actual chart context");
    let mut observations = vec![];
    for (member_index, member) in f.membership.definition().members.iter().enumerate() {
        for (pom_index, pom) in f.measurements.iter().enumerate() {
            let index = member_index * 2 + pom_index;
            observations.push(
                SizeChartObservation::new(
                    SizeChartObservationDefinition {
                        id: id(if index == 0 {
                            options.observation_alias.unwrap_or(800)
                        } else {
                            800 + index as u128
                        }),
                        membership: f.membership.reference(),
                        member: member.id,
                        design_table: id(500),
                        chart_table: id(500),
                        pom: MeasurementBinding::from(pom),
                        measurement: MeasurementBinding::from(pom),
                        provenance: id(900),
                    },
                    &chart_records,
                )
                .expect("explicit chart correspondence"),
            );
        }
    }
    if options.partial_chart {
        observations.pop();
    }
    let chart_context =
        SizeChartCollectionContext::new(&observations, &chart_records).expect("chart inventory");
    let chart = SizeChart::new(
        SizeChartDefinition {
            id: id(options.chart_alias.unwrap_or(600)),
            membership: f.membership.reference(),
            design_table: id(500),
            poms: f
                .measurements
                .iter()
                .map(MeasurementBinding::from)
                .collect(),
            observations: observations.iter().map(SizeChartBinding::from).collect(),
        },
        &chart_context,
    )
    .expect("authored chart");
    let mut available = vec![];
    if options.complete_breaks {
        for (index, (from, to, dimension)) in [(2, 3, 0), (3, 4, 0), (2, 3, 1), (3, 4, 1)]
            .into_iter()
            .enumerate()
        {
            let mut d = f.definition();
            d.id = id(700 + index as u128);
            d.from = id(from);
            d.to = id(to);
            d.dimension = MeasurementBinding::from(&f.measurements[dimension]);
            d.delta = id(if dimension == 0 { 100 } else { 101 });
            available.push(SizeBreak::new(d, &break_context).expect("explicit break"));
        }
    }
    let breaks = SizeBreaks::new(
        SizeBreaksDefinition {
            reference: f.membership.reference(),
            dimensions: f
                .measurements
                .iter()
                .map(|value| SizeBreakDimension {
                    axis: id(10),
                    dimension_table: id(500),
                    dimension: MeasurementBinding::from(value),
                })
                .collect(),
            breaks: available,
        },
        &break_context,
    )
    .expect("explicit required dimensions");
    run(&chart, &chart_context, &breaks, &break_context)
}
#[test]
fn complete_current_view_borrows_actual_records_and_validates_both_structural_paths() {
    let f = Fixture::new(MeasurementKind::Garment);
    with_parts(
        &f,
        Options {
            complete_breaks: true,
            ..Options::default()
        },
        |chart, chart_context, breaks, break_context| {
            let inputs = GarmentSizeSetInputs::new(chart, chart_context, breaks, break_context)
                .expect("one actual snapshot");
            assert_eq!(inputs.reference(), f.membership.reference());
            let family = SizeSetInputs::from(inputs);
            assert!(matches!(family, SizeSetInputs::Garment(_)));
            assert_eq!(family.reference(), inputs.reference());
            assert!(std::ptr::eq(family.membership(), inputs.membership()));
            assert!(std::ptr::eq(family.axes(), inputs.axes()));
            assert!(std::ptr::eq(family.breaks(), inputs.breaks()));
            assert_eq!(family.validate_current(), inputs.validate_current());
            for path in [
                SizeInstantiationPath::Regeneration,
                SizeInstantiationPath::GradeRules,
            ] {
                assert_eq!(
                    family.validate_inputs_for(path),
                    inputs.validate_inputs_for(path)
                );
            }

            assert!(std::ptr::eq(inputs.membership(), &f.membership));
            assert!(std::ptr::eq(inputs.axes(), &f.axes));
            assert!(std::ptr::eq(inputs.chart(), chart));
            assert!(std::ptr::eq(inputs.breaks(), breaks));
            assert_eq!(
                inputs.validate_inputs_for(SizeInstantiationPath::Regeneration),
                Ok(())
            );
            assert_eq!(
                inputs.validate_inputs_for(SizeInstantiationPath::GradeRules),
                Ok(())
            );
        },
    );
}
#[test]
fn missing_breaks_do_not_block_regeneration_and_incomplete_chart_remains_a_draft() {
    let f = Fixture::new(MeasurementKind::Garment);
    with_parts(
        &f,
        Options::default(),
        |chart, chart_context, breaks, break_context| {
            let inputs = GarmentSizeSetInputs::new(chart, chart_context, breaks, break_context)
                .expect("current partial break draft");
            assert_eq!(
                inputs.validate_inputs_for(SizeInstantiationPath::Regeneration),
                Ok(())
            );
            let refusal =
                SizeSetInputError::Breaks(Box::new(SizeBreaksError::MissingCell(SizeBreakCell {
                    axis: id(10),
                    dimension: id(200),
                    from: id(2),
                    to: id(3),
                })));
            assert_eq!(
                inputs.validate_inputs_for(SizeInstantiationPath::GradeRules),
                Err(refusal.clone())
            );
            assert!(refusal.source().is_some());
        },
    );
    with_parts(
        &f,
        Options {
            partial_chart: true,
            ..Options::default()
        },
        |chart, chart_context, breaks, break_context| {
            let inputs = GarmentSizeSetInputs::new(chart, chart_context, breaks, break_context)
                .expect("structurally valid draft chart");
            let refusal =
                SizeSetInputError::Chart(Box::new(SizeChartCollectionError::MissingCell {
                    chart: id(600),
                    member: id(4),
                    pom: id(201),
                }));
            assert_eq!(
                inputs.validate_inputs_for(SizeInstantiationPath::Regeneration),
                Err(refusal.clone())
            );
            assert!(refusal.source().is_some());
        },
    );
}
#[test]
fn matching_id_context_wrappers_do_not_substitute_for_a_shared_actual_snapshot() {
    let f = Fixture::new(MeasurementKind::Garment);
    for snapshot in [
        SizeSetSnapshot::Membership,
        SizeSetSnapshot::Measurements,
        SizeSetSnapshot::DesignTable,
    ] {
        with_parts(
            &f,
            Options {
                snapshot: Some(snapshot),
                ..Options::default()
            },
            |chart, chart_context, breaks, break_context| {
                assert_eq!(
                    GarmentSizeSetInputs::new(chart, chart_context, breaks, break_context).err(),
                    Some(SizeSetInputError::SharedSnapshot(snapshot))
                );
            },
        );
    }
}
#[test]
fn identities_cannot_alias_axes_values_or_breaks_across_independently_valid_views() {
    let f = Fixture::new(MeasurementKind::Garment);
    for (chart_alias, observation_alias, expected) in [
        (Some(10), None, 10),
        (None, Some(11), 11),
        (None, Some(700), 700),
    ] {
        with_parts(
            &f,
            Options {
                complete_breaks: true,
                chart_alias,
                observation_alias,
                ..Options::default()
            },
            |chart, chart_context, breaks, break_context| {
                assert_eq!(
                    GarmentSizeSetInputs::new(chart, chart_context, breaks, break_context).err(),
                    Some(SizeSetInputError::DuplicateIdentity(id(expected)))
                );
            },
        );
    }
}
#[test]
fn structural_path_checks_preserve_original_unavailable_numeric_inputs() {
    let mut f = Fixture::new(MeasurementKind::Garment);
    f.lengths[0] = declaration(
        100,
        LengthState::Unknown {
            observation: id(8888),
        },
    );
    with_parts(
        &f,
        Options {
            complete_breaks: true,
            ..Options::default()
        },
        |chart, chart_context, breaks, break_context| {
            let inputs = GarmentSizeSetInputs::new(chart, chart_context, breaks, break_context)
                .expect("current unknown input");
            assert_eq!(
                inputs.validate_inputs_for(SizeInstantiationPath::GradeRules),
                Ok(())
            );
            let cell = SizeBreakCell {
                axis: id(10),
                dimension: id(200),
                from: id(2),
                to: id(3),
            };
            let declaration = inputs
                .breaks()
                .declaration(cell, break_context)
                .expect("actual delta declaration");
            assert!(std::ptr::eq(
                declaration.delta(break_context).expect("original source"),
                &f.lengths[0]
            ));
            assert_eq!(
                declaration.authored_value(break_context),
                Err(SizeBreakError::Value(
                    LengthValueError::RequiresObservation {
                        declaration: id(100),
                        observation: id(8888)
                    }
                ))
            );
        },
    );
}

#[test]
fn explicit_garment_family_does_not_infer_mtm_from_custom_singleton_membership() {
    let mut f = Fixture::new(MeasurementKind::Garment);
    let mut members = f.membership.definition().clone();
    members.members.truncate(1);
    members.base = id(2);
    f.membership = SizeMembership::new(members).expect("authored custom singleton");
    let mut axes = f.axes.definition().clone();
    axes.points.retain(|point| point.member == id(2));
    let refs = f.lengths.iter().collect::<Vec<_>>();
    let sources = SizeAxisSourceContext::new(&refs, &[]).expect("canonical records");
    f.axes = SizeAxes::new(axes, &f.membership, &sources).expect("explicit singleton axis");
    with_parts(
        &f,
        Options::default(),
        |chart, chart_context, breaks, break_context| {
            let family = SizeSetInputs::from(
                GarmentSizeSetInputs::new(chart, chart_context, breaks, break_context)
                    .expect("actual garment-chart provenance"),
            );
            assert!(matches!(family, SizeSetInputs::Garment(_)));
            assert_eq!(
                family.validate_inputs_for(SizeInstantiationPath::Regeneration),
                Ok(())
            );
            assert_eq!(
                family.validate_inputs_for(SizeInstantiationPath::GradeRules),
                Err(SizeSetInputError::Breaks(Box::new(
                    SizeBreaksError::Singleton {
                        set: id(1),
                        member: id(2)
                    }
                )))
            );
        },
    );
}

#[test]
fn resolution_origin_pins_actual_source_and_separate_target_without_mutating_inputs() {
    let f = Fixture::new(MeasurementKind::Garment);
    let before = f.membership.clone();
    with_parts(
        &f,
        Options::default(),
        |chart, chart_context, breaks, break_context| {
            let inputs = GarmentSizeSetInputs::new(chart, chart_context, breaks, break_context)
                .expect("actual garment inputs")
                .into();
            let authored = SizeSetResolutionOriginDefinition {
                source: f.membership.reference(),
                profile: SizeSetProfilePin::new(SizeSetProfilePinDefinition {
                    profile_id: "acme-cmt-pt".to_owned(),
                    schema_version: "2.0".to_owned(),
                    content_version: "1.3.0".to_owned(),
                })
                .expect("selected pin"),
                resolved: SizeSetReference {
                    id: id(2000),
                    revision: sc_units::Count::new(0),
                },
                provenance: id(3000),
            };
            let origin =
                SizeSetResolutionOrigin::new(authored.clone(), inputs).expect("explicit origin");
            assert_eq!(origin.definition(), &authored);
            assert_eq!(origin.validate_current(inputs), Ok(()));
            let mut stale = authored;
            stale.source.revision = sc_units::Count::new(4);
            assert_eq!(
                SizeSetResolutionOrigin::new(stale.clone(), inputs),
                Err(SizeSetResolutionOriginError::SourceMismatch {
                    expected: stale.source,
                    actual: f.membership.reference(),
                })
            );
        },
    );
    assert_eq!(f.membership, before);
}

#[test]
fn resolved_identity_cannot_alias_any_supplied_garment_namespace_or_its_provenance() {
    let f = Fixture::new(MeasurementKind::Garment);
    with_parts(
        &f,
        Options {
            complete_breaks: true,
            ..Options::default()
        },
        |chart, chart_context, breaks, break_context| {
            let inputs = GarmentSizeSetInputs::new(chart, chart_context, breaks, break_context)
                .expect("actual garment inputs")
                .into();
            for identity in [
                1, 2, 3, 4, 10, 11, 12, 13, 100, 101, 102, 103, 150, 151, 200, 201, 300, 301, 400,
                500, 600, 700, 701, 702, 703, 800, 805, 3000,
            ] {
                let authored = SizeSetResolutionOriginDefinition {
                    source: f.membership.reference(),
                    profile: SizeSetProfilePin::new(SizeSetProfilePinDefinition {
                        profile_id: "profile".to_owned(),
                        schema_version: "schema".to_owned(),
                        content_version: "content".to_owned(),
                    })
                    .expect("explicit pin"),
                    resolved: SizeSetReference {
                        id: id(identity),
                        revision: sc_units::Count::new(0),
                    },
                    provenance: id(3000),
                };
                assert_eq!(
                    SizeSetResolutionOrigin::new(authored, inputs),
                    Err(SizeSetResolutionOriginError::DuplicateIdentity(id(
                        identity
                    ))),
                    "identity {identity}"
                );
            }
        },
    );
}

#[path = "support/size_set_transformations.rs"]
mod transformations;

#[test]
fn design_reference_requires_exact_set_and_revision_without_inventing_coverage() {
    use sc_core::design::DesignSizeSetReference;
    use sc_units::Count;
    let fixture = Fixture::new(MeasurementKind::Garment);
    with_parts(
        &fixture,
        Options {
            partial_chart: true,
            ..Options::default()
        },
        |chart, chart_context, breaks, break_context| {
            let inputs = SizeSetInputs::from(
                GarmentSizeSetInputs::new(chart, chart_context, breaks, break_context)
                    .expect("actual family snapshot"),
            );
            let original = inputs.reference();
            let actual = DesignSizeSetReference::from(original);
            assert_eq!(SizeSetReference::from(actual), original);
            assert_eq!(inputs.validate_design_reference(actual), Ok(()));
            for expected in [
                DesignSizeSetReference::new(id(9999), original.revision),
                DesignSizeSetReference::new(original.id, Count::new(original.revision.get() + 1)),
            ] {
                assert_eq!(
                    inputs.validate_design_reference(expected),
                    Err(SizeSetInputError::DesignReferenceMismatch { expected, actual })
                );
            }
            assert_eq!(inputs.reference(), original);
            assert!(matches!(
                inputs.validate_inputs_for(SizeInstantiationPath::Regeneration),
                Err(SizeSetInputError::Chart(_))
            ));
            for revision in [Count::new(0), Count::new(u32::MAX)] {
                let pin = DesignSizeSetReference::new(original.id, revision);
                assert_eq!(
                    DesignSizeSetReference::from(SizeSetReference::from(pin)),
                    pin
                );
            }
        },
    );
}
