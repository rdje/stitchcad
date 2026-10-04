//! Independent normative rows exercised through real bounded syntax and sourced initial metadata.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]
use sc_core::{
    name::MachineToken,
    ontology::{EdgeRef, EntityId, LocalTag, PointRef},
    recipe::{
        FormulaBinaryOperator as B, FormulaBuiltin as F, FormulaBuiltinOperand as O,
        FormulaCheckedOperation as Op, FormulaDeclaration as D, FormulaDeclarationSource as S,
        FormulaExpression, FormulaExpressionCheckRefusal as R, FormulaInitialDeclaration,
        FormulaInputOrigin, FormulaKind as K, FormulaNamespace, FormulaNormalizedExpression,
        FormulaOrigin, FormulaScalarInputOrigin, FormulaToleranceName as T,
        FormulaUnaryOperator as U,
    },
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState},
};

const KINDS: [(K, &str); 8] = [
    (K::Length, "length_input"),
    (K::Angle, "angle_input"),
    (K::Area, "area_input"),
    (K::Ratio, "ratio_input"),
    (K::Count, "count_input"),
    (K::Boolean, "boolean_input"),
    (K::Point, "point_input"),
    (K::Edge, "edge_input"),
];
const ARITHMETIC: [K; 5] = [K::Length, K::Angle, K::Area, K::Ratio, K::Count];
const CLASSES: [(T, &str); 5] = [
    (T::Numerical, "eps_num"),
    (T::Geometric, "eps_geo"),
    (T::Format, "eps_fmt"),
    (T::Importer, "eps_imp"),
    (T::Physical, "eps_phys"),
];
// Authored grammar5.1 rows, independent of product signature matching.
const PAIRS: [(K, K, Option<K>, K); 14] = [
    (K::Length, K::Length, Some(K::Area), K::Ratio),
    (K::Length, K::Ratio, Some(K::Length), K::Length),
    (K::Length, K::Count, Some(K::Length), K::Length),
    (K::Angle, K::Angle, None, K::Ratio),
    (K::Angle, K::Ratio, Some(K::Angle), K::Angle),
    (K::Angle, K::Count, Some(K::Angle), K::Angle),
    (K::Area, K::Length, None, K::Length),
    (K::Area, K::Ratio, Some(K::Area), K::Area),
    (K::Area, K::Count, Some(K::Area), K::Area),
    (K::Area, K::Area, None, K::Ratio),
    (K::Ratio, K::Ratio, Some(K::Ratio), K::Ratio),
    (K::Ratio, K::Count, Some(K::Ratio), K::Ratio),
    (K::Count, K::Count, Some(K::Count), K::Ratio),
    (K::Count, K::Ratio, Some(K::Ratio), K::Count),
];
const OPERATORS: [(B, &str); 10] = [
    (B::Add, "+"),
    (B::Subtract, "-"),
    (B::Multiply, "*"),
    (B::Divide, "/"),
    (B::Equal, "=="),
    (B::NotEqual, "!="),
    (B::Less, "<"),
    (B::LessEqual, "<="),
    (B::Greater, ">"),
    (B::GreaterEqual, ">="),
];
// Authored grammar6/6.1 rows; T consistency is resolved by this independent textual oracle.
const ROWS: [(&str, &str, &str); 24] = [
    ("sqrt", "area", "length"),
    ("sqrt", "ratio", "ratio"),
    ("hypot", "length, length", "length"),
    ("abs", "T", "T"),
    ("min", "T, …", "T"),
    ("max", "T, …", "T"),
    ("clamp", "T, T, T", "T"),
    ("round_to", "T, T", "T"),
    ("sin", "angle", "ratio"),
    ("cos", "angle", "ratio"),
    ("tan", "angle", "ratio"),
    ("atan", "ratio", "angle"),
    ("atan2", "length, length", "angle"),
    ("atan2", "ratio, ratio", "angle"),
    ("arc_length", "angle, length", "length"),
    ("if", "boolean, T, T", "T"),
    ("within", "T, T, tolerance", "boolean"),
    ("x", "point", "length"),
    ("y", "point", "length"),
    ("dist", "point, point", "length"),
    ("dir", "point, point", "angle"),
    ("len", "edge", "length"),
    ("param_at", "edge, length", "ratio"),
    ("point_at", "edge, ratio", "point"),
];
fn names() -> [MachineToken; 8] {
    KINDS.map(|(_, name)| MachineToken::new(name).unwrap())
}
fn initial_namespace(names: &[MachineToken; 8]) -> FormulaNamespace<'_> {
    FormulaNamespace::new(
        names
            .iter()
            .zip(KINDS)
            .map(|(name, (kind, _))| match kind {
                K::Point => D::point(
                    name,
                    PointRef::new(EntityId::from_bits(21), LocalTag::FIRST),
                ),
                K::Edge => D::edge(
                    name,
                    EdgeRef::new(EntityId::from_bits(22), LocalTag::new(9)),
                ),
                _ => D::input(
                    name,
                    FormulaScalarInputOrigin::Parameter,
                    EntityId::from_bits(31),
                    EntityId::from_bits(32),
                    kind.binding_kind().unwrap(),
                ),
            })
            .map(|d| FormulaInitialDeclaration::try_from(d).unwrap()),
    )
    .unwrap()
}
fn normalized(source: &str) -> FormulaNormalizedExpression<'_> {
    FormulaExpression::parse(source)
        .unwrap()
        .normalize_literals()
        .unwrap()
}
fn kind(token: &str) -> K {
    [
        K::Length,
        K::Angle,
        K::Area,
        K::Ratio,
        K::Count,
        K::Boolean,
        K::Point,
        K::Edge,
    ]
    .into_iter()
    .zip([
        "length", "angle", "area", "ratio", "count", "boolean", "point", "edge",
    ])
    .find(|(_, text)| *text == token)
    .unwrap()
    .0
}
fn ordinary(operand: O) -> K {
    match operand {
        O::Value(k) => k,
        O::Tolerance(_) => K::Length,
    }
}
fn expected_call(name: &str, operands: &[O]) -> Option<K> {
    for (_, arguments, result) in ROWS.iter().filter(|(token, _, _)| *token == name) {
        let variadic = arguments.ends_with(", …");
        let wants: Vec<_> = arguments.split(", ").filter(|v| *v != "…").collect();
        if operands.len() < wants.len() || (!variadic && operands.len() != wants.len()) {
            continue;
        }
        let mut bound = None;
        let matched = operands.iter().enumerate().all(|(i, operand)| {
            match if variadic { "T" } else { wants[i] } {
                "tolerance" => {
                    matches!(operand,O::Tolerance(t) if CLASSES.iter().any(|(c,_)| c==t))
                }
                "T" => {
                    let k = ordinary(*operand);
                    if !ARITHMETIC.contains(&k) || bound.is_some_and(|prior| prior != k) {
                        return false;
                    }
                    bound = Some(k);
                    true
                }
                fixed => ordinary(*operand) == kind(fixed),
            }
        });
        if matched {
            return if *result == "T" {
                bound
            } else {
                Some(kind(result))
            };
        }
    }
    None
}
fn compare(
    source: &str,
    namespace: &FormulaNamespace<'_>,
    operation: Op,
    operands: &[O],
    expected: Option<K>,
) {
    let expression = normalized(source);
    let result = expression.check_kinds(namespace);
    assert_eq!(result.is_ok(), expected.is_some(), "{source}");
    match result {
        Ok(proof) => {
            assert_eq!(Some(proof.kind()), expected, "{source}");
            assert!(std::ptr::eq(proof.expression(), &expression));
            assert_eq!(proof.canonical_expression(), expression.canonical_form());
            assert_eq!(proof.dependencies().len(), operands.len());
        }
        Err(error) => {
            assert_eq!(error.token(), "formula_dimension");
            assert_eq!(error.span(), expression.root().span());
            assert!(std::ptr::eq(error.expression(), &expression));
            assert_eq!(error.canonical_expression(), expression.canonical_form());
            let R::Dimension(payload) = error.refusal() else {
                assert!(matches!(error.refusal(), R::Dimension(_)));
                return;
            };
            assert_eq!(payload.operation(), operation);
            assert_eq!(payload.operands(), operands, "{source}");
            assert_eq!(payload.wanted_signatures(), operation.signatures());
            let hint = matches!(operation, Op::Binary(B::Multiply))
                && matches!(operands,[a,b] if [(K::Angle,K::Length),(K::Length,K::Angle)].contains(&(ordinary(*a),ordinary(*b))));
            assert_eq!(payload.arc_length_hint(), hint, "{source}");
        }
    }
}
#[test]
fn every_operator_kind_tuple_reaches_the_independent_normative_rows() {
    let names = names();
    let namespace = initial_namespace(&names);
    let mut cases = 0;
    for (k, name) in KINDS {
        for (operation, source, expected) in [
            (
                U::Negate,
                format!("-{name}"),
                [K::Length, K::Angle, K::Area, K::Ratio]
                    .contains(&k)
                    .then_some(k),
            ),
            (
                U::Square,
                format!("{name} ^ 2"),
                [
                    (K::Length, K::Area),
                    (K::Ratio, K::Ratio),
                    (K::Count, K::Count),
                ]
                .into_iter()
                .find(|(a, _)| *a == k)
                .map(|(_, b)| b),
            ),
        ] {
            compare(
                &source,
                &namespace,
                Op::Unary(operation),
                &[O::Value(k)],
                expected,
            );
            cases += 1;
        }
    }
    for (operation, token) in OPERATORS {
        for (left, lname) in KINDS {
            for (right, rname) in KINDS {
                let same = left == right && ARITHMETIC.contains(&left);
                let expected = match operation {
                    B::Add | B::Subtract => same.then_some(left),
                    B::Multiply => PAIRS
                        .iter()
                        .find(|(l, r, _, _)| {
                            (*l == left && *r == right) || (*l == right && *r == left)
                        })
                        .and_then(|(_, _, result, _)| *result),
                    B::Divide => PAIRS
                        .iter()
                        .find(|(l, r, _, _)| *l == left && *r == right)
                        .map(|(_, _, _, result)| *result),
                    _ => same.then_some(K::Boolean),
                };
                compare(
                    &format!("{lname} {token} {rname}"),
                    &namespace,
                    Op::Binary(operation),
                    &[O::Value(left), O::Value(right)],
                    expected,
                );
                cases += 1;
            }
        }
    }
    assert_eq!(cases, 656);
}
#[test]
fn every_known_call_kind_and_direct_class_tuple_checks_real_syntax_before_signature() {
    use std::collections::BTreeSet;
    assert_eq!(
        F::ALL.into_iter().map(F::token).collect::<BTreeSet<_>>(),
        ROWS.iter()
            .map(|(name, _, _)| *name)
            .collect::<BTreeSet<_>>()
    );
    let names = names();
    let namespace = initial_namespace(&names);
    let population: Vec<_> = KINDS
        .into_iter()
        .map(|(k, n)| (O::Value(k), n))
        .chain(CLASSES.into_iter().map(|(t, n)| (O::Tolerance(t), n)))
        .collect();
    let mut cases = 0;
    for builtin in F::ALL {
        let name = builtin.token();
        assert!(ROWS.iter().any(|(n, _, _)| *n == name));
        for arity in 1..=3 {
            if builtin == F::If && arity != 3 {
                continue;
            }
            for mut code in 0..13usize.pow(arity) {
                let mut operands = Vec::new();
                let mut spellings = Vec::new();
                for _ in 0..arity {
                    let (operand, spelling) = population[code % 13];
                    code /= 13;
                    operands.push(operand);
                    spellings.push(spelling);
                }
                let source = format!("{name}({})", spellings.join(","));
                let expected = expected_call(name, &operands);
                compare(
                    &source,
                    &namespace,
                    Op::Builtin(builtin),
                    &operands,
                    expected,
                );
                cases += 1;
            }
        }
    }
    assert_eq!(cases, 52156);
    for builtin in F::ALL.into_iter().filter(|builtin| *builtin != F::If) {
        let operands = [O::Value(K::Length); 4];
        let source = format!("{}({})", builtin.token(), ["length_input"; 4].join(","));
        compare(
            &source,
            &namespace,
            Op::Builtin(builtin),
            &operands,
            expected_call(builtin.token(), &operands),
        );
    }
    for source in [
        "min()",
        "max()",
        "if(is_base_size,1 mm)",
        "if(is_base_size,1 mm,2 mm,3 mm)",
    ] {
        assert!(FormulaExpression::parse(source).is_err());
    }
}
#[test]
fn child_order_and_callee_priority_select_real_inner_context_without_partial_proof() {
    let namespace = FormulaNamespace::new([]).unwrap();
    for (source, token, name, node) in [
        (
            "hypot(missing_left,missing_right)",
            "formula_unbound_name",
            "missing_left",
            "missing_left",
        ),
        (
            "if(1,missing_then,missing_else)",
            "formula_unbound_name",
            "missing_then",
            "missing_then",
        ),
        (
            "if(missing_condition,missing_then,missing_else)",
            "formula_unbound_name",
            "missing_condition",
            "missing_condition",
        ),
        (
            "within(1 mm,missing_value,missing_class)",
            "formula_unbound_name",
            "missing_value",
            "missing_value",
        ),
        (
            "if(is_base_size,1 mm,missing_else)",
            "formula_unbound_name",
            "missing_else",
            "missing_else",
        ),
        (
            "if(is_base_size,missing_then,1 mm)",
            "formula_unbound_name",
            "missing_then",
            "missing_then",
        ),
        (
            "hypot(1 deg,missing_later)",
            "formula_unbound_name",
            "missing_later",
            "missing_later",
        ),
        (
            "missing_call(missing_argument)",
            "formula_unbound_name",
            "missing_call",
            "missing_call(missing_argument)",
        ),
        (
            "spline(missing_argument)",
            "env_nurbs",
            "spline",
            "spline(missing_argument)",
        ),
        (
            "constraint(missing_argument)",
            "env_sketch_constraints",
            "constraint",
            "constraint(missing_argument)",
        ),
        (
            "if(1,1 mm,spline(missing_argument))",
            "env_nurbs",
            "spline",
            "spline(missing_argument)",
        ),
        (
            "hypot(1 mm, (missing_later))",
            "formula_unbound_name",
            "missing_later",
            "(missing_later)",
        ),
    ] {
        let expression = normalized(source);
        let result = expression.check_kinds(&namespace);
        assert!(result.is_err(), "{source}");
        let error = result.unwrap_err();
        assert_eq!(error.token(), token);
        assert_eq!(error.to_string(), token);
        assert_eq!(&source[error.span().start()..error.span().end()], node);
        assert_eq!(error.canonical_expression(), expression.canonical_form());
        assert!(std::ptr::eq(error.expression(), &expression));
        match error.refusal() {
            R::UnboundName(payload) => {
                assert_eq!(payload.name(), name);
                assert_eq!(payload.origins_searched(), &FormulaOrigin::ALL);
            }
            R::Call(payload) => {
                assert_eq!(payload.name(), name);
                assert_eq!(payload.token(), token);
                assert_eq!(payload.lookup_scope(), "formula_call");
            }
            R::Dimension(_) => assert!(
                !matches!(error.refusal(), R::Dimension(_)),
                "child/callee error must precede incomplete tuple"
            ),
        }
        // The allowed stable token may contain an alias (constraint / env_sketch_constraints).
        assert!(!format!("{error:?}").replace(token, "").contains(name));
        assert!(!format!("{:?}", error.refusal()).contains(&format!("{name:?}")));
    }
    let expression = normalized("if(1,1 mm,2 deg)");
    let error = expression.check_kinds(&namespace).unwrap_err();
    let R::Dimension(payload) = error.refusal() else {
        assert!(matches!(error.refusal(), R::Dimension(_)));
        return;
    };
    assert_eq!(
        payload.operands(),
        [O::Value(K::Count), O::Value(K::Length), O::Value(K::Angle)]
    );
    assert_eq!(payload.operation(), Op::Builtin(F::If));
    let expression = normalized("1 deg ^ 2 + missing_later");
    let error = expression.check_kinds(&namespace).unwrap_err();
    assert_eq!(error.token(), "formula_dimension");
    assert_eq!(
        &"1 deg ^ 2 + missing_later"[error.span().start()..error.span().end()],
        "1 deg ^ 2"
    );
}
#[test]
fn grouping_preserves_only_direct_symbolic_classes_and_every_resolved_operand() {
    let namespace = FormulaNamespace::new([]).unwrap();
    for (_, class) in CLASSES {
        let source = format!("within(1 mm,2 mm,(({class})))");
        let expression = normalized(&source);
        let result = expression.check_kinds(&namespace);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().kind(), K::Boolean);
        for computed in [
            format!("{class} + 0 mm"),
            format!("abs({class})"),
            format!("-{class}"),
            format!("if(is_base_size,{class},{class})"),
        ] {
            let source = format!("within(1 mm,2 mm,{computed})");
            let expression = normalized(&source);
            let result = expression.check_kinds(&namespace);
            assert!(result.is_err());
            let error = result.unwrap_err();
            let R::Dimension(payload) = error.refusal() else {
                assert!(matches!(error.refusal(), R::Dimension(_)));
                return;
            };
            assert_eq!(payload.operation(), Op::Builtin(F::Within));
            assert_eq!(payload.operands(), [O::Value(K::Length); 3]);
            assert_eq!(payload.wanted_signatures(), F::Within.signatures());
            assert!(!payload.arc_length_hint());
        }
    }
}
#[test]
fn dependencies_keep_repeated_and_untaken_uses_exact_sources_and_canonical_borrows() {
    let name = MachineToken::new("private_measurement").unwrap();
    let record = LengthDeclaration::new(LengthDeclarationDefinition {
        id: EntityId::from_bits(1),
        source: EntityId::from_bits(2),
        state: LengthState::Unknown {
            observation: EntityId::from_bits(3),
        },
    })
    .unwrap();
    let source = "if(is_base_size,(private_measurement),private_measurement + eps_phys)";
    let expression = normalized(source);
    let proof = {
        let namespace = FormulaNamespace::new([D::length_input(
            &name,
            FormulaInputOrigin::Measurement,
            EntityId::from_bits(4),
            &record,
        )
        .try_into()
        .unwrap()])
        .unwrap();
        expression.check_kinds(&namespace).unwrap()
    };
    assert_eq!(proof.kind(), K::Length);
    assert_eq!(
        proof
            .dependencies()
            .iter()
            .map(|d| d.declaration().name())
            .collect::<Vec<_>>(),
        [
            "is_base_size",
            "private_measurement",
            "private_measurement",
            "eps_phys"
        ]
    );
    assert_eq!(
        proof
            .dependencies()
            .iter()
            .map(|d| &source[d.span().start()..d.span().end()])
            .collect::<Vec<_>>(),
        [
            "is_base_size",
            "(private_measurement)",
            "private_measurement",
            "eps_phys"
        ]
    );
    for dependency in &proof.dependencies()[1..=2] {
        let S::LengthInput {
            input, declaration, ..
        } = dependency.declaration().source()
        else {
            assert!(matches!(
                dependency.declaration().source(),
                S::LengthInput { .. }
            ));
            return;
        };
        assert_eq!(input, EntityId::from_bits(4));
        assert!(std::ptr::eq(declaration, &record));
        assert_eq!(
            dependency.declaration().origin(),
            FormulaOrigin::Measurement
        );
        assert!(!format!("{dependency:?}").contains("private_measurement"));
    }
    assert!(std::ptr::eq(proof.expression(), &expression));
    assert!(!format!("{proof:?}").contains("private_measurement"));
    assert_eq!(
        proof.canonical_expression().as_str(),
        "(if is_base_size private_measurement (+ private_measurement eps_phys))"
    );
}
#[test]
fn literal_and_runtime_domain_cases_are_checked_without_computing_a_verdict_or_value() {
    let namespace = FormulaNamespace::new([]).unwrap();
    for (source, expected) in [
        ("7", K::Count),
        ("7.0", K::Ratio),
        ("7 pct", K::Ratio),
        ("7 cm", K::Length),
        ("720 deg", K::Angle),
        ("1 mm / 0", K::Length),
        ("sqrt(-1.0)", K::Ratio),
        ("tan(90 deg)", K::Ratio),
        ("clamp(1 mm,3 mm,2 mm)", K::Length),
        ("within(1 mm,2 mm,eps_phys)", K::Boolean),
        ("if(is_base_size,1 mm,1 mm / 0)", K::Length),
        ("size_index + size_count", K::Count),
    ] {
        let expression = normalized(source);
        let result = expression.check_kinds(&namespace);
        assert!(result.is_ok(), "{source}");
        assert_eq!(result.unwrap().kind(), expected);
    }
    let names = names();
    let namespace = initial_namespace(&names);
    for (source, expected) in [
        ("point_at(edge_input,ratio_input)", K::Point),
        ("x(point_input)", K::Length),
        ("len(edge_input)", K::Length),
    ] {
        let expression = normalized(source);
        let proof = expression.check_kinds(&namespace).unwrap();
        assert_eq!(proof.kind(), expected);
        for d in proof.dependencies() {
            if d.declaration().kind() == K::Point {
                assert!(
                    matches!(d.declaration().source(),S::Point(p) if p==PointRef::new(EntityId::from_bits(21),LocalTag::FIRST))
                );
            }
            if d.declaration().kind() == K::Edge {
                assert!(
                    matches!(d.declaration().source(),S::Edge(e) if e==EdgeRef::new(EntityId::from_bits(22),LocalTag::new(9)))
                );
            }
        }
    }
}
#[test]
fn bounded_flat_traversal_handles_deep_unary_grouping_variadics_and_conditional_limits() {
    let namespace = FormulaNamespace::new([]).unwrap();
    for source in [
        format!("{}1 mm", "-".repeat(255)),
        format!("{}1 mm{}", "(".repeat(4000), ")".repeat(4000)),
    ] {
        let expression = normalized(&source);
        let result = expression.check_kinds(&namespace);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().kind(), K::Length);
    }
    let source = format!("min({})", vec!["eps_phys"; 255].join(","));
    let expression = normalized(&source);
    assert_eq!(expression.node_count(), 256);
    let proof = expression.check_kinds(&namespace).unwrap();
    assert_eq!(proof.kind(), K::Length);
    assert_eq!(proof.dependencies().len(), 255);
    assert!(FormulaExpression::parse(&format!("{}1 mm", "-".repeat(256))).is_err());
    let mut source = "1 mm".to_owned();
    for _ in 0..16 {
        source = format!("if(is_base_size,{source},2 mm)");
    }
    let expression = normalized(&source);
    assert_eq!(expression.conditional_depth(), 16);
    let proof = expression.check_kinds(&namespace).unwrap();
    assert_eq!(proof.kind(), K::Length);
    assert_eq!(proof.dependencies().len(), 16);
    assert!(FormulaExpression::parse(&format!("if(is_base_size,{source},2 mm)")).is_err());
}
