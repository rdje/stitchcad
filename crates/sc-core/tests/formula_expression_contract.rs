//! Independent machine-syntax/shape/boundary oracles; no numeric evaluation claim.
use sc_core::recipe::{
    FormulaBinaryOperator as B, FormulaExpression as E, FormulaExpressionLimit as L,
    FormulaLexicalRule as LR, FormulaNode as N, FormulaNodeKind as K, FormulaParseRule as R,
    FormulaUnit as U,
};
#[allow(clippy::expect_used)] // Contract helper: an unexpected refusal is a test failure.
fn parse(source: &str) -> E<'_> {
    E::parse(source).expect("valid expression grammar")
}
fn shape(node: N<'_>) -> String {
    match node.kind() {
        K::Name(name) => name.to_owned(),
        K::Literal { number, unit } => format!("{number}{}", unit.map_or("", U::token)),
        K::Negate(child) => format!("(neg {})", shape(child)),
        K::Square(child) => format!("(square {})", shape(child)),
        K::Binary {
            operator,
            left,
            right,
        } => format!("({operator:?} {} {})", shape(left), shape(right)),
        K::Call { name, arguments } => format!(
            "({name} {})",
            arguments.map(shape).collect::<Vec<_>>().join(" ")
        ),
        K::Conditional {
            condition,
            then_branch,
            else_branch,
        } => format!(
            "(if {} {} {})",
            shape(condition),
            shape(then_branch),
            shape(else_branch)
        ),
    }
}
#[allow(clippy::expect_used)] // Contract helper: accepting invalid syntax is a test failure.
fn refusal(source: &str, rule: R, span: (usize, usize), code: &str) {
    let error = E::parse(source).expect_err("assertion: invalid grammar must refuse");
    assert_eq!(error.rule(), rule, "{source}");
    assert_eq!((error.span().start(), error.span().end()), span, "{source}");
    assert_eq!(error.diagnostic_code(), code);
}
#[test]
fn precedence_and_left_associativity_are_normative() {
    for (source, expected) in [
        ("a + b * c", "(Add a (Multiply b c))"),
        ("a * b + c", "(Add (Multiply a b) c)"),
        ("a - b - c", "(Subtract (Subtract a b) c)"),
        ("a / b * c", "(Multiply (Divide a b) c)"),
        ("a + b < c * d", "(Less (Add a b) (Multiply c d))"),
        ("-x ^ 2", "(neg (square x))"),
        ("--x ^ 2", "(neg (neg (square x)))"),
        ("(-x) ^ 2", "(square (neg x))"),
        ("a * -b + c", "(Add (Multiply a (neg b)) c)"),
        ("-(a + b) / -c", "(Divide (neg (Add a b)) (neg c))"),
        ("(a ^ 2) ^ 2", "(square (square a))"),
    ] {
        assert_eq!(shape(parse(source).root()), expected, "{source}");
    }
}
#[test]
fn all_binary_comparisons_have_separate_syntax_roles() {
    for (token, operator) in [
        ("==", B::Equal),
        ("!=", B::NotEqual),
        ("<", B::Less),
        ("<=", B::LessEqual),
        (">", B::Greater),
        (">=", B::GreaterEqual),
    ] {
        let source = format!("a {token} b");
        let tree = parse(&source);
        let K::Binary {
            operator: actual, ..
        } = tree.root().kind()
        else {
            assert!(
                matches!(tree.root().kind(), K::Binary { .. }),
                "binary expected"
            );
            return;
        };
        assert_eq!(actual, operator);
    }
    assert_eq!(shape(parse("(a < b) < c").root()), "(Less (Less a b) c)");
    assert_eq!(shape(parse("a < (b < c)").root()), "(Less a (Less b c))");
    refusal("a < b < c", R::ChainedComparison, (6, 7), "formula_parse");
    refusal("a == b != c", R::ChainedComparison, (7, 9), "formula_parse");
}
#[test]
fn ordinary_calls_and_conditionals_keep_every_ordered_child() {
    assert_eq!(
        shape(parse("if(a < b, abs(-x), max(y, z, 1))").root()),
        "(if (Less a b) (abs (neg x)) (max y z 1))"
    );
    assert_eq!(
        shape(parse("f(a < b, c < d)").root()),
        "(f (Less a b) (Less c d))"
    );
    for name in ["abs", "if_else", "nurbs", "solve", "unknown_function"] {
        let source = format!("{name}(a)");
        let tree = parse(&source);
        let K::Call {
            name: actual,
            mut arguments,
        } = tree.root().kind()
        else {
            assert!(
                matches!(tree.root().kind(), K::Call { .. }),
                "call expected"
            );
            return;
        };
        assert_eq!(actual, name);
        assert_eq!(arguments.len(), 1);
        assert_eq!(shape(arguments.next().expect("argument")), "a");
        assert_eq!(arguments.len(), 0);
        assert!(arguments.next().is_none());
        assert!(arguments.next().is_none());
    }
    refusal("if(a, b)", R::ConditionalArity, (0, 8), "formula_parse");
    refusal(
        "if(a, b, c, d)",
        R::ConditionalArity,
        (0, 14),
        "formula_parse",
    );
    refusal("if a", R::ExpectedCallParenthesis, (3, 4), "formula_parse");
}
#[test]
fn number_text_and_closed_units_remain_unconverted_and_borrowed() {
    for unit in [
        U::Micrometre,
        U::Millimetre,
        U::Centimetre,
        U::Metre,
        U::Inch,
        U::Degree,
        U::Percent,
    ] {
        let source = format!("0001.0500 {}", unit.token());
        let tree = parse(&source);
        let K::Literal {
            number,
            unit: actual,
        } = tree.root().kind()
        else {
            assert!(
                matches!(tree.root().kind(), K::Literal { .. }),
                "literal expected"
            );
            return;
        };
        assert_eq!(number, "0001.0500");
        assert_eq!(number.as_ptr(), source.as_ptr());
        assert_eq!(actual, Some(unit));
        assert_eq!(tree.node_count(), 1);
        for gap in ["", "  ", "\t", "\n", "\r", "\x0b", "\x0c"] {
            let source = format!("1{gap}{}", unit.token());
            refusal(
                &source,
                R::UnitSeparator,
                (1, 1 + gap.len()),
                "formula_parse",
            );
        }
    }
    for number in [
        "0004",
        "01.0500",
        "999999999999999999999999999999999999999999999999",
    ] {
        assert_eq!(shape(parse(number).root()), number);
    }
    for source in ["1 rad", "1 area", "1 alien_unit", "1e3", "1 2"] {
        assert_eq!(
            E::parse(source)
                .expect_err("no adjacent atoms")
                .diagnostic_code(),
            "formula_parse"
        );
    }
}
#[test]
fn grouping_changes_extent_without_inflating_semantic_count() {
    let tree = parse("\t((a + (b))) \n");
    assert_eq!(tree.node_count(), 3);
    assert_eq!(tree.conditional_depth(), 0);
    assert_eq!(
        (tree.root().span().start(), tree.root().span().end()),
        (1, 12)
    );
    let K::Binary { left, right, .. } = tree.root().kind() else {
        assert!(
            matches!(tree.root().kind(), K::Binary { .. }),
            "binary expected"
        );
        return;
    };
    assert_eq!((left.span().start(), left.span().end()), (3, 4));
    assert_eq!((right.span().start(), right.span().end()), (7, 10));
    assert_eq!(shape(tree.root()), "(Add a b)");
    assert_eq!(parse("x ^ 2").node_count(), 2);
}
#[test]
fn malformed_delimiters_operands_keywords_and_comments_refuse_precisely() {
    for (source, rule, span) in [
        ("", R::ExpectedOperand, (0, 0)),
        (" \t", R::ExpectedOperand, (2, 2)),
        ("a +", R::ExpectedOperand, (3, 3)),
        ("+a", R::ExpectedOperand, (0, 1)),
        ("a b", R::UnexpectedToken, (2, 3)),
        ("a(1)(2)", R::UnexpectedToken, (4, 5)),
        ("(a", R::UnclosedParenthesis, (0, 1)),
        ("a)", R::UnexpectedDelimiter, (1, 2)),
        ("a, b", R::UnexpectedDelimiter, (1, 2)),
        ("()", R::ExpectedOperand, (1, 2)),
        ("f()", R::EmptyArguments, (2, 3)),
        ("if()", R::EmptyArguments, (3, 4)),
        ("f(a,)", R::ExpectedOperand, (4, 5)),
        ("f(,a)", R::ExpectedOperand, (2, 3)),
        ("let", R::ExpectedOperand, (0, 3)),
        ("assert(a)", R::ExpectedOperand, (0, 6)),
        ("a = b", R::UnexpectedToken, (2, 3)),
        ("a // note", R::ExpectedOperand, (3, 4)),
    ] {
        refusal(source, rule, span, "formula_parse");
    }
}
#[test]
fn square_payload_is_exact_and_other_exponents_are_unsupported() {
    for source in [
        "x ^ 3", "x ^ 02", "x ^ 2.0", "x ^ -2", "x ^ y", "x ^ (2)", "x ^",
    ] {
        let error = E::parse(source).expect_err("assertion: only exact 2 is supported");
        assert_eq!(error.rule(), R::UnsupportedExponent, "{source}");
        assert_eq!(error.diagnostic_code(), "formula_unsupported");
    }
    refusal("x ^ 2 ^ 2", R::RepeatedPower, (6, 7), "formula_parse");
}
#[test]
fn lexical_preflight_and_errors_preserve_exact_rules_and_locations() {
    refusal(
        "a + CUSTOMER",
        R::Lexical(LR::IdentifierSpelling),
        (4, 12),
        "formula_parse",
    );
    refusal(
        "+a é",
        R::Lexical(LR::MachineAscii),
        (3, 5),
        "formula_parse",
    );
    refusal(
        "1.",
        R::Lexical(LR::DecimalFraction),
        (1, 2),
        "formula_parse",
    );
    refusal(
        "!a",
        R::Lexical(LR::ComparisonPair),
        (0, 1),
        "formula_parse",
    );
    for whitespace in [' ', '\t', '\n', '\r', '\x0b', '\x0c'] {
        let source = format!("{whitespace}a{whitespace}+{whitespace}b{whitespace}");
        assert_eq!(shape(parse(&source).root()), "(Add a b)");
    }
}
fn nested_if(depth: usize) -> String {
    let mut source = "x".to_owned();
    for _ in 0..depth {
        source = format!("if(a < b, c, abs({source}))");
    }
    source
}
#[test]
fn semantic_node_bounds_count_calls_and_square_payload_correctly() {
    for nodes in [255, 256] {
        let source = format!("{}x", "-".repeat(nodes - 1));
        assert_eq!(parse(&source).node_count(), nodes);
    }
    let source = format!("{}x", "-".repeat(256));
    let error = E::parse(&source).expect_err("assertion: node bound");
    assert_eq!(
        error.rule(),
        R::StructuralLimit {
            limit: L::Nodes,
            measured: 257
        }
    );
    assert_eq!(error.diagnostic_code(), "formula_domain");
    assert_eq!(L::Nodes.bound(), 256);
    assert_eq!((error.span().start(), error.span().end()), (256, 257));
    let source = format!("f({})", vec!["a"; 255].join(","));
    assert_eq!(parse(&source).node_count(), 256);
    let source = format!("f({})", vec!["a"; 256].join(","));
    assert_eq!(
        E::parse(&source).expect_err("call arguments count").rule(),
        R::StructuralLimit {
            limit: L::Nodes,
            measured: 257
        }
    );
    assert_eq!(parse("f(a ^ 2, b)").node_count(), 4);
}
#[test]
fn conditional_depth_visits_calls_and_untaken_branches_but_not_siblings() {
    for depth in [15, 16] {
        let source = nested_if(depth);
        let tree = parse(&source);
        assert_eq!(tree.conditional_depth(), depth);
        assert_eq!(tree.node_count(), depth * 6 + 1);
    }
    let source = nested_if(17);
    let error = E::parse(&source).expect_err("assertion: depth bound inside call/else");
    assert_eq!(
        error.rule(),
        R::StructuralLimit {
            limit: L::ConditionalDepth,
            measured: 17
        }
    );
    assert_eq!(error.diagnostic_code(), "formula_domain");
    assert_eq!(L::ConditionalDepth.bound(), 16);
    let nested = nested_if(8);
    let source = format!("if(a < b, {nested}, {nested})");
    assert_eq!(parse(&source).conditional_depth(), 9);
}
#[test]
fn deeply_grouped_and_pathological_input_is_safe_on_a_small_thread_stack() {
    let thread = std::thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(|| {
            let source = format!("{}a{}", "(".repeat(50000), ")".repeat(50000));
            let tree = parse(&source);
            assert_eq!(tree.node_count(), 1);
            assert_eq!(tree.root().span().end(), source.len());
            drop(tree);
            for source in ["-".repeat(50000), "f(".repeat(50000)] {
                assert_eq!(
                    E::parse(&source)
                        .expect_err("bounded before recursion")
                        .rule(),
                    R::StructuralLimit {
                        limit: L::Nodes,
                        measured: 257
                    }
                );
            }
            let source = "(".repeat(50000);
            assert_eq!(
                E::parse(&source).expect_err("missing closer").rule(),
                R::UnclosedParenthesis
            );
        })
        .expect("small-stack test thread");
    thread.join().expect("no stack overflow or parser panic");
}
#[test]
fn debug_and_diagnostics_do_not_dump_customer_source() {
    let tree = parse("customer_secret + 1 cm");
    for output in [format!("{tree:?}"), format!("{:?}", tree.root())] {
        assert!(!output.contains("customer_secret"));
    }
    let error = E::parse("customer_secret +").expect_err("missing operand");
    assert!(!format!("{error:?}").contains("customer_secret"));
    assert!(!error.to_string().contains("customer_secret"));
    let cloned = tree.clone();
    assert_eq!(shape(cloned.root()), shape(tree.root()));
}
#[test]
fn every_worked_expression_has_an_explicit_syntax_only_verdict() {
    let examples = include_str!("../../../docs/book/src/spec/formula-language/examples.md");
    let mut section = "";
    let (mut bindings, mut sides, mut refused, mut deferred) = (0, 0, 0, 0);
    for line in examples.lines() {
        if line.starts_with("## ") {
            section = line;
        }
        if !line.starts_with("| `") {
            continue;
        }
        let cells: Vec<_> = line.split('|').map(str::trim).collect();
        if section.starts_with("## 2.") {
            let source = cells.get(3).expect("expression column").trim_matches('`');
            assert!(E::parse(source).is_ok(), "{source}");
            bindings += 1;
        } else if section.starts_with("## 3.") {
            let statement = cells.get(1).expect("assertion column").trim_matches('`');
            let expression = statement.split_once(" = ").expect("assert assignment").1;
            let (left, right) = expression.split_once(" == ").expect("assert separator");
            assert!(E::parse(left).is_ok());
            assert!(E::parse(right).is_ok());
            sides += 2;
        } else if section.starts_with("## 4.") {
            let form = cells
                .get(1)
                .expect("refusal column")
                .split('`')
                .nth(1)
                .expect("machine form");
            let expression = form
                .split_once(" = ")
                .map_or(form, |(_, expression)| expression);
            if expression == "\"wide\""
                || expression == "if(dart_intake > 4 cm, 5 cm)"
                || expression == "dart_intake ^ 3"
            {
                assert!(E::parse(expression).is_err());
                refused += 1;
            } else {
                assert!(E::parse(expression).is_ok(), "later owner: {expression}");
                deferred += 1;
            }
        }
    }
    assert_eq!((bindings, sides, refused, deferred), (17, 8, 3, 10));
}

#[test]
fn shared_explicit_shapes_also_agree_with_the_independent_reference() {
    let cases =
        include_str!("../../../docs/tasks/artifacts/formula_structure/expression_shapes.tsv");
    let mut checked = 0;
    for row in cases
        .lines()
        .filter(|row| !row.starts_with('#') && !row.is_empty())
    {
        let columns: Vec<_> = row.split('\t').collect();
        let source = columns.first().expect("source");
        let tree = parse(source);
        assert_eq!(
            shape(tree.root()),
            *columns.get(1).expect("shape"),
            "{source}"
        );
        assert_eq!(
            tree.node_count(),
            columns
                .get(2)
                .expect("nodes")
                .parse::<usize>()
                .expect("integer")
        );
        assert_eq!(
            tree.conditional_depth(),
            columns
                .get(3)
                .expect("depth")
                .parse::<usize>()
                .expect("integer")
        );
        checked += 1;
    }
    assert_eq!(checked, 12);
}

#[test]
fn short_token_corpus_never_panics_or_returns_an_internal_structure_refusal() {
    let tokens = ["a", "1", "(", ")", "-", "+", "*", "^", "<", ",", "if", "cm"];
    for mut code in 0..tokens.len().pow(4) {
        let mut source = String::new();
        for _ in 0..4 {
            source.push_str(tokens.get(code % tokens.len()).expect("corpus digit"));
            source.push(' ');
            code /= tokens.len();
        }
        match E::parse(&source) {
            Err(error) => assert_ne!(error.rule(), R::InternalStructure, "{source}"),
            Ok(tree) => {
                let mut stack = vec![tree.root()];
                let mut visited = 0;
                while let Some(node) = stack.pop() {
                    visited += 1;
                    match node.kind() {
                        K::Negate(child) | K::Square(child) => stack.push(child),
                        K::Binary { left, right, .. } => {
                            stack.push(left);
                            stack.push(right);
                        }
                        K::Call { arguments, .. } => stack.extend(arguments),
                        K::Conditional {
                            condition,
                            then_branch,
                            else_branch,
                        } => {
                            stack.push(condition);
                            stack.push(then_branch);
                            stack.push(else_branch);
                        }
                        K::Name(_) | K::Literal { .. } => {}
                    }
                }
                assert_eq!(
                    visited,
                    tree.node_count(),
                    "every arena node belongs to root: {source}"
                );
            }
        }
    }
}
