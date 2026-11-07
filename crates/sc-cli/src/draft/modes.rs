//! What the three grading modes need from a draft: where each point with a rule is, what each piece's pair is, and each piece's net line with the same number of points at every size.
use super::grade::points;
use super::{boundary, lines, required, Fields, Lengths, TOLERANCE};
use sc_artifacts::{Line, MarkKind, NestPiece, Paired, Pattern, RulePoint};
use sc_geometry::Point;
use sc_grading::GradeTable;
use sc_store::Value;
use std::collections::BTreeMap;

fn listed(description: &Fields) -> Result<Vec<&Fields>, String> {
    Ok(required(description, "pieces")?
        .as_list()
        .ok_or("pieces must be a list")?
        .iter()
        .filter_map(Value::as_object)
        .collect())
}

/// The sample with its mirrors taken out, because a file says a pair by a piece's quantity, and which pieces those were.
///
/// # Errors
///
/// A piece has no name.
pub fn without_mirrors(
    pattern: &Pattern,
    description: &Fields,
) -> Result<(Pattern, Vec<Paired>), String> {
    let mut paired = Vec::new();
    let mut mirrors = Vec::new();
    for item in listed(description)? {
        if let Some(source) = item.get("mirror_of").and_then(Value::as_text) {
            let name = required(item, "name")?
                .as_text()
                .ok_or("a piece has a name")?;
            mirrors.push(name.to_owned());
            paired.push(Paired {
                piece: source.to_owned(),
                left: item
                    .get("quantity")
                    .and_then(Value::as_int)
                    .and_then(|number| u32::try_from(number).ok())
                    .unwrap_or(1),
            });
        }
    }
    let mut kept = pattern.clone();
    kept.pieces.retain(|piece| !mirrors.contains(&piece.name));
    Ok((kept, paired))
}

/// Every point of the sample a file can name: a corner, a dart's or an internal line's end, a grainline's end, or a drill mark. A label is not a point, and an arc's middle is not a vertex of what is written.
///
/// # Errors
///
/// A length has no value.
pub fn rule_points(
    lengths: &Lengths,
    description: &Fields,
    table: &GradeTable,
) -> Result<Vec<RulePoint>, String> {
    let at = points(lengths, description)?;
    let mut found = Vec::new();
    for rule in table.rules() {
        let Some(place) = at.get(&rule.point) else {
            continue;
        };
        let parts: Vec<&str> = rule.point.split('/').collect();
        let on = match parts.as_slice() {
            [_, "grain", _] => "grain",
            [_, "internal", _, "from" | "to"] => "internal",
            [_, "darts", _, _] => "internal",
            [_, "hole", _, "at"] => "internal",
            [_, "button", _, "at"] => "drill",
            [_, _, "to"] => "boundary",
            _ => continue,
        };
        found.push(RulePoint {
            piece: parts.first().copied().unwrap_or("").to_owned(),
            at: *place,
            id: rule.id,
            on,
        });
    }
    Ok(found)
}

/// How many points each curve of each piece needs so that it is written with the same number at every size. Twice the most any size's own flattening uses, since these are evenly spaced and those are not.
///
/// # Errors
///
/// A piece's boundary cannot be worked out at one of the sizes.
pub fn curve_counts(
    description: &Fields,
    at_sizes: &[(String, Lengths)],
) -> Result<BTreeMap<String, Vec<usize>>, String> {
    let mut counts: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for item in listed(description)? {
        if item.contains_key("mirror_of") {
            continue;
        }
        let name = required(item, "name")?
            .as_text()
            .ok_or("a piece has a name")?;
        for (_, lengths) in at_sizes {
            let (edges, _) = boundary(lengths, item, name, &[])?;
            let here = counts.entry(name.to_owned()).or_default();
            if here.is_empty() {
                here.resize(edges.curves.len(), 1);
            }
            for (slot, curve) in here.iter_mut().zip(&edges.curves) {
                *slot = (*slot).max(2 * (curve.flatten(TOLERANCE).len().saturating_sub(1)));
            }
        }
    }
    Ok(counts)
}

/// Each piece at one size, for a nest: its net line with exactly the counts given, and a straight line starting at its stack point to stack it by.
///
/// # Errors
///
/// A piece has no boundary or no grainline, or its stack point has no value.
pub fn nest_at(
    lengths: &Lengths,
    description: &Fields,
    drawn: &Pattern,
    counts: &BTreeMap<String, Vec<usize>>,
    table: &GradeTable,
) -> Result<Vec<NestPiece>, String> {
    let at = points(lengths, description)?;
    let mut out = Vec::new();
    for item in listed(description)? {
        if item.contains_key("mirror_of") {
            continue;
        }
        let name = required(item, "name")?
            .as_text()
            .ok_or("a piece has a name")?;
        let (edges, _) = boundary(lengths, item, name, &[])?;
        let wanted = counts.get(name).ok_or("a piece has no counts")?;
        let mut net = Vec::new();
        for (curve, segments) in edges.curves.iter().zip(wanted) {
            let sampled = curve.sampled(*segments);
            net.extend(sampled.iter().take(sampled.len().saturating_sub(1)));
        }
        let prefix = format!("{name}/");
        let stack = table
            .attributes()
            .stack
            .iter()
            .find(|stack| stack.starts_with(&prefix))
            .and_then(|stack| at.get(stack))
            .ok_or_else(|| format!("{name} has no stack point"))?;
        let along =
            Point::new(stack.x() + 100_000, stack.y()).map_err(|error| error.to_string())?;
        let grain = lines(lengths, item, "grain")?
            .into_iter()
            .next()
            .ok_or_else(|| format!("{name} needs one grainline"))?;
        let mut drills: Vec<Point> = drawn
            .pieces
            .iter()
            .find(|piece| piece.name == name)
            .map(|piece| {
                piece
                    .marks
                    .iter()
                    .filter(|mark| mark.kind == MarkKind::Drill)
                    .map(|mark| mark.at)
                    .collect()
            })
            .unwrap_or_default();
        drills.sort();
        out.push(NestPiece {
            piece: name.to_owned(),
            net,
            grade_reference: Line {
                from: *stack,
                to: along,
            },
            grain,
            drills,
        });
    }
    Ok(out)
}
