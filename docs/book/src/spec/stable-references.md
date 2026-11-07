# References that survive an edit

A notch on a side seam means something about the cloth at that place. So when somebody splits the side seam, reverses it or moves its end, the notch has to stay on the same cloth, and when somebody deletes the seam, it has to say so rather than slide onto whatever is nearest. Here a notch, a grade point and a seam's end each hold an edge and a place along it, and where that lands now is the answer of [the identity contract](ontology.md).

**What works today.** Named edges. Notches, grade points and seam ends that hold them. Six edits on an edge, each a command with an undo. A repair task for anything left with nowhere to go, and a cutting file refused while one is open. The [reference skirt](reference-skirt.md) holds every notch and seam this way. **What does not exist yet:** a dart, tuck, pleat or gather holding an edge: their ends are still positions, and one left off the sew line is refused by name. Grade rules that move a grade point, which are `G3-GRADING.7`. Any of this in the viewer.

```rust
use sc_cli::Running;
use sc_store::{parse, MemoryRepository};

let session = parse(r#"{"format":1,"design":"01ARZ3NDEKTSV4RRFFQ69G5FAV",
  "inputs":[{"token":"wide","um":400000},{"token":"tall","um":600000},{"token":"sa","um":10000}],
  "recipe":"let depth: length = 5 mm\n","steps":[]}"#).expect("valid");
let draft = r#"{"do":"draft","style":"panel","size":"M","pieces":[{"name":"panel","quantity":1,"start":["zero","zero"],
  "edges":[{"name":"hem","to":["wide","zero"],"width":"sa","corner":"miter"},
           {"name":"side","to":["wide","tall"],"width":"sa","corner":"miter"},
           {"name":"top","to":["zero","tall"],"width":"sa","corner":"miter"},
           {"name":"back","to":["zero","zero"],"width":"sa","corner":"miter"}],
  "grain":[[["sa","sa"],["sa","tall"]]],
  "notches":[{"name":"hip","edge":"side","at":"1/3","depth":"depth"}]}]}"#;

let mut disk = MemoryRepository::new();
let mut running = Running::open(&session, &mut disk).expect("opens");
running.step(&parse(draft).expect("valid")).expect("draws");
let where_it_is = |running: &mut Running<&mut MemoryRepository>| {
    let record = running.step(&parse(r#"{"do":"references"}"#).expect("valid")).expect("lists");
    record.as_object().expect("a record")["result"].clone()
};

// Split the side above the notch. The notch keeps its cloth, and it is now on the lower part, at 2/3 of it.
let split = r#"{"do":"edit_edge","piece":"panel","op":"split","edge":"side","at":"1/2","into":["lower","upper"]}"#;
running.step(&parse(split).expect("valid")).expect("splits");
let seen = where_it_is(&mut running);
let notch = &seen.as_object().expect("a report")["references"].as_list().expect("a list")[0];
assert_eq!(notch.as_object().expect("a reference")["now"].as_text(), Some("lower at 2/3"));

// Delete the lower part. Nothing moves onto the straight edge that closes the gap, and the notch is a repair task instead.
let delete = r#"{"do":"edit_edge","piece":"panel","op":"delete","edge":"lower","bridge":"patch"}"#;
running.step(&parse(delete).expect("valid")).expect("deletes");
let seen = where_it_is(&mut running);
let task = &seen.as_object().expect("a report")["repairs"].as_list().expect("a list")[0];
assert_eq!(task.as_object().expect("a task")["carrier"].as_text(), Some("notch hip"));

// It can still be drawn and saved. It cannot be released as a cutting file until someone says where it goes.
running.step(&parse(r#"{"do":"draft"}"#).expect("valid")).expect("draws");
let export = r#"{"do":"export_pattern","format":"dxf","semantics":"cut_as_1","path":"panel.dxf"}"#;
assert!(running.step(&parse(export).expect("valid")).is_err());
let repair = r#"{"do":"repair_reference","piece":"panel","notch":"hip","edge":"patch","at":"1/2"}"#;
running.step(&parse(repair).expect("valid")).expect("repairs");
running.step(&parse(r#"{"do":"draft"}"#).expect("valid")).expect("draws");
assert!(running.step(&parse(export).expect("valid")).is_ok());
```

## An edge has a name

An edge in a draft's `edges` list may carry a `name`, and a name is what a reference holds. Two edges of a piece cannot share one, and a name given once is never given again, even after its edge is gone, because a reference may still hold it. A piece's position in the list and a corner's coordinates mean nothing here: an index or a vertex would change under exactly the edits this chapter is about.

## What holds a reference

| Carrier | Where it is written | What it is for |
| --- | --- | --- |
| Notch | An object in a piece's `notches` | A mark on the cut line, square to the edge unless it says otherwise |
| Grade point | An object in a piece's `grade_points` | A named point on the sew line, which grading will move |
| Seam end | The `from` or `to` of a seam's side | Where a stretch of a [seam](walking-and-truing.md) begins or stops |

A notch and a grade point each need a `name`, so that a repair task can say which. A notch also names its `depth`, and it may give an `aim`, two lengths for where it reaches from the sew line to the cut line. An edge gives a corner two different squares, so a notch at a corner states one.

A notch has a `type`, and the ones after these are [the export types of G4](../availability.md):

| Type | What it needs | What it draws |
| --- | --- | --- |
| `single` | `depth`, and this is what a notch with no type is | One mark |
| `double` | `depth` and `spacing` | Two marks either side of the place it holds, each answering for itself, as `notch back, first` and `notch back, second`. Repairing the notch repairs both |
| `drill` | `inset` | A hole inside the piece, that far in from the edge and square to it. It keeps its place if the cut line outside it changes |

## A place on an edge

| Field | What it holds |
| --- | --- |
| `edge` | The edge's name |
| `at` | A fraction of it from 0 to 1, as text, such as `1/3`. Exact, and never rounded |
| `along` | Or a distance from its start: a length's name, or a list of them to add, where a leading `-` subtracts |
| `before_end` | Or a distance back from its end, written the same way |
| `choose` | At a split, `first` or `second` |

**The fraction is in the frame the edge was born in.** A reference is never rewritten. When an edge has been split, a reference to it still says the old edge and the old fraction, and the answer comes from replaying what has been done since. So the same document always gets the same answer, and it can be read to see what was meant. `references` shows both what a carrier holds and where that lands now.

## What each edit does

An edit is an entry in its piece's `edits`, done again at every draft, so it follows a changed measurement. The step that adds one is `edit_edge`, which names the `piece` and the `op`.

| `op` | Fields | What happens to what held the edge |
| --- | --- | --- |
| `split` | `edge`, `at`, `into` with two names | Each stays on the part that has its cloth, with its fraction recomputed. One exactly at the split must say which part it wants, because both contain it |
| `merge` | `first`, `second`, `into` | Each stays, with its fraction recomputed by the two lengths. A pair running opposite ways is refused until one is reversed |
| `reverse` | `edge` | Each stays. The fraction becomes one minus itself, and `direction` says `reversed` so that anything that cares which way the edge runs can tell |
| `delete` | `edge`, `bridge` | Each becomes a repair task with no candidates. `bridge` names the straight edge that closes the gap, and it is a new identity that nothing lands on |
| `allowance` | `edge`, `width`, `corner` | Each stays on the sew line. A notch follows the new cut line |

Two more things change an edge without an entry in `edits`:

| What changes | What happens to what held the edge |
| --- | --- |
| An allowance trimming part of an edge away, as an inside corner does | A notch on the part left has its place. A notch on the part gone is a repair task, because it has no cut line to sit on. A grade point or a seam end stays, since the sew line is still there |
| A `mirror_of` piece | Holds the same edges, and each lands on the mirrored cloth. `direction` says `reversed`, because a mirror runs its boundary the other way. An edit to it is made on the piece it mirrors |

A changed measurement keeps every edge and every fraction, so the cloth moves with the edge. [Slash and spread](closures.md) turns what is on the part it turns, so a reference on that part turns with it.

## A repair task

| Field | What it holds |
| --- | --- |
| `piece` | The piece |
| `carrier` | What holds the reference, such as `notch hip`, or `seam side, side a, from` |
| `reference` | What it holds, as written |
| `orphaned_by` | The edit that orphaned it, in words |
| `candidates` | Where it might go, which is empty when nothing can be suggested honestly |

**Nothing is ever moved for you.** A notch with a task is not drawn, a grade point with a task is not listed, and a seam with an end that has one leaves the graph, along with any balance notch or true that hangs off it. Each of those says so. The design can be saved, drawn, walked and looked at, including by `export_pattern` to the canonical form, which lists what is open. A DXF or PDF is refused, because a cutting file with a notch missing does not look wrong to the person cutting it.

`repair_reference` closes one. It names the `piece` and the `notch` or `grade_point`, or the `seam`, `side` and `end`, and then says what to do.

| Field | What it does |
| --- | --- |
| `edge`, with `at`, `along` or `before_end` | Holds this place instead |
| `choose` | Says which part it wants at a split |
| `retire` | Removes the carrier, or the whole seam |

## They are commands

An edit and a repair are [`ChangeDrafting`](walking-and-truing.md) on the bus, each with a revision, an actor, an authority check, an audit entry and an undo, and a caller without `commit` is refused by the bus itself. Whatever cannot be drawn is refused, with nothing changed. So an edit that is drawn may leave repair tasks, and one that would leave no boundary does not happen.

```text
$ cargo test -p sc-cli --test references
```

## Sessions written before this

A notch written as two points, and a seam end written as a point, still work, because a saved design has to keep opening: the sessions recorded by [the agent gate](api-and-mcp.md) are written that way. They are not references. A split leaves them where they are, because they name a place and not an edge, and if the place is no longer on the line they are refused by name. They do not survive an edit, and they do not pretend to.

**What this does not show.** That a notch is in the right place to begin with, which is a sewing judgement nobody has reviewed. A fraction stays on the same cloth only while the edge keeps its shape: when a measurement changes, a notch held at a fraction moves in proportion, and one that should stay a fixed distance from a corner should use `along`. How much of an edge the cut line still follows is found by trying 64 places along it, and every place a notch holds, so a trimmed stretch shorter than that spacing with no notch in it is not reported, which makes no difference to anything. The two models of a piece are [still apart](closures.md): this uses the core's journal and answers, and not its structural `Piece`.

## Names used in this chapter

| Name | Meaning |
| --- | --- |
| `G3-GRADING.7` | The leaf for grade rules |
| `edges` | A draft's list of a piece's edges |
| `name` | What an edge, notch or grade point is called |
| `notches` | A piece's list of notches |
| `grade_points` | A piece's list of grade points |
| `from` | Where a seam's side starts |
| `to` | Where a seam's side stops |
| `depth` | How deep a notch is |
| `aim` | The line a notch takes from the sew line to the cut line |
| `edge` | Which edge a place is on |
| `at` | A fraction of an edge |
| `1/3` | A fraction, as `at` writes it |
| `along` | A distance from an edge's start |
| `before_end` | A distance back from an edge's end |
| `type` | What kind of notch it is |
| `single` | One mark |
| `double` | Two marks |
| `drill` | A hole inside the piece |
| `spacing` | How far apart a double notch's marks are |
| `inset` | How far in from its edge a drill is |
| `notch back, first` | A carrier, as a double notch's first mark is named |
| `notch back, second` | A carrier, as its second is named |
| `-` | The sign that subtracts a length |
| `choose` | Which part a place wants at a split |
| `first` | The part before a split |
| `second` | The part after a split |
| `references` | The step that lists every reference and repair task |
| `edits` | A piece's list of edits |
| `edit_edge` | The step that adds one |
| `piece` | Which piece an edit or a task is on |
| `op` | Which edit it is |
| `split` | Makes one edge two |
| `into` | The names of what an edit makes |
| `merge` | Makes two edges one |
| `reverse` | Turns an edge's own direction around |
| `direction` | Whether a place's edge now runs the other way |
| `reversed` | It does |
| `delete` | Removes an edge |
| `bridge` | The straight edge that closes a deleted edge's gap |
| `allowance` | Gives an edge a new allowance |
| `width` | An allowance's width |
| `corner` | An allowance's corner treatment |
| `mirror_of` | The piece a piece mirrors |
| `carrier` | What holds a reference |
| `notch hip` | A carrier, as a task names it |
| `seam side, side a, from` | A carrier, as a task names it |
| `reference` | What a carrier holds |
| `orphaned_by` | The edit that orphaned it |
| `candidates` | Where it might go |
| `export_pattern` | The step that writes a file |
| `repair_reference` | The step that closes a repair task |
| `notch` | A notch's name, in a repair |
| `grade_point` | A grade point's name, in a repair |
| `seam` | A seam's name, in a repair |
| `side` | Side `a` or `b`, in a repair |
| `a` | A seam's first side |
| `b` | A seam's second side |
| `end` | `from` or `to`, in a repair |
| `retire` | Removes what held the reference |
| `ChangeDrafting` | The bus command that replaces how a garment is drawn |
| `commit` | The authority an edit needs |
| `Piece` | The core's structural piece |
