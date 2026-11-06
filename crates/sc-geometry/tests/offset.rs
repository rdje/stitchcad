//! The offset engine's promises, judged by checks that do not use its own code path: all-pairs crossing, winding numbers and
//! independent distance measures, on fixed shapes and on thousands of seeded ones.
#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]
mod common;

use common::p;
use sc_geometry::{Allowance, Corner, Curve, Offset, OffsetError, Outline, Point};

const TOLERANCE: i64 = 10;

fn line(a: (i64, i64), b: (i64, i64)) -> Curve {
    Curve::Line {
        from: p(a.0, a.1),
        to: p(b.0, b.1),
    }
}

fn polygon(points: &[(i64, i64)]) -> Outline {
    let curves = (0..points.len())
        .map(|at| line(points[at], points[(at + 1) % points.len()]))
        .collect();
    Outline::new(curves).expect("closed")
}

fn all(width: i64, corner: Corner, count: usize) -> Vec<Allowance> {
    vec![Allowance { width, corner }; count]
}

const SQUARE: [(i64, i64); 4] = [(0, 0), (100_000, 0), (100_000, 100_000), (0, 100_000)];

fn pts(offset: &Offset) -> Vec<(i64, i64)> {
    offset
        .points()
        .iter()
        .map(|point| (point.x(), point.y()))
        .collect()
}

fn assert_clean(points: &[Point], what: &str) {
    common::clean(points).unwrap_or_else(|reason| panic!("{what}: {reason}"));
}

fn doubled_area(points: &[Point]) -> i128 {
    common::doubled_area(points)
}

fn assert_valid(net: &Outline, offset: &Offset, minimum: i64, what: &str) {
    common::valid(net, offset.points(), minimum, TOLERANCE)
        .unwrap_or_else(|reason| panic!("{what}: {reason}"));
}

#[test]
fn a_mitered_square_grows_exactly() {
    let net = polygon(&SQUARE);
    let offset = net
        .offset(&all(10_000, Corner::Miter, 4), TOLERANCE)
        .expect("offsets");
    assert_eq!(
        pts(&offset),
        [
            (-10_000, -10_000),
            (110_000, -10_000),
            (110_000, 110_000),
            (-10_000, 110_000)
        ]
    );
    assert_eq!(offset.length(), 480_000);
    assert_valid(&net, &offset, 10_000, "square");
}

#[test]
fn an_edge_split_into_two_straight_pieces_leaves_no_extra_corner() {
    let net = polygon(&[
        (0, 0),
        (50_000, 0),
        (100_000, 0),
        (100_000, 100_000),
        (0, 100_000),
    ]);
    let offset = net
        .offset(&all(10_000, Corner::Miter, 5), TOLERANCE)
        .expect("offsets");
    assert_eq!(
        pts(&offset),
        [
            (-10_000, -10_000),
            (110_000, -10_000),
            (110_000, 110_000),
            (-10_000, 110_000)
        ]
    );
}

#[test]
fn each_corner_treatment_makes_its_own_shape_at_a_right_angle() {
    let net = polygon(&SQUARE);
    for (corner, expected_points) in [
        (Corner::Miter, 4),
        (Corner::Slant, 8),
        (Corner::Trim, 8),
        (Corner::Step, 8),
        (Corner::Envelope, 8),
    ] {
        let offset = net
            .offset(&all(10_000, corner, 4), TOLERANCE)
            .expect("offsets");
        assert_valid(&net, &offset, 10_000, &format!("{corner:?}"));
        if corner != Corner::Step && corner != Corner::Envelope {
            assert_eq!(
                offset.points().len(),
                expected_points,
                "{corner:?}: {:?}",
                pts(&offset)
            );
        }
    }
    // A slant cuts straight between the two ends.
    let slant = net
        .offset(&all(10_000, Corner::Slant, 4), TOLERANCE)
        .expect("offsets");
    assert!(
        pts(&slant).contains(&(100_000, -10_000)) && pts(&slant).contains(&(110_000, 0)),
        "{:?}",
        pts(&slant)
    );
    assert!(
        !pts(&slant).contains(&(110_000, -10_000)),
        "a slant has no tip: {:?}",
        pts(&slant)
    );
    // A trim keeps a flat tip as far from the corner as the allowance: 10 mm along the diagonal.
    let trim = net
        .offset(&all(10_000, Corner::Trim, 4), TOLERANCE)
        .expect("offsets");
    let farthest = pts(&trim)
        .iter()
        .map(|(x, y)| ((*x - 100_000) as f64).hypot((*y) as f64))
        .fold(f64::MAX, f64::min);
    assert!(farthest >= 10_000.0 - 2.0, "{farthest}");
    let tip = pts(&trim)
        .iter()
        .filter(|(x, y)| *x > 100_000 && *y < 0)
        .count();
    assert_eq!(tip, 2, "a flat tip has two points: {:?}", pts(&trim));
}

#[test]
fn the_more_specific_corner_treatment_wins_where_two_edges_differ() {
    let net = polygon(&SQUARE);
    let mut mixed = all(10_000, Corner::Miter, 4);
    mixed[1] = Allowance {
        width: 10_000,
        corner: Corner::Slant,
    };
    let offset = net.offset(&mixed, TOLERANCE).expect("offsets");
    // Edge 1 runs up the right side, so both its corners are slanted and the other two stay mitered.
    assert!(
        pts(&offset).contains(&(-10_000, -10_000)),
        "{:?}",
        pts(&offset)
    );
    assert!(
        !pts(&offset).contains(&(110_000, -10_000)),
        "{:?}",
        pts(&offset)
    );
    assert!(
        !pts(&offset).contains(&(110_000, 110_000)),
        "{:?}",
        pts(&offset)
    );
}

#[test]
fn an_edge_with_no_allowance_stays_on_the_net_line() {
    // A fold: the left side has no allowance.
    let net = polygon(&SQUARE);
    let mut widths = all(10_000, Corner::Miter, 4);
    widths[3] = Allowance {
        width: 0,
        corner: Corner::Miter,
    };
    let offset = net.offset(&widths, TOLERANCE).expect("offsets");
    assert!(
        offset.points().iter().all(|point| point.x() >= 0),
        "{:?}",
        pts(&offset)
    );
    assert!(
        pts(&offset).contains(&(0, -10_000)) && pts(&offset).contains(&(0, 110_000)),
        "{:?}",
        pts(&offset)
    );
}

#[test]
fn different_widths_meet_where_their_lines_cross() {
    let net = polygon(&SQUARE);
    let widths = [10_000, 30_000, 10_000, 30_000].map(|width| Allowance {
        width,
        corner: Corner::Miter,
    });
    let offset = net.offset(&widths, TOLERANCE).expect("offsets");
    assert_eq!(
        pts(&offset),
        [
            (-30_000, -10_000),
            (130_000, -10_000),
            (130_000, 110_000),
            (-30_000, 110_000)
        ]
    );
}

#[test]
fn a_concave_corner_meets_inside_with_no_loop() {
    let shape = [
        (0, 0),
        (200_000, 0),
        (200_000, 100_000),
        (100_000, 100_000),
        (100_000, 200_000),
        (0, 200_000),
    ];
    let net = polygon(&shape);
    for corner in [
        Corner::Miter,
        Corner::Slant,
        Corner::Trim,
        Corner::Step,
        Corner::Envelope,
    ] {
        let offset = net
            .offset(&all(10_000, corner, 6), TOLERANCE)
            .expect("offsets");
        assert_valid(&net, &offset, 10_000, &format!("L shape, {corner:?}"));
        assert!(
            pts(&offset).contains(&(110_000, 110_000)),
            "{corner:?}: the inner corner, {:?}",
            pts(&offset)
        );
    }
}

#[test]
fn acute_tips_stay_bounded_and_clean() {
    // Triangles with tips of 45, 30, 15 and 5 degrees. A miter past its limit is cut straight, so nothing spikes.
    for (depth, what) in [
        (100_000, "45"),
        (173_205, "30"),
        (373_205, "15"),
        (1_143_005, "5"),
    ] {
        let net = polygon(&[(0, 0), (depth, -50_000), (depth, 50_000)]);
        for corner in [
            Corner::Miter,
            Corner::Slant,
            Corner::Trim,
            Corner::Step,
            Corner::Envelope,
        ] {
            let offset = net
                .offset(&all(10_000, corner, 3), TOLERANCE)
                .expect("offsets");
            assert_valid(
                &net,
                &offset,
                10_000,
                &format!("{what} degrees, {corner:?}"),
            );
            let reach = offset
                .points()
                .iter()
                .map(|point| point.x())
                .min()
                .unwrap_or(0);
            assert!(
                reach >= -(sc_geometry::MITER_LIMIT * 10_000 + 3_000),
                "{what} degrees, {corner:?}: spike to {reach}"
            );
        }
    }
}

#[test]
fn a_circle_offset_measures_true_and_accumulates_little_error() {
    let radius = 100_000i64;
    let diagonal = ((radius as f64) * core::f64::consts::FRAC_1_SQRT_2).round() as i64;
    let arc = |a: (i64, i64), b: (i64, i64), c: (i64, i64)| {
        Curve::from_kind("arc", &[a, b, c]).expect("an arc")
    };
    let net = Outline::new(vec![
        arc((radius, 0), (diagonal, diagonal), (0, radius)),
        arc((0, radius), (-diagonal, diagonal), (-radius, 0)),
        arc((-radius, 0), (-diagonal, -diagonal), (0, -radius)),
        arc((0, -radius), (diagonal, -diagonal), (radius, 0)),
    ])
    .expect("closed");
    let width = 20_000;
    let offset = net
        .offset(&all(width, Corner::Miter, 4), TOLERANCE)
        .expect("offsets");
    assert_valid(&net, &offset, width, "circle");
    let true_length = core::f64::consts::TAU * (radius + width) as f64;
    let error = (offset.length() as f64 - true_length).abs();
    assert!(
        error <= 200.0,
        "length {} against {true_length}: {error} off",
        offset.length()
    );
    for point in offset.points() {
        let from_centre = (point.x() as f64).hypot(point.y() as f64);
        assert!(
            (from_centre - (radius + width) as f64).abs() <= (TOLERANCE + 3) as f64,
            "{point:?} is {from_centre}"
        );
    }
}

#[test]
fn a_curve_tighter_than_its_allowance_folds_and_the_loop_is_removed() {
    // A concave scoop of about 10 mm radius in a straight edge, with a 20 mm allowance: the raw offset folds over itself.
    let scoop =
        Curve::from_kind("arc", &[(40_000, 0), (50_000, 10_000), (60_000, 0)]).expect("an arc");
    let net = Outline::new(vec![
        line((0, 0), (40_000, 0)),
        scoop,
        line((60_000, 0), (100_000, 0)),
        line((100_000, 0), (100_000, 100_000)),
        line((100_000, 100_000), (0, 100_000)),
        line((0, 100_000), (0, 0)),
    ])
    .expect("closed");
    let offset = net
        .offset(&all(20_000, Corner::Miter, 6), TOLERANCE)
        .expect("offsets");
    assert_valid(&net, &offset, 20_000, "scoop");
}

#[test]
fn a_cusp_and_a_near_tangent_join_stay_clean() {
    // Two Béziers meeting at a sharp point on the outside.
    let net = Outline::new(vec![
        Curve::from_kind(
            "cubic",
            &[
                (0, 0),
                (30_000, 2_000),
                (70_000, 20_000),
                (100_000, 100_000),
            ],
        )
        .expect("a cubic"),
        Curve::from_kind(
            "cubic",
            &[
                (100_000, 100_000),
                (70_000, 40_000),
                (30_000, 60_000),
                (0, 100_000),
            ],
        )
        .expect("a cubic"),
        line((0, 100_000), (0, 0)),
    ])
    .expect("closed");
    let offset = net
        .offset(&all(5_000, Corner::Slant, 3), TOLERANCE)
        .expect("offsets");
    assert_valid(&net, &offset, 5_000, "cusp");

    // A curve leaving a line almost along it, with only a sliver of angle.
    let net = Outline::new(vec![
        line((0, 0), (100_000, 0)),
        Curve::from_kind(
            "cubic",
            &[
                (100_000, 0),
                (150_000, 5),
                (200_000, 40_000),
                (200_000, 100_000),
            ],
        )
        .expect("a cubic"),
        line((200_000, 100_000), (0, 100_000)),
        line((0, 100_000), (0, 0)),
    ])
    .expect("closed");
    let offset = net
        .offset(&all(10_000, Corner::Miter, 4), TOLERANCE)
        .expect("offsets");
    assert_valid(&net, &offset, 10_000, "near tangent");
}

#[test]
fn a_slot_narrower_than_two_allowances_is_filled_and_reports_the_edge_it_covered() {
    let net = polygon(&[
        (0, 0),
        (40_000, 0),
        (40_000, 80_000),
        (45_000, 80_000),
        (45_000, 0),
        (100_000, 0),
        (100_000, 100_000),
        (0, 100_000),
    ]);
    // The true offset fills the slot, which has the cloth it needs, and the result says the slot's own edges were swallowed.
    let offset = net
        .offset(&all(10_000, Corner::Miter, 8), TOLERANCE)
        .expect("offsets");
    assert_clean(offset.points(), "slot");
    assert!(
        offset.swallowed_edges().contains(&2),
        "{:?}",
        offset.swallowed_edges()
    );
    // A narrow enough allowance fits.
    let offset = net
        .offset(&all(2_000, Corner::Miter, 8), TOLERANCE)
        .expect("it fits");
    assert_valid(&net, &offset, 2_000, "slot");
}

#[test]
fn a_refused_allowance_names_what_is_wrong() {
    let net = polygon(&SQUARE);
    assert_eq!(
        net.offset(&all(1, Corner::Miter, 3), TOLERANCE),
        Err(OffsetError::WrongCount {
            curves: 4,
            allowances: 3
        })
    );
    let mut widths = all(1, Corner::Miter, 4);
    widths[2].width = -1;
    assert_eq!(
        net.offset(&widths, TOLERANCE),
        Err(OffsetError::NegativeWidth { edge: 2 })
    );
    assert_eq!(
        net.reversed().offset(&all(1, Corner::Miter, 4), TOLERANCE),
        Err(OffsetError::NotCounterclockwise)
    );
    let bowtie = polygon(&[(0, 0), (10, 10), (10, 0), (0, 10)]);
    assert!(net.offset(&all(1, Corner::Miter, 4), TOLERANCE).is_ok());
    let error = polygon(&[(0, 0), (100, 100), (100, 0), (50, 200)])
        .offset(&all(1, Corner::Miter, 4), TOLERANCE);
    assert!(
        matches!(
            error,
            Err(OffsetError::NetCrossing { .. }) | Err(OffsetError::NotCounterclockwise)
        ),
        "{error:?}"
    );
    let error = bowtie
        .offset(&all(1, Corner::Miter, 4), TOLERANCE)
        .expect_err("a figure of eight");
    assert!(!error.to_string().is_empty());
    assert_eq!(
        OffsetError::BoundExceeded {
            edge: 3,
            short_by: 40
        }
        .to_string(),
        "edge 3's allowance falls 40 micrometres short of its width, past the tolerance"
    );
}

#[test]
fn the_reference_skirt_front_takes_its_allowances() {
    // Reference skirt §7 on the front quadrant: fold at CF, a 3 cm enveloped hem, 1 cm mitered side seam and 1 cm trimmed waist.
    let (waist_side, hip, hem, hip_y, hem_y) = (225_000, 255_000, 285_000, -200_000, -620_000);
    let net = Outline::new(vec![
        line((0, 0), (0, hem_y)),
        line((0, hem_y), (hem, hem_y)),
        line((hem, hem_y), (hip, hip_y)),
        Curve::from_kind(
            "cubic",
            &[
                (hip, hip_y),
                (hip, -100_000),
                (waist_side + 10_000, -30_000),
                (waist_side, 0),
            ],
        )
        .expect("the hip curve"),
        line((waist_side, 0), (0, 0)),
    ])
    .expect("closed");
    let widths = [
        Allowance {
            width: 0,
            corner: Corner::Miter,
        },
        Allowance {
            width: 30_000,
            corner: Corner::Envelope,
        },
        Allowance {
            width: 10_000,
            corner: Corner::Miter,
        },
        Allowance {
            width: 10_000,
            corner: Corner::Miter,
        },
        Allowance {
            width: 10_000,
            corner: Corner::Trim,
        },
    ];
    let offset = net.offset(&widths, 10).expect("offsets");
    assert_clean(offset.points(), "skirt");
    assert!(doubled_area(offset.points()) > 0);
    let lowest = offset
        .points()
        .iter()
        .map(|point| point.y())
        .min()
        .unwrap_or(0);
    assert_eq!(lowest, hem_y - 30_000, "the hem's 3 cm: {:?}", pts(&offset));
    assert!(
        offset.points().iter().all(|point| point.x() >= 0),
        "the fold stays on CF: {:?}",
        pts(&offset)
    );
    let highest = offset
        .points()
        .iter()
        .map(|point| point.y())
        .max()
        .unwrap_or(0);
    assert_eq!(highest, 10_000, "the waist's 1 cm: {:?}", pts(&offset));
}

struct Random(u64);
impl Random {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, bound: u64) -> u64 {
        self.next() % bound
    }
}

/// A star-shaped polygon around the origin: always simple and counterclockwise, with acute and concave corners of every size.
fn star(random: &mut Random) -> Vec<(i64, i64)> {
    let count = 3 + random.below(14) as usize;
    (0..count)
        .map(|at| {
            let angle = core::f64::consts::TAU
                * (at as f64 + 0.15 * (random.below(100) as f64 / 100.0))
                / count as f64;
            let radius = 40_000.0 + random.below(160_000) as f64;
            (
                (radius * angle.cos()).round() as i64,
                (radius * angle.sin()).round() as i64,
            )
        })
        .collect()
}

#[test]
fn every_seeded_shape_gives_a_clean_allowance_around_it_or_a_named_refusal() {
    let mut random = Random(5);
    let (mut clean, mut refused) = (0, 0);
    let corners = [
        Corner::Miter,
        Corner::Slant,
        Corner::Trim,
        Corner::Step,
        Corner::Envelope,
    ];
    for seed in 0..4_000 {
        let shape = star(&mut random);
        let net = polygon(&shape);
        let allowances: Vec<Allowance> = (0..shape.len())
            .map(|_| Allowance {
                width: 2_000 + random.below(20_000) as i64,
                corner: corners[random.below(5) as usize],
            })
            .collect();
        let minimum = allowances
            .iter()
            .map(|allowance| allowance.width)
            .min()
            .unwrap_or(0);
        match net.offset(&allowances, TOLERANCE) {
            Ok(offset) => {
                assert_valid(&net, &offset, minimum, &format!("seed {seed}"));
                clean += 1;
            }
            // A refusal is allowed, and it must be one that names an edge. A panic or a wrong result is not.
            Err(OffsetError::BoundExceeded { .. }) => refused += 1,
            Err(other) => panic!("seed {seed}: {other}"),
        }
    }
    assert!(clean > 3_000, "{clean} clean, {refused} refused");
    println!("{clean} clean, {refused} refused");
}
