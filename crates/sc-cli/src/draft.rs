//! Drafts pieces from a session's description and the recipe's evaluated values: boundaries, allowances, marks and labels.
use sc_artifacts::{along, Closure, ClosureKind, DrawnPiece, Line, Mark, MarkKind, Pattern, Text};
use sc_geometry::{Allowance, Corner, Curve, Outline, Point};
use sc_store::Value;
use sc_units::pure;
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

fn boundary(
    lengths: &Lengths,
    fields: &Fields,
) -> Result<(Outline, Vec<Allowance>, Vec<Spread>), String> {
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
    let mut spreads = Vec::new();
    if let Some(listed) = fields.get("slash_spreads").and_then(Value::as_list) {
        for item in listed {
            let item = item
                .as_object()
                .ok_or("a slash and spread must be an object")?;
            let spread = slash_spread(lengths, item, &mut curves, &mut allowances)?;
            spreads.push(spread);
        }
    }
    Ok((
        Outline::new(curves).map_err(|error| error.to_string())?,
        allowances,
        spreads,
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

// One closure's range along the net line, refused when an end is not on it.
fn range_on(sew: &[Point], from: Point, to: Point, what: &str) -> Result<i64, String> {
    let open: f64 = sew
        .iter()
        .enumerate()
        .filter_map(|(index, a)| {
            sew.get((index + 1) % sew.len()).map(|b| {
                let (x, y) = (ratio(b.x() - a.x()), ratio(b.y() - a.y()));
                pure::hypot(x, y)
            })
        })
        .sum();
    let on = |at: Point| {
        along(sew, at).ok_or_else(|| {
            format!(
                "{what} has an end at ({}, {}), which is not on the sew line",
                at.x(),
                at.y()
            )
        })
    };
    let direct = (on(from)? - on(to)?).abs();
    Ok(whole(direct.min(open - direct)))
}

#[allow(clippy::cast_precision_loss)]
fn ratio(value: i64) -> f64 {
    value as f64
}

#[allow(clippy::cast_possible_truncation)]
fn whole(value: f64) -> i64 {
    value.round() as i64
}

// Whether a point is inside a closed line, by counting crossings of a ray.
fn inside(line: &[Point], at: Point) -> bool {
    let mut within = false;
    for (index, a) in line.iter().enumerate() {
        let Some(b) = line.get((index + 1) % line.len()) else {
            continue;
        };
        let (ay, by, y) = (ratio(a.y()), ratio(b.y()), ratio(at.y()));
        if (ay > y) != (by > y)
            && ratio(at.x()) < ratio(b.x() - a.x()) * (y - ay) / (by - ay) + ratio(a.x())
        {
            within = !within;
        }
    }
    within
}

// Darts, tucks, pleats and gathers, each with what it takes up measured from the drawn line. A declared intake that disagrees is a wrong dart, and says so.
fn closures(
    lengths: &Lengths,
    fields: &Fields,
    sew: &[Point],
    internal: &mut Vec<Line>,
) -> Result<Vec<Closure>, String> {
    let mut found = Vec::new();
    let listed = |name: &str| fields.get(name).and_then(Value::as_list).unwrap_or(&[]);
    let text = |item: &Fields, field: &str| -> Result<String, String> {
        Ok(required(item, field)?
            .as_text()
            .ok_or_else(|| format!("{field} must be text"))?
            .to_owned())
    };
    let named_length = |item: &Fields, field: &str| -> Result<Option<i64>, String> {
        match item.get(field) {
            None => Ok(None),
            Some(value) => length(
                lengths,
                value
                    .as_text()
                    .ok_or_else(|| format!("{field} names a length"))?,
            )
            .map(Some),
        }
    };
    for item in listed("darts") {
        let item = item.as_object().ok_or("a dart must be an object")?;
        let name = text(item, "name")?;
        let near = point(lengths, required(item, "near")?)?;
        let far = point(lengths, required(item, "far")?)?;
        let apex = point(lengths, required(item, "apex")?)?;
        let what = format!("the dart {name}");
        let intake = range_on(sew, near, far, &what)?;
        if let Some(declared) = named_length(item, "intake")? {
            if (declared - intake).abs() > 1 {
                return Err(format!(
                    "{what} declares an intake of {declared}, and its legs take up {intake}"
                ));
            }
        }
        if !inside(sew, apex) {
            return Err(format!(
                "{what} has its apex at ({}, {}), outside the piece",
                apex.x(),
                apex.y()
            ));
        }
        internal.push(Line {
            from: near,
            to: apex,
        });
        internal.push(Line {
            from: far,
            to: apex,
        });
        found.push(Closure {
            kind: ClosureKind::Dart,
            name,
            from: near,
            to: far,
            apex: Some(apex),
            intake,
        });
    }
    for (field, kind, word) in [
        ("tucks", ClosureKind::Tuck, "tuck"),
        ("pleats", ClosureKind::Pleat, "pleat"),
    ] {
        for item in listed(field) {
            let item = item
                .as_object()
                .ok_or("a tuck or pleat must be an object")?;
            let name = text(item, "name")?;
            let (from, to) = (
                point(lengths, required(item, "from")?)?,
                point(lengths, required(item, "to")?)?,
            );
            let what = format!("the {word} {name}");
            let intake = range_on(sew, from, to, &what)?;
            if let Some(declared) = named_length(item, "intake")? {
                if (declared - intake).abs() > 1 {
                    return Err(format!(
                        "{what} declares an intake of {declared}, and its folds take up {intake}"
                    ));
                }
            }
            for end in ["from_end", "to_end"] {
                if let Some(value) = item.get(end) {
                    let target = point(lengths, value)?;
                    let start = if end == "from_end" { from } else { to };
                    internal.push(Line {
                        from: start,
                        to: target,
                    });
                }
            }
            found.push(Closure {
                kind,
                name,
                from,
                to,
                apex: None,
                intake,
            });
        }
    }
    for item in listed("gathers") {
        let item = item.as_object().ok_or("a gather must be an object")?;
        let name = text(item, "name")?;
        let (from, to) = (
            point(lengths, required(item, "from")?)?,
            point(lengths, required(item, "to")?)?,
        );
        let what = format!("the gather {name}");
        let range = range_on(sew, from, to, &what)?;
        let gathered = named_length(item, "gathered_to")?
            .ok_or_else(|| format!("{what} needs gathered_to"))?;
        if gathered > range || gathered <= 0 {
            return Err(format!(
                "{what} draws {range} up to {gathered}, and a gather can only make a range shorter"
            ));
        }
        found.push(Closure {
            kind: ClosureKind::Gather,
            name,
            from,
            to,
            apex: None,
            intake: range - gathered,
        });
    }
    Ok(found)
}

// A turn about a pivot, for whatever lay in the part that was spread.
#[derive(Clone, Debug)]
struct Spread {
    pivot: Point,
    end: Point,
    sine: f64,
    cosine: f64,
    region: Vec<Point>,
}

impl Spread {
    fn turned(&self, at: Point) -> Result<Point, String> {
        let (x, y) = (
            ratio(at.x() - self.pivot.x()),
            ratio(at.y() - self.pivot.y()),
        );
        Point::new(
            self.pivot.x() + whole(x * self.cosine - y * self.sine),
            self.pivot.y() + whole(x * self.sine + y * self.cosine),
        )
        .map_err(|error| error.to_string())
    }

    // A point goes with the part it is on, including one on that part's own edge. One on the slash itself stays, because that side did not move.
    fn moved(&self, at: Point) -> Result<Point, String> {
        let on_part = inside(&self.region, at) || along(&self.region, at).is_some();
        if on_part && !self.on_slash(at) {
            self.turned(at)
        } else {
            Ok(at)
        }
    }

    fn on_slash(&self, at: Point) -> bool {
        along(&[self.pivot, self.end], at).is_some()
    }
}

fn turned_curve(curve: Curve, turn: &Spread) -> Result<Curve, String> {
    Ok(match curve {
        Curve::Line { from, to } => Curve::Line {
            from: turn.turned(from)?,
            to: turn.turned(to)?,
        },
        Curve::Arc { from, through, to } => Curve::Arc {
            from: turn.turned(from)?,
            through: turn.turned(through)?,
            to: turn.turned(to)?,
        },
        Curve::Cubic {
            from,
            first,
            second,
            to,
        } => Curve::Cubic {
            from: turn.turned(from)?,
            first: turn.turned(first)?,
            second: turn.turned(second)?,
            to: turn.turned(to)?,
        },
    })
}

// Makes a corner of a point on the boundary, splitting a straight edge if it lies along one. Curved edges are refused rather than split where nobody asked.
fn corner_at(
    at: Point,
    curves: &mut Vec<Curve>,
    allowances: &mut Vec<Allowance>,
    what: &str,
) -> Result<usize, String> {
    if let Some(index) = curves.iter().position(|curve| curve.from() == at) {
        return Ok(index);
    }
    for index in 0..curves.len() {
        let Some(&Curve::Line { from, to }) = curves.get(index) else {
            continue;
        };
        let cross = i128::from(to.x() - from.x()) * i128::from(at.y() - from.y())
            - i128::from(to.y() - from.y()) * i128::from(at.x() - from.x());
        let between = i128::from(at.x() - from.x()) * i128::from(to.x() - from.x())
            + i128::from(at.y() - from.y()) * i128::from(to.y() - from.y());
        let whole_length =
            i128::from(to.x() - from.x()).pow(2) + i128::from(to.y() - from.y()).pow(2);
        if cross == 0 && between > 0 && between < whole_length {
            let allowance = *allowances.get(index).ok_or("an edge has no allowance")?;
            curves.splice(
                index..=index,
                [Curve::Line { from, to: at }, Curve::Line { from: at, to }],
            );
            allowances.insert(index, allowance);
            return Ok(index + 1);
        }
    }
    Err(format!(
        "{what} at ({}, {}) is on no corner or straight edge of the boundary",
        at.x(),
        at.y()
    ))
}

// Slash and spread, as the recipe's own operation: the part of the boundary from the slash's end round to the pivot turns about the pivot, opening the slash by the spread. It is
// done again from the recipe at every draft, so a changed measurement moves it, and nothing about it is stored as geometry.
fn slash_spread(
    lengths: &Lengths,
    item: &Fields,
    curves: &mut Vec<Curve>,
    allowances: &mut Vec<Allowance>,
) -> Result<Spread, String> {
    let name = required(item, "name")?
        .as_text()
        .ok_or("a slash's name is text")?
        .to_owned();
    let what = format!("the slash {name}");
    let pivot = point(lengths, required(item, "pivot")?)?;
    let end = point(lengths, required(item, "slash_to")?)?;
    let opening = length(
        lengths,
        required(item, "spread")?
            .as_text()
            .ok_or("spread names a length")?,
    )?;
    let reach = pure::hypot(ratio(end.x() - pivot.x()), ratio(end.y() - pivot.y()));
    if opening <= 0 || ratio(opening) >= 2.0 * reach {
        return Err(format!(
            "{what} cannot open by {opening} from a slash {} long",
            whole(reach)
        ));
    }
    let start = corner_at(end, curves, allowances, &format!("the end of {what}"))?;
    corner_at(pivot, curves, allowances, &format!("the pivot of {what}"))?;
    // Splitting the pivot's edge can move the slash's end, so it is found again.
    let start = curves
        .iter()
        .position(|curve| curve.from() == end)
        .unwrap_or(start);
    let stop = curves
        .iter()
        .position(|curve| curve.from() == pivot)
        .ok_or("the pivot is on no corner")?;
    let half = ratio(opening) / 2.0;
    let angle = 2.0 * pure::atan2(half, (reach * reach - half * half).sqrt());
    // Turning the part the other way would close the slash, so each sense is tried and the one that adds cloth is kept.
    let mut best: Option<(i128, Vec<Curve>, Vec<Allowance>, Spread)> = None;
    let chain: Vec<usize> = {
        let mut indices = Vec::new();
        let mut at = start;
        while at != stop {
            indices.push(at);
            at = (at + 1) % curves.len();
        }
        indices
    };
    let region: Vec<Point> = chain
        .iter()
        .filter_map(|index| curves.get(*index))
        .flat_map(|curve| curve.flatten(TOLERANCE))
        .collect();
    for sense in [1.0, -1.0] {
        let (sine, cosine) = pure::sin_cos(sense * angle);
        let turn = Spread {
            pivot,
            end,
            sine,
            cosine,
            region: region.clone(),
        };
        let mut next_curves = Vec::with_capacity(curves.len() + 1);
        let mut next_allowances = Vec::with_capacity(curves.len() + 1);
        let before = (start + curves.len() - 1) % curves.len();
        for (index, curve) in curves.iter().enumerate() {
            let allowance = *allowances.get(index).ok_or("an edge has no allowance")?;
            if index == start {
                next_curves.push(Curve::Line {
                    from: end,
                    to: turn.turned(end)?,
                });
                next_allowances.push(*allowances.get(before).ok_or("an edge has no allowance")?);
            }
            next_curves.push(if chain.contains(&index) {
                turned_curve(*curve, &turn)?
            } else {
                *curve
            });
            next_allowances.push(allowance);
        }
        let Ok(outline) = Outline::new(next_curves.clone()) else {
            continue;
        };
        if !outline.self_crossings(TOLERANCE).is_empty() {
            continue;
        }
        let area = outline.doubled_area(TOLERANCE);
        if best.as_ref().is_none_or(|(most, ..)| area > *most) {
            best = Some((area, next_curves, next_allowances, turn));
        }
    }
    let (_, next_curves, next_allowances, turn) =
        best.ok_or_else(|| format!("{what} cannot open without the piece crossing itself"))?;
    *curves = next_curves;
    *allowances = next_allowances;
    Ok(turn)
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
        closures: source
            .closures
            .iter()
            .map(|closure| {
                Ok(Closure {
                    kind: closure.kind,
                    name: closure.name.clone(),
                    from: mirrored_point(closure.from)?,
                    to: mirrored_point(closure.to)?,
                    apex: closure.apex.map(mirrored_point).transpose()?,
                    intake: closure.intake,
                })
            })
            .collect::<Result<_, String>>()?,
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
            let (net, allowances, spreads) =
                boundary(lengths, fields).map_err(|error| format!("{name}: {error}"))?;
            let offset = net
                .offset(&allowances, TOLERANCE)
                .map_err(|error| format!("{name}: {error}"))?;
            let moved = |at: Point| {
                spreads
                    .iter()
                    .try_fold(at, |at, spread: &Spread| spread.moved(at))
            };
            let grain = lines(lengths, fields, "grain")?;
            let [grain] = grain.as_slice() else {
                return Err(format!("{name} needs one grainline"));
            };
            let mut cut = offset.points().to_vec();
            let mut notches = marks(lengths, fields)?;
            for mark in &mut notches {
                mark.at = moved(mark.at)?;
                mark.toward = moved(mark.toward)?;
                seat_notch(&mut cut, mark).map_err(|error| format!("{name}: {error}"))?;
            }
            let sew = net.polyline(TOLERANCE);
            let mut internal = Vec::new();
            for line in lines(lengths, fields, "internal")? {
                internal.push(Line {
                    from: moved(line.from)?,
                    to: moved(line.to)?,
                });
            }
            let closures = closures(lengths, fields, &sew, &mut internal)
                .map_err(|error| format!("{name}: {error}"))?;
            DrawnPiece {
                name,
                size: size.clone(),
                quantity,
                cut,
                sew,
                grain: *grain,
                internal,
                marks: notches,
                labels: labels(lengths, fields)?,
                closures,
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
