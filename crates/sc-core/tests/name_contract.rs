//! Machine identifiers retain meaning across input paths, locales and metadata replacement.
#![allow(clippy::unwrap_used, clippy::expect_used)]
use sc_core::name::{MachineToken, MachineTokenError};
use std::collections::BTreeSet;

#[test]
fn canonical_measurement_and_parameter_spellings_are_preserved_exactly() {
    for spelling in [
        "waist_girth",
        "hip_girth",
        "waist_to_hem",
        "a",
        "point_2_x",
        "x2",
    ] {
        let token = MachineToken::new(spelling).unwrap();
        assert_eq!(token.as_str(), spelling);
        assert_eq!(token.to_string(), spelling);
    }
}

#[test]
fn invalid_spelling_never_gets_trimmed_normalized_or_silently_repaired() {
    for spelling in [
        "",
        " waist_girth",
        "waist_girth ",
        "waist girth",
        "waist-girth",
        "Waist_Girth",
        "waist_Girth",
        "waist__girth",
        "_waist",
        "waist_",
        "1_waist",
        "waist.girth",
        "waist\ngirth",
        "taille_é",
        "waist_２",
        "wаist",
        "waist\0girth",
    ] {
        assert_eq!(
            MachineToken::new(spelling),
            Err(MachineTokenError::InvalidSyntax(spelling.to_owned()))
        );
    }
}

#[test]
fn statement_keywords_refuse_but_similarly_named_identifiers_do_not() {
    for keyword in ["let", "assert", "if"] {
        let error = MachineToken::new(keyword).unwrap_err();
        assert_eq!(
            error,
            MachineTokenError::ReservedKeyword(keyword.to_owned())
        );
        assert!(error.to_string().contains(keyword));
    }
    for spelling in ["letter", "assertion", "if_enabled", "let_value"] {
        assert_eq!(MachineToken::new(spelling).unwrap().as_str(), spelling);
    }
}

#[test]
fn reserved_namespace_inputs_are_valid_references_not_automatically_rebound() {
    for spelling in [
        "eps_num",
        "eps_geo",
        "eps_fmt",
        "eps_imp",
        "eps_phys",
        "size_index",
        "size_count",
        "is_base_size",
    ] {
        assert_eq!(MachineToken::new(spelling).unwrap().as_str(), spelling);
    }
    // Metadata/recipe binding must refuse redeclaration; lexical validity grants no binding authority.
}

#[test]
fn exact_byte_identity_drives_collection_keys_without_aliasing_distinct_names() {
    let names = ["waist", "waist_2", "waist2", "waist"];
    let set: BTreeSet<_> = names
        .into_iter()
        .map(|name| MachineToken::new(name).unwrap())
        .collect();
    assert_eq!(set.len(), 3);
    assert!(set.contains(&MachineToken::new("waist").unwrap()));
    assert!(set.contains(&MachineToken::new("waist2").unwrap()));
    assert!(set.contains(&MachineToken::new("waist_2").unwrap()));
}

#[test]
fn reconstructed_replacement_cannot_change_an_earlier_token() {
    let original = MachineToken::new("waist_girth").unwrap();
    let mut editable = original.as_str().to_owned();
    editable.push_str("_2");
    let replacement = MachineToken::new(editable).unwrap();
    assert_eq!(original.as_str(), "waist_girth");
    assert_eq!(replacement.as_str(), "waist_girth_2");
}
