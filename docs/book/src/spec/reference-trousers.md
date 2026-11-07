# The reference trousers

The second jump after the skirt, and the last garment the [roadmap names](../availability.md). Trousers bring a crotch curve, an inseam that two differently shaped pieces have to share, and a waistband that closes with a button. They also carry the two constructions this garment is here to prove: a **pocket** that is an object and not lines on a piece, and a **buttonhole** whose length comes from its button and has no way of being written down twice. Every number has a source, as [the skirt's](reference-skirt.md) and [the bodice's](reference-bodice.md) do.

**What works today.** A front and back in mirrored pairs, a straight waistband and a pair of in-seam side pockets, drafted through the recipe from body measurements. An inseam fitted so the two pieces meet. A pocket and a button pair that the canonical form lists by name. Five sizes, each of which draws, closes and walks. **What does not exist yet:** a fly, which is `env_fly` and refused, so these close at CB with a zipper. Grading by rule rather than re-drafting each size, which is `G3-GRADING.7`. Any fit on a body: nobody has cut this in cloth.

```rust
use sc_cli::Running;
use sc_store::{parse, MemoryRepository};

let whole = parse(include_str!("../../../conformance/reference-trousers/session.json")).expect("valid");
let fields = whole.as_object().expect("a session");
let mut disk = MemoryRepository::new();
let mut running = Running::open(&whole, &mut disk).expect("opens");
for step in fields["steps"].as_list().expect("steps") {
    running.step(step).expect("runs");
}
let pattern = running.pattern().expect("drawn");

// The hole is the button and what its policy adds, and nothing else can set it.
let pair = &pattern.buttons[0];
assert_eq!(pair.size, 18_000);
assert_eq!(pair.hole_length, 23_000);

// A pocket is a thing with a place, a direction, an opening and pieces.
let pocket = &pattern.pockets[0];
assert_eq!(pocket.opening, "in_seam");
assert_eq!(pocket.pieces, ["pocket_bag_left"]);
```

## 1. The garment

Straight-legged trousers at a base size that shares [the skirt's](reference-skirt.md) waist and hip:

- a **front and a back** as mirrored pairs, with one waist dart in each;
- a **CB zipper**, because a fly is outside the envelope and is refused rather than approximated;
- a **straight waistband**, cut once and folded along its length, with an extension at CB for the button;
- a **pair of in-seam side pockets**, each two layers of one bag piece, opening on the side seam.

## 2. Measurements

Declared fixture values, chosen as a mid-range base and not copied from any size standard.

| Token | Value | What it is | Landmark or procedure |
| --- | --- | --- | --- |
| `waist_girth` | 740 mm | waist circumference | natural waist, narrowest circumference |
| `hip_girth` | 980 mm | hip circumference | fullest seat, at the waist-to-hip level |
| `waist_to_hip` | 200 mm | waist to hip | down the side, from the waist to the hip level |
| `crotch_depth` | 270 mm | crotch depth | seated on a flat surface, from the waist to the surface, at the side |
| `waist_to_hem` | 1020 mm | waist to hem | down the side, from the waist to where the hem should fall |
| `thigh_girth` | 580 mm | thigh circumference | at the fullest part of the thigh |

Ease, per point of measure:

| Token | Value | From | Fit intent | What it gives |
| --- | --- | --- | --- | --- |
| `ease_waist` | 10 mm | `waist_girth` | `close` | `garment_waist` |
| `ease_hip` | 40 mm | `hip_girth` | `semi` | `garment_hip` |
| `ease_thigh` | 40 mm | `thigh_girth` | the least it may be | `garment_thigh` |

## 3. The drafting's own constants

Named so that they can be reviewed and changed, and all `assumed`.

| Token | Value | Meaning |
| --- | --- | --- |
| `front_shift` | 10 mm | how much narrower the front is than a quarter of the hip, and the back wider |
| `side_suppress` | 20 mm | what each side seam takes in between the hip and the waist |
| `cf_in` | 10 mm | how far the front rise leans in at the waist |
| `cb_in` | 30 mm | how far the back rise leans in at the waist |
| `back_rise` | 25 mm | how far the back waist rises at CB |
| `side_taper` | 40 mm | how far in the side seam comes between the hip and the hem |
| `hem_front` | 200 mm | the front's width at the hem |
| `hem_back` | 240 mm | the back's width at the hem |
| `front_dart_len` | 90 mm | the front dart's length |
| `back_dart_len` | 120 mm | the back dart's length |
| `wb_width` | 40 mm | the finished band's height |
| `wb_extension` | 35 mm | how far the band runs on past CB for the button |
| `pocket_from_waist` | 30 mm | how far below the waist the pocket opens |
| `pocket_opening` | 160 mm | how long the opening is |
| `bag_width` | 170 mm | how far across the bag runs |
| `bag_depth` | 60 mm | how far the bag runs on below the opening |
| `zip_len` | 180 mm | how far down CB the zipper runs |
| `notch_depth` | 5 mm | how deep a notch is |
| `button_diameter` | 18 mm | the button's size |
| `button_thickness` | 3 mm | what its thickness adds to the hole |
| `hole_clearance` | 2 mm | what the hole adds to get past it |
| `hole_from_end` | 12 mm | how far the hole starts from the end of the band |

Allowances, each on its own edge:

| Token | Value | On |
| --- | --- | --- |
| `sa_side` | 15 mm | the side seams |
| `sa_inseam` | 15 mm | the inseams |
| `sa_rise` | 10 mm | the front rise and each crotch curve |
| `sa_cb` | 20 mm | the back rise, for the zipper |
| `sa_waist` | 10 mm | the waist and the band's lower edge |
| `sa_hem` | 40 mm | the hems |
| `sa_band` | 10 mm | the band's other edges |
| `sa_bag` | 10 mm | the pocket bag |

## 4. Derived values

Every value here comes from the recipe, in `conformance/reference-trousers/session.json`. A test reads this table and holds each one to what the recipe evaluates to.

| Token | Value | Formula |
| --- | --- | --- |
| `garment_waist` | 750 mm | `waist_girth + ease_waist` |
| `garment_hip` | 1020 mm | `hip_girth + ease_hip` |
| `garment_thigh` | 620 mm | `thigh_girth + ease_thigh` |
| `quarter_waist` | 187.5 mm | `garment_waist / 4` |
| `quarter_hip` | 255 mm | `garment_hip / 4` |
| `front_hip_x` | 245 mm | `quarter_hip - front_shift` |
| `back_hip_x` | 265 mm | `quarter_hip + front_shift` |
| `front_extension` | 63.75 mm | `garment_hip / 16` |
| `back_extension` | 127.5 mm | `garment_hip / 8` |
| `front_waist_side` | 225 mm | `front_hip_x - side_suppress` |
| `back_waist_side` | 245 mm | `back_hip_x - side_suppress` |
| `front_waist_run` | 215 mm | `front_waist_side - cf_in` |
| `back_waist_run` | 215 mm | `back_waist_side - cb_in` |
| `back_waist_length` | 216.449 mm | √(back_waist_run² + back_rise²) |
| `front_dart` | 27.5 mm | `front_waist_run - quarter_waist` |
| `back_dart` | 28.949 mm | `back_waist_length - quarter_waist` |
| `front_side_hem_x` | 205 mm | `front_hip_x - side_taper` |
| `back_side_hem_x` | 225 mm | `back_hip_x - side_taper` |
| `front_inner_hem_x` | 5 mm | `front_side_hem_x - hem_front` |
| `back_inner_hem_x` | -15 mm | `back_side_hem_x - hem_back` |
| `thigh_width` | 701.25 mm | `front_hip_x + front_extension + back_hip_x + back_extension` |
| `band_length` | 785 mm | `garment_waist + wb_extension` |
| `band_height` | 80 mm | `2 * wb_width` |
| `hole_length` | 23 mm | `button_diameter + button_thickness + hole_clearance` |
| `button_x` | 762 mm | `garment_waist + hole_from_end` |
| `pocket_end` | 190 mm | `pocket_from_waist + pocket_opening` |

## 5. What is solved

The front and back share an inseam, and they reach it from different places: the back's crotch extension is twice the front's. So the two lengths meet only if the back's crotch point drops a little below the front's. A [fit](walking-and-truing.md) finds how far.

| Token | Value | Decided by | Between |
| --- | --- | --- | --- |
| `back_crotch_drop` | 5.308 mm | `inseam_right` | `crotch_drop_low` and `crotch_drop_high` |

**The side seams need no fit.** Each piece comes in by the same `side_suppress` and goes out by the same `side_taper` over the same heights, so they match by construction.

## 6. A pocket

A draft's `pockets` list holds each one as an object, because a pocket drawn as lines on a piece is [a substitution the receiver cannot see](feature-matrix.md). It says what the [ontology](ontology.md) says a pocket is.

| Field | What it holds |
| --- | --- |
| `name` | The pocket's name |
| `piece` | The piece it serves |
| `edge` | The edge it opens on, with `at`, `along` or `before_end` for where the opening starts |
| `length` | How long the opening is, by name. It runs on along the edge, or back toward its start when the place is written with `before_end` |
| `opening` | How it opens. `in_seam` is the one that can be built today, and any other is refused by name |
| `pieces` | The pieces it is made of, each of which has to exist |

Its place is [a reference](stable-references.md), as `pocket side_pocket_right, from` and `pocket side_pocket_right, to`. So an edit to the side seam keeps it on the same cloth, and a delete makes a repair task and blocks a cutting file. The canonical form lists each pocket with where its opening starts and stops, which gives its position and its orientation. Each layer of the bag is sewn to the side seam along the opening, and those are seams like any other, so they walk.

## 7. A button and its hole

A draft's `buttons` list holds each pair.

| Field | What it holds |
| --- | --- |
| `name` | The pair's name |
| `size` | The button's size, by name |
| `policy` | What the hole adds to the size, as a list of names |
| `button` | The `piece` the button is on, and `at` for where it goes |
| `hole` | The `piece` the hole is in, `at` for where it starts, and `toward` for which way it runs |

**There is no length for a hole.** It is the size plus the policy, worked out when drawn, and a draft that gives one is refused. So there is one number to change and nothing else to forget. A test changes the button and reads the new hole from the drawing. The policy here is an engineer's, since the ontology deliberately does not prescribe one: size, thickness and a clearance.

The hole is written as an internal line, since it is cut after stitching, and the button as a drill mark. Either one outside its piece's sew line is refused by name.

## 8. Closure checks

Each is an `assert` in the recipe, so a measurement that breaks one refuses the whole recipe by that name.

| Token | What it says | Why |
| --- | --- | --- |
| `hip_allocation` | `2 × (front_hip_x + back_hip_x) = garment_hip` | The four quarters are the hip |
| `front_waist_closure` | `front_waist_run − front_dart = quarter_waist` | With the dart closed, the front waist is what it owes |
| `back_waist_closure` | The back's slanted waist less `back_dart` is `quarter_waist` | The same at the back, to T2 |
| `band_closure` | `band_length − wb_extension = garment_waist` | The band is the waist it is sewn to |
| `thigh_clears` | `thigh_width` is at least `garment_thigh` | A thigh has to fit in it |
| `button_on_extension` | The button stops short of the band's end | It has somewhere to go |

## 9. Pieces

| Piece | Quantity | What it is |
| --- | --- | --- |
| `trouser_front_right` | 1 | The right front, with `dart` |
| `trouser_front_left` | 1 | Its mirror |
| `trouser_back_right` | 1 | The right back, with `dart` |
| `trouser_back_left` | 1 | Its mirror |
| `waistband` | 1 | The band, cut once and folded along `wb_width` |
| `pocket_bag_right` | 2 | Both layers of the right pocket |
| `pocket_bag_left` | 2 | Both layers of the left pocket |

| Edge | On | Runs |
| --- | --- | --- |
| `rise` | Each front and back | The waist to the hip level, at CF or CB |
| `crotch` | Each front and back | The hip level to the crotch point |
| `inseam` | Each front and back | The crotch point to the hem |
| `hem` | Each front and back | Across the bottom of the leg |
| `side_lower` | Each front and back | The hem to the hip |
| `side_upper` | Each front and back | The hip to the waist, curving in |
| `waist` | Each front and back | The side to CF or CB |
| `band_lower` | `waistband` | The edge sewn to the waist |
| `band_right` | `waistband` | The end of the extension |
| `band_upper` | `waistband` | The edge folded inside |
| `band_left` | `waistband` | The end that overlaps |
| `mouth` | Each bag | The opening |
| `bag_lower` | Each bag | Round the bottom of the bag |
| `bag_top` | Each bag | Across the top of the bag |

## 10. Sewing graph

| Seam | Side A | Side B | Declares |
| --- | --- | --- | --- |
| `side_right`, `side_left` | The front's side | The back's side | Nothing |
| `inseam_right`, `inseam_left` | The front's `inseam` | The back's `inseam` | Nothing |
| `front_rise` | The right front's `rise` and `crotch` | The left front's | Nothing |
| `back_rise` | The right back's `rise` and `crotch` | The left back's | Nothing |
| `pocket_front_right`, `pocket_front_left` | One layer's `mouth` | The front's side, along the opening | Nothing |
| `pocket_back_right`, `pocket_back_left` | The other layer's `mouth` | The back's side, along the opening | Nothing |
| `band_back_left` | `band_lower`, from its start | The left back's `waist`, with the dart sewn away | Nothing |
| `band_front_left` | `band_lower`, on from there | The left front's `waist` | Nothing |
| `band_front_right` | `band_lower`, on from there | The right front's `waist` | Nothing |
| `band_back_right` | `band_lower`, as far as the extension | The right back's `waist` | Nothing |

Everything else is finished rather than sewn to another piece. A test holds this list against the edges, in both directions:

| Piece | Edge | Finished by |
| --- | --- | --- |
| `trouser_front` | `hem` | A hem |
| `trouser_back` | `hem` | A hem |
| `waistband` | `band_right` | Folded and stitched closed |
| `waistband` | `band_upper` | Folded inside |
| `waistband` | `band_left` | Folded and stitched closed |
| `pocket_bag` | `bag_lower` | The bag's other layer |
| `pocket_bag` | `bag_top` | The bag's other layer |

## 11. Notch scheme

| Notch | On | Type | Where |
| --- | --- | --- | --- |
| `hip` | Each front and back | `single` | Where the side stops curving |
| `pocket_top` | Each front and back | `single` | `pocket_from_waist` back from the end of `side_upper` |
| `pocket_bottom` | Each front and back | `single` | `pocket_end` back from the end of `side_upper` |
| `knee` | Each front and back | `single` | Halfway down `inseam` |
| `zip` | Each back | `double` | `zip_len` down `rise` |
| `side_left` | `waistband` | `single` | Where the left side seam meets it |
| `cf` | `waistband` | `single` | Where CF meets it |
| `side_right` | `waistband` | `single` | Where the right side seam meets it |
| `cb` | `waistband` | `single` | Where the extension starts |

## 12. Sizes

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
| `waist_girth` | 40 mm change |
| `hip_girth` | 40 mm change |
| `waist_to_hip` | 5 mm change |
| `crotch_depth` | 5 mm change |
| `waist_to_hem` | 10 mm change |
| `thigh_girth` | 20 mm change |
| `hem_front` | 5 mm change |
| `hem_back` | 5 mm change |

## 13. The files

`conformance/reference-trousers/session.json` is this chapter as a session, and its files are frozen at `conformance/goldens/reference-trousers/`. `cargo test -p sc-cli --test trousers` holds it to this chapter.

## 14. What this does not show

**Whether it fits anybody.** Every constant in §3 is `assumed`, the domain seat in [governance](../governance.md) is vacant, and no toile has been made. What is shown is that the garment is consistent: it closes, its seams meet where they say they do, its pocket and button are what they say they are. The [physical protocol](physical-evaluation.md) says how to find out.

| Value | What to look at | What would show it is wrong |
| --- | --- | --- |
| `crotch_depth` and `back_rise` | Worn, standing and seated | It drags at the front or the seat, or cuts in when seated |
| `front_extension` and `back_extension` | Worn: the crotch and the seat | It pulls across the seat, or there is a fold under it |
| `side_suppress` and `front_dart` | Worn: the waist | It gaps at CB, or the hip pulls toward the side |
| `pocket_opening` and `bag_depth` | A hand in the pocket | The hand does not fit, or the bag shows or pulls at the hem of the hip |
| `hole_clearance` | The real button, through the sewn hole | It will not go through, or it will not stay |

The bag's mouth is straight and the seam it is sewn to has a slight curve, so they agree in length and not in shape. The inseam is one straight line with no shape at the knee, and the crease line is not drawn. A dart's end, a button and a hole are [positions and not references](stable-references.md). **The two models are still apart:** this is in the drafting layer, and the core's structural `Pocket` and `ButtonAndButtonhole` describe the same things with no coordinates.

## Names used in this chapter

| Name | Meaning |
| --- | --- |
| `env_fly` | What asking for a fly says |
| `G3-GRADING.7` | The leaf for grade rules |
| `close` | A fit intent: little room |
| `semi` | A fit intent: neither close nor loose |
| `assumed` | A value nobody has confirmed: the ontology's state |
| `back_crotch_drop` | How far below the front's the back's crotch point is |
| `crotch_drop_low` | The smallest the fit may try |
| `crotch_drop_high` | The largest the fit may try |
| `pockets` | A draft's list of pockets |
| `at` | A fraction of an edge, or where a mark goes |
| `along` | A distance from an edge's start |
| `before_end` | A distance back from an edge's end |
| `in_seam` | A pocket opening in a seam |
| `pocket side_pocket_right, from` | A carrier, as a pocket's start is named |
| `pocket side_pocket_right, to` | A carrier, as its stop is named |
| `buttons` | A draft's list of button pairs |
| `size` | A button's size |
| `policy` | What a hole adds to its button |
| `button` | Where a button goes |
| `hole` | Where its hole is |
| `toward` | Which way a hole runs |
| `assert` | A closure check in the recipe |
| `dart` | A piece's waist dart |
| `single` | One mark |
| `double` | Two marks |
| `conformance/reference-trousers/session.json` | This chapter as a session |
| `conformance/goldens/reference-trousers/` | Its frozen files |
| `cargo test -p sc-cli --test trousers` | What holds it to this chapter |
| `Pocket` | The core's structural pocket |
| `ButtonAndButtonhole` | The core's structural button pair |
