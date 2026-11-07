# Grading a garment

A pattern at one size is not yet a product. The [instantiation chapter](instantiation-paths.md) says what the two ways of getting a garment's other sizes mean, and why neither replaces the other. This chapter is what you can do today.

**What works today.** A draft can name its size set. `regenerate` draws every size of it, each a fresh evaluation of the recipe with nothing inherited from another, and says what drove every point. **What does not exist yet:** moving a base size by a table of rules, `.rul` files and comparing the two paths, which are `G3-GRADING.7` and `G3-GRADING.11`.

```rust
use sc_cli::Running;
use sc_store::{parse, MemoryRepository};

let whole = parse(include_str!("../../../conformance/reference-skirt/session.json")).expect("valid");
let fields = whole.as_object().expect("a session");
let mut disk = MemoryRepository::new();
let mut running = Running::open(&whole, &mut disk).expect("opens");
for step in fields["steps"].as_list().expect("steps") {
    running.step(step).expect("runs");
}

let asked = parse(r#"{"do":"regenerate"}"#).expect("valid");
let record = running.step(&asked).expect("regenerates");
let report = record.as_object().expect("a record")["result"].as_object().expect("a report").clone();
assert_eq!(report["sample"].as_text(), Some("M"));

// Five sizes, and none of them was edited by hand.
let sizes = report["sizes"].as_list().expect("sizes");
assert_eq!(sizes.len(), 5);
let smallest = sizes[0].as_object().expect("a size");
assert_eq!(smallest["size"].as_text(), Some("XS"));

// The hem of the front at XS: which formula put it there, and what that comes down to.
let hem = smallest["points"].as_list().expect("points").iter()
    .map(|point| point.as_object().expect("a point"))
    .find(|point| point["piece"].as_text() == Some("skirt_front") && point["edge"].as_text() == Some("hem"))
    .expect("the point");
let x = hem["x"].as_object().expect("an axis");
assert_eq!(x["name"].as_text(), Some("quarter_hem_width"));
assert_eq!(x["formula"].as_text(), Some("quarter_hip + a_line_flare"));
assert_eq!(x["value"].as_int(), Some(265_000));
```

## A size set

A draft's `sizes` names the `sample`, which is the size it is drawn at, and a `list` running from smallest to largest, because that is the order a rule table's size list runs in. Each size has a `name`, and all but the sample say what they set in `inputs`, by name and in micrometres. The sample sets nothing, because it is the design as it stands.

A label is written with `{size}` where the size goes, so that each one says the size it is on.

| The size set | Refused because |
| --- | --- |
| Its sample is not the size the draft is drawn at | The two would disagree about what the design is |
| Its sample is not in the list | There would be nothing to measure the others from |
| Its sample sets inputs | The sample is the design as it stands |
| A size is listed twice | A size has one set of values |
| A size sets an input that does not exist | It says what it set, and nothing is ignored |

The reference skirt's set moves these, per step, from XS through S, M and L to XL. [The bodice's](reference-bodice.md) and [the trousers'](reference-trousers.md) tables are in their chapters. A test holds each table to its session.

| Measurement | One step |
| --- | --- |
| `waist_girth` | 40 mm change |
| `hip_girth` | 40 mm change |
| `waist_to_hem` | 10 mm change |

## Regenerating

`regenerate` draws them all. It changes nothing in the Design, gives the same bytes every time, and with a `path` it writes the whole family in canonical form, which the next `save` lands. For each size it reports:

| Field | What it holds |
| --- | --- |
| `size` | The size |
| `canonical_sha256` | The digest of that size's canonical form |
| `walk` | Each [seam's walk](walking-and-truing.md) at that size |
| `closures` | Each [closure](closures.md) at that size |
| `fits` | What each [fit](walking-and-truing.md) came to at that size |
| `points` | For every point of every boundary, what drove it |

Each entry in `points` names its `piece`, its `edge` and which `point` of that edge it is, and on each axis, `x` and `y`, gives the length's `name`, its `value` at that size, its `formula` as the recipe writes it, and the `inputs` it comes down to. So a point that lands somewhere surprising at XS can be traced to the measurement that put it there.

**A size that cannot be drawn stops the whole thing,** and it says which size and why, including when a recipe's closure does not hold there.

**What this does not show.** That these sizes fit anybody: they are what the recipe gives at those measurements, and a size chart nobody has fitted is a set of numbers. The steps are an engineer's and come from no standard. A test does measure each size from its drawing against what it asked for, which is what stops a family that closes on paper being wrong in the cloth, and it does that for three garments and only for the measurements named there.

## Names used in this chapter

| Name | Meaning |
| --- | --- |
| `regenerate` | The step that draws every size |
| `G3-GRADING.7` | The leaf for grade rules |
| `G3-GRADING.11` | The leaf for comparing the two paths |
| `.rul` | The file a table of grade rules is written to |
| `sizes` | A draft's size set |
| `sample` | The size the design is drawn at |
| `list` | Every size, from smallest to largest |
| `name` | A size's name, or a length's |
| `inputs` | What a size sets, or what a length comes down to |
| `{size}` | Where a label's size goes |
| `path` | Where to write the family |
| `save` | The step that lands a project's files |
| `piece` | Which piece a point is on |
| `edge` | Which edge |
| `point` | Which point of that edge: `to`, `first`, `second` or `through` |
| `to` | Where an edge ends |
| `first` | A Bézier's first control point |
| `second` | Its second |
| `through` | A point an arc passes through |
| `x` | Across |
| `y` | Up |
| `value` | A length at that size |
| `formula` | How the recipe writes it |
