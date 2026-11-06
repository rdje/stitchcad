//! Finite-domain propagation and search. Fixed order, no randomness, no wall-clock (roadmap §6.1).
use crate::ast::{Binding, Comparison, Status, Value};
use crate::problem::{Compiled, CompiledRule, Problem, Slot};
use crate::Rational;
use std::cmp::Ordering;
use std::collections::HashSet;

/// Resource bounds on solver work. Steps count solver operations, so a limit is deterministic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    /// The most operations one solve may spend, including explanation.
    pub max_steps: u64,
}

impl Default for Limits {
    fn default() -> Limits {
        Limits {
            max_steps: 5_000_000,
        }
    }
}

/// How a selected parameter got its value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Origin {
    /// A known fact.
    Known,
    /// A recorded assumption.
    Assumed,
    /// A design choice the solver selected.
    Selected,
    /// A preference that held.
    PreferenceHeld,
    /// A preference the rules overrode, with the preferred value.
    PreferenceOverridden(Value),
}

/// One parameter's value in a solution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Selection {
    /// The parameter.
    pub name: String,
    /// Its profile field.
    pub field: String,
    /// Its value.
    pub value: Value,
    /// How it got that value.
    pub origin: Origin,
}

/// An unknown fact left open. It is never assigned;these are the answers that would keep the problem feasible.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpenFact {
    /// The parameter.
    pub name: String,
    /// Its profile field.
    pub field: String,
    /// The answers that have a solution, in declared order.
    pub feasible: Vec<Value>,
    /// False when the work limit stopped the check, so an answer may be missing.
    pub complete: bool,
}

/// A feasible result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Solution {
    /// Every parameter except open unknown facts. Conditional on those facts while any remain.
    pub selections: Vec<Selection>,
    /// Unknown facts, which need an answer from a person.
    pub open_facts: Vec<OpenFact>,
    /// Soft rules that do not hold, by ID.
    pub violated_soft: Vec<String>,
    /// Soft rules the work limit left undecided, by ID.
    pub undecided_soft: Vec<String>,
    /// Operations spent.
    pub steps: u64,
}

/// One member of an unsat core.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CoreItem {
    /// A pinned parameter.
    Fact {
        /// The parameter.
        name: String,
        /// Its profile field.
        field: String,
        /// Its pinned value.
        value: Value,
        /// True when it is a recorded assumption, not a known fact.
        assumed: bool,
    },
    /// A hard rule.
    Rule {
        /// The rule ID.
        id: String,
        /// The profile fields it reads, in declared order.
        fields: Vec<String>,
    },
}

/// A proof that no assignment exists, with the conflicting set.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unsatisfiable {
    /// A conflicting set of facts and rules.
    pub core: Vec<CoreItem>,
    /// True when removing any one member makes the rest feasible.
    pub minimal: bool,
    /// Operations spent.
    pub steps: u64,
}

/// The result of a solve. Four outcomes, never merged (roadmap §6.2).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// A solution exists.
    Satisfied(Solution),
    /// No solution exists, and the core shows why.
    ProvenUnsatisfiable(Unsatisfiable),
    /// The work limit ended the solve before an answer. Says nothing about feasibility.
    Unknown {
        /// Operations spent.
        steps: u64,
    },
    /// An exact number did not fit, so this rule could not be decided.
    NumericallyFailed {
        /// The rule ID.
        rule: String,
        /// The profile fields it reads, in declared order.
        fields: Vec<String>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tri {
    True,
    False,
    Open,
}

/// What stopped a solve early.
#[derive(Clone, Copy, Debug)]
enum Halt {
    Limit,
    Arithmetic(usize),
}

struct Meter {
    spent: u64,
    limit: u64,
}

impl Meter {
    fn charge(&mut self, work: usize) -> Result<(), Halt> {
        self.spent = self
            .spent
            .saturating_add(u64::try_from(work).unwrap_or(u64::MAX));
        if self.spent > self.limit {
            Err(Halt::Limit)
        } else {
            Ok(())
        }
    }
}

/// Which facts and rules a look holds. The core search switches them off one at a time.
#[derive(Clone)]
struct Active {
    pins: Vec<bool>,
    rules: Vec<bool>,
}

/// For each parameter, its live value positions in declared order.
type State = Vec<Vec<usize>>;

/// Domains up to this size are pruned value by value. Larger ones are shaved from both ends.
const VALUE_PRUNE_LIMIT: usize = 512;

fn only(live: &[usize]) -> Option<usize> {
    if let [one] = live {
        Some(*one)
    } else {
        None
    }
}

fn negate(value: Tri) -> Tri {
    match value {
        Tri::True => Tri::False,
        Tri::False => Tri::True,
        Tri::Open => Tri::Open,
    }
}

fn compare(op: Comparison, low: Rational, high: Rational) -> Tri {
    let (bottom, top) = (low.sign(), high.sign());
    match op {
        Comparison::Equal => {
            if bottom == Ordering::Equal && top == Ordering::Equal {
                Tri::True
            } else if bottom == Ordering::Greater || top == Ordering::Less {
                Tri::False
            } else {
                Tri::Open
            }
        }
        Comparison::NotEqual => negate(compare(Comparison::Equal, low, high)),
        Comparison::Less => {
            if top == Ordering::Less {
                Tri::True
            } else if bottom != Ordering::Less {
                Tri::False
            } else {
                Tri::Open
            }
        }
        Comparison::AtMost => {
            if top != Ordering::Greater {
                Tri::True
            } else if bottom == Ordering::Greater {
                Tri::False
            } else {
                Tri::Open
            }
        }
        Comparison::Greater => negate(compare(Comparison::AtMost, low, high)),
        Comparison::AtLeast => negate(compare(Comparison::Less, low, high)),
    }
}

fn number(slot: &Slot, index: usize) -> Option<Rational> {
    match slot.values.get(index) {
        Some(Value::Number(value)) => Some(*value),
        _ => None,
    }
}

struct Solver<'a> {
    problem: &'a Problem,
}

impl Solver<'_> {
    fn slot(&self, position: usize) -> Option<&Slot> {
        self.problem.slots.get(position)
    }

    /// The live range of one numeric parameter. `Arithmetic` names the rule being evaluated.
    fn bounds(
        &self,
        position: usize,
        state: &State,
        rule: usize,
        meter: &mut Meter,
    ) -> Result<(Rational, Rational), Halt> {
        let fail = Halt::Arithmetic(rule);
        let slot = self.slot(position).ok_or(fail)?;
        let live = state.get(position).ok_or(fail)?;
        let mut range: Option<(Rational, Rational)> = None;
        let ends;
        let scanned: &[usize] = if slot.monotonic {
            ends = [
                live.first().copied().ok_or(fail)?,
                live.last().copied().ok_or(fail)?,
            ];
            &ends
        } else {
            meter.charge(live.len())?;
            live
        };
        for index in scanned {
            let value = number(slot, *index).ok_or(fail)?;
            range = Some(match range {
                None => (value, value),
                Some((low, high)) => (
                    if value.checked_cmp(low).ok_or(fail)? == Ordering::Less {
                        value
                    } else {
                        low
                    },
                    if value.checked_cmp(high).ok_or(fail)? == Ordering::Greater {
                        value
                    } else {
                        high
                    },
                ),
            });
        }
        range.ok_or(fail)
    }

    fn eval(
        &self,
        condition: &Compiled,
        state: &State,
        rule: usize,
        meter: &mut Meter,
    ) -> Result<Tri, Halt> {
        let fail = Halt::Arithmetic(rule);
        Ok(match condition {
            Compiled::Never => Tri::False,
            Compiled::Is(position, index) => {
                let live = state.get(*position).ok_or(fail)?;
                if live.binary_search(index).is_err() {
                    Tri::False
                } else if live.len() == 1 {
                    Tri::True
                } else {
                    Tri::Open
                }
            }
            Compiled::OneOf(position, indices) => {
                let live = state.get(*position).ok_or(fail)?;
                meter.charge(indices.len())?;
                let mut inside: Vec<usize> = indices
                    .iter()
                    .copied()
                    .filter(|index| live.binary_search(index).is_ok())
                    .collect();
                inside.sort_unstable();
                inside.dedup();
                if inside.is_empty() {
                    Tri::False
                } else if inside.len() == live.len() {
                    Tri::True
                } else {
                    Tri::Open
                }
            }
            Compiled::Same(first, second) => {
                let (left, right) = (
                    self.slot(*first).ok_or(fail)?,
                    self.slot(*second).ok_or(fail)?,
                );
                let (left_live, right_live) = (
                    state.get(*first).ok_or(fail)?,
                    state.get(*second).ok_or(fail)?,
                );
                meter.charge(left_live.len().saturating_add(right_live.len()))?;
                let others: HashSet<&Value> = right_live
                    .iter()
                    .filter_map(|index| right.values.get(*index))
                    .collect();
                let overlap = left_live
                    .iter()
                    .filter_map(|index| left.values.get(*index))
                    .any(|value| others.contains(value));
                if !overlap {
                    Tri::False
                } else if left_live.len() == 1 && right_live.len() == 1 {
                    Tri::True
                } else {
                    Tri::Open
                }
            }
            Compiled::Compare(op, terms, constant) => {
                let (mut low, mut high) = (*constant, *constant);
                for (coefficient, position) in terms {
                    let (bottom, top) = self.bounds(*position, state, rule, meter)?;
                    let first = coefficient.checked_mul(bottom).ok_or(fail)?;
                    let second = coefficient.checked_mul(top).ok_or(fail)?;
                    let (least, most) = if coefficient.sign() == Ordering::Less {
                        (second, first)
                    } else {
                        (first, second)
                    };
                    low = low.checked_add(least).ok_or(fail)?;
                    high = high.checked_add(most).ok_or(fail)?;
                }
                compare(*op, low, high)
            }
            Compiled::Not(inner) => negate(self.eval(inner, state, rule, meter)?),
            Compiled::All(items) => {
                let mut result = Tri::True;
                for item in items {
                    match self.eval(item, state, rule, meter)? {
                        Tri::False => return Ok(Tri::False),
                        Tri::Open => result = Tri::Open,
                        Tri::True => {}
                    }
                }
                result
            }
            Compiled::Any(items) => {
                let mut result = Tri::False;
                for item in items {
                    match self.eval(item, state, rule, meter)? {
                        Tri::True => return Ok(Tri::True),
                        Tri::Open => result = Tri::Open,
                        Tri::False => {}
                    }
                }
                result
            }
            Compiled::Implies(first, second) => {
                match (
                    self.eval(first, state, rule, meter)?,
                    self.eval(second, state, rule, meter)?,
                ) {
                    (Tri::False, _) | (_, Tri::True) => Tri::True,
                    (Tri::True, Tri::False) => Tri::False,
                    _ => Tri::Open,
                }
            }
        })
    }

    /// Removes values no rule can support, in declared order, until nothing changes. `false` is a conflict.
    fn propagate(
        &self,
        state: &mut State,
        active: &Active,
        meter: &mut Meter,
    ) -> Result<bool, Halt> {
        loop {
            let mut changed = false;
            for (position, rule) in self.problem.rules.iter().enumerate() {
                if !active.rules.get(position).copied().unwrap_or(false) {
                    continue;
                }
                meter.charge(1)?;
                match self.eval(&rule.condition, state, position, meter)? {
                    Tri::False => return Ok(false),
                    Tri::True => continue,
                    Tri::Open => {}
                }
                if self.prune(rule, position, state, meter)? {
                    changed = true;
                    if state.iter().any(Vec::is_empty) {
                        return Ok(false);
                    }
                }
            }
            if !changed {
                return Ok(true);
            }
        }
    }

    /// Whether the rule is false with this parameter held at this value. Leaves the parameter held there.
    fn refuted(
        &self,
        rule: &CompiledRule,
        position: usize,
        state: &mut State,
        read: usize,
        index: usize,
        meter: &mut Meter,
    ) -> Result<bool, Halt> {
        meter.charge(1)?;
        if let Some(entry) = state.get_mut(read) {
            entry.clear();
            entry.push(index);
        }
        Ok(self.eval(&rule.condition, state, position, meter)? == Tri::False)
    }

    fn prune(
        &self,
        rule: &CompiledRule,
        position: usize,
        state: &mut State,
        meter: &mut Meter,
    ) -> Result<bool, Halt> {
        let mut changed = false;
        for read in &rule.reads {
            let Some(live) = state.get(*read).cloned() else {
                continue;
            };
            if live.len() < 2 {
                continue;
            }
            let kept = if live.len() <= VALUE_PRUNE_LIMIT {
                let mut kept = Vec::with_capacity(live.len());
                for index in &live {
                    if !self.refuted(rule, position, state, *read, *index, meter)? {
                        kept.push(*index);
                    }
                }
                kept
            } else {
                let (mut low, mut high) = (0, live.len());
                while let Some(index) = live.get(low).copied().filter(|_| low < high) {
                    if !self.refuted(rule, position, state, *read, index, meter)? {
                        break;
                    }
                    low += 1;
                }
                while let Some(index) = live
                    .get(high.wrapping_sub(1))
                    .copied()
                    .filter(|_| low < high)
                {
                    if !self.refuted(rule, position, state, *read, index, meter)? {
                        break;
                    }
                    high -= 1;
                }
                live.get(low..high)
                    .map(<[usize]>::to_vec)
                    .unwrap_or_default()
            };
            let emptied = kept.is_empty();
            changed = changed || kept.len() != live.len();
            if let Some(entry) = state.get_mut(*read) {
                *entry = kept;
            }
            if emptied {
                return Ok(true);
            }
        }
        Ok(changed)
    }

    fn start(&self, active: &Active, meter: &mut Meter) -> Result<State, Halt> {
        let mut state = Vec::with_capacity(self.problem.slots.len());
        for (position, slot) in self.problem.slots.iter().enumerate() {
            let pin = slot
                .pinned
                .filter(|_| active.pins.get(position).copied().unwrap_or(false));
            state.push(match pin {
                Some(held) => vec![held],
                None => {
                    meter.charge(slot.values.len())?;
                    (0..slot.values.len()).collect()
                }
            });
        }
        Ok(state)
    }

    /// Depth-first in declared variable and value order;the first solution found is the first in that order.
    fn search(
        &self,
        mut state: State,
        active: &Active,
        meter: &mut Meter,
    ) -> Result<Option<Vec<usize>>, Halt> {
        struct Frame {
            state: State,
            variable: usize,
            next: usize,
        }
        if !self.propagate(&mut state, active, meter)? {
            return Ok(None);
        }
        let mut stack: Vec<Frame> = Vec::new();
        loop {
            let open = state.iter().position(|live| only(live).is_none());
            let Some(variable) = open else {
                return Ok(state.iter().map(|live| only(live)).collect());
            };
            stack.push(Frame {
                state: state.clone(),
                variable,
                next: 0,
            });
            loop {
                let Some(frame) = stack.last_mut() else {
                    return Ok(None);
                };
                let candidate = frame
                    .state
                    .get(frame.variable)
                    .and_then(|live| live.get(frame.next))
                    .copied();
                let Some(index) = candidate else {
                    stack.pop();
                    continue;
                };
                frame.next += 1;
                // Copying the live lists is real work, so a branch costs in proportion to them.
                meter.charge(1 + frame.state.iter().map(Vec::len).sum::<usize>() / 256)?;
                let mut child = frame.state.clone();
                if let Some(entry) = child.get_mut(frame.variable) {
                    *entry = vec![index];
                }
                if self.propagate(&mut child, active, meter)? {
                    state = child;
                    break;
                }
            }
        }
    }

    fn feasible(&self, active: &Active, meter: &mut Meter) -> Result<Option<Vec<usize>>, Halt> {
        let state = self.start(active, meter)?;
        self.search(state, active, meter)
    }

    fn fields(&self, rule: &CompiledRule) -> Vec<String> {
        let mut fields = Vec::new();
        for read in &rule.reads {
            if let Some(slot) = self.slot(*read) {
                if !fields.contains(&slot.source.field) {
                    fields.push(slot.source.field.clone());
                }
            }
        }
        fields
    }

    /// Deletion-based shrinking in declared order:facts first,then rules. Each removal is re-solved.
    fn core(&self, hard: &Active, meter: &mut Meter) -> (Vec<CoreItem>, bool) {
        let mut kept = hard.clone();
        let mut minimal = true;
        for position in 0..self.problem.slots.len() {
            if kept.pins.get(position).copied().unwrap_or(false) {
                let mut trial = kept.clone();
                if let Some(entry) = trial.pins.get_mut(position) {
                    *entry = false;
                }
                match self.feasible(&trial, meter) {
                    Ok(None) => kept = trial,
                    Ok(Some(_)) => {}
                    Err(_) => minimal = false,
                }
            }
        }
        for position in 0..self.problem.rules.len() {
            if kept.rules.get(position).copied().unwrap_or(false) {
                let mut trial = kept.clone();
                if let Some(entry) = trial.rules.get_mut(position) {
                    *entry = false;
                }
                match self.feasible(&trial, meter) {
                    Ok(None) => kept = trial,
                    Ok(Some(_)) => {}
                    Err(_) => minimal = false,
                }
            }
        }
        let mut core = Vec::new();
        for (position, slot) in self.problem.slots.iter().enumerate() {
            if kept.pins.get(position).copied().unwrap_or(false) {
                let value = slot.pinned.and_then(|held| slot.values.get(held)).cloned();
                if let Some(value) = value {
                    core.push(CoreItem::Fact {
                        name: slot.source.name.clone(),
                        field: slot.source.field.clone(),
                        value,
                        assumed: matches!(slot.source.binding, Binding::Assumed(_)),
                    });
                }
            }
        }
        for (position, rule) in self.problem.rules.iter().enumerate() {
            if kept.rules.get(position).copied().unwrap_or(false) {
                core.push(CoreItem::Rule {
                    id: rule.source.id.clone(),
                    fields: self.fields(rule),
                });
            }
        }
        (core, minimal)
    }

    fn selection(&self, position: usize, slot: &Slot, indices: &[usize]) -> Option<Selection> {
        let value = indices
            .get(position)
            .and_then(|index| slot.values.get(*index))?
            .clone();
        let origin = match &slot.source.binding {
            Binding::Known(_) => Origin::Known,
            Binding::Assumed(_) => Origin::Assumed,
            Binding::Choice => Origin::Selected,
            Binding::Preference(preferred) if *preferred == value => Origin::PreferenceHeld,
            Binding::Preference(preferred) => Origin::PreferenceOverridden(preferred.clone()),
            Binding::Unknown => return None,
        };
        Some(Selection {
            name: slot.source.name.clone(),
            field: slot.source.field.clone(),
            value,
            origin,
        })
    }

    /// Each answer to an unknown fact is tried with every design choice still free. No answer is adopted.
    fn open_fact(
        &self,
        position: usize,
        slot: &Slot,
        hard: &Active,
        meter: &mut Meter,
    ) -> OpenFact {
        let mut feasible = Vec::new();
        let mut complete = true;
        for (index, value) in slot.values.iter().enumerate() {
            let attempt = self.start(hard, meter).and_then(|mut state| {
                if let Some(live) = state.get_mut(position) {
                    *live = vec![index];
                }
                self.search(state, hard, meter)
            });
            match attempt {
                Ok(Some(_)) => feasible.push(value.clone()),
                Ok(None) => {}
                Err(_) => complete = false,
            }
        }
        OpenFact {
            name: slot.source.name.clone(),
            field: slot.source.field.clone(),
            feasible,
            complete,
        }
    }

    fn outcome(&self, limits: Limits) -> Outcome {
        let mut meter = Meter {
            spent: 0,
            limit: limits.max_steps,
        };
        let rules = &self.problem.rules;
        let hard = Active {
            pins: self
                .problem
                .slots
                .iter()
                .map(|slot| slot.pinned.is_some())
                .collect(),
            rules: rules
                .iter()
                .map(|rule| rule.source.status == Status::Hard)
                .collect(),
        };
        let halted = |halt: Halt, spent: u64| -> Outcome {
            match halt {
                Halt::Limit => Outcome::Unknown { steps: spent },
                Halt::Arithmetic(position) => {
                    let rule = rules.get(position);
                    Outcome::NumericallyFailed {
                        rule: rule.map(|rule| rule.source.id.clone()).unwrap_or_default(),
                        fields: rule.map(|rule| self.fields(rule)).unwrap_or_default(),
                    }
                }
            }
        };
        let mut best = match self.feasible(&hard, &mut meter) {
            Ok(Some(indices)) => indices,
            Ok(None) => {
                let (core, minimal) = self.core(&hard, &mut meter);
                return Outcome::ProvenUnsatisfiable(Unsatisfiable {
                    core,
                    minimal,
                    steps: meter.spent,
                });
            }
            Err(halt) => return halted(halt, meter.spent),
        };
        // Soft rules in declared order:each is kept if the rules kept so far still have a solution.
        let mut kept = hard.clone();
        let (mut violated_soft, mut undecided_soft) = (Vec::new(), Vec::new());
        for (position, rule) in rules.iter().enumerate() {
            if rule.source.status != Status::Soft {
                continue;
            }
            let mut trial = kept.clone();
            if let Some(entry) = trial.rules.get_mut(position) {
                *entry = true;
            }
            match self.feasible(&trial, &mut meter) {
                Ok(Some(indices)) => {
                    best = indices;
                    kept = trial;
                }
                Ok(None) => violated_soft.push(rule.source.id.clone()),
                Err(_) => undecided_soft.push(rule.source.id.clone()),
            }
        }
        let selections = self
            .problem
            .slots
            .iter()
            .enumerate()
            .filter_map(|(position, slot)| self.selection(position, slot, &best))
            .collect();
        let open_facts = self
            .problem
            .slots
            .iter()
            .enumerate()
            .filter(|(_, slot)| matches!(slot.source.binding, Binding::Unknown))
            .map(|(position, slot)| self.open_fact(position, slot, &hard, &mut meter))
            .collect();
        Outcome::Satisfied(Solution {
            selections,
            open_facts,
            violated_soft,
            undecided_soft,
            steps: meter.spent,
        })
    }
}

impl Problem {
    /// Resolves the problem. The same problem and limits always give the same outcome, on every target.
    pub fn solve(&self, limits: Limits) -> Outcome {
        Solver { problem: self }.outcome(limits)
    }
}
