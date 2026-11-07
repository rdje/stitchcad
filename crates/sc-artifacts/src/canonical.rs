//! One written form for one pattern: stable order and exact numbers, with no clock anywhere in it.
use crate::pattern::{
    Closure, ClosureKind, DrawnPiece, GradePoint, Line, Mark, MarkKind, Pattern, Repair, Seam,
    SeamSide, Text,
};
use core::fmt;
use sc_geometry::Point;
use sc_store::{parse, Value};
use std::collections::BTreeMap;

const FORMAT: i64 = 1;

fn object(fields: Vec<(&str, Value)>) -> Value {
    Value::Object(
        fields
            .into_iter()
            .map(|(key, value)| (key.to_owned(), value))
            .collect(),
    )
}

fn text(value: &str) -> Value {
    Value::Text(value.to_owned())
}

fn point(at: Point) -> Value {
    Value::List(vec![Value::Int(at.x()), Value::Int(at.y())])
}

fn line(at: Line) -> Value {
    Value::List(vec![point(at.from), point(at.to)])
}

fn kind_name(kind: MarkKind) -> &'static str {
    match kind {
        MarkKind::VNotch => "v_notch",
        MarkKind::SlitNotch => "slit_notch",
        MarkKind::Drill => "drill",
    }
}

// A closed polyline has no first point of its own, so one is chosen: the lowest, then the leftmost.
pub(crate) fn from_lowest(points: &[Point]) -> Vec<Point> {
    let start = points
        .iter()
        .enumerate()
        .min_by_key(|(_, at)| (at.y(), at.x()))
        .map_or(0, |(at, _)| at);
    let mut rotated = points.to_vec();
    rotated.rotate_left(start);
    rotated
}

fn piece_value(piece: &DrawnPiece) -> Value {
    let points = |list: &[Point]| Value::List(from_lowest(list).into_iter().map(point).collect());
    let mut internal = piece.internal.clone();
    internal.sort();
    let mut marks = piece.marks.clone();
    marks.sort();
    let mut labels = piece.labels.clone();
    labels.sort();
    let mut closures = piece.closures.clone();
    closures.sort();
    let mut fields = vec![
        ("name", text(&piece.name)),
        ("size", text(&piece.size)),
        ("quantity", Value::Int(i64::from(piece.quantity))),
        ("cut", points(&piece.cut)),
        ("sew", points(&piece.sew)),
        ("grain", line(piece.grain)),
        (
            "internal",
            Value::List(internal.into_iter().map(line).collect()),
        ),
        (
            "marks",
            Value::List(
                marks
                    .into_iter()
                    .map(|mark| {
                        object(vec![
                            ("kind", text(kind_name(mark.kind))),
                            ("at", point(mark.at)),
                            ("toward", point(mark.toward)),
                        ])
                    })
                    .collect(),
            ),
        ),
        (
            "labels",
            Value::List(
                labels
                    .into_iter()
                    .map(|label| object(vec![("at", point(label.at)), ("text", text(&label.text))]))
                    .collect(),
            ),
        ),
    ];
    // A piece with no closures writes no field, so a form written before closures existed still reads the same.
    if !closures.is_empty() {
        fields.push((
            "closures",
            Value::List(closures.iter().map(closure_value).collect()),
        ));
    }
    let mut grade_points = piece.grade_points.clone();
    grade_points.sort();
    if !grade_points.is_empty() {
        fields.push((
            "grade_points",
            Value::List(
                grade_points
                    .iter()
                    .map(|item| object(vec![("name", text(&item.name)), ("at", point(item.at))]))
                    .collect(),
            ),
        ));
    }
    object(fields)
}

fn side_value(side: &SeamSide) -> Value {
    object(vec![
        ("piece", text(&side.piece)),
        ("from", point(side.from)),
        ("to", point(side.to)),
    ])
}

fn seam_value(seam: &Seam) -> Value {
    object(vec![
        ("name", text(&seam.name)),
        ("a", side_value(&seam.a)),
        ("b", side_value(&seam.b)),
        ("ease", Value::Int(seam.ease)),
    ])
}

fn closure_kind_name(kind: ClosureKind) -> &'static str {
    match kind {
        ClosureKind::Dart => "dart",
        ClosureKind::Tuck => "tuck",
        ClosureKind::Pleat => "pleat",
        ClosureKind::Gather => "gather",
    }
}

fn closure_value(closure: &Closure) -> Value {
    let mut fields = vec![
        ("kind", text(closure_kind_name(closure.kind))),
        ("name", text(&closure.name)),
        ("from", point(closure.from)),
        ("to", point(closure.to)),
        ("intake", Value::Int(closure.intake)),
    ];
    if let Some(apex) = closure.apex {
        fields.push(("apex", point(apex)));
    }
    object(fields)
}

/// The one written form: pieces in name then size order, each closed line from its lowest point, and marks in a fixed order.
#[must_use]
pub fn to_canonical_text(pattern: &Pattern) -> String {
    let mut pieces: Vec<&DrawnPiece> = pattern.pieces.iter().collect();
    pieces.sort_by(|a, b| (&a.name, &a.size).cmp(&(&b.name, &b.size)));
    let mut fields = vec![
        ("format", Value::Int(FORMAT)),
        ("style", text(&pattern.style)),
        ("design", text(&pattern.design)),
        ("revision", text(&pattern.revision)),
        (
            "pieces",
            Value::List(pieces.into_iter().map(piece_value).collect()),
        ),
    ];
    // A pattern with no sewing graph writes no field, so a form written before there was one still reads the same.
    if !pattern.repairs.is_empty() {
        let mut repairs = pattern.repairs.clone();
        repairs.sort();
        fields.push((
            "repairs",
            Value::List(repairs.iter().map(Repair::to_value).collect()),
        ));
    }
    if !pattern.seams.is_empty() {
        let mut seams = pattern.seams.clone();
        seams.sort();
        fields.push(("seams", Value::List(seams.iter().map(seam_value).collect())));
    }
    object(fields).to_text()
}

/// Why a text is not a canonical pattern.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanonicalError {
    /// What is wrong.
    pub message: String,
}

impl fmt::Display for CanonicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "canonical pattern: {}", self.message)
    }
}

impl std::error::Error for CanonicalError {}

fn fail<T>(message: impl Into<String>) -> Result<T, CanonicalError> {
    Err(CanonicalError {
        message: message.into(),
    })
}

fn field<'a>(fields: &'a BTreeMap<String, Value>, name: &str) -> Result<&'a Value, CanonicalError> {
    fields.get(name).ok_or_else(|| CanonicalError {
        message: format!("no {name} field"),
    })
}

fn read_text(fields: &BTreeMap<String, Value>, name: &str) -> Result<String, CanonicalError> {
    field(fields, name)?
        .as_text()
        .map(str::to_owned)
        .ok_or_else(|| CanonicalError {
            message: format!("{name} must be text"),
        })
}

fn read_list<'a>(
    fields: &'a BTreeMap<String, Value>,
    name: &str,
) -> Result<&'a [Value], CanonicalError> {
    field(fields, name)?
        .as_list()
        .ok_or_else(|| CanonicalError {
            message: format!("{name} must be a list"),
        })
}

fn read_point(value: &Value) -> Result<Point, CanonicalError> {
    match value.as_list() {
        Some([x, y]) => match (x.as_int(), y.as_int()) {
            (Some(x), Some(y)) => Point::new(x, y).map_err(|error| CanonicalError {
                message: error.to_string(),
            }),
            _ => fail("a point's coordinates must be whole numbers"),
        },
        _ => fail("a point must be two numbers"),
    }
}

fn read_line(value: &Value) -> Result<Line, CanonicalError> {
    match value.as_list() {
        Some([from, to]) => Ok(Line {
            from: read_point(from)?,
            to: read_point(to)?,
        }),
        _ => fail("a line must be two points"),
    }
}

fn read_piece(value: &Value) -> Result<DrawnPiece, CanonicalError> {
    let Some(fields) = value.as_object() else {
        return fail("a piece must be an object");
    };
    let points = |name: &str| -> Result<Vec<Point>, CanonicalError> {
        read_list(fields, name)?.iter().map(read_point).collect()
    };
    let quantity = field(fields, "quantity")?
        .as_int()
        .and_then(|number| u32::try_from(number).ok())
        .ok_or_else(|| CanonicalError {
            message: "quantity must be a small whole number".to_owned(),
        })?;
    let mut marks = Vec::new();
    for item in read_list(fields, "marks")? {
        let Some(mark) = item.as_object() else {
            return fail("a mark must be an object");
        };
        let kind = match read_text(mark, "kind")?.as_str() {
            "v_notch" => MarkKind::VNotch,
            "slit_notch" => MarkKind::SlitNotch,
            "drill" => MarkKind::Drill,
            other => return fail(format!("no mark is called {other}")),
        };
        marks.push(Mark {
            kind,
            at: read_point(field(mark, "at")?)?,
            toward: read_point(field(mark, "toward")?)?,
        });
    }
    let mut labels = Vec::new();
    for item in read_list(fields, "labels")? {
        let Some(label) = item.as_object() else {
            return fail("a label must be an object");
        };
        labels.push(Text {
            at: read_point(field(label, "at")?)?,
            text: read_text(label, "text")?,
        });
    }
    let mut closures = Vec::new();
    if fields.contains_key("closures") {
        for item in read_list(fields, "closures")? {
            let Some(closure) = item.as_object() else {
                return fail("a closure must be an object");
            };
            let kind = match read_text(closure, "kind")?.as_str() {
                "dart" => ClosureKind::Dart,
                "tuck" => ClosureKind::Tuck,
                "pleat" => ClosureKind::Pleat,
                "gather" => ClosureKind::Gather,
                other => return fail(format!("no closure is called {other}")),
            };
            closures.push(Closure {
                kind,
                name: read_text(closure, "name")?,
                from: read_point(field(closure, "from")?)?,
                to: read_point(field(closure, "to")?)?,
                apex: closure.get("apex").map(read_point).transpose()?,
                intake: field(closure, "intake")?
                    .as_int()
                    .ok_or_else(|| CanonicalError {
                        message: "a closure's intake must be a whole number".to_owned(),
                    })?,
            });
        }
    }
    let mut grade_points = Vec::new();
    if fields.contains_key("grade_points") {
        for item in read_list(fields, "grade_points")? {
            let Some(item) = item.as_object() else {
                return fail("a grade point must be an object");
            };
            grade_points.push(GradePoint {
                name: read_text(item, "name")?,
                at: read_point(field(item, "at")?)?,
            });
        }
    }
    Ok(DrawnPiece {
        closures,
        grade_points,
        name: read_text(fields, "name")?,
        size: read_text(fields, "size")?,
        quantity,
        cut: points("cut")?,
        sew: points("sew")?,
        grain: read_line(field(fields, "grain")?)?,
        internal: read_list(fields, "internal")?
            .iter()
            .map(read_line)
            .collect::<Result<_, _>>()?,
        marks,
        labels,
    })
}

fn read_side(value: &Value) -> Result<SeamSide, CanonicalError> {
    let Some(fields) = value.as_object() else {
        return fail("a seam's side must be an object");
    };
    Ok(SeamSide {
        piece: read_text(fields, "piece")?,
        from: read_point(field(fields, "from")?)?,
        to: read_point(field(fields, "to")?)?,
    })
}

fn read_seam(value: &Value) -> Result<Seam, CanonicalError> {
    let Some(fields) = value.as_object() else {
        return fail("a seam must be an object");
    };
    Ok(Seam {
        name: read_text(fields, "name")?,
        a: read_side(field(fields, "a")?)?,
        b: read_side(field(fields, "b")?)?,
        ease: field(fields, "ease")?
            .as_int()
            .ok_or_else(|| CanonicalError {
                message: "a seam's ease must be a whole number".to_owned(),
            })?,
    })
}

/// A pattern from its canonical text.
///
/// # Errors
/// The text does not parse, declares another format, or lacks a field.
pub fn from_canonical_text(source: &str) -> Result<Pattern, CanonicalError> {
    let value = parse(source).map_err(|error| CanonicalError {
        message: error.to_string(),
    })?;
    let Some(fields) = value.as_object() else {
        return fail("it must be an object");
    };
    if field(fields, "format")?.as_int() != Some(FORMAT) {
        return fail(format!("this version reads format {FORMAT}"));
    }
    Ok(Pattern {
        style: read_text(fields, "style")?,
        design: read_text(fields, "design")?,
        revision: read_text(fields, "revision")?,
        pieces: read_list(fields, "pieces")?
            .iter()
            .map(read_piece)
            .collect::<Result<_, _>>()?,
        seams: if fields.contains_key("seams") {
            read_list(fields, "seams")?
                .iter()
                .map(read_seam)
                .collect::<Result<_, _>>()?
        } else {
            Vec::new()
        },
        repairs: if fields.contains_key("repairs") {
            read_list(fields, "repairs")?
                .iter()
                .map(read_repair)
                .collect::<Result<_, _>>()?
        } else {
            Vec::new()
        },
    })
}

fn read_repair(item: &Value) -> Result<Repair, CanonicalError> {
    let Some(item) = item.as_object() else {
        return fail("a repair must be an object");
    };
    let mut candidates = Vec::new();
    for candidate in read_list(item, "candidates")? {
        let Some(candidate) = candidate.as_text() else {
            return fail("a repair's candidate must be text");
        };
        candidates.push(candidate.to_owned());
    }
    Ok(Repair {
        piece: read_text(item, "piece")?,
        carrier: read_text(item, "carrier")?,
        reference: read_text(item, "reference")?,
        orphaned_by: read_text(item, "orphaned_by")?,
        candidates,
    })
}
