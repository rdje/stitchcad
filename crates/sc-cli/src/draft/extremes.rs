//! A size's checks after it has been rebuilt from a written file. Each one fails with its own diagnostic rather than a clamped value, because a size five breaks away from a valid sample can still be wrong in any of these ways.
use super::{length, ratio, whole, Fields, Lengths, WALK_TOLERANCE};
use sc_artifacts::{closure_reports, walk, DrawnPiece, Pattern};
use sc_geometry::Point;
use sc_store::Value;
use sc_units::pure;

/// A coordinate or a length, from the units chapter.
const DOMAIN: i64 = 1_000_000_000;
/// A piece's bounding box, from the units chapter.
const PIECE_BOX: i64 = 10_000_000;

/// One check at one size.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Check {
    /// Which check.
    pub check: &'static str,
    /// The size.
    pub size: String,
    /// The class it is judged at, from the instantiation chapter.
    pub class: &'static str,
    /// `holds`, `fails`, or `no_limit` where nothing has declared what would be too much.
    pub verdict: &'static str,
    /// What it found, in words.
    pub detail: String,
}

impl Check {
    /// The check as a value.
    #[must_use]
    pub fn to_value(&self) -> Value {
        let mut fields = Fields::new();
        fields.insert("check".to_owned(), Value::Text(self.check.to_owned()));
        fields.insert("size".to_owned(), Value::Text(self.size.clone()));
        fields.insert("class".to_owned(), Value::Text(self.class.to_owned()));
        fields.insert("verdict".to_owned(), Value::Text(self.verdict.to_owned()));
        fields.insert("detail".to_owned(), Value::Text(self.detail.clone()));
        Value::Object(fields)
    }
}

fn limit(lengths: &Lengths, description: &Fields, name: &str) -> Result<Option<i64>, String> {
    let token = description
        .get("grading")
        .and_then(Value::as_object)
        .and_then(|grading| grading.get("limits"))
        .and_then(Value::as_object)
        .and_then(|limits| limits.get(name))
        .and_then(Value::as_text);
    token.map(|token| length(lengths, token)).transpose()
}

fn crosses(line: &[Point]) -> bool {
    let at = |index: usize| {
        line.get(index % line.len())
            .map_or((0.0, 0.0), |point| (ratio(point.x()), ratio(point.y())))
    };
    let side = |p: (f64, f64), q: (f64, f64), r: (f64, f64)| {
        (q.0 - p.0) * (r.1 - p.1) - (q.1 - p.1) * (r.0 - p.0)
    };
    for first in 0..line.len() {
        for second in first + 2..line.len() {
            if first == 0 && second == line.len() - 1 {
                continue;
            }
            let (a, b, c, d) = (at(first), at(first + 1), at(second), at(second + 1));
            if side(a, b, c) * side(a, b, d) < 0.0 && side(c, d, a) * side(c, d, b) < 0.0 {
                return true;
            }
        }
    }
    false
}

// The tightest bend on a line, leaving out what is a corner rather than a curve: three points turning by less than a third of a right angle are on a curve, and a circle through them has a radius.
fn tightest_bend(line: &[Point]) -> Option<f64> {
    const CURVE_TURN: f64 = 0.5;
    let mut least: Option<f64> = None;
    let count = line.len();
    for index in 0..count {
        let (Some(a), Some(b), Some(c)) = (
            line.get(index),
            line.get((index + 1) % count),
            line.get((index + 2) % count),
        ) else {
            continue;
        };
        let (ux, uy) = (ratio(b.x() - a.x()), ratio(b.y() - a.y()));
        let (vx, vy) = (ratio(c.x() - b.x()), ratio(c.y() - b.y()));
        let (lu, lv) = (pure::hypot(ux, uy), pure::hypot(vx, vy));
        if lu == 0.0 || lv == 0.0 {
            continue;
        }
        let (cross, dot) = (ux * vy - uy * vx, ux * vx + uy * vy);
        let turn = pure::atan2(cross.abs(), dot);
        if turn <= 1e-6 || turn >= CURVE_TURN {
            continue;
        }
        let (wx, wy) = (ratio(c.x() - a.x()), ratio(c.y() - a.y()));
        let radius = lu * lv * pure::hypot(wx, wy) / (2.0 * cross.abs());
        least = Some(least.map_or(radius, |was: f64| was.min(radius)));
    }
    least
}

fn across_grain(piece: &DrawnPiece) -> i64 {
    let (dx, dy) = (
        ratio(piece.grain.to.x() - piece.grain.from.x()),
        ratio(piece.grain.to.y() - piece.grain.from.y()),
    );
    let reach = pure::hypot(dx, dy).max(1.0);
    let (nx, ny) = (-dy / reach, dx / reach);
    let across: Vec<f64> = piece
        .cut
        .iter()
        .map(|at| ratio(at.x()) * nx + ratio(at.y()) * ny)
        .collect();
    let (low, high) = across
        .iter()
        .fold((f64::MAX, f64::MIN), |(low, high), value| {
            (low.min(*value), high.max(*value))
        });
    whole(high - low)
}

fn verdict_against(found: i64, most: Option<i64>) -> (&'static str, String) {
    match most {
        None => (
            "no_limit",
            format!("{found} µm, and nothing has declared a limit"),
        ),
        Some(most) if found > most => ("fails", format!("{found} µm, past the limit of {most} µm")),
        Some(most) => ("holds", format!("{found} µm, within {most} µm")),
    }
}

// What a garment says its finished dimensions are, read off the rebuilt drawing rather than from what the recipe declared.
fn finished_dimensions(
    size: &str,
    description: &Fields,
    at_size: &Lengths,
    pattern: &Pattern,
    out: &mut Vec<Check>,
) -> Result<(), String> {
    let listed = description
        .get("grading")
        .and_then(Value::as_object)
        .and_then(|grading| grading.get("finished"))
        .and_then(Value::as_list)
        .unwrap_or(&[]);
    let report = walk(pattern).map_err(|error| format!("{}: {}", error.seam, error.message))?;
    for item in listed.iter().filter_map(Value::as_object) {
        let name = item
            .get("name")
            .and_then(Value::as_text)
            .ok_or("a finished dimension has a name")?;
        let wanted = length(
            at_size,
            item.get("equals")
                .and_then(Value::as_text)
                .ok_or("a finished dimension says what it equals")?,
        )?;
        let times = item.get("times").and_then(Value::as_int).unwrap_or(1);
        let mut found = 0;
        for seam in item.get("seams").and_then(Value::as_list).unwrap_or(&[]) {
            let seam = seam.as_text().ok_or("a seam's name is text")?;
            let walked = report
                .iter()
                .find(|walked| walked.seam == seam)
                .ok_or_else(|| {
                    format!("{name} adds up the seam {seam}, and no seam has that name")
                })?;
            found += if item.get("side").and_then(Value::as_text) == Some("a") {
                walked.a_length
            } else {
                walked.b_length
            };
        }
        for piece in item.get("widest").and_then(Value::as_list).unwrap_or(&[]) {
            let piece = piece.as_text().ok_or("a piece's name is text")?;
            let drawn = pattern
                .pieces
                .iter()
                .find(|drawn| drawn.name == piece)
                .ok_or_else(|| format!("{name} measures {piece}, and no piece has that name"))?;
            found += drawn.sew.iter().map(|at| at.x()).max().unwrap_or(0)
                - drawn.sew.iter().map(|at| at.x()).min().unwrap_or(0);
        }
        found *= times;
        let tolerance = WALK_TOLERANCE * 4 * times.max(1);
        let difference = found - wanted;
        out.push(Check {
            check: "finished_dimension",
            size: size.to_owned(),
            class: "exact",
            verdict: if difference.abs() <= tolerance { "holds" } else { "fails" },
            detail: format!(
                "{name} measures {found} µm on the drawing, and the recipe says {wanted} µm, which is {difference} µm"
            ),
        });
    }
    Ok(())
}

/// Every check, on a size that was rebuilt.
///
/// # Errors
///
/// A limit names a length that has no value, or a dimension names what does not exist.
pub fn checks_on(
    size: &str,
    description: &Fields,
    at_size: &Lengths,
    pattern: &Pattern,
) -> Result<Vec<Check>, String> {
    let mut out = Vec::new();
    let check =
        |check: &'static str, class: &'static str, verdict: &'static str, detail: String| Check {
            check,
            size: size.to_owned(),
            class,
            verdict,
            detail,
        };
    // A coordinate out of range means a unit went wrong somewhere, and no later check means anything then.
    let mut largest_coordinate = 0;
    let mut largest_box = 0;
    for piece in &pattern.pieces {
        let (mut low_x, mut low_y, mut high_x, mut high_y) =
            (i64::MAX, i64::MAX, i64::MIN, i64::MIN);
        for at in piece.cut.iter().chain(&piece.sew) {
            largest_coordinate = largest_coordinate.max(at.x().abs()).max(at.y().abs());
            low_x = low_x.min(at.x());
            low_y = low_y.min(at.y());
            high_x = high_x.max(at.x());
            high_y = high_y.max(at.y());
        }
        largest_box = largest_box.max(high_x - low_x).max(high_y - low_y);
    }
    out.push(check(
        "declared_domain",
        "numerical",
        if largest_coordinate <= DOMAIN && largest_box <= PIECE_BOX { "holds" } else { "fails" },
        format!("the largest coordinate is {largest_coordinate} µm, and the largest piece {largest_box} µm"),
    ));
    let crossing: Vec<&str> = pattern
        .pieces
        .iter()
        .filter(|piece| crosses(&piece.cut))
        .map(|piece| piece.name.as_str())
        .collect();
    out.push(check(
        "offset_validity",
        "geometric",
        if crossing.is_empty() {
            "holds"
        } else {
            "fails"
        },
        if crossing.is_empty() {
            "every allowance came out without crossing itself".to_owned()
        } else {
            format!("the cut line of {} crosses itself", crossing.join(", "))
        },
    ));
    let tightest = pattern
        .pieces
        .iter()
        .filter_map(|piece| tightest_bend(&piece.cut).map(|radius| (radius, piece.name.as_str())))
        .fold(None, |best: Option<(f64, &str)>, here| match best {
            Some(best) if best.0 <= here.0 => Some(best),
            _ => Some(here),
        });
    let least = limit(at_size, description, "min_radius")?;
    let (verdict, detail) = match (tightest, least) {
        (None, _) => ("holds", "no piece has a curve".to_owned()),
        (Some((radius, piece)), None) => (
            "no_limit",
            format!(
                "the tightest bend is {} µm on {piece}, and nothing has declared a limit",
                whole(radius)
            ),
        ),
        (Some((radius, piece)), Some(least)) if whole(radius) < least => (
            "fails",
            format!(
                "the tightest bend is {} µm on {piece}, under the limit of {least} µm",
                whole(radius)
            ),
        ),
        (Some((radius, piece)), Some(least)) => (
            "holds",
            format!(
                "the tightest bend is {} µm on {piece}, over the limit of {least} µm",
                whole(radius)
            ),
        ),
    };
    out.push(check(
        "minimum_curvature_radius",
        "geometric",
        verdict,
        detail,
    ));
    let widest = pattern
        .pieces
        .iter()
        .map(|piece| (across_grain(piece), piece.name.as_str()))
        .max_by_key(|(across, _)| *across);
    if let Some((across, piece)) = widest {
        let (verdict, detail) =
            verdict_against(across, limit(at_size, description, "fabric_width")?);
        out.push(check(
            "bounding_box_against_fabric",
            "physical",
            verdict,
            format!("{piece}, across its grain: {detail}"),
        ));
    }
    let report = walk(pattern).map_err(|error| format!("{}: {}", error.seam, error.message))?;
    let out_of_ease: Vec<String> = report
        .iter()
        .filter(|seam| seam.unexplained.abs() > WALK_TOLERANCE)
        .map(|seam| format!("{} by {} µm", seam.seam, seam.unexplained))
        .collect();
    out.push(check(
        "seam_differential_against_ease",
        "geometric",
        if out_of_ease.is_empty() {
            "holds"
        } else {
            "fails"
        },
        if out_of_ease.is_empty() {
            "every seam walks to what it declares".to_owned()
        } else {
            out_of_ease.join(", ")
        },
    ));
    let mut closest: Option<(i64, &str)> = None;
    for piece in &pattern.pieces {
        for (index, a) in piece.marks.iter().enumerate() {
            for b in piece.marks.iter().skip(index + 1) {
                let apart = whole(pure::hypot(
                    ratio(a.at.x() - b.at.x()),
                    ratio(a.at.y() - b.at.y()),
                ));
                if closest.is_none_or(|(least, _)| apart < least) {
                    closest = Some((apart, &piece.name));
                }
            }
        }
    }
    if let Some((apart, piece)) = closest {
        let least = limit(at_size, description, "notch_spacing")?;
        let (verdict, detail) = match least {
            None => (
                "no_limit",
                format!("{apart} µm on {piece}, and nothing has declared a limit"),
            ),
            Some(least) if apart < least => (
                "fails",
                format!("{apart} µm on {piece}, under the limit of {least} µm"),
            ),
            Some(least) => (
                "holds",
                format!("{apart} µm on {piece}, over the limit of {least} µm"),
            ),
        };
        out.push(check("notch_spacing", "physical", verdict, detail));
    }
    let closures = closure_reports(pattern);
    if let Some(biggest) = closures.iter().max_by_key(|closure| closure.intake) {
        let (verdict, detail) =
            verdict_against(biggest.intake, limit(at_size, description, "dart_intake")?);
        out.push(check(
            "dart_intake_against_maximum",
            "physical",
            verdict,
            format!("{}/{}: {detail}", biggest.piece, biggest.name),
        ));
    }
    finished_dimensions(size, description, at_size, pattern, &mut out)?;
    Ok(out)
}
