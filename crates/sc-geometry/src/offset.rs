//! The offset engine: from a piece's net boundary to the boundary with its seam allowances.
//!
//! An exact offset of a Bézier is not a Bézier, so this offsets a polyline that stays within a stated tolerance of the
//! curve. The raw offset can fold back on itself at a concave corner or a tight curve. Those loops are removed by tracing the
//! outer boundary, and the result is checked against the net boundary before it is returned: an allowance that cannot be
//! produced within the bound is refused, naming the edge, rather than exported wrong.
use crate::curve::{distance, Curve};
use crate::outline::{Outline, Winding};
use crate::predicate::{segment_crossing, Point, SegmentCrossing};
use core::fmt;
use std::collections::BTreeMap;

/// How the allowance turns a corner between two edges. Where two edges name different treatments, the corner takes the more
/// specific one: envelope, then step, then trim, then slant, then miter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Corner {
    /// The two offset edges run on until they meet, within a limit of [`MITER_LIMIT`] allowance widths.
    Miter,
    /// A straight cut between the two offset edges' ends.
    Slant,
    /// A miter with its tip cut off square, as far from the net corner as the wider allowance, to reduce bulk.
    Trim,
    /// The first offset edge runs on by the second edge's width, then turns square until it meets the second offset edge.
    Step,
    /// The edge naming this is a fold line, and its neighbour's offset is mirrored across it, the way a hem folds up over a side seam.
    Envelope,
}

/// How far a miter may reach from its net corner, in widths of the wider allowance, before it is cut straight.
pub const MITER_LIMIT: i64 = 4;

/// One edge's allowance: a curve's width and how it treats its corners. A width of 0 is an edge with no allowance, such as a fold.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Allowance {
    /// Micrometres outward from the net edge.
    pub width: i64,
    /// How it treats its corners.
    pub corner: Corner,
}

/// Why an allowance could not be produced. Nothing is returned in that case, so nothing wrong can be exported.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffsetError {
    /// A different number of allowances than curves.
    WrongCount {
        /// How many curves the boundary has.
        curves: usize,
        /// How many allowances were given.
        allowances: usize,
    },
    /// A width cannot be negative: an allowance adds cloth.
    NegativeWidth {
        /// The edge, by position.
        edge: usize,
    },
    /// The net boundary must run counterclockwise, so outward is known.
    NotCounterclockwise,
    /// The net boundary already crosses itself between these edges, so no allowance can be right.
    NetCrossing {
        /// The earlier edge.
        first: usize,
        /// The later edge.
        second: usize,
    },
    /// A point of the result is closer to the net boundary than the allowance allows, beyond the tolerance.
    BoundExceeded {
        /// The edge, by position, whose allowance is too narrow there.
        edge: usize,
        /// How far short it falls, in micrometres.
        short_by: i64,
    },
    /// The result still crossed itself after its loops were removed, so nothing was returned.
    Tangled,
}

impl fmt::Display for OffsetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongCount { curves, allowances } => {
                write!(
                    f,
                    "this boundary has {curves} edges and {allowances} allowances were given"
                )
            }
            Self::NegativeWidth { edge } => write!(f, "edge {edge} has a negative allowance"),
            Self::NotCounterclockwise => {
                f.write_str("an allowance needs a boundary running counterclockwise")
            }
            Self::NetCrossing { first, second } => write!(
                f,
                "the net boundary crosses itself between edges {first} and {second}"
            ),
            Self::BoundExceeded { edge, short_by } => {
                write!(f, "edge {edge}'s allowance falls {short_by} micrometres short of its width, past the tolerance")
            }
            Self::Tangled => {
                f.write_str("the allowance still crossed itself after its loops were removed")
            }
        }
    }
}

impl std::error::Error for OffsetError {}

/// A piece's boundary with its allowances: a clean closed polyline, with no self-crossings, running counterclockwise.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Offset {
    points: Vec<Point>,
    swallowed: Vec<usize>,
}

impl Offset {
    /// The closed polyline, leaving out the repeated closing point.
    #[must_use]
    pub fn points(&self) -> &[Point] {
        &self.points
    }

    /// Edges with an allowance whose own line vanished, because a neighbour's allowance covers it: a slot narrower than two
    /// allowances, or a concave curve tighter than one. The true offset does this too, and it has the cloth it needs, but the cut
    /// boundary no longer follows that edge, so a caller may want to refuse it.
    #[must_use]
    pub fn swallowed_edges(&self) -> &[usize] {
        &self.swallowed
    }

    /// Its length, rounded to a micrometre per chord.
    #[must_use]
    pub fn length(&self) -> i64 {
        let mut total = 0i64;
        let mut previous = self.points.last().copied();
        for point in &self.points {
            if let Some(before) = previous {
                total = total.saturating_add(distance(before, *point));
            }
            previous = Some(*point);
        }
        total
    }
}

#[allow(clippy::cast_precision_loss)]
fn vector(a: Point, b: Point) -> (f64, f64) {
    ((b.x() - a.x()) as f64, (b.y() - a.y()) as f64)
}

fn unit(v: (f64, f64)) -> (f64, f64) {
    let length = v.0.hypot(v.1);
    if length == 0.0 {
        (0.0, 0.0)
    } else {
        (v.0 / length, v.1 / length)
    }
}

// Outward for a counterclockwise boundary is to the right of the direction of travel.
const fn outward(direction: (f64, f64)) -> (f64, f64) {
    (direction.1, -direction.0)
}

#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn moved(point: Point, by: (f64, f64)) -> Point {
    Point::clamped(
        ((point.x() as f64) + by.0).round() as i64,
        ((point.y() as f64) + by.1).round() as i64,
    )
}

// Where two infinite lines meet: `a` along `u`, and `b` along `v`.
#[allow(clippy::cast_precision_loss)]
fn line_meet(a: Point, u: (f64, f64), b: Point, v: (f64, f64)) -> Option<(f64, f64)> {
    let denominator = u.0 * v.1 - u.1 * v.0;
    if denominator.abs() < 1e-12 {
        return None;
    }
    let (dx, dy) = (
        (b.x() as f64) - (a.x() as f64),
        (b.y() as f64) - (a.y() as f64),
    );
    let t = (dx * v.1 - dy * v.0) / denominator;
    Some(((a.x() as f64) + u.0 * t, (a.y() as f64) + u.1 * t))
}

#[allow(clippy::cast_possible_truncation)]
fn rounded(point: (f64, f64)) -> Point {
    Point::clamped(point.0.round() as i64, point.1.round() as i64)
}

/// One edge, as its polyline and its offset polyline.
struct Edge {
    width: i64,
    corner: Corner,
    net: Vec<Point>,
    start_direction: (f64, f64),
    end_direction: (f64, f64),
    offset: Vec<Point>,
}

// The offset of one curve's polyline, joined along its smooth run. A small turn meets at the lines' crossing, which is close and
// well conditioned. A sharp convex turn inside one curve is a cusp, and the true offset goes around it on an arc of the allowance's
// radius, so this does too, within the tolerance. A sharp concave turn keeps both ends, and loop removal deals with the crossing.
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn offset_polyline(net: &[Point], width: i64, tolerance: i64) -> Vec<Point> {
    let shift = width as f64;
    let mut out = Vec::with_capacity(net.len());
    let segments: Vec<(Point, (f64, f64))> = net
        .windows(2)
        .filter_map(|pair| match pair {
            [a, b] if a != b => Some((*a, unit(vector(*a, *b)))),
            _ => None,
        })
        .collect();
    let (Some(first), Some(last)) = (segments.first(), segments.last()) else {
        return out;
    };
    let normal = outward(first.1);
    out.push(moved(first.0, (normal.0 * shift, normal.1 * shift)));
    for pair in segments.windows(2) {
        let [(_, before), (joint, after)] = pair else {
            continue;
        };
        let (n0, n1) = (outward(*before), outward(*after));
        let (a, b) = (
            moved(*joint, (n0.0 * shift, n0.1 * shift)),
            moved(*joint, (n1.0 * shift, n1.1 * shift)),
        );
        let cosine = before.0 * after.0 + before.1 * after.1;
        // Under about 60 degrees of turn, the lines' crossing is close and well conditioned.
        match line_meet(a, *before, b, *after) {
            Some(point) if cosine > 0.5 => out.push(rounded(point)),
            _ => {
                out.push(a);
                let turn = before.0 * after.1 - before.1 * after.0;
                if turn > 0.0 && width > 0 {
                    // The chord over this arc stays within the tolerance of a circle of the allowance's radius.
                    let radius = width as f64;
                    let bound = ((tolerance.max(1)) as f64 / radius).min(1.0);
                    let step = (2.0 * (1.0 - bound).acos()).max(1e-3);
                    let sweep = turn.atan2(cosine);
                    let pieces = (sweep / step).ceil().clamp(1.0, 256.0) as u32;
                    let start = n0.1.atan2(n0.0);
                    for piece in 1..pieces {
                        let angle = start + sweep * f64::from(piece) / f64::from(pieces);
                        out.push(moved(*joint, (radius * angle.cos(), radius * angle.sin())));
                    }
                }
                out.push(b);
            }
        }
    }
    let normal = outward(last.1);
    if let Some(end) = net.last() {
        out.push(moved(*end, (normal.0 * shift, normal.1 * shift)));
    }
    out
}

// How one corner joins: the points to add, and whether they replace the first edge's end and the second edge's start. A join
// that replaces an end avoids leaving the path to double back over itself.
struct Joined {
    points: Vec<Point>,
    replaces_end: bool,
    replaces_start: bool,
}

const fn keep_both(points: Vec<Point>) -> Joined {
    Joined {
        points,
        replaces_end: false,
        replaces_start: false,
    }
}

const fn replace_both(points: Vec<Point>) -> Joined {
    Joined {
        points,
        replaces_end: true,
        replaces_start: true,
    }
}

// The points that take the allowance from one edge's offset end to the next edge's offset start, around the net corner.
#[allow(clippy::cast_precision_loss)]
fn join(a: &Edge, b: &Edge, corner: Point) -> Joined {
    let (Some(a_end), Some(b_start)) = (a.offset.last().copied(), b.offset.first().copied()) else {
        return keep_both(Vec::new());
    };
    if a.width == 0 && b.width == 0 {
        return keep_both(Vec::new());
    }
    let (u, v) = (a.end_direction, b.start_direction);
    let turn = u.0 * v.1 - u.1 * v.0;
    let meet = line_meet(a_end, u, b_start, v);
    // Turning left on a counterclockwise boundary is a convex corner of the piece, where the allowance has to go around the
    // outside. Turning right is a concave corner, where the offsets cross inside and every treatment means the same thing.
    if turn <= 0.0 {
        return meet.map_or_else(
            || keep_both(Vec::new()),
            |point| replace_both(vec![rounded(point)]),
        );
    }
    let wider = a.width.max(b.width) as f64;
    let slant = || keep_both(Vec::new());
    let joined = match a.corner.max(b.corner) {
        Corner::Slant => slant(),
        Corner::Miter => meet.map_or_else(slant, |point| replace_both(vec![rounded(point)])),
        Corner::Trim => {
            let Some(point) = meet else { return slant() };
            let (dx, dy) = (point.0 - corner.x() as f64, point.1 - corner.y() as f64);
            let reach = dx.hypot(dy);
            if reach <= wider {
                return replace_both(vec![rounded(point)]);
            }
            // A cut across the tip, square to the line from the net corner, as far out as the wider allowance.
            let axis = unit((dx, dy));
            let cut_point = rounded((
                corner.x() as f64 + axis.0 * wider,
                corner.y() as f64 + axis.1 * wider,
            ));
            let along = (-axis.1, axis.0);
            match (
                line_meet(a_end, u, cut_point, along),
                line_meet(b_start, v, cut_point, along),
            ) {
                (Some(first), Some(second)) => replace_both(vec![rounded(first), rounded(second)]),
                _ => slant(),
            }
        }
        Corner::Step => {
            let reach = b.width as f64;
            let out = moved(a_end, (u.0 * reach, u.1 * reach));
            match line_meet(out, outward(u), b_start, v) {
                Some(point) => Joined {
                    points: vec![out, rounded(point)],
                    replaces_end: false,
                    replaces_start: true,
                },
                None => slant(),
            }
        }
        // The edge naming it is the fold. Two meeting at a corner have no fold between them, as at a mitred hem corner, so they miter,
        // and either way the join does not depend on which way the boundary happens to run.
        Corner::Envelope => match (a.corner == Corner::Envelope, b.corner == Corner::Envelope) {
            (true, true) => meet.map_or_else(slant, |point| replace_both(vec![rounded(point)])),
            (false, _) => fold_over_second(a_end, b_start, corner, u, v),
            (true, false) => fold_over_first(a_end, b_start, corner, u, v),
        },
    };
    // No treatment may reach further than a miter's limit: past it, an acute corner gets a straight cut.
    let limit = wider * MITER_LIMIT as f64;
    let reach_of = |point: &Point| {
        (point.x() as f64 - corner.x() as f64).hypot(point.y() as f64 - corner.y() as f64)
    };
    if joined.points.iter().any(|point| reach_of(point) > limit) {
        return slant();
    }
    joined
}

// The direction `u` mirrored across a line running along `v`.
fn mirrored(u: (f64, f64), v: (f64, f64)) -> (f64, f64) {
    let along = u.0 * v.0 + u.1 * v.1;
    (2.0 * along * v.0 - u.0, 2.0 * along * v.1 - u.1)
}

// The first edge's offset runs to the second edge's net line, then mirrors across it to reach the second edge's offset.
fn fold_over_second(
    a_end: Point,
    b_start: Point,
    corner: Point,
    u: (f64, f64),
    v: (f64, f64),
) -> Joined {
    let Some(on_fold) = line_meet(a_end, u, corner, v) else {
        return keep_both(Vec::new());
    };
    let fold = rounded(on_fold);
    match line_meet(fold, mirrored(u, v), b_start, v) {
        Some(point) => Joined {
            points: vec![fold, rounded(point)],
            replaces_end: true,
            replaces_start: true,
        },
        None => keep_both(Vec::new()),
    }
}

// The second edge's offset, traced back to the first edge's net line, mirrors across it to reach the first edge's offset.
fn fold_over_first(
    a_end: Point,
    b_start: Point,
    corner: Point,
    u: (f64, f64),
    v: (f64, f64),
) -> Joined {
    let back = (-v.0, -v.1);
    let Some(on_fold) = line_meet(b_start, back, corner, u) else {
        return keep_both(Vec::new());
    };
    let fold = rounded(on_fold);
    match line_meet(fold, mirrored(back, u), a_end, u) {
        Some(point) => Joined {
            points: vec![rounded(point), fold],
            replaces_end: true,
            replaces_start: true,
        },
        None => keep_both(Vec::new()),
    }
}

// How far clockwise a move from `from` through `at` to `to` turns: largest is the rightmost, which on a counterclockwise walk
// is the outside. Going straight back is the smallest, so it is the last resort.
#[allow(clippy::cast_precision_loss)]
fn clockwise_turn(from: Point, at: Point, to: Point) -> f64 {
    let (a, b) = (vector(from, at), vector(at, to));
    -(a.0 * b.1 - a.1 * b.0).atan2(a.0 * b.0 + a.1 * b.1)
}

// Exact duplicates, points on a straight run, zero-area spikes and pinched-off loops all go. Repeats until nothing changes.
fn tidied(points: Vec<Point>) -> Vec<Point> {
    let mut current = points;
    loop {
        let mut next: Vec<Point> = Vec::with_capacity(current.len());
        let last = current.len().saturating_sub(1);
        for (index, point) in current.iter().enumerate() {
            match next.iter().position(|seen| seen == point) {
                // The last point coming back to the first is the boundary closing, not a loop.
                Some(0) if index == last => {}
                // Back at a point already on the path: everything since is a loop hanging off it.
                Some(at) => next.truncate(at + 1),
                None => next.push(*point),
            }
        }
        let count = next.len();
        let mut kept = Vec::with_capacity(count);
        for at in 0..count {
            let (Some(before), Some(here), Some(after)) = (
                next.get((at + count - 1) % count),
                next.get(at),
                next.get((at + 1) % count),
            ) else {
                continue;
            };
            // Keep a point unless it lies exactly on the straight line through its neighbours.
            let straight = crate::predicate::cross(*before, *after, *here) == 0;
            if !(straight && count > 3) {
                kept.push(*here);
            }
        }
        if kept.len() == current.len() {
            return kept;
        }
        current = kept;
    }
}

fn crossings_of(points: &[Point]) -> bool {
    let count = points.len();
    for i in 0..count {
        for j in (i + 1)..count {
            let (Some(a), Some(b), Some(c), Some(d)) = (
                points.get(i),
                points.get((i + 1) % count),
                points.get(j),
                points.get((j + 1) % count),
            ) else {
                continue;
            };
            let neighbours = j == i + 1 || (i == 0 && j + 1 == count);
            match segment_crossing(*a, *b, *c, *d) {
                SegmentCrossing::Apart => {}
                SegmentCrossing::Overlap => return true,
                SegmentCrossing::At { .. } => {
                    if !neighbours {
                        return true;
                    }
                }
            }
        }
    }
    false
}

// The first crossing in a closed polyline that is not two neighbours sharing their point: the two segments, and where.
#[allow(clippy::cast_precision_loss)]
fn first_crossing(points: &[Point]) -> Option<(usize, usize, Point)> {
    let count = points.len();
    for i in 0..count {
        for j in (i + 1)..count {
            let (a, b, c, d) = (
                points.get(i)?,
                points.get((i + 1) % count)?,
                points.get(j)?,
                points.get((j + 1) % count)?,
            );
            let neighbours = j == i + 1 || (i == 0 && j + 1 == count);
            match segment_crossing(*a, *b, *c, *d) {
                SegmentCrossing::Apart => {}
                SegmentCrossing::Overlap => return Some((i, j, *c)),
                SegmentCrossing::At { first, .. } => {
                    if !neighbours {
                        let t = first.top() as f64 / first.bottom() as f64;
                        let at = rounded((
                            (a.x() as f64) + t * ((b.x() - a.x()) as f64),
                            (a.y() as f64) + t * ((b.y() - a.y()) as f64),
                        ));
                        return Some((i, j, at));
                    }
                }
            }
        }
    }
    None
}

// Cuts each remaining loop at its crossing, keeping the longer way round. The result still has to pass the clearance check.
fn cut_loops(mut points: Vec<Point>) -> Vec<Point> {
    for _ in 0..points.len().saturating_mul(4).max(16) {
        let Some((i, j, at)) = first_crossing(&points) else {
            return points;
        };
        let count = points.len();
        let inner = j - i;
        points = if inner * 2 <= count {
            // Drop the points between the two segments.
            let mut kept: Vec<Point> = points.iter().take(i + 1).copied().collect();
            kept.push(at);
            kept.extend(points.iter().skip(j + 1).copied());
            kept
        } else {
            // Drop the points around the far side instead.
            let mut kept = vec![at];
            kept.extend(points.iter().skip(i + 1).take(inner).copied());
            kept
        };
        points = tidied(points);
        if points.len() < 3 {
            return points;
        }
    }
    points
}

/// Removes every loop from a closed polyline by walking its outer boundary: split at every crossing, start at a point certainly on
/// the outside, and at each junction keep to the outside. Rounding a crossing to the grid can leave a new crossing a micrometre
/// away, so this repeats, and any loop left after that is cut at its crossing.
fn outer_boundary(points: &[Point]) -> Option<Vec<Point>> {
    let mut current = tidied(points.to_vec());
    for _ in 0..4 {
        if !crossings_of(&current) {
            return Some(current);
        }
        current = tidied(walk_outside(&current));
    }
    current = cut_loops(current);
    (current.len() >= 3 && !crossings_of(&current)).then_some(current)
}

#[allow(clippy::cast_precision_loss)]
fn fraction_along(a: Point, b: Point, point: Point) -> f64 {
    let (dx, dy) = vector(a, b);
    let (px, py) = vector(a, point);
    let length = dx * dx + dy * dy;
    if length == 0.0 {
        0.0
    } else {
        (px * dx + py * dy) / length
    }
}

fn walk_outside(closed: &[Point]) -> Vec<Point> {
    let count = closed.len();
    if count < 3 {
        return closed.to_vec();
    }
    // Split every segment at every point where another touches it.
    let mut cuts: Vec<Vec<(f64, Point)>> = vec![Vec::new(); count];
    for i in 0..count {
        for j in (i + 1)..count {
            let (Some(a), Some(b), Some(c), Some(d)) = (
                closed.get(i),
                closed.get((i + 1) % count),
                closed.get(j),
                closed.get((j + 1) % count),
            ) else {
                continue;
            };
            match segment_crossing(*a, *b, *c, *d) {
                SegmentCrossing::Apart => {}
                SegmentCrossing::At { first, .. } => {
                    #[allow(clippy::cast_precision_loss)]
                    let t = first.top() as f64 / first.bottom() as f64;
                    #[allow(clippy::cast_precision_loss)]
                    let at = rounded((
                        (a.x() as f64) + t * ((b.x() - a.x()) as f64),
                        (a.y() as f64) + t * ((b.y() - a.y()) as f64),
                    ));
                    if let Some(list) = cuts.get_mut(i) {
                        list.push((t, at));
                    }
                    if let Some(list) = cuts.get_mut(j) {
                        list.push((fraction_along(*c, *d, at), at));
                    }
                }
                SegmentCrossing::Overlap => {
                    // Overlapping runs split at each other's ends.
                    for point in [c, d] {
                        let t = fraction_along(*a, *b, *point);
                        if (0.0..=1.0).contains(&t) {
                            if let Some(list) = cuts.get_mut(i) {
                                list.push((t, *point));
                            }
                        }
                    }
                    for point in [a, b] {
                        let t = fraction_along(*c, *d, *point);
                        if (0.0..=1.0).contains(&t) {
                            if let Some(list) = cuts.get_mut(j) {
                                list.push((t, *point));
                            }
                        }
                    }
                }
            }
        }
    }
    // Walk the split segments into directed links from each point.
    let mut links: BTreeMap<Point, Vec<Point>> = BTreeMap::new();
    for i in 0..count {
        let (Some(a), Some(b)) = (closed.get(i), closed.get((i + 1) % count)) else {
            continue;
        };
        let mut stops = cuts.get(i).cloned().unwrap_or_default();
        stops.sort_by(|left, right| left.0.total_cmp(&right.0));
        let mut previous = *a;
        for (_, stop) in stops.iter().chain(core::iter::once(&(1.0, *b))) {
            if *stop != previous {
                let targets = links.entry(previous).or_default();
                if !targets.contains(stop) {
                    targets.push(*stop);
                }
                previous = *stop;
            }
        }
    }
    // The lowest point, then the leftmost, is certainly on the outer boundary.
    let Some(start) = links
        .keys()
        .copied()
        .min_by_key(|point| (point.y(), point.x()))
    else {
        return closed.to_vec();
    };
    let mut walked = vec![start];
    let (mut before, mut current) = (start, start);
    let limit = links.values().map(Vec::len).sum::<usize>() + 2;
    for step in 0..limit {
        let Some(options) = links.get(&current) else {
            break;
        };
        let next = if step == 0 {
            // Leaving the lowest point, the outside is the direction turning least to the left of due east.
            options.iter().copied().min_by(|left, right| {
                let angle = |p: Point| {
                    let v = vector(current, p);
                    v.1.atan2(v.0).rem_euclid(core::f64::consts::TAU)
                };
                angle(*left).total_cmp(&angle(*right))
            })
        } else {
            options.iter().copied().max_by(|left, right| {
                clockwise_turn(before, current, *left)
                    .total_cmp(&clockwise_turn(before, current, *right))
            })
        };
        let Some(next) = next else { break };
        if next == start {
            return walked;
        }
        walked.push(next);
        before = current;
        current = next;
    }
    // A walk that never returned means the links are broken, so hand back the input for the checks to refuse.
    closed.to_vec()
}

// The distance from a point to a polyline, and whether its nearest place is one of the polyline's two ends.
#[allow(clippy::cast_precision_loss)]
fn point_to_polyline(net: &[Point], point: Point) -> (f64, bool) {
    let (px, py) = (point.x() as f64, point.y() as f64);
    let mut best = (f64::MAX, false);
    let last = net.len().saturating_sub(2);
    for (at, pair) in net.windows(2).enumerate() {
        let [a, b] = pair else { continue };
        let (ax, ay) = (a.x() as f64, a.y() as f64);
        let (dx, dy) = ((b.x() as f64) - ax, (b.y() as f64) - ay);
        let length = dx * dx + dy * dy;
        let t = if length == 0.0 {
            0.0
        } else {
            (((px - ax) * dx + (py - ay) * dy) / length).clamp(0.0, 1.0)
        };
        let here = (px - (ax + t * dx)).hypot(py - (ay + t * dy));
        if here < best.0 {
            best = (here, (at == 0 && t <= 0.0) || (at == last && t >= 1.0));
        }
    }
    best
}

// One closed polyline has one written form: it starts at its lowest point, then its leftmost.
fn canonical_start(mut points: Vec<Point>) -> Vec<Point> {
    let start = points
        .iter()
        .enumerate()
        .min_by_key(|(_, point)| (point.y(), point.x()))
        .map_or(0, |(at, _)| at);
    points.rotate_left(start);
    points
}

impl Outline {
    /// This boundary with its allowances, one per curve in order.
    ///
    /// `tolerance` is the bound on geometric error, in micrometres. The result has no self-crossings, and every point of it whose
    /// nearest place is inside an edge is at least that edge's width from it, within the tolerance. A corner join may come closer to
    /// the corner: a slanted cut has to.
    ///
    /// # Errors
    /// See [`OffsetError`]. Each refusal names the edge at fault.
    pub fn offset(&self, allowances: &[Allowance], tolerance: i64) -> Result<Offset, OffsetError> {
        let tolerance = tolerance.max(1);
        if allowances.len() != self.curves().len() {
            return Err(OffsetError::WrongCount {
                curves: self.curves().len(),
                allowances: allowances.len(),
            });
        }
        if let Some(edge) = allowances.iter().position(|allowance| allowance.width < 0) {
            return Err(OffsetError::NegativeWidth { edge });
        }
        let flat = tolerance;
        // A boundary that crosses itself has no one direction, so saying which way it runs would name the wrong fault.
        if let Some(crossing) = self.self_crossings(flat).first() {
            return Err(OffsetError::NetCrossing {
                first: crossing.first,
                second: crossing.second,
            });
        }
        if self.winding(flat) != Winding::Counterclockwise {
            return Err(OffsetError::NotCounterclockwise);
        }
        let edges: Vec<Edge> = self
            .curves()
            .iter()
            .zip(allowances)
            .map(|(curve, allowance)| edge_of(*curve, *allowance, flat))
            .collect();
        let count = edges.len();
        let joins: Vec<Joined> = (0..count)
            .filter_map(|at| {
                let (edge, next) = (edges.get(at)?, edges.get((at + 1) % count)?);
                Some(join(edge, next, *next.net.first()?))
            })
            .collect();
        let mut raw = Vec::new();
        for (at, edge) in edges.iter().enumerate() {
            let skip_start = joins
                .get((at + count - 1) % count)
                .is_some_and(|joined| joined.replaces_start);
            let skip_end = joins.get(at).is_some_and(|joined| joined.replaces_end);
            let last = edge.offset.len().saturating_sub(1);
            for (index, point) in edge.offset.iter().enumerate() {
                if !((skip_start && index == 0) || (skip_end && index == last)) {
                    raw.push(*point);
                }
            }
            if let Some(joined) = joins.get(at) {
                raw.extend_from_slice(&joined.points);
            }
        }
        let points = outer_boundary(&raw).ok_or(OffsetError::Tangled)?;
        let swallowed = check(&edges, &points, tolerance)?;
        Ok(Offset {
            points: canonical_start(points),
            swallowed,
        })
    }
}

fn edge_of(curve: Curve, allowance: Allowance, flat: i64) -> Edge {
    let net = curve.flatten(flat);
    let direction = |a: Option<&Point>, b: Option<&Point>| match (a, b) {
        (Some(a), Some(b)) => unit(vector(*a, *b)),
        _ => (0.0, 0.0),
    };
    let start_direction = direction(net.first(), net.get(1));
    let end_direction = direction(
        net.len().checked_sub(2).and_then(|at| net.get(at)),
        net.last(),
    );
    let offset = offset_polyline(&net, allowance.width, flat);
    Edge {
        width: allowance.width,
        corner: allowance.corner,
        net,
        start_direction,
        end_direction,
        offset,
    }
}

// Points along the result: its corners, and enough points between them, since a long straight run has only its two ends.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn samples(points: &[Point]) -> Vec<Point> {
    const BETWEEN: i64 = 8;
    let count = points.len();
    let mut out = Vec::with_capacity(count * usize::try_from(BETWEEN).unwrap_or(8));
    for at in 0..count {
        let (Some(a), Some(b)) = (points.get(at), points.get((at + 1) % count)) else {
            continue;
        };
        for step in 0..BETWEEN {
            out.push(Point::clamped(
                a.x() + (b.x() - a.x()) * step / BETWEEN,
                a.y() + (b.y() - a.y()) * step / BETWEEN,
            ));
        }
    }
    out
}

// The result must clear each edge by its width. A corner join may come closer to the corner itself, since a slanted cut has to, so
// only a place whose nearest point is inside the edge is held to it. Returns the edges with no stretch of their own.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn check(edges: &[Edge], points: &[Point], tolerance: i64) -> Result<Vec<usize>, OffsetError> {
    let slack = (tolerance + 2) as f64;
    let along = samples(points);
    let mut swallowed = Vec::new();
    for (at, edge) in edges.iter().enumerate() {
        if edge.width == 0 {
            continue;
        }
        let wanted = edge.width as f64;
        let mut closest = f64::MAX;
        let mut present = false;
        for point in &along {
            let (here, at_end) = point_to_polyline(&edge.net, *point);
            if at_end {
                continue;
            }
            closest = closest.min(here);
            if (here - wanted).abs() <= slack {
                present = true;
            }
        }
        if !present {
            swallowed.push(at);
        }
        let narrowest_neighbour = neighbour_width(edges, at);
        let floor = wanted.min(narrowest_neighbour as f64) - slack;
        if closest < floor {
            return Err(OffsetError::BoundExceeded {
                edge: at,
                short_by: (floor - closest).ceil() as i64,
            });
        }
    }
    Ok(swallowed)
}

fn neighbour_width(edges: &[Edge], at: usize) -> i64 {
    let count = edges.len();
    let before = edges
        .get((at + count - 1) % count)
        .map_or(0, |edge| edge.width);
    let after = edges.get((at + 1) % count).map_or(0, |edge| edge.width);
    before.min(after)
}
