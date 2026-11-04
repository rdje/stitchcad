//! Original Profile/pin/parameter loans and exact held pin text,without source acceptance.
use super::{limits, root, semantic_root::canonical, Source, TestResult};
use sc_core::{construction::*, design::*, ontology::*, value::*};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn pin() -> TestResult<SelectedProfilePin> {
    Ok(SelectedProfilePin::new(SelectedProfilePinDefinition {
        profile_id: "private factory é".into(),
        schema_version: " 2.0 ".into(),
        content_version: "7".into(),
    })?)
}
struct ProfileBody {
    original: String,
}
struct ParameterMetadata {
    parameter: ProfileParameterRef,
    target: LogicalInputTarget,
    original: String,
}
#[test]
fn raw_profile_sources_preserve_original_pins_inventories_records_and_distinct_parameter_roles(
) -> TestResult {
    let root = root()?;
    let source = Source::new(&root);
    let selected = pin()?;
    let inputs = canonical()?;
    let profile = DraftingSourceRecord::new(ProfileBody {
        original: "private original profile body".into(),
    });
    let metadata = DraftingSourceRecord::new(ParameterMetadata {
        parameter: ProfileParameterRef::new(id(600)),
        target: LogicalInputTarget {
            declaration: id(90),
            kind: LogicalInputKind::PocketOpening,
        },
        original: "private parameter mapping".into(),
    });
    let declaration = inputs
        .logicals()
        .declaration(metadata.data().target.declaration)
        .ok_or("actual Profile declaration")?;
    let origin = DraftingProfileSource {
        root: &root,
        selected: &selected,
        inputs: &inputs,
        profile: &profile,
        snapshot: &source.snapshot,
    };
    let parameter = DraftingProfileLogicalSource {
        profile: origin,
        parameter: &metadata.data().parameter,
        declaration,
        metadata: &metadata,
    };
    let copied = origin;
    assert!(std::ptr::eq(copied.root, &root));
    assert!(std::ptr::eq(copied.selected, &selected));
    assert!(std::ptr::eq(copied.inputs, &inputs));
    assert!(!std::ptr::eq(copied.inputs, root.inputs()));
    assert!(copied.profile.is_same_record(&profile));
    assert!(copied.snapshot.is_same_record(&source.snapshot));
    let copied = parameter;
    assert!(std::ptr::eq(copied.parameter, &metadata.data().parameter));
    assert_eq!(copied.parameter.id(), id(600));
    assert!(std::ptr::eq(copied.declaration, declaration));
    assert_eq!(copied.declaration.id(), id(90));
    assert!(copied.metadata.is_same_record(&metadata));
    assert!(
        matches!(copied.declaration.definition().state,LogicalInputState::Derived {operation} if operation==id(80))
    );
    assert_eq!(selected.definition().schema_version, " 2.0 ");
    assert_eq!(
        selected.validation(),
        ProfileBindingValidation::DeferredToG4
    );
    // Opaque payloads deliberately implement neither Clone nor Debug;carriers copy only loans.
    assert_eq!(
        copied.profile.profile.data().original,
        "private original profile body"
    );
    assert_eq!(copied.metadata.data().original, "private parameter mapping");
    for debug in [format!("{origin:?}"), format!("{parameter:?}")] {
        for private in ["private", "option_a", " 2.0 "] {
            assert!(!debug.contains(private));
        }
    }
    assert_eq!(source.trace.get(), 0);
    Ok(())
}
#[test]
fn raw_profile_responses_keep_foreign_equal_data_sources_inspectable_without_accepting_them(
) -> TestResult {
    let first = root()?;
    let other = root()?;
    let source = Source::new(&first);
    let other_source = Source::new(&other);
    let selected = pin()?;
    let other_pin = pin()?;
    let inputs = canonical()?;
    let foreign = canonical()?;
    let body = DraftingSourceRecord::new(ProfileBody {
        original: "first explicit profile".into(),
    });
    let other_body = DraftingSourceRecord::new(ProfileBody {
        original: "other explicit profile".into(),
    });
    let raw = DraftingProfileSource {
        root: &other,
        selected: &other_pin,
        inputs: &foreign,
        profile: &other_body,
        snapshot: &other_source.snapshot,
    };
    assert_eq!(selected, other_pin);
    assert!(!std::ptr::eq(raw.selected, &selected));
    assert_eq!(inputs, foreign);
    assert!(!std::ptr::eq(raw.inputs, &inputs));
    assert!(!std::ptr::eq(raw.root, &first));
    assert!(!raw.profile.is_same_record(&body));
    assert!(!raw.snapshot.is_same_record(&source.snapshot));
    assert!(std::ptr::eq(raw.root, &other));
    let metadata = DraftingSourceRecord::new(ParameterMetadata {
        parameter: ProfileParameterRef::new(id(600)),
        target: LogicalInputTarget {
            declaration: id(90),
            kind: LogicalInputKind::PocketOpening,
        },
        original: "other mapping".into(),
    });
    let detached = inputs
        .logicals()
        .declaration(id(90))
        .ok_or("detached equal member")?;
    let parameter = DraftingProfileLogicalSource {
        profile: raw,
        parameter: &metadata.data().parameter,
        declaration: detached,
        metadata: &metadata,
    };
    assert_eq!(
        Some(parameter.declaration),
        foreign.logicals().declaration(id(90))
    );
    assert!(!std::ptr::eq(
        parameter.declaration,
        foreign
            .logicals()
            .declaration(id(90))
            .ok_or("actual foreign member")?
    ));
    assert_eq!((source.trace.get(), other_source.trace.get()), (0, 0));
    Ok(())
}
#[test]
fn selected_profile_pin_resources_charge_every_original_utf8_field_and_preserve_locations_and_latches(
) -> TestResult {
    let selected = pin()?;
    let d = selected.definition();
    let [profile_bytes, schema_bytes, content_bytes] = [
        d.profile_id.len(),
        d.schema_version.len(),
        d.content_version.len(),
    ];
    let total = profile_bytes + schema_bytes + content_bytes;
    let at = |field| DesignResourceLocation {
        scope: DesignResourceScope::SourceHost,
        owner: Some(id(400)),
        field,
        index: Some(2),
    };
    let charge = |meter: &mut dyn DesignResourceSink| {
        charge_selected_profile_pin_resources(
            &selected,
            meter,
            DesignResourceScope::SourceHost,
            Some(id(400)),
            Some(2),
        )
    };
    let mut meter = DesignResourceMeter::new(limits(1, total, 1)?);
    charge(&mut meter)?;
    let demand = meter.finish()?;
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (0, total, 0)
    );
    for (maximum, consumed, requested, field) in [
        (
            profile_bytes - 1,
            0,
            profile_bytes,
            "selected_profile.profile_id",
        ),
        (
            profile_bytes + schema_bytes - 1,
            profile_bytes,
            schema_bytes,
            "selected_profile.schema_version",
        ),
        (
            total - 1,
            profile_bytes + schema_bytes,
            content_bytes,
            "selected_profile.content_version",
        ),
    ] {
        let mut meter = DesignResourceMeter::new(limits(1, maximum, 1)?);
        let expected = DesignResourceLimitError::Count {
            unit: DesignResourceCount::TextBytes,
            maximum,
            consumed,
            requested,
            location: at(field),
        };
        assert_eq!(charge(&mut meter), Err(expected));
        assert_eq!(meter.finish(), Err(expected));
    }
    let prior = DesignResourceLocation {
        scope: DesignResourceScope::CommandState,
        owner: None,
        field: "prior",
        index: None,
    };
    let mut meter = DesignResourceMeter::new(limits(1, total + 2, 1)?);
    meter.charge_entries(1, prior)?;
    meter.charge_text("é", prior)?;
    meter.charge_counter_bits(1, prior)?;
    charge(&mut meter)?;
    let demand = meter.finish()?;
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (1, total + 2, 1)
    );
    let mut meter = DesignResourceMeter::new(limits(1, 1, 1)?);
    let original = meter
        .charge_text("é", prior)
        .err()
        .ok_or("prior UTF8 text failure")?;
    assert_eq!(charge(&mut meter), Err(original));
    assert_eq!(meter.finish(), Err(original));
    assert_eq!(selected.definition().schema_version, " 2.0 ");
    assert_eq!(
        selected.validation(),
        ProfileBindingValidation::DeferredToG4
    );
    Ok(())
}
