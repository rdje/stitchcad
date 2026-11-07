# The reference bodice and sleeve

A skirt's seams all match, so it cannot show the thing that makes a set-in sleeve hard: the sleeve's cap is meant to be longer than the armhole it is sewn into, by an amount somebody chose, and that difference has to be told apart from a mistake. So this garment exists to carry [a declared ease](walking-and-truing.md) on a real seam, and every number in this chapter has a source, as the [reference skirt's](reference-skirt.md) do.

**What works today.** A darted bodice front and back with a set-in sleeve, drafted through the recipe from body measurements. A cap fitted to its armhole by a declared ease at the front and the back. A bust dart that the side seam accounts for. Single and double notches, and a drill hole at the bust point. Five sizes, each of which draws, closes and walks. **What does not exist yet:** the [classic collar](../availability.md) that finishes the neck, which is `G3-GRADING.15`. Grading this garment by rule rather than by re-drafting each size, which is `G3-GRADING.7`. Any fit on a body: nobody has cut this in cloth, and every drafting constant below is an engineer's, unreviewed.

```rust
use sc_cli::Running;
use sc_store::{parse, MemoryRepository};

let whole = parse(include_str!("../../../conformance/reference-bodice/session.json")).expect("valid");
let fields = whole.as_object().expect("a session");
let mut disk = MemoryRepository::new();
let mut running = Running::open(&whole, &mut disk).expect("opens");
for step in fields["steps"].as_list().expect("steps") {
    running.step(step).expect("runs");
}

// The front of the cap is 12 mm longer than the armhole it meets, and somebody said it should be.
let walk = running.step(&parse(r#"{"do":"walk"}"#).expect("valid")).expect("walks");
let reports = walk.as_object().expect("a record")["result"].as_list().expect("reports");
let cap = reports.iter()
    .map(|report| report.as_object().expect("a report"))
    .find(|report| report["seam"].as_text() == Some("cap_front_right"))
    .expect("the seam");
assert_eq!(cap["ease"].as_int(), Some(12_000));
assert_eq!(cap["finding"].as_text(), Some("declared_ease"));
assert!(cap["unexplained"].as_int().expect("a length").abs() <= 20);
```

## 1. The garment

A fitted bodice with a one-piece long sleeve, at a base size that shares the [reference skirt's](reference-skirt.md) waist:

- a **front cut on the fold** at CF, with a bust dart from the side seam and a waist dart below the bust point;
- a **back as a mirrored pair** with a waist dart and a CB seam for a zipper. The back shoulder is longer than the front's and eased to it, in place of a shoulder dart;
- a **set-in sleeve** in one piece, a mirrored pair, taking its width from the armhole it has to fit.

## 2. Measurements

Declared fixture values, chosen as a mid-range base and not copied from any size standard. The table below these gives the sizes.

| Token | Value | What it is | Landmark or procedure |
| --- | --- | --- | --- |
| `bust_girth` | 940 mm | bust circumference | fullest part of the bust, tape level |
| `waist_girth` | 740 mm | waist circumference | natural waist, narrowest circumference |
| `nape_to_waist` | 410 mm | nape to waist | centre back, from the prominent neck bone to the waist |
| `neck_girth` | 370 mm | neck circumference | at the neck's base, through the front hollow and the nape |
| `shoulder_length` | 125 mm | shoulder length | neck side to the shoulder's end |
| `across_back` | 360 mm | across the back | between the arm creases, at about a third of the way from nape to underarm |
| `across_chest` | 340 mm | across the chest | between the arm creases, at the same level on the front |
| `scye_depth` | 220 mm | scye depth | nape down to underarm level |
| `waist_to_apex` | 170 mm | waist to bust point | straight up from the waist to the bust point's level |
| `apex_from_cf` | 90 mm | bust point from CF | half the distance between the bust points |
| `upper_arm_girth` | 290 mm | upper arm circumference | at the fullest part, relaxed |
| `wrist_girth` | 160 mm | wrist circumference | over the wrist bone |
| `sleeve_length` | 590 mm | sleeve length | shoulder's end to wrist, arm slightly bent |

Ease, per point of measure:

| Token | Value | From | Fit intent | What it gives |
| --- | --- | --- | --- | --- |
| `ease_bust` | 80 mm | `bust_girth` | `semi` | `garment_bust` |
| `ease_waist` | 40 mm | `waist_girth` | `semi` | `garment_waist` |
| `ease_upper_arm` | 40 mm | `upper_arm_girth` | the least it may be | `garment_upper_arm` |
| `ease_wrist` | 70 mm | `wrist_girth` | enough to pass a hand | `garment_wrist` |

## 3. The drafting's own constants

Named so that they can be reviewed and changed. They are not a reproduction of any drafting system, and the blocks of the [system the project named](formula-language.md) are `G3-GRADING.16`. All are `assumed`.

| Token | Value | Meaning |
| --- | --- | --- |
| `front_shift` | 5 mm | how much wider the front is than a quarter of the bust, and the back narrower |
| `side_suppress` | 25 mm | what each side seam takes in between the bust and the waist |
| `side_break` | 100 mm | how far up the side seam its slant stops and it runs straight |
| `bust_dart_intake` | 35 mm | what the bust dart takes up, and how much longer the front's side is for it |
| `bust_dart_gap` | 25 mm | how far the dart stops short of the bust point |
| `waist_dart_gap` | 20 mm | how far below the bust point the waist dart stops |
| `back_dart_len` | 140 mm | the back waist dart's height |
| `back_neck_rise` | 20 mm | how far the neck's side is above the nape |
| `front_neck_extra` | 10 mm | how much deeper the front neck is than a fifth of the neck |
| `front_slope` | 50 mm | how far the front shoulder falls |
| `back_slope` | 40 mm | how far the back shoulder falls |
| `shoulder_ease` | 6 mm | how much longer the back shoulder is, eased to the front |
| `cap_ease_front` | 12 mm | what the cap has beyond the front armhole, above its notch |
| `cap_ease_back` | 18 mm | what the cap has beyond the back armhole, above its notches |
| `under_span_front` | 75 mm | how far from the underarm the front notch is, on both sides of the seam |
| `under_span_back` | 85 mm | how far from the underarm the back notches are centred, on both sides |
| `balance_front` | 60 mm | how far above the front notch a balance notch sits, along the cap |
| `balance_back` | 60 mm | how far above the back notches a balance notch sits, along the cap |
| `zip_len` | 350 mm | how far down CB the zipper runs |

Allowances, each on its own edge:

| Token | Value | On |
| --- | --- | --- |
| `sa_side` | 15 mm | the side seams |
| `sa_shoulder` | 15 mm | the shoulders |
| `sa_armhole` | 10 mm | the armholes |
| `sa_neck` | 8 mm | the neck, until the collar |
| `sa_waist` | 15 mm | the waist |
| `sa_cb` | 20 mm | CB, for the zipper |
| `sa_cap` | 10 mm | the sleeve cap |
| `sa_under` | 15 mm | the sleeve's under-seam |
| `sa_sleeve_hem` | 30 mm | the sleeve hem |

## 4. Derived values

Every value here comes from the recipe, in `conformance/reference-bodice/session.json`. A test reads this table and holds each one to what the recipe evaluates to.

| Token | Value | Formula |
| --- | --- | --- |
| `garment_bust` | 1020 mm | `bust_girth + ease_bust` |
| `garment_waist` | 780 mm | `waist_girth + ease_waist` |
| `garment_upper_arm` | 330 mm | `upper_arm_girth + ease_upper_arm` |
| `garment_wrist` | 230 mm | `wrist_girth + ease_wrist` |
| `quarter_bust` | 255 mm | `garment_bust / 4` |
| `quarter_waist` | 195 mm | `garment_waist / 4` |
| `front_bust_x` | 260 mm | `quarter_bust + front_shift` |
| `back_bust_x` | 250 mm | `quarter_bust - front_shift` |
| `front_waist_side` | 235 mm | `front_bust_x - side_suppress` |
| `back_waist_side` | 225 mm | `back_bust_x - side_suppress` |
| `waist_dart_front` | 40 mm | `front_waist_side - quarter_waist` |
| `waist_dart_back` | 30 mm | `back_waist_side - quarter_waist` |
| `neck_width` | 74 mm | `neck_girth / 5` |
| `front_neck_drop` | 84 mm | `neck_girth / 5 + front_neck_extra` |
| `back_underarm_y` | 190 mm | `nape_to_waist - scye_depth` |
| `front_underarm_y` | 225 mm | `back_underarm_y + bust_dart_intake` |
| `back_neck_y` | 430 mm | `nape_to_waist + back_neck_rise` |
| `front_neck_y` | 465 mm | `back_neck_y + bust_dart_intake` |
| `front_cf_y` | 381 mm | `front_neck_y - front_neck_drop` |
| `front_shoulder_y` | 415 mm | `front_neck_y - front_slope` |
| `back_shoulder_y` | 390 mm | `back_neck_y - back_slope` |
| `back_shoulder_length` | 131 mm | `shoulder_length + shoulder_ease` |
| `front_shoulder_x` | 188.564 mm | neck_width + √(shoulder_length² − front_slope²) |
| `back_shoulder_x` | 198.744 mm | neck_width + √(back_shoulder_length² − back_slope²) |
| `average_arm_rise` | 195 mm | `the mean of each armhole's height` |
| `cap_height` | 156 mm | average_arm_rise × 4/5 |
| `sleeve_width` | 341.405 mm | `sleeve_front_width + sleeve_back_width` |
| `sleeve_taper` | 55.703 mm | (sleeve_width − garment_wrist) / 2 |
| `wrist_y` | -434 mm | cap_height − sleeve_length |

## 5. What is solved, and why

An armhole is a curve, so its length is only known once it is drawn, and a cap's length depends on its width. There is no formula that gives a width from a length here, so the drawing finds it. Two [fits](walking-and-truing.md) do: each varies one width until its seam walks to what it declares.

| Token | Value | Decided by | Between |
| --- | --- | --- | --- |
| `sleeve_front_width` | 168.979 mm | `cap_front_right` | `sleeve_width_low` and `sleeve_width_high` |
| `sleeve_back_width` | 172.426 mm | `cap_back_right` | `sleeve_width_low` and `sleeve_width_high` |

**The cap's height is chosen and its width is solved.** That is an engineer's choice. A patternmaker might fix the width from the arm and find the height instead, and one height would then have to suit two seams at once. So here the sleeve's width comes out of the armhole, and the recipe's `sleeve_clears_arm` is what stops it coming out too narrow for the arm: the sleeve here clears `garment_upper_arm` by 11.4 mm.

## 6. Closure checks

Each is an `assert` in the recipe, so a measurement that breaks one refuses the whole recipe by that name. Each holds at the solved widths, and need not hold at the values tried on the way there.

| Token | What it says | Why |
| --- | --- | --- |
| `bust_allocation` | `2 × (front_bust_x + back_bust_x) = garment_bust` | The four quarters are the bust |
| `waist_closure_front` | `front_waist_side − waist_dart_front = quarter_waist` | With the dart closed, the front waist is what it owes |
| `waist_closure_back` | `back_waist_side − waist_dart_back = quarter_waist` | The same at the back |
| `side_closure` | The front's straight run less `bust_dart_intake` is the back's | The side seams meet once the dart is closed |
| `shoulder_closure` | The back shoulder less `shoulder_ease` is the front's | Each is drawn from a length and a slope, so this is to T2 |
| `sleeve_clears_arm` | The sleeve is at least `garment_upper_arm` wide | An arm has to fit in it |
| `sleeve_taper_closure` | The two wrist positions add to `garment_wrist` | The wrist is what was asked for |

## 7. Pieces

| Piece | Quantity | What it is |
| --- | --- | --- |
| `bodice_front` | 1 | The front, cut on the fold, with `waist_dart` and `bust_dart` |
| `bodice_back_right` | 1 | The right back, with `waist_dart` |
| `bodice_back_left` | 1 | Its mirror |
| `sleeve_right` | 1 | The right sleeve. The front of the arm is at positive x |
| `sleeve_left` | 1 | Its mirror |

The edges of each are named, and everything that holds a place on one [holds it by that name](stable-references.md).

| Edge | On | Runs |
| --- | --- | --- |
| `waist` | Each bodice piece | CF or CB to the side |
| `side_lower` | Each bodice piece | Waist to `side_break`, slanting |
| `side_upper` | Each bodice piece | `side_break` to the underarm, straight |
| `armhole` | Each bodice piece | Underarm to the shoulder's end |
| `shoulder` | Each bodice piece | The shoulder's end to the neck |
| `neck` | Each bodice piece | The neck's side to CF or CB |
| `cf` | `bodice_front` | Neck to waist, with no allowance, on the fold |
| `cb` | The back | Nape to waist |
| `wrist` | Each sleeve | Back to front |
| `under_front` | Each sleeve | Wrist to the front underarm |
| `cap_front` | Each sleeve | The front underarm to the crown |
| `cap_back` | Each sleeve | The crown to the back underarm |
| `under_back` | Each sleeve | The back underarm to the wrist |

## 8. Sewing graph

Each armhole is two stretches, so that what the cap has to spare goes where it is wanted. Below the notches nothing is eased, because a sleeve eased under the arm pulls. Above them everything is, because that is what makes the room over the shoulder.

| Seam | Side A | Side B | Declares |
| --- | --- | --- | --- |
| `side_right`, `side_left` | The front's side, with the bust dart sewn away | The back's side | Nothing |
| `shoulder_right`, `shoulder_left` | The back's shoulder | The front's shoulder | `shoulder_ease` |
| `under_right`, `under_left` | `under_front` | `under_back` | Nothing |
| `underarm_front_right`, `underarm_front_left` | `cap_front` to its notch | The front `armhole` to its notch | Nothing |
| `cap_front_right`, `cap_front_left` | `cap_front`, notch to crown | The front `armhole`, notch to shoulder | `cap_ease_front` |
| `underarm_back_right`, `underarm_back_left` | `cap_back`, underarm to its notches | The back `armhole` to its notches | Nothing |
| `cap_back_right`, `cap_back_left` | `cap_back`, notches to crown | The back `armhole`, notches to shoulder | `cap_ease_back` |
| `cb` | The right back's CB | The left back's | Nothing |

Everything else is finished rather than sewn to another piece. A test holds this list against the edges, in both directions:

| Piece | Edge | Finished by |
| --- | --- | --- |
| `bodice_front` | `waist` | Whatever it is joined to, or a hem |
| `bodice_front` | `neck` | The collar, which is `G3-GRADING.15` |
| `bodice_front` | `cf` | The fold |
| `bodice_back` | `waist` | The same as the front |
| `bodice_back` | `neck` | The collar |
| `sleeve` | `wrist` | A hem |

## 9. Notch scheme

**One at the front, two at the back,** so that a sleeve cannot be set in the wrong way round without somebody seeing it. A `double` notch is two marks either side of the place it holds. Each stretch of the armhole has a balance notch too, [planted](walking-and-truing.md) at the same share of both sides, so the ease lies evenly between landmarks.

| Notch | On | Type | Where |
| --- | --- | --- | --- |
| `side` | Each bodice piece | `single` | Where the side stops slanting |
| `armhole_front` | `bodice_front` | `single` | `under_span_front` along `armhole` |
| `armhole_back` | The back | `double` | `under_span_back` along `armhole` |
| `front` | Each sleeve | `single` | `under_span_front` along `cap_front` |
| `back` | Each sleeve | `double` | `under_span_back` back from the end of `cap_back` |
| `crown` | Each sleeve | `single` | The top of the cap, meeting the shoulder seam |
| `zip` | The back | `single` | `zip_len` down `cb` |
| `bust_point` | `bodice_front` | `drill` | `apex_from_cf` in from `cf`, at the bust point's level |

## 10. Sizes

One step moves each measurement by the amount below. A test draws each size from the base by changing the measurements, then checks that it closes and walks. That is a re-draft for each size, and not grading, and a label on any of them still says M.

| Size | Steps from M |
| --- | --- |
| XS | −2 |
| S | −1 |
| M | 0 |
| L | 1 |
| XL | 2 |

| Measurement | One step |
| --- | --- |
| `bust_girth` | 40 mm change |
| `waist_girth` | 40 mm change |
| `nape_to_waist` | 5 mm change |
| `neck_girth` | 10 mm change |
| `shoulder_length` | 3 mm change |
| `across_back` | 10 mm change |
| `across_chest` | 10 mm change |
| `scye_depth` | 5 mm change |
| `waist_to_apex` | 5 mm change |
| `apex_from_cf` | 2.5 mm change |
| `upper_arm_girth` | 15 mm change |
| `wrist_girth` | 5 mm change |
| `sleeve_length` | 5 mm change |

## 11. The files

`conformance/reference-bodice/session.json` is this chapter as a session, and its files are frozen at `conformance/goldens/reference-bodice/`. `cargo test -p sc-cli --test bodice` holds it to this chapter. It measures the cap and the armhole a second way as well, by integrating each curve from its control points rather than by walking a drawn line.

## 12. What this does not show

**Whether it fits anybody.** Every constant in §3 is `assumed`, the domain seat in [governance](../governance.md) is vacant, and no toile has been made. So what is shown is that the garment is consistent: it closes, its seams meet where they say they do, and its ease is what it declares. A cap that is 30 mm longer than its armhole is not therefore a cap that sets well, and the split between front and back is an engineer's. The [physical protocol](physical-evaluation.md) says how to find out, and it applies here as it does to the skirt.

| Value | What to look at | What would show it is wrong |
| --- | --- | --- |
| `cap_ease_front` and `cap_ease_back` | Set the sleeve and press the cap | It puckers where it should ease, or lies flat with nothing to spare and pulls across the arm |
| `cap_height` | Worn: the arm hanging, then raised | It drags from the crown when hanging, or the bodice lifts when the arm comes up |
| `bust_dart_intake` | Worn: the side seam and the front's balance | It pulls toward the front, or a fold forms below the bust |
| `shoulder_ease` | Sew the shoulder | It will not ease in without a pucker, or it could have taken more |
| `ease_bust` | Worn, seated and reaching forward | It strains across the back or the bust |
| `apex_from_cf` and `waist_to_apex` | Worn: the bust point's mark against the body | The dart's tip points somewhere other than the fullest part |

The cap is one cubic on each side, which is a smoother shape than a patternmaker would draw and has no separate underarm curve. The armhole's shape is a cubic too, chosen to pass inside the across-back and across-chest widths rather than through them, so those two measurements only guide it. A dart's end is still [a position and not a reference](stable-references.md). The bust dart's closure leaves the side seam's two legs meeting without a drawn dart cap. Nothing here says how the neck or waist is finished, because those belong to the pieces that finish them.

## Names used in this chapter

| Name | Meaning |
| --- | --- |
| `G3-GRADING.15` | The leaf for the collar |
| `G3-GRADING.16` | The leaf for the reference blocks |
| `G3-GRADING.7` | The leaf for grade rules |
| `semi` | A fit intent: neither close nor loose |
| `assumed` | A value nobody has confirmed: the ontology's state |
| `sleeve_front_width` | How far the sleeve runs from its centre to the front underarm |
| `sleeve_back_width` | How far it runs to the back underarm |
| `sleeve_width_low` | The narrowest a fit may try |
| `sleeve_width_high` | The widest a fit may try |
| `assert` | A closure check in the recipe |
| `waist_dart` | The dart at a bodice piece's waist |
| `bust_dart` | The dart from the front's side seam toward the bust point |
| `single` | One mark |
| `double` | Two marks, either side of the place |
| `drill` | A hole inside the piece |
| `conformance/reference-bodice/session.json` | This chapter as a session |
| `conformance/goldens/reference-bodice/` | Its frozen files |
| `cargo test -p sc-cli --test bodice` | What holds it to this chapter |
