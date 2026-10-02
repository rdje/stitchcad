//! Canonical bytes are expression identity, not evaluation or a project envelope.
use sc_core::recipe::{FormulaCanonicalExpression as C, FormulaExpression as E};

#[allow(clippy::expect_used)] // Fixture refusal/unwind is a failed public contract.
fn canonical(source: &str) -> C {
    let syntax = E::parse(source).expect("fixture syntax");
    let normalized = syntax
        .normalize_literals()
        .expect("fixture input conversion");
    std::panic::catch_unwind(|| normalized.canonical_form())
        .expect("assertion: canonical serialization must not unwind")
}

#[test]
#[allow(clippy::expect_used)]
fn authored_bytes_cover_every_role_symbol_precedence_and_wide_literal() {
    let fixtures = include_str!(
        "../../../docs/tasks/artifacts/formula_structure/canonical_expression_cases.tsv"
    );
    let mut count = 0;
    for row in fixtures
        .lines()
        .filter(|row| !row.is_empty() && !row.starts_with('#'))
    {
        let (source, expected) = row.split_once('\t').expect("authored fixture row");
        let actual = canonical(source);
        assert_eq!(actual.as_str(), expected, "assertion: {source}");
        assert!(actual.as_str().is_ascii());
        assert_eq!(actual.as_str(), actual.as_str().trim());
        assert!(!actual.as_str().contains('\n'));
        count += 1;
    }
    assert_eq!(count, 55);
}

#[test]
fn equality_keeps_kind_sign_operators_and_order_but_ignores_respelling() {
    let length = canonical("2.5 cm");
    for alias in ["25 mm", "25000 um", "0.025 m", " (( 25 mm )) "] {
        assert_eq!(length, canonical(alias));
    }
    for (left, right) in [
        ("0", "0.0"),
        ("0 um", "0 deg"),
        ("0 deg", "-0 deg"),
        ("-0 deg", "--0 deg"),
        ("a - b", "b - a"),
        ("a + b", "a - b"),
        ("a == b", "a != b"),
        ("foo(a, b)", "foo(b, a)"),
        ("if(a, b, c)", "if(a, c, b)"),
        ("-1 um", "neg(1 um)"),
        ("1 um ^ 2", "square(1 um)"),
        ("a + 0", "a"),
    ] {
        assert_ne!(canonical(left), canonical(right), "{left} versus {right}");
    }
}

#[test]
#[allow(clippy::expect_used)]
fn every_independent_literal_case_reaches_identity_or_refuses_before_it() {
    let fixtures = include_str!(
        "../../../docs/tasks/artifacts/formula_structure/literal_normalization_cases.tsv"
    );
    let mut count = 0;
    for row in fixtures
        .lines()
        .filter(|row| !row.is_empty() && !row.starts_with('#'))
    {
        let cells: Vec<_> = row.split('\t').collect();
        let source = cells.first().expect("source");
        let kind = cells.get(1).expect("kind");
        let expected = cells.get(2).expect("magnitude or refusal");
        let nested = format!("probe({source}, a)");
        let syntax = E::parse(&nested).expect("fixture syntax");
        let normalized = syntax.normalize_literals();
        if matches!(*expected, "width" | "length") {
            assert!(
                normalized.is_err(),
                "assertion: invalid input must not publish identity"
            );
        } else {
            assert!(
                normalized.is_ok(),
                "assertion: valid literal must normalize"
            );
            let value = normalized.expect("asserted acceptance").canonical_form();
            assert_eq!(value.as_str(), format!("(probe {kind}:{expected} a)"));
        }
        count += 1;
    }
    assert_eq!(count, 100);
}

#[test]
#[allow(clippy::expect_used)]
fn all_six_published_canonical_examples_match_exact_bytes() {
    let chapter = include_str!("../../../docs/book/src/annexes/formula-literals.md");
    let mut count = 0;
    for line in chapter.lines() {
        if let Some((source, expected)) = line.split_once("=>") {
            assert_eq!(canonical(source.trim()).as_str(), expected.trim());
            count += 1;
        }
    }
    assert_eq!(count, 6);
}

#[test]
fn owned_identity_survives_source_and_arenas_and_debug_omits_customer_data() {
    let value = {
        let source = String::from("customer_secret(123456789 um, account_secret)");
        canonical(&source)
    };
    assert_eq!(
        value.as_str(),
        "(customer_secret length:123456789 account_secret)"
    );
    let debug = format!("{value:?}");
    for secret in ["customer_secret", "account_secret", "123456789", "length:"] {
        assert!(
            !debug.contains(secret),
            "assertion: opaque Debug leaked {secret}"
        );
    }
    let clone = value.clone();
    assert_eq!(clone, value);
    let mut extracted = clone.into_string();
    extracted.clear();
    assert_eq!(
        value.as_str(),
        "(customer_secret length:123456789 account_secret)"
    );
}

#[test]
fn serialization_retains_unevaluated_operators_and_all_unknown_branches() {
    assert_eq!(
        canonical("if(a, 1 / 0, unknown_fn(b))").as_str(),
        "(if a (/ count:1 count:0) (unknown_fn b))"
    );
    assert_eq!(
        canonical("1 um + 1 deg").as_str(),
        "(+ length:1 angle:1000000)"
    );
    assert_eq!(canonical("-1").as_str(), "(- count:1)");
    let wide = u128::MAX.to_string();
    assert_eq!(
        canonical(&format!("{wide} + {wide}")).as_str(),
        format!("(+ count:{wide} count:{wide})")
    );
}

#[test]
#[allow(clippy::expect_used)]
fn flat_serialization_handles_structural_bounds_and_large_sources_on_small_stack() {
    std::thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(|| {
            let unary = "-".repeat(255) + "1";
            assert_eq!(
                canonical(&unary).as_str(),
                "(- ".repeat(255) + "count:1" + &")".repeat(255)
            );
            let calls = "f(".repeat(255) + "1" + &")".repeat(255);
            assert_eq!(
                canonical(&calls).as_str(),
                "(f ".repeat(255) + "count:1" + &")".repeat(255)
            );
            let grouping = "(".repeat(50_000) + "2.5 cm" + &")".repeat(50_000);
            assert_eq!(canonical(&grouping).as_str(), "length:25000");
            let mut source = String::from("1");
            let mut expected = String::from("count:1");
            for _ in 0..16 {
                source = format!("if(a, {source}, 2)");
                expected = format!("(if a {expected} count:2)");
            }
            assert_eq!(canonical(&source).as_str(), expected);
            assert!(E::parse(&format!("if(a, {source}, 2)")).is_err());
            assert!(E::parse(&("-".repeat(256) + "1")).is_err());
            let name = "a".repeat(100_000);
            let owned = canonical(&name);
            assert_eq!(owned.as_str(), name);
            assert_eq!(owned.clone(), owned);
        })
        .expect("small-stack worker")
        .join()
        .expect("assertion: stack-bound contracts");
}
