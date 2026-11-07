# Grading a garment

A pattern at one size is not yet a product. The [instantiation chapter](instantiation-paths.md) says what the two ways of getting a garment's other sizes mean, and why neither replaces the other. This chapter is what you can do today.

**What works today.** A draft can name its size set. `regenerate` draws every size of it, each a fresh evaluation of the recipe with nothing inherited from another, and says what drove every point. `grade` moves the sample by a [table of rules](grade-rules.md) and rebuilds each size with the recipe left out of it. Both are compared, quantity by quantity. `check_extremes` runs every check on the smallest and largest sizes after they have come through a written file. `export_graded` writes the grading in each of three named modes. **What does not exist yet:** any receiver reading the result, which is `G3-GRADING.8`.

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

## Grading by rule

A draft's `grading` says what its table is called and, for each piece, which corner it `stack`s on, as `edge/to`. `grade` then works a table out from a regeneration of each size, or takes one, as `table` for a project's file or `table_text` for the table itself, and rebuilds every size from the sample. It changes nothing in the Design.

| Field of `grade` | What it does |
| --- | --- |
| `table` | Uses the table at this path in the project |
| `table_text` | Uses this table |
| `convention` | Writes the table in `cumulative` or `incremental` |
| `rules_path` | Writes the table here |
| `rul` | Writes a `.rul` file here, with its record beside it |
| `path` | Writes the graded family here |

**How a size is rebuilt.** Each corner goes where its rule says. A straight edge stays straight. An arc goes through its moved ends and middle, and if those three lie on a line it is drawn straight and says so. A cubic's control points move by the interpolation of its two ends. An allowance is worked out again at its own width, never graded. A notch, a seam's end and a pocket keep their place on their edge rather than their distance along it, and a balance notch keeps its share of the seam. A dart's ends, a label and a grain line have rules of their own, as any point does. A point the table does not mention moves in proportion: a corner to the corners either side of it, and anything on a straight edge along that edge.

For each size it reports:

| Field | What it holds |
| --- | --- |
| `walk` | Each seam's walk at that size |
| `unmentioned` | Points of this design the table does not mention |
| `unmatched` | Rules for points this design does not have |
| `notes` | What was only approximated, or could not be rebuilt as it was |
| `conflicts` | Any contour declared fixed that changed, which stops the files being written |

A draft with a true in it is refused, because a true is worked out from the drawing and a rule cannot follow it yet.

**The two paths do not agree, and this is by how much.** Worked out from regenerations, every corner lands exactly where the recipe puts it, and on the reference skirt everything does. On the bodice and trousers, what is between the corners does not. These are the most any line differs, with rules on the corners alone, two sizes from the sample:

| Garment | Most any sew line differs | What it does to a seam |
| --- | --- | --- |
| Reference skirt | Not at all | Its waist seams are up to 0.8 mm out, because the band's ends keep their fraction |
| Reference trousers | 1.3 mm, at the crotch curve | Its waist seams are up to 0.9 mm out, and a pocket's mouth up to 7.9 mm, because an opening held by fraction grows with its edge and the bag does not |
| Reference bodice | 4.5 mm, at the armhole | The cap's ease drifts by up to 2.6 mm from the 12 mm declared, and the seams at the underarm and the collar by up to 0.9 mm |

So walking a size that came from a rule can find a defect that regenerating that size does not, and that is the right answer: it is what a factory's system would do with those rules. More grade points along a curve would bring it down, and a table here has none yet.

## Comparing the two paths

`grade` always reports `equivalence`, comparing what the rules gave against what the recipe draws at that size. **Neither wins, and nothing is reconciled.** The [instantiation chapter](instantiation-paths.md) says why, and this is what you get.

| Field | What it holds |
| --- | --- |
| `provenance` | Where the table came from, because a table worked out here and one received are different evidence |
| `classes` | Each tolerance class used, its bound, and where that number comes from |
| `compared` | How many quantities were compared |
| `not_compared` | Any size of the table the draft's own set does not have, so the recipe cannot speak to it |
| `findings` | Each quantity beyond its class |

With `equivalence_path` it writes all of that with every quantity in `quantities`, whether or not anything differs, because a report that only appears when something is wrong says nothing when it is absent. Each one names its `quantity` and `size`, `by_recipe`, `by_rule`, `difference_um`, its `class` and its `verdict`, which is `holds` or `finding`. A finding also names its `owner`.

| Quantity | Judged at | Because |
| --- | --- | --- |
| A point a rule moves | `T1`, 1 µm | Both paths compute an integer, so anything else is a bug in the table or in the rebuild |
| A sew line | `T2`, 10 µm | The tightest thing that uses this geometry is a seam's walk |
| What a seam has beyond what it declares | `seam`, 20 µm | Twice `T2`: what a walk can tell apart |
| What a closure takes up | `seam`, 20 µm | It is taken out of a seam's length |

**These are not chosen to pass, and they do not all pass.** On the bodice the armhole is a finding at every size but the sample. `T4` and `T5` are listed with no bound, because a receiver's tolerance and a factory's are theirs to declare, and none has. So a finding here says the two paths differ by more than this project can call the same. It does not say the garment is unusable, and only a factory's tolerance can say that. A test reads the table above against what the report measures.

## The two ends of the set

A size five breaks from a valid sample can still be wrong, so `check_extremes` rebuilds the smallest and largest sizes from a `.rul` file it has written and read back, and runs every check on each. It reports `through`, which says what read that file: **this project's own engine, because no target system is available,** and that is not the same evidence. Each check fails with its own words, and none clamps anything.

| Check | Class | What it refuses | Limit |
| --- | --- | --- | --- |
| `declared_domain` | numerical | A coordinate past 1 km or a piece past 10 m, which means a unit went wrong | The units chapter's |
| `offset_validity` | geometric | An allowance that crosses itself, or a size that cannot be drawn at all | None needed |
| `minimum_curvature_radius` | geometric | A bend too tight for a knife, on a curve and not at a corner | `min_radius` |
| `bounding_box_against_fabric` | physical | A piece wider across its grain than the cloth | `fabric_width` |
| `seam_differential_against_ease` | geometric | A seam with more or less than it declares | The walk's own |
| `notch_spacing` | physical | Two marks too close to tell apart | `notch_spacing` |
| `dart_intake_against_maximum` | physical | A dart taking more than one can | `dart_intake` |
| `finished_dimension` | exact | A dimension read from the drawing that is not what the recipe says | The walk's own, for each thing added up |

A verdict is `holds`, `fails`, or `no_limit`. **A limit belongs to a factory,** and no factory profile exists, so each reference garment's draft declares the four in `limits` under `grading`, as assumptions: 5 mm, 1.5 m, 3 mm and 50 mm. With none declared, the verdict says so and no number is made up. `finished` lists what to read off the drawing: each has a `name`, says what it `equals`, and adds up `seams`, on `side` A or B, or measures the `widest` of pieces, `times` as often as the garment has them.

**What it found.** On all three garments, at both ends, everything holds except the seams, which do not walk to what they declare, for the reason the table above gives. That is a bounded difference and not a clamped one: the bodice's cap is 2.6 mm out at its worst, and it says which seam by how much. A size too far to draw is refused by name when regenerating it, before anything is rebuilt.

## The three modes

`export_graded` takes a `mode`, `semantics` and a `path`, and writes one mode's files beside its record. **No mode serves every receiver,** there is no universal one, and asking for one is refused. Which a receiver wants is its own to say, and none has said.

| `mode` | Target | What it writes |
| --- | --- | --- |
| `base_plus_rules` | `dxf-aama-named`, R12 | The sample on named layers, with a `# ` identifier at each point, and a `.rul` file with the deltas |
| `embedded_rules` | `dxf-astm-num-r13`, R13 | The sample on numbered layers, with style text and an identifier at each point, and a `.rul` file with the deltas |
| `all_contours` | `dxf-astm-num-r13`, R13 | A block for each piece at each size, in size order, each with a grade reference line. No rules |

**Only `sew_as_1` exists,** because a rule moves a net corner, so a graded file puts the net line on the layer a receiver reads, and generates its allowances downstream. Asked for `cut_as_1` it refuses and says why. A mirror is not a block of its own: a pair is told by the piece's `Quantity:`, as right and left, because the file's convention has that and a receiver flips a rule's X for the left.

**"Embedded" does not mean what the roadmap's words suggest.** The standard's pages hold no place in a DXF for a delta. A file holds `# ` identifiers, and "the grade rules associated with" them "may optionally be defined in a separate" table file. So a receiver of `embedded_rules` still needs the table its style text names, and the mode's first loss says so. This chapter calls that out because [the dialects chapter](interchange-dialects.md) described it otherwise.

**The same points at every size.** A nest's sizes have to correspond point by point, in the same order and number, so its net lines are written with evenly spaced points from the curves themselves, and a test holds them within the 100 µm a polyline target allows. So a nest's line is not the line the other modes write. Notches, labels and the cut line are on the sample only, as the standard says for notches.

Each record states what that mode loses, in these words:

| Loss | In |
| --- | --- |
| `named_layers_merge_turn_points_curve_points_and_the_boundary` | `base_plus_rules` |
| `rules_only_on_the_points_listed_and_between_them_a_receiver_interpolates` | `base_plus_rules`, `embedded_rules` |
| `stack_points_fixed_contours_and_declared_zeros_are_not_carried` | `base_plus_rules`, `embedded_rules` |
| `the_recipe_is_not_carried` | All three |
| `the_file_holds_identifiers_and_the_deltas_are_in_a_table_it_names` | `embedded_rules` |
| `turn_points_and_curve_points_are_not_written_on_their_own_layers` | `embedded_rules` |
| `coordinates_are_held_to_two_decimal_places` | `embedded_rules`, `all_contours` |
| `no_rules_so_a_receiver_can_make_no_size_outside_the_nest` | `all_contours` |
| `only_the_sample_carries_notches_and_labels` | `all_contours` |
| `the_cut_line_is_only_on_the_sample_since_its_vertices_differ_by_size` | `all_contours` |

Each record also says `receiver`, which is that none has read it. A test reads each file with a small reader of its own, finds every identifier on a point on its own layer, and moves each corner by the `.rul` file's rule to where the nest has that corner at each size, so two files that were not written from each other agree. Each mode's files are frozen by digest for three garments, in `conformance/goldens/grading/digests.tsv`, and two of the skirt's in full.

## Frozen, and what that catches

Every size of every garment is frozen through its canonical form, by both paths, along with the table between them, in `conformance/goldens/graded/families.tsv`. `bash scripts/check_golden_suites.sh` is one command for that and [the offset corpus](offsets.md), because an allowance and a graded point can each be wrong in a way the other does not see.

A suite is only worth what it catches, so `bash docs/tasks/artifacts/grading/prove_goldens_catch_a_mutation.sh` injects one mutation into a copy of the tree, runs the suite, and puts it back. **The result is recorded in `docs/tasks/artifacts/grading/mutation_proof.tsv`.** One mutation widens every allowance in the offset engine by a micrometre, and the other moves every graded point by a micrometre. It has to pass unmutated first, or the rest would mean nothing, and it stops and says so.

A refreeze is a decision, and the test prints the lines to review rather than taking any.

**What this does not show.** That these sizes fit anybody: they are what the recipe gives at those measurements, and a size chart nobody has fitted is a set of numbers. The steps are an engineer's and come from no standard. A test does measure each size from its drawing against what it asked for, which is what stops a family that closes on paper being wrong in the cloth, and it does that for three garments and only for the measurements named there.

## Names used in this chapter

| Name | Meaning |
| --- | --- |
| `regenerate` | The step that draws every size |
| `grade` | The step that moves the sample by a table |
| `G3-GRADING.8` | The leaf for a receiver reading the result |
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
| `grading` | How a draft says it grades |
| `stack` | Which corner a piece stacks on |
| `edge/to` | A corner, as the end of its edge |
| `table` | A table in the project |
| `table_text` | A table itself |
| `convention` | Which way to write it |
| `cumulative` | From the sample size |
| `incremental` | From the next size toward the sample |
| `rules_path` | Where to write the table |
| `rul` | Where to write the file |
| `walk` | Each seam's walk |
| `unmentioned` | Points the table does not mention |
| `unmatched` | Rules for points this design does not have |
| `notes` | What was only approximated |
| `conflicts` | Contours that were not to change, and did |
| `equivalence` | The two paths compared |
| `equivalence_path` | Where to write the whole report |
| `export_graded` | The step that writes one grading mode |
| `mode` | Which of the three |
| `semantics` | Which line a receiver reads first |
| `base_plus_rules` | Named layers and a `.rul` file |
| `embedded_rules` | Numbered layers, identifiers at the points, and a `.rul` file |
| `all_contours` | Every size's own contour, and no rules |
| `dxf-aama-named` | A registered target |
| `dxf-astm-num-r13` | A registered target |
| `# ` | What starts an identifier |
| `sew_as_1` | The net line on the first layer |
| `cut_as_1` | The cut line on the first layer |
| `Quantity:` | How many of a piece, right and left |
| `receiver` | Who has read it |
| `named_layers_merge_turn_points_curve_points_and_the_boundary` | A loss |
| `rules_only_on_the_points_listed_and_between_them_a_receiver_interpolates` | A loss |
| `stack_points_fixed_contours_and_declared_zeros_are_not_carried` | A loss |
| `the_recipe_is_not_carried` | A loss |
| `the_file_holds_identifiers_and_the_deltas_are_in_a_table_it_names` | A loss |
| `turn_points_and_curve_points_are_not_written_on_their_own_layers` | A loss |
| `coordinates_are_held_to_two_decimal_places` | A loss |
| `no_rules_so_a_receiver_can_make_no_size_outside_the_nest` | A loss |
| `only_the_sample_carries_notches_and_labels` | A loss |
| `the_cut_line_is_only_on_the_sample_since_its_vertices_differ_by_size` | A loss |
| `conformance/goldens/grading/digests.tsv` | Each mode's files, frozen by digest |
| `conformance/goldens/graded/families.tsv` | Every size by both paths, frozen by digest |
| `bash scripts/check_golden_suites.sh` | The offset corpus and the graded families together |
| `bash docs/tasks/artifacts/grading/prove_goldens_catch_a_mutation.sh` | The proof that the suite catches a mutation |
| `docs/tasks/artifacts/grading/mutation_proof.tsv` | What that proof found |
| `check_extremes` | The step that checks the two ends of the set |
| `through` | What read the file |
| `declared_domain` | A check |
| `offset_validity` | A check |
| `minimum_curvature_radius` | A check |
| `bounding_box_against_fabric` | A check |
| `seam_differential_against_ease` | A check |
| `notch_spacing` | A check |
| `dart_intake_against_maximum` | A check |
| `finished_dimension` | A check |
| `fails` | It is past its limit |
| `no_limit` | Nothing has declared what would be too much |
| `limits` | What a draft assumes a factory's limits are |
| `min_radius` | The tightest bend allowed |
| `fabric_width` | The cloth's width |
| `dart_intake` | The most one dart may take |
| `finished` | What to read off the drawing |
| `equals` | What it should come to |
| `seams` | Seams to add up |
| `side` | Which side of a seam to add up |
| `widest` | Pieces to measure across |
| `times` | How many the garment has |
| `provenance` | Where a table came from |
| `classes` | The tolerance classes used |
| `compared` | How many quantities |
| `not_compared` | Sizes the recipe cannot speak to |
| `findings` | Quantities beyond their class |
| `quantities` | All of them |
| `quantity` | What was compared |
| `by_recipe` | What regenerating gave |
| `by_rule` | What the rules gave |
| `difference_um` | The difference, in micrometres |
| `class` | What it is judged against |
| `verdict` | `holds` or `finding` |
| `holds` | It is within its class |
| `finding` | It is not |
| `owner` | Who a finding belongs to |
| `T1` | The numerical class |
| `T2` | The geometric class |
| `T4` | The importer's class |
| `T5` | The factory's class |
| `seam` | Twice `T2` |
