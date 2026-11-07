//! Each notch type, in each dialect, in each encoding: what the mark means is one thing, and how a file carries it is another.
#![allow(clippy::expect_used, clippy::panic, clippy::indexing_slicing)]
use sc_artifacts::{
    write_base_plus_rules, write_embedded_rules, DrawnPiece, Flattening, Line, Mark, MarkKind,
    NotchEncoding, Pattern,
};
use sc_geometry::Point;

const FLAT: Flattening = Flattening {
    bound: 100,
    achieved: 11,
};
const GOLDEN_NAMED: &str = include_str!("../../../conformance/goldens/notches/named.dxf");
const GOLDEN_DRAWN: &str = include_str!("../../../conformance/goldens/notches/numbered_drawn.dxf");
const GOLDEN_CODED: &str = include_str!("../../../conformance/goldens/notches/numbered_coded.dxf");

fn at(x: i64, y: i64) -> Point {
    Point::new(x, y).expect("in range")
}

fn notch(from: (i64, i64), toward: (i64, i64)) -> Mark {
    Mark {
        kind: MarkKind::VNotch,
        at: at(from.0, from.1),
        toward: at(toward.0, toward.1),
    }
}

// A square with a single notch on the bottom, a double on the right, and a drill mark in the middle.
fn pattern() -> Pattern {
    Pattern {
        style: "notches".to_owned(),
        design: "01ARZ3NDEKTSV4RRFFQ69G5FAV".to_owned(),
        revision: "1".to_owned(),
        seams: vec![],
        repairs: vec![],
        fits: vec![],
        pockets: vec![],
        buttons: vec![],
        pieces: vec![DrawnPiece {
            name: "square".to_owned(),
            size: "M".to_owned(),
            quantity: 1,
            cut: vec![
                at(0, 0),
                at(100_000, 0),
                at(100_000, 100_000),
                at(0, 100_000),
            ],
            sew: vec![
                at(10_000, 10_000),
                at(90_000, 10_000),
                at(90_000, 90_000),
                at(10_000, 90_000),
            ],
            grain: Line {
                from: at(20_000, 20_000),
                to: at(20_000, 80_000),
            },
            internal: vec![],
            marks: vec![
                notch((50_000, 0), (50_000, 5_000)),
                notch((100_000, 46_000), (95_000, 46_000)),
                notch((100_000, 54_000), (95_000, 54_000)),
                Mark {
                    kind: MarkKind::Drill,
                    at: at(50_000, 50_000),
                    toward: at(50_000, 50_000),
                },
            ],
            labels: vec![],
            closures: vec![],
            grade_points: vec![],
        }],
    }
}

// The entities of a kind on a layer in the piece's block, as their group codes and values.
fn entities(text: &str, kind: &str, layer: &str) -> Vec<Vec<(String, String)>> {
    let lines: Vec<&str> = text.lines().collect();
    let pairs: Vec<(&str, &str)> = lines
        .chunks(2)
        .map(|pair| (pair[0].trim(), pair[1].trim()))
        .collect();
    let mut out = Vec::new();
    let mut index = 0;
    while index < pairs.len() {
        if pairs[index] == ("0", kind) {
            let mut fields = Vec::new();
            let mut cursor = index + 1;
            while pairs[cursor].0 != "0" {
                fields.push((pairs[cursor].0.to_owned(), pairs[cursor].1.to_owned()));
                cursor += 1;
            }
            if fields
                .iter()
                .any(|(code, value)| code == "8" && value == layer)
            {
                out.push(fields);
            }
            index = cursor;
        } else {
            index += 1;
        }
    }
    out
}

fn value<'a>(fields: &'a [(String, String)], code: &str) -> &'a str {
    fields
        .iter()
        .find(|(have, _)| have == code)
        .map(|(_, value)| value.as_str())
        .unwrap_or_else(|| panic!("no group code {code}"))
}

fn written(notches: NotchEncoding) -> String {
    write_embedded_rules(&pattern(), &[], &[], ("M", "notches"), FLAT, notches)
        .expect("writes")
        .text
}

#[test]
fn a_named_file_draws_each_notch_on_its_own_layer_and_a_single_and_a_double_are_counted() {
    let text = write_base_plus_rules(&pattern(), &[], &[], FLAT)
        .expect("writes")
        .text;
    let lines = entities(&text, "LINE", "NOTCH");
    assert_eq!(lines.len(), 3, "one for the single and two for the double");
    assert_eq!(
        entities(&text, "POLYLINE", "DRILL").len(),
        1,
        "a named file's drill is two half circles"
    );
    assert!(
        entities(&text, "POINT", "NOTCH").is_empty(),
        "a named file has no coded form"
    );
    assert_eq!(text, GOLDEN_NAMED);
}

#[test]
fn a_numbered_file_draws_a_notch_on_the_internal_layer_by_default() {
    let text = written(NotchEncoding::Drawn);
    assert_eq!(entities(&text, "LINE", "8").len(), 3);
    assert!(entities(&text, "POINT", "4").is_empty());
    let drills = entities(&text, "POINT", "13");
    assert_eq!(drills.len(), 1, "a drill hole is a point");
    assert_eq!(value(&drills[0], "10"), "50.00");
    assert!(
        entities(&text, "POLYLINE", "13").is_empty(),
        "layer 13 cannot hold a polyline"
    );
    assert_eq!(text, GOLDEN_DRAWN);
}

#[test]
fn a_coded_notch_is_a_point_with_its_depth_and_its_angle() {
    let text = written(NotchEncoding::Coded);
    assert!(
        entities(&text, "LINE", "8").is_empty(),
        "nothing is drawn as well"
    );
    let notches = entities(&text, "POINT", "4");
    assert_eq!(notches.len(), 3);
    for fields in &notches {
        assert_eq!(value(fields, "30"), "5.00", "the depth");
        assert!(
            !fields.iter().any(|(code, _)| code == "39"),
            "a slit has no width, and a V's is a profile's"
        );
    }
    let at_of = |x: &str, y: &str| {
        notches
            .iter()
            .find(|fields| value(fields, "10") == x && value(fields, "20") == y)
            .unwrap_or_else(|| panic!("no notch at {x}, {y}"))
    };
    assert_eq!(
        value(at_of("50.00", "0.00"), "50"),
        "90.000",
        "the bottom's points up"
    );
    assert_eq!(
        value(at_of("100.00", "46.00"), "50"),
        "180.000",
        "the right's points left"
    );
    assert_eq!(value(at_of("100.00", "54.00"), "50"), "180.000");
    assert_eq!(text, GOLDEN_CODED);
}

#[test]
fn each_angle_is_counter_clockwise_from_x_all_the_way_round() {
    for (toward, wanted) in [
        ((5_000, 0), "0.000"),
        ((0, 5_000), "90.000"),
        ((-5_000, 0), "180.000"),
        ((0, -5_000), "270.000"),
        ((5_000, -5_000), "315.000"),
    ] {
        let mut drawn = pattern();
        drawn.pieces[0].marks = vec![notch(
            (50_000, 50_000),
            (50_000 + toward.0, 50_000 + toward.1),
        )];
        let text = write_embedded_rules(&drawn, &[], &[], ("M", "n"), FLAT, NotchEncoding::Coded)
            .expect("writes")
            .text;
        let fields = entities(&text, "POINT", "4");
        assert_eq!(value(&fields[0], "50"), wanted, "{toward:?}");
    }
}

#[test]
fn the_record_says_which_encoding_it_used() {
    for (notches, token) in [
        (NotchEncoding::Drawn, "drawn"),
        (NotchEncoding::Coded, "coded"),
    ] {
        let artifact =
            write_embedded_rules(&pattern(), &[], &[], ("M", "n"), FLAT, notches).expect("writes");
        assert!(
            artifact
                .record
                .contains(&format!("\"notch_encoding\": \"{token}\"")),
            "{}",
            artifact.record
        );
    }
}
