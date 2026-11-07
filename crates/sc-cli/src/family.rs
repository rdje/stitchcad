//! A garment at every size of its set, by evaluating its recipe again for each. Path 1 of the instantiation chapter: nothing is inherited from another size.
use crate::design::Input;
use sc_store::Value;
use sc_units::Length;
use std::collections::{BTreeMap, BTreeSet};

type Fields = BTreeMap<String, Value>;

/// One size of a set: its name, and what it changes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Size {
    /// Its name.
    pub name: String,
    /// The inputs it sets, in micrometres. The sample sets none.
    pub inputs: BTreeMap<String, i64>,
}

/// The sizes a draft names, smallest first, as the size list of a rule table runs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sizes {
    /// The size the design is drawn at.
    pub sample: String,
    /// Every size, including the sample.
    pub list: Vec<Size>,
}

/// A draft's size set.
///
/// # Errors
///
/// None is declared, or what is declared does not hang together.
pub fn sizes_of(description: &Fields) -> Result<Sizes, String> {
    let declared = description
        .get("sizes")
        .and_then(Value::as_object)
        .ok_or("this draft names no sizes")?;
    let sample = declared
        .get("sample")
        .and_then(Value::as_text)
        .ok_or("a size set names its sample")?
        .to_owned();
    let drawn = description.get("size").and_then(Value::as_text);
    if drawn != Some(sample.as_str()) {
        return Err(format!(
            "the sample size is {sample}, and this draft is drawn at {}",
            drawn.unwrap_or("no size")
        ));
    }
    let listed = declared
        .get("list")
        .and_then(Value::as_list)
        .ok_or("a size set lists its sizes")?;
    let mut seen = BTreeSet::new();
    let mut list = Vec::with_capacity(listed.len());
    for item in listed {
        let item = item.as_object().ok_or("a size must be an object")?;
        let name = item
            .get("name")
            .and_then(Value::as_text)
            .ok_or("a size has a name")?
            .to_owned();
        if !seen.insert(name.clone()) {
            return Err(format!("the size {name} is listed twice"));
        }
        let mut inputs = BTreeMap::new();
        if let Some(set) = item.get("inputs") {
            let set = set
                .as_object()
                .ok_or_else(|| format!("the size {name} lists its inputs by name"))?;
            for (token, value) in set {
                let value = value.as_int().ok_or_else(|| {
                    format!("the size {name} sets {token} to something that is not micrometres")
                })?;
                inputs.insert(token.clone(), value);
            }
        }
        list.push(Size { name, inputs });
    }
    let Some(at_sample) = list.iter().find(|size| size.name == sample) else {
        return Err(format!("the sample size {sample} is not in the list"));
    };
    if !at_sample.inputs.is_empty() {
        return Err(format!(
            "the sample size {sample} sets inputs, and the sample is the design as it stands"
        ));
    }
    Ok(Sizes { sample, list })
}

/// The inputs at a size.
///
/// # Errors
///
/// The size sets an input that does not exist, or a value out of range.
pub fn inputs_at(from: &[Input], size: &Size) -> Result<Vec<Input>, String> {
    let mut inputs = from.to_vec();
    for (token, value) in &size.inputs {
        let input = inputs
            .iter_mut()
            .find(|input| input.token == *token)
            .ok_or_else(|| {
                format!(
                    "the size {} sets {token}, and no input has that name",
                    size.name
                )
            })?;
        input.value = Length::from_micrometres(*value)
            .map_err(|error| format!("the size {}: {token}: {error}", size.name))?;
    }
    Ok(inputs)
}

/// What a recipe says each binding is, and the inputs it comes down to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Formula {
    /// The expression, as written.
    pub expression: String,
    /// Every input it depends on, however indirectly.
    pub inputs: BTreeSet<String>,
}

fn words(text: &str) -> impl Iterator<Item = &str> {
    text.split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .filter(|word| !word.is_empty())
}

/// The formula of every `let`, in a recipe whose every name is defined before it is used.
#[must_use]
pub fn formulas(recipe: &str, inputs: &[Input]) -> BTreeMap<String, Formula> {
    let tokens: BTreeSet<&str> = inputs.iter().map(|input| input.token.as_str()).collect();
    let mut found: BTreeMap<String, Formula> = BTreeMap::new();
    let mut statements: Vec<String> = Vec::new();
    // A statement ends where the next one starts.
    for line in recipe.lines() {
        let start = line.trim_start();
        if start.starts_with("let ") || start.starts_with("assert ") {
            statements.push(start.to_owned());
        } else if let Some(last) = statements.last_mut() {
            last.push(' ');
            last.push_str(start);
        }
    }
    for statement in statements {
        let Some(rest) = statement.strip_prefix("let ") else {
            continue;
        };
        let Some((head, expression)) = rest.split_once('=') else {
            continue;
        };
        let name = head.split(':').next().unwrap_or(head).trim().to_owned();
        let expression = expression.trim().to_owned();
        let mut from = BTreeSet::new();
        for word in words(&expression) {
            if tokens.contains(word) {
                from.insert(word.to_owned());
            } else if let Some(formula) = found.get(word) {
                from.extend(formula.inputs.iter().cloned());
            }
        }
        found.insert(
            name,
            Formula {
                expression,
                inputs: from,
            },
        );
    }
    found
}

fn axis(
    name: &str,
    lengths: &BTreeMap<String, i64>,
    formulas: &BTreeMap<String, Formula>,
) -> Value {
    let bare = name.strip_prefix('-').unwrap_or(name);
    let mut fields = Fields::new();
    fields.insert("name".to_owned(), Value::Text(name.to_owned()));
    let sign = if name.starts_with('-') { -1 } else { 1 };
    if let Some(value) = lengths.get(bare) {
        fields.insert("value".to_owned(), Value::Int(sign * value));
    }
    match formulas.get(bare) {
        Some(formula) => {
            fields.insert(
                "formula".to_owned(),
                Value::Text(formula.expression.clone()),
            );
            fields.insert(
                "inputs".to_owned(),
                Value::List(formula.inputs.iter().cloned().map(Value::Text).collect()),
            );
        }
        None if bare == "zero" => {
            fields.insert("formula".to_owned(), Value::Text("zero".to_owned()));
        }
        None => {
            // An input is its own formula.
            fields.insert("formula".to_owned(), Value::Text(bare.to_owned()));
            fields.insert(
                "inputs".to_owned(),
                Value::List(vec![Value::Text(bare.to_owned())]),
            );
        }
    }
    Value::Object(fields)
}

/// For every point a piece's boundary names, what drove it: the name on each axis, its value at this size, its formula and the inputs that come down to.
#[must_use]
pub fn points_of(
    description: &Fields,
    lengths: &BTreeMap<String, i64>,
    formulas: &BTreeMap<String, Formula>,
) -> Vec<Value> {
    let mut out = Vec::new();
    let listed = description
        .get("pieces")
        .and_then(Value::as_list)
        .unwrap_or(&[]);
    for piece in listed.iter().filter_map(Value::as_object) {
        let Some(name) = piece.get("name").and_then(Value::as_text) else {
            continue;
        };
        for edge in piece
            .get("edges")
            .and_then(Value::as_list)
            .unwrap_or(&[])
            .iter()
            .filter_map(Value::as_object)
        {
            let Some(edge_name) = edge.get("name").and_then(Value::as_text) else {
                continue;
            };
            for point in ["to", "first", "second", "through"] {
                let Some([x, y]) = edge.get(point).and_then(Value::as_list) else {
                    continue;
                };
                let (Some(x), Some(y)) = (x.as_text(), y.as_text()) else {
                    continue;
                };
                let mut fields = Fields::new();
                fields.insert("piece".to_owned(), Value::Text(name.to_owned()));
                fields.insert("edge".to_owned(), Value::Text(edge_name.to_owned()));
                fields.insert("point".to_owned(), Value::Text(point.to_owned()));
                fields.insert("x".to_owned(), axis(x, lengths, formulas));
                fields.insert("y".to_owned(), axis(y, lengths, formulas));
                out.push(Value::Object(fields));
            }
        }
    }
    out
}
