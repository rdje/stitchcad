//! Whole input conversion contracts; byte composition here is inspection, not a product serializer.
use sc_core::recipe::{
    FormulaLiteralRule as Rule, FormulaNormalizedRecipe as N, FormulaNormalizedStatement as S,
    FormulaNormalizedStatementKind as K, FormulaRecipe as R, FormulaStatement,
    FormulaStatementExpression as Part,
};
use std::error::Error;

#[allow(clippy::expect_used)]
fn normalized(source: &str) -> N<'_> {
    let syntax = R::parse(source).expect("fixture syntax");
    let result = std::panic::catch_unwind(|| syntax.normalize_literals())
        .expect("assertion: conversion must not unwind");
    assert!(
        result.is_ok(),
        "assertion: accepted input conversion: {result:?}"
    );
    result.expect("asserted acceptance")
}
fn inspected(statement: &S<'_>) -> String {
    match statement.kind() {
        K::Let {
            declared_kind,
            expression,
        } => format!(
            "(bind {} {} {})",
            statement.name(),
            declared_kind.token(),
            expression.canonical_form().as_str()
        ),
        K::Assert {
            tolerance,
            left,
            right,
        } => format!(
            "(assert {} {} {} {})",
            statement.name(),
            tolerance.token(),
            left.canonical_form().as_str(),
            right.canonical_form().as_str()
        ),
    }
}
fn decode(source: &str) -> String {
    if source == "-" {
        return String::new();
    }
    source
        .replace("\\t", "\t")
        .replace("\\n", "\n")
        .replace("\\r", "\r")
        .replace("\\f", "\x0c")
        .replace("\\v", "\x0b")
}

#[test]
#[allow(clippy::expect_used)]
fn authored_statement_headers_and_all_operand_inputs_match_reference_bytes() {
    let fixture = include_str!(
        "../../../docs/tasks/artifacts/formula_structure/canonical_statement_cases.tsv"
    );
    let mut rows = 0;
    for row in fixture.lines().filter(|row| !row.starts_with('#')) {
        let (source, expected) = row.split_once('\t').expect("authored source/bytes");
        let syntax = FormulaStatement::parse(source).expect("authored syntax");
        let result = syntax.normalize_literals();
        assert!(
            result.is_ok(),
            "assertion: authored input accepted: {result:?}"
        );
        let converted = result.expect("asserted acceptance");
        assert_eq!(inspected(&converted), expected);
        assert_eq!(converted.span(), syntax.span());
        assert_eq!(converted.name(), syntax.name());
        assert_eq!(converted.name_span(), syntax.name_span());
        assert_eq!(converted.annotation_span(), syntax.annotation_span());
        assert_eq!(converted.name().as_ptr(), syntax.name().as_ptr());
        drop(syntax);
        assert_eq!(inspected(&converted.clone()), expected);
        rows += 1;
    }
    assert_eq!(rows, 16);
}

#[test]
#[allow(clippy::expect_used)]
fn authored_whole_sources_keep_empty_order_and_every_statement() {
    let fixture =
        include_str!("../../../docs/tasks/artifacts/formula_structure/canonical_recipe_cases.tsv");
    let mut rows = 0;
    for row in fixture.lines().filter(|row| !row.starts_with('#')) {
        let cells: Vec<_> = row.split('\t').collect();
        assert_eq!(cells.len(), 3);
        let source = decode(cells.first().expect("source"));
        let syntax = R::parse(&source).expect("syntax");
        let converted = normalized(&source);
        assert_eq!(converted.statements().len(), syntax.statements().len());
        for (original, statement) in syntax.statements().iter().zip(converted.statements()) {
            assert_eq!(statement.span(), original.span());
            assert_eq!(statement.name(), original.name());
            assert_eq!(statement.name_span(), original.name_span());
            assert_eq!(statement.annotation_span(), original.annotation_span());
        }
        let body = converted
            .statements()
            .iter()
            .map(inspected)
            .collect::<Vec<_>>()
            .join(" ");
        let actual = if body.is_empty() {
            String::from("(recipe)")
        } else {
            format!("(recipe {body})")
        };
        assert_eq!(actual, *cells.get(2).expect("authored bytes"));
        rows += 1;
    }
    assert_eq!(rows, 9);
}

#[test]
#[allow(clippy::expect_used)]
fn source_borrows_global_child_spans_clone_and_opaque_views_survive_syntax_drop() {
    let source = String::from(" \nlet customer_width: length = customer_base + ((0002.500 cm))\nassert customer_check: eps_geo = (0001 mm) == -720 deg ");
    let syntax = R::parse(&source).expect("syntax");
    let normalized = syntax.normalize_literals().expect("valid literal inputs");
    drop(syntax);
    let cloned = normalized.clone();
    drop(normalized);
    let statement = cloned.statements().first().expect("first");
    let K::Let { expression, .. } = statement.kind() else {
        assert!(matches!(statement.kind(), K::Let { .. }));
        return;
    };
    use sc_core::recipe::FormulaNormalizedNodeKind as Node;
    let Node::Binary { left, right, .. } = expression.root().kind() else {
        assert!(matches!(expression.root().kind(), Node::Binary { .. }));
        return;
    };
    assert_eq!(
        source.get(left.span().start()..left.span().end()),
        Some("customer_base")
    );
    assert_eq!(
        source.get(right.span().start()..right.span().end()),
        Some("((0002.500 cm))")
    );
    let Node::Literal(literal) = right.kind() else {
        assert!(matches!(right.kind(), Node::Literal(_)));
        return;
    };
    assert_eq!(literal.span(), right.span());
    assert_eq!(literal.number(), "0002.500");
    assert_eq!(literal.magnitude(), 25000);
    assert_eq!(
        literal.number().as_ptr(),
        source
            .as_bytes()
            .get(source.find("0002.500").expect("offset")..)
            .expect("suffix")
            .as_ptr()
    );
    assert_eq!(
        statement.name().as_ptr(),
        source
            .as_bytes()
            .get(source.find("customer_width").expect("offset")..)
            .expect("suffix")
            .as_ptr()
    );
    for debug in [
        format!("{cloned:?}"),
        format!("{statement:?}"),
        format!("{:?}", statement.kind()),
        format!("{expression:?}"),
    ] {
        for sensitive in [
            "customer_width",
            "customer_base",
            "customer_check",
            "0002.500",
            "25000",
        ] {
            assert!(
                !debug.contains(sensitive),
                "assertion: private diagnostic shape: {debug}"
            );
        }
    }
}

#[test]
#[allow(clippy::expect_used)]
fn all_operand_refusals_retain_later_ordinal_rule_span_and_source_chain() {
    let prefix = "let first: count = 1\nassert prior: eps_num = 1 == 1\n";
    let wide = "340282366920938463463374607431768211456";
    for (tail, needle, part, width) in [
        (
            format!("let customer_fail: count = ({wide})"),
            format!("({wide})"),
            Part::Binding,
            true,
        ),
        (
            format!("assert customer_fail: eps_geo = probe({wide}) == 1 um"),
            wide.to_owned(),
            Part::AssertionLeft,
            true,
        ),
        (
            String::from("assert customer_fail: eps_geo = 1 um == (1000.000001 m)"),
            String::from("(1000.000001 m)"),
            Part::AssertionRight,
            false,
        ),
        (
            String::from("let customer_fail: length = if(1 == 1, 1 um, 1000.000001 m)"),
            String::from("1000.000001 m"),
            Part::Binding,
            false,
        ),
    ] {
        let standalone = FormulaStatement::parse(&tail).expect("refusal is conversion, not syntax");
        let single = standalone.normalize_literals();
        assert!(
            single.is_err(),
            "assertion: refused input must not normalize"
        );
        let single = single.expect_err("asserted refusal");
        assert_eq!(single.expression_part(), part);
        let source = prefix.to_owned() + &tail + "\nlet fourth: count = 4";
        let syntax = R::parse(&source).expect("syntax");
        let result = syntax.normalize_literals();
        assert!(
            result.is_err(),
            "assertion: no partial accepted recipe: {result:?}"
        );
        let error = result.expect_err("asserted refusal");
        assert_eq!(error.statement_index(), 3);
        assert_eq!(error.statement_error().expression_part(), part);
        let start = prefix.len() + tail.find(&needle).expect("authored span");
        assert_eq!(error.span().start(), start);
        assert_eq!(error.span().end(), start + needle.len());
        assert_eq!(error.span(), error.statement_error().span());
        assert_eq!(error.span(), error.statement_error().literal_error().span());
        assert_eq!(error.diagnostic_code(), "formula_domain");
        assert_eq!(
            error.statement_error().diagnostic_code(),
            error.diagnostic_code()
        );
        let literal = error.statement_error().literal_error();
        if width {
            assert_eq!(literal.rational_bit_bound(), Some(128));
            assert_eq!(literal.limit_token(), Some("max_rational_bits"));
            assert!(matches!(
                literal.rule(),
                Rule::RationalWidth {
                    measured_bits_at_least: 129,
                    ..
                }
            ));
        } else {
            assert_eq!(
                literal.rule(),
                Rule::LengthDomain {
                    maximum: 1_000_000_000,
                    measured: 1_000_000_001
                }
            );
        }
        assert_eq!(single.literal_error().rule(), literal.rule());
        let cause = error.source();
        assert!(cause.is_some(), "assertion: recipe retains statement cause");
        let typed = cause
            .expect("asserted cause")
            .downcast_ref::<sc_core::recipe::FormulaStatementLiteralError>();
        assert!(
            typed.is_some(),
            "assertion: statement cause retains original type"
        );
        let nested = typed.expect("asserted statement type");
        assert_eq!(*nested, error.statement_error());
        let cause = nested.source();
        assert!(
            cause.is_some(),
            "assertion: statement retains literal cause"
        );
        let typed = cause
            .expect("asserted cause")
            .downcast_ref::<sc_core::recipe::FormulaLiteralError>();
        assert!(
            typed.is_some(),
            "assertion: literal cause retains original type"
        );
        assert_eq!(*typed.expect("asserted literal type"), literal);
        assert_eq!(syntax.statements().len(), 4);
        for display in [
            format!("{error}"),
            format!("{error:?}"),
            format!("{single}"),
            format!("{single:?}"),
        ] {
            assert!(!display.contains("customer_fail") && !display.contains(&tail));
        }
    }
}

#[test]
#[allow(clippy::expect_used)]
fn earliest_refusal_is_source_ordered_and_never_skips_an_assertion_operand() {
    let wide = "340282366920938463463374607431768211456";
    let source =
        format!("assert first: eps_num = {wide} == 1000.000001 m\nlet second: count = {wide}");
    let syntax = R::parse(&source).expect("syntax");
    let result = syntax.normalize_literals();
    assert!(result.is_err(), "assertion: first operand refused");
    let error = result.expect_err("asserted refusal");
    assert_eq!(error.statement_index(), 1);
    assert_eq!(
        error.statement_error().expression_part(),
        Part::AssertionLeft
    );
    assert_eq!(
        error.span().start(),
        source.find(wide).expect("first input")
    );
    let source =
        format!("let first: length = 1000.000001 m\nassert second: eps_num = {wide} == {wide}");
    let syntax = R::parse(&source).expect("syntax");
    let result = syntax.normalize_literals();
    assert!(result.is_err(), "assertion: earliest statement refused");
    assert_eq!(result.expect_err("asserted refusal").statement_index(), 1);
}

#[test]
fn normalization_supplies_no_binding_type_name_tolerance_or_evaluation_authority() {
    let source = "let duplicate: boolean = 340282366920938463463374607431768211455\nlet duplicate: count = -1\nlet forward: length = unknown(1 um / 0, missing, if(flag, 360 deg, -720 deg))\nassert unchecked: eps_phys = 1 cm == 2 cm";
    let recipe = normalized(source);
    assert_eq!(recipe.statements().len(), 4);
    let actual: Vec<_> = recipe.statements().iter().map(inspected).collect();
    assert_eq!(actual, [
        "(bind duplicate boolean count:340282366920938463463374607431768211455)",
        "(bind duplicate count (- count:1))",
        "(bind forward length (unknown (/ length:1 count:0) missing (if flag angle:360000000 (- angle:720000000))))",
        "(assert unchecked eps_phys length:10000 length:20000)",
    ]);
}

#[test]
#[allow(clippy::expect_used)]
fn every_statement_and_operand_limit_normalizes_clones_and_drops_on_small_stack() {
    std::thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(|| {
            let mut expression = String::from("1 um");
            for _ in 0..16 {
                expression = format!("if(flag, {expression}, 2 um)");
            }
            let expression = "-".repeat(207) + &expression;
            let source = (0..4096)
                .map(|index| {
                    format!("assert check_{index}: eps_num = {expression} == {expression}\n")
                })
                .collect::<String>();
            let syntax = R::parse(&source).expect("maximal syntax");
            let result = syntax.normalize_literals();
            assert!(result.is_ok(), "assertion: independent normative maxima");
            let recipe = result.expect("asserted acceptance");
            drop(syntax);
            let clone = recipe.clone();
            drop(recipe);
            assert_eq!(clone.statements().len(), 4096);
            let mut nodes = 0;
            for statement in clone.statements() {
                let K::Assert { left, right, .. } = statement.kind() else {
                    assert!(matches!(statement.kind(), K::Assert { .. }));
                    continue;
                };
                assert_eq!(left.node_count(), 256);
                assert_eq!(right.node_count(), 256);
                assert_eq!(left.conditional_depth(), 16);
                assert_eq!(right.conditional_depth(), 16);
                assert_eq!(left.canonical_form(), right.canonical_form());
                nodes += left.node_count() + right.node_count();
            }
            assert_eq!(nodes, 4096 * 2 * 256);
            drop(clone);
        })
        .expect("small-stack conversion")
        .join()
        .expect("assertion: flat conversion/clone/drop");
}

#[test]
#[allow(clippy::expect_used)]
fn independent_decimal_fraction_rows_reach_every_whole_statement_operand() {
    let fixture = include_str!(
        "../../../docs/tasks/artifacts/formula_structure/literal_normalization_cases.tsv"
    );
    let mut rows = 0;
    for row in fixture.lines().filter(|row| !row.starts_with('#')) {
        let cells: Vec<_> = row.split('\t').collect();
        assert_eq!(cells.len(), 3);
        let literal = cells.first().expect("literal");
        let kind = cells.get(1).expect("kind");
        let expected = cells.get(2).expect("magnitude/refusal");
        for (tail, part) in [
            (format!("let value: {kind} = {literal}"), Part::Binding),
            (
                format!("assert check: eps_num = probe({literal}) == 1"),
                Part::AssertionLeft,
            ),
            (
                format!("assert check: eps_geo = 1 == if(flag, 1, {literal})"),
                Part::AssertionRight,
            ),
        ] {
            let source = String::from("let prior: count = 1\n") + &tail;
            let syntax = R::parse(&source).expect("literal syntax");
            let result = syntax.normalize_literals();
            if *expected == "width" || *expected == "length" {
                assert!(result.is_err(), "assertion: authored refusal: {literal}");
                let error = result.expect_err("asserted refusal");
                assert_eq!(error.statement_index(), 2);
                assert_eq!(error.statement_error().expression_part(), part);
                if *expected == "width" {
                    assert!(matches!(
                        error.statement_error().literal_error().rule(),
                        Rule::RationalWidth { .. }
                    ));
                } else {
                    assert!(matches!(
                        error.statement_error().literal_error().rule(),
                        Rule::LengthDomain { .. }
                    ));
                }
            } else {
                assert!(result.is_ok(), "assertion: authored conversion: {literal}");
                let recipe = result.expect("asserted acceptance");
                let statement = recipe.statements().get(1).expect("second");
                let token = format!("{kind}:{expected}");
                let bytes = inspected(statement);
                let wanted = match part {
                    Part::Binding => format!("(bind value {kind} {token})"),
                    Part::AssertionLeft => {
                        format!("(assert check eps_num (probe {token}) count:1)")
                    }
                    Part::AssertionRight => {
                        format!("(assert check eps_geo count:1 (if flag count:1 {token}))")
                    }
                };
                assert_eq!(bytes, wanted, "assertion: {literal}");
            }
        }
        rows += 1;
    }
    assert_eq!(rows, 100);
}
