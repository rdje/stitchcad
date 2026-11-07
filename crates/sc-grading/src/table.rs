//! A table of grade rules: a point, what it does at each size, and what the table as a whole promises. Exact integers, never rounded twice.
use core::fmt;
use sc_store::Value;
use std::collections::{BTreeMap, BTreeSet};

/// How a rule's deltas are measured.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Convention {
    /// Each delta is from the sample size.
    Cumulative,
    /// Each delta is from the next size toward the sample.
    Incremental,
}

impl Convention {
    /// Its name.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Cumulative => "cumulative",
            Self::Incremental => "incremental",
        }
    }

    /// The convention of that name.
    ///
    /// # Errors
    ///
    /// No convention has that name.
    pub fn named(name: &str) -> Result<Self, GradeError> {
        match name {
            "cumulative" => Ok(Self::Cumulative),
            "incremental" => Ok(Self::Incremental),
            other => Err(GradeError::new(format!(
                "no convention is named {other}: a table is cumulative or incremental"
            ))),
        }
    }
}

/// Where a table came from, because a table worked out from two drawings and one from a factory are different evidence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Provenance {
    /// Worked out here, by regenerating each size.
    Regenerations,
    /// Received from somewhere else.
    Received,
}

impl Provenance {
    /// Its name.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Regenerations => "regenerations",
            Self::Received => "received",
        }
    }
}

/// What a rule of nothing but zeros means.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZeroState {
    /// This table says the point does not move.
    Declared,
    /// A file said zero, and a file cannot say whether that was meant.
    Unknown,
}

/// One point's rule.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rule {
    /// The point, by a name that survives an edit: its piece, and what on that piece it is.
    pub point: String,
    /// The number a file calls it by, which holds only within one export.
    pub id: i64,
    /// What it does at each size of the table, in order, in micrometres.
    pub deltas: Vec<(i64, i64)>,
    /// What a rule of all zeros means. Unused otherwise.
    pub zero: ZeroState,
}

/// What a table promises beyond its deltas.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Attributes {
    /// The points that do not move, by declaration, so that graded pieces stack aligned. A table names at least one.
    pub stack: Vec<String>,
    /// The contours whose length must not change across sizes, each as named by the draft.
    pub fixed: Vec<Value>,
    /// The contours to fair. No fairing exists, so a table that asks for it is refused.
    pub smoothing: Vec<String>,
}

/// Why a table cannot be what it says it is.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GradeError {
    /// What is wrong.
    pub message: String,
}

impl GradeError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for GradeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for GradeError {}

/// A table of grade rules.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GradeTable {
    name: String,
    sample: String,
    sizes: Vec<String>,
    convention: Convention,
    provenance: Provenance,
    rules: Vec<Rule>,
    attributes: Attributes,
}

fn text_of(value: &Value) -> Option<String> {
    value.as_text().map(str::to_owned)
}

fn sum_pair(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (a.0 + b.0, a.1 + b.1)
}

impl GradeTable {
    /// A table, checked.
    ///
    /// # Errors
    ///
    /// Its sizes do not hang together, a rule does not fit them, it names no stack point, a stack point moves, or it asks for smoothing.
    pub fn new(
        name: String,
        (sample, sizes): (String, Vec<String>),
        convention: Convention,
        provenance: Provenance,
        mut rules: Vec<Rule>,
        attributes: Attributes,
    ) -> Result<Self, GradeError> {
        if !attributes.smoothing.is_empty() {
            return Err(GradeError::new(format!(
                "this table asks to smooth {}, and no fairing exists, so it is refused rather than ignored",
                attributes.smoothing.join(", ")
            )));
        }
        let unique: BTreeSet<&String> = sizes.iter().collect();
        if unique.len() != sizes.len() {
            return Err(GradeError::new("a table lists a size twice"));
        }
        if !sizes.contains(&sample) {
            return Err(GradeError::new(format!(
                "the sample size {sample} is not in the table's sizes"
            )));
        }
        rules.sort_by(|a, b| a.point.cmp(&b.point));
        let mut seen_ids = BTreeSet::new();
        for pair in rules.windows(2) {
            if let [a, b] = pair {
                if a.point == b.point {
                    return Err(GradeError::new(format!(
                        "two rules are for the point {}",
                        a.point
                    )));
                }
            }
        }
        let sample_at = sizes.iter().position(|size| *size == sample).unwrap_or(0);
        for rule in &rules {
            if !seen_ids.insert(rule.id) {
                return Err(GradeError::new(format!(
                    "two rules have the number {}",
                    rule.id
                )));
            }
            if rule.deltas.len() != sizes.len() {
                return Err(GradeError::new(format!(
                    "the rule for {} has {} deltas, and the table has {} sizes",
                    rule.point,
                    rule.deltas.len(),
                    sizes.len()
                )));
            }
            if rule.deltas.get(sample_at) != Some(&(0, 0)) {
                return Err(GradeError::new(format!(
                    "the rule for {} moves the sample size",
                    rule.point
                )));
            }
        }
        let table = Self {
            name,
            sample,
            sizes,
            convention,
            provenance,
            rules,
            attributes,
        };
        if table.attributes.stack.is_empty() {
            return Err(GradeError::new(
                "a table names at least one stack point, or its graded pieces drift apart",
            ));
        }
        for point in &table.attributes.stack {
            let rule = table
                .rule(point)
                .ok_or_else(|| GradeError::new(format!("the stack point {point} has no rule")))?;
            if rule.deltas.iter().any(|delta| *delta != (0, 0)) {
                return Err(GradeError::new(format!(
                    "the stack point {point} moves, and a stack point does not"
                )));
            }
        }
        Ok(table)
    }

    /// The table's name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The sample size.
    #[must_use]
    pub fn sample(&self) -> &str {
        &self.sample
    }

    /// Every size, in order.
    #[must_use]
    pub fn sizes(&self) -> &[String] {
        &self.sizes
    }

    /// How its deltas are measured.
    #[must_use]
    pub const fn convention(&self) -> Convention {
        self.convention
    }

    /// Where it came from.
    #[must_use]
    pub const fn provenance(&self) -> Provenance {
        self.provenance
    }

    /// Its rules, by point.
    #[must_use]
    pub fn rules(&self) -> &[Rule] {
        &self.rules
    }

    /// What it promises beyond its deltas.
    #[must_use]
    pub const fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    /// The rule for a point, when it mentions one.
    #[must_use]
    pub fn rule(&self, point: &str) -> Option<&Rule> {
        self.rules
            .binary_search_by(|rule| rule.point.as_str().cmp(point))
            .ok()
            .and_then(|at| self.rules.get(at))
    }

    /// Where a point is at a size, from the sample, whichever convention the table holds.
    ///
    /// # Errors
    ///
    /// The table has no such size.
    pub fn delta_at(&self, rule: &Rule, size: &str) -> Result<(i64, i64), GradeError> {
        let at = self
            .sizes
            .iter()
            .position(|name| name == size)
            .ok_or_else(|| GradeError::new(format!("the table has no size {size}")))?;
        let sample_at = self.sample_at();
        match self.convention {
            Convention::Cumulative => Ok(rule.deltas.get(at).copied().unwrap_or((0, 0))),
            Convention::Incremental => {
                let steps = if at >= sample_at {
                    rule.deltas.get(sample_at + 1..=at)
                } else {
                    rule.deltas.get(at..sample_at)
                };
                Ok(steps.unwrap_or(&[]).iter().copied().fold((0, 0), sum_pair))
            }
        }
    }

    fn sample_at(&self) -> usize {
        self.sizes
            .iter()
            .position(|name| *name == self.sample)
            .unwrap_or(0)
    }

    /// The same table in the other convention. Nothing is lost, because a delta is a whole number of micrometres either way.
    ///
    /// # Errors
    ///
    /// This table's own sizes do not hold, which [`GradeTable::new`] refused already.
    pub fn in_convention(&self, convention: Convention) -> Result<Self, GradeError> {
        if convention == self.convention {
            return Ok(self.clone());
        }
        let sample_at = self.sample_at();
        let mut rules = Vec::with_capacity(self.rules.len());
        for rule in &self.rules {
            let mut cumulative = Vec::with_capacity(self.sizes.len());
            for size in &self.sizes {
                cumulative.push(self.delta_at(rule, size)?);
            }
            let deltas = match convention {
                Convention::Cumulative => cumulative,
                Convention::Incremental => {
                    let mut steps = vec![(0, 0); cumulative.len()];
                    for (at, here) in cumulative.iter().enumerate() {
                        let toward_sample = match at.cmp(&sample_at) {
                            core::cmp::Ordering::Greater => cumulative.get(at - 1),
                            core::cmp::Ordering::Less => cumulative.get(at + 1),
                            core::cmp::Ordering::Equal => None,
                        };
                        if let (Some(before), Some(step)) = (toward_sample, steps.get_mut(at)) {
                            *step = (here.0 - before.0, here.1 - before.1);
                        }
                    }
                    steps
                }
            };
            rules.push(Rule {
                point: rule.point.clone(),
                id: rule.id,
                deltas,
                zero: rule.zero,
            });
        }
        Self::new(
            self.name.clone(),
            (self.sample.clone(), self.sizes.clone()),
            convention,
            self.provenance,
            rules,
            self.attributes.clone(),
        )
    }

    /// The table in canonical form.
    #[must_use]
    pub fn to_value(&self) -> Value {
        let texts = |list: &[String]| Value::List(list.iter().cloned().map(Value::Text).collect());
        let rules = self
            .rules
            .iter()
            .map(|rule| {
                let mut fields = BTreeMap::new();
                fields.insert("point".to_owned(), Value::Text(rule.point.clone()));
                fields.insert("id".to_owned(), Value::Int(rule.id));
                fields.insert(
                    "deltas".to_owned(),
                    Value::List(
                        rule.deltas
                            .iter()
                            .map(|(x, y)| Value::List(vec![Value::Int(*x), Value::Int(*y)]))
                            .collect(),
                    ),
                );
                if rule.deltas.iter().all(|delta| *delta == (0, 0)) {
                    fields.insert(
                        "zero".to_owned(),
                        Value::Text(
                            match rule.zero {
                                ZeroState::Declared => "declared",
                                ZeroState::Unknown => "unknown",
                            }
                            .to_owned(),
                        ),
                    );
                }
                Value::Object(fields)
            })
            .collect();
        let mut fields = BTreeMap::new();
        fields.insert("format".to_owned(), Value::Int(1));
        fields.insert("name".to_owned(), Value::Text(self.name.clone()));
        fields.insert("sample".to_owned(), Value::Text(self.sample.clone()));
        fields.insert("sizes".to_owned(), texts(&self.sizes));
        fields.insert(
            "convention".to_owned(),
            Value::Text(self.convention.token().to_owned()),
        );
        fields.insert(
            "provenance".to_owned(),
            Value::Text(self.provenance.token().to_owned()),
        );
        fields.insert("stack".to_owned(), texts(&self.attributes.stack));
        fields.insert(
            "fixed".to_owned(),
            Value::List(self.attributes.fixed.clone()),
        );
        fields.insert("smoothing".to_owned(), texts(&self.attributes.smoothing));
        fields.insert("rules".to_owned(), Value::List(rules));
        Value::Object(fields)
    }

    /// A table from its canonical form.
    ///
    /// # Errors
    ///
    /// It is not a table, or it is one that [`GradeTable::new`] refuses.
    pub fn from_value(value: &Value) -> Result<Self, GradeError> {
        let bad = |what: &str| GradeError::new(format!("a table's {what} is missing or malformed"));
        let fields = value.as_object().ok_or_else(|| bad("form"))?;
        let text = |name: &str| {
            fields
                .get(name)
                .and_then(Value::as_text)
                .map(str::to_owned)
                .ok_or_else(|| bad(name))
        };
        let texts = |name: &str| -> Result<Vec<String>, GradeError> {
            fields
                .get(name)
                .and_then(Value::as_list)
                .ok_or_else(|| bad(name))?
                .iter()
                .map(|item| text_of(item).ok_or_else(|| bad(name)))
                .collect()
        };
        let convention = Convention::named(&text("convention")?)?;
        let provenance = match text("provenance")?.as_str() {
            "regenerations" => Provenance::Regenerations,
            "received" => Provenance::Received,
            other => {
                return Err(GradeError::new(format!(
                    "no table comes from {other}: it is from regenerations, or received"
                )))
            }
        };
        let mut rules = Vec::new();
        for item in fields
            .get("rules")
            .and_then(Value::as_list)
            .ok_or_else(|| bad("rules"))?
        {
            let item = item.as_object().ok_or_else(|| bad("rules"))?;
            let mut deltas = Vec::new();
            for pair in item
                .get("deltas")
                .and_then(Value::as_list)
                .ok_or_else(|| bad("deltas"))?
            {
                let Some([x, y]) = pair.as_list() else {
                    return Err(bad("deltas"));
                };
                let (Some(x), Some(y)) = (x.as_int(), y.as_int()) else {
                    return Err(bad("deltas"));
                };
                deltas.push((x, y));
            }
            rules.push(Rule {
                point: item
                    .get("point")
                    .and_then(text_of)
                    .ok_or_else(|| bad("rules"))?,
                id: item
                    .get("id")
                    .and_then(Value::as_int)
                    .ok_or_else(|| bad("rules"))?,
                deltas,
                zero: match item.get("zero").and_then(Value::as_text) {
                    Some("unknown") => ZeroState::Unknown,
                    _ => ZeroState::Declared,
                },
            });
        }
        Self::new(
            text("name")?,
            (text("sample")?, texts("sizes")?),
            convention,
            provenance,
            rules,
            Attributes {
                stack: texts("stack")?,
                fixed: fields
                    .get("fixed")
                    .and_then(Value::as_list)
                    .map(<[Value]>::to_vec)
                    .unwrap_or_default(),
                smoothing: texts("smoothing")?,
            },
        )
    }
}
