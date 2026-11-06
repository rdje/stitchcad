//! One pattern has one written form, a comparison tells meaning from formatting, and each field's change is reported as itself.
#![allow(clippy::expect_used, clippy::panic, clippy::indexing_slicing)]
use sc_artifacts::{
    compare, from_canonical_text, to_canonical_text, Comparison, DrawnPiece, Line, Mark, MarkKind,
    Pattern, Text,
};
use sc_geometry::Point;

fn p(x: i64, y: i64) -> Point {
    Point::new(x, y).expect("in range")
}

fn front() -> DrawnPiece {
    DrawnPiece {
        name: "front".to_owned(),
        size: "M".to_owned(),
        quantity: 2,
        cut: vec![
            p(-1000, -1000),
            p(101_000, -1000),
            p(101_000, 101_000),
            p(-1000, 101_000),
        ],
        sew: vec![p(0, 0), p(100_000, 0), p(100_000, 100_000), p(0, 100_000)],
        grain: Line {
            from: p(50_000, 10_000),
            to: p(50_000, 90_000),
        },
        internal: vec![
            Line {
                from: p(20_000, 100_000),
                to: p(25_000, 80_000),
            },
            Line {
                from: p(30_000, 100_000),
                to: p(25_000, 80_000),
            },
        ],
        marks: vec![
            Mark {
                kind: MarkKind::VNotch,
                at: p(100_000, 50_000),
                toward: p(99_000, 50_000),
            },
            Mark {
                kind: MarkKind::Drill,
                at: p(25_000, 82_000),
                toward: p(25_000, 82_000),
            },
        ],
        labels: vec![
            Text {
                at: p(40_000, 40_000),
                text: "front".to_owned(),
            },
            Text {
                at: p(40_000, 35_000),
                text: "size M".to_owned(),
            },
        ],
    }
}

fn back() -> DrawnPiece {
    DrawnPiece {
        name: "back".to_owned(),
        ..front()
    }
}

fn pattern(pieces: Vec<DrawnPiece>) -> Pattern {
    Pattern {
        style: "skirt".to_owned(),
        design: "01ARZ3NDEKTSV4RRFFQ69G5FAV".to_owned(),
        revision: "3".to_owned(),
        pieces,
    }
}

#[test]
fn drawing_order_and_starting_points_do_not_change_the_form() {
    let expected = to_canonical_text(&pattern(vec![front(), back()]));
    assert_eq!(
        to_canonical_text(&pattern(vec![back(), front()])),
        expected,
        "piece order"
    );
    let mut shuffled = front();
    shuffled.internal.reverse();
    shuffled.marks.reverse();
    shuffled.labels.reverse();
    shuffled.cut.rotate_left(2);
    shuffled.sew.rotate_left(3);
    assert_eq!(
        to_canonical_text(&pattern(vec![shuffled, back()])),
        expected,
        "marks and starting points"
    );
}

#[test]
fn the_form_reads_back_to_an_equal_pattern_and_holds_no_clock() {
    let text = to_canonical_text(&pattern(vec![front(), back()]));
    let back_again = from_canonical_text(&text).expect("reads");
    assert_eq!(to_canonical_text(&back_again), text);
    for forbidden in ["time", "date", "created", "modified", "version", "host"] {
        assert!(!text.contains(forbidden), "{forbidden}");
    }
}

#[test]
fn formatting_is_not_a_change_of_meaning() {
    let text = to_canonical_text(&pattern(vec![front()]));
    assert_eq!(
        compare(&text, &text, 0).expect("reads"),
        Comparison::Identical
    );
    let reformatted = text.replace("\n  ", " ").replace('\n', " ");
    assert_ne!(reformatted, text);
    assert_eq!(
        compare(&text, &reformatted, 0).expect("reads"),
        Comparison::FormattingOnly
    );
}

fn only_difference(changed: DrawnPiece, field: &str) {
    let (a, b) = (
        to_canonical_text(&pattern(vec![front()])),
        to_canonical_text(&pattern(vec![changed])),
    );
    let Comparison::Different(found) = compare(&a, &b, 0).expect("reads") else {
        panic!("{field} must be reported");
    };
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(
        (found[0].piece.as_str(), found[0].field),
        ("front", field),
        "{found:?}"
    );
}

#[test]
fn each_field_reports_as_itself() {
    let mut piece = front();
    piece.quantity = 1;
    only_difference(piece, "quantity");
    let mut piece = front();
    piece.cut[1] = p(101_001, -1000);
    only_difference(piece, "cut");
    let mut piece = front();
    piece.cut.pop();
    only_difference(piece, "cut");
    let mut piece = front();
    piece.sew[2] = p(100_001, 100_000);
    only_difference(piece, "sew");
    let mut piece = front();
    piece.grain.to = p(50_000, 90_001);
    only_difference(piece, "grain");
    let mut piece = front();
    piece.grain = Line {
        from: piece.grain.to,
        to: piece.grain.from,
    };
    only_difference(piece, "grain");
    let mut piece = front();
    piece.internal.pop();
    only_difference(piece, "internal");
    let mut piece = front();
    piece.marks.pop();
    only_difference(piece, "marks");
    let mut piece = front();
    piece.marks[0].kind = MarkKind::SlitNotch;
    only_difference(piece, "marks");
    let mut piece = front();
    piece.marks[0].at = p(100_000, 50_001);
    only_difference(piece, "marks");
    let mut piece = front();
    piece.labels[0].text = "frnt".to_owned();
    only_difference(piece, "labels");
}

#[test]
fn a_missing_piece_and_an_added_piece_are_named() {
    let (one, two) = (
        to_canonical_text(&pattern(vec![front()])),
        to_canonical_text(&pattern(vec![front(), back()])),
    );
    let Comparison::Different(found) = compare(&two, &one, 0).expect("reads") else {
        panic!("different")
    };
    assert_eq!(
        (found[0].piece.as_str(), found[0].field),
        ("back", "missing")
    );
    let Comparison::Different(found) = compare(&one, &two, 0).expect("reads") else {
        panic!("different")
    };
    assert_eq!((found[0].piece.as_str(), found[0].field), ("back", "added"));
    let mut later = pattern(vec![front()]);
    later.revision = "4".to_owned();
    let Comparison::Different(found) = compare(&one, &to_canonical_text(&later), 0).expect("reads")
    else {
        panic!("different")
    };
    assert_eq!((found[0].piece.as_str(), found[0].field), ("", "revision"));
}

#[test]
fn a_tolerance_lets_a_small_move_through_and_says_how_far() {
    let mut piece = front();
    piece.sew[1] = p(100_007, 0);
    let (a, b) = (
        to_canonical_text(&pattern(vec![front()])),
        to_canonical_text(&pattern(vec![piece])),
    );
    assert_eq!(
        compare(&a, &b, 7).expect("reads"),
        Comparison::WithinTolerance { largest_move: 7 }
    );
    assert!(matches!(
        compare(&a, &b, 6).expect("reads"),
        Comparison::Different(_)
    ));
}

#[test]
fn a_text_that_is_not_a_pattern_is_refused() {
    for (text, words) in [
        ("not text", "canonical pattern"),
        ("[]", "must be an object"),
        ("{\"format\": 2}", "reads format 1"),
        ("{\"format\": 1}", "no style"),
        ("{\"format\": 1, \"style\": \"a\", \"design\": \"b\", \"revision\": \"c\", \"pieces\": [1]}", "must be an object"),
    ] {
        let error = from_canonical_text(text).expect_err(text);
        assert!(error.to_string().contains(words), "{text}: {error}");
    }
}

// A frozen file, so a run on another system or another day that writes different bytes fails. Hosted CI runs this on Linux.
const GOLDEN: &str = include_str!("../../../conformance/goldens/canonical-two-pieces.json");

#[test]
fn the_form_equals_its_frozen_golden_byte_for_byte() {
    assert_eq!(to_canonical_text(&pattern(vec![front(), back()])), GOLDEN);
    assert_eq!(
        compare(GOLDEN, GOLDEN, 0).expect("reads"),
        Comparison::Identical
    );
}
