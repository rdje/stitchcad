//! Every seeded problem is solved by the kernel and by plain enumeration;any difference is a kernel bug.
#![allow(clippy::panic, clippy::indexing_slicing, clippy::expect_used)]
mod common;

use common::{all_pins, first_solution, first_where, generate, hard_rules, holds, names, VERSION};
use sc_constraints::{Binding, CoreItem, Limits, Outcome, Problem, Rule, Status, Value, Variable};

const SEEDS: u64 = 4_000;

fn check(seed: u64) {
    let (variables, rules) = generate(seed);
    let problem = Problem::new(VERSION, variables.clone(), rules.clone())
        .unwrap_or_else(|error| panic!("seed {seed}: a generated problem must validate: {error}"));
    let outcome = problem.solve(Limits::default());
    assert_eq!(
        outcome,
        problem.solve(Limits::default()),
        "seed {seed}: two runs differ"
    );

    let pins = all_pins(&variables);
    let hard = hard_rules(&rules);
    match first_solution(&variables, &hard, &pins) {
        None => check_unsat(seed, &variables, &rules, &outcome),
        Some(_) => check_sat(seed, &variables, &rules, &outcome),
    }
}

fn check_unsat(seed: u64, variables: &[Variable], rules: &[Rule], outcome: &Outcome) {
    let Outcome::ProvenUnsatisfiable(unsat) = outcome else {
        panic!("seed {seed}: enumeration finds no solution, the kernel says {outcome:?}");
    };
    assert!(
        unsat.minimal,
        "seed {seed}: a small problem must reach a minimal core"
    );
    let mut pins = vec![false; variables.len()];
    let mut kept: Vec<&Rule> = Vec::new();
    for item in &unsat.core {
        match item {
            CoreItem::Fact { name, value, .. } => {
                let position = variables
                    .iter()
                    .position(|variable| variable.name == *name)
                    .expect("a fact names a parameter");
                pins[position] = true;
                let held = match &variables[position].binding {
                    Binding::Known(held) | Binding::Assumed(held) => held,
                    other => panic!("seed {seed}: core holds {name}, bound as {other:?}"),
                };
                assert_eq!(held, value, "seed {seed}: core misreports {name}");
            }
            CoreItem::Rule { id, .. } => {
                let rule = rules
                    .iter()
                    .find(|rule| rule.id == *id)
                    .expect("a core rule exists");
                assert_eq!(
                    rule.status,
                    Status::Hard,
                    "seed {seed}: soft rule {id} in a core"
                );
                kept.push(rule);
            }
        }
    }
    assert!(
        first_solution(variables, &kept, &pins).is_none(),
        "seed {seed}: the core alone has a solution"
    );
    for (index, removed) in kept.iter().enumerate() {
        let rest: Vec<&Rule> = kept
            .iter()
            .enumerate()
            .filter(|(other, _)| *other != index)
            .map(|(_, rule)| *rule)
            .collect();
        assert!(
            first_solution(variables, &rest, &pins).is_some(),
            "seed {seed}: rule {} is not needed",
            removed.id
        );
    }
    for index in 0..pins.len() {
        if pins[index] {
            let mut fewer = pins.clone();
            fewer[index] = false;
            assert!(
                first_solution(variables, &kept, &fewer).is_some(),
                "seed {seed}: fact {index} is not needed"
            );
        }
    }
}

fn check_sat(seed: u64, variables: &[Variable], rules: &[Rule], outcome: &Outcome) {
    let Outcome::Satisfied(solution) = outcome else {
        panic!("seed {seed}: enumeration finds a solution, the kernel says {outcome:?}");
    };
    let pins = all_pins(variables);
    // Soft rules in declared order, each kept if a solution remains.
    let mut kept = hard_rules(rules);
    let mut violated = Vec::new();
    for rule in rules.iter().filter(|rule| rule.status == Status::Soft) {
        let mut trial = kept.clone();
        trial.push(rule);
        if first_solution(variables, &trial, &pins).is_some() {
            kept = trial;
        } else {
            violated.push(rule.id.clone());
        }
    }
    assert_eq!(
        solution.violated_soft, violated,
        "seed {seed}: soft rules differ"
    );
    assert!(
        solution.undecided_soft.is_empty(),
        "seed {seed}: nothing is undecided on a small problem"
    );
    let first = first_solution(variables, &kept, &pins).expect("kept rules have a solution");

    // The kernel returns the first solution in declared order, and never reports an unknown fact's value.
    let mut reported = solution.selections.iter();
    for (variable, value) in variables.iter().zip(&first) {
        if matches!(variable.binding, Binding::Unknown) {
            assert!(
                solution
                    .selections
                    .iter()
                    .all(|selection| selection.name != variable.name),
                "seed {seed}: unknown {} assigned",
                variable.name
            );
        } else {
            let selection = reported.next().expect("every other parameter is selected");
            assert_eq!(selection.name, variable.name, "seed {seed}");
            assert_eq!(
                selection.value, *value,
                "seed {seed}: {} differs from the first solution",
                variable.name
            );
        }
    }
    assert!(reported.next().is_none(), "seed {seed}: extra selections");

    // The answers offered for an unknown fact are exactly those that have a solution.
    let all = names(variables);
    let hard = hard_rules(rules);
    let open: Vec<&Variable> = variables
        .iter()
        .filter(|variable| matches!(variable.binding, Binding::Unknown))
        .collect();
    assert_eq!(solution.open_facts.len(), open.len(), "seed {seed}");
    for (fact, variable) in solution.open_facts.iter().zip(open) {
        let position = all
            .iter()
            .position(|name| *name == variable.name)
            .expect("the parameter exists");
        let expected: Vec<Value> = common::values_of(&variable.domain)
            .into_iter()
            .filter(|answer| {
                first_where(variables, &pins, &mut |assignment| {
                    assignment[position] == *answer
                        && hard
                            .iter()
                            .all(|rule| holds(&rule.condition, &all, assignment))
                })
                .is_some()
            })
            .collect();
        assert!(
            fact.complete,
            "seed {seed}: a small problem must finish its answers"
        );
        assert_eq!(
            fact.feasible, expected,
            "seed {seed}: answers for {} differ",
            variable.name
        );
    }
}

#[test]
fn the_kernel_agrees_with_enumeration_on_every_seed() {
    let (mut sat, mut unsat) = (0, 0);
    for seed in 0..SEEDS {
        check(seed);
        let (variables, rules) = generate(seed);
        if first_solution(&variables, &hard_rules(&rules), &all_pins(&variables)).is_some() {
            sat += 1;
        } else {
            unsat += 1;
        }
    }
    // A campaign that only ever saw one outcome would prove little.
    assert!(
        sat > SEEDS / 5 && unsat > SEEDS / 5,
        "unbalanced campaign: {sat} satisfiable, {unsat} not"
    );
    println!(
        "{SEEDS} seeds: {sat} satisfiable, {unsat} unsatisfiable, kernel and enumeration agree"
    );
}
