//! Exact predicates on whole micrometres. Integer arithmetic has no rounding, so a near tie cannot flip.
use core::fmt;

/// The largest coordinate allowed, in either direction: over a thousand kilometres, far past any pattern. Staying
/// inside it means every product here fits an `i128`, so no predicate can overflow.
pub const MAX_COORDINATE: i64 = 1 << 60;

/// A point in whole micrometres.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    x: i64,
    y: i64,
}

/// A coordinate is outside what the kernel accepts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PointError {
    /// The refused horizontal coordinate.
    pub x: i64,
    /// The refused vertical coordinate.
    pub y: i64,
}

impl fmt::Display for PointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "point ({}, {}) is outside the kernel's range of 2^60 micrometres",
            self.x, self.y
        )
    }
}

impl std::error::Error for PointError {}

impl Point {
    /// A point at whole micrometres.
    ///
    /// # Errors
    /// A coordinate is beyond [`MAX_COORDINATE`].
    pub const fn new(x: i64, y: i64) -> Result<Self, PointError> {
        if x.abs_diff(0) > MAX_COORDINATE as u64 || y.abs_diff(0) > MAX_COORDINATE as u64 {
            return Err(PointError { x, y });
        }
        Ok(Self { x, y })
    }

    // For a point computed from points already in range, clamped so a rounding error cannot leave the range.
    pub(crate) const fn clamped(x: i64, y: i64) -> Self {
        const fn inside(value: i64) -> i64 {
            if value > MAX_COORDINATE {
                MAX_COORDINATE
            } else if value < -MAX_COORDINATE {
                -MAX_COORDINATE
            } else {
                value
            }
        }
        Self {
            x: inside(x),
            y: inside(y),
        }
    }

    /// Across, in micrometres.
    #[must_use]
    pub const fn x(self) -> i64 {
        self.x
    }

    /// Up, in micrometres.
    #[must_use]
    pub const fn y(self) -> i64 {
        self.y
    }
}

/// Which side of a directed line a point lies on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    /// To the left, turning counterclockwise.
    Left,
    /// To the right, turning clockwise.
    Right,
    /// Exactly on the line.
    Collinear,
}

/// Which side of the line from `a` to `b` the point `c` lies on, exactly.
#[must_use]
pub const fn orientation(a: Point, b: Point, c: Point) -> Orientation {
    let cross = cross(a, b, c);
    if cross > 0 {
        Orientation::Left
    } else if cross < 0 {
        Orientation::Right
    } else {
        Orientation::Collinear
    }
}

// Each difference fits 62 bits, so each product fits 124 and their difference fits an i128.
pub(crate) const fn cross(a: Point, b: Point, c: Point) -> i128 {
    let (bx, by) = ((b.x as i128) - (a.x as i128), (b.y as i128) - (a.y as i128));
    let (cx, cy) = ((c.x as i128) - (a.x as i128), (c.y as i128) - (a.y as i128));
    bx * cy - by * cx
}

/// An exact fraction, always in lowest terms with a positive denominator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fraction {
    top: i128,
    bottom: i128,
}

const fn gcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let next = a % b;
        a = b;
        b = next;
    }
    a
}

impl Fraction {
    pub(crate) const fn new(top: i128, bottom: i128) -> Self {
        let divisor = gcd(top, bottom);
        let divisor = if divisor == 0 { 1 } else { divisor };
        let sign = if bottom < 0 { -1 } else { 1 };
        Self {
            top: sign * top / divisor,
            bottom: sign * bottom / divisor,
        }
    }

    /// The numerator.
    #[must_use]
    pub const fn top(self) -> i128 {
        self.top
    }

    /// The denominator, which is always positive.
    #[must_use]
    pub const fn bottom(self) -> i128 {
        self.bottom
    }
}

/// How two straight segments meet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SegmentCrossing {
    /// They do not touch.
    Apart,
    /// They touch at one point, as a fraction along each: 0 is a segment's start and 1 its end.
    At {
        /// How far along the first segment.
        first: Fraction,
        /// How far along the second segment.
        second: Fraction,
    },
    /// They lie along the same line and share more than one point.
    Overlap,
}

const fn on_segment(a: Point, b: Point, c: Point) -> bool {
    let (low_x, high_x) = if a.x < b.x { (a.x, b.x) } else { (b.x, a.x) };
    let (low_y, high_y) = if a.y < b.y { (a.y, b.y) } else { (b.y, a.y) };
    low_x <= c.x && c.x <= high_x && low_y <= c.y && c.y <= high_y
}

/// Whether and where the segments `a`–`b` and `c`–`d` meet, exactly.
#[must_use]
pub fn segment_crossing(a: Point, b: Point, c: Point, d: Point) -> SegmentCrossing {
    let (ax, ay) = (i128::from(a.x), i128::from(a.y));
    let (cx, cy) = (i128::from(c.x), i128::from(c.y));
    let (rx, ry) = (i128::from(b.x) - ax, i128::from(b.y) - ay);
    let (sx, sy) = (i128::from(d.x) - cx, i128::from(d.y) - cy);
    let denominator = rx * sy - ry * sx;
    let (qx, qy) = (cx - ax, cy - ay);
    if denominator == 0 {
        // Both tests are needed: when one segment is a single point, its own direction is zero and says nothing.
        if qx * ry - qy * rx != 0 || qx * sy - qy * sx != 0 {
            return SegmentCrossing::Apart;
        }
        // On one line. Which endpoints fall on the other segment says how they meet.
        let mut shared: Vec<Point> = Vec::with_capacity(4);
        for point in [a, b] {
            if on_segment(c, d, point) && !shared.contains(&point) {
                shared.push(point);
            }
        }
        for point in [c, d] {
            if on_segment(a, b, point) && !shared.contains(&point) {
                shared.push(point);
            }
        }
        return match shared.as_slice() {
            [] => SegmentCrossing::Apart,
            [one] => SegmentCrossing::At {
                first: fraction_along(a, b, *one),
                second: fraction_along(c, d, *one),
            },
            _ => SegmentCrossing::Overlap,
        };
    }
    // Written over a common denominator: `first = t / denominator`, `second = u / denominator`.
    let t = qx * sy - qy * sx;
    let u = qx * ry - qy * rx;
    let inside = |value: i128| {
        if denominator > 0 {
            0 <= value && value <= denominator
        } else {
            denominator <= value && value <= 0
        }
    };
    if inside(t) && inside(u) {
        SegmentCrossing::At {
            first: Fraction::new(t, denominator),
            second: Fraction::new(u, denominator),
        }
    } else {
        SegmentCrossing::Apart
    }
}

fn fraction_along(a: Point, b: Point, point: Point) -> Fraction {
    let (dx, dy) = (
        i128::from(b.x) - i128::from(a.x),
        i128::from(b.y) - i128::from(a.y),
    );
    if dx != 0 {
        Fraction::new(i128::from(point.x) - i128::from(a.x), dx)
    } else if dy != 0 {
        Fraction::new(i128::from(point.y) - i128::from(a.y), dy)
    } else {
        Fraction::new(0, 1)
    }
}
