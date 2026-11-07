# Constraint kernel

A Factory Profile is a set of parameters and the rules between them: this system needs that unit, that
plotter accepts only these formats. The constraint kernel takes those parameters and rules and answers one
question: which values fit? It is the `sc-constraints` crate, the first of the three passes in roadmap §6.1.

**What works today.** You can build a problem in Rust, solve it, and read the result in sentences, on native
and in the browser build. **What does not exist yet:** turning a [profile file](factory-profiles.md) into a problem, composing several profiles by
precedence, working out which unknowns matter to one artifact, and the Profile Editor. Those belong to G4, and
this kernel is what they will call.

## The vocabulary

| Name | What it is |
| --- | --- |
| `sc-constraints` | The crate. Pure Rust, no dependencies |
| `Problem` | A validated set of parameters and rules. `Problem::new` refuses anything malformed |
| `Variable` | One parameter: name, profile field, label, domain, and what is known about it |
| `Domain` | The values a parameter can take, in the order the search tries them |
| `Binding` | What is known about a parameter |
| `Known` | A known fact. The solver cannot change it |
| `Assumed` | A recorded assumption. Held like a fact, and reported as an assumption |
| `Unknown` | An unknown fact. The solver never gives it a value |
| `Choice` | A design choice. The solver selects the first value that fits |
| `Preference` | A preferred value, tried first, and reported if the rules override it |
| `Rule` | One constraint: stable ID, scope, hard or soft, provenance, sentence and condition |
| `Expr` | A rule's condition. A typed tree, never a string |
| `Limits` | How much work one solve may do, counted in solver steps, never in time |
| `Outcome` | The result, which is one of the four below |
| `Catalog` | The sentences the kernel itself owns, so a language pack can replace them |
| `csp-z3` | The non-default feature that compares the kernel with Z3 during development |
| `z3` | The Z3 executable, which only that feature's test starts |

## Four outcomes, never merged

| Outcome | What it means |
| --- | --- |
| `Satisfied` | Values exist. Facts still unknown are listed, not filled in |
| `ProvenUnsatisfiable` | No values can exist. The core names the facts and rules in conflict, with their profile fields |
| `Unknown` | The work limit ended the solve. This says nothing about whether the profile can be satisfied |
| `NumericallyFailed` | A number was too large to hold exactly, so one named rule could not be decided |

A result of `Unknown` is not a soft "no", and it is never reported as one.

## A choice the rules decide

The search walks parameters in the order they are declared, and each one's values in its domain's order. So
the first declared value that fits is the one selected, and the same problem always gives the same answer.

```rust
use sc_constraints::{Binding, Domain, Expr, Limits, Outcome, Problem, Rule, TemplatePart, Value, Variable, AST_VERSION};

let symbol = |token: &str| Value::Symbol(token.to_owned());
let system = Variable::new("system", "Factory system",
    Domain::symbols(&[("modaris13", "Lectra Modaris 13.x"), ("other", "another system")]),
    Binding::Known(symbol("modaris13")));
let units = Variable::new("units", "Pattern units",
    Domain::symbols(&[("mm", "millimeters"), ("cm", "centimeters")]),
    Binding::Choice);
let rule = Rule::hard("modaris-units",
    vec![
        TemplatePart::Text("Your factory uses ".to_owned()),
        TemplatePart::ValueOf("system".to_owned()),
        TemplatePart::Text(" — this profile requires centimeters".to_owned()),
    ],
    Expr::Implies(
        Box::new(Expr::Is("system".to_owned(), symbol("modaris13"))),
        Box::new(Expr::Is("units".to_owned(), symbol("cm"))),
    ));
let problem = Problem::new(AST_VERSION, vec![system, units], vec![rule]).expect("a valid problem");

let Outcome::Satisfied(solution) = problem.solve(Limits::default()) else { panic!("this one fits") };
let chosen = solution.selections.iter().find(|selection| selection.name == "units").expect("selected");
assert_eq!(chosen.value, symbol("cm"));
```

Millimeters come first in the domain, but the rule excludes them, so centimeters are selected.

## A conflict, explained

When facts and rules cannot all hold, the kernel proves it and shrinks the conflict to a minimal core: remove
any one member and the rest can hold. Each member names its profile field, so a form can point at the field
to fix. The sentences come from the rule's own template and from `Catalog`.

```rust
use sc_constraints::{Binding, Catalog, Domain, Expr, Limits, Outcome, Problem, Rule, TemplatePart, Value, Variable, AST_VERSION};

let symbol = |token: &str| Value::Symbol(token.to_owned());
let system = Variable::new("system", "Factory system",
    Domain::symbols(&[("modaris13", "Lectra Modaris 13.x"), ("other", "another system")]),
    Binding::Known(symbol("modaris13")));
let units = Variable::new("units", "Pattern units",
    Domain::symbols(&[("mm", "millimeters"), ("cm", "centimeters")]),
    Binding::Known(symbol("mm")));
let rule = Rule::hard("modaris-units",
    vec![
        TemplatePart::Text("Your factory uses ".to_owned()),
        TemplatePart::ValueOf("system".to_owned()),
        TemplatePart::Text(" — this profile requires centimeters".to_owned()),
    ],
    Expr::Implies(
        Box::new(Expr::Is("system".to_owned(), symbol("modaris13"))),
        Box::new(Expr::Is("units".to_owned(), symbol("cm"))),
    ));
let problem = Problem::new(AST_VERSION, vec![system, units], vec![rule]).expect("a valid problem");

let outcome = problem.solve(Limits::default());
assert!(matches!(&outcome, Outcome::ProvenUnsatisfiable(unsat) if unsat.minimal));
assert_eq!(problem.explain(&outcome, &Catalog::english()), [
    "Factory system is Lectra Modaris 13.x.",
    "Pattern units is millimeters.",
    "Your factory uses Lectra Modaris 13.x — this profile requires centimeters",
]);
```

## An unknown fact stays unknown

The solver never assigns an unknown fact, even when only one answer fits. It reports which answers would keep
the profile feasible, and a person gives the answer.

```rust
use sc_constraints::{Binding, Catalog, Domain, Expr, Limits, Outcome, Problem, Rule, TemplatePart, Value, Variable, AST_VERSION};

let symbol = |token: &str| Value::Symbol(token.to_owned());
let units = Variable::new("units", "Pattern units",
    Domain::symbols(&[("mm", "millimeters"), ("cm", "centimeters")]),
    Binding::Unknown);
let rule = Rule::hard("only-cm", vec![TemplatePart::Text("This profile requires centimeters".to_owned())],
    Expr::Is("units".to_owned(), symbol("cm")));
let problem = Problem::new(AST_VERSION, vec![units], vec![rule]).expect("a valid problem");

let outcome = problem.solve(Limits::default());
let Outcome::Satisfied(solution) = &outcome else { panic!("this one fits") };
assert!(solution.selections.is_empty());
assert_eq!(solution.open_facts[0].feasible, [symbol("cm")]);
assert_eq!(problem.explain(&outcome, &Catalog::english()),
    ["Pattern units is not known. Answers that fit: centimeters."]);
```

## Preferences and soft rules

A preference is tried first and gives way to a hard rule. A soft rule holds whenever it can, in declared order,
and one that cannot hold is reported. Neither one can make a profile infeasible.

```rust
use sc_constraints::{Binding, Catalog, Domain, Expr, Limits, Outcome, Problem, Rule, TemplatePart, Value, Variable, AST_VERSION};

let symbol = |token: &str| Value::Symbol(token.to_owned());
let text = |words: &str| vec![TemplatePart::Text(words.to_owned())];
let units = Variable::new("units", "Pattern units",
    Domain::symbols(&[("mm", "millimeters"), ("cm", "centimeters")]),
    Binding::Preference(symbol("cm")));
let only_mm = Rule::hard("only-mm", text("Only millimeters are accepted"), Expr::Is("units".to_owned(), symbol("mm")));
let nice = Rule::soft("label-cm", text("Centimeter labels read better"), Expr::Is("units".to_owned(), symbol("cm")));
let problem = Problem::new(AST_VERSION, vec![units], vec![only_mm, nice]).expect("a valid problem");

let outcome = problem.solve(Limits::default());
assert!(matches!(&outcome, Outcome::Satisfied(_)));
assert_eq!(problem.explain(&outcome, &Catalog::english()), [
    "Pattern units is millimeters, not the preferred centimeters.",
    "Centimeter labels read better (not required, but it does not hold)",
]);
```

## What a problem may contain

Parameters are booleans, enumerated symbols, exact fractions and whole-number ranges. Conditions compare linear
sums and combine with not, all, any and implies. Real-valued nonlinear constraints are outside this version.

A domain holds at most 65,536 values. Arithmetic is exact, with no floating point anywhere. These are refused
when the problem is built, each with a message naming what to fix:

- a name that does not resolve, including a symbol its enumeration does not declare;
- a value of the wrong kind, or a known value outside its domain;
- a comparison across different units;
- a rule with no sentence, or a repeated parameter or rule;
- an AST version other than the one this solver reads.

## How it is checked

Run `cargo test -p sc-constraints`. Every seeded random problem, 4,000 of them, is also solved by plain
enumeration in the test itself, which shares no code with the solver. They must agree on the outcome, on the
first solution in declared order, on the answers offered for each unknown fact, and on every core being minimal.

The `csp-z3` feature runs 1,000 such problems against Z3 too, and hosted CI runs it with the distribution's
`z3` package. It starts Z3 as a separate program over SMT-LIB text, so Z3 is never compiled, linked or shipped.
With the feature off, `cargo tree -p sc-constraints` lists the crate alone.

**That comparison has not run yet.** The test compiles, and its first run comes with the next push to the
hosted CI. Until then, only the enumeration above has checked the kernel.
