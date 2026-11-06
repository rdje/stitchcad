//! Seeded problem generator and an exhaustive oracle that shares no code with the solver.
#![allow(
    dead_code,
    clippy::indexing_slicing,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
use sc_constraints::{
    Binding, Comparison, Domain, Expr, Linear, Rational, Rule, Status, TemplatePart, Value,
    Variable, AST_VERSION,
};

/// SplitMix64. The seed is recorded in every failure message (roadmap §6.3).
pub struct Random(u64);

impl Random {
    pub fn new(seed: u64) -> Random {
        Random(seed)
    }

    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    pub fn below(&mut self, bound: usize) -> usize {
        (self.next() % bound as u64) as usize
    }

    fn range(&mut self, low: i64, high: i64) -> i64 {
        low + self.below((high - low + 1) as usize) as i64
    }

    fn chance(&mut self, percent: usize) -> bool {
        self.below(100) < percent
    }
}

fn fraction(random: &mut Random) -> Rational {
    Rational::new(random.range(-6, 6), random.range(1, 4)).unwrap_or(Rational::ZERO)
}

fn domain(random: &mut Random) -> Domain {
    match random.below(4) {
        0 => Domain::Bool {
            first: random.chance(50),
            yes: "yes".to_owned(),
            no: "no".to_owned(),
        },
        1 => {
            let count = 2 + random.below(3);
            let pairs: Vec<(String, String)> = (0..count)
                .map(|i| (format!("s{i}"), format!("S{i}")))
                .collect();
            let borrowed: Vec<(&str, &str)> = pairs
                .iter()
                .map(|(t, l)| (t.as_str(), l.as_str()))
                .collect();
            Domain::symbols(&borrowed)
        }
        2 => {
            let mut values: Vec<Rational> = Vec::new();
            for _ in 0..(2 + random.below(3)) {
                let value = fraction(random);
                if !values.contains(&value) {
                    values.push(value);
                }
            }
            Domain::Numbers {
                values,
                unit: Some("mm".to_owned()),
            }
        }
        _ => {
            let low = random.range(-4, 3);
            Domain::Integers {
                low,
                high: low + random.range(1, 5),
                step: random.range(1, 2),
                descending: random.chance(30),
                unit: Some("mm".to_owned()),
            }
        }
    }
}

pub fn values_of(domain: &Domain) -> Vec<Value> {
    match domain {
        Domain::Bool { first, .. } => vec![Value::Bool(*first), Value::Bool(!*first)],
        Domain::Enum(symbols) => symbols
            .iter()
            .map(|symbol| Value::Symbol(symbol.token.clone()))
            .collect(),
        Domain::Numbers { values, .. } => {
            values.iter().map(|value| Value::Number(*value)).collect()
        }
        Domain::Integers {
            low,
            high,
            step,
            descending,
            ..
        } => {
            let mut values = Vec::new();
            let mut current = *low;
            while current <= *high {
                values.push(Value::Number(Rational::int(current)));
                current += *step;
            }
            if *descending {
                values.reverse();
            }
            values
        }
    }
}

fn variable(random: &mut Random, index: usize) -> Variable {
    let domain = domain(random);
    let values = values_of(&domain);
    let pick = values[random.below(values.len())].clone();
    let binding = match random.below(10) {
        0 | 1 => Binding::Known(pick),
        2 => Binding::Assumed(pick),
        3 | 4 => Binding::Unknown,
        5 => Binding::Preference(pick),
        _ => Binding::Choice,
    };
    Variable::new(&format!("v{index}"), &format!("V{index}"), domain, binding)
}

fn is_number(variable: &Variable) -> bool {
    matches!(
        variable.domain,
        Domain::Numbers { .. } | Domain::Integers { .. }
    )
}

fn linear(random: &mut Random, variables: &[Variable]) -> Linear {
    let numeric: Vec<&Variable> = variables.iter().filter(|v| is_number(v)).collect();
    let mut terms = Vec::new();
    for _ in 0..(1 + random.below(2)) {
        if !numeric.is_empty() {
            let chosen = numeric[random.below(numeric.len())];
            terms.push((fraction(random), chosen.name.clone()));
        }
    }
    Linear {
        terms,
        constant: fraction(random),
        unit: Some("mm".to_owned()),
    }
}

fn expression(random: &mut Random, variables: &[Variable], depth: usize) -> Expr {
    let kind = if depth == 0 {
        random.below(4)
    } else {
        random.below(8)
    };
    match kind {
        0 => {
            let chosen = &variables[random.below(variables.len())];
            let values = values_of(&chosen.domain);
            Expr::Is(
                chosen.name.clone(),
                values[random.below(values.len())].clone(),
            )
        }
        1 => {
            let chosen = &variables[random.below(variables.len())];
            let values = values_of(&chosen.domain);
            let picked = (0..(1 + random.below(2)))
                .map(|_| values[random.below(values.len())].clone())
                .collect();
            Expr::OneOf(chosen.name.clone(), picked)
        }
        2 => {
            let chosen = &variables[random.below(variables.len())];
            let same_kind: Vec<&Variable> = variables
                .iter()
                .filter(|other| {
                    std::mem::discriminant(&values_of(&other.domain)[0])
                        == std::mem::discriminant(&values_of(&chosen.domain)[0])
                })
                .collect();
            Expr::Same(
                chosen.name.clone(),
                same_kind[random.below(same_kind.len())].name.clone(),
            )
        }
        3 => {
            let op = [
                Comparison::Equal,
                Comparison::NotEqual,
                Comparison::Less,
                Comparison::AtMost,
                Comparison::Greater,
                Comparison::AtLeast,
            ][random.below(6)];
            Expr::Compare(op, linear(random, variables), linear(random, variables))
        }
        4 => Expr::Not(Box::new(expression(random, variables, depth - 1))),
        5 => Expr::All(
            (0..random.below(3))
                .map(|_| expression(random, variables, depth - 1))
                .collect(),
        ),
        6 => Expr::Any(
            (0..random.below(3))
                .map(|_| expression(random, variables, depth - 1))
                .collect(),
        ),
        _ => Expr::Implies(
            Box::new(expression(random, variables, depth - 1)),
            Box::new(expression(random, variables, depth - 1)),
        ),
    }
}

/// One well-typed random problem. Recording the seed reproduces it exactly.
pub fn generate(seed: u64) -> (Vec<Variable>, Vec<Rule>) {
    let mut random = Random::new(seed);
    let variables: Vec<Variable> = (0..(1 + random.below(5)))
        .map(|index| variable(&mut random, index))
        .collect();
    let rules = (0..random.below(6))
        .map(|index| {
            let condition = expression(&mut random, &variables, 2);
            let id = format!("r{index}");
            let template = vec![TemplatePart::Text(id.clone())];
            if random.chance(20) {
                Rule::soft(&id, template, condition)
            } else {
                Rule::hard(&id, template, condition)
            }
        })
        .collect();
    (variables, rules)
}

// ---- the oracle:plain fractions over i128 and plain enumeration ---------------------------------

#[derive(Clone, Copy)]
struct Fraction(i128, i128);

fn plain(value: Rational) -> Fraction {
    Fraction(value.numerator(), value.denominator())
}

fn add(a: Fraction, b: Fraction) -> Fraction {
    Fraction(a.0 * b.1 + b.0 * a.1, a.1 * b.1)
}

fn scale(a: Fraction, b: Fraction) -> Fraction {
    Fraction(a.0 * b.0, a.1 * b.1)
}

fn sum(linear: &Linear, names: &[String], assignment: &[Value]) -> Fraction {
    let mut total = plain(linear.constant);
    for (coefficient, name) in &linear.terms {
        let position = names
            .iter()
            .position(|candidate| candidate == name)
            .unwrap_or(usize::MAX);
        if let Value::Number(value) = &assignment[position] {
            total = add(total, scale(plain(*coefficient), plain(*value)));
        }
    }
    total
}

pub fn holds(expr: &Expr, names: &[String], assignment: &[Value]) -> bool {
    let at = |name: &String| {
        names
            .iter()
            .position(|candidate| candidate == name)
            .unwrap_or(usize::MAX)
    };
    match expr {
        Expr::Is(name, value) => assignment[at(name)] == *value,
        Expr::OneOf(name, values) => values.contains(&assignment[at(name)]),
        Expr::Same(left, right) => assignment[at(left)] == assignment[at(right)],
        Expr::Compare(op, left, right) => {
            let (a, b) = (sum(left, names, assignment), sum(right, names, assignment));
            let (x, y) = (a.0 * b.1, b.0 * a.1);
            match op {
                Comparison::Equal => x == y,
                Comparison::NotEqual => x != y,
                Comparison::Less => x < y,
                Comparison::AtMost => x <= y,
                Comparison::Greater => x > y,
                Comparison::AtLeast => x >= y,
            }
        }
        Expr::Not(inner) => !holds(inner, names, assignment),
        Expr::All(items) => items.iter().all(|item| holds(item, names, assignment)),
        Expr::Any(items) => items.iter().any(|item| holds(item, names, assignment)),
        Expr::Implies(first, second) => {
            !holds(first, names, assignment) || holds(second, names, assignment)
        }
    }
}

/// The search order a variable declares:its domain,with a preference moved to the front.
fn ordered(variable: &Variable) -> Vec<Value> {
    let mut values = values_of(&variable.domain);
    if let Binding::Preference(preferred) = &variable.binding {
        let position = values
            .iter()
            .position(|value| value == preferred)
            .unwrap_or(0);
        let value = values.remove(position);
        values.insert(0, value);
    }
    values
}

/// Every assignment in declared order, stopping at the first that `accept` takes.
pub fn first_where(
    variables: &[Variable],
    pins: &[bool],
    accept: &mut dyn FnMut(&[Value]) -> bool,
) -> Option<Vec<Value>> {
    fn walk(
        variables: &[Variable],
        pins: &[bool],
        current: &mut Vec<Value>,
        accept: &mut dyn FnMut(&[Value]) -> bool,
    ) -> bool {
        let Some(variable) = variables.get(current.len()) else {
            return accept(current);
        };
        let held = match &variable.binding {
            Binding::Known(value) | Binding::Assumed(value) if pins[current.len()] => {
                Some(value.clone())
            }
            _ => None,
        };
        let options = held.map_or_else(|| ordered(variable), |value| vec![value]);
        for option in options {
            current.push(option);
            if walk(variables, pins, current, accept) {
                return true;
            }
            current.pop();
        }
        false
    }
    let mut current = Vec::new();
    walk(variables, pins, &mut current, accept).then_some(current)
}

/// Names in declared order.
pub fn names(variables: &[Variable]) -> Vec<String> {
    variables
        .iter()
        .map(|variable| variable.name.clone())
        .collect()
}

/// The first solution under these switched-on pins and rules.
pub fn first_solution(
    variables: &[Variable],
    rules: &[&Rule],
    pins: &[bool],
) -> Option<Vec<Value>> {
    let all = names(variables);
    first_where(variables, pins, &mut |assignment| {
        rules
            .iter()
            .all(|rule| holds(&rule.condition, &all, assignment))
    })
}

/// All pins on.
pub fn all_pins(variables: &[Variable]) -> Vec<bool> {
    vec![true; variables.len()]
}

/// The hard rules.
pub fn hard_rules(rules: &[Rule]) -> Vec<&Rule> {
    rules
        .iter()
        .filter(|rule| rule.status == Status::Hard)
        .collect()
}

/// The AST version these fixtures are written for.
pub const VERSION: u32 = AST_VERSION;
