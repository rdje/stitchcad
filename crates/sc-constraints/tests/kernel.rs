//! The kernel's contract, one behaviour per test:outcomes, ordering, unknown facts, refusals, sentences.
#![allow(
    clippy::panic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing
)]
use sc_constraints::{
    Binding, Catalog, Comparison, CoreItem, Domain, Expr, Limits, Linear, Origin, Outcome, Problem,
    ProblemError, Rational, Rule, TemplatePart, Value, Variable, AST_VERSION,
};

fn symbol(token: &str) -> Value {
    Value::Symbol(token.to_owned())
}

fn text(sentence: &str) -> Vec<TemplatePart> {
    vec![TemplatePart::Text(sentence.to_owned())]
}

fn units(binding: Binding) -> Variable {
    Variable::new(
        "units",
        "Pattern units",
        Domain::symbols(&[("mm", "millimeters"), ("cm", "centimeters")]),
        binding,
    )
}

fn system(binding: Binding) -> Variable {
    Variable::new(
        "system",
        "Factory system",
        Domain::symbols(&[
            ("modaris13", "Lectra Modaris 13.x"),
            ("other", "another system"),
        ]),
        binding,
    )
}

/// Roadmap §6.2's example rule.
fn modaris_needs_cm() -> Rule {
    Rule::hard(
        "modaris-units",
        vec![
            TemplatePart::Text("Your factory uses ".to_owned()),
            TemplatePart::ValueOf("system".to_owned()),
            TemplatePart::Text(" — this profile requires centimeters".to_owned()),
        ],
        Expr::Implies(
            Box::new(Expr::Is("system".to_owned(), symbol("modaris13"))),
            Box::new(Expr::Is("units".to_owned(), symbol("cm"))),
        ),
    )
}

fn solve(variables: Vec<Variable>, rules: Vec<Rule>) -> (Problem, Outcome) {
    let problem = Problem::new(AST_VERSION, variables, rules).unwrap();
    let outcome = problem.solve(Limits::default());
    (problem, outcome)
}

fn satisfied(outcome: &Outcome) -> &sc_constraints::Solution {
    match outcome {
        Outcome::Satisfied(solution) => solution,
        other => panic!("expected a solution, got {other:?}"),
    }
}

#[test]
fn a_choice_takes_the_first_declared_value_the_rules_allow() {
    let (_, outcome) = solve(
        vec![
            system(Binding::Known(symbol("modaris13"))),
            units(Binding::Choice),
        ],
        vec![modaris_needs_cm()],
    );
    let solution = satisfied(&outcome);
    let chosen = solution
        .selections
        .iter()
        .find(|selection| selection.name == "units")
        .unwrap();
    assert_eq!(chosen.value, symbol("cm"));
    assert_eq!(chosen.origin, Origin::Selected);

    let (_, outcome) = solve(
        vec![
            system(Binding::Known(symbol("other"))),
            units(Binding::Choice),
        ],
        vec![modaris_needs_cm()],
    );
    let chosen = satisfied(&outcome)
        .selections
        .iter()
        .find(|selection| selection.name == "units")
        .unwrap()
        .clone();
    assert_eq!(
        chosen.value,
        symbol("mm"),
        "with no rule in the way the first declared value stands"
    );
}

#[test]
fn a_conflict_is_proven_with_a_minimal_field_linked_core_in_sentences() {
    let unrelated = Variable::new(
        "notch",
        "Notch code",
        Domain::symbols(&[("v", "V notch"), ("t", "T notch")]),
        Binding::Choice,
    );
    let (problem, outcome) = solve(
        vec![
            system(Binding::Known(symbol("modaris13"))),
            units(Binding::Known(symbol("mm"))),
            unrelated,
        ],
        vec![
            Rule::hard(
                "notch-any",
                text("A notch code is set"),
                Expr::OneOf("notch".to_owned(), vec![symbol("v"), symbol("t")]),
            ),
            modaris_needs_cm(),
        ],
    );
    let Outcome::ProvenUnsatisfiable(unsat) = &outcome else {
        panic!("{outcome:?}")
    };
    assert!(unsat.minimal);
    assert_eq!(
        unsat.core.len(),
        3,
        "the unrelated rule stays out: {:?}",
        unsat.core
    );
    assert!(
        matches!(&unsat.core[0], CoreItem::Fact { field, assumed: false, .. } if field == "system")
    );
    assert!(matches!(&unsat.core[1], CoreItem::Fact { field, .. } if field == "units"));
    assert!(
        matches!(&unsat.core[2], CoreItem::Rule { id, fields } if id == "modaris-units" && fields == &["system", "units"])
    );
    assert_eq!(
        problem.explain(&outcome, &Catalog::english()),
        [
            "Factory system is Lectra Modaris 13.x.",
            "Pattern units is millimeters.",
            "Your factory uses Lectra Modaris 13.x — this profile requires centimeters",
        ]
    );
}

#[test]
fn an_unknown_fact_is_never_assigned_and_its_fitting_answers_are_listed() {
    let (problem, outcome) = solve(
        vec![
            system(Binding::Known(symbol("modaris13"))),
            units(Binding::Unknown),
        ],
        vec![modaris_needs_cm()],
    );
    let solution = satisfied(&outcome);
    assert!(
        solution
            .selections
            .iter()
            .all(|selection| selection.name != "units"),
        "{solution:?}"
    );
    assert_eq!(solution.open_facts[0].feasible, [symbol("cm")]);
    assert!(solution.open_facts[0].complete);
    assert_eq!(
        problem.explain(&outcome, &Catalog::english()),
        ["Pattern units is not known. Answers that fit: centimeters."]
    );
}

#[test]
fn an_assumption_is_reported_as_an_assumption() {
    let (problem, outcome) = solve(
        vec![
            system(Binding::Known(symbol("modaris13"))),
            units(Binding::Assumed(symbol("mm"))),
        ],
        vec![modaris_needs_cm()],
    );
    let Outcome::ProvenUnsatisfiable(unsat) = &outcome else {
        panic!("{outcome:?}")
    };
    assert!(unsat
        .core
        .iter()
        .any(|item| matches!(item, CoreItem::Fact { assumed: true, .. })));
    assert_eq!(
        problem.explain(&outcome, &Catalog::english())[1],
        "Pattern units is assumed to be millimeters."
    );
}

#[test]
fn a_preference_is_tried_first_and_reported_when_overridden() {
    let preferred = units(Binding::Preference(symbol("cm")));
    let (_, outcome) = solve(vec![preferred.clone()], Vec::new());
    assert_eq!(
        satisfied(&outcome).selections[0].origin,
        Origin::PreferenceHeld
    );
    assert_eq!(satisfied(&outcome).selections[0].value, symbol("cm"));

    let rule = Rule::hard(
        "only-mm",
        text("Only millimeters are accepted"),
        Expr::Is("units".to_owned(), symbol("mm")),
    );
    let (problem, outcome) = solve(vec![preferred], vec![rule]);
    assert_eq!(
        satisfied(&outcome).selections[0].origin,
        Origin::PreferenceOverridden(symbol("cm"))
    );
    assert_eq!(
        problem.explain(&outcome, &Catalog::english()),
        ["Pattern units is millimeters, not the preferred centimeters."]
    );
}

#[test]
fn a_soft_rule_shapes_the_answer_and_a_violated_one_is_reported() {
    let prefer_cm = Rule::soft(
        "prefer-cm",
        text("Centimeters are preferred"),
        Expr::Is("units".to_owned(), symbol("cm")),
    );
    let (_, outcome) = solve(vec![units(Binding::Choice)], vec![prefer_cm.clone()]);
    assert_eq!(
        satisfied(&outcome).selections[0].value,
        symbol("cm"),
        "a soft rule that can hold does"
    );
    assert!(satisfied(&outcome).violated_soft.is_empty());

    let only_mm = Rule::hard(
        "only-mm",
        text("Only millimeters are accepted"),
        Expr::Is("units".to_owned(), symbol("mm")),
    );
    let (problem, outcome) = solve(vec![units(Binding::Choice)], vec![only_mm, prefer_cm]);
    assert_eq!(satisfied(&outcome).violated_soft, ["prefer-cm"]);
    assert_eq!(
        problem.explain(&outcome, &Catalog::english()),
        ["Centimeters are preferred (not required, but it does not hold)"]
    );
}

#[test]
fn the_work_limit_says_unknown_not_unsatisfiable() {
    let variables = (0..12)
        .map(|index| {
            Variable::new(
                &format!("x{index}"),
                &format!("X{index}"),
                Domain::Integers {
                    low: 0,
                    high: 9,
                    step: 1,
                    descending: false,
                    unit: None,
                },
                Binding::Choice,
            )
        })
        .collect();
    let total = Linear {
        terms: (0..12)
            .map(|index| (Rational::int(1), format!("x{index}")))
            .collect(),
        constant: Rational::ZERO,
        unit: None,
    };
    let rule = Rule::hard(
        "exact-total",
        text("The total is fixed"),
        Expr::Compare(
            Comparison::Equal,
            total,
            Linear::constant(Rational::int(53), None),
        ),
    );
    let problem = Problem::new(AST_VERSION, variables, vec![rule]).unwrap();
    let outcome = problem.solve(Limits { max_steps: 200 });
    assert!(
        matches!(outcome, Outcome::Unknown { steps } if steps > 200),
        "{outcome:?}"
    );
    assert!(problem.explain(&outcome, &Catalog::english())[0].contains("Nothing is known"));
    assert!(
        matches!(problem.solve(Limits::default()), Outcome::Satisfied(_)),
        "more work finds the answer"
    );
}

#[test]
fn a_number_too_large_to_hold_exactly_is_numerically_failed() {
    let huge = Rational::new(i64::MAX, 1).unwrap();
    let doubled = huge.checked_mul(huge).unwrap();
    let variable = Variable::new(
        "x",
        "X",
        Domain::Numbers {
            values: vec![huge, Rational::int(1)],
            unit: None,
        },
        Binding::Choice,
    );
    let term = Linear {
        terms: vec![(doubled, "x".to_owned())],
        constant: Rational::ZERO,
        unit: None,
    };
    let rule = Rule::hard(
        "too-large",
        text("Too large"),
        Expr::Compare(
            Comparison::AtLeast,
            term,
            Linear::constant(Rational::ZERO, None),
        ),
    );
    let (problem, outcome) = solve(vec![variable], vec![rule]);
    assert_eq!(
        outcome,
        Outcome::NumericallyFailed {
            rule: "too-large".to_owned(),
            fields: vec!["x".to_owned()]
        }
    );
    assert_eq!(
        problem.explain(&outcome, &Catalog::english()),
        ["Rule too-large needs a number too large to hold exactly, so it could not be decided."]
    );
}

#[test]
fn overflow_at_either_end_of_a_range_is_numerically_failed() {
    let huge = Rational::new(i64::MAX, 1).unwrap();
    let nearly = Rational::new(i64::MAX - 1, 1).unwrap();
    let square = huge.checked_mul(huge).unwrap();
    let variable = Variable::new(
        "x",
        "X",
        Domain::Numbers {
            values: vec![huge, nearly],
            unit: None,
        },
        Binding::Choice,
    );
    let term = Linear {
        terms: vec![(square, "x".to_owned())],
        constant: Rational::ZERO,
        unit: None,
    };
    let rule = Rule::hard(
        "too-large",
        text("Too large"),
        Expr::Compare(
            Comparison::AtLeast,
            term,
            Linear::constant(Rational::ZERO, None),
        ),
    );
    let (_, outcome) = solve(vec![variable], vec![rule]);
    assert_eq!(
        outcome,
        Outcome::NumericallyFailed {
            rule: "too-large".to_owned(),
            fields: vec!["x".to_owned()]
        }
    );
}

#[test]
fn equal_fractions_are_one_value() {
    let half = Rational::new(1, 2).unwrap();
    assert_eq!(Rational::new(2, 4), Some(half));
    assert_eq!(Rational::new(-1, -2), Some(half));
    assert_eq!(Rational::new(1, -2).unwrap().to_string(), "-1/2");
    assert_eq!(Rational::new(4, 2).unwrap().to_string(), "2");
    assert_eq!(Rational::new(1, 0), None);
    assert_eq!(half.denominator(), 2);

    let repeated = Domain::Numbers {
        values: vec![half, Rational::new(2, 4).unwrap()],
        unit: None,
    };
    assert_eq!(
        refused(
            vec![Variable::new("x", "X", repeated, Binding::Choice)],
            Vec::new()
        ),
        ProblemError::DuplicateValue("x".to_owned())
    );

    let values = Domain::Numbers {
        values: vec![Rational::int(1), half],
        unit: None,
    };
    let rule = Rule::hard(
        "half",
        text("It is a half"),
        Expr::Is("x".to_owned(), Value::Number(Rational::new(2, 4).unwrap())),
    );
    let (_, outcome) = solve(
        vec![Variable::new("x", "X", values, Binding::Choice)],
        vec![rule],
    );
    assert_eq!(satisfied(&outcome).selections[0].value, Value::Number(half));
}

#[test]
fn a_large_domain_is_narrowed_without_visiting_every_value() {
    let wide = |name: &str| {
        Variable::new(
            name,
            name,
            Domain::Integers {
                low: 0,
                high: 60_000,
                step: 1,
                descending: false,
                unit: Some("mm".to_owned()),
            },
            Binding::Choice,
        )
    };
    let rule = Rule::hard(
        "sum-limit",
        text("The two widths fit"),
        Expr::Compare(
            Comparison::AtMost,
            Linear {
                terms: vec![
                    (Rational::int(1), "x".to_owned()),
                    (Rational::int(1), "y".to_owned()),
                ],
                constant: Rational::ZERO,
                unit: None,
            },
            Linear::constant(Rational::int(10), Some("mm")),
        ),
    );
    let (_, outcome) = solve(vec![wide("x"), wide("y")], vec![rule]);
    assert_eq!(
        satisfied(&outcome).selections[0].value,
        Value::Number(Rational::int(0))
    );
    assert!(
        satisfied(&outcome).steps < 1_000_000,
        "{}",
        satisfied(&outcome).steps
    );
}

#[test]
fn the_solve_is_deterministic_across_repeated_runs() {
    let variables = vec![
        system(Binding::Known(symbol("modaris13"))),
        units(Binding::Unknown),
    ];
    let problem = Problem::new(AST_VERSION, variables, vec![modaris_needs_cm()]).unwrap();
    let first = format!("{:?}", problem.solve(Limits::default()));
    for _ in 0..50 {
        assert_eq!(first, format!("{:?}", problem.solve(Limits::default())));
    }
}

fn refused(variables: Vec<Variable>, rules: Vec<Rule>) -> ProblemError {
    Problem::new(AST_VERSION, variables, rules).expect_err("this problem must not validate")
}

#[test]
fn a_problem_with_a_wrong_name_type_unit_or_domain_does_not_build() {
    assert_eq!(
        Problem::new(99, Vec::new(), Vec::new()).unwrap_err(),
        ProblemError::UnsupportedVersion(99)
    );
    assert_eq!(
        refused(
            vec![units(Binding::Choice), units(Binding::Choice)],
            Vec::new()
        ),
        ProblemError::DuplicateVariable("units".to_owned())
    );
    assert_eq!(
        refused(vec![units(Binding::Known(symbol("inch")))], Vec::new()),
        ProblemError::ValueOutsideDomain("units".to_owned())
    );
    assert_eq!(
        refused(
            vec![Variable::new(
                "empty",
                "E",
                Domain::symbols(&[]),
                Binding::Choice
            )],
            Vec::new()
        ),
        ProblemError::EmptyDomain("empty".to_owned())
    );
    let duplicate = Domain::symbols(&[("a", "A"), ("a", "A again")]);
    assert_eq!(
        refused(
            vec![Variable::new("twice", "T", duplicate, Binding::Choice)],
            Vec::new()
        ),
        ProblemError::DuplicateValue("twice".to_owned())
    );
    let backwards = Domain::Integers {
        low: 5,
        high: 1,
        step: 1,
        descending: false,
        unit: None,
    };
    assert_eq!(
        refused(
            vec![Variable::new("range", "R", backwards, Binding::Choice)],
            Vec::new()
        ),
        ProblemError::BadRange("range".to_owned())
    );
    let huge = Domain::Integers {
        low: 0,
        high: 70_000,
        step: 1,
        descending: false,
        unit: None,
    };
    assert_eq!(
        refused(
            vec![Variable::new("big", "B", huge, Binding::Choice)],
            Vec::new()
        ),
        ProblemError::DomainTooLarge("big".to_owned())
    );

    let missing = Rule::hard("r", text("x"), Expr::Is("nope".to_owned(), symbol("mm")));
    assert_eq!(
        refused(vec![units(Binding::Choice)], vec![missing]),
        ProblemError::UnresolvedName {
            rule: "r".to_owned(),
            name: "nope".to_owned()
        }
    );
    let unknown_symbol = Rule::hard("r", text("x"), Expr::Is("units".to_owned(), symbol("inch")));
    assert_eq!(
        refused(vec![units(Binding::Choice)], vec![unknown_symbol]),
        ProblemError::TypeMismatch {
            rule: "r".to_owned(),
            name: "units".to_owned()
        }
    );
    let wrong_kind = Rule::hard(
        "r",
        text("x"),
        Expr::Is("units".to_owned(), Value::Bool(true)),
    );
    assert_eq!(
        refused(vec![units(Binding::Choice)], vec![wrong_kind]),
        ProblemError::TypeMismatch {
            rule: "r".to_owned(),
            name: "units".to_owned()
        }
    );
    let enum_in_sum = Rule::hard(
        "r",
        text("x"),
        Expr::Compare(
            Comparison::Less,
            Linear::var("units"),
            Linear::constant(Rational::int(1), None),
        ),
    );
    assert_eq!(
        refused(vec![units(Binding::Choice)], vec![enum_in_sum]),
        ProblemError::TypeMismatch {
            rule: "r".to_owned(),
            name: "units".to_owned()
        }
    );

    let width = |name: &str, unit: &str| {
        Variable::new(
            name,
            name,
            Domain::Numbers {
                values: vec![Rational::int(1)],
                unit: Some(unit.to_owned()),
            },
            Binding::Choice,
        )
    };
    let mixed = Rule::hard(
        "r",
        text("x"),
        Expr::Compare(Comparison::Less, Linear::var("a"), Linear::var("b")),
    );
    assert_eq!(
        refused(vec![width("a", "mm"), width("b", "cm")], vec![mixed]),
        ProblemError::UnitMismatch {
            rule: "r".to_owned()
        }
    );
    let bare = Rule::hard(
        "r",
        text("x"),
        Expr::Compare(
            Comparison::Less,
            Linear::var("a"),
            Linear::constant(Rational::int(5), None),
        ),
    );
    assert_eq!(
        refused(vec![width("a", "mm")], vec![bare]),
        ProblemError::UnitMismatch {
            rule: "r".to_owned()
        }
    );

    let silent = Rule::hard("r", Vec::new(), Expr::All(Vec::new()));
    assert_eq!(
        refused(Vec::new(), vec![silent]),
        ProblemError::MissingTemplate("r".to_owned())
    );
    let dangling = Rule::hard(
        "r",
        vec![TemplatePart::ValueOf("nope".to_owned())],
        Expr::All(Vec::new()),
    );
    assert_eq!(
        refused(Vec::new(), vec![dangling]),
        ProblemError::UnresolvedName {
            rule: "r".to_owned(),
            name: "nope".to_owned()
        }
    );
    let twice = || Rule::hard("r", text("x"), Expr::All(Vec::new()));
    assert_eq!(
        refused(Vec::new(), vec![twice(), twice()]),
        ProblemError::DuplicateRule("r".to_owned())
    );
}

#[test]
fn a_problem_with_no_parameters_still_has_an_answer() {
    let (_, outcome) = solve(Vec::new(), Vec::new());
    assert!(satisfied(&outcome).selections.is_empty());
    let never = Rule::hard("never", text("This cannot hold"), Expr::Any(Vec::new()));
    let (problem, outcome) = solve(Vec::new(), vec![never]);
    assert!(matches!(&outcome, Outcome::ProvenUnsatisfiable(unsat) if unsat.minimal));
    assert_eq!(
        problem.explain(&outcome, &Catalog::english()),
        ["This cannot hold"]
    );
}
