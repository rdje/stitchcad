# Seam allowances and offsets

A pattern piece's net boundary is where the stitching goes. The cut boundary lies outside it by each edge's seam allowance.
Computing that outer boundary is the offset, which the roadmap calls a first-class risk: an exact offset of a Bézier is not a
Bézier, concave corners fold the raw result over itself, and acute corners want to spike. This chapter says what the `offset` method
of [the geometry kernel](geometry.md) does about each, and when it refuses.

**What works today.** Per-edge widths, including 0 for a fold; five corner treatments; loop removal; a clearance check on the result; and
a corpus that holds it to all of this. **What does not exist yet:** drawing a real piece from a recipe, which is `G2-2D.8`; exporting the
result, which is `G2-2D.5` and `.6`; and a review of the corner shapes by a sewing expert.

```rust
use sc_geometry::{Allowance, Corner, Curve, Outline, Point};

let p = |x, y| Point::new(x, y).expect("in range");
let line = |a, b| Curve::Line { from: a, to: b };
let square = Outline::new(vec![
    line(p(0, 0), p(100_000, 0)), line(p(100_000, 0), p(100_000, 100_000)),
    line(p(100_000, 100_000), p(0, 100_000)), line(p(0, 100_000), p(0, 0)),
]).expect("closed");

// 1 cm on every edge, in micrometres, with mitered corners: a square 12 cm across.
let allowance = Allowance { width: 10_000, corner: Corner::Miter };
let offset = square.offset(&[allowance; 4], 10).expect("offsets");
let corners: Vec<(i64, i64)> = offset.points().iter().map(|point| (point.x(), point.y())).collect();
assert_eq!(corners, [(-10_000, -10_000), (110_000, -10_000), (110_000, 110_000), (-10_000, 110_000)]);
assert_eq!(offset.length(), 480_000);
```

## What the result promises

- **It runs counterclockwise** and starts at its lowest point, then its leftmost, so one shape has one written form.
- **It never crosses itself.** The G2 exit asks for zero self-intersecting loops.
- **It clears each edge by that edge's width**, within the tolerance, wherever its nearest place is inside the edge. A corner join may come
  closer to the corner itself, as a slanted cut must. A fold has a width of 0, and the result runs along it.

Everything else is refused, naming what is wrong, so nothing wrong reaches an exporter.

| Refusal | Meaning |
| --- | --- |
| `WrongCount` | A different number of allowances than edges |
| `NegativeWidth` | An allowance adds cloth, so a width cannot be negative |
| `NotCounterclockwise` | The net boundary runs the wrong way, so outward is unknown |
| `NetCrossing` | The net boundary already crosses itself |
| `BoundExceeded` | Part of the result falls short of an edge's width by more than the tolerance, naming the edge |
| `Tangled` | The result still crossed itself after its loops were removed |

## Corner treatments

The ontology names five treatments and does not say what shape each makes. **These definitions are this engine's, written by an engineer, and
no sewing expert has reviewed them.** The domain expert's seat is vacant, as [governance](../governance.md) says, so a patternmaker may well
want them changed. They are one enum and one function.

| Treatment | Shape at a convex corner |
| --- | --- |
| `Miter` | The two offset edges run on until they meet |
| `Slant` | A straight cut between the two offset edges' ends |
| `Trim` | A miter with its tip cut off square, as far from the net corner as the wider allowance, to reduce bulk |
| `Step` | The first offset edge runs on by the second edge's width, then turns square until it meets the second offset edge |
| `Envelope` | The edge naming it is a fold line, and its neighbour's offset mirrors across it, the way a hem folds up over a side seam. Two meeting edges both naming it have no fold between them, so they miter, as a mitred hem corner does |

Where two edges name different treatments, the corner takes the more specific: envelope, then step, then trim, then slant, then miter. So the
fixture's hem, enveloped, governs its corners with the mitered side seams.

At a concave corner, the offsets cross inside and all five mean the same thing: they meet where their lines cross.

**No treatment reaches past the limit.** A join may reach `MITER_LIMIT` allowance widths from its net corner, which is 4. Past that, an acute corner
gets a straight cut instead of a spike.

```rust
use sc_geometry::{Allowance, Corner, Curve, Outline, Point};

let p = |x, y| Point::new(x, y).expect("in range");
let line = |a, b| Curve::Line { from: a, to: b };
// A tip of about 5 degrees, 10 mm of allowance. An unbounded miter would reach out over 11 cm.
let tip = Outline::new(vec![
    line(p(0, 0), p(1_143_005, -50_000)),
    line(p(1_143_005, -50_000), p(1_143_005, 50_000)),
    line(p(1_143_005, 50_000), p(0, 0)),
]).expect("closed");
let offset = tip.offset(&[Allowance { width: 10_000, corner: Corner::Miter }; 3], 10).expect("offsets");
let reach = offset.points().iter().map(|point| point.x()).min().expect("points");
assert!(reach >= -43_000, "reaches {reach}");
```

## Loops, and edges that get covered

A sharp convex turn inside one curve, a true cusp, goes around on an arc of the allowance's radius, as the true offset does. The raw offset of a concave
corner, or of a curve tighter than its allowance, folds back over itself. The engine splits the raw path at every
crossing and walks its outer boundary, starting from a point certainly on the outside and keeping right at each junction. Rounding a crossing to
the micrometre grid can leave a new crossing a micrometre away, so this repeats, and any loop left after that is cut at its crossing. The clearance
check then judges the result.

A slot narrower than two allowances gets filled, and so does a concave curve tighter than one. The true offset does this too, and the piece has the
cloth it needs. But the cut boundary no longer follows that edge, so `swallowed_edges` reports it, and a caller can refuse.

## How it is checked

`conformance/offsets/cases.json` is the pathology corpus the G2 exit names. It holds 37 cases, each stating what it exercises and what a pass means:

| Family | Cases | What it holds the engine to |
| --- | --- | --- |
| `acute` | 20 | Tips of 45, 30, 15 and 5 degrees under each treatment, with no spike |
| `loops` | 4 | Concave corners, a scoop tighter than its allowance, and a slot, with no crossing |
| `seam-length` | 3 | Circles and a rounded rectangle, whose lengths have exact answers, so accumulated error is measured against a stated bound |
| `near-tangent` | 2 | Joins at a sliver of an angle |
| `cusp` | 3 | Sharp points between Béziers, and a true cusp inside one |
| `variable` | 2 | Different widths, and a fold |
| `fixture` | 1 | The reference skirt's front quadrant, with its own allowances |
| `refusal` | 2 | Boundaries that must be refused by name |

```text
$ cargo test -p sc-geometry --test offset_corpus
```

A result is judged by checks that share no code path with the engine: all pairs of segments for crossings, winding numbers for whether it surrounds
the net piece, and an independent distance for clearance. A test hands those checks deliberately broken results, and they catch each one.

`cargo test -p sc-geometry --test offset` adds 4,000 seeded star shapes with acute and concave corners of every size and random widths and treatments.
On those, 3,993 give a valid result and 7 refuse with `BoundExceeded`, because a neighbour's corner cut really does leave an edge short of its width. A wrong result
or a panic fails the test.

`cargo test -p sc-geometry --test metamorphic` moves, turns and mirrors every case the corpus expects to offset, then compares the result with the original result moved the same way:

| Transform | Compared with | Allowed difference, and why |
| --- | --- | --- |
| Mirror twice | The original boundary | None. It is exact, tighter than T1 asks |
| Move | The offset moved | None on straight edges. Twice T2 on curves, because a curve is flattened again, and two approximations each within T2 of one curve can be that far apart |
| Quarter turn | The offset turned | 2 µm on straight edges, and twice T2 on curves |
| Any other turn | The offset turned | Twice T2, plus what rounding the net's own points does at its sharpest corner |
| Mirror | The offset mirrored | As a quarter turn, except for `Step` |

`Step` is directional by its definition: the first edge runs on past the corner, and a mirror reverses which edge is first. So a mirrored piece's step would land on the other side, which is why a mirrored piece mirrors the finished cut line and is not drafted again. T1 and T2 are [the units chapter's classes](units-and-tolerances.md).

This found a real fault. A corner where both edges named `Envelope` folded over whichever came second, so a piece and its mirror got different corners, 4 mm apart at a 45 degree tip. It now miters. Of 36 cases moved a long way, three came back with a chord moved, 11 µm at most, all on curves.

**What this does not cover.** It is measured on polylines within the tolerance, so a chord's error is included in the bound and not separated out. Lengths
have exact answers only for lines and arcs. The treatments' shapes are untested on cloth. Curved results are polylines, so an exporter that wants a Bézier
back has to fit one. Its loop search compares every pair of segments, so its work grows with the square of their number.

## Names used in this chapter

| Name | Meaning |
| --- | --- |
| `offset` | The method that adds a boundary's allowances |
| `swallowed_edges` | Edges whose own line vanished because a neighbour's allowance covers them |
| `MITER_LIMIT` | How many allowance widths a corner join may reach from its net corner: 4 |
| `acute` | Corpus cases with sharp tips |
| `loops` | Corpus cases whose raw offset folds over itself |
| `seam-length` | Corpus cases whose lengths have exact answers |
| `near-tangent` | Corpus cases joining at a sliver of an angle |
| `cusp` | Corpus cases with sharp points between curves |
| `variable` | Corpus cases with differing widths |
| `fixture` | The corpus case built from the reference skirt |
| `refusal` | Corpus cases that must be refused |
| `Step` | The corner treatment that runs the first edge on past the corner |
