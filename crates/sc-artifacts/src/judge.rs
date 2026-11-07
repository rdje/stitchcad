//! The judge: reads a drawn pattern against what a fixture declares, and names what is wrong. A fault it finds has a code of its own, so a mutation
//! test can require that a fault is caught for the right reason and not by some other check failing nearby.
//!
//! It shares no code with the writers, and reads the DXF and PDF files as text. The sewing spans it walks are a fixture's table until the
//! sewing graph is data, which is G3.
use crate::pattern::{DrawnPiece, Pattern};
use sc_geometry::Point;
use sc_store::{parse, Value};
use std::collections::{BTreeMap, BTreeSet};

/// What the judge found: a stable code, and a sentence naming the piece or the seam.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    /// What kind of fault this is.
    pub code: &'static str,
    /// Which thing, and what was found.
    pub detail: String,
}

fn finding(code: &'static str, detail: String) -> Finding {
    Finding { code, detail }
}

/// One side of a sewing span: a stretch of a piece's net line, walked in the seam's direction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpanSide {
    /// The piece.
    pub piece: String,
    /// Where the seam starts on this piece.
    pub from: (i64, i64),
    /// Where it ends.
    pub to: (i64, i64),
    /// The seam runs against the net line's own order here, as it does on a mirrored piece.
    pub against: bool,
    /// The stretch is straight between two points of the net line that need not be corners, such as a band's quarter marks.
    pub straight: bool,
    /// What sewing takes out of this stretch before it meets the other side: a dart's intake.
    pub less: i64,
}

/// A seam of the fixture: two stretches sewn together, with the ease it declares.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span {
    /// The span's name.
    pub name: String,
    /// One side.
    pub a: SpanSide,
    /// The other side.
    pub b: SpanSide,
    /// How far along the seam each notch must sit, in micrometres from the start, on both sides.
    pub notches_at: Vec<i64>,
}

/// One piece as a fixture declares it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PieceExpectation {
    /// The piece.
    pub name: String,
    /// How many are cut.
    pub quantity: u32,
    /// Its cut line's width and height, in micrometres.
    pub extent: (i64, i64),
    /// How many notches it carries.
    pub notches: usize,
    /// How deep each notch is, in micrometres.
    pub notch_depth: i64,
    /// Words its labels must hold.
    pub label_has: Vec<String>,
    /// Words its labels must not hold.
    pub label_lacks: Vec<String>,
    /// Which way its grain runs: `up` or `across`.
    pub grain: String,
    /// Any internal line that must stay off the cut line, by its y.
    pub fold_at_y: Option<i64>,
    /// How it is attached when it is in no span: `fused`, or empty.
    pub attachment: String,
}

/// What a fixture declares about its pattern.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expectations {
    /// Each input's name and its fixture value in micrometres.
    pub inputs: Vec<(String, i64)>,
    /// How far an input may stray from the fixture before it is implausible, as a factor.
    pub plausible_factor: i64,
    /// Each piece.
    pub pieces: Vec<PieceExpectation>,
    /// Each seam.
    pub spans: Vec<Span>,
    /// How far one line may differ from another that should equal it, in micrometres: the T2 class, because a curve is walked.
    pub tolerance: i64,
    /// How far a notch may land from where its seam declares it. A notch sits on the cut line, one allowance out from the seam, so on a slanted
    /// seam it projects short of the corner it marks, by the width times the sine of the slant.
    pub notch_tolerance: i64,
}

fn get<'a>(fields: &'a BTreeMap<String, Value>, name: &str) -> Result<&'a Value, String> {
    fields.get(name).ok_or_else(|| format!("no field {name}"))
}

fn whole(value: &Value, name: &str) -> Result<i64, String> {
    value
        .as_int()
        .ok_or_else(|| format!("{name} must be a whole number"))
}

fn words(value: &Value, name: &str) -> Result<String, String> {
    value
        .as_text()
        .map(str::to_owned)
        .ok_or_else(|| format!("{name} must be text"))
}

fn pair(value: &Value, name: &str) -> Result<(i64, i64), String> {
    match value.as_list() {
        Some([x, y]) => Ok((whole(x, name)?, whole(y, name)?)),
        _ => Err(format!("{name} must be two whole numbers")),
    }
}

fn listed<'a>(fields: &'a BTreeMap<String, Value>, name: &str) -> Result<&'a [Value], String> {
    get(fields, name)?
        .as_list()
        .ok_or_else(|| format!("{name} must be a list"))
}

fn object(value: &Value) -> Result<&BTreeMap<String, Value>, String> {
    value
        .as_object()
        .ok_or_else(|| "an entry must be an object".to_owned())
}

fn all_words(fields: &BTreeMap<String, Value>, name: &str) -> Result<Vec<String>, String> {
    listed(fields, name)?
        .iter()
        .map(|item| words(item, name))
        .collect()
}

fn side(value: &Value) -> Result<SpanSide, String> {
    let fields = object(value)?;
    let flag = |name: &str| matches!(fields.get(name), Some(Value::Bool(true)));
    Ok(SpanSide {
        piece: words(get(fields, "piece")?, "piece")?,
        from: pair(get(fields, "from")?, "from")?,
        to: pair(get(fields, "to")?, "to")?,
        against: flag("against"),
        straight: flag("straight"),
        less: fields
            .get("less")
            .map_or(Ok(0), |value| whole(value, "less"))?,
    })
}

impl Expectations {
    /// Reads a fixture's declaration from its text.
    ///
    /// # Errors
    /// The text is not a declaration.
    pub fn from_text(source: &str) -> Result<Self, String> {
        let parsed = parse(source).map_err(|error| error.to_string())?;
        let fields = object(&parsed)?;
        let inputs = listed(fields, "inputs")?
            .iter()
            .map(|item| {
                let item = object(item)?;
                Ok((
                    words(get(item, "token")?, "token")?,
                    whole(get(item, "um")?, "um")?,
                ))
            })
            .collect::<Result<_, String>>()?;
        let pieces = listed(fields, "pieces")?
            .iter()
            .map(|item| {
                let item = object(item)?;
                Ok(PieceExpectation {
                    name: words(get(item, "name")?, "name")?,
                    quantity: u32::try_from(whole(get(item, "quantity")?, "quantity")?)
                        .map_err(|error| error.to_string())?,
                    extent: pair(get(item, "extent")?, "extent")?,
                    notches: usize::try_from(whole(get(item, "notches")?, "notches")?)
                        .map_err(|error| error.to_string())?,
                    notch_depth: whole(get(item, "notch_depth")?, "notch_depth")?,
                    label_has: all_words(item, "label_has")?,
                    label_lacks: all_words(item, "label_lacks")?,
                    grain: words(get(item, "grain")?, "grain")?,
                    fold_at_y: item
                        .get("fold_at_y")
                        .map(|value| whole(value, "fold_at_y"))
                        .transpose()?,
                    attachment: item
                        .get("attachment")
                        .map_or(Ok(String::new()), |value| words(value, "attachment"))?,
                })
            })
            .collect::<Result<_, String>>()?;
        let spans = listed(fields, "spans")?
            .iter()
            .map(|item| {
                let item = object(item)?;
                Ok(Span {
                    name: words(get(item, "name")?, "name")?,
                    a: side(get(item, "a")?)?,
                    b: side(get(item, "b")?)?,
                    notches_at: listed(item, "notches_at")?
                        .iter()
                        .map(|value| whole(value, "notches_at"))
                        .collect::<Result<_, _>>()?,
                })
            })
            .collect::<Result<_, String>>()?;
        Ok(Self {
            inputs,
            plausible_factor: whole(get(fields, "plausible_factor")?, "plausible_factor")?,
            pieces,
            spans,
            tolerance: whole(get(fields, "tolerance")?, "tolerance")?,
            notch_tolerance: whole(get(fields, "notch_tolerance")?, "notch_tolerance")?,
        })
    }
}

// The factors a wrong unit would multiply a length by, as numerator and denominator.
const UNIT_SLIPS: [(&str, i64, i64); 6] = [
    ("millimetres read as centimetres", 10, 1),
    ("centimetres read as millimetres", 1, 10),
    ("inches read as millimetres", 1, 25),
    ("millimetres read as inches", 25, 1),
    ("micrometres read as millimetres", 1000, 1),
    ("millimetres read as micrometres", 1, 1000),
];

/// Reads a session's inputs against the fixture's, before anything is drawn from them.
#[must_use]
pub fn judge_inputs(inputs: &[(String, i64)], expected: &Expectations) -> Vec<Finding> {
    let mut found = Vec::new();
    for (token, value) in inputs {
        let Some((_, wanted)) = expected.inputs.iter().find(|(name, _)| name == token) else {
            found.push(finding(
                "unknown_input",
                format!("{token} is not an input of this fixture"),
            ));
            continue;
        };
        // A value of nothing is a stated zero, such as no ease, and has no scale to be wrong about.
        if *wanted == 0 || *value == 0 {
            continue;
        }
        let factor = expected.plausible_factor;
        if *value * factor >= *wanted && *value <= *wanted * factor {
            continue;
        }
        // Would one unit slip bring it back to about what the fixture says?
        let slip = UNIT_SLIPS.iter().find(|(_, top, bottom)| {
            let (restored, target) = (
                i128::from(*value) * i128::from(*bottom),
                i128::from(*wanted) * i128::from(*top),
            );
            restored * 5 >= target * 4 && restored * 4 <= target * 5
        });
        match slip {
            Some((what, ..)) => found.push(finding(
                "wrong_units",
                format!("{token} is {value} micrometres where {wanted} is expected: {what}"),
            )),
            None => found.push(finding(
                "implausible_input",
                format!("{token} is {value} micrometres, more than {factor} times from {wanted}"),
            )),
        }
    }
    found
}

fn extent(points: &[Point]) -> (i64, i64) {
    let xs = points.iter().map(|at| at.x());
    let ys = points.iter().map(|at| at.y());
    (
        xs.clone().max().unwrap_or(0) - xs.min().unwrap_or(0),
        ys.clone().max().unwrap_or(0) - ys.min().unwrap_or(0),
    )
}

// `sqrt` is correctly rounded on every platform, and `hypot` is not.
#[allow(clippy::cast_precision_loss)]
fn distance_between(a: Point, b: Point) -> f64 {
    let (x, y) = ((b.x() - a.x()) as f64, (b.y() - a.y()) as f64);
    (x * x + y * y).sqrt()
}

#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn length_between(a: Point, b: Point) -> i64 {
    distance_between(a, b).round() as i64
}

// Whether a phrase appears as whole words, so "R" is a right-hand piece and not the r in "shirt".
fn holds_words(text: &str, phrase: &str) -> bool {
    let word = |c: char| c.is_alphanumeric() || c == '_';
    text.match_indices(phrase).any(|(at, _)| {
        let before = text[..at].chars().next_back();
        let after = text[at + phrase.len()..].chars().next();
        !before.is_some_and(word) && !after.is_some_and(word)
    })
}

fn judge_piece(
    piece: &DrawnPiece,
    wanted: &PieceExpectation,
    tolerance: i64,
    found: &mut Vec<Finding>,
) {
    let name = &piece.name;
    if piece.quantity != wanted.quantity {
        found.push(finding(
            "wrong_quantity",
            format!(
                "{name} is cut {}, and should be {}",
                piece.quantity, wanted.quantity
            ),
        ));
    }
    if wanted.attachment == "fused" && piece.cut != piece.sew {
        found.push(finding(
            "allowance_on_fused_piece",
            format!("{name} is fused and should have no allowance"),
        ));
    }
    let size = extent(&piece.cut);
    if (size.0 - wanted.extent.0).abs() > tolerance || (size.1 - wanted.extent.1).abs() > tolerance
    {
        found.push(finding(
            "wrong_dimension",
            format!(
                "{name} is {} by {}, and should be {} by {}",
                size.0, size.1, wanted.extent.0, wanted.extent.1
            ),
        ));
    }
    if piece.marks.len() < wanted.notches {
        found.push(finding(
            "missing_notch",
            format!(
                "{name} has {} notches, and should have {}",
                piece.marks.len(),
                wanted.notches
            ),
        ));
    } else if piece.marks.len() > wanted.notches {
        found.push(finding(
            "extra_notch",
            format!(
                "{name} has {} notches, and should have {}",
                piece.marks.len(),
                wanted.notches
            ),
        ));
    }
    for mark in &piece.marks {
        let depth = length_between(mark.at, mark.toward);
        if (depth - wanted.notch_depth).abs() > tolerance {
            found.push(finding(
                "wrong_notch_depth",
                format!(
                    "{name} has a notch {depth} deep, and should have {}",
                    wanted.notch_depth
                ),
            ));
        }
        if !piece.cut.contains(&mark.at) {
            found.push(finding(
                "notch_off_cut_line",
                format!(
                    "{name} has a notch at ({}, {}) that is no vertex of its cut line",
                    mark.at.x(),
                    mark.at.y()
                ),
            ));
        }
    }
    let label = piece
        .labels
        .iter()
        .map(|item| item.text.as_str())
        .collect::<Vec<_>>()
        .join(" | ");
    for has in &wanted.label_has {
        if !holds_words(&label, has) {
            found.push(finding(
                "label_missing",
                format!("{name}'s labels do not say {has:?}: {label}"),
            ));
        }
    }
    for lacks in &wanted.label_lacks {
        if holds_words(&label, lacks) {
            found.push(finding(
                "label_forbidden",
                format!("{name}'s labels must not say {lacks:?}: {label}"),
            ));
        }
    }
    let (across, up) = (
        piece.grain.to.x() - piece.grain.from.x(),
        piece.grain.to.y() - piece.grain.from.y(),
    );
    let parallel = match wanted.grain.as_str() {
        "up" => across == 0 && up != 0,
        "across" => up == 0 && across != 0,
        _ => false,
    };
    if !parallel {
        found.push(finding(
            "grain_off",
            format!(
                "{name}'s grain runs ({across}, {up}), and should run {}",
                wanted.grain
            ),
        ));
    }
    if let Some(y) = wanted.fold_at_y {
        if !piece
            .internal
            .iter()
            .any(|line| line.from.y() == y && line.to.y() == y)
        {
            found.push(finding(
                "fold_not_internal",
                format!("{name} has no internal fold line at {y}"),
            ));
        }
        if piece.cut.iter().any(|at| at.y() == y) {
            found.push(finding(
                "fold_on_cut_line",
                format!("{name}'s fold at {y} touches its cut line"),
            ));
        }
    }
    if piece.sew.len() < 3 || piece.cut.len() < 3 {
        found.push(finding(
            "invalid_boundary",
            format!("{name} has too few points to be a boundary"),
        ));
    }
}

// The stretch of a closed line from one corner to another, in the direction that gets there along the line's running order.
fn stretch(line: &[Point], from: (i64, i64), to: (i64, i64)) -> Option<Vec<Point>> {
    let at = |wanted: (i64, i64)| {
        line.iter()
            .position(|point| (point.x(), point.y()) == wanted)
    };
    let (start, end) = (at(from)?, at(to)?);
    let mut out = vec![*line.get(start)?];
    let mut index = start;
    while index != end {
        index = (index + 1) % line.len();
        out.push(*line.get(index)?);
    }
    Some(out)
}

#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn length_of(points: &[Point]) -> f64 {
    points
        .windows(2)
        .map(|pair| match pair {
            [a, b] => distance_between(*a, *b),
            _ => 0.0,
        })
        .sum()
}

// How far along a stretch each of a piece's notches sits, measured to the point of the stretch nearest it, keeping those within reach of the seam.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn notch_parameters(piece: &DrawnPiece, along: &[Point], reach: f64) -> Vec<i64> {
    let mut found = Vec::new();
    for mark in &piece.marks {
        let (px, py) = (mark.at.x() as f64, mark.at.y() as f64);
        let (mut best, mut parameter, mut walked) = (f64::MAX, 0.0, 0.0);
        for pair in along.windows(2) {
            let [a, b] = pair else { continue };
            let (ax, ay) = (a.x() as f64, a.y() as f64);
            let (dx, dy) = (b.x() as f64 - ax, b.y() as f64 - ay);
            let length = (dx * dx + dy * dy).sqrt();
            if length > 0.0 {
                let t = (((px - ax) * dx + (py - ay) * dy) / (length * length)).clamp(0.0, 1.0);
                let (ox, oy) = (px - ax - t * dx, py - ay - t * dy);
                let away = (ox * ox + oy * oy).sqrt();
                if away < best {
                    best = away;
                    parameter = walked + t * length;
                }
            }
            walked += length;
        }
        if best <= reach {
            found.push(parameter.round() as i64);
        }
    }
    found.sort_unstable();
    found
}

/// Reads a drawn pattern against the fixture: every piece, then every span walked over the drawn net lines.
#[must_use]
pub fn judge_pattern(pattern: &Pattern, expected: &Expectations) -> Vec<Finding> {
    let mut found = Vec::new();
    let mut names = BTreeSet::new();
    for piece in &pattern.pieces {
        if !names.insert(piece.name.as_str()) {
            found.push(finding(
                "duplicate_piece",
                format!("{} is drafted twice", piece.name),
            ));
        }
        if !expected
            .pieces
            .iter()
            .any(|wanted| wanted.name == piece.name)
        {
            found.push(finding(
                "unexpected_piece",
                format!("{} is not a piece of this fixture", piece.name),
            ));
        }
    }
    for wanted in &expected.pieces {
        match pattern
            .pieces
            .iter()
            .find(|piece| piece.name == wanted.name)
        {
            Some(piece) => judge_piece(piece, wanted, expected.tolerance, &mut found),
            None => found.push(finding(
                "omitted_piece",
                format!("{} is missing", wanted.name),
            )),
        }
    }
    // D27's class: every piece is sewn, or declares how else it is attached.
    for wanted in &expected.pieces {
        let sewn = expected
            .spans
            .iter()
            .any(|span| span.a.piece == wanted.name || span.b.piece == wanted.name);
        if !sewn && wanted.attachment.is_empty() {
            found.push(finding(
                "unaccounted_piece",
                format!(
                    "{} is in no span and declares no other attachment",
                    wanted.name
                ),
            ));
        }
    }
    for span in &expected.spans {
        judge_span(pattern, span, expected, &mut found);
    }
    found
}

// A side's stretch of its piece's net line, in the seam's direction.
fn side_line(piece: &DrawnPiece, side: &SpanSide) -> Option<Vec<Point>> {
    if side.straight {
        return Some(vec![
            Point::new(side.from.0, side.from.1).ok()?,
            Point::new(side.to.0, side.to.1).ok()?,
        ]);
    }
    if side.against {
        let mut line = stretch(&piece.sew, side.to, side.from)?;
        line.reverse();
        return Some(line);
    }
    stretch(&piece.sew, side.from, side.to)
}

#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn judge_span(pattern: &Pattern, span: &Span, expected: &Expectations, found: &mut Vec<Finding>) {
    let (name, tolerance) = (&span.name, expected.tolerance);
    let piece_of = |side: &SpanSide| pattern.pieces.iter().find(|piece| piece.name == side.piece);
    let (Some(piece_a), Some(piece_b)) = (piece_of(&span.a), piece_of(&span.b)) else {
        // The piece itself is reported as omitted. A span cannot be walked without it.
        return;
    };
    let (Some(line_a), Some(line_b)) = (side_line(piece_a, &span.a), side_line(piece_b, &span.b))
    else {
        found.push(finding(
            "span_off_piece",
            format!("{name} names a corner its piece's net line does not have"),
        ));
        return;
    };
    let (walked_a, walked_b) = (length_of(&line_a), length_of(&line_b));
    let (length_a, length_b) = (walked_a - span.a.less as f64, walked_b - span.b.less as f64);
    if (length_a - length_b).abs() > tolerance as f64 {
        found.push(finding(
            "seam_length",
            format!(
                "{name}: {} walks {length_a:.0} and {} walks {length_b:.0}, with no ease declared",
                span.a.piece, span.b.piece
            ),
        ));
        return;
    }
    // Lengths alone cannot see a seam sewn end for end: a notch has to land at the same distance along it.
    let reach = 40_000.0;
    for (side, piece, line, walked) in [
        (&span.a, piece_a, &line_a, walked_a),
        (&span.b, piece_b, &line_b, walked_b),
    ] {
        let notches = notch_parameters(piece, line, reach);
        let holds = |wanted: &i64| {
            notches
                .iter()
                .any(|at| (at - wanted).abs() <= expected.notch_tolerance)
        };
        if span.notches_at.iter().all(holds) {
            continue;
        }
        let flipped = |wanted: &i64| holds(&(walked.round() as i64 - wanted));
        let code = if span.notches_at.iter().all(flipped) {
            "reversed_correspondence"
        } else {
            "notch_misaligned"
        };
        found.push(finding(
            code,
            format!(
                "{name}: {} has notches at {notches:?} along the seam, which declares {:?}",
                side.piece, span.notches_at
            ),
        ));
    }
}

// Each block and layer's polylines, in millimetres.
type DxfLines = BTreeMap<(String, String), Vec<Vec<(f64, f64)>>>;

// A DXF's coordinates by block and layer, read as text with no help from the writer.
fn dxf_lines(text: &str) -> DxfLines {
    let lines: Vec<&str> = text.lines().collect();
    let mut out = DxfLines::new();
    let (mut block, mut kind, mut layer, mut x) =
        (String::new(), String::new(), String::new(), 0.0);
    for pair in lines.chunks(2) {
        let [code, value] = pair else { continue };
        let (Ok(code), value) = (code.trim().parse::<i32>(), value.trim()) else {
            continue;
        };
        match (code, kind.as_str()) {
            (0, _) => value.clone_into(&mut kind),
            (2, "BLOCK") => value.clone_into(&mut block),
            (8, "POLYLINE") => {
                value.clone_into(&mut layer);
                out.entry((block.clone(), layer.clone()))
                    .or_default()
                    .push(Vec::new());
            }
            (10, "VERTEX") => x = value.parse().unwrap_or(f64::NAN),
            (20, "VERTEX") => {
                if let Some(list) = out
                    .get_mut(&(block.clone(), layer.clone()))
                    .and_then(|listed| listed.last_mut())
                {
                    list.push((x, value.parse().unwrap_or(f64::NAN)));
                }
            }
            _ => {}
        }
    }
    out
}

/// Reads a written DXF against the pattern it was written from: is every line on the layer its record says, at the scale it says?
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn judge_dxf(text: &str, pattern: &Pattern, cut_layer_holds_cut: bool) -> Vec<Finding> {
    let mut found = Vec::new();
    let read = dxf_lines(text);
    let to_mm = |points: &[Point]| -> Vec<(f64, f64)> {
        points
            .iter()
            .map(|at| (at.x() as f64 / 1000.0, at.y() as f64 / 1000.0))
            .collect()
    };
    // A closed line has no first point of its own, so two are the same line if one starts anywhere along the other.
    let same = |a: &[(f64, f64)], b: &[(f64, f64)]| {
        let close = |p: &(f64, f64), q: &(f64, f64)| {
            (p.0 - q.0).abs() < 0.0015 && (p.1 - q.1).abs() < 0.0015
        };
        let Some(first) = a.first() else {
            return b.is_empty();
        };
        a.len() == b.len()
            && b.iter()
                .enumerate()
                .filter(|(_, q)| close(first, q))
                .any(|(start, _)| {
                    a.iter()
                        .zip(b.iter().cycle().skip(start))
                        .all(|(p, q)| close(p, q))
                })
    };
    for piece in &pattern.pieces {
        let block = format!("{}_{}", piece.name, piece.size);
        let of = |layer: &str| {
            read.get(&(block.clone(), layer.to_owned()))
                .and_then(|listed| listed.first())
        };
        let (Some(on_cut), Some(on_draw)) = (of("CUT"), of("DRAW")) else {
            found.push(finding(
                "missing_block",
                format!("{block} has no cut and draw lines in the file"),
            ));
            continue;
        };
        let (cut, sew) = (to_mm(&piece.cut), to_mm(&piece.sew));
        let (want_cut, want_draw) = if cut_layer_holds_cut {
            (&cut, &sew)
        } else {
            (&sew, &cut)
        };
        if same(on_cut, want_cut) && same(on_draw, want_draw) {
            continue;
        }
        if cut != sew && same(on_cut, want_draw) && same(on_draw, want_cut) {
            found.push(finding(
                "cut_and_sew_swapped",
                format!("{block}: the cut and sew lines are on each other's layers"),
            ));
            continue;
        }
        // Each point scaled alike is a unit, and anything else is a wrong line.
        let scale = |a: &[(f64, f64)], b: &[(f64, f64)]| {
            let extent = |list: &[(f64, f64)]| {
                list.iter().map(|at| at.0).fold(f64::MIN, f64::max)
                    - list.iter().map(|at| at.0).fold(f64::MAX, f64::min)
            };
            extent(a) / extent(b).max(f64::MIN_POSITIVE)
        };
        let ratio = scale(on_cut, want_cut);
        if [25.4, 1.0 / 25.4, 10.0, 0.1, 1000.0, 0.001]
            .iter()
            .any(|factor: &f64| (ratio / factor - 1.0).abs() < 0.001)
        {
            found.push(finding(
                "wrong_units",
                format!("{block}: the file's lines are {ratio:.4} times the pattern's"),
            ));
        } else {
            found.push(finding(
                "wrong_geometry",
                format!("{block}: the file's lines are not the pattern's"),
            ));
        }
    }
    found
}

/// Reads a written PDF's first drawn square, which is the scale square, as text: it must measure 100 mm.
#[must_use]
pub fn judge_pdf_square(bytes: &[u8]) -> Vec<Finding> {
    let text = String::from_utf8_lossy(bytes);
    for line in text.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let [_, _, width, height, "re", "S"] = parts.as_slice() {
            let side = |value: &str| value.parse::<f64>().unwrap_or(f64::NAN) * 25.4 / 72.0;
            let (across, up) = (side(width), side(height));
            if (across - 100.0).abs() > 0.001 || (up - 100.0).abs() > 0.001 {
                return vec![finding(
                    "scale_square_wrong",
                    format!("the scale square is {across:.3} by {up:.3} mm"),
                )];
            }
            return Vec::new();
        }
    }
    vec![finding(
        "scale_square_missing",
        "the file has no scale square".to_owned(),
    )]
}
