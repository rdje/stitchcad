# Geometry kernel

Every pattern piece is built from three kinds of curve: straight lines, circular arcs and cubic Béziers, the ones
factory file formats carry. The `sc-geometry` crate holds them in whole micrometres and answers exactly which way a
point lies from a line, where two segments cross, and whether a boundary closes and crosses itself.

**What works today.** Curves, exact predicates, flattening to a polyline within a stated tolerance, lengths, boundaries with winding
and self-crossings, and [seam allowances](offsets.md). **What does not exist yet:** anything drawing a piece from a recipe, and NURBS,
which the roadmap defers and this refuses by name.

## Why whole numbers

Floating-point arithmetic rounds, and near a tie the rounding can flip the answer. On a long edge, far from the origin, a
naive check can say a point lies on the line when it does not, and a boundary then quietly fails to close or crosses itself.
Whole micrometres avoid this: every comparison here is exact integer arithmetic, so a near tie cannot flip. Coordinates may
reach 2^60 micrometres, well over a thousand kilometres, which keeps every product inside 128 bits.

```rust
use sc_geometry::{orientation, segment_crossing, Orientation, Point, SegmentCrossing};

let p = |x, y| Point::new(x, y).expect("in range");

// One micrometre to the left of a line about nine metres long, far from the origin.
assert_eq!(orientation(p(0, 0), p(9_000_000, 0), p(4_500_000, 1)), Orientation::Left);
assert_eq!(orientation(p(0, 0), p(9_000_000, 0), p(4_500_000, 0)), Orientation::Collinear);

// A crossing that is not on a whole micrometre stays an exact fraction: 4/9 of the way along the first segment.
let SegmentCrossing::At { first, .. } = segment_crossing(p(0, 0), p(3, 0), p(1, -1), p(2, 2)) else {
    panic!("these cross");
};
assert_eq!((first.top(), first.bottom()), (4, 9));
```

## Curves

`Curve` is a line, an arc through three points, or a cubic Bézier. Its ends are always exact. `flatten` turns it into a
polyline that stays within a tolerance you state: the default is 100 micrometres, the fixture's bound for formats that only
hold polylines. Each interior point rounds to a whole micrometre, so a point can stray one micrometre past the tolerance.

```rust
use sc_geometry::{Curve, DEFAULT_TOLERANCE, Point};

let curve = Curve::from_kind("cubic", &[(0, 0), (0, 500_000), (500_000, 500_000), (500_000, 0)]).expect("a cubic");
let points = curve.flatten(DEFAULT_TOLERANCE);
assert_eq!(points.first(), Some(&Point::new(0, 0).expect("in range")));
assert_eq!(points.last(), Some(&Point::new(500_000, 0).expect("in range")));
assert!(points.len() > 8);

// Only the declared curves exist. NURBS is refused by name rather than approximated.
let error = Curve::from_kind("nurbs", &[]).expect_err("refused");
assert_eq!(error.to_string(), "a nurbs curve is not supported: curves here are lines, circular arcs and cubic Béziers");
```

Flattening a Bézier compares its control points with the chord, including past the chord's ends, where a control point can
overshoot. Arcs use floating point to find their circle and to place interior points, and the result is rounded to the
micrometre grid, so exact predicates still apply to what comes out. The curve's own ends are never rounded.

## Boundaries

An `Outline` is a closed chain of curves. Each curve must start exactly where the last one ended, with no tolerance, because a
small gap is exactly the fault that makes a cutter stop. The roadmap's outer boundaries run counterclockwise.

```rust
use sc_geometry::{Curve, Outline, Point, Winding};

let p = |x, y| Point::new(x, y).expect("in range");
let line = |a, b| Curve::Line { from: a, to: b };
let square = Outline::new(vec![
    line(p(0, 0), p(10, 0)), line(p(10, 0), p(10, 10)), line(p(10, 10), p(0, 10)), line(p(0, 10), p(0, 0)),
]).expect("closed");
assert_eq!(square.winding(1), Winding::Counterclockwise);
assert_eq!(square.doubled_area(1), 200);
assert!(square.self_crossings(1).is_empty());

// A gap is refused, naming where.
let open = Outline::new(vec![line(p(0, 0), p(10, 0)), line(p(10, 1), p(0, 0))]).expect_err("a gap");
assert_eq!(open.to_string(), "curve 1 starts at (10, 1), and the curve before it ended at (10, 0)");

// A figure of eight is found.
let bowtie = Outline::new(vec![
    line(p(0, 0), p(10, 10)), line(p(10, 10), p(10, 0)), line(p(10, 0), p(0, 10)), line(p(0, 10), p(0, 0)),
]).expect("closed");
let crossing = bowtie.self_crossings(1);
assert_eq!((crossing[0].first, crossing[0].second), (0, 2));
```

## How it is checked

Run `cargo test -p sc-geometry`. Orientation is compared with arbitrary-precision integers, which share no arithmetic with the
kernel, on 50,000 seeded triples including near ties and coordinates at the range limit, and a test requires at least one
case naive floating point gets wrong. Flattened curves are sampled densely against their true shape. The reference skirt's front
quadrant closes, runs counterclockwise, has no crossing, and its side seam measures the book's 42.107 cm.

`self_crossings` compares every pair of polyline segments, so its work grows with the square of their number. That is fine for a
pattern piece, and a large file will want something better.

## Names used in this chapter

| Name | Meaning |
| --- | --- |
| `sc-geometry` | The crate |
| `Curve` | A line, an arc or a cubic Bézier |
| `Outline` | A closed chain of curves |
| `flatten` | Turns a curve into a polyline within a stated tolerance |
| `self_crossings` | The pairs of curves in a boundary that cross or overlap |
