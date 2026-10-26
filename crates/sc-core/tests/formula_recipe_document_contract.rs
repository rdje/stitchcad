//! Owned authored source remains distinct from canonical syntax and any execution proof.
use sc_core::recipe::{
    FormulaNormalizedStatementKind, FormulaRecipe, FormulaRecipeDocument as Document,
    FormulaRecipeDocumentError as Error, FormulaStatementExpression,
};

#[test]
fn source_outlives_the_callers_buffer_and_views_retain_original_spans_and_order() {
    let document = {
        let original = String::from(
            " \nlet width: length = 2.5 cm\nassert closure: eps_geo = width == 25 mm \t",
        );
        Document::new(original).unwrap()
    };
    let normalized = document.normalized().unwrap();
    assert_eq!(normalized.statements().len(), 2);
    let first = normalized.statements().first().unwrap();
    assert_eq!(first.name(), "width");
    assert_eq!(
        &document.source()[first.span().start()..first.span().end()],
        "let width: length = 2.5 cm"
    );
    let second = normalized.statements().last().unwrap();
    assert_eq!(second.name(), "closure");
    assert!(matches!(
        second.kind(),
        FormulaNormalizedStatementKind::Assert { .. }
    ));
    assert_eq!(
        &document.source()[second.name_span().start()..second.name_span().end()],
        "closure"
    );
    if let FormulaNormalizedStatementKind::Assert { left, right, .. } = second.kind() {
        assert_eq!(left.canonical_form().as_str(), "width");
        assert_eq!(right.canonical_form().as_str(), "length:25000");
    }
    assert_eq!(
        document.canonical().as_str(),
        "(recipe (bind width length length:25000) (assert closure eps_geo width length:25000))"
    );
    assert_eq!(&normalized.canonical_form(), document.canonical());
    drop(normalized);
    assert_eq!(
        document.normalized().unwrap().canonical_form(),
        document.canonical().clone()
    );
}

#[test]
fn exact_source_equality_is_separate_from_canonical_alias_equality() {
    let a = Document::new("let width: length = 2.5 cm".into()).unwrap();
    let b = Document::new(" let width : length = ((25 mm)) \n".into()).unwrap();
    assert_ne!(a, b);
    assert_eq!(a.canonical(), b.canonical());
    let c = Document::new("let width: length = 26 mm".into()).unwrap();
    assert_ne!(a.canonical(), c.canonical());
    let original = a.clone();
    let mut detached = a.source().to_owned();
    detached.clear();
    assert_eq!(a, original);
    assert!(detached.is_empty());
}

#[test]
fn syntax_refusals_are_the_original_complete_source_errors() {
    for source in [
        "let width: length =",
        "let width: length = 25 mm\nlet next: count = 1 ^ 3",
        "let é: count = 1",
    ] {
        let expected = FormulaRecipe::parse(source).unwrap_err();
        let error = Document::new(source.into()).unwrap_err();
        assert_eq!(error, Error::Syntax(expected));
        assert!(std::error::Error::source(&error).is_some());
        assert!(!error.to_string().contains(source));
    }
}

#[test]
fn literal_refusals_retain_later_ordinal_operand_and_original_location() {
    let source = "let width: length = 25 mm\nassert closure: eps_geo = width == (1000.000001 m)";
    let expected = FormulaRecipe::parse(source)
        .unwrap()
        .normalize_literals()
        .unwrap_err();
    let error = Document::new(source.into()).unwrap_err();
    assert_eq!(error, Error::Literal(expected.clone()));
    assert_eq!(expected.statement_index(), 2);
    assert_eq!(
        expected.statement_error().expression_part(),
        FormulaStatementExpression::AssertionRight
    );
    assert_eq!(
        &source[expected.span().start()..expected.span().end()],
        "(1000.000001 m)"
    );
    assert!(std::error::Error::source(&error).is_some());
}

#[test]
fn empty_documents_preserve_authored_whitespace_and_have_empty_identity() {
    for source in ["", " \n\t"] {
        let document = Document::new(source.into()).unwrap();
        assert_eq!(document.source(), source);
        assert!(document.normalized().unwrap().statements().is_empty());
        assert_eq!(document.canonical().as_str(), "(recipe)");
    }
}

#[test]
fn syntactic_ownership_does_not_check_names_kinds_or_evaluate_untaken_branches() {
    let source =
        "let value: length = missing(1) + 1 deg\nlet value: boolean = if(1 == 2, 1 / 0, 2)";
    let document = Document::new(source.into()).unwrap();
    assert_eq!(document.source(), source);
    assert_eq!(document.normalized().unwrap().statements().len(), 2);
    assert!(document.canonical().as_str().contains("missing"));
    assert!(Document::new("let value: length = if(1 == 2, 1000.000001 m, 1 mm)".into()).is_err());
}

#[test]
fn debug_omits_customer_text_and_canonical_values() {
    let document = Document::new("let confidential_width: length = 123 mm".into()).unwrap();
    let debug = format!("{document:?}");
    assert!(debug.contains("FormulaRecipeDocument"));
    assert!(!debug.contains("confidential_width"));
    assert!(!debug.contains("123"));
}
