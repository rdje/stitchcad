# Walking and truing

A patternmaker walks one seam along another to see whether they meet, and trues them when they do not. Here both are operations a person or an agent asks for, on a sewing graph that is data in the design, and a difference between two sides is either ease somebody declared or a defect nobody did.

**What works today.** Seams between stretches of sew lines, with declared ease. `walk`, which reports each one. Balance notches planted on both sides. `true_seam`, which makes the sides match within a stated limit. [The reference skirt's](reference-skirt.md) seven seams are declared and each walks to a match. **What does not exist yet:** a seam reference that survives an edit to its edge, which is `G3-GRADING.3`, because a side still names its ends by position. Truing a curved end. The sewing graph in the viewer.

```rust
use sc_cli::Running;
use sc_store::{parse, MemoryRepository};

// Two panels whose top edges are sewn together: side A is 42 cm and side B is 40, with 2 cm of ease declared.
let session = parse(r#"{"format":1,"design":"01ARZ3NDEKTSV4RRFFQ69G5FAV",
  "inputs":[{"token":"a_width","um":420000},{"token":"b_width","um":400000},{"token":"tall","um":100000},{"token":"cap_ease","um":20000}],
  "recipe":"let quarter: length = a_width / 4\n","steps":[]}"#).expect("valid");
let piece = |name: &str, width: &str| format!(r#"{{"name":"{name}","quantity":1,"start":["zero","zero"],
  "edges":[{{"to":["{width}","zero"],"width":"zero","corner":"miter"}},{{"to":["{width}","tall"],"width":"zero","corner":"miter"}},
           {{"to":["zero","tall"],"width":"zero","corner":"miter"}},{{"to":["zero","zero"],"width":"zero","corner":"miter"}}],
  "grain":[[["zero","zero"],["zero","tall"]]]}}"#);
let draft = format!(r#"{{"do":"draft","style":"two","size":"M","pieces":[{},{}],
  "seams":[{{"name":"cap","ease":"cap_ease",
             "a":{{"piece":"a","from":["zero","tall"],"to":["a_width","tall"]}},
             "b":{{"piece":"b","from":["zero","tall"],"to":["b_width","tall"]}}}}]}}"#, piece("a", "a_width"), piece("b", "b_width"));

let mut disk = MemoryRepository::new();
let mut running = Running::open(&session, &mut disk).expect("opens");
running.step(&parse(&draft).expect("valid")).expect("draws");
let walked = running.step(&parse(r#"{"do":"walk"}"#).expect("valid")).expect("walks");
let report = &walked.as_object().expect("a record")["result"].as_list().expect("reports")[0];
let report = report.as_object().expect("a report");
assert_eq!(report["differential"].as_int(), Some(20_000));
assert_eq!(report["finding"].as_text(), Some("declared_ease"));
```

## A seam

A draft's `seams` list names each seam and its two sides. A side is a stretch of a piece's sew line, from one point to another, walked the shorter way round. `ease` names a length, side A less side B, and a seam that gives none declares zero.

| Field | What it holds |
| --- | --- |
| `name` | The seam's name |
| `a` | One side: `piece`, `from` and `to` |
| `b` | The other side, in the same direction along the seam |
| `ease` | The difference that is intended, by name. Negative means side B is the longer |

**A closure is sewn away before a seam is measured.** Each side's length is its stretch less the intake of any [closure](closures.md) with both ends on it. So a waist with a 4 cm dart sews to a band 4 cm shorter, with nothing else to say.

## What walking finds

| Report field | What it holds |
| --- | --- |
| `a_length` | Side A's sewn length |
| `b_length` | Side B's sewn length |
| `differential` | Side A less side B |
| `ease` | What the seam declares |
| `unexplained` | The part of the differential no declaration explains |
| `finding` | `matches`, `declared_ease` or `defect` |

A sleeve cap 2 cm longer than its armscye with 2 cm declared is `declared_ease`: intentional, and named as such. The same 2 cm with nothing declared is `defect`, and so is the wrong amount declared. An end that is not on its sew line, or a piece that does not exist, is refused by name. The tolerance is twice T2, because two lines each within T2 of one curve can differ by that much.

## Balance notches

`plant_balance_notch` takes a `seam`, a distance `at` along side A, and a `depth`, and plants a notch on both sides: at that distance along A, and at the same share of B, so the ease is spread evenly between notches and a notch means the same on both. Each is marked on the cut line, as every notch is. It is written in the design, so it moves when a measurement does.

## Truing

`true_seam` takes a `seam`, a `side` and a `limit_um`, and moves the end of that side's last straight run along itself until what is unexplained is gone. What the seam declares as ease stays. So 3 cm of difference with 2 cm declared moves the end by 1 cm.

**It never moves a thing it was not asked to.** The caller says which side and how far it will accept, a true past that limit is refused and changes nothing, and it reports the walk before and after. A last run too short to take the change is refused by name. It is an operation in the design and not an edit to its geometry, so a new measurement is trued again when it is drawn.

## They are commands

The way a garment is drawn is part of the Design, and an edit to it is `ChangeDrafting` on the command bus, with a revision, an actor, an authority check, an audit entry and an undo. Planting and truing are two such edits, and a caller without `commit` is refused by the bus itself. After one, the drawing is gone until the next `draft`, because it no longer describes the Design. The first drawing is part of creating the design, so it takes no revision, and what came before it cannot be undone past it.

The core holds that document as text and does not read it, which keeps [the two models of a piece](closures.md) apart until they are bound.

```text
$ cargo test -p sc-cli --test walk
```

**What this does not show.** That a declared ease is the right amount: that is a sewing judgement, and nobody who sews has reviewed one here. No garment has a real eased seam yet, because the skirt's all declare zero, and the sleeve cap is `G3-GRADING.4`. A balance notch does not account for a closure between it and the seam's start, so one placed past a dart lands by distance along the open line.

## Names used in this chapter

| Name | Meaning |
| --- | --- |
| `walk` | The step that walks every seam |
| `plant_balance_notch` | The step that plants a notch on both sides of a seam |
| `true_seam` | The step that makes a seam's sides match |
| `ChangeDrafting` | The bus command that replaces how a garment is drawn |
| `G3-GRADING.3` | The leaf for references that survive an edit |
| `G3-GRADING.4` | The leaf for the bodice and sleeve |
| `seams` | A draft's list of seams |
| `name` | A seam's name |
| `a` | A seam's first side |
| `b` | A seam's second side |
| `piece` | The piece a side is on |
| `from` | Where a side starts |
| `to` | Where a side ends |
| `ease` | The intended difference between the sides |
| `a_length` | Side A's sewn length |
| `b_length` | Side B's sewn length |
| `differential` | Side A less side B |
| `unexplained` | What no declaration explains |
| `finding` | What the walk concludes |
| `matches` | The sides agree, and none was declared |
| `declared_ease` | What differs is what was declared |
| `defect` | What differs was not declared |
| `seam` | Which seam a notch or a true is on |
| `at` | How far along side A a notch sits |
| `depth` | How deep a notch is |
| `side` | Which side a true moves |
| `limit_um` | How far a true may move, in micrometres |
| `commit` | The authority an edit needs |
| `draft` | The step that draws the garment |
