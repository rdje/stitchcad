//! The kernel's promises, with an oracle that shares no arithmetic with it: arbitrary-precision integers for the
//! predicates, and dense floating-point sampling for the curves.
#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]
use num_bigint::BigInt;
use sc_geometry::{
    orientation, segment_crossing, Curve, CurveKindError, Orientation, Outline, OutlineError,
    Point, SegmentCrossing, Winding, DEFAULT_TOLERANCE, MAX_COORDINATE,
};

fn p(x: i64, y: i64) -> Point {
    Point::new(x, y).expect("in range")
}

fn big_orientation(a: Point, b: Point, c: Point) -> Orientation {
    let (ax, ay) = (BigInt::from(a.x()), BigInt::from(a.y()));
    let cross = (BigInt::from(b.x()) - &ax) * (BigInt::from(c.y()) - &ay)
        - (BigInt::from(b.y()) - &ay) * (BigInt::from(c.x()) - &ax);
    match cross.sign() {
        num_bigint::Sign::Plus => Orientation::Left,
        num_bigint::Sign::Minus => Orientation::Right,
        num_bigint::Sign::NoSign => Orientation::Collinear,
    }
}

fn naive_orientation(a: Point, b: Point, c: Point) -> Orientation {
    let (ax, ay) = (a.x() as f64, a.y() as f64);
    let cross =
        (b.x() as f64 - ax) * (c.y() as f64 - ay) - (b.y() as f64 - ay) * (c.x() as f64 - ax);
    if cross > 0.0 {
        Orientation::Left
    } else if cross < 0.0 {
        Orientation::Right
    } else {
        Orientation::Collinear
    }
}

#[test]
fn near_degenerate_cases_that_naive_floating_point_gets_wrong_are_right_here() {
    // Points a few micrometres off a long line, far from the origin, where f64 can no longer hold the differences' products.
    let base = 1i64 << 53;
    let cases = [
        (p(base, base), p(base + 2, base + 2), p(base + 1, base + 1)),
        (p(0, 0), p(base + 1, base), p(base, base - 1)),
        (p(0, 0), p(base + 1, base), p(base - 1, base - 1)),
        (p(1, 1), p(base, base + 1), p(base - 1, base)),
        (p(0, 0), p(base + 1, base + 3), p(base - 1, base + 1)),
    ];
    let mut naive_wrong = 0;
    for (a, b, c) in cases {
        // The arbitrary-precision oracle decides what is right.
        assert_eq!(
            orientation(a, b, c),
            big_orientation(a, b, c),
            "{a:?} {b:?} {c:?}"
        );
        if naive_orientation(a, b, c) != big_orientation(a, b, c) {
            naive_wrong += 1;
        }
    }
    assert!(
        naive_wrong > 0,
        "this test must include a case floating point gets wrong"
    );
}

fn on_box(a: Point, b: Point, c: Point) -> bool {
    a.x().min(b.x()) <= c.x()
        && c.x() <= a.x().max(b.x())
        && a.y().min(b.y()) <= c.y()
        && c.y() <= a.y().max(b.y())
}

/// How two segments meet, by the classic orientation test through arbitrary precision: a different method from the kernel's.
fn oracle_crossing(a: Point, b: Point, c: Point, d: Point) -> &'static str {
    let (o1, o2, o3, o4) = (
        big_orientation(a, b, c),
        big_orientation(a, b, d),
        big_orientation(c, d, a),
        big_orientation(c, d, b),
    );
    let collinear = Orientation::Collinear;
    if o1 == collinear && o2 == collinear && o3 == collinear && o4 == collinear {
        // All on one line. Count the distinct shared endpoints.
        let mut shared = Vec::new();
        for point in [a, b] {
            if on_box(c, d, point) && !shared.contains(&point) {
                shared.push(point);
            }
        }
        for point in [c, d] {
            if on_box(a, b, point) && !shared.contains(&point) {
                shared.push(point);
            }
        }
        return match shared.len() {
            0 => "apart",
            1 => "at",
            _ => "overlap",
        };
    }
    let crosses = o1 != o2 && o3 != o4;
    let touches = (o1 == collinear && on_box(a, b, c))
        || (o2 == collinear && on_box(a, b, d))
        || (o3 == collinear && on_box(c, d, a))
        || (o4 == collinear && on_box(c, d, b));
    if crosses || touches {
        "at"
    } else {
        "apart"
    }
}

fn kernel_crossing(a: Point, b: Point, c: Point, d: Point) -> &'static str {
    match segment_crossing(a, b, c, d) {
        SegmentCrossing::Apart => "apart",
        SegmentCrossing::At { .. } => "at",
        SegmentCrossing::Overlap => "overlap",
    }
}

#[test]
fn crossings_match_a_different_method_on_every_seed() {
    let mut random = Random(23);
    let mut seen = std::collections::BTreeMap::new();
    for _ in 0..200_000 {
        // A small grid, so touching, collinear and parallel cases all come up often, in every direction.
        let mut point = || {
            p(
                (random.next() % 7) as i64 - 3,
                (random.next() % 7) as i64 - 3,
            )
        };
        let (a, b, c, d) = (point(), point(), point(), point());
        let found = kernel_crossing(a, b, c, d);
        assert_eq!(
            found,
            oracle_crossing(a, b, c, d),
            "{a:?} {b:?} {c:?} {d:?}"
        );
        assert_eq!(
            found,
            kernel_crossing(c, d, a, b),
            "swapping the segments changed it: {a:?} {b:?} {c:?} {d:?}"
        );
        *seen.entry(found).or_insert(0usize) += 1;
    }
    assert!(
        seen.len() == 3 && seen.values().all(|count| *count > 500),
        "{seen:?}"
    );
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
    // Mixes huge coordinates, small ones and near-repeats, where a predicate is most likely to fail.
    fn coordinate(&mut self) -> i64 {
        match self.next() % 4 {
            0 => (self.next() % (2 * MAX_COORDINATE as u64)) as i64 - MAX_COORDINATE,
            1 => (self.next() % 2_000) as i64 - 1_000,
            2 => (1i64 << 53) + (self.next() % 8) as i64,
            _ => -(1i64 << 40) + (self.next() % 8) as i64,
        }
    }
    fn point(&mut self) -> Point {
        let (x, y) = (self.coordinate(), self.coordinate());
        p(x, y)
    }
}

#[test]
fn orientation_matches_arbitrary_precision_on_every_seed() {
    let mut random = Random(11);
    let mut seen = [0usize; 3];
    for _ in 0..50_000 {
        let (a, b, c) = (random.point(), random.point(), random.point());
        let found = orientation(a, b, c);
        assert_eq!(found, big_orientation(a, b, c), "{a:?} {b:?} {c:?}");
        seen[match found {
            Orientation::Left => 0,
            Orientation::Right => 1,
            Orientation::Collinear => 2,
        }] += 1;
    }
    assert!(seen.iter().all(|count| *count > 0), "{seen:?}");
}

#[test]
fn extreme_coordinates_do_not_overflow() {
    let m = MAX_COORDINATE;
    assert_eq!(orientation(p(-m, -m), p(m, m), p(-m, m)), Orientation::Left);
    assert_eq!(
        orientation(p(-m, -m), p(m, m), p(m, -m)),
        Orientation::Right
    );
    assert_eq!(
        orientation(p(-m, -m), p(m, m), p(0, 0)),
        Orientation::Collinear
    );
    assert!(matches!(
        segment_crossing(p(-m, -m), p(m, m), p(-m, m), p(m, -m)),
        SegmentCrossing::At { .. }
    ));
    assert!(Point::new(m + 1, 0).is_err());
    assert!(Point::new(0, -m - 1).is_err());
}

#[test]
fn segments_meet_as_they_should() {
    let crossing = |a, b, c, d| segment_crossing(p(a, 0), p(b, 10), p(c, 10), p(d, 0));
    let SegmentCrossing::At { first, second } = crossing(0, 10, 0, 10) else {
        panic!("an X")
    };
    assert_eq!((first.top(), first.bottom()), (1, 2));
    assert_eq!((second.top(), second.bottom()), (1, 2));
    assert_eq!(
        segment_crossing(p(0, 0), p(4, 0), p(0, 1), p(4, 1)),
        SegmentCrossing::Apart,
        "parallel"
    );
    assert_eq!(
        segment_crossing(p(0, 0), p(1, 1), p(5, 0), p(6, 1)),
        SegmentCrossing::Apart,
        "apart"
    );
    assert_eq!(
        segment_crossing(p(0, 0), p(1, 0), p(2, 0), p(3, 0)),
        SegmentCrossing::Apart,
        "collinear, apart"
    );
    assert_eq!(
        segment_crossing(p(0, 0), p(4, 0), p(2, 0), p(6, 0)),
        SegmentCrossing::Overlap,
        "collinear, overlapping"
    );
    assert_eq!(
        segment_crossing(p(0, 0), p(4, 0), p(0, 0), p(4, 0)),
        SegmentCrossing::Overlap,
        "the same segment"
    );
    let SegmentCrossing::At { first, second } =
        segment_crossing(p(0, 0), p(2, 0), p(2, 0), p(4, 0))
    else {
        panic!("collinear segments sharing one end");
    };
    assert_eq!((first.top(), first.bottom(), second.top()), (1, 1, 0));
    let SegmentCrossing::At { first, .. } = segment_crossing(p(0, 0), p(4, 0), p(2, 0), p(2, 3))
    else {
        panic!("a T junction");
    };
    assert_eq!((first.top(), first.bottom()), (1, 2));
    // A crossing whose position is not a whole micrometre stays exact.
    let SegmentCrossing::At { first, .. } = segment_crossing(p(0, 0), p(3, 0), p(1, -1), p(2, 2))
    else {
        panic!("a crossing");
    };
    assert_eq!((first.top(), first.bottom()), (4, 9));
}

fn distance_to_polyline(points: &[Point], x: f64, y: f64) -> f64 {
    let mut best = f64::MAX;
    for pair in points.windows(2) {
        let (ax, ay, bx, by) = (
            pair[0].x() as f64,
            pair[0].y() as f64,
            pair[1].x() as f64,
            pair[1].y() as f64,
        );
        let (dx, dy) = (bx - ax, by - ay);
        let length = dx * dx + dy * dy;
        let t = if length == 0.0 {
            0.0
        } else {
            (((x - ax) * dx + (y - ay) * dy) / length).clamp(0.0, 1.0)
        };
        best = best.min((x - (ax + t * dx)).hypot(y - (ay + t * dy)));
    }
    best
}

fn cubic_at(c: [(f64, f64); 4], t: f64) -> (f64, f64) {
    let u = 1.0 - t;
    let blend = |pick: fn(&(f64, f64)) -> f64| {
        u * u * u * pick(&c[0])
            + 3.0 * u * u * t * pick(&c[1])
            + 3.0 * u * t * t * pick(&c[2])
            + t * t * t * pick(&c[3])
    };
    (blend(|point| point.0), blend(|point| point.1))
}

#[test]
fn a_flattened_bezier_stays_within_its_tolerance_and_keeps_its_exact_ends() {
    let shapes = [
        [
            (0, 0),
            (100_000, 300_000),
            (200_000, -300_000),
            (300_000, 0),
        ], // an S
        [(0, 0), (0, 500_000), (500_000, 500_000), (500_000, 0)], // an arch
        [(0, 0), (400_000, 0), (-100_000, 0), (300_000, 0)], // straight, overshooting both ends
        [(0, 0), (300_000, 300_000), (0, 300_000), (300_000, 0)], // crossing its own handles
        [(0, 0), (900_000, 0), (900_000, 0), (300_000, 0)],  // straight, running far past its end
        [(0, 0), (-600_000, 0), (-600_000, 0), (300_000, 0)], // straight, running far back past its start
    ];
    for shape in shapes {
        let curve = Curve::from_kind("cubic", &shape).expect("a cubic");
        for tolerance in [10, DEFAULT_TOLERANCE, 1_000] {
            let points = curve.flatten(tolerance);
            assert_eq!(points.first().copied(), Some(curve.from()));
            assert_eq!(points.last().copied(), Some(curve.to()));
            let exact = shape.map(|(x, y)| (x as f64, y as f64));
            let mut worst = 0.0_f64;
            for step in 0..=2000 {
                let (x, y) = cubic_at(exact, f64::from(step) / 2000.0);
                worst = worst.max(distance_to_polyline(&points, x, y));
            }
            // One micrometre of grid rounding per level is the stated allowance.
            assert!(
                worst <= tolerance as f64 + 2.0,
                "{shape:?} at {tolerance}: strayed {worst}"
            );
        }
        assert_eq!(curve.reversed().reversed(), curve);
    }
}

#[test]
fn a_reversed_curve_follows_the_same_path() {
    // Lopsided on purpose: a Bézier whose handles were not swapped would trace a different shape.
    let curve = Curve::from_kind(
        "cubic",
        &[(0, 0), (500_000, 900_000), (600_000, 50_000), (700_000, 0)],
    )
    .expect("a cubic");
    let (forward, backward) = (curve.flatten(10), curve.reversed().flatten(10));
    assert_eq!(backward.first().copied(), Some(curve.to()));
    assert_eq!(backward.last().copied(), Some(curve.from()));
    for point in &backward {
        let strayed = distance_to_polyline(&forward, point.x() as f64, point.y() as f64);
        assert!(strayed <= 12.0, "{point:?} strayed {strayed}");
    }
    let arc =
        Curve::from_kind("arc", &[(100_000, 0), (70_711, 70_711), (0, 100_000)]).expect("an arc");
    let (forward, backward) = (arc.flatten(10), arc.reversed().flatten(10));
    for point in &backward {
        let strayed = distance_to_polyline(&forward, point.x() as f64, point.y() as f64);
        assert!(strayed <= 12.0, "{point:?} strayed {strayed}");
    }
}

#[test]
fn a_flattened_arc_stays_on_its_circle_and_measures_true() {
    // A quarter circle of 100 mm about the origin, through its exact 45-degree point to the micrometre.
    let radius = 100_000.0_f64;
    let through = (radius * core::f64::consts::FRAC_1_SQRT_2).round() as i64;
    let arc =
        Curve::from_kind("arc", &[(100_000, 0), (through, through), (0, 100_000)]).expect("an arc");
    let points = arc.flatten(DEFAULT_TOLERANCE);
    assert_eq!(points.first().copied(), Some(p(100_000, 0)));
    assert_eq!(points.last().copied(), Some(p(0, 100_000)));
    assert!(points.len() > 4);
    for step in 0..=1000 {
        let angle = core::f64::consts::FRAC_PI_2 * f64::from(step) / 1000.0;
        let strayed = distance_to_polyline(&points, radius * angle.cos(), radius * angle.sin());
        assert!(
            strayed <= DEFAULT_TOLERANCE as f64 + 2.0,
            "strayed {strayed}"
        );
    }
    let true_length = core::f64::consts::FRAC_PI_2 * radius;
    let measured = arc.length(10) as f64;
    assert!(
        (measured - true_length).abs() < 50.0,
        "measured {measured}, true {true_length}"
    );
    // The other way round, through the far side, the arc is three quarters of the circle.
    let long = Curve::from_kind("arc", &[(100_000, 0), (-through, -through), (0, 100_000)])
        .expect("an arc");
    assert!((long.length(10) as f64 - 3.0 * true_length).abs() < 150.0);
    // A straight or doubled-back arc is lines, not a crash.
    let flat = Curve::from_kind("arc", &[(0, 0), (5, 5), (10, 10)]).expect("an arc");
    assert_eq!(flat.flatten(10), vec![p(0, 0), p(5, 5), p(10, 10)]);
}

#[test]
fn only_the_declared_curves_exist() {
    for kind in ["nurbs", "spline", "circle"] {
        assert_eq!(
            Curve::from_kind(kind, &[(0, 0), (1, 1)]),
            Err(CurveKindError::Unsupported(kind.to_owned()))
        );
    }
    let error = Curve::from_kind("nurbs", &[]).expect_err("refused");
    assert_eq!(
        error.to_string(),
        "a nurbs curve is not supported: curves here are lines, circular arcs and cubic Béziers"
    );
    assert!(matches!(
        Curve::from_kind("line", &[(0, 0)]),
        Err(CurveKindError::WrongPointCount {
            kind: "line",
            needed: 2,
            given: 1
        })
    ));
    assert!(matches!(
        Curve::from_kind("arc", &[(0, 0)]),
        Err(CurveKindError::WrongPointCount { needed: 3, .. })
    ));
    assert!(matches!(
        Curve::from_kind("cubic", &[(0, 0)]),
        Err(CurveKindError::WrongPointCount { needed: 4, .. })
    ));
    assert!(matches!(
        Curve::from_kind("line", &[(i64::MAX, 0), (0, 0)]),
        Err(CurveKindError::Point(_))
    ));
}

fn line(a: (i64, i64), b: (i64, i64)) -> Curve {
    Curve::Line {
        from: p(a.0, a.1),
        to: p(b.0, b.1),
    }
}

fn square() -> Outline {
    Outline::new(vec![
        line((0, 0), (10, 0)),
        line((10, 0), (10, 10)),
        line((10, 10), (0, 10)),
        line((0, 10), (0, 0)),
    ])
    .expect("closed")
}

#[test]
fn a_boundary_must_close_exactly() {
    assert_eq!(Outline::new(vec![]), Err(OutlineError::Empty));
    let open = Outline::new(vec![line((0, 0), (10, 0)), line((10, 0), (10, 10))]);
    assert!(
        matches!(open, Err(OutlineError::Gap { at: 0, .. })),
        "{open:?}"
    );
    let gap = Outline::new(vec![line((0, 0), (10, 0)), line((10, 1), (0, 0))]);
    assert!(
        matches!(gap, Err(OutlineError::Gap { at: 1, .. })),
        "{gap:?}"
    );
    let error = gap.expect_err("a gap");
    assert_eq!(
        error.to_string(),
        "curve 1 starts at (10, 1), and the curve before it ended at (10, 0)"
    );
    let flat = Outline::new(vec![line((0, 0), (0, 0))]);
    assert_eq!(flat, Err(OutlineError::ZeroLength { at: 0 }));
}

#[test]
fn winding_and_area_are_exact() {
    let shape = square();
    assert_eq!(shape.doubled_area(1), 200);
    assert_eq!(shape.winding(1), Winding::Counterclockwise);
    assert_eq!(shape.reversed().winding(1), Winding::Clockwise);
    assert_eq!(shape.reversed().reversed(), shape);
    assert_eq!(shape.length(1), 40);
    let flat = Outline::new(vec![line((0, 0), (10, 0)), line((10, 0), (0, 0))]).expect("closed");
    assert_eq!(flat.winding(1), Winding::Degenerate);
}

#[test]
fn self_crossings_are_found_and_shared_corners_are_not() {
    assert!(square().self_crossings(1).is_empty(), "a clean square");
    let bowtie = Outline::new(vec![
        line((0, 0), (10, 10)),
        line((10, 10), (10, 0)),
        line((10, 0), (0, 10)),
        line((0, 10), (0, 0)),
    ])
    .expect("closed");
    let found = bowtie.self_crossings(1);
    assert_eq!(found.len(), 1);
    assert_eq!((found[0].first, found[0].second), (0, 2));
    // A spike that runs back along its own edge overlaps itself.
    let spike = Outline::new(vec![
        line((0, 0), (10, 0)),
        line((10, 0), (5, 0)),
        line((5, 0), (5, 5)),
        line((5, 5), (0, 0)),
    ])
    .expect("closed");
    assert!(!spike.self_crossings(1).is_empty(), "overlap");
    // A Bézier that loops on itself crosses within one curve.
    let looped = Curve::from_kind(
        "cubic",
        &[
            (0, 0),
            (300_000, 300_000),
            (-200_000, 300_000),
            (100_000, 0),
        ],
    )
    .expect("a cubic");
    let closing = line((100_000, 0), (0, 0));
    let shape = Outline::new(vec![looped, closing]).expect("closed");
    assert!(
        shape
            .self_crossings(100)
            .iter()
            .any(|crossing| crossing.first == 0 && crossing.second == 0),
        "{:?}",
        shape.self_crossings(100)
    );
    // A smooth closed curve is clean, including where its last curve meets its first.
    let lens = Outline::new(vec![
        Curve::from_kind("arc", &[(0, 0), (50_000, 20_000), (100_000, 0)]).expect("an arc"),
        Curve::from_kind("arc", &[(100_000, 0), (50_000, -20_000), (0, 0)]).expect("an arc"),
    ])
    .expect("closed");
    assert!(
        lens.self_crossings(100).is_empty(),
        "{:?}",
        lens.self_crossings(100)
    );
}

#[test]
fn the_reference_skirt_front_quadrant_is_a_clean_counterclockwise_boundary() {
    // From the reference skirt chapter §5, in micrometres, with y up and the hem below the waist.
    let (waist_side, hip, hem, hip_y, hem_y) = (225_000, 255_000, 285_000, -200_000, -620_000);
    let outline = Outline::new(vec![
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
    assert_eq!(
        outline.winding(DEFAULT_TOLERANCE),
        Winding::Counterclockwise
    );
    assert!(outline.self_crossings(DEFAULT_TOLERANCE).is_empty());
    // The side seam from hip to hem is the book's 42.107 cm.
    assert_eq!(line((hip, hip_y), (hem, hem_y)).length(1), 421_070);
}
