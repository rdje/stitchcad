//! Drafts pieces from a session's description and the recipe's evaluated values: boundaries, allowances, marks and labels.
use sc_artifacts::{
    along, stretch, stretch_length, walk, ButtonPair, Closure, ClosureKind, DrawnPiece, Fit,
    GradePoint, Line, Mark, MarkKind, Pattern, Pocket, Repair, Seam, SeamSide, Text,
    WALK_TOLERANCE,
};
use sc_geometry::{Corner, Curve, Outline, Point};
use sc_store::Value;
use sc_units::pure;
use std::collections::BTreeMap;

mod edges;
mod equivalence;
mod extremes;
mod grade;
mod modes;
use edges::{Answer, Edges, Held, Located};
pub use equivalence::{classes, compare_size, FORMAT, NUMERICAL};
pub use extremes::{checks_on, Check};
pub use grade::{derive_table, reconstruct, Conflict};
pub use modes::{curve_counts, nest_at, rule_points, without_mirrors};

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

// The boundary: its edges, what has been edited on them, its spreads, and where truing moved a corner, in that order.
fn boundary(
    lengths: &Lengths,
    fields: &Fields,
    piece: &str,
    moved_corners: &[(Point, Point)],
) -> Result<(Edges, Vec<Spread>), String> {
    let mut edges = Edges::describe(lengths, fields)?;
    edges.edit_all(lengths, fields, piece)?;
    let mut spreads = Vec::new();
    if let Some(listed) = fields.get("slash_spreads").and_then(Value::as_list) {
        for item in listed {
            let item = item
                .as_object()
                .ok_or("a slash and spread must be an object")?;
            spreads.push(slash_spread(lengths, item, &mut edges)?);
        }
    }
    // A trued seam moves one corner along its straight edge, and both edges meeting there follow it.
    for (old, new) in moved_corners {
        for curve in &mut edges.curves {
            let at = |point: Point| if point == *old { *new } else { point };
            *curve = match *curve {
                Curve::Line { from, to } => Curve::Line {
                    from: at(from),
                    to: at(to),
                },
                Curve::Arc { from, through, to } => Curve::Arc {
                    from: at(from),
                    through,
                    to: at(to),
                },
                Curve::Cubic {
                    from,
                    first,
                    second,
                    to,
                } => Curve::Cubic {
                    from: at(from),
                    first,
                    second,
                    to: at(to),
                },
            };
        }
    }
    Ok((edges, spreads))
}

fn lines(lengths: &Lengths, fields: &Fields, name: &str) -> Result<Vec<Line>, String> {
    let Some(listed) = fields.get(name).and_then(Value::as_list) else {
        return Ok(Vec::new());
    };
    let mut found = Vec::with_capacity(listed.len());
    for item in listed {
        if let Some(item) = item.as_object() {
            let flat = Curve::Arc {
                from: point(lengths, required(item, "from")?)?,
                through: point(lengths, required(item, "through")?)?,
                to: point(lengths, required(item, "to")?)?,
            }
            .flatten(TOLERANCE);
            for pair in flat.windows(2) {
                if let [from, to] = pair {
                    found.push(Line {
                        from: *from,
                        to: *to,
                    });
                }
            }
            continue;
        }
        let Some([from, to]) = item.as_list() else {
            return Err("a line names two points, or an arc as from, through and to".to_owned());
        };
        found.push(Line {
            from: point(lengths, from)?,
            to: point(lengths, to)?,
        });
    }
    Ok(found)
}

// What each reference in a drawing came to: an answer for every one, and a repair task for each that has none.
#[derive(Default)]
struct Book {
    repairs: Vec<Repair>,
    references: Vec<Value>,
}

impl Book {
    fn settle(
        &mut self,
        piece: &str,
        carrier: &str,
        held: &Held,
        answer: Answer,
        mirrored: bool,
    ) -> Result<Option<Located>, String> {
        let mut fields = BTreeMap::new();
        fields.insert("piece".to_owned(), Value::Text(piece.to_owned()));
        fields.insert("carrier".to_owned(), Value::Text(carrier.to_owned()));
        fields.insert("holds".to_owned(), Value::Text(held.words()));
        match answer {
            Answer::Here(mut located) => {
                if mirrored {
                    located.point = mirrored_point(located.point)?;
                }
                // A mirror runs its boundary the other way, so a directed consumer has to be told that as well.
                let reversed =
                    (located.direction == sc_core::ontology::Direction::Reversed) != mirrored;
                fields.insert("now".to_owned(), Value::Text(located.now.clone()));
                fields.insert(
                    "direction".to_owned(),
                    Value::Text(if reversed { "reversed" } else { "original" }.to_owned()),
                );
                fields.insert(
                    "point".to_owned(),
                    Value::List(vec![
                        Value::Int(located.point.x()),
                        Value::Int(located.point.y()),
                    ]),
                );
                self.references.push(Value::Object(fields));
                Ok(Some(located))
            }
            Answer::Repair {
                orphaned_by,
                candidates,
            } => {
                fields.insert("now".to_owned(), Value::Text("unresolved".to_owned()));
                self.references.push(Value::Object(fields));
                self.repairs.push(Repair {
                    piece: piece.to_owned(),
                    carrier: carrier.to_owned(),
                    reference: held.words(),
                    orphaned_by,
                    candidates,
                });
                Ok(None)
            }
        }
    }

    fn is_open(&self, seam: &str) -> bool {
        let start = format!("seam {seam},");
        self.repairs
            .iter()
            .any(|repair| repair.carrier.starts_with(&start))
    }

    // Something that hangs off a seam waits while that seam has nowhere to go, and says so.
    fn waiting(&mut self, what: &str, seam: &str) {
        self.repairs.push(Repair {
            piece: String::new(),
            carrier: format!("{what} on seam {seam}"),
            reference: format!("the seam {seam}"),
            orphaned_by: format!("the seam {seam}, which is itself waiting for a repair"),
            candidates: Vec::new(),
        });
    }
}

fn named_item<'a>(item: &'a Fields, what: &str, seen: &mut Vec<String>) -> Result<&'a str, String> {
    let name = required(item, "name")
        .map_err(|_| format!("a {what} that holds an edge needs a name"))?
        .as_text()
        .ok_or_else(|| format!("a {what}'s name is text"))?;
    if seen.iter().any(|other| other == name) {
        return Err(format!("two {what}s are named {name}"));
    }
    seen.push(name.to_owned());
    Ok(name)
}

// A notch from a piece's description. One written as two points is where a session put it, and it is refused if the cut line is no longer there.
// One written as an edge and a place on it is a reference: it lands where the ledger says, or it is a repair task.
// A notch's type, which the ontology stores as what the mark means. Where a profile wants it drawn differently is the profile's.
enum NotchType {
    Single,
    Double,
    Drill,
}

fn notch_type(item: &Fields, name: &str) -> Result<NotchType, String> {
    match item.get("type").and_then(Value::as_text) {
        None | Some("single") => Ok(NotchType::Single),
        Some("double") => Ok(NotchType::Double),
        Some("drill") => Ok(NotchType::Drill),
        Some(other) => Err(format!(
            "the notch {name} is of type {other}, and that is env_notch_type: a notch is single, double or drill until the export types of G4"
        )),
    }
}

// Where a notch reaches, with its point on the cut line and its aim into the piece. Square to the edge, as far as its allowance, unless it states its own line, which it must at a corner that gives two squares.
fn notch_mark(lengths: &Lengths, item: &Fields, located: &Located) -> Result<Mark, String> {
    let depth = length(
        lengths,
        required(item, "depth")?
            .as_text()
            .ok_or("a notch's depth names a length")?,
    )?;
    let (reach_x, reach_y) = match item.get("aim") {
        Some(aim) => {
            let names = aim.as_list().ok_or("a notch's aim names two lengths")?;
            let [x, y] = names else {
                return Err("a notch's aim names two lengths".to_owned());
            };
            let read = |name: &Value| {
                length(
                    lengths,
                    name.as_text().ok_or("an aim's lengths are named as text")?,
                )
            };
            (ratio(read(x)?), ratio(read(y)?))
        }
        None => (
            located.outward.0 * ratio(located.width),
            located.outward.1 * ratio(located.width),
        ),
    };
    let reach = pure::hypot(reach_x, reach_y);
    let (unit_x, unit_y) = if reach > 0.0 {
        (reach_x / reach, reach_y / reach)
    } else {
        located.outward
    };
    let at = Point::new(
        located.point.x() + whole(reach_x),
        located.point.y() + whole(reach_y),
    )
    .map_err(|error| error.to_string())?;
    Ok(Mark {
        kind: MarkKind::VNotch,
        at,
        toward: Point::new(
            at.x() - whole(unit_x * ratio(depth)),
            at.y() - whole(unit_y * ratio(depth)),
        )
        .map_err(|error| error.to_string())?,
    })
}

// A drill hole is a place inside the piece, held from an edge: as far in, square to that edge, as it says.
fn drill_mark(
    lengths: &Lengths,
    item: &Fields,
    located: &Located,
    cut: &[Point],
    name: &str,
) -> Result<Mark, String> {
    let inset = length(
        lengths,
        required(item, "inset")?
            .as_text()
            .ok_or("a drill's inset names a length")?,
    )?;
    let at = Point::new(
        located.point.x() - whole(located.outward.0 * ratio(inset)),
        located.point.y() - whole(located.outward.1 * ratio(inset)),
    )
    .map_err(|error| error.to_string())?;
    if !inside(cut, at) {
        return Err(format!(
            "the drill {name} lands at ({}, {}), outside the piece",
            at.x(),
            at.y()
        ));
    }
    Ok(Mark {
        kind: MarkKind::Drill,
        at,
        toward: at,
    })
}

// One carrier for a single notch or a drill. A double is two marks either side of the place it holds, each answering for itself.
fn notch_places(
    lengths: &Lengths,
    item: &Fields,
    edges: &Edges,
    name: &str,
    kind: &NotchType,
) -> Result<Vec<(String, Held)>, String> {
    let what = format!("the notch {name}");
    let place = edges.held(lengths, item, &what)?;
    if !matches!(kind, NotchType::Double) {
        return Ok(vec![(format!("notch {name}"), place)]);
    }
    let spacing = length(
        lengths,
        required(item, "spacing")?
            .as_text()
            .ok_or("a double notch's spacing names a length")?,
    )?;
    Ok(vec![
        (
            format!("notch {name}, first"),
            edges.beside(&place, -spacing / 2, &what)?,
        ),
        (
            format!("notch {name}, second"),
            edges.beside(&place, spacing - spacing / 2, &what)?,
        ),
    ])
}

fn notches_of(
    lengths: &Lengths,
    fields: &Fields,
    edges: &Edges,
    piece: &str,
    cut: &mut Vec<Point>,
    moved: &dyn Fn(Point) -> Result<Point, String>,
    book: &mut Book,
) -> Result<Vec<Mark>, String> {
    let listed = fields
        .get("notches")
        .and_then(Value::as_list)
        .unwrap_or(&[]);
    let mut seen = Vec::new();
    let mut held = Vec::new();
    for item in listed {
        if let Some(item) = item.as_object() {
            let name = named_item(item, "notch", &mut seen)?;
            let kind = notch_type(item, name)?;
            let places = notch_places(lengths, item, edges, name, &kind)?;
            held.push((name, item, kind, places));
        }
    }
    let places: Vec<Held> = held
        .iter()
        .flat_map(|(_, _, _, places)| places.iter().map(|(_, place)| place.clone()))
        .collect();
    let trimmed = edges.trimmed(cut, &places)?;
    let mut held = held.into_iter();
    let mut out = Vec::with_capacity(listed.len());
    for item in listed {
        if let Some(points) = item.as_list() {
            let [at, toward] = points else {
                return Err("a notch names where it is and a point it aims at".to_owned());
            };
            let mut mark = Mark {
                kind: MarkKind::VNotch,
                at: moved(point(lengths, at)?)?,
                toward: moved(point(lengths, toward)?)?,
            };
            seat_notch(cut, &mut mark)?;
            out.push(mark);
            continue;
        }
        let (name, item, kind, places) = held
            .next()
            .ok_or("a notch must be two points or an object")?;
        for (carrier, place) in places {
            // A drill is inside the piece, so it does not care what happened to the cut line outside it.
            let answer = if matches!(kind, NotchType::Drill) {
                edges.answer(&place)?
            } else {
                edges.answer_on_cut(&trimmed, &place)?
            };
            let Some(located) = book.settle(piece, &carrier, &place, answer, false)? else {
                continue;
            };
            if matches!(kind, NotchType::Drill) {
                out.push(drill_mark(lengths, item, &located, cut, name)?);
            } else {
                let mut mark = notch_mark(lengths, item, &located)?;
                seat_notch(cut, &mut mark)?;
                out.push(mark);
            }
        }
    }
    Ok(out)
}

fn grade_points_of(
    lengths: &Lengths,
    fields: &Fields,
    edges: &Edges,
    piece: &str,
    book: &mut Book,
) -> Result<Vec<GradePoint>, String> {
    let mut seen = Vec::new();
    let mut out = Vec::new();
    for item in fields
        .get("grade_points")
        .and_then(Value::as_list)
        .unwrap_or(&[])
    {
        let item = item.as_object().ok_or("a grade point must be an object")?;
        let name = named_item(item, "grade point", &mut seen)?;
        let place = edges.held(lengths, item, &format!("the grade point {name}"))?;
        let answer = edges.answer(&place)?;
        if let Some(located) =
            book.settle(piece, &format!("grade point {name}"), &place, answer, false)?
        {
            out.push(GradePoint {
                name: name.to_owned(),
                at: located.point,
            });
        }
    }
    Ok(out)
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

// A label is written with `{size}` where the size goes, so that it says the size it is on.
fn labels(lengths: &Lengths, fields: &Fields, size: &str) -> Result<Vec<Text>, String> {
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
                    .replace("{size}", size),
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
fn corner_at(at: Point, edges: &mut Edges, what: &str) -> Result<usize, String> {
    if let Some(index) = edges.curves.iter().position(|curve| curve.from() == at) {
        return Ok(index);
    }
    for index in 0..edges.curves.len() {
        let Some(&Curve::Line { from, to }) = edges.curves.get(index) else {
            continue;
        };
        let cross = i128::from(to.x() - from.x()) * i128::from(at.y() - from.y())
            - i128::from(to.y() - from.y()) * i128::from(at.x() - from.x());
        let between = i128::from(at.x() - from.x()) * i128::from(to.x() - from.x())
            + i128::from(at.y() - from.y()) * i128::from(to.y() - from.y());
        let whole_length =
            i128::from(to.x() - from.x()).pow(2) + i128::from(to.y() - from.y()).pow(2);
        if cross == 0 && between > 0 && between < whole_length {
            let allowance = *edges
                .allowances
                .get(index)
                .ok_or("an edge has no allowance")?;
            let owner = edges.owners.get(index).copied().flatten();
            edges.curves.splice(
                index..=index,
                [Curve::Line { from, to: at }, Curve::Line { from: at, to }],
            );
            edges.allowances.insert(index, allowance);
            edges.owners.insert(index, owner);
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
// How much cloth a sense of turning adds, and what the boundary comes to with it.
type Opened = (
    i128,
    Vec<Curve>,
    Vec<sc_geometry::Allowance>,
    Vec<Option<usize>>,
    Spread,
);

fn slash_spread(lengths: &Lengths, item: &Fields, edges: &mut Edges) -> Result<Spread, String> {
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
    let start = corner_at(end, edges, &format!("the end of {what}"))?;
    corner_at(pivot, edges, &format!("the pivot of {what}"))?;
    let (curves, allowances, owners) =
        (&mut edges.curves, &mut edges.allowances, &mut edges.owners);
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
    let mut best: Option<Opened> = None;
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
        let mut next_owners = Vec::with_capacity(curves.len() + 1);
        let before = (start + curves.len() - 1) % curves.len();
        for (index, curve) in curves.iter().enumerate() {
            let allowance = *allowances.get(index).ok_or("an edge has no allowance")?;
            if index == start {
                next_curves.push(Curve::Line {
                    from: end,
                    to: turn.turned(end)?,
                });
                next_allowances.push(*allowances.get(before).ok_or("an edge has no allowance")?);
                // The opening is new cloth, and no edge's frame runs through it.
                next_owners.push(None);
            }
            next_curves.push(if chain.contains(&index) {
                turned_curve(*curve, &turn)?
            } else {
                *curve
            });
            next_allowances.push(allowance);
            next_owners.push(owners.get(index).copied().flatten());
        }
        let Ok(outline) = Outline::new(next_curves.clone()) else {
            continue;
        };
        if !outline.self_crossings(TOLERANCE).is_empty() {
            continue;
        }
        let area = outline.doubled_area(TOLERANCE);
        if best.as_ref().is_none_or(|(most, ..)| area > *most) {
            best = Some((area, next_curves, next_allowances, next_owners, turn));
        }
    }
    let (_, next_curves, next_allowances, next_owners, turn) =
        best.ok_or_else(|| format!("{what} cannot open without the piece crossing itself"))?;
    *curves = next_curves;
    *allowances = next_allowances;
    *owners = next_owners;
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
        grade_points: source
            .grade_points
            .iter()
            .map(|item| {
                Ok(GradePoint {
                    name: item.name.clone(),
                    at: mirrored_point(item.at)?,
                })
            })
            .collect::<Result<_, String>>()?,
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

/// A drawn pattern from a session's description. Each piece names its boundary by evaluated lengths, or mirrors another piece. A seam a description says to true is
/// worked out from a first drawing and applied to the second, so it is done again whenever a measurement moves.
pub fn draft(
    lengths: &Lengths,
    description: &Fields,
    design: &str,
    revision: &str,
) -> Result<Pattern, String> {
    drawing(lengths, description, design, revision).map(|drawn| drawn.pattern)
}

/// What each reference in a drawing came to, and every repair task that leaves.
///
/// # Errors
///
/// What `draft` refuses.
pub fn references(
    lengths: &Lengths,
    description: &Fields,
    design: &str,
    revision: &str,
) -> Result<(Vec<Value>, Vec<Repair>), String> {
    drawing(lengths, description, design, revision)
        .map(|drawn| (drawn.references, drawn.pattern.repairs))
}

fn drawing(
    lengths: &Lengths,
    description: &Fields,
    design: &str,
    revision: &str,
) -> Result<Drawing, String> {
    let first = draw(lengths, description, design, revision, &BTreeMap::new())?;
    let listed = description
        .get("trues")
        .and_then(Value::as_list)
        .unwrap_or(&[]);
    if listed.is_empty() {
        return Ok(first);
    }
    let mut moves: Moves = BTreeMap::new();
    let mut waiting = Vec::new();
    for item in listed {
        let item = item.as_object().ok_or("a true must be an object")?;
        if let Some(seam) = item.get("seam").and_then(Value::as_text) {
            if first.open_seams.iter().any(|open| open == seam) {
                waiting.push(seam.to_owned());
                continue;
            }
        }
        let (piece, old, new) = trued_corner(item, &first.pattern)?;
        moves.entry(piece).or_default().push((old, new));
    }
    let mut drawn = draw(lengths, description, design, revision, &moves)?;
    for seam in waiting {
        let mut book = Book::default();
        book.waiting("a true", &seam);
        drawn.pattern.repairs.append(&mut book.repairs);
    }
    drawn.pattern.repairs.sort();
    Ok(drawn)
}

// Which corner truing a seam moves, and where to. Side A or B is lengthened or shortened along the straight edge ending at its `to`, by what the seam's walk says it is out.
fn trued_corner(item: &Fields, first: &Pattern) -> Result<(String, Point, Point), String> {
    let name = required(item, "seam")?
        .as_text()
        .ok_or("a true names its seam")?;
    let which = required(item, "side")?
        .as_text()
        .ok_or("a true names its side, a or b")?;
    let seam = first
        .seams
        .iter()
        .find(|seam| seam.name == name)
        .ok_or_else(|| format!("no seam is named {name}"))?;
    let report = walk(first).map_err(|error| format!("{}: {}", error.seam, error.message))?;
    let out = report
        .iter()
        .find(|item| item.seam == name)
        .ok_or_else(|| format!("no seam is named {name}"))?
        .unexplained;
    let (side, change) = match which {
        "a" => (&seam.a, -out),
        "b" => (&seam.b, out),
        other => return Err(format!("a seam has sides a and b, and no side {other}")),
    };
    let piece = first
        .pieces
        .iter()
        .find(|piece| piece.name == side.piece)
        .ok_or_else(|| format!("no piece is named {}", side.piece))?;
    let walked = stretch(&piece.sew, side.from, side.to).ok_or_else(|| {
        format!(
            "the seam {name} has an end off the sew line of {}",
            side.piece
        )
    })?;
    let [.., before, end] = walked.as_slice() else {
        return Err(format!("the seam {name} is too short to true"));
    };
    let (dx, dy) = (ratio(end.x() - before.x()), ratio(end.y() - before.y()));
    let reach = pure::hypot(dx, dy);
    if ratio(change.abs()) >= reach {
        return Err(format!(
            "the seam {name} is out by {out}, and {}'s last straight run is only {} long",
            side.piece,
            whole(reach)
        ));
    }
    let moved = Point::new(
        end.x() + whole(dx / reach * ratio(change)),
        end.y() + whole(dy / reach * ratio(change)),
    )
    .map_err(|error| error.to_string())?;
    Ok((side.piece.clone(), *end, moved))
}

type Tables = BTreeMap<String, (Edges, bool)>;

// One end of a side: where a session put it, or a reference that lands or becomes a repair task.
fn seam_end(
    lengths: &Lengths,
    tables: &Tables,
    piece: &str,
    value: &Value,
    moves: &Moves,
    carrier: &str,
    book: &mut Book,
) -> Result<Option<Point>, String> {
    let Some(fields) = value.as_object() else {
        let at = point(lengths, value)?;
        return Ok(Some(
            moves
                .get(piece)
                .and_then(|listed| listed.iter().find(|(old, _)| *old == at))
                .map_or(at, |(_, new)| *new),
        ));
    };
    let (edges, mirrored) = tables
        .get(piece)
        .ok_or_else(|| format!("no piece is named {piece}"))?;
    let place = edges.held(lengths, fields, carrier)?;
    let answer = edges.answer(&place)?;
    Ok(book
        .settle(piece, carrier, &place, answer, *mirrored)?
        .map(|located| located.point))
}

fn seam_side(
    lengths: &Lengths,
    tables: &Tables,
    value: &Value,
    moves: &Moves,
    carrier: &str,
    book: &mut Book,
) -> Result<Option<SeamSide>, String> {
    let fields = value.as_object().ok_or("a seam's side must be an object")?;
    let piece = required(fields, "piece")?
        .as_text()
        .ok_or("a seam's side names its piece")?
        .to_owned();
    let from = seam_end(
        lengths,
        tables,
        &piece,
        required(fields, "from")?,
        moves,
        &format!("{carrier}, from"),
        book,
    )?;
    let to = seam_end(
        lengths,
        tables,
        &piece,
        required(fields, "to")?,
        moves,
        &format!("{carrier}, to"),
        book,
    )?;
    Ok(match (from, to) {
        (Some(from), Some(to)) => Some(SeamSide { piece, from, to }),
        _ => None,
    })
}

fn seams_of(
    lengths: &Lengths,
    description: &Fields,
    tables: &Tables,
    moves: &Moves,
    book: &mut Book,
) -> Result<Vec<Seam>, String> {
    let mut seams = Vec::new();
    for item in description
        .get("seams")
        .and_then(Value::as_list)
        .unwrap_or(&[])
    {
        let item = item.as_object().ok_or("a seam must be an object")?;
        let ease = match item.get("ease") {
            None => 0,
            Some(value) => length(
                lengths,
                value.as_text().ok_or("a seam's ease names a length")?,
            )?,
        };
        let name = required(item, "name")?
            .as_text()
            .ok_or("a seam's name is text")?
            .to_owned();
        let a = seam_side(
            lengths,
            tables,
            required(item, "a")?,
            moves,
            &format!("seam {name}, side a"),
            book,
        )?;
        let b = seam_side(
            lengths,
            tables,
            required(item, "b")?,
            moves,
            &format!("seam {name}, side b"),
            book,
        )?;
        // A seam with an end that has nowhere to go is not drawn at some other length. It is out of the graph until it is repaired.
        if let (Some(a), Some(b)) = (a, b) {
            seams.push(Seam { name, a, b, ease });
        }
    }
    Ok(seams)
}

// The point a distance along a polyline.
fn point_along(points: &[Point], distance: f64) -> Result<Point, String> {
    let mut walked = 0.0;
    for pair in points.windows(2) {
        let [a, b] = pair else { continue };
        let (dx, dy) = (ratio(b.x() - a.x()), ratio(b.y() - a.y()));
        let length = pure::hypot(dx, dy);
        if length > 0.0 && walked + length >= distance {
            let t = (distance - walked) / length;
            return Point::new(a.x() + whole(dx * t), a.y() + whole(dy * t))
                .map_err(|error| error.to_string());
        }
        walked += length;
    }
    points
        .last()
        .copied()
        .ok_or_else(|| "an empty stretch".to_owned())
}

// One balance notch on one side of a seam: at this distance along the sew line, marked on the cut line, aiming back at the sew line.
fn plant(
    piece: &mut DrawnPiece,
    side: &SeamSide,
    distance: f64,
    depth: i64,
    seam: &str,
) -> Result<(), String> {
    let walked = stretch(&piece.sew, side.from, side.to).ok_or_else(|| {
        format!(
            "the seam {seam} has an end off the sew line of {}",
            piece.name
        )
    })?;
    let on_sew = point_along(&walked, distance)?;
    let mut best: Option<(f64, f64, f64)> = None;
    for (index, a) in piece.cut.iter().enumerate() {
        let Some(b) = piece.cut.get((index + 1) % piece.cut.len()) else {
            continue;
        };
        let (dx, dy) = (ratio(b.x() - a.x()), ratio(b.y() - a.y()));
        let reach = dx * dx + dy * dy;
        if reach == 0.0 {
            continue;
        }
        let t = ((ratio(on_sew.x() - a.x()) * dx + ratio(on_sew.y() - a.y()) * dy) / reach)
            .clamp(0.0, 1.0);
        let (x, y) = (ratio(a.x()) + dx * t, ratio(a.y()) + dy * t);
        let away = pure::hypot(ratio(on_sew.x()) - x, ratio(on_sew.y()) - y);
        if best.is_none_or(|(least, ..)| away < least) {
            best = Some((away, x, y));
        }
    }
    let (away, x, y) = best.ok_or("a piece with no cut line")?;
    let at = Point::new(whole(x), whole(y)).map_err(|error| error.to_string())?;
    // On a piece with no allowance here, the notch aims in along the seam's own normal, so it still has a direction.
    let (ax, ay) = if away > 0.5 {
        (
            (ratio(on_sew.x()) - x) / away,
            (ratio(on_sew.y()) - y) / away,
        )
    } else {
        let centre = piece.sew.first().copied().unwrap_or(at);
        let (cx, cy) = (ratio(centre.x()) - x, ratio(centre.y()) - y);
        let far = pure::hypot(cx, cy).max(1.0);
        (cx / far, cy / far)
    };
    let toward = Point::new(
        at.x() + whole(ax * ratio(depth)),
        at.y() + whole(ay * ratio(depth)),
    )
    .map_err(|error| error.to_string())?;
    let mut mark = Mark {
        kind: MarkKind::VNotch,
        at,
        toward,
    };
    // A piece on the same side of two seams, as a front cut on the fold is, has the one notch and not two on top of each other.
    let already = piece.marks.iter().any(|other| {
        other.kind == mark.kind
            && (other.at.x() - mark.at.x()).abs() <= TOLERANCE
            && (other.at.y() - mark.at.y()).abs() <= TOLERANCE
    });
    if already {
        return Ok(());
    }
    seat_notch(&mut piece.cut, &mut mark)?;
    piece.marks.push(mark);
    Ok(())
}

// Balance notches, planted at a distance along side A and at the same share of side B, so the ease between them is spread evenly and the notch means the same on both.
fn balance_notches(
    lengths: &Lengths,
    description: &Fields,
    pieces: &mut [DrawnPiece],
    seams: &[Seam],
    book: &mut Book,
) -> Result<(), String> {
    for item in description
        .get("balance_notches")
        .and_then(Value::as_list)
        .unwrap_or(&[])
    {
        let item = item
            .as_object()
            .ok_or("a balance notch must be an object")?;
        let name = required(item, "seam")?
            .as_text()
            .ok_or("a balance notch names its seam")?;
        if book.is_open(name) {
            book.waiting("a balance notch", name);
            continue;
        }
        let seam = seams
            .iter()
            .find(|seam| seam.name == name)
            .ok_or_else(|| format!("no seam is named {name}"))?;
        let depth = length(
            lengths,
            required(item, "depth")?
                .as_text()
                .ok_or("depth names a length")?,
        )?;
        let side_line = |side: &SeamSide| -> Result<f64, String> {
            let piece = pieces
                .iter()
                .find(|piece| piece.name == side.piece)
                .ok_or_else(|| format!("no piece is named {}", side.piece))?;
            let walked = stretch(&piece.sew, side.from, side.to).ok_or_else(|| {
                format!(
                    "the seam {name} has an end off the sew line of {}",
                    side.piece
                )
            })?;
            Ok(stretch_length(&walked))
        };
        let (a_length, b_length) = (side_line(&seam.a)?, side_line(&seam.b)?);
        // Graded by a rule it keeps its share of the seam, and everywhere else it is a distance along side A.
        let along_a = match item.get("share").and_then(Value::as_text) {
            Some(written) => {
                let (top, bottom) = written.split_once('/').ok_or_else(|| {
                    format!("a balance notch's share is written as a/b, not {written}")
                })?;
                let (top, bottom): (i64, i64) = (
                    top.parse()
                        .map_err(|_| format!("{written} is not a share"))?,
                    bottom
                        .parse()
                        .map_err(|_| format!("{written} is not a share"))?,
                );
                ratio(top) / ratio(bottom) * a_length
            }
            None => {
                let at = length(
                    lengths,
                    required(item, "at")?.as_text().ok_or("at names a length")?,
                )?;
                if at <= 0 || ratio(at) >= a_length {
                    return Err(format!(
                        "a balance notch at {at} is not along the seam {name}, which is {} long",
                        whole(a_length)
                    ));
                }
                ratio(at)
            }
        };
        let share = along_a / a_length;
        for (side, distance) in [(&seam.a, along_a), (&seam.b, share * b_length)] {
            let piece = pieces
                .iter_mut()
                .find(|piece| piece.name == side.piece)
                .ok_or_else(|| format!("no piece is named {}", side.piece))?;
            plant(piece, side, distance, depth, name)?;
        }
    }
    Ok(())
}

type Moves = BTreeMap<String, Vec<(Point, Point)>>;

// A pocket is an object, and not lines on a piece. It opens on a named edge, so it follows an edit to that edge or says it cannot, and the pieces it is made of have to exist.
fn pockets_of(
    lengths: &Lengths,
    description: &Fields,
    tables: &Tables,
    pieces: &[DrawnPiece],
    book: &mut Book,
) -> Result<Vec<Pocket>, String> {
    let mut seen = Vec::new();
    let mut found = Vec::new();
    for item in description
        .get("pockets")
        .and_then(Value::as_list)
        .unwrap_or(&[])
    {
        let item = item.as_object().ok_or("a pocket must be an object")?;
        let name = named_item(item, "pocket", &mut seen)?;
        let text = |field: &str| -> Result<&str, String> {
            required(item, field)?
                .as_text()
                .ok_or_else(|| format!("the pocket {name}: {field} is text"))
        };
        let piece = text("piece")?;
        let opening = text("opening")?;
        if opening != "in_seam" {
            return Err(format!(
                "the pocket {name} opens {opening}, and in_seam is the opening that can be built today"
            ));
        }
        let listed = required(item, "pieces")?
            .as_list()
            .ok_or_else(|| format!("the pocket {name} lists the pieces it is made of"))?;
        let mut parts = Vec::with_capacity(listed.len());
        for part in listed {
            let part = part
                .as_text()
                .ok_or_else(|| format!("the pocket {name} names its pieces"))?;
            if !pieces.iter().any(|drawn| drawn.name == part) {
                return Err(format!(
                    "the pocket {name} is made of {part}, and no piece has that name"
                ));
            }
            parts.push(part.to_owned());
        }
        if parts.is_empty() {
            return Err(format!("the pocket {name} is made of nothing"));
        }
        let (edges, mirrored) = tables.get(piece).ok_or_else(|| {
            format!("the pocket {name} is on {piece}, and no piece has that name")
        })?;
        let what = format!("the pocket {name}");
        let start = edges.held(lengths, item, &what)?;
        let along = length(lengths, text("length")?)?;
        // It runs on along the edge from where it starts, or, when that is written back from the edge's end, on back toward its start.
        let run = if item.contains_key("before_end")
            || item.get("runs").and_then(Value::as_text) == Some("back")
        {
            -along
        } else {
            along
        };
        let stop = edges.beside(&start, run, &what)?;
        let answer = edges.answer(&start)?;
        let from = book.settle(
            piece,
            &format!("pocket {name}, from"),
            &start,
            answer,
            *mirrored,
        )?;
        let answer = edges.answer(&stop)?;
        let to = book.settle(
            piece,
            &format!("pocket {name}, to"),
            &stop,
            answer,
            *mirrored,
        )?;
        if let (Some(from), Some(to)) = (from, to) {
            found.push(Pocket {
                name: name.to_owned(),
                piece: piece.to_owned(),
                from: from.point,
                to: to.point,
                opening: opening.to_owned(),
                pieces: parts,
            });
        }
    }
    found.sort();
    Ok(found)
}

// A button and its hole. The hole's length is the button's size plus what the policy names, and it cannot be given any other way: that is what stops the two drifting apart.
fn buttons_of(
    lengths: &Lengths,
    description: &Fields,
    pieces: &mut [DrawnPiece],
) -> Result<Vec<ButtonPair>, String> {
    let mut seen = Vec::new();
    let mut found = Vec::new();
    for item in description
        .get("buttons")
        .and_then(Value::as_list)
        .unwrap_or(&[])
    {
        let item = item.as_object().ok_or("a button must be an object")?;
        let name = named_item(item, "button", &mut seen)?.to_owned();
        let hole = required(item, "hole")?
            .as_object()
            .ok_or_else(|| format!("the button {name} says where its hole is"))?;
        if item.contains_key("hole_length") || hole.contains_key("length") {
            return Err(format!(
                "the button {name} gives its hole a length, and a hole's length comes from its button"
            ));
        }
        let button = required(item, "button")?
            .as_object()
            .ok_or_else(|| format!("the button {name} says where it goes"))?;
        let size = length(
            lengths,
            required(item, "size")?
                .as_text()
                .ok_or_else(|| format!("the button {name}: size names a length"))?,
        )?;
        let mut hole_length = size;
        for part in required(item, "policy")?
            .as_list()
            .ok_or_else(|| format!("the button {name} lists what its hole adds to it"))?
        {
            hole_length += length(
                lengths,
                part.as_text()
                    .ok_or_else(|| format!("the button {name}: its policy names lengths"))?,
            )?;
        }
        if size <= 0 || hole_length <= 0 {
            return Err(format!(
                "the button {name} comes to a size of {size} and a hole of {hole_length}"
            ));
        }
        let piece_named = |side: &Fields| -> Result<String, String> {
            Ok(required(side, "piece")?
                .as_text()
                .ok_or_else(|| format!("the button {name} names its pieces"))?
                .to_owned())
        };
        let (button_piece, hole_piece) = (piece_named(button)?, piece_named(hole)?);
        let button_at = point(lengths, required(button, "at")?)?;
        let hole_from = point(lengths, required(hole, "at")?)?;
        let aim = point(lengths, required(hole, "toward")?)?;
        let (dx, dy) = (
            ratio(aim.x() - hole_from.x()),
            ratio(aim.y() - hole_from.y()),
        );
        let reach = pure::hypot(dx, dy);
        if reach == 0.0 {
            return Err(format!("the button {name}'s hole aims at where it starts"));
        }
        let hole_to = Point::new(
            hole_from.x() + whole(dx / reach * ratio(hole_length)),
            hole_from.y() + whole(dy / reach * ratio(hole_length)),
        )
        .map_err(|error| error.to_string())?;
        let on = |wanted: &str, pieces: &[DrawnPiece], at: Point, what: &str| {
            let drawn = pieces
                .iter()
                .find(|drawn| drawn.name == wanted)
                .ok_or_else(|| {
                    format!("the button {name} is on {wanted}, and no piece has that name")
                })?;
            if inside(&drawn.sew, at) {
                Ok(())
            } else {
                Err(format!(
                    "the button {name}'s {what} is at ({}, {}), outside the sew line of {wanted}",
                    at.x(),
                    at.y()
                ))
            }
        };
        on(&button_piece, pieces, button_at, "button")?;
        on(&hole_piece, pieces, hole_from, "hole")?;
        on(&hole_piece, pieces, hole_to, "hole")?;
        if let Some(drawn) = pieces.iter_mut().find(|drawn| drawn.name == hole_piece) {
            drawn.internal.push(Line {
                from: hole_from,
                to: hole_to,
            });
        }
        if let Some(drawn) = pieces.iter_mut().find(|drawn| drawn.name == button_piece) {
            drawn.marks.push(Mark {
                kind: MarkKind::Drill,
                at: button_at,
                toward: button_at,
            });
        }
        found.push(ButtonPair {
            name,
            size,
            hole_length,
            button_piece,
            button_at,
            hole_piece,
            hole_from,
            hole_to,
        });
    }
    found.sort();
    Ok(found)
}

// What the drawing solved for, as the lengths it was given. A fit on a seam with nowhere to go waits, and says so.
fn fits_of(lengths: &Lengths, description: &Fields, book: &mut Book) -> Result<Vec<Fit>, String> {
    let mut found = Vec::new();
    for item in description
        .get("fits")
        .and_then(Value::as_list)
        .unwrap_or(&[])
    {
        let item = item.as_object().ok_or("a fit must be an object")?;
        let text = |name: &str| -> Result<&str, String> {
            required(item, name)?
                .as_text()
                .ok_or_else(|| format!("a fit's {name} is a name"))
        };
        let (vary, seam) = (text("vary")?, text("seam")?);
        if book.is_open(seam) {
            book.waiting("a fit", seam);
            continue;
        }
        found.push(Fit {
            vary: vary.to_owned(),
            seam: seam.to_owned(),
            value: length(lengths, vary)?,
        });
    }
    Ok(found)
}

/// What a seam has that it does not declare, drawing only the pieces it joins. `None` when an end of it has nowhere to go.
///
/// # Errors
///
/// What `draft` refuses, and a seam that does not exist.
pub fn seam_gap(
    lengths: &Lengths,
    description: &Fields,
    seam: &str,
) -> Result<Option<i64>, String> {
    let seams = description
        .get("seams")
        .and_then(Value::as_list)
        .unwrap_or(&[]);
    let item = seams
        .iter()
        .find(|item| {
            item.as_object()
                .and_then(|item| item.get("name"))
                .and_then(Value::as_text)
                == Some(seam)
        })
        .ok_or_else(|| format!("a fit decides by the seam {seam}, and no seam has that name"))?;
    let side_piece = |side: &str| {
        item.as_object()
            .and_then(|item| item.get(side))
            .and_then(Value::as_object)
            .and_then(|side| side.get("piece"))
            .and_then(Value::as_text)
            .map(str::to_owned)
    };
    let mut wanted: Vec<String> = [side_piece("a"), side_piece("b")]
        .into_iter()
        .flatten()
        .collect();
    let listed = required(description, "pieces")?
        .as_list()
        .ok_or("pieces must be a list")?;
    // A mirror needs the piece it mirrors, which comes before it.
    for piece in listed.iter().rev().filter_map(Value::as_object) {
        let name = piece.get("name").and_then(Value::as_text);
        if name.is_some_and(|name| wanted.iter().any(|other| other == name)) {
            if let Some(source) = piece.get("mirror_of").and_then(Value::as_text) {
                wanted.push(source.to_owned());
            }
        }
    }
    let kept: Vec<Value> = listed
        .iter()
        .filter(|piece| {
            piece
                .as_object()
                .and_then(|piece| piece.get("name"))
                .and_then(Value::as_text)
                .is_some_and(|name| wanted.iter().any(|other| other == name))
        })
        .cloned()
        .collect();
    let mut reduced = Fields::new();
    for name in ["style", "size"] {
        reduced.insert(name.to_owned(), required(description, name)?.clone());
    }
    reduced.insert("pieces".to_owned(), Value::List(kept));
    reduced.insert("seams".to_owned(), Value::List(vec![item.clone()]));
    let drawn = draw(lengths, &reduced, "", "", &BTreeMap::new())?;
    if drawn.open_seams.iter().any(|open| open == seam) {
        return Ok(None);
    }
    let report =
        walk(&drawn.pattern).map_err(|error| format!("{}: {}", error.seam, error.message))?;
    Ok(report
        .iter()
        .find(|item| item.seam == seam)
        .map(|item| item.unexplained))
}

/// The value between two lengths that leaves a seam nothing it does not declare, by false position with integers, so that it is the same answer on every machine.
///
/// # Errors
///
/// Whatever trying a value refuses, and a seam that nothing between the two can make walk.
pub fn solve_fit(
    (vary, seam): (&str, &str),
    (low, high): (i64, i64),
    gap: &mut dyn FnMut(i64) -> Result<Option<i64>, String>,
) -> Result<Option<i64>, String> {
    // A flattened curve's length is good to T2, so there is no more to be had than this.
    const CLOSE_ENOUGH: i128 = 2;
    // Each try draws two pieces, and more than this means the seam does not follow the input smoothly at all.
    const MOST_TRIES: u32 = 128;
    if low >= high {
        return Err(format!(
            "a fit varies {vary} between {low} and {high}, which is no range"
        ));
    }
    let (Some(at_low), Some(at_high)) = (gap(low)?, gap(high)?) else {
        return Ok(None);
    };
    if (at_low > 0) == (at_high > 0) && at_low != 0 && at_high != 0 {
        return Err(format!(
            "no value of {vary} between {low} and {high} gives the seam {seam} what it declares: it is out by {at_low} at one end and {at_high} at the other"
        ));
    }
    let (mut a, mut b) = (low, high);
    let (mut gap_a, mut gap_b) = (i128::from(at_low), i128::from(at_high));
    let (mut best, mut least) = if gap_a.abs() <= gap_b.abs() {
        (a, gap_a.abs())
    } else {
        (b, gap_b.abs())
    };
    let mut kept_a = None;
    let mut tries = 0;
    while least > CLOSE_ENOUGH && b - a > 1 && tries < MOST_TRIES {
        tries += 1;
        let span = i128::from(b - a);
        let guess = span * gap_a.abs() / (gap_a.abs() + gap_b.abs()).max(1);
        let at = a + i64::try_from(guess).map_err(|error| error.to_string())?;
        let at = at.clamp(a + 1, b - 1);
        let Some(found) = gap(at)? else {
            return Ok(None);
        };
        let found = i128::from(found);
        if found.abs() < least {
            (best, least) = (at, found.abs());
        }
        // An end kept twice in a row has its weight halved, which is what stops this creeping in from one side.
        if (found > 0) == (gap_a > 0) {
            (a, gap_a) = (at, found);
            if kept_a == Some(false) {
                gap_b /= 2;
            }
            kept_a = Some(false);
        } else {
            (b, gap_b) = (at, found);
            if kept_a == Some(true) {
                gap_a /= 2;
            }
            kept_a = Some(true);
        }
    }
    if least > i128::from(WALK_TOLERANCE) {
        return Err(format!(
            "no value of {vary} between {low} and {high} gives the seam {seam} what it declares: the closest leaves it out by {least}"
        ));
    }
    Ok(Some(best))
}

// A drawing, and what each reference in it came to.
struct Drawing {
    pattern: Pattern,
    references: Vec<Value>,
    open_seams: Vec<String>,
}

fn draw(
    lengths: &Lengths,
    description: &Fields,
    design: &str,
    revision: &str,
    moves: &Moves,
) -> Result<Drawing, String> {
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
    let mut tables: Tables = BTreeMap::new();
    let mut book = Book::default();
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
            if let Some((edges, mirrored)) = tables.get(source) {
                let copy = (edges.clone(), !*mirrored);
                tables.insert(name.clone(), copy);
            }
            DrawnPiece {
                quantity,
                ..mirror(original, &name, labels(lengths, fields, &size)?)?
            }
        } else {
            let moved_corners = moves.get(&name).map_or(&[][..], Vec::as_slice);
            let (edges, spreads) = boundary(lengths, fields, &name, moved_corners)
                .map_err(|error| format!("{name}: {error}"))?;
            let net = Outline::new(edges.curves.clone())
                .map_err(|error| error.to_string())
                .map_err(|error| format!("{name}: {error}"))?;
            let offset = net
                .offset(&edges.allowances, TOLERANCE)
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
            let notches = notches_of(lengths, fields, &edges, &name, &mut cut, &moved, &mut book)
                .map_err(|error| format!("{name}: {error}"))?;
            let grade_points = grade_points_of(lengths, fields, &edges, &name, &mut book)
                .map_err(|error| format!("{name}: {error}"))?;
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
            tables.insert(name.clone(), (edges, false));
            DrawnPiece {
                name,
                size: size.clone(),
                quantity,
                cut,
                sew,
                grain: *grain,
                internal,
                marks: notches,
                labels: labels(lengths, fields, &size)?,
                closures,
                grade_points,
            }
        };
        pieces.push(piece);
    }
    let seams = seams_of(lengths, description, &tables, moves, &mut book)?;
    balance_notches(lengths, description, &mut pieces, &seams, &mut book)?;
    let mut fits = fits_of(lengths, description, &mut book)?;
    fits.sort();
    let pockets = pockets_of(lengths, description, &tables, &pieces, &mut book)?;
    let buttons = buttons_of(lengths, description, &mut pieces)?;
    let open_seams = description
        .get("seams")
        .and_then(Value::as_list)
        .unwrap_or(&[])
        .iter()
        .filter_map(|item| item.as_object()?.get("name")?.as_text())
        .filter(|name| book.is_open(name))
        .map(str::to_owned)
        .collect();
    // In the order the canonical form writes them, so that what is read back is what was drawn.
    book.repairs.sort();
    Ok(Drawing {
        pattern: Pattern {
            style,
            design: design.to_owned(),
            revision: revision.to_owned(),
            pieces,
            seams,
            fits,
            pockets,
            buttons,
            repairs: book.repairs,
        },
        references: book.references,
        open_seams,
    })
}
