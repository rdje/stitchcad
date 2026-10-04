//! Public standalone statement syntax contracts; no binding or evaluation implied.
use sc_core::recipe::{
    FormulaBindingKind as B, FormulaExpression as E, FormulaExpressionLimit as L,
    FormulaLexicalRule as X, FormulaNodeKind as N, FormulaParseRule as P, FormulaStatement as S,
    FormulaStatementExpression as Part, FormulaStatementKind as K, FormulaStatementRule as R,
};

#[allow(clippy::expect_used)]
fn parse(source: &str) -> S<'_> {
    let caught = std::panic::catch_unwind(|| S::parse(source));
    assert!(
        caught.is_ok(),
        "assertion: statement parser must not unwind"
    );
    let result = caught.expect("asserted no unwind");
    assert!(
        result.is_ok(),
        "assertion: valid statement {source}: {:?}",
        result.as_ref().err()
    );
    result.expect("asserted acceptance")
}
#[allow(clippy::expect_used)]
fn refuse(source: &str) -> sc_core::recipe::FormulaStatementError {
    let result = S::parse(source);
    assert!(
        result.is_err(),
        "assertion: malformed statement accepted {source}"
    );
    result.expect_err("asserted refusal")
}
#[allow(clippy::expect_used)]
fn canonical(expression: &E<'_>) -> String {
    expression
        .normalize_literals()
        .expect("explicit fixture conversion")
        .canonical_form()
        .into_string()
}

#[test]
#[allow(clippy::expect_used)]
fn independent_authored_headers_and_operand_bytes_preserve_closed_roles() {
    let fixture =
        include_str!("../../../docs/tasks/artifacts/formula_structure/statement_cases.tsv");
    let mut count = 0;
    for row in fixture
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
    {
        let cells: Vec<_> = row.split('\t').collect();
        assert_eq!(cells.len(), 6);
        let source = cells.first().expect("source");
        let statement = parse(source);
        assert_eq!(statement.name(), *cells.get(2).expect("name"));
        assert_eq!(
            source.get(statement.name_span().start()..statement.name_span().end()),
            Some(statement.name())
        );
        let annotation = cells.get(3).expect("annotation");
        assert_eq!(
            source.get(statement.annotation_span().start()..statement.annotation_span().end()),
            Some(*annotation)
        );
        match statement.kind() {
            K::Let {
                declared_kind,
                expression,
            } => {
                assert_eq!(*cells.get(1).expect("role"), "let");
                assert_eq!(declared_kind.token(), *annotation);
                assert_eq!(
                    canonical(expression),
                    *cells.get(4).expect("expression bytes")
                );
                assert_eq!(*cells.get(5).expect("unused right"), "-");
            }
            K::Assert {
                tolerance,
                left,
                right,
            } => {
                assert_eq!(*cells.get(1).expect("role"), "assert");
                assert_eq!(tolerance.token(), *annotation);
                assert_eq!(canonical(left), *cells.get(4).expect("left bytes"));
                assert_eq!(canonical(right), *cells.get(5).expect("right bytes"));
            }
        }
        count += 1;
    }
    assert_eq!(count, 15);
}

#[test]
#[allow(clippy::expect_used)]
fn original_whole_source_spans_reach_every_child_and_error_part() {
    let source = " \tlet customer: length = (1 cm + 25 mm)\r\n";
    let statement = parse(source);
    assert_eq!(
        source.get(statement.span().start()..statement.span().end()),
        Some("let customer: length = (1 cm + 25 mm)")
    );
    let K::Let { expression, .. } = statement.kind() else {
        panic_kind(false);
        return;
    };
    let root = expression.root();
    assert_eq!(
        source.get(root.span().start()..root.span().end()),
        Some("(1 cm + 25 mm)")
    );
    let N::Binary { left, right, .. } = root.kind() else {
        panic_kind(false);
        return;
    };
    assert_eq!(
        source.get(left.span().start()..left.span().end()),
        Some("1 cm")
    );
    assert_eq!(
        source.get(right.span().start()..right.span().end()),
        Some("25 mm")
    );
    for (source, part, offending) in [
        ("  let a: count = 1 ^ 3", Part::Binding, "3"),
        (" assert a: eps_num = 1\tcm == b", Part::AssertionLeft, "\t"),
        (
            " assert a: eps_num = a == 1\tcm",
            Part::AssertionRight,
            "\t",
        ),
    ] {
        let error = refuse(source);
        assert_eq!(
            source.get(error.span().start()..error.span().end()),
            Some(offending)
        );
        let R::Expression {
            part: actual,
            error: nested,
        } = error.rule()
        else {
            panic_kind(false);
            continue;
        };
        assert_eq!(actual, part);
        assert_eq!(nested.span(), error.span());
        assert_eq!(nested.diagnostic_code(), error.diagnostic_code());
    }
    for (source, expected) in [
        ("let a: count = 1!", Part::Binding),
        ("assert a: eps_num = 1! == 2", Part::AssertionLeft),
        ("assert a: eps_num = 1 == 2!", Part::AssertionRight),
    ] {
        let error = refuse(source);
        assert!(
            matches!(error.rule(), R::Expression { part, error } if part == expected && error.rule() == P::Lexical(X::ComparisonPair))
        );
        assert_eq!(
            source.get(error.span().start()..error.span().end()),
            Some("!")
        );
    }
    let source = "assert a: eps_num = 1 cm == b +  \n";
    let error = refuse(source);
    assert_eq!(error.span().start(), source.len());
    assert_eq!(error.span().end(), source.len());
}

fn panic_kind(condition: bool) {
    assert!(condition, "assertion: expected statement/node/error role");
}

#[test]
#[allow(clippy::expect_used)]
fn header_refusals_have_exact_rules_spans_and_reference_families() {
    for (source, rule, code, offending) in [
        ("1", R::ExpectedKeyword, "formula_parse", "1"),
        (
            "letter a: count = 1",
            R::ExpectedKeyword,
            "formula_parse",
            "letter",
        ),
        ("let if: count = 1", R::ExpectedName, "formula_parse", "if"),
        (
            "assert let: eps_num = a == b",
            R::ExpectedName,
            "formula_parse",
            "let",
        ),
        (
            "let a count = 1",
            R::ExpectedColon,
            "formula_parse",
            "count",
        ),
        ("let a: 1 = 1", R::ExpectedAnnotation, "formula_parse", "1"),
        (
            "let a: point = p",
            R::UnbindableKind,
            "formula_dimension",
            "point",
        ),
        (
            "let a: edge = p",
            R::UnbindableKind,
            "formula_dimension",
            "edge",
        ),
        (
            "let a: invented = p",
            R::UnbindableKind,
            "formula_dimension",
            "invented",
        ),
        (
            "assert a: eps_chord = a == b",
            R::UnknownTolerance,
            "formula_parse",
            "eps_chord",
        ),
        (
            "assert a: length = a == b",
            R::UnknownTolerance,
            "formula_parse",
            "length",
        ),
        (
            "let a: count == 1",
            R::ExpectedAssignment,
            "formula_parse",
            "==",
        ),
        (
            "let a: count 1",
            R::ExpectedAssignment,
            "formula_parse",
            "1",
        ),
    ] {
        let error = refuse(source);
        assert_eq!(error.rule(), rule, "assertion: {source}");
        assert_eq!(error.diagnostic_code(), code);
        assert_eq!(
            source.get(error.span().start()..error.span().end()),
            Some(offending)
        );
    }
    for source in ["", " \t\r\n", "let", "let a", "let a:", "let a: count"] {
        let error = refuse(source);
        assert_eq!(error.diagnostic_code(), "formula_parse");
        assert_eq!(error.span().start(), source.len());
        assert_eq!(error.span().end(), source.len());
    }
}

#[test]
#[allow(clippy::expect_used)]
fn assertion_separator_ignores_all_grouped_and_call_comparisons() {
    let source = "assert check: eps_num = probe(a == b, if(c == d, e, f)) == (g == h)";
    let statement = parse(source);
    let K::Assert { left, right, .. } = statement.kind() else {
        panic_kind(false);
        return;
    };
    assert_eq!(canonical(left), "(probe (== a b) (if (== c d) e f))");
    assert_eq!(canonical(right), "(== g h)");
    for source in [
        "assert a: eps_num = a",
        "assert a: eps_num = a == b == c",
        "assert a: eps_num = (a == b)",
    ] {
        let error = refuse(source);
        assert_eq!(error.rule(), R::AssertionSeparator);
        assert_eq!(error.diagnostic_code(), "formula_parse");
    }
    for source in [
        "assert a: eps_num = == b",
        "assert a: eps_num = a ==",
        "assert a: eps_num = a == b +",
    ] {
        assert!(matches!(refuse(source).rule(), R::Expression { .. }));
    }
}

#[test]
#[allow(clippy::expect_used)]
fn standalone_consumption_and_machine_source_rules_remain_strict() {
    for source in [
        "let a: count = 1 let b: count = 2",
        "let a: count = 1\nassert b: eps_num = 1 == 1",
        "let a: count = 1;",
        "let a: count = 1 # comment",
        "let a: count = 1 // comment",
        "let a: count = 1cm",
        "assert a: eps_num = 1 cm == 1  cm",
        "let a: count = 1 ^ 2.0",
    ] {
        assert!(
            S::parse(source).is_err(),
            "assertion: whole standalone input {source}"
        );
    }
    let source = "let a: count = 1\nµ";
    let error = refuse(source);
    assert_eq!(error.rule(), R::Lexical(X::MachineAscii));
    assert_eq!(
        source.get(error.span().start()..error.span().end()),
        Some("µ")
    );
    let source = "let Upper: count = 1";
    let error = refuse(source);
    assert_eq!(error.rule(), R::Lexical(X::IdentifierSpelling));
}

#[test]
#[allow(clippy::expect_used)]
fn no_literal_conversion_name_type_binding_or_tolerance_execution_occurs() {
    for source in [
        "let a: count = -1",
        "let a: boolean = 1 cm",
        "let eps_num: length = unknown(a)",
        "let a: length = 1 um / 0",
        "assert a: eps_phys = unknown_a == unknown_b",
        "let a: count = 999999999999999999999999999999999999999999999999999999999999999",
    ] {
        parse(source);
    }
    let statement = parse("let a: length = 99999999999999999999999999999999999999999999 cm");
    let K::Let {
        declared_kind,
        expression,
    } = statement.kind()
    else {
        panic_kind(false);
        return;
    };
    assert_eq!(declared_kind, B::Length);
    assert!(expression.normalize_literals().is_err());
    let statement = parse("assert a: eps_num = 1 cm == 2 cm");
    assert!(matches!(statement.kind(), K::Assert { .. }));
}

#[test]
#[allow(clippy::expect_used)]
fn flat_statement_bounds_apply_independently_to_both_operands_on_small_stack() {
    std::thread::Builder::new().stack_size(64 * 1024).spawn(|| {
        let valid = "-".repeat(255) + "1";
        let source = format!("assert a: eps_num = {valid} == {valid}");
        let both = parse(&source);
        let K::Assert { left, right, .. } = both.kind() else { panic_kind(false); return; };
        assert_eq!(left.node_count(), 256);
        assert_eq!(right.node_count(), 256);
        drop(both);
        for part in [Part::Binding, Part::AssertionLeft, Part::AssertionRight] {
            let excess = "-".repeat(256) + "1";
            let source = match part {
                Part::Binding => format!("let a: count = {excess}"),
                Part::AssertionLeft => format!("assert a: eps_num = {excess} == 1"),
                Part::AssertionRight => format!("assert a: eps_num = 1 == {excess}"),
            };
            let error = refuse(&source);
            assert_eq!(error.diagnostic_code(), "formula_domain");
            assert!(matches!(error.rule(), R::Expression { part: actual, error } if actual == part && error.rule() == P::StructuralLimit { limit: L::Nodes, measured: 257 }));
        }
        let groups = "(".repeat(50_000) + "1" + &")".repeat(50_000);
        let input = format!("let a: count = {groups}");
        let statement = parse(&input);
        let clone = statement.clone();
        drop(statement);
        assert_eq!(clone.name(), "a");
        drop(clone);
        let name = "a".repeat(100_000);
        let source = format!("let {name}: count = 1");
        assert_eq!(parse(&source).name(), name);
        let mut depth = String::from("1");
        for _ in 0..16 { depth = format!("if(a, {depth}, 2)"); }
        for source in [format!("let a: count = {depth}"), format!("assert a: eps_num = {depth} == {depth}")] { parse(&source); }
        let excess = format!("if(a, {depth}, 2)");
        for source in [format!("let a: count = {excess}"), format!("assert a: eps_num = 1 == {excess}"), format!("assert a: eps_num = {excess} == 1")] {
            assert!(matches!(refuse(&source).rule(), R::Expression { error, .. } if error.rule() == P::StructuralLimit { limit: L::ConditionalDepth, measured: 17 }));
        }
    }).expect("small-stack worker").join().expect("assertion: bounded flat statements");
}

#[test]
fn clone_and_all_diagnostic_debug_surfaces_omit_customer_content() {
    let source = String::from("let customer_secret: length = account_secret + 123456789 um");
    let statement = parse(&source);
    let cloned = statement.clone();
    drop(statement);
    assert_eq!(cloned.name(), "customer_secret");
    let bad = "let customer_secret: length = account_secret + 123456789 um ^ 3";
    #[allow(clippy::expect_used)]
    let error = refuse(bad);
    for output in [
        format!("{cloned:?}"),
        format!("{:?}", cloned.kind()),
        format!("{error:?}"),
        error.to_string(),
    ] {
        for customer in ["customer_secret", "account_secret", "123456789"] {
            assert!(
                !output.contains(customer),
                "assertion: customer leak {customer}"
            );
        }
    }
}

#[test]
#[allow(clippy::expect_used)]
fn every_worked_statement_retains_header_and_expression_identity() {
    let chapter = include_str!("../../../docs/book/src/spec/formula-language/examples.md");
    let bytes =
        include_str!("../../../docs/tasks/artifacts/formula_structure/canonical_worked_cases.tsv");
    let expected: Vec<_> = bytes
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| line.split_once('\t').expect("authored byte row").1)
        .collect();
    let (mut section, mut cursor, mut statements) = ("", 0, 0);
    for line in chapter.lines() {
        if line.starts_with("## ") {
            section = line;
        }
        if !line.starts_with("| `") {
            continue;
        }
        let cells: Vec<_> = line.split('|').map(str::trim).collect();
        let source = if section.starts_with("## 2.") {
            format!(
                "let {}: {} = {}",
                cells.get(1).expect("name").trim_matches('`'),
                cells.get(2).expect("kind"),
                cells.get(3).expect("expression").trim_matches('`')
            )
        } else if section.starts_with("## 3.") {
            cells
                .get(1)
                .expect("assertion")
                .trim_matches('`')
                .to_owned()
        } else {
            continue;
        };
        let statement = parse(&source);
        match statement.kind() {
            K::Let { expression, .. } => {
                assert_eq!(
                    canonical(expression),
                    *expected.get(cursor).expect("binding identity")
                );
                cursor += 1;
            }
            K::Assert { left, right, .. } => {
                assert_eq!(
                    canonical(left),
                    *expected.get(cursor).expect("left identity")
                );
                cursor += 1;
                assert_eq!(
                    canonical(right),
                    *expected.get(cursor).expect("right identity")
                );
                cursor += 1;
            }
        }
        statements += 1;
    }
    assert_eq!(statements, 21);
    assert_eq!(cursor, expected.len());
    assert_eq!(cursor, 25);
}

#[test]
#[allow(clippy::expect_used)]
fn invalid_assertion_class_is_syntax_without_runtime_context() {
    let mut refusals = 0;
    for annotation in [
        "eps_chord",
        "size_index",
        "size_count",
        "is_base_size",
        "length",
        "point",
        "edge",
        "unlisted",
        "eps_num_extra",
    ] {
        for tail in [" = 1 mm == 1 mm", " = (", "", " == 1 mm"] {
            let source = format!(" \tassert closure: {annotation}{tail}");
            let error = refuse(&source);
            assert_eq!(error.rule(), R::UnknownTolerance);
            assert_eq!(error.diagnostic_code(), "formula_parse");
            assert_eq!(
                source.get(error.span().start()..error.span().end()),
                Some(annotation)
            );
            refusals += 1;
            for (prefix, ordinal) in [
                ("", 1),
                ("let first:count=1\n", 2),
                ("let first:count=1\nassert prior:eps_num=1==1\n", 3),
            ] {
                let whole = prefix.to_owned() + &source;
                let result = sc_core::recipe::FormulaRecipe::parse(&whole);
                assert!(result.is_err());
                let recipe_error = result.expect_err("asserted syntax refusal");
                assert_eq!(recipe_error.diagnostic_code(), "formula_parse");
                assert_eq!(recipe_error.statement_index(), Some(ordinal));
                assert_eq!(
                    recipe_error.span().start(),
                    prefix.len() + error.span().start()
                );
                assert_eq!(recipe_error.span().end(), prefix.len() + error.span().end());
                assert!(
                    matches!(recipe_error.rule(), sc_core::recipe::FormulaRecipeRule::Statement(nested)
                    if nested.rule() == R::UnknownTolerance && nested.span() == recipe_error.span())
                );
                refusals += 1;
            }
        }
    }
    let mut accepted = 0;
    for annotation in ["eps_num", "eps_geo", "eps_fmt", "eps_imp", "eps_phys"] {
        let source = format!("assert closure:{annotation}=missing_left==missing_right");
        let statement = parse(&source);
        let K::Assert { tolerance, .. } = statement.kind() else {
            panic_kind(false);
            continue;
        };
        assert_eq!(tolerance.token(), annotation);
        let recipe = sc_core::recipe::FormulaRecipe::parse(&source);
        assert!(recipe.is_ok());
        accepted += 2;
    }
    println!("D146 public syntax: {refusals} invalid-class refusals / {accepted} valid-class acceptances; exact rules/spans/ordinals");
}
