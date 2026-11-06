//! Validation: a problem that builds has no unresolved name, no type or unit mismatch and no bad domain.
use crate::ast::{
    Binding, Comparison, Domain, Expr, Linear, Rule, Status, TemplatePart, Value, Variable,
    AST_VERSION,
};
use crate::Rational;
use std::collections::{HashMap, HashSet};
use std::fmt;

/// The largest domain one parameter may declare. The supported fragment is bounded (roadmap §2.6).
pub const MAX_DOMAIN_VALUES: u64 = 65_536;

/// Why a problem cannot be built. Each one names what to fix.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProblemError {
    /// The AST is another version.
    UnsupportedVersion(u32),
    /// A name, field or rule ID is blank.
    BlankName,
    /// Two parameters share a name.
    DuplicateVariable(String),
    /// Two rules share an ID.
    DuplicateRule(String),
    /// A domain has no values.
    EmptyDomain(String),
    /// A domain repeats a value.
    DuplicateValue(String),
    /// A numeric range is backwards, or its step is not positive.
    BadRange(String),
    /// A domain is larger than the supported bound.
    DomainTooLarge(String),
    /// A binding's value is not in the parameter's domain.
    ValueOutsideDomain(String),
    /// A rule names a parameter that does not exist.
    UnresolvedName {
        /// The rule.
        rule: String,
        /// The missing parameter.
        name: String,
    },
    /// A rule uses a value or parameter of the wrong kind.
    TypeMismatch {
        /// The rule.
        rule: String,
        /// The parameter.
        name: String,
    },
    /// A rule mixes units.
    UnitMismatch {
        /// The rule.
        rule: String,
    },
    /// A rule has no explanation.
    MissingTemplate(String),
    /// A rule's constants are too large to combine exactly.
    NumberTooLarge(String),
}

impl fmt::Display for ProblemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedVersion(found) => {
                write!(f, "constraint AST version {found} is not supported; this solver reads version {AST_VERSION}")
            }
            Self::BlankName => write!(f, "a name, field or rule ID is blank"),
            Self::DuplicateVariable(name) => write!(f, "parameter {name} is declared twice"),
            Self::DuplicateRule(id) => write!(f, "rule {id} is declared twice"),
            Self::EmptyDomain(name) => write!(f, "parameter {name} has no values"),
            Self::DuplicateValue(name) => write!(f, "parameter {name} repeats a value"),
            Self::BadRange(name) => write!(
                f,
                "parameter {name} has a backwards range or a step below 1"
            ),
            Self::DomainTooLarge(name) => {
                write!(
                    f,
                    "parameter {name} has more than {MAX_DOMAIN_VALUES} values"
                )
            }
            Self::ValueOutsideDomain(name) => {
                write!(
                    f,
                    "the value given for parameter {name} is not in its domain"
                )
            }
            Self::UnresolvedName { rule, name } => {
                write!(
                    f,
                    "rule {rule} names parameter {name}, which does not exist"
                )
            }
            Self::TypeMismatch { rule, name } => {
                write!(
                    f,
                    "rule {rule} uses parameter {name} with the wrong kind of value"
                )
            }
            Self::UnitMismatch { rule } => write!(f, "rule {rule} compares different units"),
            Self::MissingTemplate(id) => write!(f, "rule {id} has no explanation"),
            Self::NumberTooLarge(id) => {
                write!(f, "rule {id} has numbers too large to combine exactly")
            }
        }
    }
}

impl std::error::Error for ProblemError {}

/// A condition with its names resolved to positions.
#[derive(Clone, Debug)]
pub(crate) enum Compiled {
    Is(usize, usize),
    OneOf(usize, Vec<usize>),
    Same(usize, usize),
    Compare(Comparison, Vec<(Rational, usize)>, Rational),
    /// A reference to a value absent from the domain: never true.
    Never,
    Not(Box<Compiled>),
    All(Vec<Compiled>),
    Any(Vec<Compiled>),
    Implies(Box<Compiled>, Box<Compiled>),
}

/// A parameter with its domain expanded in search order.
#[derive(Clone, Debug)]
pub(crate) struct Slot {
    pub(crate) source: Variable,
    pub(crate) values: Vec<Value>,
    /// The position every look starts from, for a pinned parameter.
    pub(crate) pinned: Option<usize>,
    pub(crate) unit: Option<String>,
    /// The live values stay in numeric order, so the ends are the bounds.
    pub(crate) monotonic: bool,
}

/// A rule with its condition compiled and the parameters it reads, in declared order.
#[derive(Clone, Debug)]
pub(crate) struct CompiledRule {
    pub(crate) source: Rule,
    pub(crate) condition: Compiled,
    pub(crate) reads: Vec<usize>,
}

/// A validated problem, ready to solve.
#[derive(Clone, Debug)]
pub struct Problem {
    pub(crate) slots: Vec<Slot>,
    pub(crate) rules: Vec<CompiledRule>,
}

fn expand(variable: &Variable) -> Result<(Vec<Value>, Option<String>), ProblemError> {
    let name = || variable.name.clone();
    let (values, unit) = match &variable.domain {
        Domain::Bool { first, .. } => (vec![Value::Bool(*first), Value::Bool(!*first)], None),
        Domain::Enum(symbols) => (
            symbols
                .iter()
                .map(|symbol| Value::Symbol(symbol.token.clone()))
                .collect(),
            None,
        ),
        Domain::Numbers { values, unit } => (
            values.iter().map(|value| Value::Number(*value)).collect(),
            unit.clone(),
        ),
        Domain::Integers {
            low,
            high,
            step,
            descending,
            unit,
        } => {
            if low > high || *step < 1 {
                return Err(ProblemError::BadRange(name()));
            }
            let span = i128::from(*high) - i128::from(*low);
            let count = span / i128::from(*step) + 1;
            if count > i128::from(MAX_DOMAIN_VALUES) {
                return Err(ProblemError::DomainTooLarge(name()));
            }
            let mut values = Vec::new();
            let mut current = i128::from(*low);
            while current <= i128::from(*high) {
                let whole = i64::try_from(current).map_err(|_| ProblemError::BadRange(name()))?;
                values.push(Value::Number(Rational::int(whole)));
                current += i128::from(*step);
            }
            if *descending {
                values.reverse();
            }
            (values, unit.clone())
        }
    };
    if values.is_empty() {
        return Err(ProblemError::EmptyDomain(name()));
    }
    if u64::try_from(values.len()).map_or(true, |count| count > MAX_DOMAIN_VALUES) {
        return Err(ProblemError::DomainTooLarge(name()));
    }
    let mut seen = HashSet::with_capacity(values.len());
    if !values.iter().all(|value| seen.insert(value)) {
        return Err(ProblemError::DuplicateValue(name()));
    }
    Ok((values, unit))
}

struct Names<'a> {
    positions: &'a HashMap<String, usize>,
    slots: &'a [Slot],
    rule: &'a str,
}

impl Names<'_> {
    fn slot(&self, name: &str) -> Result<(usize, &Slot), ProblemError> {
        let position =
            self.positions
                .get(name)
                .copied()
                .ok_or_else(|| ProblemError::UnresolvedName {
                    rule: self.rule.to_owned(),
                    name: name.to_owned(),
                })?;
        let slot = self
            .slots
            .get(position)
            .ok_or_else(|| ProblemError::UnresolvedName {
                rule: self.rule.to_owned(),
                name: name.to_owned(),
            })?;
        Ok((position, slot))
    }

    fn mismatch(&self, name: &str) -> ProblemError {
        ProblemError::TypeMismatch {
            rule: self.rule.to_owned(),
            name: name.to_owned(),
        }
    }

    /// A value of the right kind that the domain does not hold can never be true,
    /// so it compiles to `None`; a value of the wrong kind is a type error.
    fn value(&self, slot: &Slot, name: &str, value: &Value) -> Result<Option<usize>, ProblemError> {
        let kind =
            |candidate: &Value| std::mem::discriminant(candidate) == std::mem::discriminant(value);
        if !slot.values.iter().any(kind) {
            return Err(self.mismatch(name));
        }
        if let (Value::Symbol(_), Domain::Enum(_)) = (value, &slot.source.domain) {
            // A symbol outside an enum is an unresolved identifier, not a false fact.
            return match slot.values.iter().position(|candidate| candidate == value) {
                Some(position) => Ok(Some(position)),
                None => Err(self.mismatch(name)),
            };
        }
        Ok(slot.values.iter().position(|candidate| candidate == value))
    }

    fn linear(
        &self,
        linear: &Linear,
        sign: Rational,
        terms: &mut Vec<(Rational, usize)>,
        unit: &mut Option<String>,
    ) -> Result<Rational, ProblemError> {
        let overflow = || ProblemError::NumberTooLarge(self.rule.to_owned());
        for (coefficient, name) in &linear.terms {
            let (position, slot) = self.slot(name)?;
            if !matches!(slot.values.first(), Some(Value::Number(_))) {
                return Err(self.mismatch(name));
            }
            self.same_unit(unit, slot.unit.as_deref())?;
            terms.push((
                coefficient.checked_mul(sign).ok_or_else(overflow)?,
                position,
            ));
        }
        if linear.constant != Rational::ZERO {
            self.same_unit(unit, linear.unit.as_deref())?;
        }
        linear.constant.checked_mul(sign).ok_or_else(overflow)
    }

    fn same_unit(&self, seen: &mut Option<String>, unit: Option<&str>) -> Result<(), ProblemError> {
        // The first side fixes the unit, including "no unit": `Some("")` records a dimensionless side.
        let tag = unit.unwrap_or("");
        match seen {
            Some(existing) if existing != tag => Err(ProblemError::UnitMismatch {
                rule: self.rule.to_owned(),
            }),
            Some(_) => Ok(()),
            None => {
                *seen = Some(tag.to_owned());
                Ok(())
            }
        }
    }

    fn compile(&self, expr: &Expr) -> Result<Compiled, ProblemError> {
        Ok(match expr {
            Expr::Is(name, value) => {
                let (position, slot) = self.slot(name)?;
                match self.value(slot, name, value)? {
                    Some(index) => Compiled::Is(position, index),
                    None => Compiled::Never,
                }
            }
            Expr::OneOf(name, values) => {
                let (position, slot) = self.slot(name)?;
                let mut indices = Vec::new();
                for value in values {
                    if let Some(index) = self.value(slot, name, value)? {
                        indices.push(index);
                    }
                }
                Compiled::OneOf(position, indices)
            }
            Expr::Same(left, right) => {
                let (first, left_slot) = self.slot(left)?;
                let (second, right_slot) = self.slot(right)?;
                let kinds = (left_slot.values.first(), right_slot.values.first());
                let comparable = match kinds {
                    (Some(a), Some(b)) => std::mem::discriminant(a) == std::mem::discriminant(b),
                    _ => false,
                };
                if !comparable {
                    return Err(self.mismatch(right));
                }
                if left_slot.unit != right_slot.unit {
                    return Err(ProblemError::UnitMismatch {
                        rule: self.rule.to_owned(),
                    });
                }
                Compiled::Same(first, second)
            }
            Expr::Compare(op, left, right) => {
                let mut terms = Vec::new();
                let mut unit = None;
                let positive = self.linear(left, Rational::int(1), &mut terms, &mut unit)?;
                let negative = self.linear(right, Rational::int(-1), &mut terms, &mut unit)?;
                let constant = positive
                    .checked_add(negative)
                    .ok_or_else(|| ProblemError::NumberTooLarge(self.rule.to_owned()))?;
                Compiled::Compare(*op, terms, constant)
            }
            Expr::Not(inner) => Compiled::Not(Box::new(self.compile(inner)?)),
            Expr::All(items) => Compiled::All(
                items
                    .iter()
                    .map(|item| self.compile(item))
                    .collect::<Result<_, _>>()?,
            ),
            Expr::Any(items) => Compiled::Any(
                items
                    .iter()
                    .map(|item| self.compile(item))
                    .collect::<Result<_, _>>()?,
            ),
            Expr::Implies(first, second) => Compiled::Implies(
                Box::new(self.compile(first)?),
                Box::new(self.compile(second)?),
            ),
        })
    }
}

fn collect_reads(condition: &Compiled, reads: &mut Vec<usize>) {
    match condition {
        Compiled::Is(position, _) | Compiled::OneOf(position, _) => reads.push(*position),
        Compiled::Same(first, second) => {
            reads.push(*first);
            reads.push(*second);
        }
        Compiled::Compare(_, terms, _) => reads.extend(terms.iter().map(|(_, position)| *position)),
        Compiled::Never => {}
        Compiled::Not(inner) => collect_reads(inner, reads),
        Compiled::All(items) | Compiled::Any(items) => {
            for item in items {
                collect_reads(item, reads);
            }
        }
        Compiled::Implies(first, second) => {
            collect_reads(first, reads);
            collect_reads(second, reads);
        }
    }
}

impl Problem {
    /// Validates a problem. Declaration order is the search order, so it is part of the problem's meaning.
    pub fn new(
        version: u32,
        variables: Vec<Variable>,
        rules: Vec<Rule>,
    ) -> Result<Problem, ProblemError> {
        if version != AST_VERSION {
            return Err(ProblemError::UnsupportedVersion(version));
        }
        let mut positions = HashMap::new();
        let mut slots = Vec::with_capacity(variables.len());
        for variable in variables {
            if variable.name.trim().is_empty() || variable.field.trim().is_empty() {
                return Err(ProblemError::BlankName);
            }
            if positions
                .insert(variable.name.clone(), slots.len())
                .is_some()
            {
                return Err(ProblemError::DuplicateVariable(variable.name));
            }
            let (mut values, unit) = expand(&variable)?;
            let held = match &variable.binding {
                Binding::Known(value) | Binding::Assumed(value) | Binding::Preference(value) => {
                    Some(value)
                }
                Binding::Unknown | Binding::Choice => None,
            };
            let mut pinned = None;
            if let Some(value) = held {
                let position = values
                    .iter()
                    .position(|candidate| candidate == value)
                    .ok_or_else(|| ProblemError::ValueOutsideDomain(variable.name.clone()))?;
                if matches!(variable.binding, Binding::Preference(_)) {
                    // The preference is tried first;the rest keep their declared order.
                    let preferred = values.remove(position);
                    values.insert(0, preferred);
                } else {
                    pinned = Some(position);
                }
            }
            let monotonic = matches!(variable.domain, Domain::Integers { .. })
                && !matches!(variable.binding, Binding::Preference(_));
            slots.push(Slot {
                source: variable,
                values,
                pinned,
                unit,
                monotonic,
            });
        }
        let mut seen_rules = HashMap::new();
        let mut compiled = Vec::with_capacity(rules.len());
        for rule in rules {
            if rule.id.trim().is_empty() {
                return Err(ProblemError::BlankName);
            }
            if seen_rules.insert(rule.id.clone(), ()).is_some() {
                return Err(ProblemError::DuplicateRule(rule.id));
            }
            if rule.template.is_empty() {
                return Err(ProblemError::MissingTemplate(rule.id));
            }
            let names = Names {
                positions: &positions,
                slots: &slots,
                rule: &rule.id,
            };
            for part in &rule.template {
                if let TemplatePart::Label(name) | TemplatePart::ValueOf(name) = part {
                    names.slot(name)?;
                }
            }
            let condition = names.compile(&rule.condition)?;
            let mut reads = Vec::new();
            collect_reads(&condition, &mut reads);
            reads.sort_unstable();
            reads.dedup();
            compiled.push(CompiledRule {
                source: rule,
                condition,
                reads,
            });
        }
        Ok(Problem {
            slots,
            rules: compiled,
        })
    }

    /// Whether the problem has any soft rule.
    pub fn has_soft_rules(&self) -> bool {
        self.rules
            .iter()
            .any(|rule| rule.source.status == Status::Soft)
    }
}
