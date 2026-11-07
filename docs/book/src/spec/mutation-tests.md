# The mutation tests

A test suite proves little until it has been shown to fail. So each fault below is injected on purpose into the reference skirt, and each must be caught **by the check that names it**. A fault caught by some other check failing nearby fails the test, because a harness that passes for the wrong reason would keep passing once the right check had gone.

```text
$ cargo test -p sc-cli --test mutants
```

**What works today.** 31 mutants across the families the roadmap names, the judge that reads a drawn pattern and its files, and a control: the unmodified skirt raises nothing. **What does not exist yet:** grade mutants, which need grading at G3, and these checks as a command a person can run on their own pattern, since the only declaration written so far is the reference skirt's.

## The judge

`judge_inputs`, `judge_pattern`, `judge_dxf` and `judge_pdf_square` read what they are given against what a fixture declares, and each `Finding` has a stable code and names what it found. They share no code with the writers, and they read the DXF and PDF as text.

```rust
use sc_artifacts::{judge_inputs, Expectations};

let declared = Expectations::from_text(r#"{
  "inputs": [{"token": "hip_girth", "um": 980000}],
  "plausible_factor": 2, "tolerance": 10, "notch_tolerance": 1000,
  "pieces": [], "spans": []
}"#).expect("a declaration");

// 98 cm, as declared, raises nothing.
assert!(judge_inputs(&[("hip_girth".to_owned(), 980_000)], &declared).is_empty());

// 98 cm typed as 98 mm is named as a unit slip, and it says which.
let found = judge_inputs(&[("hip_girth".to_owned(), 98_000)], &declared);
assert_eq!(found[0].code, "wrong_units");
assert!(found[0].detail.contains("centimetres read as millimetres"));
```

`conformance/reference-skirt/expectations.json` is that declaration for the whole skirt, taken from [its chapter](reference-skirt.md): each piece's size, notches, labels and grain, and each seam of §8 with where its notches must land.

The harness runs in one order and reports the first thing anything said: the inputs, then the steps of the session, then the drawing, then the files.

## Every mutant, and what must catch it

| Mutant | Family | Caught by | The fault |
| --- | --- | --- | --- |
| `hip_girth_as_millimetres` | wrong units | `wrong_units` | 98 cm typed as 98 mm |
| `hip_girth_ten_times` | wrong units | `wrong_units` | 98 cm typed as 980 cm |
| `waist_to_hem_in_inches` | wrong units | `wrong_units` | 24.41 inches typed as millimetres |
| `notch_literal_in_centimetres` | wrong units | `wrong_notch_depth` | The recipe says 5 cm where it means 5 mm |
| `dxf_written_in_inches` | wrong units | `wrong_units` | Every coordinate in the DXF divided by 25.4 |
| `pdf_square_shrunk` | wrong units | `scale_square_wrong` | The printed scale square drawn at 99 mm |
| `front_hip_notch_missing` | missing notch | `missing_notch` | The front loses its hip notch |
| `zipper_notch_missing` | missing notch | `missing_notch` | The back loses its zipper-stop notch |
| `band_quarter_notch_missing` | missing notch | `missing_notch` | The band loses a quarter notch |
| `hip_notch_ten_millimetres_off` | missing notch | `notch_misaligned` | One back hip notch sits 10 mm up the seam |
| `boundary_does_not_close` | invalid boundary | `draft` | A boundary whose last edge misses its start |
| `boundary_runs_clockwise` | invalid boundary | `draft` | A boundary running the wrong way, so outward is unknown |
| `boundary_crosses_itself` | invalid boundary | `draft` | A boundary tied in a bow |
| `allowance_is_negative` | invalid boundary | `implausible_input` | A seam allowance of −1 cm |
| `waist_construction_wrong` | invalid boundary | `recipe` | The waist point built from the waist, not the hip: defect D33 |
| `side_seam_sewn_end_for_end` | reversed seam | `reversed_correspondence` | A side seam sewn hem to waist. Its lengths still agree |
| `centre_back_sewn_end_for_end` | reversed seam | `reversed_correspondence` | The centre back sewn end for end |
| `side_seam_sewn_to_the_wrong_edge` | reversed seam | `seam_length` | A side seam sewn to the centre back |
| `interfacing_omitted` | omitted piece | `omitted_piece` | The interfacing is not drafted |
| `back_left_omitted` | omitted piece | `omitted_piece` | One back is not drafted |
| `front_drafted_twice` | omitted piece | `duplicate_piece` | A piece drafted twice |
| `a_piece_nobody_asked_for` | omitted piece | `unexpected_piece` | A sixth piece |
| `a_piece_nobody_sews` | omitted piece | `unaccounted_piece` | The band in no seam: defect D27 |
| `band_label_says_fold` | wrong marking | `label_forbidden` | The band labelled "place on fold", the glossary's hazard |
| `front_label_loses_fold` | wrong marking | `label_missing` | The front not labelled "place on fold" |
| `band_fold_missing` | wrong marking | `fold_not_internal` | The band loses its fold line |
| `grain_turned_across` | wrong marking | `grain_off` | A grainline turned across the piece |
| `cut_quantity_wrong` | wrong marking | `wrong_quantity` | The band cut twice |
| `hem_allowance_a_millimetre_off` | wrong dimension | `wrong_dimension` | A hem 1 mm deeper than the fixture |
| `allowance_on_the_fused_piece` | wrong dimension | `allowance_on_fused_piece` | An allowance on a piece that is fused |
| `cut_and_sew_swapped_in_the_file` | wrong dimension | `cut_and_sew_swapped` | The cut and sew lines on each other's DXF layers |

## Why a reversed seam needs a notch

Sewing a side seam end for end leaves both lengths equal, so a length check alone cannot see it, and a test shows exactly that with the notch taken out of the declaration. What catches it is the hip notch: walked the wrong way, it lands 421 mm from the wrong end. This is why [the skirt's chapter](reference-skirt.md) marks each seam.

A notch sits on the cut line, one allowance out from the seam. On a slanted seam it therefore projects short of the corner it marks, by the width times the sine of the slant, which is 0.71 mm on this skirt. So `notch_tolerance` is 1 mm, for that reason and not because a smaller number failed.

## What building this found

| Finding | Before | Now |
| --- | --- | --- |
| The wrong fault named | A boundary tied in a bow was refused as running the wrong way | Crossing is checked first, because a crossing boundary has no one direction |
| One pattern, two files | A closed line started wherever it was drawn from, so the same pattern reached two ways wrote different DXF and PDF bytes (D398) | Both writers use the canonical first point, and a test rotates a line's start |
| A closure blind to the geometry | The session's `waist_closure` re-derived the arithmetic, and the D33 fault passed it (D399) | It reads `waist_side`, the point the geometry uses |
| A nameless refusal | A boundary that did not close did not say which piece | It names the piece |
| A harness catching the wrong thing | A sixth piece was refused because the sheet was full, before anything looked at it | The drawing is judged before any writer runs |

## What this does not show

- **The list is not everything.** These are the faults the roadmap names and the hazards the book marks. A fault nobody thought of is not on it.
- **The seams are a table for now.** The sewing graph is not data until G3, so the seams are the fixture's, and the walk here is the judge's own and not the `walk` command.
- **A unit slip inside a factor of 2 is not caught at the input.** `plausible_factor` is an engineer's choice. It would still show as `wrong_dimension`, because the fixture's sizes are declared, but a garment with no declaration has neither check.
- **Only one garment.** A wrong answer on another garment proves nothing here, and G3's garments will each need a declaration.

## Names used in this chapter

| Name | Meaning |
| --- | --- |
| `judge_inputs` | Reads a session's inputs against the fixture's |
| `judge_pattern` | Reads a drawing against the fixture: pieces, then seams |
| `judge_dxf` | Reads a written DXF against its pattern |
| `judge_pdf_square` | Reads a written PDF's scale square |
| `Finding` | A code, and what was found |
| `notch_tolerance` | How far a notch may land from where its seam declares it |
| `plausible_factor` | How far an input may stray from the fixture before it is implausible |
| `waist_closure` | The closure that the waist the geometry drew is the waist the panel owes |
| `waist_side` | The waist point the geometry uses |
| `walk` | The command §8 of the skirt's chapter specifies, not yet built |
| `allowance_on_fused_piece` | A finding, or a refused step of that name |
| `cut_and_sew_swapped` | A finding, or a refused step of that name |
| `draft` | A finding, or a refused step of that name |
| `duplicate_piece` | A finding, or a refused step of that name |
| `fold_not_internal` | A finding, or a refused step of that name |
| `grain_off` | A finding, or a refused step of that name |
| `implausible_input` | A finding, or a refused step of that name |
| `label_forbidden` | A finding, or a refused step of that name |
| `label_missing` | A finding, or a refused step of that name |
| `missing_notch` | A finding, or a refused step of that name |
| `notch_misaligned` | A finding, or a refused step of that name |
| `omitted_piece` | A finding, or a refused step of that name |
| `recipe` | A finding, or a refused step of that name |
| `reversed_correspondence` | A finding, or a refused step of that name |
| `scale_square_wrong` | A finding, or a refused step of that name |
| `seam_length` | A finding, or a refused step of that name |
| `unaccounted_piece` | A finding, or a refused step of that name |
| `unexpected_piece` | A finding, or a refused step of that name |
| `wrong_dimension` | A finding, or a refused step of that name |
| `wrong_notch_depth` | A finding, or a refused step of that name |
| `wrong_quantity` | A finding, or a refused step of that name |
| `wrong_units` | A finding, or a refused step of that name |
