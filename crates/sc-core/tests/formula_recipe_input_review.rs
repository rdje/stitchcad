//! Coupled syntax/input/identity proof, with semantic and numerical obligations kept explicit.
use sc_core::recipe::{
    FormulaLiteralRule as LR, FormulaNormalizedStatementKind as K, FormulaRecipe as R,
    FormulaRecipeRule as RR, FormulaStatementExpression as Part,
};

#[allow(clippy::expect_used)]
fn authored() -> Vec<(&'static str, &'static str)> {
    include_str!(
        "../../../docs/tasks/artifacts/formula_structure/canonical_worked_recipe_cases.tsv"
    )
    .lines()
    .filter(|s| !s.is_empty() && !s.starts_with('#'))
    .map(|s| s.split_once('\t').expect("authored complete statement row"))
    .collect()
}

#[allow(clippy::expect_used)]
fn worked() -> Vec<String> {
    let book = include_str!("../../../docs/book/src/spec/formula-language/examples.md");
    let mut section = "";
    let mut sources = Vec::new();
    for line in book.lines() {
        if line.starts_with("## ") {
            section = line;
        }
        if !line.starts_with("| `") {
            continue;
        }
        let cells: Vec<_> = line.split('|').map(str::trim).collect();
        if section.starts_with("## 2.") {
            sources.push(format!(
                "let {}: {} = {}",
                cells.get(1).expect("token").trim_matches('`'),
                cells.get(2).expect("kind").trim_matches('`'),
                cells.get(3).expect("expression").trim_matches('`')
            ));
        } else if section.starts_with("## 3.") {
            sources.push(
                cells
                    .get(1)
                    .expect("assertion")
                    .trim_matches('`')
                    .to_owned(),
            );
        }
    }
    sources
}

#[allow(clippy::expect_used)]
fn parsed(source: &str) -> R<'_> {
    let result = R::parse(source);
    assert!(result.is_ok(), "assertion: coupled authored syntax");
    result.expect("asserted syntax")
}

#[allow(clippy::expect_used)]
fn identity(source: &str) -> sc_core::recipe::FormulaCanonicalRecipe {
    let syntax = parsed(source);
    let result = syntax.normalize_literals();
    assert!(result.is_ok(), "assertion: coupled authored input");
    result.expect("asserted inputs").canonical_form()
}

fn expected_whole(rows: &[(&str, &str)]) -> String {
    format!(
        "(recipe {})",
        rows.iter()
            .map(|(_, bytes)| *bytes)
            .collect::<Vec<_>>()
            .join(" ")
    )
}

#[test]
#[allow(clippy::expect_used)]
fn actual_worked_population_retains_all_headers_global_locations_and_complete_identity() {
    let sources = worked();
    let rows = authored();
    assert_eq!(sources.len(), 21);
    assert_eq!(rows.len(), sources.len());
    for (source, (authored_source, _)) in sources.iter().zip(&rows) {
        assert_eq!(source, authored_source, "assertion: actual book population");
    }
    let source = sources.join("\n");
    let syntax = parsed(&source);
    assert_eq!(syntax.statements().len(), 21);
    let result = syntax.normalize_literals();
    assert!(result.is_ok(), "assertion: complete worked input");
    let normalized = result.expect("asserted input");
    assert_eq!(normalized.statements().len(), 21);
    let (mut cursor, mut operands, mut bindings, mut assertions) = (0, 0, 0, 0);
    for ((statement, original), (chunk, expected)) in normalized
        .statements()
        .iter()
        .zip(syntax.statements())
        .zip(&rows)
    {
        let header = chunk.split_once(" = ").expect("assignment").0;
        let (role_name, annotation) = header.split_once(": ").expect("annotation");
        let (role, name) = role_name.split_once(' ').expect("role/name");
        assert_eq!(statement.name(), name);
        assert_eq!(statement.name_span(), original.name_span());
        assert_eq!(statement.annotation_span(), original.annotation_span());
        assert_eq!(statement.span().start(), cursor);
        assert_eq!(statement.span().end(), cursor + chunk.len());
        assert_eq!(
            source.get(statement.name_span().start()..statement.name_span().end()),
            Some(name)
        );
        assert_eq!(
            source.get(statement.annotation_span().start()..statement.annotation_span().end()),
            Some(annotation)
        );
        assert_eq!(
            statement.name().as_ptr(),
            source
                .get(statement.name_span().start()..statement.name_span().end())
                .expect("original name")
                .as_ptr()
        );
        match statement.kind() {
            K::Let {
                declared_kind,
                expression,
            } => {
                assert_eq!(role, "let");
                assert_eq!(declared_kind.token(), annotation);
                assert!(expression.root().span().start() >= cursor);
                assert!(expression.root().span().end() <= cursor + chunk.len());
                operands += 1;
                bindings += 1;
            }
            K::Assert {
                tolerance,
                left,
                right,
            } => {
                assert_eq!(role, "assert");
                assert_eq!(tolerance.token(), annotation);
                assert!(left.root().span().end() < right.root().span().start());
                assert!(left.root().span().start() >= cursor);
                assert!(right.root().span().end() <= cursor + chunk.len());
                operands += 2;
                assertions += 1;
            }
        }
        assert_eq!(statement.canonical_form().as_str(), *expected);
        assert!(
            !format!("{statement:?}").contains(name),
            "assertion: customer-bearing name remains opaque"
        );
        cursor += chunk.len() + 1;
    }
    assert_eq!((bindings, assertions, operands), (17, 4, 25));
    assert_eq!(normalized.canonical_form().as_str(), expected_whole(&rows));
    drop(syntax);
    let clone = normalized.clone();
    drop(normalized);
    let owned = clone.canonical_form();
    drop(clone);
    drop(source);
    assert_eq!(owned.as_str(), expected_whole(&rows));
    assert_eq!(owned.clone(), owned);
}

#[test]
#[allow(clippy::expect_used)]
fn every_actual_refusal_example_has_the_correct_current_stage_without_semantic_authority() {
    let book = include_str!("../../../docs/book/src/spec/formula-language/examples.md");
    let refusals = book
        .split_once("## 4. Refusals\n")
        .expect("actual refusal section")
        .1;
    let actual: Vec<_> = refusals
        .lines()
        .filter(|s| s.starts_with("| `"))
        .map(|line| {
            let cells: Vec<_> = line.split('|').map(str::trim).collect();
            (
                cells
                    .get(1)
                    .expect("source")
                    .split('`')
                    .nth(1)
                    .expect("machine source"),
                cells.get(2).expect("diagnostic").trim_matches('`'),
            )
        })
        .collect();
    let fixtures: Vec<_> = include_str!(
        "../../../docs/tasks/artifacts/formula_structure/recipe_refusal_stage_cases.tsv"
    )
    .lines()
    .filter(|s| !s.is_empty() && !s.starts_with('#'))
    .map(|s| s.split('\t').collect::<Vec<_>>())
    .collect();
    assert_eq!(actual.len(), 13);
    assert_eq!(fixtures.len(), actual.len());
    let prefix = worked().join("\n");
    let (mut syntax_refusals, mut semantic_checks) = (0, 0);
    for ((actual_source, actual_diagnostic), cells) in actual.into_iter().zip(fixtures) {
        let source = cells.first().expect("authored source");
        let stage = cells.get(1).expect("stage");
        let diagnostic = cells.get(2).expect("eventual diagnostic");
        let expected = cells.get(3).expect("authored input identity");
        assert_eq!(actual_source, *source);
        assert_eq!(actual_diagnostic, *diagnostic);
        let statement = if source.starts_with("let ") {
            source.to_string()
        } else {
            format!("let candidate: length = {source}")
        };
        let complete = format!("{prefix}\n{statement}");
        let parsed = R::parse(&complete);
        if *stage == "syntax" {
            assert!(
                parsed.is_err(),
                "assertion: syntax refusal cannot produce whole input"
            );
            let error = parsed.expect_err("asserted syntax refusal");
            assert_eq!(error.statement_index(), Some(22));
            assert_eq!(error.diagnostic_code(), *diagnostic);
            assert_eq!(*expected, "-");
            syntax_refusals += 1;
        } else {
            assert_eq!(*stage, "semantic");
            assert!(
                parsed.is_ok(),
                "assertion: semantic check is not a syntax refusal"
            );
            let result = parsed.expect("asserted syntax").normalize_literals();
            assert!(
                result.is_ok(),
                "assertion: semantic check is not a literal-input refusal"
            );
            let normalized = result.expect("asserted input");
            assert_eq!(normalized.statements().len(), 22);
            assert_eq!(
                normalized
                    .statements()
                    .last()
                    .expect("appended statement")
                    .canonical_form()
                    .as_str(),
                *expected
            );
            let expected = "(recipe ".to_owned()
                + &authored()
                    .iter()
                    .map(|(_, bytes)| *bytes)
                    .collect::<Vec<_>>()
                    .join(" ")
                + " "
                + expected
                + ")";
            assert_eq!(normalized.canonical_form().as_str(), expected);
            semantic_checks += 1;
        }
    }
    assert_eq!((syntax_refusals, semantic_checks), (3, 10));
}

#[test]
#[allow(clippy::expect_used)]
fn later_input_refusals_abort_whole_identity_even_in_untaken_branches_and_call_arguments() {
    let prefix = worked().join("\n");
    let wide = "340282366920938463463374607431768211456";
    let long = "1000.000001 m";
    for (tail, literal, part, width) in [
        (
            format!("let candidate: length = if(flag, 1 um, {wide})"),
            wide,
            Part::Binding,
            true,
        ),
        (
            format!("assert check: eps_num = if(flag, {wide}, 1 um) == target"),
            wide,
            Part::AssertionLeft,
            true,
        ),
        (
            format!("assert check: eps_num = target == probe(1 um, {long})"),
            long,
            Part::AssertionRight,
            false,
        ),
    ] {
        let source = format!("{prefix}\n{tail}");
        let syntax = parsed(&source);
        assert_eq!(syntax.statements().len(), 22);
        let result = syntax.normalize_literals();
        assert!(
            result.is_err(),
            "assertion: no partial accepted whole input or identity"
        );
        let error = result.expect_err("asserted input refusal");
        assert_eq!(error.statement_index(), 22);
        assert_eq!(error.statement_error().expression_part(), part);
        assert_eq!(error.diagnostic_code(), "formula_domain");
        let start = source.rfind(literal).expect("original bad literal");
        assert_eq!(error.span().start(), start);
        assert_eq!(error.span().end(), start + literal.len());
        if width {
            assert!(matches!(
                error.statement_error().literal_error().rule(),
                LR::RationalWidth { .. }
            ));
            assert_eq!(
                error.statement_error().literal_error().limit_token(),
                Some("max_rational_bits")
            );
        } else {
            assert_eq!(
                error.statement_error().literal_error().rule(),
                LR::LengthDomain {
                    maximum: 1_000_000_000,
                    measured: 1_000_000_001
                }
            );
        }
        assert_eq!(syntax.statements().len(), 22);
        let prefix_inputs = parsed(&prefix).normalize_literals();
        assert!(
            prefix_inputs.is_ok(),
            "assertion: valid predecessor inputs remain usable"
        );
        assert_eq!(
            prefix_inputs
                .expect("asserted prefix")
                .canonical_form()
                .as_str(),
            expected_whole(&authored())
        );
    }
}

#[test]
#[allow(clippy::expect_used)]
fn combined_maximum_and_each_first_excess_preserve_stage_and_later_context() {
    let worker = std::thread::Builder::new().stack_size(64 * 1024).spawn(|| {
        let prefix = "let n: count = 1\n".repeat(4095);
        let mut nested = String::from("1 um");
        let mut bytes = String::from("length:1");
        for _ in 0..16 {
            nested = format!("if(flag, {nested}, 2 um)");
            bytes = format!("(if flag {bytes} length:2)");
        }
        let expression = "-".repeat(207) + &nested;
        let bytes = "(- ".repeat(207) + &bytes + &")".repeat(207);
        let tail = format!("assert last: eps_num = {expression} == {expression}");
        let source = prefix.clone() + &tail;
        let syntax = parsed(&source);
        assert_eq!(syntax.statements().len(), 4096);
        let result = syntax.normalize_literals();
        assert!(
            result.is_ok(),
            "assertion: simultaneous normative maxima accepted"
        );
        let normalized = result.expect("asserted maximum");
        let last = normalized.statements().last().expect("last statement");
        assert!(matches!(last.kind(), K::Assert { .. }));
        if let K::Assert { left, right, .. } = last.kind() {
            assert_eq!((left.node_count(), right.node_count()), (256, 256));
            assert_eq!(
                (left.conditional_depth(), right.conditional_depth()),
                (16, 16)
            );
        }
        let expected = "(recipe".to_owned()
            + &" (bind n count count:1)".repeat(4095)
            + &format!(" (assert last eps_num {bytes} {bytes}))");
        assert_eq!(normalized.canonical_form().as_str(), expected);
        let excess_source = source + "\nlet excess: count = 1";
        let result = R::parse(&excess_source);
        assert!(
            result.is_err(),
            "assertion: first excess statement refused before normalization"
        );
        let error = result.expect_err("asserted statement excess");
        assert_eq!(error.statement_index(), Some(4097));
        assert_eq!(
            error.rule(),
            RR::StatementLimit {
                bound: 4096,
                measured: 4097
            }
        );
        for bad in [
            "-".to_owned() + &expression,
            format!("if(flag, {nested}, 2 um)"),
        ] {
            let source = format!("{prefix}assert last: eps_num = 1 um == {bad}");
            let result = R::parse(&source);
            assert!(
                result.is_err(),
                "assertion: first excess operand refused before normalization"
            );
            let error = result.expect_err("asserted expression excess");
            assert_eq!(error.statement_index(), Some(4096));
            assert_eq!(error.diagnostic_code(), "formula_domain");
            assert!(error.span().start() >= prefix.len());
        }
    });
    assert!(worker.is_ok(), "assertion: small-stack worker starts");
    let result = worker.expect("asserted worker").join();
    assert!(
        result.is_ok(),
        "assertion: coupled bounds preserve flat stack"
    );
}

#[test]
#[allow(clippy::expect_used)]
fn whole_worked_identity_ignores_respelling_and_keeps_authored_order() {
    let rows = authored();
    let baseline = worked().join("\n");
    let aliases: Vec<_> = rows
        .iter()
        .map(|(source, _)| {
            let (head, expression) = source.split_once(" = ").expect("authored assignment");
            let expression = if source.starts_with("assert ") {
                expression
                    .split(" == ")
                    .map(|s| format!("({s})"))
                    .collect::<Vec<_>>()
                    .join(" == ")
            } else {
                format!("({expression})")
            };
            format!("{head} = {expression}")
        })
        .collect();
    assert_eq!(
        identity(&baseline),
        identity(&aliases.join("\t\r\n\u{c}\u{b}"))
    );
    let reverse = rows
        .iter()
        .rev()
        .map(|(source, _)| *source)
        .collect::<Vec<_>>()
        .join("\n");
    assert_ne!(identity(&baseline), identity(&reverse));
    assert_eq!(identity(&baseline).as_str(), expected_whole(&rows));
    assert!(!format!("{:?}", identity(&baseline)).contains("garment_waist"));
}
