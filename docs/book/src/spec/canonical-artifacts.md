# Canonical artifacts

Before any factory file exists, a drawn pattern has one written form. The same pattern always gives the same bytes, on any machine, because
every file made from it inherits that. The `sc-artifacts` crate holds this form, and its goldens compare meaning first and bytes second, so a
changed notch cannot hide and a reformatted file cannot alarm. It is the canonicalizer the roadmap asks for before any writer.

**What works today.** The written form, reading it back, and comparing two patterns. [The command line](command-line.md) drafts real pieces into it, and [the DXF](dxf-output.md) and [PDF](pdf-output.md) writers read it.
A project does not hold this form: it holds [how to draw the pattern](project-store.md), because the drawing is derived.

```rust
use sc_artifacts::{compare, to_canonical_text, Comparison, DrawnPiece, Line, Pattern};
use sc_geometry::Point;

let p = |x, y| Point::new(x, y).expect("in range");
let square = |size: i64| vec![p(0, 0), p(size, 0), p(size, size), p(0, size)];
let piece = |name: &str, cut: Vec<_>| DrawnPiece {
    name: name.to_owned(),
    size: "M".to_owned(),
    quantity: 1,
    sew: square(90_000),
    cut,
    grain: Line { from: p(45_000, 10_000), to: p(45_000, 80_000) },
    internal: vec![],
    marks: vec![],
    closures: vec![],
    grade_points: vec![],
    labels: vec![],
};
let pattern = |pieces| Pattern { seams: vec![], repairs: vec![], fits: vec![], pockets: vec![], buttons: vec![],
        style: "skirt".to_owned(), design: "01ARZ3NDEKTSV4RRFFQ69G5FAV".to_owned(), revision: "1".to_owned(), pieces };

// Authoring order does not change the form.
let (front, back) = (piece("front", square(100_000)), piece("back", square(100_000)));
let first = to_canonical_text(&pattern(vec![front.clone(), back.clone()]));
let second = to_canonical_text(&pattern(vec![back, front.clone()]));
assert_eq!(first, second);
assert!(!first.contains("2026"), "no date, because a pattern has none");

// A point moved 2 micrometres is the same pattern at a tolerance of 10, and a different one at a tolerance of 1.
let mut moved = square(100_000);
moved[2] = p(100_002, 100_000);
let nudged = to_canonical_text(&pattern(vec![piece("front", moved)]));
let original = to_canonical_text(&pattern(vec![front]));
assert_eq!(compare(&original, &nudged, 10).expect("both read"), Comparison::WithinTolerance { largest_move: 2 });
let Comparison::Different(found) = compare(&original, &nudged, 1).expect("both read") else {
    panic!("a real difference");
};
assert_eq!((found[0].piece.as_str(), found[0].field), ("front", "cut"));
```

## The form

The form is [canonical text](project-store.md#canonical-text), with whole micrometres and no floating point.

- Pieces go in name then size order, whatever order they were drawn in.
- A closed line starts at its lowest point, then its leftmost, whatever point it was drawn from.
- Internal lines, marks and labels go in a fixed order.
- There is no timestamp, no tool version and no machine name. Those belong to the export's record, which [the interchange chapter](interchange-dialects.md) describes.

## What a comparison says

| Result | Meaning |
| --- | --- |
| `Identical` | The same bytes |
| `FormattingOnly` | Different bytes and the same meaning: whitespace or key order differs |
| `WithinTolerance` | Points moved, by no more than the tolerance, and it says how far |
| `Different` | Something means something else, with the piece and field named |

A golden check should accept the first two, and it decides by the tolerance class of what it compares. [The units chapter](units-and-tolerances.md) sets those.

## How it is checked

Run `cargo test -p sc-artifacts`. It shuffles pieces, marks and starting points and requires one form, reads every form back to an equal pattern, and changes one
field at a time, requiring that field and no other to be reported. It also compares the form byte for byte with a frozen file, `conformance/goldens/canonical-two-pieces.json`,
which hosted CI checks on Linux at each push. That Linux run has not happened yet, so two platforms are not shown until it does.

## Names used in this chapter

| Name | Meaning |
| --- | --- |
| `sc-artifacts` | The crate |
