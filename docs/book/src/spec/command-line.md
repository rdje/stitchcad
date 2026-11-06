# Command line

The `stitchcad` program runs a recorded session against a project directory: evaluate a recipe, edit a measurement,
undo and redo, solve a constraint, save and reopen. Every edit goes through the command bus, the same path every
other front end will use. The same session always prints the same bytes and saves the same files. It is the
`sc-cli` crate, the `native-headless` front end of roadmap §7.3.

**What works today.** A session supplies named lengths and a scalar recipe. The program builds a real Design from
them, runs the steps below, and prints a transcript. **What does not exist yet:** geometry, so `render` draws straight
lines between points the session names rather than a drafted Piece, which is G2's job; any edit but a measurement
change; `sc-api` and MCP, leaf `.9`.

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
| `change` | Replaces one input's value through the bus, at a new revision. `token`, `um`, and optionally `expect_revision` |
| `undo` | Returns the Design to its state before the last change, under a fresh authority check |
| `redo` | Reapplies the last change undone |
| `solve` | Checks one constraint: the input named by `token` may be at most `at_most_um`, using [the constraint kernel](constraints.md) |
| `render` | Draws an outline as SVG. `outline` lists points, each naming two lengths: a binding, an input, or `zero` |
| `export` | Adds the last drawing to the project at `path`, and reports its digest. The next `save` writes it |
| `save` | Saves the project as one [crash-consistent commit](project-store.md), keeping files and fields it did not write |
| `reopen` | Rebuilds the Design from the saved project |

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
