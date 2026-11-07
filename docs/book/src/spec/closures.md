# Closures

A dart is not two lines. It takes up a stated amount of cloth, and that amount has to survive being closed. So a dart, tuck, pleat or gather is an object on a piece, carrying what it takes up, and a piece's closures are what a seam's length accounts for.

**What works today.** All four, measured from the drawn line. Closing a dart. Slash and spread as a recipe operation. [The reference skirt's](reference-skirt.md) darts are these objects. **What does not exist yet:** truing the waist across a closed dart, and a garment using a tuck, pleat or gather. [Truing a seam](walking-and-truing.md) works, and a seam sees a closure as sewn away.

**Two models exist for now.** This is executed in the drafting layer that [the session](command-line.md) drives. The core's own `Dart`, `Tuck`, `Pleat` and `Gather` describe the same things structurally and hold no coordinates, so `CloseDart` on the bus still answers that it has no handler. A later leaf has to bind them, and until then a closure here is not one of those.

```rust
use sc_cli::replay;
use sc_store::{parse, FilePath, MemoryRepository, Repository};

// A panel 40 cm wide, with a dart at the top taking 4 cm.
let session = parse(r#"{"format":1,"design":"01ARZ3NDEKTSV4RRFFQ69G5FAV",
  "inputs":[{"token":"width","um":400000},{"token":"height","um":600000}],
  "recipe":"let half: length = width / 2\nlet near_x: length = half - 20 mm\nlet far_x: length = half + 20 mm\nlet apex_y: length = height - 120 mm\nlet intake: length = 40 mm\n",
  "steps":[{"do":"draft","style":"panel","size":"M","pieces":[{"name":"panel","quantity":1,"start":["zero","zero"],
    "edges":[{"to":["width","zero"],"width":"zero","corner":"miter"},{"to":["width","height"],"width":"zero","corner":"miter"},
             {"to":["zero","height"],"width":"zero","corner":"miter"},{"to":["zero","zero"],"width":"zero","corner":"miter"}],
    "grain":[[["half","zero"],["half","height"]]],
    "darts":[{"name":"top","near":["near_x","height"],"far":["far_x","height"],"apex":["half","apex_y"],"intake":"intake"}]}]},
    {"do":"closures"}]}"#).expect("valid");

let transcript = replay(&session, MemoryRepository::new()).expect("it draws");
let report = &transcript.as_list().expect("a list")[1].as_object().expect("a step")["result"].as_list().expect("reports")[0];
let report = report.as_object().expect("a report");
assert_eq!(report["intake"].as_int(), Some(40_000));
assert_eq!(report["open_length"].as_int(), Some(2_000_000));
assert_eq!(report["closed_length"].as_int(), Some(1_960_000));
assert_eq!(report["gap_after_closing"].as_int(), Some(0));
```

## What each one is

| Kind | Fields | What it takes up |
| --- | --- | --- |
| Dart | `near`, `far` and `apex`, each a position | The sew line between its legs' tops |
| Tuck | `from` and `to`, with optional fold lines to `from_end` and `to_end` | The sew line between its folds |
| Pleat | The same as a tuck | The sew line between its folds |
| Gather | `from`, `to` and `gathered_to`, a named length | The range less what it is drawn up to |

**The intake is measured, not believed.** It comes from the drawn line. A dart or fold may also declare an `intake`, and a declaration that disagrees by more than T1 is refused by name, because a number nobody can falsify is decoration. An end that is not on the sew line, an apex outside the piece, or a gather that would lengthen its range is refused the same way.

## Closing a dart

`closures` reports, for each one, the intake and the sew line's length open and closed. A dart is also really closed: its far leg is turned about the apex onto its near one, and what is left between their tops is `gap_after_closing`. `conserved` says that what closing removes equals the intake within T1, and that the gap is within T1 too. Unequal legs show here, as a gap.

It reports and does not change the piece, because the cut pattern is drawn open.

| Report field | What it holds |
| --- | --- |
| `intake` | What the closure carries |
| `open_length` | The sew line's length with it open |
| `closed_length` | The sew line's length with it closed |
| `gap_after_closing` | What is left between a dart's leg tops once it is turned shut |
| `conserved` | Whether closing removed exactly the intake |

## Slash and spread

A slash runs from a `pivot` on the boundary to a point `slash_to` on the boundary. The part of the boundary from `slash_to` round to the pivot turns about the pivot until the slash has opened by `spread`, and the opening becomes a new edge with the allowance of the edge it was cut from. Notches and internal lines on the turned part move with it, and one on the slash itself stays.

**It is an operation and not an edit.** It is written in the piece's description and done again at every draft, so a changed measurement moves it and the same inputs give the same bytes. Each sense of the turn is tried, and the one that adds cloth is kept. Nothing about it is stored as geometry. A slash that cannot open, or whose ends are not on a corner or a straight edge, is refused by name. A slash ending on a curved edge is refused rather than split where nobody asked.

```text
$ cargo test -p sc-cli --test closures
```

**What this does not show.** That these shapes sew well: nobody who sews has reviewed them. Truing across a slash: slash and spread leaves the waist with a corner at the pivot, and a patternmaker would true it. That a tuck's or pleat's fold direction means anything yet, because nothing reads it.

## Names used in this chapter

| Name | Meaning |
| --- | --- |
| `Dart` | The core's structural dart |
| `Tuck` | The core's structural tuck |
| `Pleat` | The core's structural pleat |
| `Gather` | The core's structural gather |
| `CloseDart` | The bus's command for closing a dart, which has no handler yet |
| `near` | A dart's near leg's top |
| `far` | A dart's far leg's top |
| `apex` | A dart's apex |
| `from` | Where a range starts |
| `to` | Where a range ends |
| `from_end` | Where a tuck's first fold line ends |
| `to_end` | Where a tuck's second fold line ends |
| `gathered_to` | The length a gather draws its range up to |
| `closures` | The session step that reports these |
| `pivot` | The point a slash turns about |
| `slash_to` | Where a slash meets the boundary |
| `spread` | How far the slash opens there |
