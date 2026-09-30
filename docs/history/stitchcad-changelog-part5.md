# Sealed archive — StitchCAD changelog, slices 30–31

Immutable historical segment, sealed out of `CHANGELOG.md` under the rollover rule recorded in
`.doctrine/live_document_size/surfaces.tsv` (row `changelog`, lifecycle `rolling_ledger`): when the live
window passes its health target, the oldest entries are sealed here and a pointer is left behind.

- **Sealed identity:** 82 lines, 7505 bytes, `sha256:18548ff78f8345d14e5f79f01fa02328a57268b7e3aebb45838d248f02cabd72`
- **Sealed by:** leaf `G0-CONTRACT.14` on `2026-09-30` — the append that crossed the rollover milestone
  performed the rollover in the same commit, as the doctrine requires and as `G0-CONTRACT.1` and `.8` did
  before it. The window was in commit order, so "seal the oldest" sealed the oldest.
- **Coverage:** slices 30–31 — `STITCHCAD-G0-0005`, `STITCHCAD-G0-0013c`, newest first, exactly as they stood.
- **Predecessors:** `stitchcad-changelog-part1.md` … `part4.md`. part1's own coverage line overstates its
  range by one slice; the correction is recorded in part2's descriptor and in the live pointer, because a
  sealed segment is never edited (defect D30).
- **Retrieval:** this file, or `git log --follow CHANGELOG.md`. The sealed content is everything below the
  `---` rule and the blank line after it; `shasum -a 256` over exactly those bytes reproduces the digest
  above, and the `DESCRIPTOR` rule of `docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh` runs
  that comparison on every `make probes`.

---

## STITCHCAD-G0-0005 - both instantiation paths, and the loss between them stated (leaf `G0-CONTRACT.5`)

docs/book/src/spec/instantiation-paths.md specifies the two ways a design becomes a sized garment -
regeneration from the recipe, and grade-rule instantiation from a base instance - with each path's inputs,
process, outputs, authority and oracle in one table, because roadmap §3.3 requires both and requires the
divergence between them to be declared rather than discovered by a factory.

- **the information loss is stated in three parts, not waved at**: the recipe is not recoverable from base
  plus rules (a delta says how much a point moves, never why, so path 2 cannot serve a body that differs
  from the chart - the MTM case); `delta = 0` is ambiguous between "this deliberately does not grade" and
  "this was omitted", so the canonical form marks declared zeros and marks a receiver's zeros `unknown`
  with the state that implies; and nonlinear steps (re-fitting a curve, truing, squaring a hem to the
  grain) do not commute with point motion, which is where the paths actually diverge
- **a worked example, re-derived rather than recalled**: grading the reference skirt with breaks of
  +4.0 / +4.0 / +1.0 cm gives 15 quantities and their deltas, and both paths agree EXACTLY - the one
  nonlinear value, `side_seam_length` = √(drop² + flare²), reproduces to `43.104524` cm by both routes with
  a difference of `0.00e+00`, and the graded `waist_closure` still equals the graded `quarter_waist`
  (`19.500 = 19.500`). The reason is measured: 17 of the fixture's 18 derived values are affine in the
  measurements, and the eighteenth is a function of points that grading moves
- **that exactness is a property of the fixture and an inconvenient truth for testing**: the reference
  skirt cannot exercise the equivalence tolerance, because there is nothing to tolerate. It is the right
  first test for path 2 (exact equality is assertable) and the tolerance is exercised at G3 by the bodice
  and set-in sleeve, whose cap ease and armscye are not affine, and by any rule table that came from a
  factory rather than from two regenerations
- **normative decisions this chapter makes rather than defers**: grade points are `PointRef`s and never
  indices (which is what makes G3's independent-engine re-import possible); allowances are RE-DERIVED after
  grading and never graded themselves, with the consequence stated - a factory whose rules were built on cut
  contours differs at corners by the corner treatment's own geometry, and that difference is what the
  equivalence report exists to show; the three `.rul` attributes get StitchCAD semantics (`stack point`
  anchors the table, `fixed perimeter` reports a conflict rather than absorbing a length change,
  `smoothing` is bounded by the geometric-approximation class and never moves a point another rule fixes)
- **extreme sizes are checked after target-system reconstruction**, as roadmap §3.3 requires: eight named
  checks, each with the tolerance class it is judged against, and the last one generalises D33's lesson - a
  check over declared quantities cannot see a constructed point, so the suite re-derives finished
  dimensions from the reconstruction as well
- **the equivalence contract reports and does not reconcile**: neither path wins; the report names the
  quantity, both values, the difference, the class and the verdict, a difference beyond its class is a
  finding with an owner, and the report is release evidence rather than a log line
- every external claim carries its status: the `.rul` semantics are cited from the roadmap and confirmed at
  G3 by an independent engine (the format's text has not been read here), the interchange modes are cited
  from ADR-0004, the tolerance classes are the units chapter's and none is invented
- the glossary absorbed the terms this chapter introduces (265 -> **270**: `instance`, `reconstruction`,
  `extreme size`, `declared zero`, `equivalence report`) and the six grading entries that said "specified by
  `G0-CONTRACT.5`" now cite the chapter that specifies them; the A-Z index is re-derived
- gates: glossary census -> "270 terms / 8 parts / 140 tokens / 0 failure(s)" with 119 tokens used by the
  spec set and 0 unaccounted; feature-matrix census -> "105 rows / 29 diagnostics / 0 failure(s)";
  `make book` -> exit=0 with 7 spec pages; `make gate` -> "=== all doctrines green ==="; containment -> OK,
  the chapter at 254 lines / 19 496 B inside its per-part health and its widest row trimmed 292 -> 231 B

## STITCHCAD-G0-0013c - the reference fixture's waist, corrected (leaf `G0-CONTRACT.13c`)

The fixture every G2 golden, mutation test and offset-pathology case is built around drafted a skirt whose
finished waist was **46.0 cm instead of the declared 74.0 cm** - and the check the chapter called "the
fixture's own invariant" passed throughout (defect D33).

- **the error**: §5 step 2 placed the waist side point at `quarter_waist - ss_suppress` = 15.5 cm from CF.
  The side seam takes its 3.0 cm of suppression off the **hip** width, not the waist width, so the point
  belongs at `quarter_hip - ss_suppress` = 22.5 cm. Drafted as written, the panel's waist edge was 7.0 cm
  short and the dart took another 4.0 cm out of it: `4 x (15.5 - 4.0)` = 46.0 cm against a declared
  `waist_girth + ease_waist` = 74.0 cm
- **why no check saw it**: the allocation balance `4 x (ss_suppress + dart_intake) = garment_hip -
  garment_waist` closes over DECLARED quantities, and every declared quantity was consistent. The error was
  in where a point is CONSTRUCTED, which no relationship between declared quantities mentions. Re-derived
  with python3 rather than by reading: `4*((98.0+4.0)/4 - 3.0 - 4.0)` -> `74.0`, `4*((74.0+0.0)/4 - 3.0 -
  4.0)` -> `46.0`, and the balance -> `28.0 = 28.0` either way
- **the fix**: §5 steps 2 and 3 corrected; both dart-centre formulas corrected to the span they meant
  (`(quarter_hip - ss_suppress) / 2`, so 7.75 -> **11.25 cm**, with the legs at 9.25 and 13.25 cm); §4 gains
  a `waist_closure` row - `(quarter_hip - ss_suppress) - dart_intake = quarter_waist`, `18.5 = 18.5` - as the
  constructed oracle the balance could not be; §12 makes both checks test obligations; §11 records the
  correction as the superseding record for the sealed `STITCHCAD-G0-0013` segment, which is immutable
- **all 18 derived rows re-derived** from §2 and §3 in table order, feeding each result into the rows below
  as the recipe does: 0 mismatches, and the finished waist, hip and hem now equal their declared values
- **what found it was not a review of the fixture** but the slice that has to compute with it: `.5` grades
  this skirt as its worked divergence example, and grading a waist point requires knowing which span it sits
  on - the two candidate spans disagreed by 7.0 cm. A defect hides best in a document nobody has a second
  use for
- the rule generalises as `docs/decisions/decision_fixture-oracles-derive-the-finished-dimension.md`: a
  fixture needs a declared oracle AND a constructed one, and the question to ask before freezing a golden
  is "which check would fail if a point moved 5 mm?" - for this fixture, before the fix, none
- gates: `make book` -> exit=0; `make gate` -> "=== all doctrines green ==="; glossary census ->
  "265 terms / 8 parts / 139 tokens / 0 failure(s)" (the new `waist_closure` token is declared by §4's own
  table, as the token rule requires); no product code touched
