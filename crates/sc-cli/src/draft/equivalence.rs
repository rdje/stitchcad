//! The two paths compared, quantity by quantity. Neither wins. A difference beyond its class is a finding with an owner, and the report says which table it was measured with.
use super::grade::{piece_of, points, Rebuilt};
use super::{ratio, whole, Fields, Lengths, WALK_TOLERANCE};
use sc_artifacts::{closure_reports, walk, Pattern};
use sc_geometry::Point;
use sc_grading::GradeTable;
use sc_store::Value;
use sc_units::pure;
use std::collections::BTreeMap;

/// T1: one quantum. Two integers that should be the same number.
const T1: i64 = 1;
/// T2: internal geometry, from the tightest consumer of it, which is walking a seam.
const T2: i64 = 10;

/// What a comparison is judged against, and where that number comes from.
#[must_use]
pub fn classes() -> Value {
    let row = |name: &str, bound: Option<i64>, from: &str| {
        let mut fields = Fields::new();
        fields.insert("class".to_owned(), Value::Text(name.to_owned()));
        if let Some(bound) = bound {
            fields.insert("bound_um".to_owned(), Value::Int(bound));
        }
        fields.insert("from".to_owned(), Value::Text(from.to_owned()));
        Value::Object(fields)
    };
    Value::List(vec![
        row("T1", Some(T1), "one quantum of the representation: a corner two paths both compute is an integer, so anything else is a bug"),
        row("T2", Some(T2), "the tightest consumer of internal geometry, which is a seam's walk: two lines within this of one curve still walk to a match"),
        row("seam", Some(WALK_TOLERANCE), "twice T2: what a walk can tell apart, so a seam further out than this is one a walk would call a defect"),
        row("T4", None, "declared per receiver, and no receiver has read these files"),
        row("T5", None, "the factory's own tolerance, never invented here, and no profile has declared one"),
    ])
}

struct Quantity {
    quantity: String,
    size: String,
    by_recipe: Value,
    by_rule: Value,
    difference: i64,
    class: &'static str,
    bound: i64,
    owner: &'static str,
}

impl Quantity {
    fn to_value(&self) -> Value {
        let mut fields = Fields::new();
        fields.insert("quantity".to_owned(), Value::Text(self.quantity.clone()));
        fields.insert("size".to_owned(), Value::Text(self.size.clone()));
        fields.insert("by_recipe".to_owned(), self.by_recipe.clone());
        fields.insert("by_rule".to_owned(), self.by_rule.clone());
        fields.insert("difference_um".to_owned(), Value::Int(self.difference));
        fields.insert("class".to_owned(), Value::Text(self.class.to_owned()));
        let held = self.difference.abs() <= self.bound;
        fields.insert(
            "verdict".to_owned(),
            Value::Text(if held { "holds" } else { "finding" }.to_owned()),
        );
        if !held {
            fields.insert("owner".to_owned(), Value::Text(self.owner.to_owned()));
        }
        Value::Object(fields)
    }

    const fn held(&self) -> bool {
        self.difference.abs() <= self.bound
    }
}

fn pair(x: i64, y: i64) -> Value {
    Value::List(vec![Value::Int(x), Value::Int(y)])
}

fn to_segment(at: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let reach = dx * dx + dy * dy;
    let t = if reach == 0.0 {
        0.0
    } else {
        (((at.0 - a.0) * dx + (at.1 - a.1) * dy) / reach).clamp(0.0, 1.0)
    };
    pure::hypot(at.0 - a.0 - t * dx, at.1 - a.1 - t * dy)
}

fn shifted(line: &[Point], by: (i64, i64)) -> Vec<(f64, f64)> {
    line.iter()
        .map(|at| (ratio(at.x() - by.0), ratio(at.y() - by.1)))
        .collect()
}

// The furthest any point of either line is from the other.
fn furthest(a: &[(f64, f64)], b: &[(f64, f64)]) -> f64 {
    let one_way = |from: &[(f64, f64)], onto: &[(f64, f64)]| {
        from.iter()
            .map(|at| {
                onto.iter()
                    .enumerate()
                    .filter_map(|(index, start)| {
                        onto.get((index + 1) % onto.len())
                            .map(|end| to_segment(*at, *start, *end))
                    })
                    .fold(f64::MAX, f64::min)
            })
            .fold(0.0, f64::max)
    };
    one_way(a, b).max(one_way(b, a))
}

fn anchor_of<'a>(table: &'a GradeTable, source: &str) -> Option<&'a String> {
    let prefix = format!("{source}/");
    table
        .attributes()
        .stack
        .iter()
        .find(|stack| stack.starts_with(&prefix))
}

/// One size compared: what the recipe drew there, against what the table rebuilt.
///
/// # Errors
///
/// A point has no value, or a seam cannot be walked.
pub fn compare_size(
    size: &str,
    table: &GradeTable,
    description: &Fields,
    (at_sample, at_size): (&Lengths, &Lengths),
    by_recipe: &Pattern,
    rebuilt: &Rebuilt,
) -> Result<(Vec<Value>, Vec<Value>), String> {
    let (base, here) = (
        points(at_sample, description)?,
        points(at_size, description)?,
    );
    let mut mirrors: BTreeMap<&str, &str> = BTreeMap::new();
    for item in description
        .get("pieces")
        .and_then(Value::as_list)
        .unwrap_or(&[])
        .iter()
        .filter_map(Value::as_object)
    {
        if let (Some(name), Some(source)) = (
            item.get("name").and_then(Value::as_text),
            item.get("mirror_of").and_then(Value::as_text),
        ) {
            mirrors.insert(name, source);
        }
    }
    let drift_of = |piece: &str| -> (i64, i64) {
        let source = mirrors.get(piece).copied().unwrap_or(piece);
        anchor_of(table, source)
            .and_then(|anchor| Some((here.get(anchor)?, base.get(anchor)?)))
            .map_or((0, 0), |(now, was)| (now.x() - was.x(), now.y() - was.y()))
    };
    let mut quantities = Vec::new();
    // Every point a rule moves. These are integers, so anything but equality is a bug in the table or in reconstruction.
    for (key, was) in &base {
        let Some(rule) = table.rule(key) else {
            continue;
        };
        let delta = table
            .delta_at(rule, size)
            .map_err(|error| error.to_string())?;
        let Some(now) = here.get(key) else {
            continue;
        };
        let drift = drift_of(piece_of(key));
        let (recipe, by_rule) = (
            (now.x() - drift.0, now.y() - drift.1),
            (was.x() + delta.0, was.y() + delta.1),
        );
        quantities.push(Quantity {
            quantity: format!("point {key}"),
            size: size.to_owned(),
            by_recipe: pair(recipe.0, recipe.1),
            by_rule: pair(by_rule.0, by_rule.1),
            difference: whole(pure::hypot(
                ratio(recipe.0 - by_rule.0),
                ratio(recipe.1 - by_rule.1),
            )),
            class: "T1",
            bound: T1,
            owner: "the rule table",
        });
    }
    // Every sew line, once the piece is held to its stack point. What differs here is what lies between the points.
    for drawn in &by_recipe.pieces {
        let Some(other) = rebuilt
            .pattern
            .pieces
            .iter()
            .find(|piece| piece.name == drawn.name)
        else {
            continue;
        };
        let drift = drift_of(&drawn.name);
        // A mirror's stack point moves the other way along x, since that is what mirroring is.
        let drift = if mirrors.contains_key(drawn.name.as_str()) {
            (-drift.0, drift.1)
        } else {
            drift
        };
        let gap = furthest(&shifted(&drawn.sew, drift), &shifted(&other.sew, (0, 0)));
        quantities.push(Quantity {
            quantity: format!("sew line {}", drawn.name),
            size: size.to_owned(),
            by_recipe: Value::Int(i64::try_from(drawn.sew.len()).unwrap_or(0)),
            by_rule: Value::Int(i64::try_from(other.sew.len()).unwrap_or(0)),
            difference: whole(gap),
            class: "T2",
            bound: T2,
            owner: "the rule table",
        });
    }
    // Every seam: what it has beyond what it declares, by each path.
    let (recipe_walk, rule_walk) = (
        walk(by_recipe).map_err(|error| format!("{}: {}", error.seam, error.message))?,
        walk(&rebuilt.pattern).map_err(|error| format!("{}: {}", error.seam, error.message))?,
    );
    for seam in &recipe_walk {
        let Some(other) = rule_walk.iter().find(|other| other.seam == seam.seam) else {
            continue;
        };
        quantities.push(Quantity {
            quantity: format!("seam {}", seam.seam),
            size: size.to_owned(),
            by_recipe: Value::Int(seam.unexplained),
            by_rule: Value::Int(other.unexplained),
            difference: other.unexplained - seam.unexplained,
            class: "seam",
            bound: WALK_TOLERANCE,
            owner: "the rule table",
        });
    }
    // What each closure takes up.
    let (recipe_closures, rule_closures) = (
        closure_reports(by_recipe),
        closure_reports(&rebuilt.pattern),
    );
    for closure in &recipe_closures {
        let Some(other) = rule_closures
            .iter()
            .find(|other| other.piece == closure.piece && other.name == closure.name)
        else {
            continue;
        };
        quantities.push(Quantity {
            quantity: format!("closure {}/{}", closure.piece, closure.name),
            size: size.to_owned(),
            by_recipe: Value::Int(closure.intake),
            by_rule: Value::Int(other.intake),
            difference: other.intake - closure.intake,
            class: "seam",
            bound: WALK_TOLERANCE,
            owner: "the rule table",
        });
    }
    let findings = quantities
        .iter()
        .filter(|quantity| !quantity.held())
        .map(Quantity::to_value)
        .collect();
    Ok((
        quantities.iter().map(Quantity::to_value).collect(),
        findings,
    ))
}
