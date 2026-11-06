//! Sentences for people (roadmap §6.2). All kernel-owned text is in `Catalog`,so a language pack can replace it.
use crate::ast::{Domain, TemplatePart, Value};
use crate::problem::{Problem, Slot};
use crate::solve::{CoreItem, Origin, Outcome, Solution, Unsatisfiable};

/// The text the kernel itself owns. `{field}`, `{value}`, `{rule}` and `{values}` are filled in.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Catalog {
    /// A known fact in a core.
    pub known_fact: String,
    /// An assumption in a core.
    pub assumed_fact: String,
    /// The work limit ended the solve.
    pub unknown: String,
    /// A number did not fit.
    pub numerically_failed: String,
    /// A value not yet known.
    pub no_value: String,
    /// An unknown fact with answers that fit.
    pub open_fact: String,
    /// An unknown fact no answer fits.
    pub open_fact_none: String,
    /// A preference the rules overrode.
    pub preference_overridden: String,
    /// A soft rule that does not hold.
    pub soft_violated: String,
    /// A soft rule the solve could not decide.
    pub soft_undecided: String,
}

impl Catalog {
    /// The English text.
    pub fn english() -> Catalog {
        Catalog {
            known_fact: "{field} is {value}.".to_owned(),
            assumed_fact: "{field} is assumed to be {value}.".to_owned(),
            unknown: "The solver reached its work limit before finding an answer. Nothing is known about whether this profile can be satisfied.".to_owned(),
            numerically_failed: "Rule {rule} needs a number too large to hold exactly, so it could not be decided.".to_owned(),
            no_value: "not known".to_owned(),
            open_fact: "{field} is not known. Answers that fit: {values}.".to_owned(),
            open_fact_none: "{field} is not known, and no answer was found to fit.".to_owned(),
            preference_overridden: "{field} is {value}, not the preferred {values}.".to_owned(),
            soft_violated: "{value} (not required, but it does not hold)".to_owned(),
            soft_undecided: "{value} (could not be checked)".to_owned(),
        }
    }
}

fn fill(text: &str, pairs: &[(&str, &str)]) -> String {
    pairs
        .iter()
        .fold(text.to_owned(), |sentence, (key, value)| {
            sentence.replace(key, value)
        })
}

fn label_of(slot: &Slot, value: &Value) -> String {
    let unit = match &slot.source.domain {
        Domain::Numbers { unit, .. } | Domain::Integers { unit, .. } => unit.as_deref(),
        _ => None,
    };
    match (value, &slot.source.domain) {
        (Value::Bool(true), Domain::Bool { yes, .. }) => yes.clone(),
        (Value::Bool(false), Domain::Bool { no, .. }) => no.clone(),
        (Value::Symbol(token), Domain::Enum(symbols)) => symbols
            .iter()
            .find(|symbol| symbol.token == *token)
            .map_or_else(String::new, |symbol| symbol.label.clone()),
        (Value::Number(number), _) => match unit {
            Some(unit) => format!("{number} {unit}"),
            None => number.to_string(),
        },
        _ => String::new(),
    }
}

impl Problem {
    fn named(&self, name: &str) -> Option<&Slot> {
        self.slots.iter().find(|slot| slot.source.name == name)
    }

    fn sentence(&self, id: &str, values: &[(String, Value)], catalog: &Catalog) -> String {
        let Some(rule) = self.rules.iter().find(|rule| rule.source.id == id) else {
            return String::new();
        };
        rule.source
            .template
            .iter()
            .map(|part| match part {
                TemplatePart::Text(text) => text.clone(),
                TemplatePart::Label(name) => self
                    .named(name)
                    .map_or_else(String::new, |slot| slot.source.label.clone()),
                TemplatePart::ValueOf(name) => {
                    let found = values.iter().find(|(candidate, _)| candidate == name);
                    match (self.named(name), found) {
                        (Some(slot), Some((_, value))) => label_of(slot, value),
                        _ => catalog.no_value.clone(),
                    }
                }
            })
            .collect()
    }

    fn pinned_values(&self) -> Vec<(String, Value)> {
        self.slots
            .iter()
            .filter_map(|slot| {
                let value = slot.pinned.and_then(|held| slot.values.get(held))?;
                Some((slot.source.name.clone(), value.clone()))
            })
            .collect()
    }

    fn explain_solution(&self, solution: &Solution, catalog: &Catalog) -> Vec<String> {
        let values: Vec<(String, Value)> = solution
            .selections
            .iter()
            .map(|selection| (selection.name.clone(), selection.value.clone()))
            .collect();
        let mut lines = Vec::new();
        for selection in &solution.selections {
            if let (Origin::PreferenceOverridden(preferred), Some(slot)) =
                (&selection.origin, self.named(&selection.name))
            {
                lines.push(fill(
                    &catalog.preference_overridden,
                    &[
                        ("{field}", &slot.source.label),
                        ("{value}", &label_of(slot, &selection.value)),
                        ("{values}", &label_of(slot, preferred)),
                    ],
                ));
            }
        }
        for id in &solution.violated_soft {
            lines.push(fill(
                &catalog.soft_violated,
                &[("{value}", &self.sentence(id, &values, catalog))],
            ));
        }
        for id in &solution.undecided_soft {
            lines.push(fill(
                &catalog.soft_undecided,
                &[("{value}", &self.sentence(id, &values, catalog))],
            ));
        }
        for open in &solution.open_facts {
            let Some(slot) = self.named(&open.name) else {
                continue;
            };
            if open.feasible.is_empty() {
                lines.push(fill(
                    &catalog.open_fact_none,
                    &[("{field}", &slot.source.label)],
                ));
            } else {
                let answers: Vec<String> = open
                    .feasible
                    .iter()
                    .map(|value| label_of(slot, value))
                    .collect();
                lines.push(fill(
                    &catalog.open_fact,
                    &[
                        ("{field}", &slot.source.label),
                        ("{values}", &answers.join(", ")),
                    ],
                ));
            }
        }
        lines
    }

    fn explain_unsat(&self, unsat: &Unsatisfiable, catalog: &Catalog) -> Vec<String> {
        let pinned = self.pinned_values();
        unsat
            .core
            .iter()
            .map(|item| match item {
                CoreItem::Fact {
                    name,
                    value,
                    assumed,
                    ..
                } => {
                    let Some(slot) = self.named(name) else {
                        return String::new();
                    };
                    let text = if *assumed {
                        &catalog.assumed_fact
                    } else {
                        &catalog.known_fact
                    };
                    fill(
                        text,
                        &[
                            ("{field}", &slot.source.label),
                            ("{value}", &label_of(slot, value)),
                        ],
                    )
                }
                CoreItem::Rule { id, .. } => self.sentence(id, &pinned, catalog),
            })
            .collect()
    }

    /// The outcome in sentences, in declared order. A satisfied outcome with nothing to report has none.
    pub fn explain(&self, outcome: &Outcome, catalog: &Catalog) -> Vec<String> {
        match outcome {
            Outcome::Satisfied(solution) => self.explain_solution(solution, catalog),
            Outcome::ProvenUnsatisfiable(unsat) => self.explain_unsat(unsat, catalog),
            Outcome::Unknown { .. } => vec![catalog.unknown.clone()],
            Outcome::NumericallyFailed { rule, .. } => {
                vec![fill(&catalog.numerically_failed, &[("{rule}", rule)])]
            }
        }
    }
}
