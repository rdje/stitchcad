//! Whole identity bytes preserve authored syntax; no execution or project envelope is implied.
use sc_core::recipe::{
    FormulaCanonicalRecipe as CR, FormulaCanonicalStatement as CS, FormulaExpression as E,
    FormulaRecipe as R, FormulaStatement as S,
};

#[allow(clippy::expect_used)] // Explicit acceptance assertions precede extraction.
fn statement(source: &str) -> CS {
    let syntax = S::parse(source);
    assert!(syntax.is_ok(), "assertion: authored statement syntax");
    let normalized = syntax.expect("asserted syntax").normalize_literals();
    assert!(normalized.is_ok(), "assertion: authored statement input");
    let result = std::panic::catch_unwind(|| normalized.expect("asserted input").canonical_form());
    assert!(
        result.is_ok(),
        "assertion: statement identity must not unwind"
    );
    result.expect("asserted identity")
}

#[allow(clippy::expect_used)]
fn recipe(source: &str) -> CR {
    let syntax = R::parse(source);
    assert!(syntax.is_ok(), "assertion: authored recipe syntax");
    let normalized = syntax.expect("asserted syntax").normalize_literals();
    assert!(normalized.is_ok(), "assertion: authored recipe input");
    let result = std::panic::catch_unwind(|| normalized.expect("asserted input").canonical_form());
    assert!(result.is_ok(), "assertion: recipe identity must not unwind");
    result.expect("asserted identity")
}

fn decode(source: &str) -> String {
    if source == "-" {
        return String::new();
    }
    let out = source
        .replace("\\t", "\t")
        .replace("\\n", "\n")
        .replace("\\r", "\r")
        .replace("\\f", "\u{c}")
        .replace("\\v", "\u{b}");
    assert!(
        !out.contains('\\'),
        "assertion: authored escapes are closed"
    );
    out
}

#[test]
#[allow(clippy::expect_used)]
fn independently_authored_statement_and_whole_recipe_bytes_match_product() {
    let mut statements = 0;
    for row in include_str!(
        "../../../docs/tasks/artifacts/formula_structure/canonical_statement_cases.tsv"
    )
    .lines()
    .filter(|s| !s.is_empty() && !s.starts_with('#'))
    {
        let (source, expected) = row.split_once('\t').expect("authored row");
        let actual = statement(source);
        assert_eq!(actual.as_str(), expected, "assertion: {source}");
        assert_eq!(recipe(source).as_str(), format!("(recipe {expected})"));
        assert!(actual.as_str().is_ascii());
        assert_eq!(actual.as_str(), actual.as_str().trim());
        assert!(!actual.as_str().contains('\n'));
        statements += 1;
    }
    assert_eq!(statements, 16);
    let mut recipes = 0;
    for row in
        include_str!("../../../docs/tasks/artifacts/formula_structure/canonical_recipe_cases.tsv")
            .lines()
            .filter(|s| !s.is_empty() && !s.starts_with('#'))
    {
        let cells: Vec<_> = row.split('\t').collect();
        let source = decode(cells.first().expect("source"));
        let expected = cells.get(2).expect("authored bytes");
        let actual = recipe(&source);
        assert_eq!(actual.as_str(), *expected, "assertion: {source}");
        assert!(actual.as_str().is_ascii());
        assert_eq!(actual.as_str(), actual.as_str().trim());
        assert!(!actual.as_str().contains('\n'));
        recipes += 1;
    }
    assert_eq!(recipes, 9);
}

#[test]
fn equality_preserves_names_annotations_operands_and_statement_order() {
    let reference = statement("let width: length = 2.5 cm");
    for alias in [
        "\n let width : length = ((25 mm)) \t",
        "let width: length = 25000 um",
        "let width: length = 0.025 m",
    ] {
        assert_eq!(reference, statement(alias));
    }
    for (left, right) in [
        ("let a: length = 1 um", "let b: length = 1 um"),
        ("let a: length = 1 um", "let a: angle = 1 um"),
        ("let a: angle = 0 deg", "let a: angle = -0 deg"),
        ("let a: angle = 0 deg", "let a: angle = 360 deg"),
        ("let a: count = 0", "let a: count = 0.0"),
        ("let a: length = x + y", "let a: length = y + x"),
        ("assert a: eps_num = x == y", "assert a: eps_geo = x == y"),
        ("assert a: eps_num = x == y", "assert b: eps_num = x == y"),
        ("assert a: eps_num = x == y", "assert a: eps_num = y == x"),
    ] {
        assert_ne!(statement(left), statement(right), "{left} versus {right}");
        assert_ne!(recipe(left), recipe(right), "{left} versus {right}");
    }
    assert_eq!(
        recipe("let width: length = 25 mm\nassert c: eps_geo = width == 2.5 cm"),
        recipe(" let width : length = (0.025 m) assert c : eps_geo = (width) == 25000 um ")
    );
    for (left, right) in [
        (
            "let a: count = 1\nlet b: count = 2",
            "let b: count = 2\nlet a: count = 1",
        ),
        ("let a: count = 1\nlet a: count = 2", "let a: count = 2"),
        ("let a: count = 1\nlet a: count = 1", "let a: count = 1"),
        ("", "let a: count = 1"),
    ] {
        assert_ne!(recipe(left), recipe(right));
    }
    for empty in ["", " \t\n\r\u{c}\u{b}"] {
        assert_eq!(recipe(empty).as_str(), "(recipe)");
    }
}

#[test]
fn owned_bytes_outlive_all_sources_and_arenas_with_opaque_debug_and_exact_extraction() {
    let (single, whole) = {
        let source =
            String::from("let customer_secret: length = probe(123456789 um, account_secret)");
        (statement(&source), recipe(&source))
    };
    let expected = "(bind customer_secret length (probe length:123456789 account_secret))";
    assert_eq!(single.as_str(), expected);
    assert_eq!(whole.as_str(), format!("(recipe {expected})"));
    for debug in [format!("{single:?}"), format!("{whole:?}")] {
        for secret in ["customer_secret", "account_secret", "123456789", "length:"] {
            assert!(
                !debug.contains(secret),
                "assertion: opaque Debug leaked {secret}"
            );
        }
        assert!(debug.contains("byte_count"));
    }
    assert_eq!(single.clone(), single);
    assert_eq!(whole.clone(), whole);
    let mut extracted = single.clone().into_string();
    assert_eq!(extracted, expected);
    extracted.clear();
    assert_eq!(single.as_str(), expected);
    let mut extracted = whole.clone().into_string();
    assert_eq!(extracted, format!("(recipe {expected})"));
    extracted.clear();
    assert_eq!(whole.as_str(), format!("(recipe {expected})"));
}

#[test]
#[allow(clippy::expect_used)]
fn all_authored_expression_roles_reach_every_statement_operand() {
    let mut count = 0;
    for row in include_str!(
        "../../../docs/tasks/artifacts/formula_structure/canonical_expression_cases.tsv"
    )
    .lines()
    .filter(|s| !s.is_empty() && !s.starts_with('#'))
    {
        let (source, expected) = row.split_once('\t').expect("authored expression row");
        for (source, expected) in [
            (
                format!("let value: count = ({source})"),
                format!("(bind value count {expected})"),
            ),
            (
                format!("assert check: eps_num = ({source}) == target"),
                format!("(assert check eps_num {expected} target)"),
            ),
            (
                format!("assert check: eps_num = target == ({source})"),
                format!("(assert check eps_num target {expected})"),
            ),
        ] {
            assert_eq!(statement(&source).as_str(), expected);
            assert_eq!(recipe(&source).as_str(), format!("(recipe {expected})"));
            count += 1;
        }
    }
    assert_eq!(count, 55 * 3);
}

#[test]
#[allow(clippy::expect_used)]
fn independent_numeric_rows_reach_whole_identity_or_refuse_before_publication() {
    let mut count = 0;
    for row in include_str!(
        "../../../docs/tasks/artifacts/formula_structure/literal_normalization_cases.tsv"
    )
    .lines()
    .filter(|s| !s.is_empty() && !s.starts_with('#'))
    {
        let cells: Vec<_> = row.split('\t').collect();
        let source = cells.first().expect("source");
        let kind = cells.get(1).expect("kind");
        let expected = cells.get(2).expect("magnitude or refusal");
        for (source, expected_bytes) in [
            (
                format!("let value: count = {source}"),
                format!("(bind value count {kind}:{expected})"),
            ),
            (
                format!("assert check: eps_num = {source} == target"),
                format!("(assert check eps_num {kind}:{expected} target)"),
            ),
            (
                format!("assert check: eps_num = target == {source}"),
                format!("(assert check eps_num target {kind}:{expected})"),
            ),
        ] {
            if matches!(*expected, "width" | "length") {
                let syntax = S::parse(&source);
                assert!(syntax.is_ok(), "assertion: refusal has valid syntax");
                assert!(syntax
                    .expect("asserted syntax")
                    .normalize_literals()
                    .is_err());
                let syntax = R::parse(&source);
                assert!(syntax.is_ok(), "assertion: refusal has valid recipe syntax");
                assert!(syntax
                    .expect("asserted syntax")
                    .normalize_literals()
                    .is_err());
            } else {
                assert_eq!(statement(&source).as_str(), expected_bytes);
                assert_eq!(
                    recipe(&source).as_str(),
                    format!("(recipe {expected_bytes})")
                );
            }
            count += 1;
        }
    }
    assert_eq!(count, 100 * 3);
}

#[test]
#[allow(clippy::expect_used)]
fn actual_text_collisions_require_known_typed_identity_domains() {
    let expression = E::parse("bind(width, length, 25 mm)")
        .expect("expression syntax")
        .normalize_literals()
        .expect("input")
        .canonical_form();
    let single = statement("let width: length = 25 mm");
    assert_eq!(expression.as_str(), single.as_str());
    let expression = E::parse("recipe(bind(n, count, 1))")
        .expect("expression syntax")
        .normalize_literals()
        .expect("input")
        .canonical_form();
    let whole = recipe("let n: count = 1");
    assert_eq!(expression.as_str(), whole.as_str());
    assert_ne!(statement("let n: count = 1").as_str(), whole.as_str());
}

#[test]
#[allow(clippy::expect_used)]
fn all_four_normative_examples_match_actual_product_serialization() {
    let grammar = include_str!("../../../docs/book/src/spec/formula-language/grammar.md");
    let section = grammar
        .split_once("### 4.1 Statement and whole-recipe bytes\n")
        .expect("byte section")
        .1
        .split_once("\n## 5.")
        .expect("section end")
        .0;
    for text in [
        section,
        include_str!("../../../docs/decisions/decision_recipe-bytes.md"),
    ] {
        let block = text
            .split_once("```text\n")
            .expect("actual examples")
            .1
            .split_once("\n```")
            .expect("block end")
            .0;
        let mut source = Vec::new();
        let mut count = 0;
        for line in block.lines() {
            if let Some(expected) = line.strip_prefix("  => ") {
                let joined = source.join("\n");
                let actual = if joined == "empty or whitespace-only source" {
                    recipe("").into_string()
                } else if joined.contains('\n') {
                    recipe(&joined).into_string()
                } else {
                    statement(&joined).into_string()
                };
                assert_eq!(actual, expected, "assertion: actual normative population");
                count += 1;
                source.clear();
            } else {
                source.push(line);
            }
        }
        assert_eq!(count, 4);
        assert!(
            source.is_empty(),
            "assertion: no unpublished trailing source"
        );
    }
}

#[test]
#[allow(clippy::expect_used)]
fn simultaneous_statement_operand_and_conditional_bounds_serialize_on_small_stack() {
    let worker = std::thread::Builder::new().stack_size(64 * 1024).spawn(|| {
        let mut expression = String::from("1 um");
        let mut expected = String::from("length:1");
        for _ in 0..16 {
            expression = format!("if(flag, {expression}, 2 um)");
            expected = format!("(if flag {expected} length:2)");
        }
        expression = "-".repeat(207) + &expression;
        expected = "(- ".repeat(207) + &expected + &")".repeat(207);
        let source = (0..4096)
            .map(|i| format!("assert check_{i}: eps_num = {expression} == {expression}\n"))
            .collect::<String>();
        let expected = "(recipe".to_owned()
            + &(0..4096)
                .map(|i| format!(" (assert check_{i} eps_num {expected} {expected})"))
                .collect::<String>()
            + ")";
        let identity = recipe(&source);
        drop(source);
        assert_eq!(identity.as_str(), expected);
        assert_eq!(identity.clone(), identity);
        assert_eq!(identity.into_string(), expected);
    });
    assert!(worker.is_ok(), "assertion: small-stack worker starts");
    let result = worker.expect("asserted worker").join();
    assert!(result.is_ok(), "assertion: flat maximum whole identity");
}

#[test]
#[allow(clippy::expect_used)]
fn large_names_grouping_unary_and_wide_call_shapes_preserve_exact_bytes() {
    let worker = std::thread::Builder::new().stack_size(64 * 1024).spawn(|| {
        let name = "customer_".to_owned() + &"a".repeat(100_000);
        let grouped = "(".repeat(50_000) + "2.5 cm" + &")".repeat(50_000);
        let source = format!("let {name}: length = {grouped}");
        let expected = format!("(bind {name} length length:25000)");
        assert_eq!(statement(&source).as_str(), expected);
        assert_eq!(recipe(&source).as_str(), format!("(recipe {expected})"));
        let unary = "-".repeat(255) + "1";
        let expected = "(- ".repeat(255) + "count:1" + &")".repeat(255);
        assert_eq!(
            statement(&format!("let n: count = {unary}")).as_str(),
            format!("(bind n count {expected})")
        );
        let calls = "f(".repeat(255) + "1" + &")".repeat(255);
        let expected = "(f ".repeat(255) + "count:1" + &")".repeat(255);
        assert_eq!(
            recipe(&format!("let n: count = {calls}")).as_str(),
            format!("(recipe (bind n count {expected}))")
        );
        let names: Vec<_> = (0..255).map(|i| format!("a{i}")).collect();
        let source = format!(
            "assert check: eps_num = probe({}) == target",
            names.join(", ")
        );
        assert_eq!(
            statement(&source).as_str(),
            format!("(assert check eps_num (probe {}) target)", names.join(" "))
        );
    });
    assert!(worker.is_ok(), "assertion: small-stack worker starts");
    let result = worker.expect("asserted worker").join();
    assert!(result.is_ok(), "assertion: flat long-source whole identity");
}

#[test]
fn serialization_retains_all_inputs_without_binding_type_checks_or_execution() {
    let source = "let duplicate: boolean = 340282366920938463463374607431768211455\nlet duplicate: count = -1\nlet forward: length = unknown(1 um / 0, missing, if(flag, 360 deg, -720 deg))\nassert unchecked: eps_phys = 1 cm == 2 cm";
    assert_eq!(recipe(source).as_str(), "(recipe (bind duplicate boolean count:340282366920938463463374607431768211455) (bind duplicate count (- count:1)) (bind forward length (unknown (/ length:1 count:0) missing (if flag angle:360000000 (- angle:720000000)))) (assert unchecked eps_phys length:10000 length:20000))");
}
