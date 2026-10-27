//! Owned MTM sources keep canonical body/Ease data separate and preserve exact current paths.
#![allow(clippy::expect_used)]
#[path = "support/owned_size_set_fixture.rs"]
mod support;
use core::cell::Cell;
use sc_core::value::CanonicalInputs;
use sc_measure::*;
use support::{id, mtm_fixture as fixture};

#[test]
fn owned_mtm_views_borrow_actual_body_and_ease_and_retain_singleton_grading_refusal() {
    let (canonical, inputs, definition) = fixture(false, 70);
    let original = definition.clone();
    let set = MtmSizeSet::new(definition, &canonical, &inputs).expect("actual owned MTM source");
    assert_eq!(set.definition(), &original);
    assert_eq!(
        set.identities().last(),
        Some(&(id(70), MtmSizeSetPosition::Chart))
    );
    set.with_inputs(&canonical, &inputs, |view| {
        assert!(core::ptr::eq(
            view.membership(),
            &set.definition().metadata.definition().membership
        ));
        assert!(core::ptr::eq(view.chart(), &set.definition().chart));
        let body = view
            .chart()
            .body_declaration(id(2), view.chart_context())
            .expect("actual body");
        let ease = view
            .chart()
            .ease_declaration(id(2), view.chart_context())
            .expect("actual Ease amount");
        assert!(core::ptr::eq(
            body,
            canonical.length(id(10)).expect("original canonical body")
        ));
        assert!(core::ptr::eq(
            ease,
            canonical.length(id(13)).expect("original canonical Ease")
        ));
        assert!(view
            .breaks()
            .missing(view.break_context())
            .expect("singleton has no adjacent pair")
            .is_empty());
        assert_eq!(
            SizeSetInputs::from(view).validate_design_reference(set.reference().into()),
            Ok(())
        );
    })
    .expect("actual temporary shared view");
    assert_eq!(
        set.validate_inputs_for(SizeInstantiationPath::Regeneration, &canonical, &inputs),
        Ok(())
    );
    assert_eq!(
        set.validate_inputs_for(SizeInstantiationPath::GradeRules, &canonical, &inputs),
        Err(MtmSizeSetError::Inputs(Box::new(SizeSetInputError::Mtm(
            Box::new(MtmChartError::GradeRulesUnsupported {
                chart: id(70),
                member: id(41)
            })
        ))))
    );
    assert!(!format!("{set:?}").contains("Client"));
}

#[test]
fn partial_mapping_remains_a_draft_and_does_not_default_unmapped_pom() {
    let (canonical, inputs, definition) = fixture(true, 70);
    let set = MtmSizeSet::new(definition, &canonical, &inputs).expect("actual partial chart");
    assert_eq!(set.validate_current(&canonical, &inputs), Ok(()));
    assert!(matches!(
        set.validate_inputs_for(SizeInstantiationPath::Regeneration, &canonical, &inputs),
        Err(MtmSizeSetError::Inputs(_))
    ));
    assert!(matches!(
        set.validate_inputs_for(SizeInstantiationPath::GradeRules, &canonical, &inputs),
        Err(MtmSizeSetError::Inputs(_))
    ));
    assert_eq!(
        set.definition().chart.definition().ease_set.entries.len(),
        1
    );
}

#[test]
fn chart_identity_is_disjoint_from_actual_common_axes() {
    let (canonical, inputs, definition) = fixture(false, 80);
    assert_eq!(
        MtmSizeSet::new(definition, &canonical, &inputs).expect_err("cross-source identity alias"),
        MtmSizeSetError::DuplicateIdentity {
            identity: id(80),
            first: MtmSizeSetPosition::Common(SizeSetMetadataPosition::Axis { index: 0 }),
            duplicate: MtmSizeSetPosition::Chart
        }
    );
}

#[test]
fn changed_canonical_sources_refuse_before_visiting_and_restore_without_cached_success() {
    let (original, inputs, definition) = fixture(false, 70);
    let set = MtmSizeSet::new(definition, &original, &inputs).expect("original actual sources");
    let replacement = CanonicalInputs::new(
        original
            .lengths()
            .iter()
            .filter(|record| record.id() != id(13))
            .cloned()
            .collect(),
        vec![],
    )
    .expect("removed actual Ease declaration");
    let visited = Cell::new(false);
    let error = set
        .with_inputs(&replacement, &inputs, |_| visited.set(true))
        .expect_err("missing current Ease");
    assert!(!visited.get());
    assert!(matches!(error, MtmSizeSetError::Metadata(_)));
    assert_eq!(set.validate_current(&original, &inputs), Ok(()));
}

#[test]
fn composite_error_sources_expose_original_concrete_chart_mtm_and_break_causes() {
    use std::error::Error;
    let chart = SizeChartCollectionError::MissingTable {
        chart: id(70),
        table: id(999),
    };
    let mtm = MtmChartError::GradeRulesUnsupported {
        chart: id(70),
        member: id(41),
    };
    let breaks = SizeBreaksError::Singleton {
        set: id(40),
        member: id(41),
    };
    let chart_error = SizeSetInputError::Chart(Box::new(chart.clone()));
    let mtm_error = SizeSetInputError::Mtm(Box::new(mtm.clone()));
    let break_error = SizeSetInputError::Breaks(Box::new(breaks.clone()));
    let concrete = [
        chart_error
            .source()
            .and_then(|cause| cause.downcast_ref::<SizeChartCollectionError>())
            == Some(&chart),
        mtm_error
            .source()
            .and_then(|cause| cause.downcast_ref::<MtmChartError>())
            == Some(&mtm),
        break_error
            .source()
            .and_then(|cause| cause.downcast_ref::<SizeBreaksError>())
            == Some(&breaks),
    ];
    assert_eq!(
        concrete,
        [true, true, true],
        "all composite source branches must expose concrete original causes"
    );
}
