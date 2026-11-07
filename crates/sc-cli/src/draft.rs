//! Drafts pieces from a session's description and the recipe's evaluated values: boundaries, allowances, marks and labels.
use sc_artifacts::{DrawnPiece, Line, Mark, MarkKind, Pattern, Text};
use sc_geometry::{Allowance, Corner, Curve, Outline, Point};
use sc_store::Value;
use std::collections::BTreeMap;

/// The geometric tolerance for internal drafting, in micrometres: the 10 µm class of the units chapter.
pub const TOLERANCE: i64 = 10;
/// How far a named notch may lie from the cut line before it is a wrong notch rather than rounding, in micrometres: 0.1 mm.
pub const NOTCH_REACH: i64 = 100;

type Lengths = BTreeMap<String, i64>;
type Fields = BTreeMap<String, Value>;

// A length by name: a binding, an input or `zero`, with a leading `-` to negate it, so nothing in a session is a bare number.
fn length(lengths: &Lengths, name: &str) -> Result<i64, String> {
    let (sign, bare) = name.strip_prefix('-').map_or((1, name), |rest| (-1, rest));
    lengths
        .get(bare)
        .map(|value| sign * value)
        .ok_or_else(|| format!("no length is named {bare}"))
}

fn point(lengths: &Lengths, value: &Value) -> Result<Point, String> {
    let names = value.as_list().ok_or("a point names two lengths")?;
    let [x, y] = names else {
        return Err("a point names two lengths".to_owned());
    };
    let read = |name: &Value| {
        length(
            lengths,
            name.as_text()
                .ok_or("a point's lengths are named as text")?,
        )
    };
    Point::new(read(x)?, read(y)?).map_err(|error| error.to_string())
}

fn required<'a>(fields: &'a Fields, name: &str) -> Result<&'a Value, String> {
    fields
        .get(name)
        .ok_or_else(|| format!("a piece needs {name}"))
}

fn corner_named(name: &str) -> Result<Corner, String> {
    match name {
        "miter" => Ok(Corner::Miter),
        "slant" => Ok(Corner::Slant),
        "trim" => Ok(Corner::Trim),
        "step" => Ok(Corner::Step),
        "envelope" => Ok(Corner::Envelope),
        other => Err(format!("no corner treatment is named {other}")),
    }
}

fn boundary(lengths: &Lengths, fields: &Fields) -> Result<(Outline, Vec<Allowance>), String> {
    let start = point(lengths, required(fields, "start")?)?;
    let edges = required(fields, "edges")?
        .as_list()
        .ok_or("edges must be a list")?;
    let mut curves = Vec::with_capacity(edges.len());
    let mut allowances = Vec::with_capacity(edges.len());
    let mut from = start;
    for edge in edges {
        let edge = edge.as_object().ok_or("an edge must be an object")?;
        let to = point(lengths, required(edge, "to")?)?;
        let curve = match (edge.get("first"), edge.get("second")) {
            (Some(first), Some(second)) => Curve::Cubic {
                from,
                first: point(lengths, first)?,
                second: point(lengths, second)?,
                to,
            },
            (None, None) => Curve::Line { from, to },
            _ => return Err("a curved edge names both control points".to_owned()),
        };
        curves.push(curve);
        let width = length(
            lengths,
            required(edge, "width")?
                .as_text()
                .ok_or("an edge's width is named as text")?,
        )?;
        let corner = corner_named(
            required(edge, "corner")?
                .as_text()
                .ok_or("an edge's corner is named as text")?,
        )?;
        allowances.push(Allowance { width, corner });
        from = to;
    }
    Ok((
        Outline::new(curves).map_err(|error| error.to_string())?,
        allowances,
    ))
}

fn lines(lengths: &Lengths, fields: &Fields, name: &str) -> Result<Vec<Line>, String> {
    let Some(listed) = fields.get(name).and_then(Value::as_list) else {
        return Ok(Vec::new());
    };
    listed
        .iter()
        .map(|item| match item.as_list() {
            Some([from, to]) => Ok(Line {
                from: point(lengths, from)?,
                to: point(lengths, to)?,
            }),
            _ => Err("a line names two points".to_owned()),
        })
        .collect()
}

fn marks(lengths: &Lengths, fields: &Fields) -> Result<Vec<Mark>, String> {
    let Some(listed) = fields.get("notches").and_then(Value::as_list) else {
        return Ok(Vec::new());
    };
    listed
        .iter()
        .map(|item| match item.as_list() {
            Some([at, toward]) => Ok(Mark {
                kind: MarkKind::VNotch,
                at: point(lengths, at)?,
                toward: point(lengths, toward)?,
            }),
            _ => Err("a notch names where it is and a point it aims at".to_owned()),
        })
        .collect()
}

// A notch is a vertex of the cut line, so no writer can lose it between two chords. A session names it by straight arithmetic from the net
// line, which can land a hair from where the offset's joins put the cut line, so it moves there, keeping its direction and depth. Farther
// off than `NOTCH_REACH`, it is a wrong notch and is refused. These operations are exact in IEEE arithmetic on every target.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn seat_notch(cut: &mut Vec<Point>, mark: &mut Mark) -> Result<(), String> {
    let (px, py) = (mark.at.x() as f64, mark.at.y() as f64);
    let mut nearest: Option<(f64, usize, f64, f64)> = None;
    for (index, (a, b)) in cut.iter().zip(cut.iter().cycle().skip(1)).enumerate() {
        let (ax, ay) = (a.x() as f64, a.y() as f64);
        let (dx, dy) = (b.x() as f64 - ax, b.y() as f64 - ay);
        let along = (((px - ax) * dx + (py - ay) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
        let (x, y) = (ax + along * dx, ay + along * dy);
        let away = (px - x) * (px - x) + (py - y) * (py - y);
        if nearest.is_none_or(|(best, ..)| away < best) {
            nearest = Some((away, index, x, y));
        }
    }
    let (away, index, x, y) = nearest.ok_or("a notch needs a cut line")?;
    let reach = NOTCH_REACH as f64;
    if away > reach * reach {
        return Err(format!(
            "the notch at ({}, {}) is farther than {NOTCH_REACH} micrometres from the cut line",
            mark.at.x(),
            mark.at.y()
        ));
    }
    let seated =
        Point::new(x.round() as i64, y.round() as i64).map_err(|error| error.to_string())?;
    if !cut.contains(&seated) {
        cut.insert(index + 1, seated);
    }
    mark.toward = Point::new(
        mark.toward.x() + seated.x() - mark.at.x(),
        mark.toward.y() + seated.y() - mark.at.y(),
    )
    .map_err(|error| error.to_string())?;
    mark.at = seated;
    Ok(())
}

fn labels(lengths: &Lengths, fields: &Fields) -> Result<Vec<Text>, String> {
    let Some(listed) = fields.get("labels").and_then(Value::as_list) else {
        return Ok(Vec::new());
    };
    listed
        .iter()
        .map(|item| {
            let item = item.as_object().ok_or("a label must be an object")?;
            Ok(Text {
                at: point(lengths, required(item, "at")?)?,
                text: required(item, "text")?
                    .as_text()
                    .ok_or("a label's text must be text")?
                    .to_owned(),
            })
        })
        .collect()
}

fn mirrored_point(at: Point) -> Result<Point, String> {
    Point::new(-at.x(), at.y()).map_err(|error| error.to_string())
}

fn mirrored_list(points: &[Point]) -> Result<Vec<Point>, String> {
    // A mirror turns a counterclockwise line clockwise, so it runs backward to stay counterclockwise.
    points.iter().rev().map(|at| mirrored_point(*at)).collect()
}

fn mirror(source: &DrawnPiece, name: &str, labels: Vec<Text>) -> Result<DrawnPiece, String> {
    let line = |item: &Line| -> Result<Line, String> {
        Ok(Line {
            from: mirrored_point(item.from)?,
            to: mirrored_point(item.to)?,
        })
    };
    Ok(DrawnPiece {
        name: name.to_owned(),
        size: source.size.clone(),
        quantity: source.quantity,
        cut: mirrored_list(&source.cut)?,
        sew: mirrored_list(&source.sew)?,
        grain: line(&source.grain)?,
        internal: source.internal.iter().map(line).collect::<Result<_, _>>()?,
        marks: source
            .marks
            .iter()
            .map(|mark| {
                Ok(Mark {
                    kind: mark.kind,
                    at: mirrored_point(mark.at)?,
                    toward: mirrored_point(mark.toward)?,
                })
            })
            .collect::<Result<_, String>>()?,
        labels,
    })
}

/// A drawn pattern from a session's description. Each piece names its boundary by evaluated lengths, or mirrors another piece.
pub fn draft(
    lengths: &Lengths,
    description: &Fields,
    design: &str,
    revision: &str,
) -> Result<Pattern, String> {
    let text_of = |fields: &Fields, name: &str| -> Result<String, String> {
        Ok(required(fields, name)?
            .as_text()
            .ok_or_else(|| format!("{name} must be text"))?
            .to_owned())
    };
    let style = text_of(description, "style")?;
    let size = text_of(description, "size")?;
    let listed = required(description, "pieces")?
        .as_list()
        .ok_or("pieces must be a list")?;
    let mut pieces: Vec<DrawnPiece> = Vec::with_capacity(listed.len());
    for item in listed {
        let fields = item.as_object().ok_or("a piece must be an object")?;
        let name = text_of(fields, "name")?;
        let quantity = required(fields, "quantity")?
            .as_int()
            .and_then(|number| u32::try_from(number).ok())
            .ok_or("quantity must be a small whole number")?;
        let piece = if let Some(source) = fields.get("mirror_of").and_then(Value::as_text) {
            let original = pieces
                .iter()
                .find(|piece| piece.name == source)
                .ok_or_else(|| format!("no piece before {name} is named {source}"))?;
            DrawnPiece {
                quantity,
                ..mirror(original, &name, labels(lengths, fields)?)?
            }
        } else {
            let (net, allowances) =
                boundary(lengths, fields).map_err(|error| format!("{name}: {error}"))?;
            let offset = net
                .offset(&allowances, TOLERANCE)
                .map_err(|error| format!("{name}: {error}"))?;
            let grain = lines(lengths, fields, "grain")?;
            let [grain] = grain.as_slice() else {
                return Err(format!("{name} needs one grainline"));
            };
            let mut cut = offset.points().to_vec();
            let mut notches = marks(lengths, fields)?;
            for mark in &mut notches {
                seat_notch(&mut cut, mark).map_err(|error| format!("{name}: {error}"))?;
            }
            DrawnPiece {
                name,
                size: size.clone(),
                quantity,
                cut,
                sew: net.polyline(TOLERANCE),
                grain: *grain,
                internal: lines(lengths, fields, "internal")?,
                marks: notches,
                labels: labels(lengths, fields)?,
            }
        };
        pieces.push(piece);
    }
    Ok(Pattern {
        style,
        design: design.to_owned(),
        revision: revision.to_owned(),
        pieces,
    })
}
