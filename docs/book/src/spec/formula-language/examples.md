# Worked examples over the reference skirt

> One part of [the formula language](../formula-language.md). Every value below is the
> [reference skirt](../reference-skirt.md)'s, and every row is executed by the census that chapter's
> §8 names — the numbers are re-derived by an evaluator written from the
> [grammar](grammar.md), never typed twice. A row that disagrees with the fixture is a refusal, so
> the two chapters cannot drift into describing two garments (the D27 defect class).

## 1. Names these examples bind that the fixture does not

| Name | Kind | Origin | Binding | State |
| --- | --- | --- | --- | --- |
| `dart_intake_max` | length | `parameter` | `5.0 cm` | `assumed` ([the contract §8](../formula-language.md)) |
| `shrinkage` | ratio | `material` | none — nobody has measured this fabric | `unknown` ([the contract §8](../formula-language.md)) |
| `cb_seam` | edge | `geometry` | `len = waist_to_hem` | `derived` |
| `dart_apex_front` | point | `geometry` | `x = front_dart_centre`, `y = - dart_len_front` | `derived` |

`dart_apex_front` is in the fixture's drafting frame, where the waist line is `y = 0` and the hem is
negative ([fixture §5](../reference-skirt.md) step 1). Every other name these examples read is one
the fixture chapter declares: a measurement (§2 there), a drafting constant (§3), a derived value
(§4) or an allowance (§7).

The two geometry names are constructed by operations, and an operation's argument formulas are
evaluated at the operation's own position in the recipe — so `dart_apex_front`'s coordinates are
computed where §2 first reads them, after `front_dart_centre` is bound, and not before. That is
[the contract](../formula-language.md) §4.1's declaration order, applied to an operation rather than
to a `let`.

## 2. Bindings

Each row is the statement `let <token>: <kind> = <expression>` of [grammar §1](grammar.md).

| Token | Kind | Expression | Value | Shows |
| --- | --- | --- | --- | --- |
| `garment_waist` | length | `waist_girth + ease_waist` | 74.0 cm | a measurement plus its ease, both lengths |
| `garment_hip` | length | `hip_girth + ease_hip` | 102.0 cm | the same, where the ease is not zero |
| `quarter_waist` | length | `garment_waist / 4` | 18.5 cm | length ÷ count, the count being a repetition |
| `quarter_hip` | length | `garment_hip / 4` | 25.5 cm | the same |
| `suppression` | length | `quarter_hip - quarter_waist` | 7.0 cm | what one quadrant must absorb |
| `hip_to_hem_drop` | length | `waist_to_hem - waist_to_hip` | 42.0 cm | a vertical drop between two landmarks |
| `front_dart_centre` | length | `(quarter_hip - ss_suppress) / 2` | 11.25 cm | the D33 correction: suppression comes off the hip |
| `side_seam_length` | length | `hypot(hip_to_hem_drop, a_line_flare)` | 42.107 cm | the one irrational binding, displayed `√(hip_to_hem_drop² + a_line_flare²)` |
| `quarter_hem_width` | length | `quarter_hip + a_line_flare` | 28.5 cm | one quadrant's hem width, flare included |
| `garment_hem` | length | `4 * quarter_hem_width` | 114.0 cm | a repetition count over four quadrants |
| `waistband_cut_width` | length | `2 * wb_width + sa_waist + sa_wb_bottom` | 10.0 cm | the D27 formula: one band, folded, so two layers |
| `waistband_pattern_length` | length | `garment_waist + 2 * sa_cb + wb_extension` | 80.0 cm | a profile-resolved allowance inside a formula |
| `dart_leg_angle` | angle | `atan2(dart_intake / 2, dart_len_front)` | 9.462322 deg | an angle derived from two lengths |
| `dart_intake_share` | ratio | `dart_intake / suppression` | 0.571429 | length ÷ length is a ratio, at the 10⁻⁶ quantum |
| `zip_notch_param` | ratio | `param_at(cb_seam, zip_len)` | 0.290323 | a notch placed by parameter, so it survives an edit |
| `dart_apex_depth` | length | `abs(y(dart_apex_front))` | 12.0 cm | reading a point an operation constructed |
| `dart_count` | count | `if(dart_intake > dart_intake_max, 2, 1)` | 1 | a conditional; the fixture's one dart per quadrant, derived |

Twelve of these names are ones the fixture chapter derives too, so each is checked twice: against
the value published here and against the value published there.

## 3. Assertions — the fixture's four oracles, in the language

| Assertion | Class | Verdict |
| --- | --- | --- |
| `assert allocation_balance: eps_num = 4 * (ss_suppress + dart_intake) == garment_hip - garment_waist` | T1 | holds — 28.0 cm both sides |
| `assert waist_closure: eps_num = (quarter_hip - ss_suppress) - dart_intake == quarter_waist` | T1 | holds — 18.5 cm both sides |
| `assert waistband_length_closure: eps_num = waistband_pattern_length - 2 * sa_cb - wb_extension == garment_waist` | T1 | holds — 74.0 cm both sides |
| `assert waistband_width_closure: eps_num = waistband_cut_width - sa_waist - sa_wb_bottom == 2 * wb_width` | T1 | holds — 8.0 cm both sides |

These are the checks [fixture §4.1](../reference-skirt.md) declares, and both readings agree here
because every value in them is exact in the internal unit: no irrational call, so `==` and
`within(…, eps_num)` give one verdict. A recipe whose two sides are independently rounded must use
`within`; one that compares them with `==` is asking for a defect report the first time a
measurement changes. Two of the four exist because a balance over declared quantities cannot see a
constructed point — the D33 lesson, which is why the language carries `assert` at all.

## 4. Refusals

| Refusal | Diagnostic | Why |
| --- | --- | --- |
| `waist_girth + 2.5` | `formula_dimension` | length + ratio: a bare decimal is a scale factor, so the unit is missing |
| `dart_intake / 0` | `formula_division` | the divisor is zero |
| `sqrt(hip_to_hem_drop)` | `formula_dimension` | `sqrt` takes an area or a ratio, not a length |
| `dart_leg_angle * dart_len_front` | `formula_dimension` | an angle times a length has no kind; the diagnostic names `arc_length` |
| `- dart_count` | `formula_dimension` | a count has no negation: the operator table gives no rule for it |
| `let dart_intake_share: ratio = suppression / dart_intake` | `formula_rebinding` | single assignment: §2 already bound that name |
| `let a: length = b + 1 cm`, where the recipe binds b three statements lower | `formula_unbound_name` | declaration order is the authority, not a solver |
| `if(dart_intake > 4 cm, 5 cm)` | `formula_parse` | the third part of a conditional is mandatory |
| `"wide"` | `formula_parse` | the language has no text value |
| `dart_intake ^ 3` | `formula_unsupported` | only the square is in the grammar |
| `garment_hem * (100 pct + shrinkage)` | `formula_unknown` | this fabric's shrinkage was never measured; no default is invented |
| `eps_phys + 1 cm` | `formula_tolerance_unbound` | a recipe context supplies no factory tolerance |
| `spline(cb_seam, 3)` | `env_nurbs` | the envelope owns this refusal ([the contract §5.3](../formula-language.md)) |

Each row is executed, not illustrated: the census raises the diagnostic the row names or reports a
failure, so a diagnostic set that drifts from the grammar reddens this table rather than the
product.
