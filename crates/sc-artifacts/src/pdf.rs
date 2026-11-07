//! The PDF writer for the review and print path: true scale, with a scale square, on A0 or on tiled sheets. Nothing is ever rescaled, because
//! a rescaled print is a pattern nobody drafted.
use crate::canonical::from_lowest;
use crate::pattern::{DrawnPiece, Line, MarkKind, Pattern};
use core::fmt;
use sc_geometry::Point;
use sc_store::Value;
use std::collections::BTreeMap;
use std::fmt::Write as _;

/// The registered target this writes.
pub const PDF_TARGET: &str = "pdf-a0";
/// The writer's identity, recorded with each artifact.
pub const PDF_WRITER: &str = "stitchcad-pdf-1";
/// The scale square's side, in micrometres: 100 millimetres, to check with a ruler.
pub const SCALE_SQUARE: i64 = 100_000;

const MARGIN: i64 = 10_000;
const GAP: i64 = 10_000;
const REGISTRATION: i64 = 5_000;

/// A sheet's size, in micrometres.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sheet {
    /// 841 by 1189 millimetres.
    A0,
    /// 297 by 420 millimetres.
    A3,
    /// 210 by 297 millimetres.
    A4,
}

impl Sheet {
    const fn size(self) -> (i64, i64) {
        match self {
            Self::A0 => (841_000, 1_189_000),
            Self::A3 => (297_000, 420_000),
            Self::A4 => (210_000, 297_000),
        }
    }

    const fn token(self) -> &'static str {
        match self {
            Self::A0 => "a0",
            Self::A3 => "a3",
            Self::A4 => "a4",
        }
    }
}

/// How the drawing goes onto paper. Neither way rescales it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageLayout {
    /// One sheet, which must hold everything.
    Single(Sheet),
    /// As many sheets as it takes, each overlapping its neighbours by this many micrometres.
    Tiled {
        /// The sheet.
        sheet: Sheet,
        /// How far neighbouring sheets overlap, so they can be aligned and taped.
        overlap: i64,
    },
}

/// Why a file was not written. Nothing is written in that case.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PdfError {
    /// The drawing does not fit the one sheet. It is refused rather than shrunk.
    DoesNotFit {
        /// The sheet.
        sheet: &'static str,
        /// The drawing's width, in micrometres.
        width: i64,
        /// The drawing's height, in micrometres.
        height: i64,
    },
    /// The overlap leaves no new area on a tiled sheet.
    BadOverlap {
        /// The overlap asked for.
        overlap: i64,
    },
    /// A piece has no cut boundary to draw.
    EmptyPiece {
        /// The piece.
        piece: String,
    },
}

impl fmt::Display for PdfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DoesNotFit { sheet, width, height } => write!(
                f,
                "a drawing {width} by {height} micrometres does not fit one {sheet} sheet, and it is never rescaled. Tile it instead"
            ),
            Self::BadOverlap { overlap } => write!(f, "an overlap of {overlap} micrometres leaves a tiled sheet nothing new"),
            Self::EmptyPiece { piece } => write!(f, "piece {piece} has no cut boundary"),
        }
    }
}

impl std::error::Error for PdfError {}

/// A written file and the record that travels with it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PdfArtifact {
    /// The file.
    pub bytes: Vec<u8>,
    /// The record, as canonical text.
    pub record: String,
    /// How many sheets it takes.
    pub sheets: usize,
}

// Whole micrometres as points, exactly as a fraction of 25400/72, written to four places: under 0.04 micrometres of rounding.
fn points(micrometres: i64) -> String {
    let tenths = i128::from(micrometres) * 72 * 10_000;
    let rounded = (tenths * 2 + 25_400).div_euclid(50_800);
    let sign = if rounded < 0 { "-" } else { "" };
    let magnitude = rounded.unsigned_abs();
    format!("{sign}{}.{:04}", magnitude / 10_000, magnitude % 10_000)
}

fn escaped(text: &str) -> String {
    text.chars()
        .filter(|c| !c.is_control())
        .flat_map(|c| match c {
            '(' | ')' | '\\' => vec!['\\', c],
            other if other.is_ascii() => vec![other],
            _ => vec!['?'],
        })
        .collect()
}

struct Placed<'a> {
    piece: &'a DrawnPiece,
    // Moves the piece's own coordinates onto the drawing, whose origin is its lower left.
    dx: i64,
    dy: i64,
}

fn bounds(points: &[Point]) -> (i64, i64, i64, i64) {
    let (mut low_x, mut low_y, mut high_x, mut high_y) = (i64::MAX, i64::MAX, i64::MIN, i64::MIN);
    for at in points {
        low_x = low_x.min(at.x());
        low_y = low_y.min(at.y());
        high_x = high_x.max(at.x());
        high_y = high_y.max(at.y());
    }
    (low_x, low_y, high_x, high_y)
}

// Lays pieces in rows in canonical order, with the scale square first. Returns the pieces and the drawing's size.
fn lay_out<'a>(pieces: &[&'a DrawnPiece], row_width: i64) -> (Vec<Placed<'a>>, i64, i64) {
    let mut placed = Vec::with_capacity(pieces.len());
    let (mut x, mut y, mut row_height, mut width) =
        (SCALE_SQUARE + GAP, 0, SCALE_SQUARE, SCALE_SQUARE);
    for piece in pieces {
        let (low_x, low_y, high_x, high_y) = bounds(&piece.cut);
        let (w, h) = (high_x - low_x, high_y - low_y);
        if x > 0 && x + w > row_width {
            y += row_height + GAP;
            x = 0;
            row_height = 0;
        }
        placed.push(Placed {
            piece,
            dx: x - low_x,
            dy: y - low_y,
        });
        width = width.max(x + w);
        x += w + GAP;
        row_height = row_height.max(h);
    }
    (placed, width, y + row_height)
}

fn path(out: &mut String, at: &[Point], dx: i64, dy: i64, closed: bool) {
    for (index, point) in at.iter().enumerate() {
        let verb = if index == 0 { "m" } else { "l" };
        let _ = writeln!(
            out,
            "{} {} {verb}",
            points(point.x() + dx),
            points(point.y() + dy)
        );
    }
    out.push_str(if closed { "s\n" } else { "S\n" });
}

fn draw_line(out: &mut String, line: Line, dx: i64, dy: i64) {
    path(out, &[line.from, line.to], dx, dy, false);
}

fn label(out: &mut String, x: i64, y: i64, size: u32, text: &str) {
    let _ = writeln!(
        out,
        "BT /F1 {size} Tf {} {} Td ({}) Tj ET",
        points(x),
        points(y),
        escaped(text)
    );
}

fn square(out: &mut String, x: i64, y: i64, side: i64) {
    let _ = writeln!(
        out,
        "{} {} {} {} re S",
        points(x),
        points(y),
        points(side),
        points(side)
    );
}

// The whole drawing, in the drawing's own coordinates. Sheets shift it into place.
fn drawing(placed: &[Placed<'_>], style: &str) -> Result<String, PdfError> {
    let mut out = String::new();
    out.push_str("0.25 w\n0 G\n");
    square(&mut out, 0, 0, SCALE_SQUARE);
    label(
        &mut out,
        5_000,
        5_000,
        10,
        "scale square: 100 mm on each side",
    );
    for item in placed {
        let piece = item.piece;
        if piece.cut.len() < 3 {
            return Err(PdfError::EmptyPiece {
                piece: piece.name.clone(),
            });
        }
        let (dx, dy) = (item.dx, item.dy);
        out.push_str("0.5 w\n");
        // The canonical first point, so one pattern gives one file however it was reached.
        path(&mut out, &from_lowest(&piece.cut), dx, dy, true);
        out.push_str("0.25 w\n[3 2] 0 d\n");
        path(&mut out, &from_lowest(&piece.sew), dx, dy, true);
        out.push_str("[] 0 d\n");
        draw_line(&mut out, piece.grain, dx, dy);
        for line in &piece.internal {
            draw_line(&mut out, *line, dx, dy);
        }
        for mark in &piece.marks {
            match mark.kind {
                MarkKind::VNotch | MarkKind::SlitNotch => draw_line(
                    &mut out,
                    Line {
                        from: mark.at,
                        to: mark.toward,
                    },
                    dx,
                    dy,
                ),
                MarkKind::Drill => square(
                    &mut out,
                    mark.at.x() + dx - 1_000,
                    mark.at.y() + dy - 1_000,
                    2_000,
                ),
            }
        }
        let base = piece.grain.from;
        label(
            &mut out,
            base.x() + dx + 2_000,
            base.y() + dy,
            10,
            &format!(
                "{} {} of {style}, cut {}",
                piece.name, piece.size, piece.quantity
            ),
        );
        for text in &piece.labels {
            label(&mut out, text.at.x() + dx, text.at.y() + dy, 8, &text.text);
        }
    }
    Ok(out)
}

// One sheet's content: the drawing moved into place and clipped to the printable area, with marks to align it by.
fn sheet_content(
    picture: &str,
    offset: (i64, i64),
    sheet: (i64, i64),
    page: Option<(usize, usize, usize, usize)>,
) -> String {
    let (width, height) = sheet;
    let mut out = String::new();
    out.push_str("q\n");
    let _ = writeln!(
        out,
        "{} {} {} {} re W n",
        points(MARGIN),
        points(MARGIN),
        points(width - 2 * MARGIN),
        points(height - 2 * MARGIN)
    );
    let _ = writeln!(
        out,
        "1 0 0 1 {} {} cm",
        points(MARGIN - offset.0),
        points(MARGIN - offset.1)
    );
    out.push_str(picture);
    out.push_str("Q\n");
    if let Some((number, total, row, column)) = page {
        // Registration marks at the printable corners, which neighbouring sheets share within their overlap.
        out.push_str("0.25 w\n0 G\n");
        for (x, y) in [
            (MARGIN, MARGIN),
            (width - MARGIN, MARGIN),
            (width - MARGIN, height - MARGIN),
            (MARGIN, height - MARGIN),
        ] {
            let (near_x, near_y) = (x - REGISTRATION, y - REGISTRATION);
            let _ = writeln!(
                out,
                "{} {} m {} {} l S",
                points(near_x),
                points(y),
                points(x + REGISTRATION),
                points(y)
            );
            let _ = writeln!(
                out,
                "{} {} m {} {} l S",
                points(x),
                points(near_y),
                points(x),
                points(y + REGISTRATION)
            );
        }
        let _ = writeln!(
            out,
            "BT /F1 8 Tf {} {} Td (sheet {number} of {total}, row {row}, column {column}) Tj ET",
            points(MARGIN + 2_000),
            points(height - MARGIN + 1_000)
        );
    }
    out
}

fn assemble(contents: &[String], sheet: (i64, i64)) -> Vec<u8> {
    // Objects: 1 catalog, 2 page tree, 3 font, then a page and its content for each sheet.
    let count = contents.len();
    let mut objects: Vec<Vec<u8>> = Vec::with_capacity(3 + 2 * count);
    objects.push(b"<< /Type /Catalog /Pages 2 0 R >>".to_vec());
    let kids: Vec<String> = (0..count).map(|at| format!("{} 0 R", 4 + at * 2)).collect();
    objects.push(
        format!(
            "<< /Type /Pages /Kids [{}] /Count {count} >>",
            kids.join(" ")
        )
        .into_bytes(),
    );
    objects.push(b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_vec());
    for (at, content) in contents.iter().enumerate() {
        objects.push(
            format!(
                "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Resources << /Font << /F1 3 0 R >> >> /Contents {} 0 R >>",
                points(sheet.0),
                points(sheet.1),
                5 + at * 2
            )
            .into_bytes(),
        );
        objects.push(
            format!(
                "<< /Length {} >>\nstream\n{content}endstream",
                content.len()
            )
            .into_bytes(),
        );
    }
    let mut out = b"%PDF-1.4\n".to_vec();
    let mut offsets = Vec::with_capacity(objects.len());
    for (at, object) in objects.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n", at + 1).as_bytes());
        out.extend_from_slice(object);
        out.extend_from_slice(b"\nendobj\n");
    }
    let table = out.len();
    out.extend_from_slice(
        format!("xref\n0 {}\n0000000000 65535 f \n", objects.len() + 1).as_bytes(),
    );
    for offset in offsets {
        out.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{table}\n%%EOF\n",
            objects.len() + 1
        )
        .as_bytes(),
    );
    out
}

fn record(
    pattern: &Pattern,
    layout: PageLayout,
    sheets: usize,
    width: i64,
    height: i64,
    turned: bool,
) -> String {
    let text = |value: &str| Value::Text(value.to_owned());
    let (sheet, overlap) = match layout {
        PageLayout::Single(sheet) => (sheet, 0),
        PageLayout::Tiled { sheet, overlap } => (sheet, overlap),
    };
    let mut fields = BTreeMap::new();
    fields.insert("target".to_owned(), text(PDF_TARGET));
    fields.insert("writer".to_owned(), text(PDF_WRITER));
    fields.insert("unit".to_owned(), text("point"));
    fields.insert("scale".to_owned(), text("true_scale_never_rescaled"));
    fields.insert("sheet".to_owned(), text(sheet.token()));
    fields.insert(
        "orientation".to_owned(),
        text(if turned { "landscape" } else { "portrait" }),
    );
    fields.insert(
        "sheets".to_owned(),
        Value::Int(i64::try_from(sheets).unwrap_or(i64::MAX)),
    );
    fields.insert("overlap_um".to_owned(), Value::Int(overlap));
    fields.insert("drawing_width_um".to_owned(), Value::Int(width));
    fields.insert("drawing_height_um".to_owned(), Value::Int(height));
    fields.insert("scale_square_um".to_owned(), Value::Int(SCALE_SQUARE));
    fields.insert("coordinate_quantum".to_owned(), text("0.0001_point"));
    fields.insert("design".to_owned(), text(&pattern.design));
    fields.insert("design_revision".to_owned(), text(&pattern.revision));
    fields.insert("approval".to_owned(), text("unapproved"));
    Value::Object(fields).to_text()
}

/// Writes the pattern at true scale, with a scale square.
///
/// # Errors
/// See [`PdfError`]. A drawing that fits one sheet neither upright nor on its side is refused and never shrunk.
pub fn write_pdf(pattern: &Pattern, layout: PageLayout) -> Result<PdfArtifact, PdfError> {
    let mut pieces: Vec<&DrawnPiece> = pattern.pieces.iter().collect();
    pieces.sort_by(|a, b| (&a.name, &a.size).cmp(&(&b.name, &b.size)));
    if let Some(piece) = pieces.iter().find(|piece| piece.cut.is_empty()) {
        return Err(PdfError::EmptyPiece {
            piece: piece.name.clone(),
        });
    }
    let (sheet, overlap) = match layout {
        PageLayout::Single(sheet) => (sheet, 0),
        PageLayout::Tiled { sheet, overlap } => (sheet, overlap),
    };
    let portrait = sheet.size();
    let (usable_x, usable_y) = (portrait.0 - 2 * MARGIN, portrait.1 - 2 * MARGIN);
    let (placed, width, height, size) = match layout {
        PageLayout::Single(_) => {
            // A sheet turned on its side is the same sheet at the same scale, so that is tried before refusing.
            let upright = lay_out(&pieces, usable_x);
            if upright.1 <= usable_x && upright.2 <= usable_y {
                (upright.0, upright.1, upright.2, portrait)
            } else {
                let turned = lay_out(&pieces, usable_y);
                if turned.1 > usable_y || turned.2 > usable_x {
                    return Err(PdfError::DoesNotFit {
                        sheet: sheet.token(),
                        width: upright.1,
                        height: upright.2,
                    });
                }
                (turned.0, turned.1, turned.2, (portrait.1, portrait.0))
            }
        }
        PageLayout::Tiled { .. } => {
            let tiled = lay_out(&pieces, usable_x * 4);
            (tiled.0, tiled.1, tiled.2, portrait)
        }
    };
    let picture = drawing(&placed, &pattern.style)?;
    let contents = match layout {
        PageLayout::Single(_) => vec![sheet_content(&picture, (0, 0), size, None)],
        PageLayout::Tiled { .. } => {
            if overlap < 0 || overlap >= usable_x || overlap >= usable_y {
                return Err(PdfError::BadOverlap { overlap });
            }
            let (step_x, step_y) = (usable_x - overlap, usable_y - overlap);
            let columns = 1 + (width - overlap - 1).max(0) / step_x;
            let rows = 1 + (height - overlap - 1).max(0) / step_y;
            let total = usize::try_from(rows * columns).unwrap_or(0);
            let mut out = Vec::with_capacity(total);
            for row in 0..rows {
                for column in 0..columns {
                    let number = usize::try_from(row * columns + column + 1).unwrap_or(0);
                    let at = (
                        usize::try_from(row + 1).unwrap_or(0),
                        usize::try_from(column + 1).unwrap_or(0),
                    );
                    out.push(sheet_content(
                        &picture,
                        (column * step_x, row * step_y),
                        size,
                        Some((number, total, at.0, at.1)),
                    ));
                }
            }
            out
        }
    };
    let sheets = contents.len();
    Ok(PdfArtifact {
        bytes: assemble(&contents, size),
        record: record(pattern, layout, sheets, width, height, size.0 > size.1),
        sheets,
    })
}
