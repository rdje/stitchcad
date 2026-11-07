//! Closing a closure, and what it removes. A closure carries its intake, and this checks that against the drawn line, because a declared number nobody can falsify is decoration.
use crate::pattern::{Closure, ClosureKind, DrawnPiece, Pattern};
use sc_geometry::Point;
use sc_store::Value;
use sc_units::pure;
use std::collections::BTreeMap;

/// What closing one closure does to the drawn piece.
#[derive(Clone, Debug, PartialEq)]
pub struct ClosureReport {
    /// The piece it is on.
    pub piece: String,
    /// Its name.
    pub name: String,
    /// What kind it is.
    pub kind: ClosureKind,
    /// The intake it carries, in micrometres.
    pub intake: i64,
    /// The sew line's length with the closure open.
    pub open_length: i64,
    /// The sew line's length with it closed: the open length less what it takes up.
    pub closed_length: i64,
    /// How far the far leg's top lands from the near leg's when a dart is really closed, by turning it about the apex. Zero for a closure with no apex.
    pub gap_after_closing: i64,
    /// Whether what closing removes is the intake, within the T1 class.
    pub conserved: bool,
}

impl ClosureReport {
    /// The report as a value.
    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut fields = BTreeMap::new();
        fields.insert("piece".to_owned(), Value::Text(self.piece.clone()));
        fields.insert("name".to_owned(), Value::Text(self.name.clone()));
        let kind = match self.kind {
            ClosureKind::Dart => "dart",
            ClosureKind::Tuck => "tuck",
            ClosureKind::Pleat => "pleat",
            ClosureKind::Gather => "gather",
        };
        fields.insert("kind".to_owned(), Value::Text(kind.to_owned()));
        fields.insert("intake".to_owned(), Value::Int(self.intake));
        fields.insert("open_length".to_owned(), Value::Int(self.open_length));
        fields.insert("closed_length".to_owned(), Value::Int(self.closed_length));
        fields.insert(
            "gap_after_closing".to_owned(),
            Value::Int(self.gap_after_closing),
        );
        fields.insert("conserved".to_owned(), Value::Bool(self.conserved));
        Value::Object(fields)
    }
}

// T1 of the units chapter: values that should be identical by construction.
const T1: i64 = 1;

#[allow(clippy::cast_precision_loss)]
fn distance(a: Point, b: Point) -> f64 {
    pure::hypot((b.x() - a.x()) as f64, (b.y() - a.y()) as f64)
}

/// How far along a closed line a point sits, in micrometres from its first point, or nothing when the point is more than T2 from the line.
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn along(line: &[Point], at: Point) -> Option<f64> {
    let mut walked = 0.0;
    let mut best: Option<(f64, f64)> = None;
    for (index, a) in line.iter().enumerate() {
        let b = line.get((index + 1) % line.len())?;
        let (dx, dy) = ((b.x() - a.x()) as f64, (b.y() - a.y()) as f64);
        let length = pure::hypot(dx, dy);
        if length > 0.0 {
            let t = (((at.x() - a.x()) as f64 * dx + (at.y() - a.y()) as f64 * dy)
                / (length * length))
                .clamp(0.0, 1.0);
            let away = pure::hypot(
                (at.x() - a.x()) as f64 - t * dx,
                (at.y() - a.y()) as f64 - t * dy,
            );
            if best.is_none_or(|(least, _)| away < least) {
                best = Some((away, walked + t * length));
            }
        }
        walked += length;
    }
    best.filter(|(away, _)| *away <= 10.0)
        .map(|(_, parameter)| parameter)
}

#[allow(clippy::cast_precision_loss)]
fn length_of(line: &[Point]) -> f64 {
    line.iter()
        .enumerate()
        .filter_map(|(index, a)| line.get((index + 1) % line.len()).map(|b| distance(*a, *b)))
        .sum()
}

// A dart is closed by turning its far leg about the apex onto its near one. What is left between their tops is the gap.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn gap_after_closing(closure: &Closure) -> i64 {
    let Some(apex) = closure.apex else {
        return 0;
    };
    let (ax, ay) = (
        (closure.from.x() - apex.x()) as f64,
        (closure.from.y() - apex.y()) as f64,
    );
    let (bx, by) = (
        (closure.to.x() - apex.x()) as f64,
        (closure.to.y() - apex.y()) as f64,
    );
    let turn = pure::atan2(bx * ay - by * ax, bx * ax + by * ay);
    let (sine, cosine) = pure::sin_cos(turn);
    let (x, y) = (bx * cosine - by * sine, bx * sine + by * cosine);
    pure::hypot(ax - x, ay - y).round() as i64
}

#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn report(piece: &DrawnPiece, closure: &Closure) -> ClosureReport {
    let open = length_of(&piece.sew);
    // What it takes up is the shorter way along the line between its two ends, less what a gather still leaves there.
    let ranged = match (
        along(&piece.sew, closure.from),
        along(&piece.sew, closure.to),
    ) {
        (Some(from), Some(to)) => {
            let direct = (from - to).abs();
            Some(direct.min(open - direct))
        }
        _ => None,
    };
    let removed = match (closure.kind, ranged) {
        (ClosureKind::Gather, Some(_)) => Some(closure.intake as f64),
        (_, range) => range,
    };
    let gap = gap_after_closing(closure);
    let conserved = removed
        .is_some_and(|removed| (removed - closure.intake as f64).abs() <= T1 as f64)
        && gap <= T1;
    let removed = removed.unwrap_or(0.0);
    ClosureReport {
        piece: piece.name.clone(),
        name: closure.name.clone(),
        kind: closure.kind,
        intake: closure.intake,
        open_length: open.round() as i64,
        closed_length: (open - removed).round() as i64,
        gap_after_closing: gap,
        conserved,
    }
}

/// What closing each closure of a pattern does, in piece and closure order.
#[must_use]
pub fn closure_reports(pattern: &Pattern) -> Vec<ClosureReport> {
    let mut pieces: Vec<&DrawnPiece> = pattern.pieces.iter().collect();
    pieces.sort_by(|a, b| a.name.cmp(&b.name));
    pieces
        .into_iter()
        .flat_map(|piece| {
            let mut closures: Vec<&Closure> = piece.closures.iter().collect();
            closures.sort_by(|a, b| a.name.cmp(&b.name));
            closures
                .into_iter()
                .map(|closure| report(piece, closure))
                .collect::<Vec<_>>()
        })
        .collect()
}
