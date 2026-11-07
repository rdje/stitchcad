//! The grade rule table file. Plain text: a header, then a rule per grade rule identifier, with one X and Y per size, measured from the sample size. Metric files are in millimetres to two places.
use crate::table::{Convention, GradeError, GradeTable, ZeroState};
use core::fmt;
use sc_store::Value;
use std::collections::BTreeMap;

const QUANTUM: i64 = 10;

/// A written file, with what it could not carry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RulArtifact {
    /// The file.
    pub text: String,
    /// What was lost in writing it, as canonical text, because a file should say.
    pub record: String,
}

fn hundredths(micrometres: i64) -> i64 {
    let half = QUANTUM / 2;
    if micrometres >= 0 {
        (micrometres + half) / QUANTUM
    } else {
        (micrometres - half) / QUANTUM
    }
}

fn millimetres(micrometres: i64) -> String {
    let value = hundredths(micrometres);
    let sign = if value < 0 { "-" } else { "" };
    let whole = value.abs();
    format!("{sign}{}.{:02}", whole / 100, whole % 100)
}

/// Writes a table as a `.rul` file. Always cumulative, because that is what the file's deltas are.
///
/// # Errors
///
/// A size's name has a space in it, which the size list cannot carry.
pub fn to_rul(table: &GradeTable) -> Result<RulArtifact, GradeError> {
    let table = table.in_convention(Convention::Cumulative)?;
    if let Some(size) = table
        .sizes()
        .iter()
        .find(|size| size.chars().any(char::is_whitespace) || size.is_empty())
    {
        return Err(GradeError::new(format!(
            "the size {size:?} cannot go in a size list, which separates by spaces"
        )));
    }
    let mut out = String::new();
    out.push_str("ASTM/D13 Proposal 1 Version: D 6673-10\n");
    out.push_str("AUTHOR: STITCHCAD\n");
    out.push_str("PRODUCT: STITCHCAD\n");
    out.push_str("UNITS: METRIC\n");
    out.push_str(&format!("GRADE RULE TABLE: {}\n", table.name()));
    out.push_str(&format!("NUMBER OF SIZES: {}\n", table.sizes().len()));
    out.push_str(&format!("SIZE LIST: {}\n", table.sizes().join(" ")));
    out.push_str(&format!("SAMPLE SIZE: {}\n", table.sample()));
    let mut rules: Vec<_> = table.rules().iter().collect();
    rules.sort_by_key(|rule| rule.id);
    let mut largest_rounding = 0;
    for rule in rules {
        out.push_str(&format!("RULE: DELTA {}\n", rule.id));
        for line in rule.deltas.chunks(4) {
            let pairs: Vec<String> = line
                .iter()
                .map(|(x, y)| {
                    for value in [x, y] {
                        largest_rounding =
                            largest_rounding.max((hundredths(*value) * QUANTUM - value).abs());
                    }
                    format!("{},{}", millimetres(*x), millimetres(*y))
                })
                .collect();
            out.push_str(&pairs.join(" "));
            out.push('\n');
        }
    }
    out.push_str("END\n");
    let mut record = BTreeMap::new();
    record.insert(
        "target".to_owned(),
        Value::Text("rul-astm-d6673".to_owned()),
    );
    record.insert("unit".to_owned(), Value::Text("mm_2_decimals".to_owned()));
    record.insert(
        "convention".to_owned(),
        Value::Text("cumulative".to_owned()),
    );
    record.insert(
        "provenance".to_owned(),
        Value::Text(table.provenance().token().to_owned()),
    );
    record.insert(
        "largest_rounding_um".to_owned(),
        Value::Int(largest_rounding),
    );
    let not_carried: Vec<Value> = [
        "stack_points",
        "fixed_contours",
        "point_names",
        "declared_zero_state",
    ]
    .iter()
    .map(|what| Value::Text((*what).to_owned()))
    .collect();
    record.insert("not_carried".to_owned(), Value::List(not_carried));
    Ok(RulArtifact {
        text: out,
        record: Value::Object(record).to_text(),
    })
}

/// Why a file could not be read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RulError {
    /// What is wrong, naming the line or the rule.
    pub message: String,
}

impl fmt::Display for RulError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for RulError {}

fn refuse<T>(message: impl Into<String>) -> Result<T, RulError> {
    Err(RulError {
        message: message.into(),
    })
}

/// One rule as a file gives it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RulRule {
    /// Its grade rule identifier.
    pub id: i64,
    /// What it does at each size, in micrometres, from the sample size.
    pub deltas: Vec<(i64, i64)>,
    /// A rule of all zeros, from a file, is not known to have been meant.
    pub zero: ZeroState,
}

/// A file, read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RulFile {
    /// The table's name.
    pub name: String,
    /// Every size, in the file's order.
    pub sizes: Vec<String>,
    /// The sample size.
    pub sample: String,
    /// Each rule, by identifier.
    pub rules: Vec<RulRule>,
}

fn micrometres_from(text: &str, per_unit: i64) -> Result<i64, RulError> {
    let (sign, digits) = text.strip_prefix('-').map_or((1, text), |rest| (-1, rest));
    let digits = digits.strip_prefix('+').unwrap_or(digits);
    let (whole, fraction) = digits.split_once('.').unwrap_or((digits, ""));
    if (whole.is_empty() && fraction.is_empty())
        || !whole.chars().all(|c| c.is_ascii_digit())
        || !fraction.chars().all(|c| c.is_ascii_digit())
        || fraction.len() > 6
    {
        return refuse(format!("{text} is not a number this file can carry"));
    }
    let whole: i64 = if whole.is_empty() {
        0
    } else {
        whole
            .parse()
            .or_else(|_| refuse(format!("{text} is out of range")))?
    };
    let scale = 10_i64.pow(u32::try_from(fraction.len()).unwrap_or(0));
    let part: i64 = if fraction.is_empty() {
        0
    } else {
        fraction
            .parse()
            .or_else(|_| refuse(format!("{text} is out of range")))?
    };
    let exact = (i128::from(whole) * i128::from(scale) + i128::from(part)) * i128::from(per_unit);
    let rounded = (exact + i128::from(scale) / 2) / i128::from(scale);
    i64::try_from(rounded * i128::from(sign)).or_else(|_| refuse(format!("{text} is out of range")))
}

/// Reads a `.rul` file. A zero rule is marked unknown, because a file cannot say whether it was meant.
///
/// # Errors
///
/// It is not one of these files, or a rule does not fit its size list.
pub fn from_rul(text: &str) -> Result<RulFile, RulError> {
    let (mut name, mut count, mut sizes, mut sample) = (None, None, None, None);
    let mut per_unit = None;
    let mut standard = false;
    let mut rules: Vec<(i64, Vec<String>)> = Vec::new();
    let mut ended = false;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if ended {
            return refuse(format!("there is more after END: {line}"));
        }
        let upper = line.to_ascii_uppercase();
        if upper == "END" {
            ended = true;
            continue;
        }
        if upper.starts_with("ASTM/D13") {
            standard = true;
            continue;
        }
        if let Some(rest) = upper.strip_prefix("RULE:") {
            let mut words = rest.split_whitespace();
            let (kind, id) = (words.next(), words.next());
            if kind != Some("DELTA") {
                return refuse(format!(
                    "a rule of type {} cannot be read, and only DELTA exists",
                    kind.unwrap_or("nothing")
                ));
            }
            let Some(Ok(id)) = id.map(str::parse::<i64>) else {
                return refuse(format!("{line} has no grade rule identifier"));
            };
            rules.push((id, words.map(str::to_owned).collect()));
            continue;
        }
        if let Some((key, value)) = line.split_once(':') {
            let value = value.trim();
            match key.trim().to_ascii_uppercase().as_str() {
                "UNITS" => {
                    per_unit = Some(match value.to_ascii_uppercase().as_str() {
                        "METRIC" => 1_000,
                        "ENGLISH" => 25_400,
                        other => return refuse(format!("the units {other} are not known")),
                    });
                }
                "GRADE RULE TABLE" => name = Some(value.to_owned()),
                "NUMBER OF SIZES" => {
                    count = Some(
                        value
                            .parse::<usize>()
                            .or_else(|_| refuse(format!("{value} is not a number of sizes")))?,
                    );
                }
                "SIZE LIST" => {
                    sizes = Some(
                        value
                            .split_whitespace()
                            .map(str::to_owned)
                            .collect::<Vec<_>>(),
                    );
                }
                "SAMPLE SIZE" => sample = Some(value.to_owned()),
                // The file's own optional information: who wrote it, and when.
                _ => {}
            }
            continue;
        }
        let Some((_, numbers)) = rules.last_mut() else {
            return refuse(format!("{line} comes before any rule"));
        };
        numbers.extend(
            line.split(|c: char| c == ',' || c.is_whitespace())
                .filter(|word| !word.is_empty())
                .map(str::to_owned),
        );
    }
    if !standard {
        return refuse("this does not say it is an ASTM/D13 grade rule table");
    }
    if !ended {
        return refuse("this file stops before END, so some of it may be missing");
    }
    let Some(per_unit) = per_unit else {
        return refuse("this file does not say its units");
    };
    let Some(sizes) = sizes else {
        return refuse("this file has no size list");
    };
    if count.is_some_and(|count| count != sizes.len()) {
        return refuse(format!(
            "this file says {} sizes and lists {}",
            count.unwrap_or(0),
            sizes.len()
        ));
    }
    let Some(sample) = sample else {
        return refuse("this file does not say its sample size");
    };
    let Some(sample_at) = sizes.iter().position(|size| *size == sample) else {
        return refuse(format!("the sample size {sample} is not in the size list"));
    };
    let mut out = Vec::with_capacity(rules.len());
    for (id, numbers) in rules {
        if numbers.len() != 2 * sizes.len() {
            return refuse(format!(
                "rule {id} has {} values, and {} sizes need {}",
                numbers.len(),
                sizes.len(),
                2 * sizes.len()
            ));
        }
        let mut deltas = Vec::with_capacity(sizes.len());
        for pair in numbers.chunks(2) {
            if let [x, y] = pair {
                deltas.push((
                    micrometres_from(x, per_unit)?,
                    micrometres_from(y, per_unit)?,
                ));
            }
        }
        if deltas.get(sample_at) != Some(&(0, 0)) {
            return refuse(format!("rule {id} moves the sample size"));
        }
        out.push(RulRule {
            id,
            deltas,
            zero: ZeroState::Unknown,
        });
    }
    Ok(RulFile {
        name: name.unwrap_or_default(),
        sizes,
        sample,
        rules: out,
    })
}
