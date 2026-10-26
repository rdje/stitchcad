//! Composite singleton body/Ease inputs preserve actual sources and the original grading refusal.
#![allow(clippy::expect_used, clippy::indexing_slicing)]
#[path = "support/mtm_fixture.rs"]
mod support;
use sc_core::value::LengthState;
use sc_measure::*;
use std::error::Error;
use support::{id, scalar, Fixture};
#[derive(Default)]
struct Options {
    snapshot: Option<SizeSetSnapshot>,
    chart_alias: Option<u128>,
}
fn with_parts<T>(
    f: &Fixture,
    options: Options,
    run: impl FnOnce(&MtmChart, &MtmChartContext<'_>, &SizeBreaks, &SizeBreakContext<'_>) -> T,
) -> T {
    let records = f.records();
    let metadata = MeasurementTableContext::new(&f.measurements, &records).expect("metadata");
    let other_metadata =
        MeasurementTableContext::new(&f.measurements, &records).expect("separate wrapper");
    let other_membership = f.membership.clone();
    let other_tables = f.tables.clone();
    let refs = f.declarations.iter().collect::<Vec<_>>();
    let sources = SizeAxisSourceContext::new(&refs, &[]).expect("shared sources");
    let axis = SizeAxis::new(
        SizeAxisDefinition {
            reference: f.membership.reference(),
            id: id(80),
            name: "Client size".to_owned(),
            kind: SizeAxisKind::Label,
            values: vec![SizeAxisValueDefinition {
                id: id(81),
                source: SizeAxisValueSource::Label(
                    SizeLabel::new("Client coordinate".to_owned()).expect("authored designation"),
                ),
            }],
        },
        &sources,
    )
    .expect("explicit singleton axis");
    let axes = SizeAxes::new(
        SizeAxesDefinition {
            reference: f.membership.reference(),
            axes: vec![axis],
            points: vec![SizeMemberPoint {
                member: id(41),
                coordinates: vec![SizeAxisCoordinate {
                    axis: id(80),
                    value: id(81),
                }],
            }],
        },
        &f.membership,
        &sources,
    )
    .expect("authored singleton coordinate");
    let break_context =
        SizeBreakContext::new(&f.membership, &axes, &sources, &f.tables[1], &metadata)
            .expect("break context");
    let ease_context = EaseSetContext::new(
        if options.snapshot == Some(SizeSetSnapshot::DesignTable) {
            &other_tables
        } else {
            &f.tables
        },
        &f.eases,
        if options.snapshot == Some(SizeSetSnapshot::Measurements) {
            &other_metadata
        } else {
            &metadata
        },
    )
    .expect("current ease records");
    let chart_context = MtmChartContext::new(
        if options.snapshot == Some(SizeSetSnapshot::Membership) {
            &other_membership
        } else {
            &f.membership
        },
        &f.sets,
        &ease_context,
    )
    .expect("current mtm context");
    let mut definition = f.definition();
    if let Some(alias) = options.chart_alias {
        definition.id = id(alias);
    }
    let chart = MtmChart::new(definition, &chart_context).expect("current body/Ease chart");
    let breaks = SizeBreaks::new(
        SizeBreaksDefinition {
            reference: f.membership.reference(),
            dimensions: vec![],
            breaks: vec![],
        },
        &break_context,
    )
    .expect("explicit empty singleton breaks");
    run(&chart, &chart_context, &breaks, &break_context)
}
#[test]
fn actual_singleton_inputs_borrow_canonical_records_and_always_refuse_grading() {
    let f = Fixture::new();
    with_parts(
        &f,
        Options::default(),
        |chart, chart_context, breaks, break_context| {
            let inputs = MtmSizeSetInputs::new(chart, chart_context, breaks, break_context)
                .expect("shared actual singleton");
            assert_eq!(inputs.reference(), f.membership.reference());
            assert!(std::ptr::eq(inputs.membership(), &f.membership));
            assert_eq!(inputs.axes().definition().axes[0].definition().id, id(80));
            assert!(std::ptr::eq(inputs.chart(), chart));
            assert!(std::ptr::eq(inputs.breaks(), breaks));
            assert_eq!(
                inputs.validate_inputs_for(SizeInstantiationPath::Regeneration),
                Ok(())
            );
            assert!(inputs
                .breaks()
                .missing(break_context)
                .expect("no adjacent pairs")
                .is_empty());
            let refusal = SizeSetInputError::Mtm(Box::new(MtmChartError::GradeRulesUnsupported {
                chart: id(70),
                member: id(41),
            }));
            assert_eq!(
                inputs.validate_inputs_for(SizeInstantiationPath::GradeRules),
                Err(refusal.clone())
            );
            assert!(refusal.source().is_some());
            assert!(std::ptr::eq(
                inputs
                    .chart()
                    .body_declaration(id(2), chart_context)
                    .expect("actual body"),
                &f.declarations[0]
            ));
            assert!(std::ptr::eq(
                inputs
                    .chart()
                    .ease_declaration(id(2), chart_context)
                    .expect("actual ease"),
                &f.declarations[3]
            ));
        },
    );
}
#[test]
fn incomplete_mappings_remain_drafts_and_preserve_the_original_coverage_error() {
    let mut f = Fixture::new();
    let mut definition = f.sets[0].definition().clone();
    definition.entries.truncate(1);
    f.sets[0] = f.make_set(definition);
    with_parts(
        &f,
        Options::default(),
        |chart, chart_context, breaks, break_context| {
            let inputs = MtmSizeSetInputs::new(chart, chart_context, breaks, break_context)
                .expect("valid partial body/Ease draft");
            let refusal = chart
                .validate_complete(chart_context)
                .expect_err("missing Design POM mapping");
            assert_eq!(
                inputs.validate_inputs_for(SizeInstantiationPath::Regeneration),
                Err(SizeSetInputError::Mtm(Box::new(refusal)))
            );
            assert_eq!(
                inputs.validate_inputs_for(SizeInstantiationPath::GradeRules),
                Err(SizeSetInputError::Mtm(Box::new(
                    MtmChartError::GradeRulesUnsupported {
                        chart: id(70),
                        member: id(41)
                    }
                )))
            );
        },
    );
}
#[test]
fn same_id_wrappers_cannot_replace_shared_membership_measurements_or_design_table() {
    let f = Fixture::new();
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
                    MtmSizeSetInputs::new(chart, chart_context, breaks, break_context).err(),
                    Some(SizeSetInputError::SharedSnapshot(snapshot))
                );
            },
        );
    }
}
#[test]
fn chart_sets_eases_and_body_tables_cannot_alias_authored_axes_or_coordinates() {
    for mode in 0..4 {
        let mut f = Fixture::new();
        let mut options = Options::default();
        match mode {
            0 => options.chart_alias = Some(80),
            1 => {
                let mut d = f.sets[0].definition().clone();
                d.id = id(81);
                f.sets[0] = f.make_set(d);
            }
            2 => {
                let mut d = f.eases[0].definition().clone();
                d.id = id(81);
                f.eases[0] = f.make_ease(d);
                let mut d = f.sets[0].definition().clone();
                d.entries[0] = EaseBinding::new(d.entries[0].token.clone(), &f.eases[0]);
                f.sets[0] = f.make_set(d);
            }
            _ => {
                let mut d = f.tables[0].definition().clone();
                d.id = id(80);
                let records = f.records();
                let metadata =
                    MeasurementTableContext::new(&f.measurements, &records).expect("metadata");
                let table =
                    MeasurementTable::new(d, &metadata).expect("individually valid body table");
                f.tables[0] = table;
                let mut d = f.sets[0].definition().clone();
                d.body_table = id(80);
                f.sets[0] = f.make_set(d);
            }
        }
        with_parts(
            &f,
            options,
            |chart, chart_context, breaks, break_context| {
                assert_eq!(
                    MtmSizeSetInputs::new(chart, chart_context, breaks, break_context).err(),
                    Some(SizeSetInputError::DuplicateIdentity(id(
                        if mode == 0 || mode == 3 { 80 } else { 81 }
                    )))
                );
            },
        );
    }
}
#[test]
fn structural_regeneration_does_not_evaluate_or_default_unknown_body_inputs() {
    let mut f = Fixture::new();
    f.declarations[0] = scalar(
        10,
        LengthState::Unknown {
            observation: id(8888),
        },
    );
    with_parts(
        &f,
        Options::default(),
        |chart, chart_context, breaks, break_context| {
            let inputs = MtmSizeSetInputs::new(chart, chart_context, breaks, break_context)
                .expect("unavailable original body input");
            assert_eq!(
                inputs.validate_inputs_for(SizeInstantiationPath::Regeneration),
                Ok(())
            );
            let original = inputs
                .chart()
                .body_declaration(id(2), chart_context)
                .expect("original canonical state");
            assert!(std::ptr::eq(original, &f.declarations[0]));
            assert_eq!(
                original.state(),
                &LengthState::Unknown {
                    observation: id(8888)
                }
            );
            let refusal = chart.body_value(id(2), chart_context);
            assert!(refusal.is_err());
            assert_eq!(inputs.chart().body_value(id(2), chart_context), refusal);
        },
    );
}

#[test]
fn unified_mtm_family_keeps_original_paths_and_canonical_borrows() {
    let f = Fixture::new();
    with_parts(
        &f,
        Options::default(),
        |chart, chart_context, breaks, break_context| {
            let inputs = MtmSizeSetInputs::new(chart, chart_context, breaks, break_context)
                .expect("actual body/Ease inputs");
            let family = SizeSetInputs::from(inputs);
            assert!(matches!(family, SizeSetInputs::Mtm(_)));
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
            assert!(std::ptr::eq(
                inputs
                    .chart()
                    .body_declaration(id(2), chart_context)
                    .expect("original body"),
                &f.declarations[0]
            ));
        },
    );
}

#[test]
fn mtm_resolution_origin_preserves_source_and_refuses_body_ease_and_axis_aliases() {
    let f = Fixture::new();
    let before = f.membership.clone();
    with_parts(
        &f,
        Options::default(),
        |chart, chart_context, breaks, break_context| {
            let inputs = MtmSizeSetInputs::new(chart, chart_context, breaks, break_context)
                .expect("actual singleton inputs")
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
                SizeSetResolutionOrigin::new(authored.clone(), inputs).expect("new target");
            assert_eq!(origin.definition(), &authored);
            assert_eq!(origin.validate_current(inputs), Ok(()));
            for identity in [
                1, 2, 3, 10, 11, 12, 13, 14, 20, 23, 24, 25, 30, 31, 40, 41, 50, 51, 60, 70, 80,
                81, 3000,
            ] {
                let mut alias = authored.clone();
                alias.resolved.id = id(identity);
                assert_eq!(
                    SizeSetResolutionOrigin::new(alias, inputs),
                    Err(SizeSetResolutionOriginError::DuplicateIdentity(id(
                        identity
                    ))),
                    "identity {identity}"
                );
            }
        },
    );
    assert_eq!(f.membership, before);
}

#[test]
fn singleton_transformation_intent_retains_its_body_ease_inputs_and_original_grading_refusal() {
    let f = Fixture::new();
    with_parts(
        &f,
        Options::default(),
        |chart, chart_context, breaks, break_context| {
            let inputs: SizeSetInputs<'_> =
                MtmSizeSetInputs::new(chart, chart_context, breaks, break_context)
                    .expect("actual singleton inputs")
                    .into();
            let definition = SizeSetTransformationDefinition {
                id: id(2000),
                reference: inputs.reference(),
                source: id(3000),
                kind: SizeSetTransformationKind::BaseSubstitution,
                state: SizeSetTransformationState::Preference {
                    value: SizeSetTransformationValue::BaseSubstitution(id(41)),
                    provenance: id(3001),
                },
            };
            let result = SizeSetTransformation::new(definition.clone(), inputs)
                .expect("explicit current base");
            assert_eq!(result.definition(), &definition);
            assert_eq!(
                inputs.validate_inputs_for(SizeInstantiationPath::GradeRules),
                Err(SizeSetInputError::Mtm(Box::new(
                    MtmChartError::GradeRulesUnsupported {
                        chart: id(70),
                        member: id(41)
                    }
                )))
            );
        },
    );
}
