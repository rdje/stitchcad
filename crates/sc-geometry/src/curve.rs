//! The three curves the roadmap's interchange diet allows, and nothing else.
use crate::predicate::{Point, PointError};
use core::fmt;
use sc_units::pure;

/// The chordal tolerance used when a caller states none: 100 micrometres, the fixture's polyline bound.
pub const DEFAULT_TOLERANCE: i64 = 100;

/// Deepest subdivision of one curve, which bounds the work and the size of any polyline.
const MAX_DEPTH: u32 = 16;

/// A line, a circular arc or a cubic Bézier, between whole-micrometre points.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Curve {
    /// A straight line.
    Line {
        /// Where it starts.
        from: Point,
        /// Where it ends.
        to: Point,
    },
    /// The circular arc from `from` to `to` through `through`.
    Arc {
        /// Where it starts.
        from: Point,
        /// A point on the arc between its ends.
        through: Point,
        /// Where it ends.
        to: Point,
    },
    /// A cubic Bézier.
    Cubic {
        /// Where it starts.
        from: Point,
        /// The first control point.
        first: Point,
        /// The second control point.
        second: Point,
        /// Where it ends.
        to: Point,
    },
}

/// A curve kind this version does not hold, or the wrong number of points for one it does.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CurveKindError {
    /// The roadmap defers this kind. NURBS is the one that matters here: it is refused rather than approximated.
    Unsupported(String),
    /// The kind needs a different number of points.
    WrongPointCount {
        /// The kind named.
        kind: &'static str,
        /// How many it needs.
        needed: usize,
        /// How many were given.
        given: usize,
    },
    /// A point is out of range.
    Point(PointError),
}

impl fmt::Display for CurveKindError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported(kind) => {
                write!(f, "a {kind} curve is not supported: curves here are lines, circular arcs and cubic Béziers")
            }
            Self::WrongPointCount {
                kind,
                needed,
                given,
            } => {
                write!(f, "a {kind} takes {needed} points, and {given} were given")
            }
            Self::Point(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for CurveKindError {}

const fn midpoint(a: Point, b: Point) -> (i64, i64) {
    // Rounds toward negative infinity, deterministically on every platform.
    (a.x().midpoint(b.x()), a.y().midpoint(b.y()))
}

fn between(a: Point, b: Point) -> Point {
    let (x, y) = midpoint(a, b);
    Point::clamped(x, y)
}

fn isqrt(value: u128) -> u128 {
    if value < 2 {
        return value;
    }
    let mut guess = 1u128 << ((128 - value.leading_zeros()).div_ceil(2));
    loop {
        let next = (guess + value / guess) / 2;
        if next >= guess {
            return guess;
        }
        guess = next;
    }
}

/// The distance between two points, rounded to the nearest micrometre.
#[must_use]
pub(crate) fn distance(a: Point, b: Point) -> i64 {
    let (dx, dy) = (
        i128::from(b.x()) - i128::from(a.x()),
        i128::from(b.y()) - i128::from(a.y()),
    );
    let squared = (dx * dx + dy * dy).unsigned_abs();
    let root = isqrt(squared);
    let rounded = if squared - root * root > root {
        root + 1
    } else {
        root
    };
    i64::try_from(rounded).unwrap_or(i64::MAX)
}

// Whether a point lies within `tolerance` of the chord from `a` to `b`, including past the chord's ends, where a Bézier's
// control point can overshoot. A control polygon within the tolerance keeps its Bézier within it too.
fn within_chord(a: Point, b: Point, point: Point, tolerance: i64) -> bool {
    let (dx, dy) = (
        i128::from(b.x()) - i128::from(a.x()),
        i128::from(b.y()) - i128::from(a.y()),
    );
    let length_squared = dx * dx + dy * dy;
    if length_squared == 0 {
        return distance(a, point) <= tolerance;
    }
    let (px, py) = (
        i128::from(point.x()) - i128::from(a.x()),
        i128::from(point.y()) - i128::from(a.y()),
    );
    let along = px * dx + py * dy;
    if along < 0 {
        return distance(a, point) <= tolerance;
    }
    if along > length_squared {
        return distance(b, point) <= tolerance;
    }
    let cross = crate::predicate::cross(a, b, point).unsigned_abs();
    let limit = u128::from(tolerance.unsigned_abs()) * isqrt(length_squared.unsigned_abs());
    cross <= limit
}

impl Curve {
    /// A curve from its kind's name and its points, as a file or an agent would state it.
    ///
    /// # Errors
    /// Refuses any kind but `line`, `arc` and `cubic`, the wrong number of points, and a point out of range.
    pub fn from_kind(kind: &str, points: &[(i64, i64)]) -> Result<Self, CurveKindError> {
        let mut checked = Vec::with_capacity(points.len());
        for (x, y) in points {
            checked.push(Point::new(*x, *y).map_err(CurveKindError::Point)?);
        }
        let count = |name, needed| CurveKindError::WrongPointCount {
            kind: name,
            needed,
            given: points.len(),
        };
        match kind {
            "line" => match checked.as_slice() {
                [from, to] => Ok(Self::Line {
                    from: *from,
                    to: *to,
                }),
                _ => Err(count("line", 2)),
            },
            "arc" => match checked.as_slice() {
                [from, through, to] => Ok(Self::Arc {
                    from: *from,
                    through: *through,
                    to: *to,
                }),
                _ => Err(count("arc", 3)),
            },
            "cubic" => match checked.as_slice() {
                [from, first, second, to] => Ok(Self::Cubic {
                    from: *from,
                    first: *first,
                    second: *second,
                    to: *to,
                }),
                _ => Err(count("cubic", 4)),
            },
            other => Err(CurveKindError::Unsupported(other.to_owned())),
        }
    }

    /// Where the curve starts.
    #[must_use]
    pub const fn from(self) -> Point {
        match self {
            Self::Line { from, .. } | Self::Arc { from, .. } | Self::Cubic { from, .. } => from,
        }
    }

    /// Where the curve ends.
    #[must_use]
    pub const fn to(self) -> Point {
        match self {
            Self::Line { to, .. } | Self::Arc { to, .. } | Self::Cubic { to, .. } => to,
        }
    }

    /// The same curve traversed the other way, exactly.
    #[must_use]
    pub const fn reversed(self) -> Self {
        match self {
            Self::Line { from, to } => Self::Line { from: to, to: from },
            Self::Arc { from, through, to } => Self::Arc {
                from: to,
                through,
                to: from,
            },
            Self::Cubic {
                from,
                first,
                second,
                to,
            } => Self::Cubic {
                from: to,
                first: second,
                second: first,
                to: from,
            },
        }
    }

    /// A polyline that stays within `tolerance` micrometres of the curve, with exactly the curve's ends. Each interior
    /// point is rounded to a whole micrometre, so the result can be up to one micrometre further than the tolerance says.
    #[must_use]
    pub fn flatten(self, tolerance: i64) -> Vec<Point> {
        let tolerance = tolerance.max(1);
        let mut out = vec![self.from()];
        match self {
            Self::Line { to, .. } => out.push(to),
            Self::Arc { from, through, to } => flatten_arc(from, through, to, tolerance, &mut out),
            Self::Cubic {
                from,
                first,
                second,
                to,
            } => flatten_cubic([from, first, second, to], tolerance, 0, &mut out),
        }
        out
    }

    /// The two parts of this curve, cut where a walk along it has gone `distance`. A cut at or past an end gives nothing, because a part with no length is no part.
    #[must_use]
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    pub fn split_at_length(self, distance: i64, tolerance: i64) -> Option<(Self, Self)> {
        let total = self.length(tolerance);
        if distance <= 0 || distance >= total {
            return None;
        }
        let share = distance as f64 / total as f64;
        match self {
            Self::Line { from, to } => {
                let at = Point::clamped(
                    from.x() + ((to.x() - from.x()) as f64 * share).round() as i64,
                    from.y() + ((to.y() - from.y()) as f64 * share).round() as i64,
                );
                (at != from && at != to)
                    .then_some((Self::Line { from, to: at }, Self::Line { from: at, to }))
            }
            Self::Arc { from, through, to } => {
                let (centre_x, centre_y, radius, start, sweep) = arc_sweep(from, through, to)?;
                let on = |theta: f64| {
                    let (sine, cosine) = pure::sin_cos(theta);
                    Point::clamped(
                        (centre_x + radius * cosine).round() as i64,
                        (centre_y + radius * sine).round() as i64,
                    )
                };
                let at = on(start + sweep * share);
                Some((
                    Self::Arc {
                        from,
                        through: on(start + sweep * share / 2.0),
                        to: at,
                    },
                    Self::Arc {
                        from: at,
                        through: on(start + sweep * (1.0 + share) / 2.0),
                        to,
                    },
                ))
            }
            Self::Cubic {
                from,
                first,
                second,
                to,
            } => {
                let points = [from, first, second, to];
                // The curve's own parameter is not its length, so it is found by halving. Forty halvings is far finer than a micrometre on a garment.
                let (mut low, mut high) = (0.0, 1.0);
                for _ in 0..40 {
                    let middle = f64::midpoint(low, high);
                    if cut_cubic(points, middle).0.length(tolerance) < distance {
                        low = middle;
                    } else {
                        high = middle;
                    }
                }
                Some(cut_cubic(points, f64::midpoint(low, high)))
            }
        }
    }

    /// The curve's length, to within the tolerance's effect on a polyline, rounded to a micrometre.
    #[must_use]
    pub fn length(self, tolerance: i64) -> i64 {
        self.flatten(tolerance)
            .windows(2)
            .fold(0i64, |sum, pair| match pair {
                [a, b] => sum.saturating_add(distance(*a, *b)),
                _ => sum,
            })
    }
}

fn flatten_cubic(points: [Point; 4], tolerance: i64, depth: u32, out: &mut Vec<Point>) {
    let [a, b, c, d] = points;
    if depth >= MAX_DEPTH || (within_chord(a, d, b, tolerance) && within_chord(a, d, c, tolerance))
    {
        out.push(d);
        return;
    }
    let (ab, bc, cd) = (between(a, b), between(b, c), between(c, d));
    let (abc, bcd) = (between(ab, bc), between(bc, cd));
    let middle = between(abc, bcd);
    flatten_cubic([a, ab, abc, middle], tolerance, depth + 1, out);
    flatten_cubic([middle, bcd, cd, d], tolerance, depth + 1, out);
}

// An arc is split by halving its chord and projecting back onto the circle, in floating point only for that projection,
// whose result is then rounded to the micrometre grid. A straight or doubled-back arc becomes straight lines.
// A cubic cut in two at a parameter, by de Casteljau's construction.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn cut_cubic(points: [Point; 4], at: f64) -> (Curve, Curve) {
    let [a, b, c, d] = points;
    let mix = |p: (f64, f64), q: (f64, f64)| (p.0 + (q.0 - p.0) * at, p.1 + (q.1 - p.1) * at);
    let raw = |p: Point| (p.x() as f64, p.y() as f64);
    let (ab, bc, cd) = (
        mix(raw(a), raw(b)),
        mix(raw(b), raw(c)),
        mix(raw(c), raw(d)),
    );
    let (abc, bcd) = (mix(ab, bc), mix(bc, cd));
    let middle = mix(abc, bcd);
    let whole = |p: (f64, f64)| Point::clamped(p.0.round() as i64, p.1.round() as i64);
    let joint = whole(middle);
    (
        Curve::Cubic {
            from: a,
            first: whole(ab),
            second: whole(abc),
            to: joint,
        },
        Curve::Cubic {
            from: joint,
            first: whole(bcd),
            second: whole(cd),
            to: d,
        },
    )
}

// An arc's circle, the angle it starts at, and the signed angle it sweeps. Three points in a line have none.
#[allow(clippy::cast_precision_loss)]
fn arc_sweep(from: Point, through: Point, to: Point) -> Option<(f64, f64, f64, f64, f64)> {
    let (centre_x, centre_y, radius) = circle(from, through, to)?;
    let angle = |p: Point| pure::atan2((p.y() as f64) - centre_y, (p.x() as f64) - centre_x);
    let (start, middle, end) = (angle(from), angle(through), angle(to));
    let two_pi = core::f64::consts::TAU;
    let ccw = (middle - start).rem_euclid(two_pi) < (end - start).rem_euclid(two_pi);
    let sweep = if ccw {
        (end - start).rem_euclid(two_pi)
    } else {
        -(start - end).rem_euclid(two_pi)
    };
    Some((centre_x, centre_y, radius, start, sweep))
}

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn flatten_arc(from: Point, through: Point, to: Point, tolerance: i64, out: &mut Vec<Point>) {
    let Some((centre_x, centre_y, radius, start, sweep)) = arc_sweep(from, through, to) else {
        out.push(through);
        out.push(to);
        return;
    };
    // The sagitta of a chord spanning angle θ is r(1 − cos(θ/2)).
    let bound = (tolerance as f64 / radius).min(1.0);
    let step = 2.0 * pure::acos(1.0 - bound);
    let pieces = (sweep.abs() / step.max(1e-6)).ceil().clamp(1.0, 1024.0) as u32;
    for piece in 1..pieces {
        let theta = start + sweep * f64::from(piece) / f64::from(pieces);
        let (sine, cosine) = pure::sin_cos(theta);
        out.push(Point::clamped(
            (centre_x + radius * cosine).round() as i64,
            (centre_y + radius * sine).round() as i64,
        ));
    }
    out.push(to);
}

#[allow(clippy::cast_precision_loss)]
fn circle(a: Point, b: Point, c: Point) -> Option<(f64, f64, f64)> {
    let cross = crate::predicate::cross(a, b, c);
    if cross == 0 {
        return None;
    }
    let (bx, by) = ((b.x() - a.x()) as f64, (b.y() - a.y()) as f64);
    let (cx, cy) = ((c.x() - a.x()) as f64, (c.y() - a.y()) as f64);
    let d = 2.0 * (cross as f64);
    let (bb, cc) = (bx * bx + by * by, cx * cx + cy * cy);
    let (ux, uy) = ((cy * bb - by * cc) / d, (bx * cc - cx * bb) / d);
    Some((
        (a.x() as f64) + ux,
        (a.y() as f64) + uy,
        pure::hypot(ux, uy),
    ))
}
