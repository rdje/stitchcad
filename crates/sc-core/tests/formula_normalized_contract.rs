//! Authored normalized shape/oracle, all-input traversal and immutable source/arena contracts.
use sc_core::recipe::{
    FormulaExpression as E, FormulaLiteralKind as L, FormulaLiteralRule as R,
    FormulaNormalizedExpression as N, FormulaNormalizedNode as Node,
    FormulaNormalizedNodeKind as K, FormulaParseRule,
};

#[allow(clippy::expect_used)] // Unexpected refusal or unwind is a failed public contract.
fn normalized(source: &str) -> N<'_> {
    let syntax = E::parse(source).expect("fixture syntax");
    let result = std::panic::catch_unwind(|| syntax.normalize_literals())
        .expect("assertion: normalization must not unwind");
    assert!(
        result.is_ok(),
        "assertion: valid input must normalize: {result:?}"
    );
    result.expect("asserted acceptance")
}
fn shape(node: Node<'_>) -> String {
    match node.kind() {
        K::Literal(literal) => format!("{}:{}", literal.kind().token(), literal.magnitude()),
        K::Name(name) => name.to_owned(),
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

#[test]
#[allow(clippy::expect_used)]
fn authored_shapes_match_reference_kind_order_nodes_and_depth() {
    let fixtures = include_str!(
        "../../../docs/tasks/artifacts/formula_structure/normalized_expression_shapes.tsv"
    );
    let mut count = 0;
    for row in fixtures
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
    {
        let cells: Vec<_> = row.split('\t').collect();
        let source = cells.first().expect("source");
        let tree = normalized(source);
        assert_eq!(
            shape(tree.root()),
            *cells.get(1).expect("shape"),
            "{source}"
        );
        assert_eq!(
            tree.node_count(),
            cells
                .get(2)
                .expect("nodes")
                .parse::<usize>()
                .expect("count")
        );
        assert_eq!(
            tree.conditional_depth(),
            cells
                .get(3)
                .expect("depth")
                .parse::<usize>()
                .expect("depth")
        );
        count += 1;
    }
    assert_eq!(count, 24);
}

#[test]
#[allow(clippy::expect_used)]
fn normalized_arena_retains_source_spans_and_borrows_after_syntax_drop() {
    let source = String::from("\t((customer_waist + (0002.500 cm))) \n");
    let tree = normalized(&source);
    assert_eq!(tree.node_count(), 3);
    assert_eq!(
        source.get(tree.root().span().start()..tree.root().span().end()),
        Some("((customer_waist + (0002.500 cm)))")
    );
    let K::Binary { left, right, .. } = tree.root().kind() else {
        assert!(matches!(tree.root().kind(), K::Binary { .. }));
        return;
    };
    assert_eq!(
        source.get(left.span().start()..left.span().end()),
        Some("customer_waist")
    );
    let K::Name(name) = left.kind() else {
        assert!(matches!(left.kind(), K::Name(_)));
        return;
    };
    assert_eq!(
        name.as_ptr(),
        source
            .as_bytes()
            .get(source.find(name).expect("name offset")..)
            .expect("source suffix")
            .as_ptr()
    );
    assert_eq!(
        source.get(right.span().start()..right.span().end()),
        Some("(0002.500 cm)")
    );
    let K::Literal(literal) = right.kind() else {
        assert!(matches!(right.kind(), K::Literal(_)));
        return;
    };
    assert_eq!(literal.span(), right.span());
    assert_eq!(literal.number(), "0002.500");
    assert_eq!(
        literal.number().as_ptr(),
        source
            .as_bytes()
            .get(source.find("0002.500").expect("literal offset")..)
            .expect("source suffix")
            .as_ptr()
    );
    assert_eq!(literal.kind(), L::Length);
    assert_eq!(literal.magnitude(), 25000);
    let cloned = tree.clone();
    drop(tree);
    assert_eq!(shape(cloned.root()), "(Add customer_waist length:25000)");
}

#[test]
fn every_literal_position_refuses_atomically_with_original_span() {
    let wide = "340282366920938463463374607431768211456";
    for source in [
        format!("probe(1, {wide}, 2)"),
        format!("if({wide}, 1, 2)"),
        format!("if(a, {wide}, 2)"),
        format!("if(a, 1, {wide})"),
        format!("if(1 == 1, 1 cm, probe({wide}))"),
    ] {
        #[allow(clippy::expect_used)]
        let syntax = E::parse(&source).expect("valid syntax, bad literal");
        let before = (
            syntax.node_count(),
            syntax.conditional_depth(),
            syntax.root().span(),
        );
        #[allow(clippy::expect_used)]
        let error = syntax
            .normalize_literals()
            .expect_err("assertion: every literal must be checked");
        assert!(matches!(
            error.rule(),
            R::RationalWidth {
                measured_bits_at_least: 129,
                ..
            }
        ));
        assert_eq!(
            source.get(error.span().start()..error.span().end()),
            Some(wide)
        );
        assert_eq!(
            (
                syntax.node_count(),
                syntax.conditional_depth(),
                syntax.root().span()
            ),
            before
        );
        assert_eq!(syntax.normalize_literals().err(), Some(error));
    }
}

#[test]
fn normalization_preserves_unevaluated_unknown_dimensional_and_zero_division_syntax() {
    for (source, want) in [
        (
            "unpublished_function(1 / 0)",
            "(unpublished_function (Divide count:1 count:0))",
        ),
        ("1 cm + 2 deg", "(Add length:10000 angle:2000000)"),
        ("-4", "(neg count:4)"),
        (
            "if(1 == 1, 2 cm, 1 / 0)",
            "(if (Equal count:1 count:1) length:20000 (Divide count:1 count:0))",
        ),
        (
            "340282366920938463463374607431768211455 + 1",
            "(Add count:340282366920938463463374607431768211455 count:1)",
        ),
    ] {
        assert_eq!(shape(normalized(source).root()), want, "{source}");
    }
}

#[test]
#[allow(clippy::expect_used)]
fn argument_views_are_ordered_exact_size_fused_and_arena_bound() {
    let source = String::from("f(1 cm, 2 mm, 3 um)");
    let tree = normalized(&source);
    let K::Call {
        name,
        mut arguments,
    } = tree.root().kind()
    else {
        assert!(matches!(tree.root().kind(), K::Call { .. }));
        return;
    };
    assert_eq!(name.as_ptr(), source.as_ptr());
    assert_eq!(arguments.len(), 3);
    assert_eq!(arguments.size_hint(), (3, Some(3)));
    let cloned = arguments.clone();
    assert_eq!(cloned.len(), 3);
    for (remaining, want) in [(2, "length:10000"), (1, "length:2000"), (0, "length:3")] {
        assert_eq!(
            shape(arguments.next().expect("assertion: argument exists")),
            want
        );
        assert_eq!(arguments.len(), remaining);
        assert_eq!(arguments.size_hint(), (remaining, Some(remaining)));
    }
    assert!(arguments.next().is_none());
    assert!(arguments.next().is_none());
    assert_eq!(
        cloned.map(shape).collect::<Vec<_>>(),
        ["length:10000", "length:2000", "length:3"]
    );
}

#[test]
#[allow(clippy::expect_used)]
fn all_independent_literal_rows_also_reach_nested_arena_conversion() {
    let rows = include_str!(
        "../../../docs/tasks/artifacts/formula_structure/literal_normalization_cases.tsv"
    );
    let mut count = 0;
    for row in rows
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
    {
        let cells: Vec<_> = row.split('\t').collect();
        let source = format!("probe({})", cells.first().expect("literal"));
        let syntax = E::parse(&source).expect("syntax");
        let result = syntax.normalize_literals();
        let want = *cells.get(2).expect("verdict");
        if want == "width" || want == "length" {
            assert!(
                result.is_err(),
                "assertion: nested input must refuse {source}"
            );
            let error = result.expect_err("asserted refusal");
            assert_eq!(
                source.get(error.span().start()..error.span().end()),
                cells.first().copied()
            );
        } else {
            assert!(
                result.is_ok(),
                "assertion: nested input must accept {source}: {result:?}"
            );
            assert_eq!(
                shape(result.expect("asserted acceptance").root()),
                format!("(probe {}:{want})", cells.get(1).expect("kind"))
            );
        }
        count += 1;
    }
    assert_eq!(count, 100);
}

#[test]
#[allow(clippy::expect_used)]
fn bounded_conversion_clone_and_drop_are_safe_on_a_small_stack() {
    std::thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(|| {
            let source = format!("{}2.5 cm{}", "(".repeat(50000), ")".repeat(50000));
            let tree = normalized(&source);
            assert_eq!(tree.node_count(), 1);
            drop(tree.clone());
            drop(tree);
            let source = format!("{}1 cm", "-".repeat(255));
            let tree = normalized(&source);
            assert_eq!(tree.node_count(), 256);
            drop(tree.clone());
            drop(tree);
            let source = format!("f({})", vec!["1 cm"; 255].join(","));
            let tree = normalized(&source);
            assert_eq!(tree.node_count(), 256);
            drop(tree);
            let mut source = "1 cm".to_owned();
            for _ in 0..16 {
                source = format!("if(a, 2 cm, abs({source}))");
            }
            let tree = normalized(&source);
            assert_eq!(tree.conditional_depth(), 16);
            drop(tree.clone());
            drop(tree);
            for source in [
                format!("{}1 cm", "-".repeat(256)),
                format!("if(a, 1 cm, {source})"),
            ] {
                assert!(matches!(
                    E::parse(&source)
                        .expect_err("assertion: syntax caps still enforced")
                        .rule(),
                    FormulaParseRule::StructuralLimit { .. }
                ));
            }
        })
        .expect("small-stack thread")
        .join()
        .expect("assertion: flat conversion/clone/drop cannot overflow stack");
}

#[test]
#[allow(clippy::expect_used)]
fn every_worked_expression_normalizes_and_debug_omits_customer_content() {
    let examples = include_str!("../../../docs/book/src/spec/formula-language/examples.md");
    let (mut section, mut count) = ("", 0);
    for line in examples.lines() {
        if line.starts_with("## ") {
            section = line;
        }
        if !line.starts_with("| `") {
            continue;
        }
        let cells: Vec<_> = line.split('|').map(str::trim).collect();
        let inputs: Vec<_> = if section.starts_with("## 2.") {
            vec![cells.get(3).expect("expression").trim_matches('`')]
        } else if section.starts_with("## 3.") {
            let statement = cells.get(1).expect("assertion").trim_matches('`');
            let expression = statement.split_once(" = ").expect("assignment").1;
            let (left, right) = expression.split_once(" == ").expect("separator");
            vec![left, right]
        } else {
            vec![]
        };
        for source in inputs {
            let syntax = E::parse(source).expect("book syntax");
            let tree = normalized(source);
            assert_eq!(tree.node_count(), syntax.node_count());
            assert_eq!(tree.root().span(), syntax.root().span());
            count += 1;
        }
    }
    assert_eq!(count, 25);
    let tree = normalized("customer_secret(123.45 cm, private_name)");
    for debug in [format!("{tree:?}"), format!("{:?}", tree.root())] {
        assert!(!debug.contains("customer_secret"));
        assert!(!debug.contains("private_name"));
        assert!(!debug.contains("123.45"));
    }
    let K::Call { arguments, .. } = tree.root().kind() else {
        assert!(matches!(tree.root().kind(), K::Call { .. }));
        return;
    };
    assert!(!format!("{arguments:?}").contains("private_name"));
}
