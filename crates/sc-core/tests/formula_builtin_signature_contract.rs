//! Independently authored normative rows over every kind/class tuple at arities zero through four.
#![allow(clippy::unwrap_used, clippy::expect_used)]
use sc_core::recipe::{
    FormulaBuiltin as B, FormulaBuiltinArity as A, FormulaBuiltinCategory as C,
    FormulaBuiltinOperand as O, FormulaKind as K, FormulaToleranceName as T,
};

const KINDS: [K; 8] = [
    K::Length,
    K::Angle,
    K::Area,
    K::Ratio,
    K::Count,
    K::Boolean,
    K::Point,
    K::Edge,
];
const ARITHMETIC: [K; 5] = [K::Length, K::Angle, K::Area, K::Ratio, K::Count];
const CLASSES: [T; 5] = [
    T::Numerical,
    T::Geometric,
    T::Format,
    T::Importer,
    T::Physical,
];
const NAMES: [(B, &str, C, A); 22] = [
    (B::Sqrt, "sqrt", C::Function, A::Fixed(1)),
    (B::Hypot, "hypot", C::Function, A::Fixed(2)),
    (B::Abs, "abs", C::Function, A::Fixed(1)),
    (B::Min, "min", C::Function, A::OneOrMore),
    (B::Max, "max", C::Function, A::OneOrMore),
    (B::Clamp, "clamp", C::Function, A::Fixed(3)),
    (B::RoundTo, "round_to", C::Function, A::Fixed(2)),
    (B::Sin, "sin", C::Function, A::Fixed(1)),
    (B::Cos, "cos", C::Function, A::Fixed(1)),
    (B::Tan, "tan", C::Function, A::Fixed(1)),
    (B::Atan, "atan", C::Function, A::Fixed(1)),
    (B::Atan2, "atan2", C::Function, A::Fixed(2)),
    (B::ArcLength, "arc_length", C::Function, A::Fixed(2)),
    (B::If, "if", C::Conditional, A::Fixed(3)),
    (B::Within, "within", C::ToleranceComparison, A::Fixed(3)),
    (B::X, "x", C::Selector, A::Fixed(1)),
    (B::Y, "y", C::Selector, A::Fixed(1)),
    (B::Dist, "dist", C::Selector, A::Fixed(2)),
    (B::Dir, "dir", C::Selector, A::Fixed(2)),
    (B::Len, "len", C::Selector, A::Fixed(1)),
    (B::ParamAt, "param_at", C::Selector, A::Fixed(2)),
    (B::PointAt, "point_at", C::Selector, A::Fixed(2)),
];
// The published rows expanded by name, independently of product dispatch and arity metadata.
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

fn kind(token: &str) -> K {
    KINDS
        .into_iter()
        .zip([
            "length", "angle", "area", "ratio", "count", "boolean", "point", "edge",
        ])
        .find_map(|(kind, spelling)| (spelling == token).then_some(kind))
        .expect("authored oracle kind")
}

fn ordinary(operand: O) -> K {
    match operand {
        O::Value(k) => k,
        O::Tolerance(_) => K::Length,
    }
}
fn expected(name: &str, operands: &[O]) -> Option<K> {
    for (_, arguments, result) in ROWS.iter().filter(|(token, _, _)| *token == name) {
        let variadic = arguments.ends_with(", …");
        let wants: Vec<_> = arguments.split(", ").filter(|v| *v != "…").collect();
        if operands.len() < wants.len() || (!variadic && operands.len() != wants.len()) {
            continue;
        }
        let mut bound = None;
        let matched = operands.iter().enumerate().all(|(i, operand)| {
            let want = if variadic {
                "T"
            } else {
                wants.get(i).expect("fixed oracle arity")
            };
            match want {
                "tolerance" => matches!(operand, O::Tolerance(t) if CLASSES.contains(t)),
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

#[test]
fn closed_names_categories_arities_and_exact_lookup_match_authored_vocabulary() {
    assert_eq!(B::ALL, NAMES.map(|row| row.0));
    for (builtin, token, category, arity) in NAMES {
        assert_eq!(builtin.token(), token);
        assert_eq!(B::from_token(token), Some(builtin));
        assert_eq!(builtin.category(), category);
        assert_eq!(builtin.arity(), arity);
        for count in [0, 1, 2, 3, 4, 255, 256, usize::MAX] {
            let wanted = match arity {
                A::Fixed(n) => count == n,
                A::OneOrMore => count >= 1,
            };
            assert_eq!(builtin.arity().accepts(count), wanted, "{token}/{count}");
        }
        for invalid in [
            format!(" {token}"),
            format!("{token} "),
            token.to_uppercase(),
            format!("{token}_"),
        ] {
            assert_eq!(B::from_token(&invalid), None, "{invalid}");
        }
    }
    for unlisted in [
        "",
        "eps_geo",
        "size_index",
        "size_count",
        "is_base_size",
        "let",
        "assert",
        "unlisted_call",
        "loop",
        "repeat",
        "while",
        "fn",
        "macro",
        "nurbs",
        "spline",
        "solve",
    ] {
        assert_eq!(B::from_token(unlisted), None, "{unlisted}");
    }
}

#[test]
fn every_kind_and_class_tuple_at_zero_through_four_arguments_matches_closed_rows() {
    let population: Vec<_> = KINDS
        .into_iter()
        .map(O::Value)
        .chain(CLASSES.into_iter().map(O::Tolerance))
        .collect();
    for &operand in &population {
        assert_eq!(operand.kind(), ordinary(operand));
    }
    let mut checks = 0;
    for (builtin, token, _, _) in NAMES {
        for count in 0..=4_u32 {
            for mut index in 0..13_usize.pow(count) {
                let mut operands = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    operands.push(
                        *population
                            .get(index % 13)
                            .expect("closed operand population"),
                    );
                    index /= 13;
                }
                assert_eq!(
                    builtin.result_kind(&operands),
                    expected(token, &operands),
                    "{token}{operands:?}"
                );
                let mut results = builtin
                    .signatures()
                    .iter()
                    .filter_map(|row| row.result_kind(&operands));
                assert_eq!(
                    results.next(),
                    expected(token, &operands),
                    "wanted catalog {token}{operands:?}"
                );
                assert_eq!(
                    results.next(),
                    None,
                    "overlapping wanted rows {token}{operands:?}"
                );
                checks += 1;
            }
        }
    }
    assert_eq!(checks, 680702);
}

#[test]
fn wanted_catalog_descriptors_are_the_exact_normative_rows_and_result_positions() {
    use sc_core::recipe::{FormulaOperandRequirement as R, FormulaResultRequirement as V};
    let mut actual = Vec::new();
    for (builtin, token, _, arity) in NAMES {
        for row in builtin.signatures() {
            assert_eq!(row.arity(), arity);
            let mut arguments = row
                .operand_requirements()
                .iter()
                .map(|requirement| match requirement {
                    R::Exact(k) => k.token(),
                    R::Arithmetic => "T",
                    R::Negatable => "N",
                    R::ToleranceName => "tolerance",
                })
                .collect::<Vec<_>>()
                .join(", ");
            if arity == A::OneOrMore {
                arguments.push_str(", …");
            }
            let result = match row.result_requirement() {
                V::Exact(k) => k.token(),
                V::Operand(index) => {
                    assert_eq!(index, usize::from(builtin == B::If));
                    assert_eq!(row.operand_requirements().get(index), Some(&R::Arithmetic));
                    "T"
                }
            };
            actual.push((token, arguments, result));
        }
    }
    let mut wanted = ROWS
        .map(|(name, args, result)| (name, args.to_owned(), result))
        .to_vec();
    actual.sort_unstable();
    wanted.sort_unstable();
    assert_eq!(actual, wanted);
}

#[test]
fn variadic_signatures_remain_metadata_and_symbolic_classes_are_not_computed_lengths() {
    for builtin in [B::Min, B::Max] {
        for k in ARITHMETIC {
            for count in [1, 2, 3, 255, 256, 4096] {
                let operands = vec![O::Value(k); count];
                assert_eq!(builtin.result_kind(&operands), Some(k));
                assert_eq!(
                    builtin.signatures().first().unwrap().result_kind(&operands),
                    Some(k)
                );
            }
        }
    }
    for class in CLASSES {
        let symbol = O::Tolerance(class);
        assert_eq!(
            B::Within.result_kind(&[O::Value(K::Length), symbol, symbol]),
            Some(K::Boolean)
        );
        assert_eq!(B::Abs.result_kind(&[symbol]), Some(K::Length));
        // Signature metadata cannot turn an ordinary/computed length into a class name.
        assert_eq!(
            B::Within.result_kind(&[symbol, symbol, O::Value(K::Length)]),
            None
        );
    }
    assert_eq!(
        B::If.result_kind(&[O::Value(K::Boolean), O::Value(K::Point), O::Value(K::Point)]),
        None
    );
}

#[test]
fn actual_normative_rows_match_both_directions_and_canonical_names_are_unchanged() {
    let chapter = include_str!("../../../docs/book/src/spec/formula-language/grammar.md");
    let functions = chapter
        .split("## 6. Built-in functions\n")
        .nth(1)
        .unwrap()
        .split("## 7. Conditionals\n")
        .next()
        .unwrap();
    let mut actual = Vec::new();
    for row in functions.lines().filter(|line| line.starts_with("| `")) {
        let cells: Vec<_> = row.split('|').map(str::trim).collect();
        for token in cells
            .get(1)
            .expect("name cell")
            .split('`')
            .skip(1)
            .step_by(2)
        {
            actual.push((
                token,
                *cells.get(2).expect("argument cell"),
                *cells.get(3).expect("result cell"),
            ));
        }
    }
    let mut wanted = ROWS.to_vec();
    actual.sort_unstable();
    wanted.sort_unstable();
    assert_eq!(actual, wanted);
    use sc_core::recipe::FormulaExpression;
    for (_, token, _, arity) in NAMES {
        let count = match arity {
            A::Fixed(n) => n,
            A::OneOrMore => 1,
        };
        let args = vec!["1.0"; count].join(", ");
        let source = format!("{token}({args})");
        let canonical = FormulaExpression::parse(&source)
            .unwrap()
            .normalize_literals()
            .unwrap()
            .canonical_form();
        assert_eq!(
            canonical.as_str(),
            format!("({token}{})", " ratio:1000000".repeat(count))
        );
    }
}
