//! Whether two patterns differ in meaning, with bytes as a second question. A change of formatting is not a change of pattern, and a
//! change of a notch is, however small its effect on the bytes.
use crate::canonical::{from_canonical_text, to_canonical_text, CanonicalError};
use crate::pattern::{DrawnPiece, Line, Mark, Pattern};
use sc_geometry::Point;

/// One way two patterns differ.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Difference {
    /// Which piece, or empty for the pattern itself.
    pub piece: String,
    /// What differs, in a stable token.
    pub field: &'static str,
    /// The two values, in words.
    pub detail: String,
}

/// What a comparison found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Comparison {
    /// The same bytes.
    Identical,
    /// Different bytes, and the same meaning: the text's whitespace or key order differs.
    FormattingOnly,
    /// The same meaning within the tolerance, and points that moved by less than it.
    WithinTolerance {
        /// The largest move, in micrometres.
        largest_move: i64,
    },
    /// A difference in meaning.
    Different(Vec<Difference>),
}

fn distance(a: Point, b: Point) -> i64 {
    (a.x() - b.x()).abs().max((a.y() - b.y()).abs())
}

fn lowest_first(points: &[Point]) -> Vec<Point> {
    let start = points
        .iter()
        .enumerate()
        .min_by_key(|(_, at)| (at.y(), at.x()))
        .map_or(0, |(at, _)| at);
    let mut rotated = points.to_vec();
    rotated.rotate_left(start);
    rotated
}

fn compare_points(
    a: &[Point],
    b: &[Point],
    tolerance: i64,
    largest: &mut i64,
) -> Result<(), String> {
    if a.len() != b.len() {
        return Err(format!("{} points against {}", a.len(), b.len()));
    }
    for (index, (first, second)) in a.iter().zip(b).enumerate() {
        let moved = distance(*first, *second);
        if moved > tolerance {
            return Err(format!(
                "point {index} is ({}, {}) against ({}, {}), {moved} micrometres apart",
                first.x(),
                first.y(),
                second.x(),
                second.y()
            ));
        }
        *largest = (*largest).max(moved);
    }
    Ok(())
}

fn compare_piece(
    a: &DrawnPiece,
    b: &DrawnPiece,
    tolerance: i64,
    largest: &mut i64,
    out: &mut Vec<Difference>,
) {
    let mut report = |field: &'static str, detail: String| {
        out.push(Difference {
            piece: a.name.clone(),
            field,
            detail,
        })
    };
    if a.quantity != b.quantity {
        report("quantity", format!("{} against {}", a.quantity, b.quantity));
    }
    if let Err(detail) = compare_points(
        &lowest_first(&a.cut),
        &lowest_first(&b.cut),
        tolerance,
        largest,
    ) {
        report("cut", detail);
    }
    if let Err(detail) = compare_points(
        &lowest_first(&a.sew),
        &lowest_first(&b.sew),
        tolerance,
        largest,
    ) {
        report("sew", detail);
    }
    let line_points = |line: &Line| [line.from, line.to];
    if let Err(detail) = compare_points(
        &line_points(&a.grain),
        &line_points(&b.grain),
        tolerance,
        largest,
    ) {
        report("grain", detail);
    }
    let sorted_lines = |lines: &[Line]| {
        let mut sorted = lines.to_vec();
        sorted.sort();
        sorted.iter().flat_map(line_points).collect::<Vec<Point>>()
    };
    if let Err(detail) = compare_points(
        &sorted_lines(&a.internal),
        &sorted_lines(&b.internal),
        tolerance,
        largest,
    ) {
        report("internal", detail);
    }
    let sorted_marks = |marks: &[Mark]| {
        let mut sorted = marks.to_vec();
        sorted.sort();
        sorted
    };
    let (first, second) = (sorted_marks(&a.marks), sorted_marks(&b.marks));
    if first
        .iter()
        .map(|mark| mark.kind)
        .ne(second.iter().map(|mark| mark.kind))
    {
        report(
            "marks",
            format!("{} marks against {}", first.len(), second.len()),
        );
    } else {
        let at = |marks: &[Mark]| {
            marks
                .iter()
                .flat_map(|mark| [mark.at, mark.toward])
                .collect::<Vec<Point>>()
        };
        if let Err(detail) = compare_points(&at(&first), &at(&second), tolerance, largest) {
            report("marks", detail);
        }
    }
    let closures = |piece: &DrawnPiece| {
        let mut listed = piece.closures.clone();
        listed.sort();
        listed
    };
    if closures(a) != closures(b) {
        report(
            "closures",
            format!("{:?} against {:?}", closures(a), closures(b)),
        );
    }
    let label_text = |piece: &DrawnPiece| {
        let mut texts: Vec<String> = piece
            .labels
            .iter()
            .map(|label| label.text.clone())
            .collect();
        texts.sort();
        texts
    };
    if label_text(a) != label_text(b) {
        report(
            "labels",
            format!("{:?} against {:?}", label_text(a), label_text(b)),
        );
    }
}

fn compare_patterns(
    a: &Pattern,
    b: &Pattern,
    tolerance: i64,
    largest: &mut i64,
) -> Vec<Difference> {
    let mut out = Vec::new();
    let mut top = |field: &'static str, first: &str, second: &str| {
        if first != second {
            out.push(Difference {
                piece: String::new(),
                field,
                detail: format!("{first} against {second}"),
            });
        }
    };
    top("style", &a.style, &b.style);
    top("design", &a.design, &b.design);
    top("revision", &a.revision, &b.revision);
    for piece in &a.pieces {
        match b
            .pieces
            .iter()
            .find(|other| other.name == piece.name && other.size == piece.size)
        {
            Some(other) => compare_piece(piece, other, tolerance, largest, &mut out),
            None => out.push(Difference {
                piece: piece.name.clone(),
                field: "missing",
                detail: "the second pattern does not have this piece".to_owned(),
            }),
        }
    }
    for piece in &b.pieces {
        if !a
            .pieces
            .iter()
            .any(|other| other.name == piece.name && other.size == piece.size)
        {
            out.push(Difference {
                piece: piece.name.clone(),
                field: "added",
                detail: "the first pattern does not have this piece".to_owned(),
            });
        }
    }
    out
}

/// Compares two canonical texts: bytes first, then meaning, with points allowed to move by up to `tolerance` micrometres.
///
/// # Errors
/// Either text is not a canonical pattern.
pub fn compare(first: &str, second: &str, tolerance: i64) -> Result<Comparison, CanonicalError> {
    if first == second {
        return Ok(Comparison::Identical);
    }
    let (a, b) = (from_canonical_text(first)?, from_canonical_text(second)?);
    if to_canonical_text(&a) == to_canonical_text(&b) {
        return Ok(Comparison::FormattingOnly);
    }
    let mut largest = 0;
    let differences = compare_patterns(&a, &b, tolerance, &mut largest);
    if differences.is_empty() {
        Ok(Comparison::WithinTolerance {
            largest_move: largest,
        })
    } else {
        Ok(Comparison::Different(differences))
    }
}
