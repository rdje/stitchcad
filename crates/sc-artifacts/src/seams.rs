//! Walking a seam: the length of each side, what differs, and whether that difference is ease somebody declared or a defect nobody did.
use crate::closures::along;
use crate::pattern::{Closure, DrawnPiece, Pattern, Seam, SeamSide};
use sc_geometry::Point;
use sc_store::Value;
use sc_units::pure;
use std::collections::BTreeMap;

// T2 of the units chapter: two lines each within it of a curve can differ by this much and still be one seam.
/// How far two sides of a seam may differ and still be the same length: twice T2, because two lines each within T2 of one curve can differ by this much.
pub const WALK_TOLERANCE: i64 = 20;

/// What walking one seam found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WalkReport {
    /// The seam.
    pub seam: String,
    /// Side A's sewn length: its stretch of the sew line less the closures on it.
    pub a_length: i64,
    /// Side B's sewn length.
    pub b_length: i64,
    /// A's length less B's.
    pub differential: i64,
    /// What the seam declares that to be.
    pub ease: i64,
    /// The part of the differential no declaration explains.
    pub unexplained: i64,
    /// `matches`, `declared_ease`, or `defect`.
    pub finding: &'static str,
}

impl WalkReport {
    /// The report as a value.
    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut fields = BTreeMap::new();
        fields.insert("seam".to_owned(), Value::Text(self.seam.clone()));
        fields.insert("a_length".to_owned(), Value::Int(self.a_length));
        fields.insert("b_length".to_owned(), Value::Int(self.b_length));
        fields.insert("differential".to_owned(), Value::Int(self.differential));
        fields.insert("ease".to_owned(), Value::Int(self.ease));
        fields.insert("unexplained".to_owned(), Value::Int(self.unexplained));
        fields.insert("finding".to_owned(), Value::Text(self.finding.to_owned()));
        Value::Object(fields)
    }
}

/// A seam that cannot be walked, and why.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WalkError {
    /// The seam.
    pub seam: String,
    /// What is wrong, in a sentence.
    pub message: String,
}

#[allow(clippy::cast_precision_loss)]
fn distance(a: Point, b: Point) -> f64 {
    pure::hypot((b.x() - a.x()) as f64, (b.y() - a.y()) as f64)
}

/// The stretch of a closed line between two points, the shorter way round, in that direction. Either end may lie along an edge.
#[must_use]
pub fn stretch(line: &[Point], from: Point, to: Point) -> Option<Vec<Point>> {
    let (start, end) = (along(line, from)?, along(line, to)?);
    let perimeter: f64 = line
        .iter()
        .enumerate()
        .filter_map(|(at, a)| line.get((at + 1) % line.len()).map(|b| distance(*a, *b)))
        .sum();
    let forward = if end >= start {
        end - start
    } else {
        perimeter - start + end
    };
    let backward = perimeter - forward;
    let mut positions = Vec::with_capacity(line.len());
    let mut walked = 0.0;
    for (at, point) in line.iter().enumerate() {
        positions.push((walked, *point));
        walked += line
            .get((at + 1) % line.len())
            .map_or(0.0, |next| distance(*point, *next));
    }
    let mut out = vec![from];
    if forward <= backward {
        let mut later: Vec<(f64, Point)> = positions
            .iter()
            .map(|(at, point)| ((at - start).rem_euclid(perimeter), *point))
            .filter(|(gone, _)| *gone > 0.0 && *gone < forward)
            .collect();
        later.sort_by(|a, b| a.0.total_cmp(&b.0));
        out.extend(later.into_iter().map(|(_, point)| point));
    } else {
        let mut later: Vec<(f64, Point)> = positions
            .iter()
            .map(|(at, point)| ((start - at).rem_euclid(perimeter), *point))
            .filter(|(gone, _)| *gone > 0.0 && *gone < backward)
            .collect();
        later.sort_by(|a, b| a.0.total_cmp(&b.0));
        out.extend(later.into_iter().map(|(_, point)| point));
    }
    out.push(to);
    Some(out)
}

/// A polyline's length.
#[must_use]
pub fn stretch_length(points: &[Point]) -> f64 {
    points
        .windows(2)
        .map(|pair| match pair {
            [a, b] => distance(*a, *b),
            _ => 0.0,
        })
        .sum()
}

// A closure sits on a stretch when both of its ends do. What it takes up is sewn away, so the seam does not see it.
fn taken_up(closures: &[Closure], stretched: &[Point]) -> i64 {
    closures
        .iter()
        .filter(|closure| open_along(stretched, closure.from) && open_along(stretched, closure.to))
        .map(|closure| closure.intake)
        .sum()
}

#[allow(clippy::cast_precision_loss)]
fn open_along(line: &[Point], at: Point) -> bool {
    line.windows(2).any(|pair| {
        let [a, b] = pair else { return false };
        let (dx, dy) = ((b.x() - a.x()) as f64, (b.y() - a.y()) as f64);
        let length = pure::hypot(dx, dy);
        if length == 0.0 {
            return false;
        }
        let t = (((at.x() - a.x()) as f64 * dx + (at.y() - a.y()) as f64 * dy) / (length * length))
            .clamp(0.0, 1.0);
        pure::hypot(
            (at.x() - a.x()) as f64 - t * dx,
            (at.y() - a.y()) as f64 - t * dy,
        ) <= 10.0
    })
}

#[allow(clippy::cast_possible_truncation)]
fn side_length(pieces: &[DrawnPiece], side: &SeamSide, seam: &str) -> Result<i64, WalkError> {
    let fail = |message: String| WalkError {
        seam: seam.to_owned(),
        message,
    };
    let piece = pieces
        .iter()
        .find(|piece| piece.name == side.piece)
        .ok_or_else(|| fail(format!("no piece is named {}", side.piece)))?;
    let stretched = stretch(&piece.sew, side.from, side.to).ok_or_else(|| {
        fail(format!(
            "an end of it is not on the sew line of {}",
            side.piece
        ))
    })?;
    Ok(stretch_length(&stretched).round() as i64 - taken_up(&piece.closures, &stretched))
}

/// Walks every seam of a pattern, in name order.
///
/// # Errors
/// A seam names a piece that does not exist or an end that is not on its sew line.
pub fn walk(pattern: &Pattern) -> Result<Vec<WalkReport>, WalkError> {
    let mut seams: Vec<&Seam> = pattern.seams.iter().collect();
    seams.sort_by(|a, b| a.name.cmp(&b.name));
    seams
        .into_iter()
        .map(|seam| {
            let a_length = side_length(&pattern.pieces, &seam.a, &seam.name)?;
            let b_length = side_length(&pattern.pieces, &seam.b, &seam.name)?;
            let differential = a_length - b_length;
            let unexplained = differential - seam.ease;
            let finding = if unexplained.abs() > WALK_TOLERANCE {
                "defect"
            } else if seam.ease == 0 {
                "matches"
            } else {
                "declared_ease"
            };
            Ok(WalkReport {
                seam: seam.name.clone(),
                a_length,
                b_length,
                differential,
                ease: seam.ease,
                unexplained,
                finding,
            })
        })
        .collect()
}
