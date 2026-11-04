//! Fresh complete local record/work before duplicate keys;coordinates and source coverage stay raw.
use super::{
    resources::{limits, pin, rows},
    TestResult,
};
use sc_core::{construction::*, design::*, ontology::*};
#[test]
fn unique_invalid_coordinates_admit_only_local_structure_and_fresh_content() -> TestResult {
    let p = pin(" p é ", " s ", " c ")?;
    let location = DraftingSemanticSelectionLocation::Native {
        position: 0,
        field: DraftingNativeSelectionField::WalkNotchProfile {
            pair_index: usize::MAX,
            side: SeamSideId::B,
            role: DraftingNotchProfileRole::SampleDepth,
        },
    };
    let raw = DraftingSemanticSelectionInputs::new(vec![
        DraftingSemanticSelectionRow::Rule {
            ordinal: 0,
            origin: DraftingRuleSelection::Design,
        },
        DraftingSemanticSelectionRow::Profile {
            location,
            selected: &p,
        },
    ]);
    let policy = DraftingSemanticSelectionLimits::new(limits(2, 12, 1)?, 4);
    let proof = raw.admit(policy).map_err(|e| e.to_string())?;
    assert!(core::ptr::eq(proof.inputs(), &raw));
    assert_eq!(proof.limits(), policy);
    assert_eq!(
        (
            proof.demand().collection_entries(),
            proof.demand().text_bytes(),
            proof.demand().counter_bits()
        ),
        (2, 12, 0)
    );
    assert_eq!(proof.shape().row_count(), 2);
    assert_eq!(proof.shape().key_comparison_bound(), 4);
    assert!(matches!(
        proof.inputs().rows().first(),
        Some(DraftingSemanticSelectionRow::Rule { ordinal: 0, .. })
    ));
    assert_eq!(
        proof
            .inputs()
            .rows()
            .get(1)
            .ok_or("original invalid coordinate")?
            .key(),
        DraftingSemanticSelectionKey::Profile(location)
    );
    let mut meter = DesignResourceMeter::new(limits(3, 15, 1)?);
    let at = DesignResourceLocation {
        scope: DesignResourceScope::CommandState,
        owner: None,
        field: "prior",
        index: None,
    };
    meter.charge_entries(1, at)?;
    meter.charge_text("abc", at)?;
    meter.charge_counter_bits(1, at)?;
    proof.charge_record_resources(&mut meter, DesignResourceScope::CommandState, None)?;
    let d = meter.finish()?;
    assert_eq!(
        (d.collection_entries(), d.text_bytes(), d.counter_bits()),
        (3, 15, 1)
    );
    Ok(())
}
#[test]
fn complete_content_then_work_precede_duplicate_key_refusal_with_original_witnesses() -> TestResult
{
    let p = pin(" p é ", " s ", " c ")?;
    let q = pin(" q ", " é ", " z ")?;
    let raw = rows(&p, &q);
    let policy = DraftingSemanticSelectionLimits::new(limits(3, 100, 1)?, 0);
    let error = raw.admit(policy).err().ok_or("complete vector first")?;
    assert!(core::ptr::eq(error.inputs(), &raw));
    assert_eq!(error.limits(), policy);
    assert!(error.demand().is_none() && error.shape().is_none());
    assert!(
        matches!(error.refusal(),DraftingSemanticSelectionRefusal::Resources(DesignResourceLimitError::Count{maximum:3,consumed:0,requested:4,location,..}) if location.field=="drafting.semantic.selections.rows" && location.owner.is_none())
    );
    assert!(std::error::Error::source(&error).is_some());
    let error = raw
        .admit(DraftingSemanticSelectionLimits::new(limits(4, 33, 1)?, 0))
        .err()
        .ok_or("unused final pin before work")?;
    assert!(
        matches!(error.refusal(),DraftingSemanticSelectionRefusal::Resources(DesignResourceLimitError::Count{maximum:33,consumed:31,requested:3,location,..}) if location.field=="selected_profile.content_version" && location.index==Some(3))
    );
    let error = raw
        .admit(DraftingSemanticSelectionLimits::new(limits(4, 34, 1)?, 15))
        .err()
        .ok_or("work before duplicate")?;
    assert_eq!(
        error.refusal(),
        DraftingSemanticSelectionRefusal::KeyComparisons {
            allowed: 15,
            required: 16
        }
    );
    assert_eq!(
        error.demand().ok_or("fresh complete demand")?.text_bytes(),
        34
    );
    assert_eq!(
        error.shape().ok_or("checked shape")?.key_comparison_bound(),
        16
    );
    let error = raw
        .admit(DraftingSemanticSelectionLimits::new(limits(4, 34, 1)?, 16))
        .err()
        .ok_or("original duplicate key")?;
    let key = raw.rows().first().ok_or("original first row")?.key();
    assert_eq!(
        error.refusal(),
        DraftingSemanticSelectionRefusal::DuplicateKey {
            first: 0,
            duplicate: 2,
            key
        }
    );
    assert!(core::ptr::eq(error.inputs(), &raw));
    assert!(
        matches!(error.inputs().rows().get(2),Some(DraftingSemanticSelectionRow::Profile{selected,..}) if core::ptr::eq(*selected,&p))
    );
    Ok(())
}
#[test]
fn different_rule_origins_cannot_excuse_the_same_requested_structural_key() -> TestResult {
    let p = pin(" p é ", " s ", " c ")?;
    let raw = DraftingSemanticSelectionInputs::new(vec![
        DraftingSemanticSelectionRow::Rule {
            ordinal: 0,
            origin: DraftingRuleSelection::Design,
        },
        DraftingSemanticSelectionRow::Rule {
            ordinal: 0,
            origin: DraftingRuleSelection::Profile(&p),
        },
    ]);
    let error = raw
        .admit(DraftingSemanticSelectionLimits::new(limits(2, 12, 1)?, 4))
        .err()
        .ok_or("duplicate rule key")?;
    assert_eq!(
        error.refusal(),
        DraftingSemanticSelectionRefusal::DuplicateKey {
            first: 0,
            duplicate: 1,
            key: DraftingSemanticSelectionKey::Rule { ordinal: 0 }
        }
    );
    assert_eq!(error.inputs().rows().len(), 2);
    Ok(())
}
#[test]
fn empty_local_selection_scope_admits_zero_work_without_source_or_coverage_claim() -> TestResult {
    let raw = DraftingSemanticSelectionInputs::new(vec![]);
    let proof = raw
        .admit(DraftingSemanticSelectionLimits::new(limits(1, 1, 1)?, 0))
        .map_err(|e| e.to_string())?;
    assert_eq!(
        (
            proof.demand().collection_entries(),
            proof.demand().text_bytes(),
            proof.demand().counter_bits()
        ),
        (0, 0, 0)
    );
    assert_eq!(proof.shape().row_count(), 0);
    assert_eq!(proof.shape().key_comparison_bound(), 0);
    let mut meter = DesignResourceMeter::new(limits(1, 1, 1)?);
    let at = DesignResourceLocation {
        scope: DesignResourceScope::CommandState,
        owner: None,
        field: "earlier",
        index: None,
    };
    let original = meter.charge_entries(2, at).err().ok_or("earlier latch")?;
    assert_eq!(
        proof.charge_record_resources(&mut meter, DesignResourceScope::CommandState, None),
        Err(original)
    );
    assert_eq!(meter.finish(), Err(original));
    Ok(())
}
