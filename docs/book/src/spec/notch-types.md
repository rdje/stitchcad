# Notch types

A notch means something about the cloth: match this to that, or stop sewing here. How a file carries that is a separate question with a different answer for each receiver. So a notch here has a type, which is what it means, and an encoding, which is how a file says it.

**What works today.** `single`, `double` and `drill`, in both dialects, with each representation frozen. A notch drawn, or coded as a point, in a numbered file. **What does not exist yet:** the types the roadmap stages to G4, which are V, I, T, U and castle, and asking for one says `env_notch_type`. A notch's width. A factory profile to choose the encoding, so a caller names it, and with none named it is drawn.

```rust
use sc_artifacts::{
    write_embedded_rules, DrawnPiece, Flattening, Line, Mark, MarkKind, NotchEncoding, Pattern,
};
use sc_geometry::Point;

let at = |x, y| Point::new(x, y).expect("in range");
let pattern = Pattern {
    style: "notches".to_owned(),
    design: "01ARZ3NDEKTSV4RRFFQ69G5FAV".to_owned(),
    revision: "1".to_owned(),
    seams: vec![],
    repairs: vec![],
    fits: vec![],
    pockets: vec![],
    buttons: vec![],
    pieces: vec![DrawnPiece {
        name: "square".to_owned(),
        size: "M".to_owned(),
        quantity: 1,
        cut: vec![at(0, 0), at(100_000, 0), at(100_000, 100_000), at(0, 100_000)],
        sew: vec![at(10_000, 10_000), at(90_000, 10_000), at(90_000, 90_000), at(10_000, 90_000)],
        grain: Line { from: at(20_000, 20_000), to: at(20_000, 80_000) },
        internal: vec![],
        // A notch on the bottom edge, 5 mm deep and pointing up.
        marks: vec![Mark { kind: MarkKind::VNotch, at: at(50_000, 0), toward: at(50_000, 5_000) }],
        labels: vec![],
        closures: vec![],
        grade_points: vec![],
    }],
};
let flat = Flattening { bound: 100, achieved: 11 };

// Drawn, it is a line on the internal layer, which anything can read.
let drawn = write_embedded_rules(&pattern, &[], &[], ("M", "n"), flat, NotchEncoding::Drawn).expect("writes");
assert!(drawn.text.contains("LINE\n  8\n8\n"));

// Coded, it is a point on the notch layer: where it is, its depth as Z, and its angle.
let coded = write_embedded_rules(&pattern, &[], &[], ("M", "n"), flat, NotchEncoding::Coded).expect("writes");
assert!(coded.text.contains("POINT\n  8\n4\n 10\n50.00\n 20\n0.00\n 30\n5.00\n 50\n90.000\n"));
assert!(coded.record.contains("\"notch_encoding\": \"coded\""));
```

## What each type means

| Type | Means | In a [draft](stable-references.md) |
| --- | --- | --- |
| `single` | Match here. The front of a sleeve's cap, a hip | One mark at the place it holds |
| `double` | Match here, and it is not the single. The back of a sleeve's cap, so that it cannot go in the wrong way round | Two marks either side of the place it holds, `spacing` apart |
| `drill` | A place inside the piece. A bust point, a button | A hole, `inset` from an edge |

## How each one travels

| Type | `dxf-aama-named` | `dxf-astm-num-r13`, drawn | `dxf-astm-num-r13`, coded |
| --- | --- | --- | --- |
| `single` | A `LINE` on `NOTCH` | A `LINE` on layer 8 | A `POINT` on layer 4 |
| `double` | Two of those | Two of those | Two of those |
| `drill` | Two half circles on `DRILL` | A `POINT` on layer 13 | A `POINT` on layer 13 |

A coded notch's `POINT` is on the boundary, with its depth as group code 30 and its angle as group code 50, counter-clockwise from X and pointing into the piece. It has no width, so it reads as a slit: a V needs a width, and that is a profile's to give.

**The default is drawn,** and that is a decision with a reason. The guide this was read from says that when the import system is unknown, drawn geometry "is a good alternative" to the coded form, and no import system here is known. So `notch_encoding` on [`export_graded`](grading.md) takes `drawn` or `coded`, with `drawn` when nothing is named, and a Factory Profile is what should choose it once one exists. A named file has no notch layer in that sense, so it takes only `drawn`. The record says which was used. The layer table in [the dialects chapter](interchange-dialects.md) is the map, and three small files in `conformance/goldens/notches/` hold each of these exactly.

```text
$ cargo test -p sc-artifacts --test notch_types
```

**What this does not show.** That any receiver reads a coded point as intended, or that drawn lines on the internal layer are not cut by one. Only the first pages of the standard and a third party's guide to it were read. So group codes 30 and 50 come from that guide. Each mark here is one type at a time, so nothing says what happens where two notches share a place.

## Names used in this chapter

| Name | Meaning |
| --- | --- |
| `single` | One mark |
| `double` | Two marks |
| `drill` | A hole |
| `env_notch_type` | What asking for a later type says |
| `spacing` | How far apart a double's marks are |
| `inset` | How far in from its edge a drill is |
| `dxf-aama-named` | A registered target, with named layers |
| `dxf-astm-num-r13` | A registered target, with numbered layers |
| `LINE` | A straight line in a file |
| `POINT` | A point in a file |
| `NOTCH` | A named layer |
| `DRILL` | A named layer |
| `notch_encoding` | How a file carries a notch |
| `export_graded` | The step that writes a grading mode |
| `drawn` | As a line |
| `coded` | As a point |
| `conformance/goldens/notches/` | Each representation, frozen |
| `cargo test -p sc-artifacts --test notch_types` | What holds this chapter |
