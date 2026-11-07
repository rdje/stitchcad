//! The metamorphic suite the roadmap's §13 asks for: rigid transforms and mirror-twice, on the pathology corpus, each with its tolerance class.
//!
//! Mirror-twice is T1, identical by construction. On straight edges a transformed offset against the offset transformed is exact or T1,
//! because nothing is approximated. A curve is flattened again after it moves, and a float rounding can move one chord's end across the
//! flatness test, so two polylines each within T2 of one true curve may be up to twice T2 apart. Every bound below is derived that way.
#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]
use sc_geometry::{Allowance, Corner, Curve, Outline, Point};
use sc_store::{parse, Value};
use std::collections::BTreeMap;

const CASES: &str = include_str!("../../../conformance/offsets/cases.json");
/// T1 of the units chapter.
const T1: f64 = 1.0;
/// T2 of the units chapter, for internal geometry.
const T2: f64 = 10.0;
/// Two independent approximations of one curve, each within T2 of it.
const TWO_APPROXIMATIONS: f64 = 2.0 * T2;
// The most any point of the net can move when it is rounded to the micrometre grid.
const ROUNDING: f64 = core::f64::consts::FRAC_1_SQRT_2;

struct Case {
    name: String,
    curves: Vec<(String, Vec<(i64, i64)>)>,
    allowances: Vec<Allowance>,
    tolerance: i64,
}

impl Case {
    fn straight(&self) -> bool {
        self.curves.iter().all(|(kind, _)| kind == "line")
    }

    // What the result may differ by when nothing but rounding changed: exact arithmetic on straight edges, and a chord's freedom on curves.
    fn bound(&self) -> f64 {
        if self.straight() {
            2.0 * T1
        } else {
            TWO_APPROXIMATIONS
        }
    }
}

// The sharpest corner of a closed line, in radians.
fn sharpest(points: &[Point]) -> f64 {
    let count = points.len();
    let mut least = core::f64::consts::PI;
    for at in 0..count {
        let (a, b, c) = (
            points[at],
            points[(at + 1) % count],
            points[(at + 2) % count],
        );
        let (ux, uy) = ((a.x() - b.x()) as f64, (a.y() - b.y()) as f64);
        let (vx, vy) = ((c.x() - b.x()) as f64, (c.y() - b.y()) as f64);
        let angle = (ux * vy - uy * vx).abs().atan2(ux * vx + uy * vy);
        if angle > 0.0 {
            least = least.min(angle);
        }
    }
    least
}

fn cases() -> Vec<Case> {
    let parsed = parse(CASES).expect("valid");
    let listed = parsed.as_object().expect("an object")["cases"]
        .as_list()
        .expect("a list")
        .to_vec();
    let mut out = Vec::new();
    for case in &listed {
        let fields: &BTreeMap<String, Value> = case.as_object().expect("a case");
        if fields.contains_key("expect_refusal") {
            continue;
        }
        let pair = |value: &Value| {
            let items = value.as_list().expect("a point");
            (items[0].as_int().expect("x"), items[1].as_int().expect("y"))
        };
        out.push(Case {
            name: fields["name"].as_text().expect("a name").to_owned(),
            curves: fields["curves"]
                .as_list()
                .expect("curves")
                .iter()
                .map(|curve| {
                    let curve = curve.as_object().expect("a curve");
                    (
                        curve["kind"].as_text().expect("a kind").to_owned(),
                        curve["points"]
                            .as_list()
                            .expect("points")
                            .iter()
                            .map(pair)
                            .collect(),
                    )
                })
                .collect(),
            allowances: fields["allowances"]
                .as_list()
                .expect("allowances")
                .iter()
                .map(|item| {
                    let item = item.as_object().expect("an allowance");
                    Allowance {
                        width: item["width"].as_int().expect("a width"),
                        corner: match item["corner"].as_text().expect("a corner") {
                            "miter" => Corner::Miter,
                            "slant" => Corner::Slant,
                            "trim" => Corner::Trim,
                            "step" => Corner::Step,
                            "envelope" => Corner::Envelope,
                            other => panic!("{other}"),
                        },
                    }
                })
                .collect(),
            tolerance: fields["tolerance"].as_int().expect("a tolerance"),
        });
    }
    out
}

fn outline(case: &Case, map: impl Fn((i64, i64)) -> (i64, i64)) -> Outline {
    let curves = case
        .curves
        .iter()
        .map(|(kind, points)| {
            let moved: Vec<(i64, i64)> = points.iter().map(|point| map(*point)).collect();
            Curve::from_kind(kind, &moved).expect("a curve")
        })
        .collect();
    Outline::new(curves).expect("a closed boundary")
}

fn distance_to_segment(at: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let length = dx * dx + dy * dy;
    let t = if length == 0.0 {
        0.0
    } else {
        (((at.0 - a.0) * dx + (at.1 - a.1) * dy) / length).clamp(0.0, 1.0)
    };
    (at.0 - a.0 - t * dx).hypot(at.1 - a.1 - t * dy)
}

// The farthest either closed line gets from the other, checked at each vertex and along each edge.
fn apart(one: &[(f64, f64)], other: &[(f64, f64)]) -> f64 {
    let one_way = |from: &[(f64, f64)], to: &[(f64, f64)]| -> f64 {
        let mut worst: f64 = 0.0;
        for index in 0..from.len() {
            let (a, b) = (from[index], from[(index + 1) % from.len()]);
            for step in 0..4 {
                let t = f64::from(step) / 4.0;
                let at = (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t);
                let nearest = (0..to.len())
                    .map(|i| distance_to_segment(at, to[i], to[(i + 1) % to.len()]))
                    .fold(f64::MAX, f64::min);
                worst = worst.max(nearest);
            }
        }
        worst
    };
    one_way(one, other).max(one_way(other, one))
}

fn floats(points: &[Point], map: impl Fn(f64, f64) -> (f64, f64)) -> Vec<(f64, f64)> {
    points
        .iter()
        .map(|at| map(at.x() as f64, at.y() as f64))
        .collect()
}

fn offsets_of(case: &Case, net: &Outline, allowances: &[Allowance]) -> Option<Vec<Point>> {
    match net.offset(allowances, case.tolerance) {
        Ok(offset) => Some(offset.points().to_vec()),
        Err(_) => None,
    }
}

#[test]
fn mirroring_a_boundary_twice_returns_exactly_the_original() {
    let mirror = |(x, y): (i64, i64)| (-x, y);
    let mut count = 0;
    for case in cases() {
        let start = outline(&case, |at| at);
        let twice = outline(&case, |at| mirror(mirror(at)));
        // Exact, so tighter than T1 asks.
        assert_eq!(
            start.polyline(case.tolerance),
            twice.polyline(case.tolerance),
            "{}",
            case.name
        );
        assert_eq!(
            start.doubled_area(case.tolerance),
            twice.doubled_area(case.tolerance),
            "{}",
            case.name
        );
        count += 1;
    }
    assert!(count >= 30, "{count}");
}

#[test]
fn moving_a_piece_moves_its_offset_and_nothing_else() {
    let mut compared = 0;
    for shift in [(1_000_000, -2_000_000), (7, 13), (-123_457, 987_653)] {
        for case in cases() {
            let start = outline(&case, |at| at);
            let Some(base) = offsets_of(&case, &start, &case.allowances) else {
                continue;
            };
            let moved = outline(&case, |at| (at.0 + shift.0, at.1 + shift.1));
            let Some(after) = offsets_of(&case, &moved, &case.allowances) else {
                panic!("{}: refused only after moving by {shift:?}", case.name);
            };
            if case.straight() {
                let shifted: Vec<(i64, i64)> = base
                    .iter()
                    .map(|at| (at.x() + shift.0, at.y() + shift.1))
                    .collect();
                let got: Vec<(i64, i64)> = after.iter().map(|at| (at.x(), at.y())).collect();
                assert_eq!(shifted, got, "{}: straight edges move exactly", case.name);
            }
            let expected = floats(&base, |x, y| (x + shift.0 as f64, y + shift.1 as f64));
            let distance = apart(&expected, &floats(&after, |x, y| (x, y)));
            assert!(
                distance <= case.bound(),
                "{} moved by {shift:?}: {distance}",
                case.name
            );
            // The length of a seam cannot depend on where the piece lies.
            let length = |points: &[Point]| -> f64 {
                (0..points.len())
                    .map(|i| {
                        let (a, b) = (points[i], points[(i + 1) % points.len()]);
                        ((b.x() - a.x()) as f64).hypot((b.y() - a.y()) as f64)
                    })
                    .sum()
            };
            assert!(
                (length(&base) - length(&after)).abs() <= 2.0 * case.bound(),
                "{}: {} against {}",
                case.name,
                length(&base),
                length(&after)
            );
            compared += 1;
        }
    }
    assert!(compared >= 90, "{compared}");
}

#[test]
fn a_quarter_turn_turns_the_offset_with_it() {
    let turn = |(x, y): (i64, i64)| (-y, x);
    let mut compared = 0;
    for case in cases() {
        let start = outline(&case, |at| at);
        let Some(base) = offsets_of(&case, &start, &case.allowances) else {
            continue;
        };
        let turned = outline(&case, turn);
        let Some(after) = offsets_of(&case, &turned, &case.allowances) else {
            panic!("{}: refused only after a quarter turn", case.name);
        };
        let expected = floats(&base, |x, y| (-y, x));
        let distance = apart(&expected, &floats(&after, |x, y| (x, y)));
        assert!(distance <= case.bound(), "{}: {distance}", case.name);
        compared += 1;
    }
    assert!(compared >= 30, "{compared}");
}

#[test]
fn an_arbitrary_turn_stays_within_the_class_where_the_shape_itself_survives_rounding() {
    // Turning rounds the net's own points to the micrometre, so the two results differ by that as well, once per vertex.
    let (sine, cosine) = 0.5_f64.sin_cos();
    let turn = |(x, y): (i64, i64)| {
        let (x, y) = (x as f64, y as f64);
        (
            (x * cosine - y * sine).round() as i64,
            (x * sine + y * cosine).round() as i64,
        )
    };
    let (mut compared, mut refused) = (0, 0);
    for case in cases() {
        let start = outline(&case, |at| at);
        let Some(base) = offsets_of(&case, &start, &case.allowances) else {
            continue;
        };
        // A sliver may legitimately change its answer when its points round, so a refusal here is counted, and a wrong answer is not allowed.
        let Some(after) = offsets_of(&case, &outline(&case, turn), &case.allowances) else {
            refused += 1;
            continue;
        };
        let expected = floats(&base, |x, y| (x * cosine - y * sine, x * sine + y * cosine));
        let distance = apart(&expected, &floats(&after, |x, y| (x, y)));
        // Each net point rounds by up to `ROUNDING`, and where two offset lines meet at a sharp corner that moves the meeting point by up to
        // that rounding over the sine of half the angle. So this bound comes from the corner, and an error in the engine would not.
        let net = start.polyline(case.tolerance);
        let bound = TWO_APPROXIMATIONS + 2.0 * ROUNDING / (sharpest(&net) / 2.0).sin();
        assert!(
            distance <= bound,
            "{}: {distance} against {bound}",
            case.name
        );
        compared += 1;
    }
    assert!(
        compared >= 30 && refused <= 2,
        "compared {compared}, refused {refused}"
    );
}

#[test]
fn a_mirrored_piece_offsets_to_the_mirror_of_its_offset_where_the_corner_has_no_direction() {
    // `Step` is directional by its definition: the first edge runs on past the corner. A mirrored piece reverses which edge is first, so its
    // step lands on the other side. That is the shape, not an error, and why `mirror_of` mirrors the finished piece rather than drafting it again.
    let mut compared = 0;
    for case in cases() {
        if case
            .allowances
            .iter()
            .any(|item| item.corner == Corner::Step)
        {
            continue;
        }
        let start = outline(&case, |at| at);
        let Some(base) = offsets_of(&case, &start, &case.allowances) else {
            continue;
        };
        // A mirror runs the boundary backward, and each edge keeps its own allowance.
        let reversed = Outline::new(
            case.curves
                .iter()
                .rev()
                .map(|(kind, points)| {
                    let back: Vec<(i64, i64)> =
                        points.iter().rev().map(|at| (-at.0, at.1)).collect();
                    Curve::from_kind(kind, &back).expect("a curve")
                })
                .collect(),
        )
        .expect("a closed boundary");
        let allowances: Vec<Allowance> = case.allowances.iter().rev().copied().collect();
        let Some(after) = offsets_of(&case, &reversed, &allowances) else {
            panic!("{}: refused only when mirrored", case.name);
        };
        let expected = floats(&base, |x, y| (-x, y));
        let distance = apart(&expected, &floats(&after, |x, y| (x, y)));
        assert!(distance <= case.bound(), "{}: {distance}", case.name);
        compared += 1;
    }
    assert!(compared >= 20, "{compared}");
}

#[test]
fn the_transforms_catch_a_wrong_answer() {
    // If `apart` could not see a difference, everything above would pass on nothing.
    let square: Vec<(f64, f64)> = vec![(0.0, 0.0), (100.0, 0.0), (100.0, 100.0), (0.0, 100.0)];
    let bigger: Vec<(f64, f64)> = vec![(0.0, 0.0), (111.0, 0.0), (111.0, 100.0), (0.0, 100.0)];
    assert!(apart(&square, &square) <= f64::EPSILON);
    assert!(apart(&square, &bigger) > T2);
}
