//! The DXF writer for the one target G2 names: `dxf-aama-named`, AAMA named layers on release R12, polylines only, one size.
use crate::pattern::{DrawnPiece, Line, Mark, MarkKind, Pattern};
use core::fmt;
use sc_geometry::Point;
use sc_store::Value;
use std::collections::BTreeMap;
use std::fmt::Write as _;

/// The registered target this writes.
pub const TARGET: &str = "dxf-aama-named";
/// The writer's identity, recorded with each artifact. A receiver's finding is scoped to it.
pub const WRITER: &str = "stitchcad-dxf-1";

const RADIUS_OF_DRILL_MARK: i64 = 1_000;

/// Which line the CUT layer carries. A Factory Profile chooses, and there is no default, because the wrong one is a top cause of rejection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineSemantics {
    /// CUT carries the cut boundary, and DRAW the sew line.
    CutAsCut,
    /// CUT carries the sew line, and DRAW the cut boundary. The meaning of each line stays what it is.
    SewAsCut,
}

/// How an artifact's curves were flattened, published with it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Flattening {
    /// The target's bound, in micrometres.
    pub bound: i64,
    /// The largest deviation reached, in micrometres.
    pub achieved: i64,
}

/// Why a file was not written. Nothing is written in that case.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DxfError {
    /// A piece's name or size holds a character a block name cannot, or is empty.
    BadBlockName {
        /// The name.
        name: String,
    },
    /// Two pieces would have one block name.
    DuplicateBlock {
        /// The name.
        name: String,
    },
    /// A closed line has fewer than three points.
    NotClosed {
        /// The block.
        block: String,
        /// Which line.
        line: &'static str,
    },
    /// The curves were flattened past the target's bound, so shipping them would ship coarser geometry than it declares.
    TessellationBound {
        /// The bound.
        bound: i64,
        /// What was reached.
        achieved: i64,
    },
    /// A label holds a line break or a control character, and one label is one single-line entity.
    BadLabel {
        /// The block.
        block: String,
        /// The label.
        text: String,
    },
}

impl fmt::Display for DxfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadBlockName { name } => write!(
                f,
                "{name:?} cannot name a block: use letters, digits, - and _"
            ),
            Self::DuplicateBlock { name } => write!(f, "two pieces would both be block {name}"),
            Self::NotClosed { block, line } => {
                write!(f, "block {block}'s {line} line has fewer than three points")
            }
            Self::TessellationBound { bound, achieved } => {
                write!(f, "dialect_tessellation_bound: flattened to {achieved} micrometres, past the target's {bound}")
            }
            Self::BadLabel { block, text } => write!(
                f,
                "block {block}'s label {text:?} is not a single line of plain text"
            ),
        }
    }
}

impl std::error::Error for DxfError {}

/// A written file and its receiver-config record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DxfArtifact {
    /// The file's text.
    pub text: String,
    /// The record that travels with it, as canonical text.
    pub record: String,
}

// Whole micrometres as millimetres with three places, exactly, so a value reads back to the same micrometre.
fn millimetres(micrometres: i64) -> String {
    let sign = if micrometres < 0 { "-" } else { "" };
    let magnitude = micrometres.unsigned_abs();
    format!("{sign}{}.{:03}", magnitude / 1000, magnitude % 1000)
}

fn pair(out: &mut String, code: i32, value: &str) {
    let _ = writeln!(out, "{code:>3}\n{value}");
}

fn coordinates(out: &mut String, at: Point) {
    pair(out, 10, &millimetres(at.x()));
    pair(out, 20, &millimetres(at.y()));
    pair(out, 30, "0.0");
}

fn polyline(out: &mut String, layer: &str, points: &[Point], closed: bool) {
    pair(out, 0, "POLYLINE");
    pair(out, 8, layer);
    pair(out, 66, "1");
    pair(out, 70, if closed { "1" } else { "0" });
    for point in points {
        pair(out, 0, "VERTEX");
        pair(out, 8, layer);
        coordinates(out, *point);
    }
    pair(out, 0, "SEQEND");
    pair(out, 8, layer);
}

fn line(out: &mut String, layer: &str, from: Point, to: Point) {
    pair(out, 0, "LINE");
    pair(out, 8, layer);
    coordinates(out, from);
    pair(out, 11, &millimetres(to.x()));
    pair(out, 21, &millimetres(to.y()));
    pair(out, 31, "0.0");
}

fn text(out: &mut String, at: Point, value: &str) {
    pair(out, 0, "TEXT");
    pair(out, 8, "TEXT");
    coordinates(out, at);
    pair(out, 40, "3.000");
    pair(out, 1, value);
}

// A drill hole is a closed polyline of two half circles, each a bulge of 1, so the circle is carried exactly.
fn drill(out: &mut String, at: Point) {
    pair(out, 0, "POLYLINE");
    pair(out, 8, "DRILL");
    pair(out, 66, "1");
    pair(out, 70, "1");
    for x in [at.x() - RADIUS_OF_DRILL_MARK, at.x() + RADIUS_OF_DRILL_MARK] {
        pair(out, 0, "VERTEX");
        pair(out, 8, "DRILL");
        pair(out, 10, &millimetres(x));
        pair(out, 20, &millimetres(at.y()));
        pair(out, 30, "0.0");
        pair(out, 42, "1.000000");
    }
    pair(out, 0, "SEQEND");
    pair(out, 8, "DRILL");
}

fn block_name(piece: &DrawnPiece) -> Result<String, DxfError> {
    let name = format!("{}_{}", piece.name, piece.size);
    let allowed = |part: &str| {
        !part.is_empty()
            && part
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    };
    if allowed(&piece.name) && allowed(&piece.size) {
        Ok(name)
    } else {
        Err(DxfError::BadBlockName { name })
    }
}

fn write_block(
    out: &mut String,
    piece: &DrawnPiece,
    name: &str,
    pattern: &Pattern,
    semantics: LineSemantics,
) -> Result<(), DxfError> {
    for (points, which) in [(&piece.cut, "cut"), (&piece.sew, "sew")] {
        if points.len() < 3 {
            return Err(DxfError::NotClosed {
                block: name.to_owned(),
                line: which,
            });
        }
    }
    let (on_cut, on_draw) = match semantics {
        LineSemantics::CutAsCut => (&piece.cut, &piece.sew),
        LineSemantics::SewAsCut => (&piece.sew, &piece.cut),
    };
    pair(out, 0, "BLOCK");
    pair(out, 8, "0");
    pair(out, 2, name);
    pair(out, 70, "0");
    pair(out, 10, "0.0");
    pair(out, 20, "0.0");
    pair(out, 30, "0.0");
    pair(out, 3, name);
    polyline(out, "CUT", on_cut, true);
    polyline(out, "DRAW", on_draw, true);
    line(out, "DRAW", piece.grain.from, piece.grain.to);
    let mut internal: Vec<Line> = piece.internal.clone();
    internal.sort();
    for item in &internal {
        polyline(out, "INTCUT", &[item.from, item.to], false);
    }
    let mut marks: Vec<Mark> = piece.marks.clone();
    marks.sort();
    for mark in &marks {
        match mark.kind {
            MarkKind::VNotch | MarkKind::SlitNotch => line(out, "NOTCH", mark.at, mark.toward),
            MarkKind::Drill => drill(out, mark.at),
        }
    }
    let mut labels = piece.labels.clone();
    labels.sort();
    let base = piece.grain.from;
    let metadata = [
        format!("Piece Name: {}", piece.name),
        format!("Size: {}", piece.size),
        format!("Quantity: {}", piece.quantity),
        format!("Style Name: {}", pattern.style),
    ];
    for (index, value) in metadata.iter().enumerate() {
        let y = base.y() - i64::try_from(index).unwrap_or(0) * 5_000;
        text(out, Point::new(base.x(), y).unwrap_or(base), value);
    }
    for label in &labels {
        if label.text.is_empty() || label.text.chars().any(char::is_control) {
            return Err(DxfError::BadLabel {
                block: name.to_owned(),
                text: label.text.clone(),
            });
        }
        text(out, label.at, &label.text);
    }
    pair(out, 0, "ENDBLK");
    pair(out, 8, "0");
    Ok(())
}

fn record(pattern: &Pattern, semantics: LineSemantics, flattening: Flattening) -> String {
    let text_value = |value: &str| Value::Text(value.to_owned());
    let mut axes = BTreeMap::new();
    axes.insert("layer_naming".to_owned(), text_value("aama_named"));
    axes.insert("dxf_version_target".to_owned(), text_value("r12"));
    axes.insert(
        "cut_sew_swap".to_owned(),
        text_value(match semantics {
            LineSemantics::CutAsCut => "cut_as_1",
            LineSemantics::SewAsCut => "sew_as_1",
        }),
    );
    axes.insert(
        "seam_allowance_policy".to_owned(),
        text_value("included_in_contour"),
    );
    axes.insert("grading_mode".to_owned(), text_value("single_size"));
    axes.insert("entity_policy".to_owned(), text_value("polyline_only"));
    let mut fields = BTreeMap::new();
    fields.insert("target".to_owned(), text_value(TARGET));
    fields.insert("axes".to_owned(), Value::Object(axes));
    fields.insert("unit".to_owned(), text_value("millimetre"));
    fields.insert(
        "tessellation_bound_um".to_owned(),
        Value::Int(flattening.bound),
    );
    fields.insert(
        "tessellation_achieved_um".to_owned(),
        Value::Int(flattening.achieved),
    );
    fields.insert("writer".to_owned(), text_value(WRITER));
    fields.insert("design".to_owned(), text_value(&pattern.design));
    fields.insert("design_revision".to_owned(), text_value(&pattern.revision));
    fields.insert("approval".to_owned(), text_value("unapproved"));
    // The named layers merge what numbered layers separate, and a receiver must be told rather than discover it.
    fields.insert(
        "losses".to_owned(),
        Value::List(vec![
            text_value("cut_layer_merges_boundary_turn_points_and_curve_points"),
            text_value("draw_layer_merges_sew_line_and_grainline"),
        ]),
    );
    Value::Object(fields).to_text()
}

/// Writes the pattern as the one DXF target G2 names, with its record.
///
/// # Errors
/// See [`DxfError`]. Nothing is returned on a refusal.
pub fn write_dxf(
    pattern: &Pattern,
    semantics: LineSemantics,
    flattening: Flattening,
) -> Result<DxfArtifact, DxfError> {
    if flattening.achieved > flattening.bound {
        return Err(DxfError::TessellationBound {
            bound: flattening.bound,
            achieved: flattening.achieved,
        });
    }
    let mut pieces: Vec<&DrawnPiece> = pattern.pieces.iter().collect();
    pieces.sort_by(|a, b| (&a.name, &a.size).cmp(&(&b.name, &b.size)));
    let mut names = Vec::with_capacity(pieces.len());
    for piece in &pieces {
        let name = block_name(piece)?;
        if names.contains(&name) {
            return Err(DxfError::DuplicateBlock { name });
        }
        names.push(name);
    }
    let mut out = String::new();
    pair(
        &mut out,
        999,
        &format!("{TARGET} unit=millimetre writer={WRITER}"),
    );
    pair(&mut out, 0, "SECTION");
    pair(&mut out, 2, "HEADER");
    pair(&mut out, 9, "$ACADVER");
    pair(&mut out, 1, "AC1009");
    pair(&mut out, 0, "ENDSEC");
    pair(&mut out, 0, "SECTION");
    pair(&mut out, 2, "TABLES");
    pair(&mut out, 0, "TABLE");
    pair(&mut out, 2, "LAYER");
    pair(&mut out, 70, "6");
    for (layer, colour) in [
        ("CUT", "1"),
        ("DRAW", "7"),
        ("DRILL", "6"),
        ("INTCUT", "4"),
        ("NOTCH", "3"),
        ("TEXT", "2"),
    ] {
        pair(&mut out, 0, "LAYER");
        pair(&mut out, 2, layer);
        pair(&mut out, 70, "0");
        pair(&mut out, 62, colour);
        pair(&mut out, 6, "CONTINUOUS");
    }
    pair(&mut out, 0, "ENDTAB");
    pair(&mut out, 0, "ENDSEC");
    pair(&mut out, 0, "SECTION");
    pair(&mut out, 2, "BLOCKS");
    for (piece, name) in pieces.iter().zip(&names) {
        write_block(&mut out, piece, name, pattern, semantics)?;
    }
    pair(&mut out, 0, "ENDSEC");
    pair(&mut out, 0, "SECTION");
    pair(&mut out, 2, "ENTITIES");
    for name in &names {
        pair(&mut out, 0, "INSERT");
        pair(&mut out, 8, "0");
        pair(&mut out, 2, name);
        pair(&mut out, 10, "0.0");
        pair(&mut out, 20, "0.0");
        pair(&mut out, 30, "0.0");
    }
    pair(&mut out, 0, "ENDSEC");
    pair(&mut out, 0, "EOF");
    Ok(DxfArtifact {
        text: out,
        record: record(pattern, semantics, flattening),
    })
}
