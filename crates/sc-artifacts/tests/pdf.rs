//! The PDF writer, read back by a small reader here that shares no code with it: dimensions measured out of the file itself.
#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]
use sc_artifacts::{
    write_pdf, DrawnPiece, Line, PageLayout, Pattern, PdfError, Sheet, SCALE_SQUARE,
};
use sc_geometry::Point;
use sc_store::parse;

fn p(x: i64, y: i64) -> Point {
    Point::new(x, y).expect("in range")
}

fn rectangle(name: &str, width: i64, height: i64) -> DrawnPiece {
    DrawnPiece {
        name: name.to_owned(),
        size: "M".to_owned(),
        quantity: 1,
        cut: vec![p(0, 0), p(width, 0), p(width, height), p(0, height)],
        sew: vec![
            p(1_000, 1_000),
            p(width - 1_000, 1_000),
            p(width - 1_000, height - 1_000),
            p(1_000, height - 1_000),
        ],
        grain: Line {
            from: p(width / 2, 2_000),
            to: p(width / 2, height - 2_000),
        },
        internal: vec![],
        marks: vec![],
        closures: vec![],
        labels: vec![],
    }
}

fn pattern(pieces: Vec<DrawnPiece>) -> Pattern {
    Pattern {
        style: "skirt".to_owned(),
        design: "01ARZ3NDEKTSV4RRFFQ69G5FAV".to_owned(),
        revision: "2".to_owned(),
        pieces,
    }
}

/// Each sheet's page size in millimetres, and its content stream as text.
fn read(bytes: &[u8]) -> Vec<((f64, f64), String)> {
    let text = String::from_utf8_lossy(bytes).into_owned();
    assert!(text.starts_with("%PDF-1.4\n") && text.ends_with("%%EOF\n"));
    // The cross-reference table must point at each object exactly, or a viewer must repair the file.
    let at_table: usize = text
        .rsplit("startxref\n")
        .next()
        .expect("a table")
        .lines()
        .next()
        .expect("a line")
        .parse()
        .expect("a number");
    assert!(text[at_table..].starts_with("xref\n"));
    let table: Vec<&str> = text[at_table..]
        .lines()
        .skip(3)
        .take_while(|line| !line.starts_with("trailer"))
        .collect();
    for (index, line) in table.iter().enumerate() {
        let offset: usize = line[..10].parse().expect("an offset");
        assert!(
            text[offset..].starts_with(&format!("{} 0 obj\n", index + 1)),
            "object {}",
            index + 1
        );
    }
    let mut sheets = Vec::new();
    let mut rest = text.as_str();
    while let Some(start) = rest.find("/MediaBox [0 0 ") {
        rest = &rest[start + "/MediaBox [0 0 ".len()..];
        let numbers: Vec<f64> = rest[..rest.find(']').expect("a box")]
            .split_whitespace()
            .map(|n| n.parse().expect("a number"))
            .collect();
        let begin = rest.find("stream\n").expect("a stream") + "stream\n".len();
        let end = rest.find("endstream").expect("a stream");
        let length: usize = rest[..begin]
            .rsplit("/Length ")
            .next()
            .expect("a length")
            .split_whitespace()
            .next()
            .expect("a number")
            .parse()
            .expect("a number");
        assert_eq!(length, end - begin, "the stream's stated length");
        sheets.push((
            (numbers[0] * 25.4 / 72.0, numbers[1] * 25.4 / 72.0),
            rest[begin..end].to_owned(),
        ));
        rest = &rest[end..];
    }
    sheets
}

/// The first `re` rectangle's sides in millimetres, the scale square.
fn first_square(content: &str) -> (f64, f64) {
    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 6 && parts[4] == "re" && parts[5] == "S" {
            return (
                parts[2].parse::<f64>().expect("a number") * 25.4 / 72.0,
                parts[3].parse::<f64>().expect("a number") * 25.4 / 72.0,
            );
        }
    }
    panic!("no scale square");
}

fn close(a: f64, b: f64, within: f64) -> bool {
    (a - b).abs() <= within
}

#[test]
fn the_scale_square_and_a_piece_measure_true_out_of_the_file() {
    let artifact = write_pdf(
        &pattern(vec![rectangle("front", 200_000, 300_000)]),
        PageLayout::Single(Sheet::A0),
    )
    .expect("writes");
    let sheets = read(&artifact.bytes);
    assert_eq!(sheets.len(), 1);
    assert_eq!(artifact.sheets, 1);
    let ((width, height), content) = &sheets[0];
    assert!(
        close(*width, 841.0, 0.001) && close(*height, 1189.0, 0.001),
        "A0: {width} {height}"
    );
    let (a, b) = first_square(content);
    assert!(
        close(a, 100.0, 0.0001) && close(b, 100.0, 0.0001),
        "the scale square: {a} {b}"
    );
    assert_eq!(SCALE_SQUARE, 100_000);

    // The cut boundary is the first closed path, and its sides measure exactly the piece's.
    let moves: Vec<(f64, f64)> = content
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            (parts.len() == 3 && (parts[2] == "m" || parts[2] == "l"))
                .then(|| (parts[0].parse().expect("x"), parts[1].parse().expect("y")))
        })
        .take(4)
        .collect();
    let side =
        |i: usize, j: usize| (moves[i].0 - moves[j].0).hypot(moves[i].1 - moves[j].1) * 25.4 / 72.0;
    assert!(close(side(0, 1), 200.0, 0.0001), "{}", side(0, 1));
    assert!(close(side(1, 2), 300.0, 0.0001), "{}", side(1, 2));
}

#[test]
fn a_drawing_that_does_not_fit_is_refused_and_never_shrunk() {
    let huge = pattern(vec![rectangle("long", 2_000_000, 500_000)]);
    let error = write_pdf(&huge, PageLayout::Single(Sheet::A0)).expect_err("too wide");
    assert!(
        matches!(error, PdfError::DoesNotFit { sheet: "a0", .. }),
        "{error}"
    );
    assert!(error.to_string().contains("never rescaled"));
    // Tiled, it goes across sheets at the same scale.
    let tiled = write_pdf(
        &huge,
        PageLayout::Tiled {
            sheet: Sheet::A4,
            overlap: 10_000,
        },
    )
    .expect("tiles");
    assert!(tiled.sheets > 1);
    let sheets = read(&tiled.bytes);
    assert_eq!(sheets.len(), tiled.sheets);
    for ((width, height), _) in &sheets {
        assert!(close(*width, 210.0, 0.001) && close(*height, 297.0, 0.001));
    }
    let (a, _) = first_square(&sheets[0].1);
    assert!(close(a, 100.0, 0.0001), "still true scale: {a}");
}

#[test]
fn tiled_sheets_carry_their_order_registration_marks_and_overlap() {
    let artifact = write_pdf(
        &pattern(vec![rectangle("long", 500_000, 400_000)]),
        PageLayout::Tiled {
            sheet: Sheet::A4,
            overlap: 10_000,
        },
    )
    .expect("tiles");
    let sheets = read(&artifact.bytes);
    let total = sheets.len();
    for (index, (_, content)) in sheets.iter().enumerate() {
        assert!(
            content.contains(&format!("(sheet {} of {total},", index + 1)),
            "sheet {index}"
        );
        // Four registration marks of two strokes each.
        assert_eq!(
            content
                .lines()
                .filter(|line| line.ends_with(" l S"))
                .count(),
            8,
            "sheet {index}"
        );
    }
    // Neighbouring sheets show the drawing moved by the sheet's usable width less the overlap: 190 - 10 mm.
    let origin = |content: &str| -> f64 {
        let line = content
            .lines()
            .find(|line| line.starts_with("1 0 0 1 "))
            .expect("a shift");
        line.split_whitespace()
            .nth(4)
            .expect("x")
            .parse::<f64>()
            .expect("a number")
            * 25.4
            / 72.0
    };
    let (first, second) = (origin(&sheets[0].1), origin(&sheets[1].1));
    assert!(close(first - second, 180.0, 0.0001), "{}", first - second);
    let record = parse(&artifact.record).expect("canonical text");
    let fields = record.as_object().expect("an object");
    assert_eq!(fields["overlap_um"].as_int(), Some(10_000));
    assert_eq!(
        fields["sheets"].as_int(),
        Some(i64::try_from(total).expect("a count"))
    );
}

#[test]
fn a_tiled_sheet_is_clipped_to_its_printable_area() {
    let artifact = write_pdf(
        &pattern(vec![rectangle("long", 500_000, 400_000)]),
        PageLayout::Tiled {
            sheet: Sheet::A4,
            overlap: 10_000,
        },
    )
    .expect("tiles");
    for (_, content) in read(&artifact.bytes) {
        assert!(
            content.lines().any(|line| line.ends_with("re W n")),
            "a clip"
        );
    }
}

#[test]
fn the_record_is_complete_and_two_runs_give_one_byte_string() {
    let artifact = write_pdf(
        &pattern(vec![rectangle("front", 200_000, 300_000)]),
        PageLayout::Single(Sheet::A0),
    )
    .expect("writes");
    let record = parse(&artifact.record).expect("canonical text");
    let fields = record.as_object().expect("an object");
    for (field, wanted) in [
        ("target", "pdf-a0"),
        ("scale", "true_scale_never_rescaled"),
        ("sheet", "a0"),
        ("design_revision", "2"),
        ("approval", "unapproved"),
        ("coordinate_quantum", "0.0001_point"),
    ] {
        assert_eq!(fields[field].as_text(), Some(wanted), "{field}");
    }
    assert_eq!(fields["scale_square_um"].as_int(), Some(100_000));
    let again = write_pdf(
        &pattern(vec![rectangle("front", 200_000, 300_000)]),
        PageLayout::Single(Sheet::A0),
    )
    .expect("writes");
    assert_eq!(artifact, again);
    let text = String::from_utf8_lossy(&artifact.bytes);
    for forbidden in ["CreationDate", "ModDate", "Producer", "/ID"] {
        assert!(!text.contains(forbidden), "{forbidden}");
    }
}

#[test]
fn bad_requests_are_refused() {
    let plain = pattern(vec![rectangle("front", 200_000, 300_000)]);
    assert_eq!(
        write_pdf(
            &plain,
            PageLayout::Tiled {
                sheet: Sheet::A4,
                overlap: -1
            }
        ),
        Err(PdfError::BadOverlap { overlap: -1 })
    );
    assert!(matches!(
        write_pdf(
            &plain,
            PageLayout::Tiled {
                sheet: Sheet::A4,
                overlap: 500_000
            }
        ),
        Err(PdfError::BadOverlap { .. })
    ));
    let mut empty = rectangle("front", 10, 10);
    empty.cut.clear();
    assert_eq!(
        write_pdf(&pattern(vec![empty]), PageLayout::Single(Sheet::A0)),
        Err(PdfError::EmptyPiece {
            piece: "front".to_owned()
        })
    );
}

#[test]
fn text_that_would_break_a_pdf_string_is_escaped() {
    let mut piece = rectangle("front", 200_000, 300_000);
    piece.labels = vec![sc_artifacts::Text {
        at: p(10_000, 10_000),
        text: "a(b)c\\d".to_owned(),
    }];
    let artifact = write_pdf(&pattern(vec![piece]), PageLayout::Single(Sheet::A0)).expect("writes");
    let (_, content) = read(&artifact.bytes).remove(0);
    assert!(content.contains("(a\\(b\\)c\\\\d) Tj"), "{content}");
}

#[test]
fn a_closed_line_starting_anywhere_writes_one_file() {
    let shape = || rectangle("front", 200_000, 300_000);
    let first = write_pdf(&pattern(vec![shape()]), PageLayout::Single(Sheet::A0)).expect("writes");
    let mut turned = shape();
    turned.cut.rotate_left(1);
    turned.sew.rotate_left(3);
    assert_eq!(
        write_pdf(&pattern(vec![turned]), PageLayout::Single(Sheet::A0)).expect("writes"),
        first
    );
}
