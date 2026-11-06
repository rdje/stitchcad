//! Judges an offset by its result alone, sharing no code path with the engine: all-pairs crossing, winding numbers and independent
//! distance measures.
#![allow(
    dead_code,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]
use sc_geometry::{segment_crossing, Outline, Point, SegmentCrossing};

pub fn p(x: i64, y: i64) -> Point {
    Point::new(x, y).expect("in range")
}

/// No two segments of the closed polyline touch, apart from neighbours at their shared point.
pub fn clean(points: &[Point]) -> Result<(), String> {
    let count = points.len();
    if count < 3 {
        return Err(format!("{count} points is not a boundary"));
    }
    for i in 0..count {
        for j in (i + 1)..count {
            let (a, b) = (points[i], points[(i + 1) % count]);
            let (c, d) = (points[j], points[(j + 1) % count]);
            let neighbours = j == i + 1 || (i == 0 && j + 1 == count);
            match segment_crossing(a, b, c, d) {
                SegmentCrossing::Apart => {}
                SegmentCrossing::Overlap => return Err(format!("segments {i} and {j} overlap")),
                SegmentCrossing::At { .. } => {
                    if !neighbours {
                        return Err(format!(
                            "segments {i} and {j} cross: {a:?} {b:?} {c:?} {d:?}"
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}

/// Twice the area, positive counterclockwise, by the shoelace formula.
pub fn doubled_area(points: &[Point]) -> i128 {
    let count = points.len();
    (0..count)
        .map(|at| {
            let (a, b) = (points[at], points[(at + 1) % count]);
            i128::from(a.x()) * i128::from(b.y()) - i128::from(b.x()) * i128::from(a.y())
        })
        .sum()
}

/// The winding number of a closed polyline around a point: nonzero means the point is inside.
pub fn winding_number(points: &[Point], at: Point) -> i32 {
    let count = points.len();
    let mut winding = 0;
    for index in 0..count {
        let (a, b) = (points[index], points[(index + 1) % count]);
        let cross = (i128::from(b.x()) - i128::from(a.x()))
            * (i128::from(at.y()) - i128::from(a.y()))
            - (i128::from(at.x()) - i128::from(a.x())) * (i128::from(b.y()) - i128::from(a.y()));
        if a.y() <= at.y() {
            if b.y() > at.y() && cross > 0 {
                winding += 1;
            }
        } else if b.y() <= at.y() && cross < 0 {
            winding -= 1;
        }
    }
    winding
}

/// The nearest place on the net boundary: how far, which curve, and whether it is a corner where two curves meet.
pub fn nearest_on_net(net: &Outline, at: Point, tolerance: i64) -> (f64, usize, bool) {
    let mut best = (f64::MAX, 0, false);
    for (curve_at, curve) in net.curves().iter().enumerate() {
        let flat = curve.flatten(tolerance);
        let last = flat.len() - 2;
        for (index, pair) in flat.windows(2).enumerate() {
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
                (((at.x() as f64 - ax) * dx + (at.y() as f64 - ay) * dy) / length).clamp(0.0, 1.0)
            };
            let here = (at.x() as f64 - (ax + t * dx)).hypot(at.y() as f64 - (ay + t * dy));
            if here < best.0 {
                best = (
                    here,
                    curve_at,
                    (index == 0 && t <= 0.0) || (index == last && t >= 1.0),
                );
            }
        }
    }
    best
}

/// What an allowance must be, whatever the shape: clean, counterclockwise, around the net piece, and clear of each edge by that edge's
/// own width. A fold's width is 0. A corner join may come closer to the corner itself, so only places nearest the inside of an edge count.
pub fn valid_widths(
    net: &Outline,
    points: &[Point],
    widths: &[i64],
    tolerance: i64,
) -> Result<(), String> {
    clean(points)?;
    if doubled_area(points) <= 0 {
        return Err("it does not run counterclockwise".to_owned());
    }
    for point in net.polyline(tolerance) {
        if winding_number(points, point) == 0 && !net_point_on_boundary(points, point) {
            return Err(format!("net point {point:?} is outside the allowance"));
        }
    }
    let count = points.len();
    for index in 0..count {
        let (a, b) = (points[index], points[(index + 1) % count]);
        for step in 0..8 {
            let at = p(
                a.x() + (b.x() - a.x()) * step / 8,
                a.y() + (b.y() - a.y()) * step / 8,
            );
            let (clearance, edge, at_corner) = nearest_on_net(net, at, tolerance);
            let wanted = widths[edge];
            if !at_corner && clearance < wanted as f64 - (tolerance + 3) as f64 {
                return Err(format!(
                    "{at:?} is only {clearance} from edge {edge}, wanting {wanted}"
                ));
            }
        }
    }
    Ok(())
}

// A fold's net edge is the allowance's own boundary, so its points are on it rather than strictly inside.
fn net_point_on_boundary(points: &[Point], at: Point) -> bool {
    let count = points.len();
    (0..count).any(|index| {
        matches!(
            segment_crossing(points[index], points[(index + 1) % count], at, at),
            SegmentCrossing::At { .. } | SegmentCrossing::Overlap
        )
    })
}

/// The same with one width for every edge.
pub fn valid(net: &Outline, points: &[Point], minimum: i64, tolerance: i64) -> Result<(), String> {
    valid_widths(net, points, &vec![minimum; net.curves().len()], tolerance)
}
