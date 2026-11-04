//! Exact cumulative row/pin content and original first field causes,without target/source lookup.
use super::TestResult;
use sc_core::{construction::*, design::*, ontology::*};
pub(super) fn pin(
    p: &str,
    s: &str,
    c: &str,
) -> Result<SelectedProfilePin, Box<dyn std::error::Error>> {
    Ok(SelectedProfilePin::new(SelectedProfilePinDefinition {
        profile_id: p.into(),
        schema_version: s.into(),
        content_version: c.into(),
    })?)
}
pub(super) fn limits(
    entries: usize,
    text: usize,
    bits: u64,
) -> Result<DesignResourceLimits, Box<dyn std::error::Error>> {
    use std::num::{NonZeroU64, NonZeroUsize};
    Ok(DesignResourceLimits::new(
        NonZeroUsize::new(entries).ok_or("positive entries")?,
        NonZeroUsize::new(text).ok_or("positive text")?,
        NonZeroU64::new(bits).ok_or("positive bits")?,
    ))
}
pub(super) fn rows<'a>(
    p: &'a SelectedProfilePin,
    q: &'a SelectedProfilePin,
) -> DraftingSemanticSelectionInputs<'a> {
    let location = DraftingSemanticSelectionLocation::Native {
        position: 0,
        field: DraftingNativeSelectionField::WalkNotchProfile {
            pair_index: usize::MAX,
            side: SeamSideId::B,
            role: DraftingNotchProfileRole::SampleDepth,
        },
    };
    DraftingSemanticSelectionInputs::new(vec![
        DraftingSemanticSelectionRow::Profile {
            location,
            selected: p,
        },
        DraftingSemanticSelectionRow::Rule {
            ordinal: 0,
            origin: DraftingRuleSelection::Design,
        },
        DraftingSemanticSelectionRow::Profile {
            location,
            selected: p,
        },
        DraftingSemanticSelectionRow::Rule {
            ordinal: usize::MAX,
            origin: DraftingRuleSelection::Profile(q),
        },
    ])
}
fn at(field: &'static str, index: Option<usize>) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::CommandState,
        owner: Some(EntityId::from_bits(42)),
        field,
        index,
    }
}
#[test]
fn all_actual_repeated_and_rule_profile_pins_compose_with_prior_content() -> TestResult {
    let p = pin(" p é ", " s ", " c ")?;
    let q = pin(" q ", " é ", " z ")?;
    let rows = rows(&p, &q);
    let mut meter = DesignResourceMeter::new(limits(4, 34, 1)?);
    rows.charge_record_resources(
        &mut meter,
        DesignResourceScope::CommandState,
        Some(EntityId::from_bits(42)),
    )?;
    let d = meter.finish()?;
    assert_eq!(
        (d.collection_entries(), d.text_bytes(), d.counter_bits()),
        (4, 34, 0)
    );
    let mut meter = DesignResourceMeter::new(limits(6, 37, 1)?);
    meter.charge_entries(2, at("prior", None))?;
    meter.charge_text("abc", at("prior", None))?;
    meter.charge_counter_bits(1, at("prior", None))?;
    rows.charge_record_resources(
        &mut meter,
        DesignResourceScope::CommandState,
        Some(EntityId::from_bits(42)),
    )?;
    let d = meter.finish()?;
    assert_eq!(
        (d.collection_entries(), d.text_bytes(), d.counter_bits()),
        (6, 37, 1)
    );
    Ok(())
}
#[test]
fn complete_vector_and_unused_final_pin_text_refuse_at_original_owner_and_row() -> TestResult {
    let p = pin(" p é ", " s ", " c ")?;
    let q = pin(" q ", " é ", " z ")?;
    let rows = rows(&p, &q);
    let mut meter = DesignResourceMeter::new(limits(3, 1000, 1)?);
    let expected = DesignResourceLimitError::Count {
        unit: DesignResourceCount::CollectionEntries,
        maximum: 3,
        consumed: 0,
        requested: 4,
        location: at("drafting.semantic.selections.rows", None),
    };
    assert_eq!(
        rows.charge_record_resources(
            &mut meter,
            DesignResourceScope::CommandState,
            Some(EntityId::from_bits(42))
        ),
        Err(expected)
    );
    assert_eq!(meter.finish(), Err(expected));
    let mut meter = DesignResourceMeter::new(limits(4, 33, 1)?);
    let expected = DesignResourceLimitError::Count {
        unit: DesignResourceCount::TextBytes,
        maximum: 33,
        consumed: 31,
        requested: 3,
        location: at("selected_profile.content_version", Some(3)),
    };
    assert_eq!(
        rows.charge_record_resources(
            &mut meter,
            DesignResourceScope::CommandState,
            Some(EntityId::from_bits(42))
        ),
        Err(expected)
    );
    assert_eq!(meter.finish(), Err(expected));
    Ok(())
}
#[test]
fn empty_and_nonempty_records_preserve_the_first_latched_cause() -> TestResult {
    let p = pin(" p é ", " s ", " c ")?;
    let q = pin(" q ", " é ", " z ")?;
    for rows in [DraftingSemanticSelectionInputs::new(vec![]), rows(&p, &q)] {
        let mut meter = DesignResourceMeter::new(limits(1, 1, 1)?);
        let original = meter
            .charge_entries(2, at("earlier", None))
            .err()
            .ok_or("earlier cause")?;
        assert_eq!(
            rows.charge_record_resources(
                &mut meter,
                DesignResourceScope::CommandState,
                Some(EntityId::from_bits(42))
            ),
            Err(original)
        );
        assert_eq!(meter.finish(), Err(original));
    }
    let empty = DraftingSemanticSelectionInputs::new(vec![]);
    let mut meter = DesignResourceMeter::new(limits(1, 1, 1)?);
    empty.charge_record_resources(&mut meter, DesignResourceScope::SourceHost, None)?;
    let d = meter.finish()?;
    assert_eq!(
        (d.collection_entries(), d.text_bytes(), d.counter_bits()),
        (0, 0, 0)
    );
    Ok(())
}
