//! The synthetic corpus ADR-0002 declared before anybody measured anything. G1 has no geometry kernel, so this stands in
//! for a garment set, and the same seed always gives the same corpus.
use crate::Point;

/// Pieces, the bound of the largest garment set the envelope names.
pub const PIECES: usize = 16;
/// Boundary vertices per piece.
pub const VERTICES_PER_PIECE: usize = 400;
/// Labels, notches, a grainline and internal lines per piece: 4 + 6 + 1 + 3.
pub const ANNOTATIONS_PER_PIECE: usize = 14;
/// Pick probes.
pub const PICK_PROBES: usize = 200;
/// Snap probes.
pub const SNAP_PROBES: usize = 200;

/// One pattern piece: a closed outline and its marks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    /// The closed boundary.
    pub outline: Vec<Point>,
    /// Annotation positions.
    pub annotations: Vec<Point>,
}

/// A point to test, with the entity a correct answer must name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Probe {
    /// Where to ask.
    pub at: Point,
    /// The piece a correct answer names.
    pub piece: usize,
    /// The edge or vertex a correct answer names.
    pub entity: usize,
}

/// The whole corpus.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Corpus {
    /// The pieces.
    pub pieces: Vec<Piece>,
    /// Points on known edges.
    pub picks: Vec<Probe>,
    /// Points beside known vertices.
    pub snaps: Vec<Probe>,
}

/// SplitMix64, so the corpus depends on nothing but its seed.
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

// A wavy closed shape around (centre_x, centre_y), so edges have many directions. Integer arithmetic only: a table of
// sine values at 1/1000 scale, so the corpus is the same on every platform.
const SINE: [i64; 16] = [
    0, 383, 707, 924, 1000, 924, 707, 383, 0, -383, -707, -924, -1000, -924, -707, -383,
];

fn wave(angle: usize) -> i64 {
    SINE.get(angle % 16).copied().unwrap_or(0)
}

fn outline(random: &mut Random, centre_x: i64, centre_y: i64) -> Vec<Point> {
    let radius = 100_000 + i64::try_from(random.below(40_000)).unwrap_or(0);
    let wobble = 5_000 + i64::try_from(random.below(10_000)).unwrap_or(0);
    let count = i64::try_from(VERTICES_PER_PIECE).unwrap_or(400);
    (0..VERTICES_PER_PIECE)
        .map(|at| {
            // Walk a square, with a ripple, so a piece has long sides and fine detail.
            let at_i = i64::try_from(at).unwrap_or(0);
            let side = at_i * 4 / count;
            let along = (at_i * 4 % count) * 2 * radius / count - radius;
            let ripple = wave(at % 16) * wobble / 1000;
            let (x, y) = match side {
                0 => (along, -radius + ripple),
                1 => (radius + ripple, along),
                2 => (-along, radius + ripple),
                _ => (-radius + ripple, -along),
            };
            Point {
                x: centre_x + x,
                y: centre_y + y,
            }
        })
        .collect()
}

/// The declared corpus for a seed.
#[must_use]
pub fn corpus(seed: u64) -> Corpus {
    let mut random = Random(seed);
    let spacing = 400_000;
    let mut pieces = Vec::with_capacity(PIECES);
    for at in 0..PIECES {
        let (column, row) = (
            i64::try_from(at % 4).unwrap_or(0),
            i64::try_from(at / 4).unwrap_or(0),
        );
        let shape = outline(
            &mut random,
            200_000 + column * spacing,
            200_000 + row * spacing,
        );
        let annotations = (0..ANNOTATIONS_PER_PIECE)
            .filter_map(|mark| {
                shape
                    .get(mark * VERTICES_PER_PIECE / ANNOTATIONS_PER_PIECE)
                    .copied()
            })
            .collect();
        pieces.push(Piece {
            outline: shape,
            annotations,
        });
    }
    let mut picks = Vec::with_capacity(PICK_PROBES);
    let mut snaps = Vec::with_capacity(SNAP_PROBES);
    for _ in 0..PICK_PROBES {
        let piece = usize::try_from(random.below(16)).unwrap_or(0);
        let entity = usize::try_from(random.below(400)).unwrap_or(0);
        let shape = pieces
            .get(piece)
            .map_or(&[][..], |found| found.outline.as_slice());
        if let (Some(a), Some(b)) = (
            shape.get(entity),
            shape.get((entity + 1) % VERTICES_PER_PIECE),
        ) {
            // The edge's midpoint is on that edge and on no other, in this corpus.
            picks.push(Probe {
                at: Point {
                    x: (a.x + b.x) / 2,
                    y: (a.y + b.y) / 2,
                },
                piece,
                entity,
            });
        }
    }
    for _ in 0..SNAP_PROBES {
        let piece = usize::try_from(random.below(16)).unwrap_or(0);
        let entity = usize::try_from(random.below(400)).unwrap_or(0);
        let shape = pieces
            .get(piece)
            .map_or(&[][..], |found| found.outline.as_slice());
        if let Some(at) = shape.get(entity) {
            // A small offset, far smaller than any edge.
            let (dx, dy) = (
                i64::try_from(random.below(5)).unwrap_or(0),
                i64::try_from(random.below(5)).unwrap_or(0),
            );
            snaps.push(Probe {
                at: Point {
                    x: at.x + dx - 2,
                    y: at.y + dy - 2,
                },
                piece,
                entity,
            });
        }
    }
    Corpus {
        pieces,
        picks,
        snaps,
    }
}
