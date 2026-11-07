# DXF output

A factory's cutter reads DXF. The `write_dxf` function writes the one target G2 names, `dxf-aama-named`: AAMA named layers on release R12, polylines only, one size.
[The interchange chapter](interchange-dialects.md) says why interchange is a closed list of targets rather than one format with options.

**What works today.** The writer, its record, and a reader in the tests. **What does not exist yet:** the other targets and grading. The reference skirt's file is frozen, byte for byte, at
`conformance/goldens/reference-skirt/`. **Nobody has opened these files in a CAD system.** They have been read only by a reader written in this repository, and no third-party DXF library has
read them. Whether a real receiver accepts them is G6's question, and until then no claim of acceptance is made.

```rust
use sc_artifacts::{write_dxf, DrawnPiece, Flattening, Line, LineSemantics, Pattern};
use sc_geometry::Point;

let p = |x, y| Point::new(x, y).expect("in range");
let pattern = Pattern {
    seams: vec![],
    repairs: vec![], fits: vec![],
        style: "skirt".to_owned(),
    design: "01ARZ3NDEKTSV4RRFFQ69G5FAV".to_owned(),
    revision: "1".to_owned(),
    pieces: vec![DrawnPiece {
        name: "front".to_owned(),
        size: "M".to_owned(),
        quantity: 2,
        cut: vec![p(-1_000, -1_000), p(11_000, -1_000), p(11_000, 11_000), p(-1_000, 11_000)],
        sew: vec![p(0, 0), p(10_000, 0), p(10_000, 10_000), p(0, 10_000)],
        grain: Line { from: p(5_000, 1_000), to: p(5_000, 9_000) },
        internal: vec![],
        marks: vec![],
        closures: vec![],
        grade_points: vec![],
        labels: vec![],
    }],
};
let flat = Flattening { bound: 100, achieved: 40 };
let artifact = write_dxf(&pattern, LineSemantics::CutAsCut, flat).expect("writes");
assert!(artifact.text.contains("AC1009"));
assert!(artifact.text.contains("\n  2\nfront_M\n"), "a block named [piece]_[size]");
assert!(artifact.text.contains("\n 10\n-1.000\n"), "millimetres to three places, so every micrometre is exact");
assert!(artifact.text.ends_with("  0\nEOF\n"));
assert!(artifact.record.contains("\"target\": \"dxf-aama-named\""));
```

## What goes where

| Layer | Carries |
| --- | --- |
| `CUT` | The cut boundary, as a closed polyline |
| `DRAW` | The sew line, as a closed polyline, and the directed grainline |
| `INTCUT` | Internal lines such as dart legs and folds |
| `NOTCH` | V and slit notches |
| `DRILL` | Drill holes, as closed polylines of two half circles, so a circle is carried exactly |
| `TEXT` | Label lines, one entity each, including the piece's name, size, quantity and style |

Each piece is one `BLOCK` named `[piece]_[size]`, with no nested `INSERT`. Only `POLYLINE`, `VERTEX`, `SEQEND`, `LINE`, `TEXT`, `BLOCK` and `INSERT` appear. A layer that nothing uses is left out rather than written empty.

The file holds millimetres to three places, so each value reads back to the exact micrometre. DXF has no unit of its own, so the unit is in a comment at the top and in the record.

## Which line is on the CUT layer

`LineSemantics` chooses, with no default, because a receiver reading the wrong line is a top cause of rejection. `SewAsCut` puts the sew line on `CUT` and the cut boundary on `DRAW`, for receivers that expect that. Only the layers change: each line stays what it is.

## The record

Every file comes with a record, as canonical text: the target, all six axes, the unit, the flattening bound and what was reached, the writer, the Design and its revision, and the approval state. Without it, a dispute over a file is about memory.

It also states what these layers lose. Seven named layers cover what seventeen numbered ones separate, so `CUT` cannot tell turn points from curve points, and `DRAW` holds both the sew line and the grainline. A receiver should learn that here and not by finding out.

The file and the record hold no date, machine name or time.

## When it refuses

| Refusal | Meaning |
| --- | --- |
| `BadBlockName` | A piece's name or size holds a character a block name cannot |
| `DuplicateBlock` | Two pieces would share one block name |
| `NotClosed` | A closed line has fewer than three points |
| `TessellationBound` | The curves were flattened past the target's bound, so shipping them would ship coarser geometry than it declares |
| `BadLabel` | A label holds a line break or a control character |

## Limits

- **Arcs are not carried yet.** The interchange chapter says an arc travels as a bulge. A drawn piece holds polylines, so the first arc in a real piece has to come with that change.
- **Bézier curves are flattened by the time they reach this.** The record publishes the bound and what was reached, and the caller supplies both.

## How it is checked

Run `cargo test -p sc-artifacts --test dxf`. A reader written in the test, sharing no code with the writer, requires the blocks, the layers and every coordinate to read back exactly, nothing outside the written set, and a swap of line semantics to move only those two lines.

## Names used in this chapter

| Name | Meaning |
| --- | --- |
| `write_dxf` | Writes a pattern as this target, with its record |
| `dxf-aama-named` | The target's name in the registry |
| `CUT` | The layer holding the cut boundary |
| `DRAW` | The layer holding the sew line and grainline |
| `INTCUT` | The layer holding internal lines |
| `NOTCH` | The layer holding notches |
| `DRILL` | The layer holding drill holes |
| `TEXT` | The layer and entity holding label lines |
| `POLYLINE` | A DXF entity: a line through vertices |
| `VERTEX` | A DXF entity: one vertex of a `POLYLINE` |
| `SEQEND` | A DXF entity closing a `POLYLINE` |
| `LINE` | A DXF entity: one straight segment |
| `BLOCK` | A DXF container for one piece |
| `INSERT` | A DXF entity placing a `BLOCK` |
| `LineSemantics` | Which line goes on the `CUT` layer |
| `SewAsCut` | The sew line on `CUT`, and the cut boundary on `DRAW` |
| `BadBlockName` | A refusal: the name cannot name a block |
| `DuplicateBlock` | A refusal: two pieces share one block name |
| `NotClosed` | A refusal: too few points for a closed line |
| `TessellationBound` | A refusal: flattened past the bound |
| `BadLabel` | A refusal: a label is not one plain line |
