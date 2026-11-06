//! Differential oracle (roadmap §6.1):the same seeded problems go to Z3 as SMT-LIB text;any divergence is a kernel bug.
//!
//! Runs only with `--features csp-z3`. It starts the `z3` executable, or the one `Z3_EXECUTABLE` names.
//! A missing executable fails the run:a skipped oracle would look like a passing one.
#![cfg(feature = "csp-z3")]
#![allow(clippy::panic, clippy::indexing_slicing, clippy::expect_used)]
mod common;

use common::{generate, values_of, VERSION};
use sc_constraints::{
    Binding, Comparison, CoreItem, Expr, Limits, Linear, Outcome, Problem, Rational, Rule, Status,
    Value, Variable,
};
use std::fmt::Write as _;
use std::io::Write as _;
use std::process::{Command, Stdio};

const SEEDS: u64 = 1_000;

fn real(value: Rational) -> String {
    let (top, bottom) = (value.numerator(), value.denominator());
    if top < 0 {
        format!("(- (/ {} {bottom}))", -top)
    } else {
        format!("(/ {top} {bottom})")
    }
}

fn position(variables: &[Variable], name: &str) -> usize {
    variables
        .iter()
        .position(|variable| variable.name == name)
        .expect("a validated name resolves")
}

/// Position in the domain's declared order, which is all the encoding relies on.
fn index_of(variable: &Variable, value: &Value) -> Option<usize> {
    values_of(&variable.domain)
        .iter()
        .position(|candidate| candidate == value)
}

fn sum(linear: &Linear, variables: &[Variable]) -> String {
    let mut parts = vec![real(linear.constant)];
    for (coefficient, name) in &linear.terms {
        parts.push(format!(
            "(* {} val{})",
            real(*coefficient),
            position(variables, name)
        ));
    }
    format!("(+ {} 0.0)", parts.join(" "))
}

fn encode(expr: &Expr, variables: &[Variable]) -> String {
    match expr {
        Expr::Is(name, value) => {
            let at = position(variables, name);
            index_of(&variables[at], value).map_or_else(
                || "false".to_owned(),
                |index| format!("(= pos{at} {index})"),
            )
        }
        Expr::OneOf(name, values) => {
            let parts: Vec<String> = values
                .iter()
                .map(|value| encode(&Expr::Is(name.clone(), value.clone()), variables))
                .collect();
            format!("(or false {})", parts.join(" "))
        }
        Expr::Same(left, right) => {
            let (first, second) = (position(variables, left), position(variables, right));
            let mut parts = vec!["false".to_owned()];
            for (i, a) in values_of(&variables[first].domain).iter().enumerate() {
                for (j, b) in values_of(&variables[second].domain).iter().enumerate() {
                    if a == b {
                        parts.push(format!("(and (= pos{first} {i}) (= pos{second} {j}))"));
                    }
                }
            }
            format!("(or {})", parts.join(" "))
        }
        Expr::Compare(op, left, right) => {
            let (a, b) = (sum(left, variables), sum(right, variables));
            match op {
                Comparison::Equal => format!("(= {a} {b})"),
                Comparison::NotEqual => format!("(not (= {a} {b}))"),
                Comparison::Less => format!("(< {a} {b})"),
                Comparison::AtMost => format!("(<= {a} {b})"),
                Comparison::Greater => format!("(> {a} {b})"),
                Comparison::AtLeast => format!("(>= {a} {b})"),
            }
        }
        Expr::Not(inner) => format!("(not {})", encode(inner, variables)),
        Expr::All(items) => format!(
            "(and true {})",
            items
                .iter()
                .map(|item| encode(item, variables))
                .collect::<Vec<_>>()
                .join(" ")
        ),
        Expr::Any(items) => format!(
            "(or false {})",
            items
                .iter()
                .map(|item| encode(item, variables))
                .collect::<Vec<_>>()
                .join(" ")
        ),
        Expr::Implies(first, second) => format!(
            "(=> {} {})",
            encode(first, variables),
            encode(second, variables)
        ),
    }
}

/// The parameters as bounded positions, with numeric values attached.
fn declarations(variables: &[Variable]) -> String {
    let mut text = String::new();
    for (at, variable) in variables.iter().enumerate() {
        let values = values_of(&variable.domain);
        let _ = writeln!(
            text,
            "(declare-const pos{at} Int)\n(assert (and (<= 0 pos{at}) (< pos{at} {})))",
            values.len()
        );
        let mut chain = "0.0".to_owned();
        for (index, value) in values.iter().enumerate() {
            if let Value::Number(number) = value {
                chain = format!("(ite (= pos{at} {index}) {} {chain})", real(*number));
            }
        }
        let _ = writeln!(text, "(define-fun val{at} () Real {chain})");
    }
    text
}

fn z3(script: &str, seed: u64) -> String {
    let program = std::env::var("Z3_EXECUTABLE").unwrap_or_else(|_| "z3".to_owned());
    let mut child = Command::new(&program)
        .args(["-in", "-smt2"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|error| panic!("the csp-z3 oracle cannot start {program}: {error}"));
    child
        .stdin
        .take()
        .expect("stdin is piped")
        .write_all(script.as_bytes())
        .expect("z3 reads the script");
    let output = child.wait_with_output().expect("z3 finishes");
    let verdict = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    assert!(
        verdict == "sat" || verdict == "unsat",
        "seed {seed}: z3 answered {verdict:?}\n{script}"
    );
    verdict
}

fn pin(variables: &[Variable], at: usize) -> Option<String> {
    match &variables[at].binding {
        Binding::Known(value) | Binding::Assumed(value) => {
            let index =
                index_of(&variables[at], value).expect("a validated binding is in its domain");
            Some(format!("(assert (= pos{at} {index}))\n"))
        }
        _ => None,
    }
}

fn check(seed: u64) -> &'static str {
    let (variables, rules) = generate(seed);
    let problem = Problem::new(VERSION, variables.clone(), rules.clone())
        .expect("a generated problem validates");
    let outcome = problem.solve(Limits::default());
    let base = declarations(&variables);
    let hard: String = rules
        .iter()
        .filter(|rule| rule.status == Status::Hard)
        .map(|rule| format!("(assert {})\n", encode(&rule.condition, &variables)))
        .collect();
    let pins: String = (0..variables.len())
        .filter_map(|at| pin(&variables, at))
        .collect();
    let verdict = z3(&format!("{base}{pins}{hard}(check-sat)\n"), seed);
    match &outcome {
        Outcome::Satisfied(solution) => {
            assert_eq!(
                verdict, "sat",
                "seed {seed}: the kernel found a solution and z3 did not"
            );
            // The kernel's selections, kept soft rules included, must themselves be feasible for z3.
            let mut fixed = String::new();
            for selection in &solution.selections {
                let at = position(&variables, &selection.name);
                let _ = writeln!(
                    fixed,
                    "(assert (= pos{at} {}))",
                    index_of(&variables[at], &selection.value)
                        .expect("a selection is in its domain")
                );
            }
            let kept: String = rules
                .iter()
                .filter(|rule: &&Rule| {
                    rule.status == Status::Soft && !solution.violated_soft.contains(&rule.id)
                })
                .map(|rule| format!("(assert {})\n", encode(&rule.condition, &variables)))
                .collect();
            assert_eq!(
                z3(&format!("{base}{fixed}{hard}{kept}(check-sat)\n"), seed),
                "sat",
                "seed {seed}: z3 rejects the kernel's selections"
            );
            // Each soft rule, in declared order, is violated exactly when it conflicts with the rules kept before it.
            let mut before = String::new();
            for rule in rules.iter().filter(|rule| rule.status == Status::Soft) {
                let this = format!("(assert {})\n", encode(&rule.condition, &variables));
                let answer = z3(
                    &format!("{base}{pins}{hard}{before}{this}(check-sat)\n"),
                    seed,
                );
                assert_eq!(
                    answer == "unsat",
                    solution.violated_soft.contains(&rule.id),
                    "seed {seed}: soft rule {} differs",
                    rule.id
                );
                if answer == "sat" {
                    before.push_str(&this);
                }
            }
            for fact in &solution.open_facts {
                let at = position(&variables, &fact.name);
                for (index, value) in values_of(&variables[at].domain).iter().enumerate() {
                    let answer = z3(
                        &format!("{base}{pins}{hard}(assert (= pos{at} {index}))\n(check-sat)\n"),
                        seed,
                    );
                    assert_eq!(
                        answer == "sat",
                        fact.feasible.contains(value),
                        "seed {seed}: answers for {} differ",
                        fact.name
                    );
                }
            }
            "sat"
        }
        Outcome::ProvenUnsatisfiable(unsat) => {
            assert_eq!(
                verdict, "unsat",
                "seed {seed}: the kernel proved infeasibility and z3 found a model"
            );
            let mut core = String::new();
            for item in &unsat.core {
                match item {
                    CoreItem::Fact { name, .. } => core.push_str(
                        &pin(&variables, position(&variables, name))
                            .expect("a core fact is pinned"),
                    ),
                    CoreItem::Rule { id, .. } => {
                        let rule = rules
                            .iter()
                            .find(|rule| rule.id == *id)
                            .expect("a core rule exists");
                        let _ = writeln!(core, "(assert {})", encode(&rule.condition, &variables));
                    }
                }
            }
            assert_eq!(
                z3(&format!("{base}{core}(check-sat)\n"), seed),
                "unsat",
                "seed {seed}: the core alone is feasible for z3"
            );
            "unsat"
        }
        other => panic!("seed {seed}: a small problem cannot end in {other:?}"),
    }
}

#[test]
fn the_kernel_agrees_with_z3_on_every_seed() {
    let (mut sat, mut unsat) = (0, 0);
    for seed in 0..SEEDS {
        if check(seed) == "sat" {
            sat += 1;
        } else {
            unsat += 1;
        }
    }
    assert!(
        sat > SEEDS / 5 && unsat > SEEDS / 5,
        "unbalanced campaign: {sat} satisfiable, {unsat} not"
    );
    println!("{SEEDS} seeds: {sat} satisfiable, {unsat} unsatisfiable, kernel and z3 agree");
}
