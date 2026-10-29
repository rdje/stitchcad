//! Real original source ownership and existing conversion/identity; no derived execution claim.
#![allow(clippy::expect_used)]
use sc_core::recipe::{
    FormulaExpression, FormulaExpressionDocument as Document,
    FormulaExpressionDocumentError as Error, FormulaNamespace,
};

#[test]
fn owned_original_text_and_source_borrowed_views_preserve_spans_alias_identity_and_replacement() {
    let document = {
        let original = String::from(" \t(2.5 cm + eps_num)  ");
        Document::new(original).expect("actual original expression")
    };
    let before = document.clone();
    let normalized = document.normalized().expect("actual view");
    let span = normalized.root().span();
    assert_eq!(
        document.source().get(span.start()..span.end()),
        Some("(2.5 cm + eps_num)")
    );
    assert_eq!(&normalized.canonical_form(), document.canonical());
    assert_eq!(normalized.node_count(), 3);
    drop(normalized);
    assert_eq!(
        document.normalized().expect("repeat").canonical_form(),
        *document.canonical()
    );
    assert_eq!(document, before);
    let alias = Document::new("25 mm+eps_num".into()).expect("same normalized expression");
    assert_eq!(alias.canonical(), document.canonical());
    assert_ne!(alias, document);
    let different = Document::new("26 mm+eps_num".into()).expect("different source");
    assert_ne!(different.canonical(), document.canonical());
    let mut detached = document.source().to_owned();
    detached.clear();
    assert_eq!(document, before);
    assert!(!format!("{document:?}").contains("eps_num"));
}

#[test]
fn empty_malformed_header_and_non_ascii_sources_keep_original_syntax_refusals() {
    for source in [
        "",
        " \n\t",
        "1 mm+",
        "let width:length=1 mm",
        "é+1 mm",
        "1^3",
    ] {
        let original = FormulaExpression::parse(source).expect_err("original syntax refusal");
        let actual = Document::new(source.into()).expect_err("no partial document");
        assert_eq!(actual, Error::Syntax(original));
        assert!(std::error::Error::source(&actual).is_some());
    }
}

#[test]
fn all_branch_literal_refusals_keep_existing_rules_and_original_local_spans() {
    for source in [
        " (1000.000001 m) ",
        "if(1==1,1 mm,(1000.000001 m))",
        "340282366920938463463374607431768211456",
    ] {
        let original = FormulaExpression::parse(source)
            .expect("valid grammar")
            .normalize_literals()
            .expect_err("actual literal refusal");
        let actual = Document::new(source.into()).expect_err("no partial normalized document");
        assert_eq!(actual, Error::Literal(original));
        assert!(std::error::Error::source(&actual).is_some());
    }
}

#[test]
fn syntax_ownership_does_not_evaluate_operators_resolve_names_or_supply_a_kind_proof() {
    for source in [
        "missing+1 mm/0",
        "sqrt(-(1 mm)^2)",
        "1 mm+1 deg",
        "missing_call(1)",
    ] {
        let document = Document::new(source.into()).expect("syntax and literals only");
        let view = document.normalized().expect("same actual inputs");
        assert_eq!(&view.canonical_form(), document.canonical());
    }
    let document = Document::new("missing+1 mm/0".into()).expect("syntax only");
    let normalized = document.normalized().expect("inputs");
    let error = normalized
        .check_kinds(&FormulaNamespace::new([]).expect("namespace"))
        .expect_err("actual unbound static name precedes any operator evaluation");
    assert_eq!(error.token(), "formula_unbound_name");
}
