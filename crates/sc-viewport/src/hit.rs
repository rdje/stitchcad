//! Picking and snapping, decided by exact integer comparison so that a near tie cannot go the wrong way.
use crate::{corpus::Piece, Point};

/// One edge: the piece, and the edge's position in that piece's closed outline.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EdgeRef {
    /// The piece's position.
    pub piece: usize,
    /// The edge's position: it runs from this vertex to the next.
    pub edge: usize,
}

/// The nearest edge to a point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Hit {
    /// Which edge.
    pub edge: EdgeRef,
    // The squared distance, as a fraction: exact, so two edges compare without rounding.
    distance_top: i128,
    distance_bottom: i128,
}

impl Hit {
    /// Whether the point lies within `reach` micrometres of the edge.
    #[must_use]
    pub const fn within(&self, reach: i64) -> bool {
        let reach = reach as i128;
        self.distance_top <= reach * reach * self.distance_bottom
    }
}

// The squared distance from `p` to the segment `a`–`b`, as an exact fraction.
fn squared_distance(p: Point, a: Point, b: Point) -> (i128, i128) {
    let (px, py) = (i128::from(p.x), i128::from(p.y));
    let (ax, ay) = (i128::from(a.x), i128::from(a.y));
    let (dx, dy) = (i128::from(b.x) - ax, i128::from(b.y) - ay);
    let (rx, ry) = (px - ax, py - ay);
    let length = dx * dx + dy * dy;
    let along = rx * dx + ry * dy;
    if length == 0 || along <= 0 {
        return (rx * rx + ry * ry, 1);
    }
    if along >= length {
        let (sx, sy) = (px - i128::from(b.x), py - i128::from(b.y));
        return (sx * sx + sy * sy, 1);
    }
    // Inside the segment: the perpendicular distance squared is cross² / length.
    let cross = rx * dy - ry * dx;
    (cross * cross, length)
}

/// The edge nearest a point, across all pieces, with ties going to the earlier piece and edge.
#[must_use]
pub fn nearest_edge(pieces: &[Piece], point: Point) -> Option<Hit> {
    let mut best: Option<Hit> = None;
    for (piece_at, piece) in pieces.iter().enumerate() {
        let count = piece.outline.len();
        for edge in 0..count {
            let (Some(a), Some(b)) = (
                piece.outline.get(edge),
                piece.outline.get((edge + 1) % count),
            ) else {
                continue;
            };
            let (top, bottom) = squared_distance(point, *a, *b);
            let closer = best.as_ref().is_none_or(|current| {
                top * current.distance_bottom < current.distance_top * bottom
            });
            if closer {
                best = Some(Hit {
                    edge: EdgeRef {
                        piece: piece_at,
                        edge,
                    },
                    distance_top: top,
                    distance_bottom: bottom,
                });
            }
        }
    }
    best
}

/// The edge under a point, if one lies within `reach` micrometres.
#[must_use]
pub fn pick(pieces: &[Piece], point: Point, reach: i64) -> Option<EdgeRef> {
    nearest_edge(pieces, point)
        .filter(|hit| hit.within(reach))
        .map(|hit| hit.edge)
}

/// The vertex nearest a point, across all pieces, with ties going to the earlier piece and vertex.
#[must_use]
pub fn snap(pieces: &[Piece], point: Point) -> Option<EdgeRef> {
    let mut best: Option<(i128, EdgeRef)> = None;
    for (piece_at, piece) in pieces.iter().enumerate() {
        for (vertex, at) in piece.outline.iter().enumerate() {
            let (dx, dy) = (
                i128::from(at.x) - i128::from(point.x),
                i128::from(at.y) - i128::from(point.y),
            );
            let distance = dx * dx + dy * dy;
            if best.is_none_or(|(current, _)| distance < current) {
                best = Some((
                    distance,
                    EdgeRef {
                        piece: piece_at,
                        edge: vertex,
                    },
                ));
            }
        }
    }
    best.map(|(_, found)| found)
}
