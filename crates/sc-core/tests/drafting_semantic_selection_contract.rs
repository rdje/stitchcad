//! Original raw row buffers/pin loans and fixed relative keys,without source/coverage admission.
#[path = "support/drafting_semantic_selection_resources.rs"]
mod resources;
use sc_core::{
    construction::*,
    ontology::{SeamSideId, SelectedProfilePin, SelectedProfilePinDefinition},
};
use std::error::Error;
type TestResult = Result<(), Box<dyn Error>>;
fn pin(label: &str) -> Result<SelectedProfilePin, Box<dyn Error>> {
    Ok(SelectedProfilePin::new(SelectedProfilePinDefinition {
        profile_id: label.into(),
        schema_version: " opaque schema ".into(),
        content_version: " original content ".into(),
    })?)
}
#[test]
fn original_vector_pins_duplicate_keys_and_invalid_requested_coordinates_survive() -> TestResult {
    let first = pin(" private original é ")?;
    let second = pin(" other original ")?;
    let location = DraftingSemanticSelectionLocation::Pocket {
        pocket: 0,
        field: DraftingPocketSelectionField::Opening,
    };
    let rows = vec![
        DraftingSemanticSelectionRow::Profile {
            location,
            selected: &first,
        },
        DraftingSemanticSelectionRow::Rule {
            ordinal: 0,
            origin: DraftingRuleSelection::Design,
        },
        DraftingSemanticSelectionRow::Profile {
            location: DraftingSemanticSelectionLocation::Semantic {
                ordinal: 0,
                field: DraftingSemanticSelectionField::LogicalInput { index: 999 },
            },
            selected: &first,
        },
        DraftingSemanticSelectionRow::Profile {
            location: DraftingSemanticSelectionLocation::Native {
                position: 12,
                field: DraftingNativeSelectionField::WalkNotchProfile {
                    pair_index: 0,
                    side: SeamSideId::A,
                    role: DraftingNotchProfileRole::SampleDepth,
                },
            },
            selected: &first,
        },
        DraftingSemanticSelectionRow::Profile {
            location: DraftingSemanticSelectionLocation::Native {
                position: 12,
                field: DraftingNativeSelectionField::WalkNotchProfile {
                    pair_index: 1,
                    side: SeamSideId::A,
                    role: DraftingNotchProfileRole::SampleDepth,
                },
            },
            selected: &first,
        },
        DraftingSemanticSelectionRow::Rule {
            ordinal: 0,
            origin: DraftingRuleSelection::Profile(&second),
        },
        DraftingSemanticSelectionRow::Profile {
            location,
            selected: &second,
        },
        DraftingSemanticSelectionRow::Profile {
            location: DraftingSemanticSelectionLocation::Closure {
                closure: 99,
                field: DraftingClosureSelectionField::Size(DraftingClosureSizeRole::Hook),
            },
            selected: &second,
        },
    ];
    let original = rows.as_ptr();
    let inputs = DraftingSemanticSelectionInputs::new(rows);
    assert_eq!(inputs.rows().len(), 8);
    assert_eq!(inputs.rows().as_ptr(), original);
    let mut iter = inputs.rows().iter().copied();
    assert!(
        matches!(iter.next(),Some(DraftingSemanticSelectionRow::Profile{location:actual,selected}) if actual==location && core::ptr::eq(selected,&first))
    );
    let design = iter.next().ok_or("explicit Design row")?;
    assert!(matches!(
        design,
        DraftingSemanticSelectionRow::Rule {
            ordinal: 0,
            origin: DraftingRuleSelection::Design
        }
    ));
    assert!(
        matches!(iter.next(),Some(DraftingSemanticSelectionRow::Profile{location:DraftingSemanticSelectionLocation::Semantic{ordinal:0,field:DraftingSemanticSelectionField::LogicalInput{index:999}},selected}) if core::ptr::eq(selected,&first))
    );
    let pair0 = iter.next().ok_or("first original pair")?;
    let pair1 = iter.next().ok_or("second original pair")?;
    assert_ne!(pair0.key(), pair1.key());
    let profile = iter.next().ok_or("explicit Profile rule")?;
    assert_eq!(design.key(), profile.key());
    assert!(
        matches!(profile,DraftingSemanticSelectionRow::Rule{ordinal:0,origin:DraftingRuleSelection::Profile(actual)} if core::ptr::eq(actual,&second))
    );
    let duplicate = iter.next().ok_or("duplicate profile key")?;
    assert_eq!(
        duplicate.key(),
        DraftingSemanticSelectionKey::Profile(location)
    );
    assert!(
        matches!(duplicate,DraftingSemanticSelectionRow::Profile{selected,..} if core::ptr::eq(selected,&second))
    );
    assert!(matches!(
        iter.next(),
        Some(DraftingSemanticSelectionRow::Profile {
            location: DraftingSemanticSelectionLocation::Closure { closure: 99, .. },
            ..
        })
    ));
    assert!(iter.next().is_none());
    assert!(!format!("{inputs:?} {profile:?} {duplicate:?}").contains("original"));
    Ok(())
}
#[test]
fn relative_consumer_coordinates_keep_distinct_keys_without_binding_a_scope() -> TestResult {
    let pin = pin(" explicit ")?;
    let key = |position, pair_index, side| {
        DraftingSemanticSelectionRow::Profile {
            location: DraftingSemanticSelectionLocation::Native {
                position,
                field: DraftingNativeSelectionField::WalkNotchProfile {
                    pair_index,
                    side,
                    role: DraftingNotchProfileRole::SampleDepth,
                },
            },
            selected: &pin,
        }
        .key()
    };
    assert_ne!(key(12, 0, SeamSideId::A), key(13, 0, SeamSideId::A));
    assert_ne!(key(12, 0, SeamSideId::A), key(12, 1, SeamSideId::A));
    assert_ne!(key(12, 0, SeamSideId::A), key(12, 0, SeamSideId::B));
    let semantic = DraftingSemanticSelectionLocation::Semantic {
        ordinal: 12,
        field: DraftingSemanticSelectionField::LogicalInput { index: 0 },
    };
    assert_ne!(
        key(12, 0, SeamSideId::A),
        DraftingSemanticSelectionKey::Profile(semantic)
    );
    let empty = DraftingSemanticSelectionInputs::new(vec![]);
    assert!(empty.rows().is_empty());
    Ok(())
}
