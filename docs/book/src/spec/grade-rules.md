# Grade rules

The [instantiation chapter](instantiation-paths.md) says what a table of grade rules means. This chapter is the table itself: what it holds, the two ways of writing its deltas, what it promises beyond them, and the `.rul` file a factory's system reads.

**What works today.** A table with a stable name for each point, in either convention, read and written exactly. A stack point, honoured. A contour declared fixed, honoured by a conflict. Smoothing, refused rather than ignored. The `.rul` file, written and read. **What does not exist yet:** any receiver reading a file written here, which `G3-GRADING.8` leaves open. Fairing a contour. An alternate grade reference.

```rust
use sc_grading::{from_rul, to_rul, Attributes, Convention, GradeTable, Provenance, Rule, ZeroState};

let sizes = vec!["S".to_owned(), "M".to_owned(), "L".to_owned()];
let table = GradeTable::new(
    "shirt".to_owned(),
    ("M".to_owned(), sizes),
    Convention::Cumulative,
    Provenance::Regenerations,
    vec![
        Rule { point: "front/cf/to".to_owned(), id: 1, deltas: vec![(0, 0); 3], zero: ZeroState::Declared },
        Rule { point: "front/side/to".to_owned(), id: 2, deltas: vec![(-10_000, 0), (0, 0), (10_000, 2_500)], zero: ZeroState::Declared },
    ],
    Attributes { stack: vec!["front/cf/to".to_owned()], ..Attributes::default() },
).expect("a table");

// The same table, from the next size in rather than from the sample, and nothing is lost.
let stepped = table.in_convention(Convention::Incremental).expect("converts");
let side = stepped.rule("front/side/to").expect("the rule");
assert_eq!(stepped.delta_at(side, "L").expect("a size"), (10_000, 2_500));

// A file a factory's system reads: millimetres to two places, always from the sample.
let file = to_rul(&table).expect("writes");
assert!(file.text.contains("RULE: DELTA 2\n-10.00,0.00 0.00,0.00 10.00,2.50\n"));
let read = from_rul(&file.text).expect("reads");
assert_eq!(read.sample, "M");
assert_eq!(read.rules[1].deltas[2], (10_000, 2_500));
```

## A table

| Field | What it holds |
| --- | --- |
| `name` | The table's name |
| `sample` | The size everything is measured from |
| `sizes` | Every size, from smallest to largest |
| `convention` | `cumulative` or `incremental` |
| `provenance` | `regenerations` or `received`, because a table worked out from two drawings and one from a factory are different evidence |
| `stack` | The points that do not move |
| `fixed` | The contours whose length must not change |
| `smoothing` | The contours to fair, which is refused |
| `rules` | One for each point |

A rule has a `point`, a `deltas` list with an X and a Y for each size, and an `id`, the number a file calls it by. **The `point` is its identity,** written as its piece and what it is on that piece, such as `bodice_front/armhole/to`, so that it survives an edit. The `id` only holds within one export.

| Convention | Each delta is |
| --- | --- |
| `cumulative` | From the sample size |
| `incremental` | From the next size toward the sample |

**A zero means something.** A rule of all zeros says the point does not move, and a point with no rule says nothing about it, and those are different. A table written here marks its zeros `declared`, and one read from a file marks them `unknown`, because a file cannot say whether a zero was meant.

## What a table promises

| Attribute | What happens |
| --- | --- |
| `stack` | A table names at least one, and one that moves is refused. A table worked out here is measured from one on each piece, so that a delta is the piece's change and not its drift |
| `fixed` | A contour, as `name`, `piece`, `from` and `to`, whose length must not change. If grading would change it, that is a conflict naming the contour and the size, and the graded files are not written |
| `smoothing` | Refused by name. No fairing exists here, and a table that asks for one does not get ignored |

## The file

**The file was read for this, and it is smaller than the book first said.** [The instantiation chapter](instantiation-paths.md) cited stack points, fixed perimeters and smoothing as part of the format from the roadmap. The sample pages of ASTM D6673-10 and DH Patterns and Fit's guide to it (2011, 2012) show a file holding a header and `DELTA` rules, measured from the sample, and nothing else. So those three are this table's, and the file cannot carry them.

| Line | What it holds |
| --- | --- |
| `ASTM/D13 Proposal 1 Version: D 6673-10` | What this is |
| `AUTHOR:`, `PRODUCT:` | Who wrote it |
| `UNITS: METRIC` | Millimetres, to two places |
| `GRADE RULE TABLE:` | The table's name |
| `NUMBER OF SIZES:` | How many |
| `SIZE LIST:` | The sizes, smallest first |
| `SAMPLE SIZE:` | The sample |
| `RULE: DELTA` | The start of a rule, with its number |
| `END` | The end of the file |

Each rule is followed by an X and a Y for each size, with four to a line, and zeros at the sample. The file has no date or time, because those two lines are optional and a pattern has neither. **Each file comes with a record,** as every written file here does, saying the largest rounding two places made, and what the file could not carry: `stack_points`, `fixed_contours`, `point_names` and `declared_zero_state`.

Reading takes metric or English, and any of the separators the format allows. It refuses a rule of any type but `DELTA`, a file that stops before `END`, a rule with the wrong number of values, one that moves the sample, and units it does not know. [The grading chapter](grading.md) says how a table is made and used.

```text
$ cargo test -p sc-grading --test table
```

## Read by something else

A file only this project can read is no evidence, and [the gate](../availability.md) asks for an independent engine. **None is available.** Commercial CAD is parked by the director until the product is feature complete, and a search found no open reader of this format. So that clause stays open, and this is the nearest thing to it there is: **a reader written separately, by another agent, from the format's description and the two files alone,** without ever seeing this project's code. `bash docs/tasks/artifacts/grading/independent_reader/run_independent_reader.sh` runs it on all three garments and compares where it says each corner lands, at every size, with where the recipe puts that corner. A comparison that cannot fail proves nothing, so each garment is also compared with one row tampered with by 20 µm, and that has to be caught.

| It found | How that is classified |
| --- | --- |
| Every corner at every size agrees within 10 µm, which is the two rounded values a file adds: 0 µm on the skirt, 4 µm on the trousers and 9 µm on the bodice, over 310 corners and sizes | Agreement, at the file's own quantum |
| A rule wraps across lines, four pairs to a line, which the description does not say. It read them by counting | A documented ambiguity. The published example does the same |
| On the skirt, six rules have no identifier anywhere in the drawing | This project's convention: they are for where a label sits, which is not a point a file can name. A receiver has nothing to attach them to |
| It chose to refuse non-metric units and anything after `END`, and took the first number of `# n, alt` | The reader's own choices |

**This is not an engine.** It reads and adds, and it rebuilds no curve and cuts nothing. It shows that a careful reader of the description gets the same numbers, and it cannot show what a factory's system would do. The reader is kept verbatim, because editing it would make it not independent. Its author was the same model as the author of the writer, so a misunderstanding they share would not show.

**What this does not show.** That any receiver reads these files as intended. One file here was not written by this project, an example published in that guide, and it reads as its author wrote it, which is a fair test of the reader and no test of the writer. Only the sample pages of the standard were read, and the guide is a third party's reading of it. Two decimal places hold a delta to 5 µm, which is a loss the record states and the importer class has to allow.

## Names used in this chapter

| Name | Meaning |
| --- | --- |
| `.rul` | The grade rule table file |
| `G3-GRADING.8` | The leaf for a receiver reading these files |
| `name` | A table's name, or a contour's |
| `sample` | The size everything is measured from |
| `sizes` | A table's sizes |
| `convention` | How its deltas are measured |
| `cumulative` | From the sample size |
| `incremental` | From the next size toward the sample |
| `provenance` | Where it came from |
| `regenerations` | Worked out here |
| `received` | From somewhere else |
| `stack` | The points that do not move |
| `fixed` | The contours that must not change |
| `smoothing` | The contours to fair |
| `rules` | A table's rules |
| `point` | What a rule is for |
| `deltas` | What it does at each size |
| `id` | The number a file calls it by |
| `bodice_front/armhole/to` | A point, by its piece and what it is |
| `declared` | This table means this zero |
| `unknown` | A file said zero |
| `piece` | A contour's piece |
| `from` | Where a contour starts |
| `to` | Where it stops |
| `DELTA` | The one type of rule |
| `bash docs/tasks/artifacts/grading/independent_reader/run_independent_reader.sh` | Runs a separately written reader on the frozen files |
| `# n, alt` | An identifier with an alternate reference |
| `ASTM/D13 Proposal 1 Version: D 6673-10` | A file's first line |
| `AUTHOR:` | A header line |
| `PRODUCT:` | A header line |
| `UNITS: METRIC` | A header line |
| `GRADE RULE TABLE:` | A header line |
| `NUMBER OF SIZES:` | A header line |
| `SIZE LIST:` | A header line |
| `SAMPLE SIZE:` | A header line |
| `RULE: DELTA` | The start of a rule |
| `END` | The end of a file |
| `stack_points` | What a file cannot carry |
| `fixed_contours` | What a file cannot carry |
| `point_names` | What a file cannot carry |
| `declared_zero_state` | What a file cannot carry |
| `cargo test -p sc-grading --test table` | What holds this chapter |
