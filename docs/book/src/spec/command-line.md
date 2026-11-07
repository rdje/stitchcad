# Command line

The `stitchcad` program runs a recorded session against a project directory: evaluate a recipe, edit a measurement,
undo and redo, solve a constraint, save and reopen. Every edit goes through the command bus, the same path every
other front end will use. The same session always prints the same bytes and saves the same files. It is the
`sc-cli` crate, the `native-headless` front end of roadmap §7.3.

**What works today.** A session supplies named lengths and a scalar recipe. The program builds a real Design from
them, runs the steps below, and prints a transcript. It drafts a real garment: `conformance/reference-skirt/session.json`
is the whole [reference skirt](reference-skirt.md), and it exports the same canonical form, DXF and PDF on every run.
[The dev shell](dev-shell.md) drives these same steps from a page. **What does not exist yet:** any edit but a measurement change; a piece named by anything but evaluated
lengths and curves written in the session, which is G3; `sc-api` and MCP, leaf `.9`.

```text
$ stitchcad replay session.json project-directory
```

## A session

| Field | What it holds |
| --- | --- |
| `format` | The session format, 1 |
| `design` | The Design's identity, as a ULID. Every other identity comes from it and a counter |
| `inputs` | The named lengths: `token`, and `um` in whole micrometres |
| `recipe` | The construction recipe, in [the formula language](formula-language.md) |
| `steps` | What to do, in order. The first failure stops the session |

| Step | What it does |
| --- | --- |
| `evaluate` | Runs the recipe against the current Design and lists every binding |
| `closures` | What closing each [closure](closures.md) of the drawing does |
| `walk` | [Walks each seam](walking-and-truing.md), naming what explains any difference |
| `plant_balance_notch` | Plants a notch on both sides of a seam, as one edit |
| `true_seam` | Makes a seam's sides match within a stated limit, as one edit |
| `edit_edge` | [Edits a named edge](stable-references.md), as one edit |
| `references` | Lists what holds an edge, where it lands now, and any repair tasks |
| `repair_reference` | Says where an orphaned reference goes, as one edit |
| `preview` | What one input's new value would change, computed on a copy: `token` and `um`. The Design, its revision and its history stay as they are |
| `change` | Replaces one input's value through the bus, at a new revision. `token`, `um`, and optionally `expect_revision` |
| `undo` | Returns the Design to its state before the last change, under a fresh authority check |
| `redo` | Reapplies the last change undone |
| `solve` | Checks one constraint: the input named by `token` may be at most `at_most_um`, using [the constraint kernel](constraints.md) |
| `render` | Draws an outline as SVG. `outline` lists points, each naming two lengths: a binding, an input, or `zero` |
| `export` | Adds the last drawing to the project at `path`, and reports its digest. The next `save` writes it |
| `save` | Saves the project as one [crash-consistent commit](project-store.md), keeping files and fields it did not write |
| `reopen` | Rebuilds the Design from the saved project |
| `draft` | Draws the pattern's pieces from the evaluated lengths, below. It changes nothing in the Design, and any later change clears the drawing. With no `pieces`, it draws what the project saved |
| `export_pattern` | Adds the drafted pattern to the project at `path` in `canonical`, `dxf` or `pdf`, with a record beside it, and reports the digest. The next `save` writes them |

## An edit, an undo, and a saved project

```rust
use sc_cli::replay;
use sc_store::{parse, MemoryRepository};

let session = parse(r#"{
  "format": 1,
  "design": "01ARZ3NDEKTSV4RRFFQ69G5FAV",
  "inputs": [{"token": "waist_girth", "um": 740000}, {"token": "ease_waist", "um": 10000}],
  "recipe": "let garment_waist: length = waist_girth + ease_waist\n",
  "steps": [
    {"do": "evaluate"},
    {"do": "change", "token": "waist_girth", "um": 780000},
    {"do": "evaluate"},
    {"do": "undo"},
    {"do": "evaluate"},
    {"do": "save"}
  ]
}"#).expect("valid text");

let transcript = replay(&session, MemoryRepository::new()).expect("the session runs");
let first_step = "[\n  {\n    \"do\": \"evaluate\",\n    \"result\": [\n      {\n        \"kind\": \"length\",\n        \"name\": \"garment_waist\",\n        \"value\": \"750000\"\n      }\n    ],\n    \"revision\": \"0\",\n    \"step\": 0\n  },";
assert!(transcript.to_text().starts_with(first_step));

// The edit moved the value, and undo restored it.
let value = |at: usize| {
    let step = &transcript.as_list().expect("a transcript")[at];
    let binding = &step.as_object().expect("a record")["result"].as_list().expect("bindings")[0];
    binding.as_object().expect("a binding")["value"].as_text().expect("text").to_owned()
};
assert_eq!(value(0), "750000");
assert_eq!(value(2), "790000");
assert_eq!(value(4), "750000");

// The same session is the same bytes.
let again = replay(&session, MemoryRepository::new()).expect("the session runs");
assert_eq!(again.to_text(), transcript.to_text());
```

A binding's `value` is in the internal unit of its kind: micrometres for a length.

## A stale edit is refused

A change can state the revision it was made against. The bus compares that with the Design's revision and refuses
a mismatch, so an edit made against an old state cannot land on a newer one.

```rust
use sc_cli::replay;
use sc_store::{parse, MemoryRepository};

let session = parse(r#"{
  "format": 1,
  "design": "01ARZ3NDEKTSV4RRFFQ69G5FAV",
  "inputs": [{"token": "waist_girth", "um": 740000}],
  "recipe": "",
  "steps": [
    {"do": "change", "token": "waist_girth", "um": 750000},
    {"do": "change", "token": "waist_girth", "um": 760000, "expect_revision": "0"}
  ]
}"#).expect("valid text");

let failure = replay(&session, MemoryRepository::new()).expect_err("revision 0 is stale");
let diagnostic = failure.to_value();
let fields = diagnostic.as_object().expect("a diagnostic");
assert_eq!(fields["failed"].as_text(), Some("refused"));
assert_eq!(fields["step"].as_int(), Some(1));
```

## Reopening keeps the revision count

`save` records the revision and also the highest revision ever issued, because undo can leave the Design below it.
So a reopened Design never reissues a revision it has already used.

```rust
use sc_cli::replay;
use sc_store::{parse, MemoryRepository};

let session = parse(r#"{
  "format": 1,
  "design": "01ARZ3NDEKTSV4RRFFQ69G5FAV",
  "inputs": [{"token": "waist_girth", "um": 740000}],
  "recipe": "",
  "steps": [
    {"do": "change", "token": "waist_girth", "um": 750000},
    {"do": "change", "token": "waist_girth", "um": 760000},
    {"do": "undo"},
    {"do": "save"},
    {"do": "reopen"},
    {"do": "change", "token": "waist_girth", "um": 770000}
  ]
}"#).expect("valid text");

let transcript = replay(&session, MemoryRepository::new()).expect("the session runs");
let revision = |at: usize| {
    let step = &transcript.as_list().expect("a transcript")[at];
    step.as_object().expect("a record")["revision"].as_text().expect("text").to_owned()
};
assert_eq!(revision(1), "2");
assert_eq!(revision(2), "1"); // undo moved back to 1
assert_eq!(revision(4), "1"); // the reopened Design is at its saved revision
assert_eq!(revision(5), "3"); // 2 was already issued, so the next edit is 3
```

**Undo does not cross a reopen yet.** A reopened Design starts with empty undo and redo stacks.

## A drawing and a file

`render` draws from the evaluated values, in the page's frame: x to the right and y down, in whole micrometres, so
nothing rounds. The same values always give the same bytes. **This is a schematic, not a pattern.** It joins points with
straight lines, with no dart, curve or allowance.

```rust
use sc_cli::replay;
use sc_store::{parse, FilePath, MemoryRepository, Repository};

let session = parse(r#"{
  "format": 1,
  "design": "01ARZ3NDEKTSV4RRFFQ69G5FAV",
  "inputs": [{"token": "width", "um": 100000}, {"token": "depth", "um": 50000}],
  "recipe": "let half: length = width / 2\n",
  "steps": [
    {"do": "render", "outline": [["zero", "zero"], ["half", "zero"], ["width", "depth"], ["zero", "depth"]]},
    {"do": "export", "path": "exports/outline.svg"},
    {"do": "save"}
  ]
}"#).expect("valid text");

let mut disk = MemoryRepository::new();
replay(&session, &mut disk).expect("the session runs");
let saved = disk.read(&FilePath::new("exports/outline.svg").expect("a path")).expect("reads").expect("exported");
assert_eq!(String::from_utf8_lossy(&saved),
    "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"100.000mm\" height=\"50.000mm\" viewBox=\"0 0 100000 50000\">\n<polygon points=\"0,0 50000,0 100000,50000 0,50000\" fill=\"none\" stroke=\"black\" stroke-width=\"500\"/>\n</svg>\n");
```

## When something goes wrong

The program prints one canonical text object and exits nonzero. `failed` is a stable code, `message` is the reason,
and `step` is the step's position, absent when the session never reached one.

| Code | Meaning |
| --- | --- |
| `usage` | The command line is wrong, or a file could not be read |
| `session` | The session itself is malformed, or names a step that does not exist |
| `unknown_input` | A step names an input the session does not have |
| `invalid_value` | A value cannot be held |
| `refused` | The command bus refused the change, for example a stale revision |
| `nothing_to_change` | Undo or redo with nothing to move |
| `recipe` | The recipe failed to parse, check or run |
| `solve` | The constraint could not be built |
| `render` | The outline could not be drawn |
| `export` | The drawing could not be added to the project |
| `save` | The project could not be saved |
| `reopen` | The project could not be reopened |

A step that fails changes nothing, and the steps before it stay as they were. Nothing in the output depends on the
clock, the locale or the time zone: the operator's authority uses a fixed time, and identities come from a counter.

## How it is checked

Run `cargo test -p sc-cli`. It runs the real `stitchcad` program on a recorded session of
[the reference skirt](reference-skirt.md) twice, under a foreign locale and time zone, and requires identical
transcripts and project files. The same session computes 23 of the book's values, which the test compares with
[the reference skirt](reference-skirt.md) and [the worked examples](formula-language/examples.md), and its four
closure assertions hold.

The two geometry rows of the worked examples, `zip_notch_param` and `dart_apex_depth`, need a constructed edge and
point, so they wait for G2.

## Drafting a garment

A `draft` step names the `style`, the `size` and the `pieces`, and may name the `seams` between them and the `fits` that solve for a length, as [walking and truing](walking-and-truing.md) describes. The `style` is whatever name the design goes by. Every position is a pair of length names: a binding, an input, or `zero`, with a leading `-` to negate one. So the
numbers stay in the recipe and a changed measurement moves every piece. A step that names a length that does not exist stops the session.

| Piece field | What it holds |
| --- | --- |
| `name` | The piece's name |
| `quantity` | How many to cut |
| `start` | The net boundary's first point |
| `edges` | Each edge in turn, counterclockwise: `to`, its allowance `width` naming a length (`zero` for none), its `corner`, `first` and `second` when it is a Bézier, `through` when it is an arc, and a `name` for [what holds it](stable-references.md) |
| `grain` | A list holding one grainline, which is a list of two points |
| `internal` | A list of internal lines, each a list of two points, or an arc as `from`, `through` and `to` |
| `notches` | Each notch, as [a place on a named edge](stable-references.md), or as the point on the cut line and a point it aims at |
| `labels` | Each label, as `at` and `text` |
| `darts` | [Closures](closures.md): each with `name`, `near`, `far`, `apex`, and optionally `intake` |
| `tucks` | Each with `name`, `from`, `to`, and optionally `intake`, `from_end` and `to_end` |
| `pleats` | The same as a tuck |
| `gathers` | Each with `name`, `from`, `to` and `gathered_to` |
| `slash_spreads` | Each with `name`, `pivot`, `slash_to` and `spread`, applied to the boundary in order |
| `mirror_of` | Instead of a boundary: this piece mirrors an earlier one across x = 0, taking its lines, grain and notches, with `labels` of its own |

A notch becomes a vertex of the cut line, so no writer can lose it between two chords. Its named position comes from straight arithmetic and can land a hair from where the offset's
joins put the cut line, so it moves onto that line. More than `NOTCH_REACH` away, which is 0.1 mm, it is refused as a wrong notch.

| Export field | What it holds |
| --- | --- |
| `format` | `canonical`, `dxf` or `pdf` |
| `semantics` | For `dxf`, which line is on the cut layer: `cut_as_1` or `sew_as_1`. There is no default, because the wrong one cuts a whole order short |
| `layout` | For `pdf`: `a0`, `a0_tiled`, `a3_tiled` or `a4_tiled` |

```rust
use sc_cli::replay;
use sc_store::{parse, FilePath, MemoryRepository, Repository};

let session = parse(r#"{
  "format": 1,
  "design": "01ARZ3NDEKTSV4RRFFQ69G5FAV",
  "inputs": [{"token": "width", "um": 200000}, {"token": "height", "um": 100000}, {"token": "allowance", "um": 10000}],
  "recipe": "let inset: length = allowance - 5 mm\n",
  "steps": [
    {"do": "evaluate"},
    {"do": "draft", "style": "sample", "size": "M", "pieces": [{
      "name": "panel", "quantity": 2, "start": ["zero", "zero"],
      "edges": [
        {"to": ["width", "zero"], "width": "allowance", "corner": "miter"},
        {"to": ["width", "height"], "width": "allowance", "corner": "miter"},
        {"to": ["zero", "height"], "width": "allowance", "corner": "miter"},
        {"to": ["zero", "zero"], "width": "allowance", "corner": "miter"}
      ],
      "grain": [[["inset", "inset"], ["inset", "height"]]],
      "notches": [[["height", "-allowance"], ["height", "-inset"]]]
    }]},
    {"do": "export_pattern", "format": "canonical", "path": "exports/panel.json"},
    {"do": "save"}
  ]
}"#).expect("valid text");

let mut disk = MemoryRepository::new();
replay(&session, &mut disk).expect("the session runs");

// A session draws the panel 1 cm larger on every side, and its notch is now a vertex of that cut line.
let written = disk.read(&FilePath::new("exports/panel.json").expect("a path")).expect("reads").expect("exported");
let pattern = sc_artifacts::from_canonical_text(&String::from_utf8(written).expect("text")).expect("reads back");
let panel = &pattern.pieces[0];
let xs: Vec<i64> = panel.cut.iter().map(|at| at.x()).collect();
assert_eq!((xs.iter().min(), xs.iter().max()), (Some(&-10_000), Some(&210_000)));
assert!(panel.cut.contains(&panel.marks[0].at));
```

Exporting also writes `<path>.record.json`, which the writer's own chapter describes: [canonical](canonical-artifacts.md), [DXF](dxf-output.md) or [PDF](pdf-output.md). The step's result in the transcript is the file's digest.

## Names used in this chapter

| Name | Meaning |
| --- | --- |
| `stitchcad` | The program this chapter describes |
| `sc-cli` | The crate that holds it |
| `native-headless` | The runtime profile with no viewport, for the command line, agents and CI |
| `token` | An input's name, which the recipe reads |
| `um` | A length in whole micrometres |
| `at_most_um` | The largest value a `solve` step allows, in micrometres |
| `expect_revision` | The revision a `change` was made against, for the bus to check |
| `failed` | A diagnostic's stable code |
| `message` | A diagnostic's reason, in a sentence |
| `step` | A step's position in the session, starting at 0 |
| `kind` | A binding's kind |
| `name` | A binding's name |
| `value` | A binding's value in the internal unit of its kind |
| `revision` | The Design's revision after a step |
| `reopen` | The step that rebuilds the Design from the saved project |
| `outline` | A `render` step's points |
| `zero` | A length of nothing, for an outline point on an axis |
| `path` | Where an `export` step puts the drawing in the project |
| `draft` | The step that draws the pieces |
| `export_pattern` | The step that writes the drafted pattern |
| `style` | The drafted style's name |
| `size` | The drafted size's name |
| `pieces` | A `draft` step's pieces |
| `quantity` | How many of a piece to cut |
| `start` | A boundary's first point |
| `edges` | A boundary's edges |
| `to` | Where an edge ends |
| `width` | An edge's seam allowance, by length name |
| `corner` | An edge's corner treatment, named in [the offsets chapter](offsets.md) |
| `first` | A Bézier edge's first control point |
| `second` | A Bézier edge's second control point |
| `grain` | A piece's grainline |
| `internal` | A piece's internal lines |
| `through` | A point an arc passes through |
| `notches` | A piece's notches |
| `labels` | A piece's labels |
| `at` | Where a label sits |
| `text` | A label's words |
| `mirror_of` | The earlier piece this one mirrors |
| `NOTCH_REACH` | How far a named notch may lie from the cut line: 100 micrometres |
| `canonical` | The canonical text form |
| `dxf` | The DXF target |
| `pdf` | A true-scale PDF |
| `semantics` | Which line a DXF puts on its cut layer |
| `cut_as_1` | The cut line is on the cut layer |
| `sew_as_1` | The net line is on the cut layer |
| `layout` | A PDF's sheets |
| `a0` | One A0 sheet |
| `a0_tiled` | A0 sheets, tiled |
| `a3_tiled` | A3 sheets, tiled |
| `a4_tiled` | A4 sheets, tiled |
| `format` | A session's format version, or an export's file format |
| `closures` | The step that reports what closing each closure does |
| `darts` | A piece's darts |
| `tucks` | A piece's tucks |
| `pleats` | A piece's pleats |
| `gathers` | A piece's gathers |
| `slash_spreads` | A piece's slash-and-spread operations |
| `near` | A dart's near leg's top |
| `far` | A dart's far leg's top |
| `apex` | A dart's apex |
| `intake` | What a closure declares it takes up |
| `from` | Where a closure's range starts |
| `from_end` | Where a tuck's first fold line ends |
| `to_end` | Where a tuck's second fold line ends |
| `gathered_to` | The length a gather draws its range up to |
| `pivot` | The point a slash and spread turns about |
| `slash_to` | Where the slash meets the boundary |
| `spread` | How far the slash opens there |
| `walk` | The step that walks each seam |
| `plant_balance_notch` | The step that plants a notch on both sides of a seam |
| `true_seam` | The step that makes a seam's sides match |
| `seams` | A draft's list of seams |
| `fits` | A draft's list of lengths to solve for |
| `edit_edge` | The step that edits a named edge |
| `references` | The step that lists what holds an edge |
| `repair_reference` | The step that closes a repair task |
