//! A closed chain of curves: a piece's boundary, with its winding and its self-crossings.
use crate::curve::Curve;
use crate::predicate::{segment_crossing, Point, SegmentCrossing};
use core::fmt;

/// A closed boundary. Each curve starts exactly where the last ended, and the last ends where the first starts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Outline {
    curves: Vec<Curve>,
}

/// Which way a closed boundary runs. The roadmap's outer boundaries run counterclockwise.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Winding {
    /// Counterclockwise, with the inside on the left.
    Counterclockwise,
    /// Clockwise, with the inside on the right.
    Clockwise,
    /// It encloses no area.
    Degenerate,
}

/// Why a chain of curves is not a boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutlineError {
    /// There are no curves.
    Empty,
    /// A curve does not start where the one before it ends.
    Gap {
        /// The curve that does not connect, by position.
        at: usize,
        /// Where the one before it ended.
        ended: Point,
        /// Where this one starts.
        starts: Point,
    },
    /// A curve starts and ends at one point and is not a closed arc or Bézier.
    ZeroLength {
        /// The curve's position.
        at: usize,
    },
}

impl fmt::Display for OutlineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("a boundary needs at least one curve"),
            Self::Gap { at, ended, starts } => write!(
                f,
                "curve {at} starts at ({}, {}), and the curve before it ended at ({}, {})",
                starts.x(),
                starts.y(),
                ended.x(),
                ended.y()
            ),
            Self::ZeroLength { at } => write!(f, "curve {at} has no length"),
        }
    }
}

impl std::error::Error for OutlineError {}

/// Where a boundary crosses itself: the two curves, by position.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Crossing {
    /// The earlier curve.
    pub first: usize,
    /// The later curve.
    pub second: usize,
}

impl Outline {
    /// A boundary from its curves.
    ///
    /// # Errors
    /// There are no curves, consecutive curves do not meet exactly, or a straight line has no length.
    pub fn new(curves: Vec<Curve>) -> Result<Self, OutlineError> {
        let Some(last) = curves.last() else {
            return Err(OutlineError::Empty);
        };
        let mut ended = last.to();
        for (at, curve) in curves.iter().enumerate() {
            if curve.from() != ended {
                return Err(OutlineError::Gap {
                    at,
                    ended,
                    starts: curve.from(),
                });
            }
            if let Curve::Line { from, to } = curve {
                if from == to {
                    return Err(OutlineError::ZeroLength { at });
                }
            }
            ended = curve.to();
        }
        Ok(Self { curves })
    }

    /// The curves, in order.
    #[must_use]
    pub fn curves(&self) -> &[Curve] {
        &self.curves
    }

    /// Twice the enclosed area of the boundary's polyline, in square micrometres: positive counterclockwise.
    #[must_use]
    pub fn doubled_area(&self, tolerance: i64) -> i128 {
        let points = self.polyline(tolerance);
        let mut total = 0i128;
        let mut previous = points.last().copied();
        for point in &points {
            if let Some(before) = previous {
                total += i128::from(before.x()) * i128::from(point.y())
                    - i128::from(point.x()) * i128::from(before.y());
            }
            previous = Some(*point);
        }
        total
    }

    /// Which way the boundary runs.
    #[must_use]
    pub fn winding(&self, tolerance: i64) -> Winding {
        match self.doubled_area(tolerance) {
            area if area > 0 => Winding::Counterclockwise,
            area if area < 0 => Winding::Clockwise,
            _ => Winding::Degenerate,
        }
    }

    /// The same boundary running the other way, exactly.
    #[must_use]
    pub fn reversed(&self) -> Self {
        Self {
            curves: self
                .curves
                .iter()
                .rev()
                .map(|curve| curve.reversed())
                .collect(),
        }
    }

    /// The whole boundary as a closed polyline, leaving out the repeated closing point.
    #[must_use]
    pub fn polyline(&self, tolerance: i64) -> Vec<Point> {
        self.curves
            .iter()
            .flat_map(|curve| {
                let mut points = curve.flatten(tolerance);
                points.pop();
                points
            })
            .collect()
    }

    /// The boundary's length, rounded to a micrometre per chord.
    #[must_use]
    pub fn length(&self, tolerance: i64) -> i64 {
        self.curves.iter().fold(0i64, |sum, curve| {
            sum.saturating_add(curve.length(tolerance))
        })
    }

    /// Every pair of curves whose polylines cross or overlap, apart from the shared end of two neighbours.
    #[must_use]
    pub fn self_crossings(&self, tolerance: i64) -> Vec<Crossing> {
        let spans: Vec<Vec<Point>> = self
            .curves
            .iter()
            .map(|curve| curve.flatten(tolerance))
            .collect();
        let count = spans.len();
        let mut found = Vec::new();
        for first in 0..count {
            // A curve's own segments are compared too: a Bézier can loop on itself.
            for second in first..count {
                let (Some(a), Some(b)) = (spans.get(first), spans.get(second)) else {
                    continue;
                };
                if spans_cross(a, b, first == second, neighbours(first, second, count)) {
                    found.push(Crossing { first, second });
                }
            }
        }
        found
    }
}

// How two curves of a boundary legitimately touch. Two curves making a whole boundary do both.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Neighbour {
    // The second curve starts where the first ends.
    next: bool,
    // The first curve starts where the second ends.
    wrapped: bool,
}

const fn neighbours(first: usize, second: usize, count: usize) -> Neighbour {
    Neighbour {
        next: first + 1 == second,
        wrapped: first == 0 && second + 1 == count && count > 1,
    }
}

fn spans_cross(a: &[Point], b: &[Point], same: bool, neighbour: Neighbour) -> bool {
    for (i, pair_a) in a.windows(2).enumerate() {
        let [a0, a1] = pair_a else { continue };
        for (j, pair_b) in b.windows(2).enumerate() {
            let [b0, b1] = pair_b else { continue };
            if same && j <= i {
                continue;
            }
            let crossing = segment_crossing(*a0, *a1, *b0, *b1);
            match crossing {
                SegmentCrossing::Apart => {}
                SegmentCrossing::Overlap => return true,
                SegmentCrossing::At { first, second } => {
                    // The one point two adjacent segments must share is not a crossing.
                    let shared_end =
                        (same && j == i + 1 && first.top() == first.bottom() && second.top() == 0)
                            || (neighbour.next
                                && i + 2 == a.len()
                                && j == 0
                                && first.top() == first.bottom()
                                && second.top() == 0)
                            || (neighbour.wrapped
                                && i == 0
                                && j + 2 == b.len()
                                && first.top() == 0
                                && second.top() == second.bottom());
                    if !shared_end {
                        return true;
                    }
                }
            }
        }
    }
    false
}
