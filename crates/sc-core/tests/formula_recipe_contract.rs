//! Whole original-source recipe syntax, independently authored boundaries and scoped diagnostics.
use sc_core::recipe::{
    FormulaExpression as E, FormulaExpressionLimit as L, FormulaLexicalRule as X,
    FormulaNodeKind as N, FormulaParseRule as P, FormulaRecipe as R, FormulaRecipeRule as Rule,
    FormulaStatement as S, FormulaStatementExpression as Part, FormulaStatementKind as K,
    FormulaStatementRule as SR,
};

#[allow(clippy::expect_used)]
fn parse(source: &str) -> R<'_> {
    let caught = std::panic::catch_unwind(|| R::parse(source));
    assert!(caught.is_ok(), "assertion: recipe parser must not unwind");
    let result = caught.expect("asserted no unwind");
    assert!(
        result.is_ok(),
        "assertion: valid recipe refused: {:?}",
        result.as_ref().err()
    );
    result.expect("asserted acceptance")
}
#[allow(clippy::expect_used)]
fn refuse(source: &str) -> sc_core::recipe::FormulaRecipeError {
    let result = R::parse(source);
    assert!(result.is_err(), "assertion: invalid recipe accepted");
    result.expect_err("asserted refusal")
}
#[allow(clippy::expect_used)]
fn canonical(expression: &E<'_>) -> String {
    let result = expression.normalize_literals();
    assert!(result.is_ok(), "assertion: fixture normalization");
    result
        .expect("asserted conversion")
        .canonical_form()
        .into_string()
}
fn role(condition: bool) {
    assert!(condition, "assertion: expected statement/node/error role");
}
fn decode(source: &str) -> String {
    source
        .replace("\\t", "\t")
        .replace("\\r", "\r")
        .replace("\\n", "\n")
        .replace("\\f", "\x0c")
        .replace("\\v", "\x0b")
}
fn operands(statement: &S<'_>) -> String {
    match statement.kind() {
        K::Let { expression, .. } => canonical(expression),
        K::Assert { left, right, .. } => format!("{} ~ {}", canonical(left), canonical(right)),
    }
}

#[test]
#[allow(clippy::expect_used)]
fn authored_original_boundaries_preserve_order_headers_and_operand_bytes() {
    let fixture = include_str!("../../../docs/tasks/artifacts/formula_structure/recipe_cases.tsv");
    let mut rows = 0;
    for row in fixture.lines().filter(|row| !row.starts_with('#')) {
        let fields: Vec<_> = row.split('\t').collect();
        assert_eq!(fields.len(), 3);
        let source = decode(fields.first().expect("source"));
        let source = if source == "-" { String::new() } else { source };
        let chunks = fields.get(1).expect("ordered chunks");
        let expected: Vec<_> = if *chunks == "-" {
            Vec::new()
        } else {
            chunks.split(" | ").map(decode).collect()
        };
        let bytes = fields.get(2).expect("operand identities");
        let identities: Vec<_> = if *bytes == "-" {
            Vec::new()
        } else {
            bytes.split(" | ").collect()
        };
        let recipe = parse(&source);
        assert_eq!(recipe.statements().len(), expected.len());
        assert_eq!(identities.len(), expected.len());
        let mut cursor = 0;
        for (index, (statement, chunk)) in recipe.statements().iter().zip(&expected).enumerate() {
            let start = cursor
                + source
                    .get(cursor..)
                    .expect("remaining source")
                    .find(chunk)
                    .expect("authored chunk");
            assert_eq!(statement.span().start(), start);
            assert_eq!(statement.span().end(), start + chunk.len());
            assert_eq!(
                source.get(statement.span().start()..statement.span().end()),
                Some(chunk.as_str())
            );
            let standalone = S::parse(chunk);
            assert!(
                standalone.is_ok(),
                "assertion: independently authored statement"
            );
            let standalone = standalone.expect("asserted syntax");
            assert_eq!(statement.name(), standalone.name());
            assert_eq!(
                statement.name_span().start(),
                start + standalone.name_span().start()
            );
            assert_eq!(
                statement.annotation_span().start(),
                start + standalone.annotation_span().start()
            );
            assert_eq!(
                operands(statement),
                *identities.get(index).expect("authored bytes")
            );
            cursor = start + chunk.len();
        }
        if expected.len() > 1 {
            assert!(
                S::parse(&source).is_err(),
                "assertion: standalone still consumes whole input"
            );
        }
        rows += 1;
    }
    assert_eq!(rows, 9);
}

#[test]
#[allow(clippy::expect_used)]
fn later_statement_nodes_retain_whole_source_locations() {
    let source = "let first: count = 1\n\nlet second: length = (1 cm + 25 mm)\nassert check: eps_num = second == 35 mm";
    let recipe = parse(source);
    let statement = recipe.statements().get(1).expect("second statement");
    let K::Let { expression, .. } = statement.kind() else {
        role(false);
        return;
    };
    let root = expression.root();
    assert_eq!(
        source.get(root.span().start()..root.span().end()),
        Some("(1 cm + 25 mm)")
    );
    let N::Binary { left, right, .. } = root.kind() else {
        role(false);
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
    let K::Assert { left, right, .. } = recipe.statements().get(2).expect("third").kind() else {
        role(false);
        return;
    };
    assert_eq!(
        source.get(left.root().span().start()..left.root().span().end()),
        Some("second")
    );
    assert_eq!(
        source.get(right.root().span().start()..right.root().span().end()),
        Some("35 mm")
    );
}

#[test]
fn recipe_errors_retain_one_based_statement_and_nested_rule() {
    for (source, index, code, offending) in [
        (" \n 1", 1, "formula_parse", "1"),
        (
            "let a: count = 1\nlet if: count = 2",
            2,
            "formula_parse",
            "if",
        ),
        (
            "let a: count = 1 let let: count = 2",
            2,
            "formula_parse",
            "let",
        ),
        (
            "let a: count = 1\nlet b: point = p",
            2,
            "formula_dimension",
            "point",
        ),
        (
            "let a: count = 1\nassert b: eps_chord = a == b",
            2,
            "formula_tolerance_unbound",
            "eps_chord",
        ),
        (
            "let a: count = 1\nlet b: count = 1 ^ 3",
            2,
            "formula_unsupported",
            "3",
        ),
        (
            "let a: count = 1\nlet b: length = 25\nmm",
            2,
            "formula_parse",
            "\n",
        ),
        (
            "let a: count = 1\nlet Upper: count = 2",
            2,
            "formula_parse",
            "Upper",
        ),
        (
            "let a: count = 1\nlet b: count = (let c: count = 2)",
            2,
            "formula_parse",
            "let",
        ),
    ] {
        let error = refuse(source);
        assert_eq!(error.statement_index(), Some(index), "assertion: {source}");
        assert_eq!(error.diagnostic_code(), code);
        assert_eq!(
            source.get(error.span().start()..error.span().end()),
            Some(offending)
        );
        let Rule::Statement(nested) = error.rule() else {
            role(false);
            continue;
        };
        assert_eq!(nested.span(), error.span());
        assert_eq!(nested.diagnostic_code(), code);
    }
    let source = "let a: count = 1\nassert b: eps_num = a == let c: count = 3";
    let error = refuse(source);
    assert_eq!(error.statement_index(), Some(2));
    assert_eq!(source.get(error.span().start()..), Some("let c: count = 3"));
    assert_eq!(error.span().start(), error.span().end());
    assert!(
        matches!(error.rule(), Rule::Statement(nested) if matches!(nested.rule(), SR::Expression { part: Part::AssertionRight, .. }))
    );
    let source = "let a: count = 1\nassert b: eps_num = a == b + \n";
    let error = refuse(source);
    assert_eq!(error.statement_index(), Some(2));
    assert_eq!(error.span().start(), source.len());
    assert_eq!(error.span().end(), source.len());
}

#[test]
fn whole_source_preflight_and_trailing_junk_never_accept_a_prefix() {
    for source in [
        "let a: count = 1;",
        "let a: count = 1 # comment",
        "let a: count = 1 // comment",
        "let a: count = 1 trailing",
        "let a: count = (1 let b: count = 2)",
    ] {
        assert_eq!(refuse(source).statement_index(), Some(1));
    }
    for source in [
        "let a: count = 1\nµ",
        "let if: count = 1 let b: count = é",
        "\u{2003}",
    ] {
        let error = refuse(source);
        assert_eq!(error.statement_index(), None);
        assert_eq!(error.diagnostic_code(), "formula_parse");
        assert!(
            matches!(error.rule(), Rule::Statement(nested) if nested.rule() == SR::Lexical(X::MachineAscii))
        );
        assert!(source
            .get(error.span().start()..error.span().end())
            .is_some_and(|text| !text.is_ascii()));
    }
}

#[test]
fn exact_statement_bound_refuses_first_excess_before_its_body() {
    assert_eq!(R::MAX_STATEMENTS, 4096);
    let source = "let a: count = 1\n".repeat(4096);
    assert_eq!(parse(&source).statements().len(), 4096);
    for excess in [
        "let b: count = 2",
        "assert b: eps_num = 1 == 1",
        "let invalid header",
        "assert",
    ] {
        let whole = source.clone() + excess + "\nlet c: count = 3";
        let error = refuse(&whole);
        assert_eq!(error.statement_index(), Some(4097));
        assert_eq!(error.span().start(), source.len());
        assert_eq!(
            error.span().end(),
            source.len() + if excess.starts_with("let") { 3 } else { 6 }
        );
        assert_eq!(error.diagnostic_code(), "formula_domain");
        assert_eq!(
            error.rule(),
            Rule::StatementLimit {
                bound: 4096,
                measured: 4097
            }
        );
    }
    let unicode = source.clone() + "let b: count = µ";
    assert_eq!(refuse(&unicode).statement_index(), None);
    let junk = source + "garbage";
    let error = refuse(&junk);
    assert_eq!(error.statement_index(), Some(4096));
    assert_eq!(error.diagnostic_code(), "formula_parse");
}

#[test]
#[allow(clippy::expect_used)]
fn small_stack_recipe_keeps_independent_expression_limits_and_flat_clone_drop() {
    std::thread::Builder::new().stack_size(64 * 1024).spawn(|| {
        let chain = "-".repeat(255) + "1";
        let groups = "(".repeat(50_000) + "1" + &")".repeat(50_000);
        let name = "a".repeat(100_000);
        let source = format!("let {name}: count = {groups}\nassert b: eps_num = {chain} == {chain}");
        let recipe = parse(&source);
        let clone = recipe.clone();
        drop(recipe);
        assert_eq!(clone.statements().first().expect("first").name(), name);
        let K::Assert { left, right, .. } = clone.statements().get(1).expect("second").kind() else { role(false); return; };
        assert_eq!(left.node_count(), 256);
        assert_eq!(right.node_count(), 256);
        drop(clone);
        let full = "let a: count = 1\n".repeat(4096);
        let full = parse(&full);
        let cloned = full.clone();
        drop(full);
        assert_eq!(cloned.statements().len(), 4096);
        drop(cloned);
        let excess = "-".repeat(256) + "1";
        for tail in [format!("let b: count = {excess}"), format!("assert b: eps_num = {excess} == 1"), format!("assert b: eps_num = 1 == {excess}")] {
            let source = format!("let a: count = 1\n{tail}");
            let error = refuse(&source);
            assert_eq!(error.statement_index(), Some(2));
            assert!(matches!(error.rule(), Rule::Statement(nested) if matches!(nested.rule(), SR::Expression { error, .. } if error.rule() == P::StructuralLimit { limit: L::Nodes, measured: 257 })));
        }
        let mut depth = String::from("1");
        for _ in 0..16 { depth = format!("if(a, {depth}, 2)"); }
        parse(&format!("let a: count = 1\nassert b: eps_num = {depth} == {depth}"));
        let excess = format!("if(a, {depth}, 2)");
        let source = format!("let a: count = 1\nlet b: count = {excess}");
        let error = refuse(&source);
        assert_eq!(error.statement_index(), Some(2));
        assert!(matches!(error.rule(), Rule::Statement(nested) if matches!(nested.rule(), SR::Expression { error, .. } if error.rule() == P::StructuralLimit { limit: L::ConditionalDepth, measured: 17 })));
    }).expect("small-stack worker").join().expect("assertion: bounded flat recipe");
}

#[test]
fn inspection_is_explicit_and_debug_errors_omit_customer_content() {
    let source = "let customer_secret: length = account_secret + 123456789 um\nassert check_secret: eps_phys = customer_secret == account_secret";
    let recipe = parse(source);
    let error = refuse(&(source.to_owned() + " ^ 3"));
    for output in [
        format!("{recipe:?}"),
        format!("{error:?}"),
        error.to_string(),
    ] {
        for customer in [
            "customer_secret",
            "account_secret",
            "123456789",
            "check_secret",
        ] {
            assert!(
                !output.contains(customer),
                "assertion: customer leak {customer}"
            );
        }
    }
    let source = "let a: count = -1 let a: boolean = unknown(a) assert a: eps_phys = 1 cm == 2 cm";
    assert_eq!(parse(source).statements().len(), 3);
    let source = "let a: count = 999999999999999999999999999999999999999999999999999999999999999\nlet b: length = 1 um / 0";
    assert_eq!(parse(source).statements().len(), 2);
}

#[test]
#[allow(clippy::expect_used)]
fn all_worked_statements_compose_in_authored_order_with_unchanged_operand_identity() {
    let chapter = include_str!("../../../docs/book/src/spec/formula-language/examples.md");
    let bytes =
        include_str!("../../../docs/tasks/artifacts/formula_structure/canonical_worked_cases.tsv");
    let expected: Vec<_> = bytes
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| line.split_once('\t').expect("authored row").1)
        .collect();
    let (mut section, mut source, mut names) = ("", String::new(), Vec::new());
    for line in chapter.lines() {
        if line.starts_with("## ") {
            section = line;
        }
        if !line.starts_with("| `") {
            continue;
        }
        let cells: Vec<_> = line.split('|').map(str::trim).collect();
        if section.starts_with("## 2.") {
            let name = cells.get(1).expect("name").trim_matches('`');
            source.push_str(&format!(
                "let {name}: {} = {}\n",
                cells.get(2).expect("kind"),
                cells.get(3).expect("expression").trim_matches('`')
            ));
            names.push(name.to_owned());
        } else if section.starts_with("## 3.") {
            let assertion = cells.get(1).expect("assertion").trim_matches('`');
            names.push(
                assertion
                    .split_whitespace()
                    .nth(1)
                    .expect("assertion name")
                    .trim_end_matches(':')
                    .to_owned(),
            );
            source.push_str(assertion);
            source.push('\n');
        }
    }
    let recipe = parse(&source);
    assert_eq!(recipe.statements().len(), 21);
    assert_eq!(
        recipe.statements().iter().map(S::name).collect::<Vec<_>>(),
        names
    );
    let mut actual = Vec::new();
    for statement in recipe.statements() {
        match statement.kind() {
            K::Let { expression, .. } => actual.push(canonical(expression)),
            K::Assert { left, right, .. } => {
                actual.push(canonical(left));
                actual.push(canonical(right));
            }
        }
    }
    assert_eq!(actual, expected);
    assert_eq!(actual.len(), 25);
}

#[test]
#[allow(clippy::expect_used)]
fn coupled_token_boundaries_preserve_zero_gap_and_all_whitespace() {
    let fixture =
        include_str!("../../../docs/tasks/artifacts/formula_structure/recipe_coupled_cases.tsv");
    let mut rows = 0;
    for row in fixture.lines().filter(|row| !row.starts_with('#')) {
        let cells: Vec<_> = row.split('\t').collect();
        assert_eq!(cells.len(), 3);
        let source = cells.first().expect("whole source");
        let chunks: Vec<_> = cells
            .get(1)
            .expect("authored chunks")
            .split(" | ")
            .collect();
        let bytes: Vec<_> = cells.get(2).expect("operand bytes").split(" | ").collect();
        let recipe = parse(source);
        assert_eq!(recipe.statements().len(), 2);
        assert_eq!(chunks.len(), 2);
        assert_eq!(bytes.len(), 2);
        let mut cursor = 0;
        for (index, statement) in recipe.statements().iter().enumerate() {
            let chunk = chunks.get(index).expect("original statement");
            assert_eq!(statement.span().start(), cursor);
            assert_eq!(statement.span().end(), cursor + chunk.len());
            assert_eq!(source.get(cursor..cursor + chunk.len()), Some(*chunk));
            assert_eq!(
                operands(statement),
                *bytes.get(index).expect("authored identity")
            );
            cursor += chunk.len();
        }
        assert_eq!(cursor, source.len());
        rows += 1;
    }
    assert_eq!(rows, 4);
    for whitespace in [" ", "\t", "\r", "\n", "\x0c", "\x0b"] {
        let source = format!("let{whitespace}a:{whitespace}count{whitespace}={whitespace}1{whitespace}let{whitespace}b: count = a + 2{whitespace}");
        let recipe = parse(&source);
        assert_eq!(recipe.statements().len(), 2);
        assert_eq!(recipe.statements().first().expect("first").name(), "a");
        assert_eq!(recipe.statements().get(1).expect("second").name(), "b");
    }
    for source in [
        "let a: count = blet b: count = 2",
        "let a: length = 1 mmlet b: count = 2",
        "let a: count = 1let_value(2)",
        "let a: count = 1assertion(2)",
    ] {
        let error = refuse(source);
        assert_eq!(error.statement_index(), Some(1));
        assert_eq!(error.diagnostic_code(), "formula_parse");
    }
}

#[test]
fn coupled_later_header_and_operand_rules_keep_exact_global_provenance() {
    let prefix = "let first: count = 1\nassert prior: eps_num = 1 == 1\n";
    for (tail, expected, start, end) in [
        ("let", SR::ExpectedName, 3, 3),
        ("let let: count = 1", SR::ExpectedName, 4, 7),
        ("assert if: eps_num = a == b", SR::ExpectedName, 7, 9),
        ("let b", SR::ExpectedColon, 5, 5),
        ("let b count = 1", SR::ExpectedColon, 6, 11),
        ("let b:", SR::ExpectedAnnotation, 6, 6),
        ("let b: 1 = 1", SR::ExpectedAnnotation, 7, 8),
        ("let b: point = p", SR::UnbindableKind, 7, 12),
        ("assert b: eps_chord = a == b", SR::UnknownTolerance, 10, 19),
        ("let b: count", SR::ExpectedAssignment, 12, 12),
        ("let b: count == 1", SR::ExpectedAssignment, 13, 15),
        (
            "let Upper: count = 1",
            SR::Lexical(X::IdentifierSpelling),
            4,
            9,
        ),
    ] {
        let source = prefix.to_owned() + tail;
        let error = refuse(&source);
        assert_eq!(error.statement_index(), Some(3));
        assert_eq!(
            error.span().start(),
            prefix.len() + start,
            "assertion: {tail}"
        );
        assert_eq!(error.span().end(), prefix.len() + end, "assertion: {tail}");
        let Rule::Statement(nested) = error.rule() else {
            role(false);
            continue;
        };
        assert_eq!(nested.rule(), expected, "assertion: {tail}");
        assert_eq!(nested.span(), error.span());
    }
    for (tail, part, rule, needle, width) in [
        (
            "let b: count = \nlet fourth: count = 4",
            Part::Binding,
            P::ExpectedOperand,
            "let fourth",
            0,
        ),
        (
            "assert b: eps_num = == a",
            Part::AssertionLeft,
            P::ExpectedOperand,
            "==",
            0,
        ),
        (
            "assert b: eps_num = a == \nlet fourth: count = 4",
            Part::AssertionRight,
            P::ExpectedOperand,
            "let fourth",
            0,
        ),
        (
            "let b: length = 1\tcm",
            Part::Binding,
            P::UnitSeparator,
            "\t",
            1,
        ),
        (
            "assert b: eps_num = a == 1 ^ 3",
            Part::AssertionRight,
            P::UnsupportedExponent,
            "3",
            1,
        ),
        (
            "assert b: eps_num = a! == b",
            Part::AssertionLeft,
            P::Lexical(X::ComparisonPair),
            "!",
            1,
        ),
        (
            "assert b: eps_num = a == b!",
            Part::AssertionRight,
            P::Lexical(X::ComparisonPair),
            "!",
            1,
        ),
        (
            "let b: count = (let fourth: count = 4)",
            Part::Binding,
            P::ExpectedOperand,
            "let fourth",
            3,
        ),
    ] {
        let source = prefix.to_owned() + tail;
        let error = refuse(&source);
        assert_eq!(error.statement_index(), Some(3));
        #[allow(clippy::expect_used)]
        let expected = prefix.len() + tail.find(needle).expect("authored offending span");
        assert_eq!(error.span().start(), expected, "assertion: {tail}");
        assert_eq!(error.span().end(), expected + width, "assertion: {tail}");
        let Rule::Statement(nested) = error.rule() else {
            role(false);
            continue;
        };
        let SR::Expression {
            part: actual,
            error: nested,
        } = nested.rule()
        else {
            role(false);
            continue;
        };
        assert_eq!(actual, part);
        assert_eq!(nested.rule(), rule);
        assert_eq!(nested.span(), error.span());
        assert_eq!(nested.diagnostic_code(), error.diagnostic_code());
    }
}

#[test]
#[allow(clippy::expect_used)]
fn coupled_all_statement_and_expression_limits_reach_their_maximum_together() {
    std::thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(|| {
            // Each conditional adds three nodes to a literal;207 unary nodes complete256.
            let mut expression = String::from("1 um");
            for _ in 0..16 {
                expression = format!("if(flag, {expression}, 2 um)");
            }
            let expression = "-".repeat(207) + &expression;
            let mut source = String::new();
            for index in 0..4096 {
                source.push_str(&format!(
                    "assert check_{index}: eps_num = {expression} == {expression}\n"
                ));
            }
            let recipe = parse(&source);
            assert_eq!(recipe.statements().len(), 4096);
            let mut nodes = 0;
            for statement in recipe.statements() {
                let K::Assert { left, right, .. } = statement.kind() else {
                    role(false);
                    continue;
                };
                assert_eq!(left.node_count(), 256);
                assert_eq!(right.node_count(), 256);
                assert_eq!(left.conditional_depth(), 16);
                assert_eq!(right.conditional_depth(), 16);
                nodes += left.node_count() + right.node_count();
            }
            assert_eq!(nodes, 4096 * 2 * 256);
            drop(recipe);
        })
        .expect("small-stack combined limits")
        .join()
        .expect("assertion: independent simultaneous syntax limits");
}
