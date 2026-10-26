//! Exact selected profile labels remain separate from G4's registry and version grammar.
#![allow(clippy::expect_used)]
use sc_core::ontology::ProfileBindingValidation;
use sc_measure::{
    SizeSetProfilePin, SizeSetProfilePinDefinition, SizeSetProfilePinError, SizeSetProfilePinField,
};
fn definition() -> SizeSetProfilePinDefinition {
    SizeSetProfilePinDefinition {
        profile_id: "acme-cmt-pt".to_owned(),
        schema_version: "2.0".to_owned(),
        content_version: "1.3.0".to_owned(),
    }
}
#[test]
fn roadmap_pin_preserves_distinct_schema_and_content_versions_without_resolving() {
    let authored = definition();
    let pin = SizeSetProfilePin::new(authored.clone()).expect("explicit selected pin");
    assert_eq!(pin.definition(), &authored);
    assert_eq!(pin.validation(), ProfileBindingValidation::DeferredToG4);
    let mut edited = pin.definition().clone();
    edited.content_version = "1.4.0".to_owned();
    assert_ne!(pin.definition(), &edited);
    assert_eq!(pin.definition(), &authored);
}
#[test]
fn labels_are_opaque_and_preserve_exact_case_unicode_and_spacing() {
    let authored = SizeSetProfilePinDefinition {
        profile_id: " Atelier Étoile / PT ".to_owned(),
        schema_version: "Schema α".to_owned(),
        content_version: "Revision B / approved 2026".to_owned(),
    };
    let pin = SizeSetProfilePin::new(authored.clone()).expect("presence only at G1");
    assert_eq!(pin.definition(), &authored);
    assert_eq!(pin.validation(), ProfileBindingValidation::DeferredToG4);
}
#[test]
fn every_blank_field_is_refused_in_profile_schema_content_order_without_normalization() {
    for blank in ["", " \t\n", "\u{2003}"] {
        for field in [
            SizeSetProfilePinField::ProfileId,
            SizeSetProfilePinField::SchemaVersion,
            SizeSetProfilePinField::ContentVersion,
        ] {
            let mut authored = definition();
            match field {
                SizeSetProfilePinField::ProfileId => authored.profile_id = blank.to_owned(),
                SizeSetProfilePinField::SchemaVersion => authored.schema_version = blank.to_owned(),
                SizeSetProfilePinField::ContentVersion => {
                    authored.content_version = blank.to_owned()
                }
            }
            assert_eq!(
                SizeSetProfilePin::new(authored),
                Err(SizeSetProfilePinError {
                    field,
                    value: blank.to_owned()
                })
            );
        }
    }
    let mut authored = SizeSetProfilePinDefinition {
        profile_id: "".to_owned(),
        schema_version: " ".to_owned(),
        content_version: "\n".to_owned(),
    };
    assert_eq!(
        SizeSetProfilePin::new(authored.clone())
            .expect_err("profile first")
            .field,
        SizeSetProfilePinField::ProfileId
    );
    authored.profile_id = "profile".to_owned();
    assert_eq!(
        SizeSetProfilePin::new(authored)
            .expect_err("schema second")
            .field,
        SizeSetProfilePinField::SchemaVersion
    );
}
