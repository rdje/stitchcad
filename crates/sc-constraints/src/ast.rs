//! The versioned, typed constraint language (roadmap §6.2). Names are resolved when a problem is built.
use crate::Rational;

/// The AST version this solver reads. A problem written for another version is refused.
pub const AST_VERSION: u32 = 1;

/// A value a parameter can take.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Value {
    /// A yes/no value.
    Bool(bool),
    /// One enumerated symbol, by its machine token.
    Symbol(String),
    /// An exact number, in the parameter's unit.
    Number(Rational),
}

/// One enumerated value: a machine token, and the label people read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Symbol {
    /// The stable machine token.
    pub token: String,
    /// The text shown to people. Tokens are never rendered raw.
    pub label: String,
}

/// A finite domain. Its order is the declared value order the search follows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Domain {
    /// Yes or no, with the value tried first and the labels people read.
    Bool {
        /// The value the search tries first.
        first: bool,
        /// The label for `true`.
        yes: String,
        /// The label for `false`.
        no: String,
    },
    /// Enumerated symbols, in declared order.
    Enum(Vec<Symbol>),
    /// Exact numbers, in declared order.
    Numbers {
        /// The values.
        values: Vec<Rational>,
        /// The unit, if the numbers have one.
        unit: Option<String>,
    },
    /// Whole numbers from `low` to `high` by `step`.
    Integers {
        /// The smallest value.
        low: i64,
        /// The largest value.
        high: i64,
        /// The distance between values, at least 1.
        step: i64,
        /// Try the largest value first instead of the smallest.
        descending: bool,
        /// The unit, if the numbers have one.
        unit: Option<String>,
    },
}

/// What is known about a parameter (roadmap §2.3 and §8).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Binding {
    /// A known fact, with evidence. The solver cannot change it.
    Known(Value),
    /// A recorded assumption. Held like a fact and reported as an assumption.
    Assumed(Value),
    /// An unknown fact. The solver never assigns it a value.
    Unknown,
    /// A selectable design choice. The solver selects in declared order.
    Choice,
    /// An overridable preference, tried first and reported if overridden.
    Preference(Value),
}

/// A parameter of the problem. Declaration order is the variable order the search follows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable {
    /// The stable name rules refer to.
    pub name: String,
    /// The profile field this parameter belongs to, for field-linked diagnostics.
    pub field: String,
    /// The text shown to people.
    pub label: String,
    /// The values it can take.
    pub domain: Domain,
    /// What is known about it.
    pub binding: Binding,
    /// Evidence references for the binding.
    pub evidence: Vec<String>,
}

impl Domain {
    /// Enumerated symbols from token and label pairs, in declared order.
    pub fn symbols(pairs: &[(&str, &str)]) -> Domain {
        Domain::Enum(
            pairs
                .iter()
                .map(|(token, label)| Symbol {
                    token: (*token).to_owned(),
                    label: (*label).to_owned(),
                })
                .collect(),
        )
    }
}

impl Variable {
    /// A parameter whose field is its name, with no evidence references yet.
    pub fn new(name: &str, label: &str, domain: Domain, binding: Binding) -> Variable {
        Variable {
            name: name.to_owned(),
            field: name.to_owned(),
            label: label.to_owned(),
            domain,
            binding,
            evidence: Vec::new(),
        }
    }
}

/// How a comparison relates its two sides.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparison {
    /// Equal.
    Equal,
    /// Not equal.
    NotEqual,
    /// Strictly less.
    Less,
    /// Less or equal.
    AtMost,
    /// Strictly greater.
    Greater,
    /// Greater or equal.
    AtLeast,
}

/// A linear sum: coefficients times parameters, plus a constant in the same unit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Linear {
    /// Each dimensionless coefficient with the parameter it scales.
    pub terms: Vec<(Rational, String)>,
    /// The constant.
    pub constant: Rational,
    /// The constant's unit. A zero constant needs none.
    pub unit: Option<String>,
}

impl Linear {
    /// One parameter on its own.
    pub fn var(name: &str) -> Linear {
        Linear {
            terms: vec![(Rational::int(1), name.to_owned())],
            constant: Rational::ZERO,
            unit: None,
        }
    }

    /// A constant on its own.
    pub fn constant(value: Rational, unit: Option<&str>) -> Linear {
        Linear {
            terms: Vec::new(),
            constant: value,
            unit: unit.map(str::to_owned),
        }
    }
}

/// A condition. Real-valued nonlinear constraints are outside the v1 fragment (roadmap §6.1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    /// The parameter has this value.
    Is(String, Value),
    /// The parameter has one of these values.
    OneOf(String, Vec<Value>),
    /// Two parameters of the same kind hold equal values.
    Same(String, String),
    /// A comparison between two linear sums with the same unit.
    Compare(Comparison, Linear, Linear),
    /// The condition does not hold.
    Not(Box<Expr>),
    /// Every condition holds. Empty is true.
    All(Vec<Expr>),
    /// At least one condition holds. Empty is false.
    Any(Vec<Expr>),
    /// If the first holds, the second holds.
    Implies(Box<Expr>, Box<Expr>),
}

/// Whether a rule may be violated.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    /// Must hold. A violation makes the candidate infeasible.
    Hard,
    /// Should hold. A violation is reported and does not make the problem infeasible.
    Soft,
}

/// What a rule applies to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Scope {
    /// The whole profile.
    Profile,
    /// One named artifact kind.
    Artifact(String),
}

/// One piece of a diagnostic template.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TemplatePart {
    /// Fixed text.
    Text(String),
    /// A parameter's label.
    Label(String),
    /// A parameter's value, in its label form.
    ValueOf(String),
}

/// One typed constraint with its identity and explanation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rule {
    /// The stable rule ID.
    pub id: String,
    /// What it applies to.
    pub scope: Scope,
    /// Hard or soft.
    pub status: Status,
    /// Where the rule came from.
    pub provenance: Vec<String>,
    /// The sentence that explains it.
    pub template: Vec<TemplatePart>,
    /// The condition.
    pub condition: Expr,
}

impl Rule {
    /// A hard rule over the whole profile.
    pub fn hard(id: &str, template: Vec<TemplatePart>, condition: Expr) -> Rule {
        Rule {
            id: id.to_owned(),
            scope: Scope::Profile,
            status: Status::Hard,
            provenance: Vec::new(),
            template,
            condition,
        }
    }

    /// A soft rule over the whole profile.
    pub fn soft(id: &str, template: Vec<TemplatePart>, condition: Expr) -> Rule {
        Rule {
            status: Status::Soft,
            ..Rule::hard(id, template, condition)
        }
    }
}
