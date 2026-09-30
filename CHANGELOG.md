# CHANGELOG.md

Newest first: one section per completed slice, in commit order. Older slices live in sealed, immutable
segments under `docs/history/`, each named below with its identity and retrieval path.

# Sealed archive — earlier slices

| Segment | Coverage | Sealed identity |
| --- | --- | --- |
| [`part1.md`](docs/history/stitchcad-changelog-part1.md) | slices 1–15, `STITCHCAD-PLANNING-0001` … `STITCHCAD-SPINE-0004b` | 365 lines, 30452 bytes, `sha256:f4aec75a…` |
| [`part2.md`](docs/history/stitchcad-changelog-part2.md) | slices 16–20, `STITCHCAD-SPINE-0014` … `STITCHCAD-G0-0002` | 152 lines, 12811 bytes, `sha256:5783ac36…` |
| [`part3.md`](docs/history/stitchcad-changelog-part3.md) | slices 21–24, `STITCHCAD-G0-0013` … `STITCHCAD-G0-0018` | 147 lines, 12289 bytes, `sha256:14ad5278…` |
| [`part4.md`](docs/history/stitchcad-changelog-part4.md) | slices 25–29, `STITCHCAD-G0-0004` … `STITCHCAD-SPINE-0017` | 177 lines, 15440 bytes, `sha256:a8cc1de6…` |

**Correction (D30).** part1's own descriptor says its coverage runs "through `STITCHCAD-SPINE-0004c`".
It does not: part1's newest entry is `STITCHCAD-SPINE-0004b`, and `SPINE-0004c` is sealed in part2.
Sealed segments are immutable, so the correction is recorded here and in part2's descriptor rather than
by editing part1.

The bedrock scaffold's own changelog — the provenance of this repository's discipline spine — is sealed
in [`docs/history/bedrock-scaffold-changelog.md`](docs/history/bedrock-scaffold-changelog.md).

The live window below holds the most recent slices. When it passes its health target (400 lines /
32 768 bytes) again, the oldest entries are sealed the same way, and
`bash docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh` proves the order, the uniqueness and
the digests afterwards.

## STITCHCAD-G0-0013d - one waistband, and the instrument that keeps it one (leaf `G0-CONTRACT.13d`)

The reference fixture described two different garments at once for nine commits: §4 published the cut width
of ONE band folded lengthwise while §6 listed a faced two-piece band and §8 sewed only the outer one
(defect D27). The director ruled on 2026-09-30 that the engineer decides it; this slice decides it, and
makes the agreement between the chapter's tables mechanical rather than a matter of reading.

- **the decision**: one straight band, cut once, folded lengthwise at its midpoint (5.0 cm from either long
  edge), plus one interfacing piece cut at the band's finished dimensions (77.0 x 4.0 cm, no allowance on
  any edge) and fused inside the seam lines. Five pieces, not six - `waistband_outer` and `waistband_inner`
  no longer exist anywhere in the book
- **sourced, not preferred**: five references read on this machine on 2026-09-30, each cited by URL and date
  in `docs/decisions/decision_reference-fixture-waistband-straight-folded.md` under a `read-external` label
  that upgrades no claim about a standard. The decisive fact is that the drafting literature gives the
  straight band ONE rectangle with a fold line and reserves "cut two pieces - one for the outer waistband,
  and one for the inner waistband" for the **contoured** band, while this fixture's waist sits at the natural
  waist where a straight band belongs. The sources disagree about band height (2-5 cm against a 3 cm maximum
  for a straight band), so `wb_width` = 4.0 cm stays `assumed` with the conflict recorded rather than quietly
  resolved in favour of the convenient source
- **every piece is accounted for**: §8 gains an attachment account - a span, or a declared non-sewn method
  from a closed list that holds `fused` alone. "Every piece has a span" is false for a fused interfacing,
  and believing it is what let an inner band nobody sewed pass for nine commits
- **the numbers are derived, not read**: §4 gains `waistband_fold_position` 5.0, `waistband_finished_length`
  77.0, `wb_interfacing_width` 4.0, `wb_interfacing_length` 77.0 and the band's two closure checks
  (`waistband_pattern_length - 2 x sa_cb - wb_extension = garment_waist` -> `74.0 = 74.0`, and
  `waistband_cut_width - sa_waist - sa_wb_bottom = 2 x wb_width` -> `8.0 = 8.0`), so §4.1 carries four
  oracles: two for the body, two for the band
- **a tracked producer at last**: `docs/tasks/artifacts/reference_fixture/run_fixture_derivation.sh`
  evaluates every §4 formula over the tables above it, checks the four closures, the piece account, §12's
  published count against §6's list, and that §4's band width describes the same construction as §6's band
  pieces (rule `B1`). Over the chapter as committed: `16 derived rows / 2 closure checks / 6 pieces /
  7 mismatch(es)`, exit=1, naming D27 by shape. Over the chapter now: `20 / 4 / 5 / 0 mismatch(es)`, exit=0.
  Until this slice the arithmetic that found D33 and then D27 lived as `python3 -c` strings inside task
  leaves - re-runnable by nobody, which is the leg-3 breach this repository already committed once as D20
- **the probes are shown to discriminate, not merely to pass**: nine arms (one GREEN, eight RED) ->
  `probes: 9 pass / 0 fail`, and neutering one rule at a time in a scratch copy reddens exactly the arms
  that depend on it (`B1` removed -> `8 pass / 1 fail`; `P2` removed -> `7 pass / 2 fail`)
- **the token census corrected the chapter a fourth time at authoring time**: four undeclared tokens on the
  first draft - `Fold` (a column name of this chapter's own table), `fused` (a machine enum value a program
  reads, so it became a glossary term with one owner), and the two names of the rejected reading. The fix was
  the convention rather than an exemption: prose lost its backticks and the dead piece names survive in the
  layer-C record for anyone tracing the defect -> `276 terms / 8 parts / 145 tokens / 0 failure(s)`
- **not decided, deliberately**: whether a `SeamSpan` may name the same piece on both sides - a folded band's
  short ends are stitched across, which joins one piece to itself. Logged as D35 with `G1-SLICE.3` as owner;
  §8 records the ends as an edge finish instead of settling a type invariant by accident
- **the ledger probe refused an honest changelog, and the probe was the thing that was wrong** (D39): its
  work-unit id shape was `[0-9]+[a-c]?`, so this slice's own id `STITCHCAD-G0-0013d` parsed as
  `STITCHCAD-G0-0013` and collided with the sealed entry of that name -> `NO-DUP FAIL both live and sealed:
  STITCHCAD-G0-0013`, a duplicate that did not exist, plus an `ORDER FAIL` comparing the wrong commit. The
  same collapse hides a real mis-ordering between `-0013d` and `-0013e`, so one regex produced a false red
  AND a possible false green. Fixed to `[a-z]?` in all six places and pinned by a new GREEN arm `SUFFIX`,
  shown sensitive by reverting the class in a scratch copy -> `probes: 5 pass / 2 fail`; after the fix
  `probes: 7 pass / 0 fail`
- **also in this commit**: D36 logged and fixed (`LIVE_STATUS.md`'s spine row reported 13 universal gates and
  7 probe suites where `make gate` prints 12 universal plus the project slot's 2 and `make probes` finds 12);
  D37 logged and fixed (`DEV_NOTES.md` described itself below every lesson, at line 149 of 156); D38 logged
  (the census's open/closed counts are prose, so a naive derivation reports 33 closed and misses D34 —
  `PLANNING.5`'s goal is extended to derive them); D34's recurrence recorded - the commit that created `.13d`
  and `.4b` moved the frontier without writing `docs/TASK_TREE.md`, so the index named a leaf four commits
  done, and all three stale rows are corrected
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `12 suite(s) green`; `make book` ->
  exit=0; feature-matrix census -> `105 rows / 29 diagnostics / 0 failure(s)`; standards census ->
  `6 registered / 6 designations used / 0 failure(s)`; containment OK with the chapter and the record both
  over health and inside every ceiling, recorded in the leaf; no product code touched

## STITCHCAD-G0-0008 - ADR-0001: licence and solver are one decision (leaf `G0-CONTRACT.8`)

`docs/decisions/decision_adr-0001-license-and-solver.md` settles roadmap §5's ADR-0001 **before any solver
code exists**, which is when §5 says it must be settled: an external contributor arriving at G1 inherits a
decision, not a default.

- **the decision**: the core is dual `MIT OR Apache-2.0` (re-read from `Cargo.toml`, not recalled:
  `grep -n license Cargo.toml` -> `license = "MIT OR Apache-2.0"`), so the SolveSpace solver `slvs`
  (GPLv3) is **rejected**, and `sc-sketch`'s optional local constraints get a custom kernel or a numeric
  least-squares solver written for this repository. Licence and solver are one record because neither is
  decidable alone - the solver choice IS the licence choice
- **the exchange rate is stated, not asserted**: linking `slvs` copylefts the workspace, which decides who
  may embed the product - and the persona roadmap §1.2 names is a patternmaker producing a package for a
  *named factory*, with factories and partners embedding the tool. What `slvs` would buy is an *optional
  annotation layer* (ADR-0003 makes the recipe primary and sketch constraints local), while the production
  solver is custom anyway (§6.1's deterministic finite-domain CP engine, so native and WASM share one
  determinism argument). Trading the licence of the whole product for the second solver of an optional
  feature is the bad exchange this record refuses
- **BSL-1.1 is kept out of the permissive category**, as §5 demands: a use restriction is a commercial term
  needing the project owner's own review, and a downstream embedder cannot treat BSL code as MIT. Adopting
  it would be a new ADR superseding this one
- **consequences written down before they are needed**: no CLA to contribute; no GPL or GPL-incompatible
  dependency in a shipped crate, enforced by the re-runnable census `G1-SLICE.15` already owns; Z3 stays a
  CI-only differential oracle behind `csp-z3` (its MIT licence would permit shipping it - the reason it is
  not shipped is the C++ dependency and the browser profile, and this record does not reopen that); and an
  iterative least-squares kernel is NOT covered by the numerical contract's exactness argument, so if it
  ever lands it must quantize into the internal units with a declared tolerance class, a fixed iteration
  bound and a declared convergence test - never "until it looks converged"
- **one re-open condition, in three parts stated now** so the G1 spike cannot be argued into satisfying one
  of them later: the spike shows a capability the custom kernel cannot reach within scheduled effort on a
  realistic pattern set; a named persona or partner wants it; and the project owner accepts the licence
  consequence in writing. A partial re-open is allowed in one direction only - `sc-sketch` licensed apart
  from the core if it stays an optional crate nothing in the default build links, the shape §6.1 uses for Z3
- the rollover this append triggered is performed in the same commit: slices 25-27 sealed into
  `docs/history/stitchcad-changelog-part4.md` with a digest-verified descriptor, the pointer table rebuilt
  from every segment on disk so it cannot drift, all pre-existing entry bodies byte-identical, and
  `run_changelog_ledger_probes.sh` green on the result
- gates: `make gate` -> "=== all doctrines green ==="; `make probes` -> "11 suite(s) green"; `make book` ->
  exit=0; no product code touched and no book chapter added (this leaf's deliverable is the record)
- LIVE_STATUS: the G0 frontier moves to .9, ADR-0003 and the formula language

## STITCHCAD-G0-0007 - the standards registry, and the discipline that keeps a citation honest (leaf `G0-CONTRACT.7`)

docs/book/src/spec/standards.md registers every external standard the model draws on - six designations,
each with its role, what is adopted, what is deliberately not, a status from a closed three-word
vocabulary and a named owner. All six are `cited-from-roadmap`, so this book quotes no clause of any
standard, and that is the rule rather than a gap.

- **three chapters had already routed their standards claims to a chapter that did not exist** (ontology
  §8, reference skirt §11, size sets §8), which is the shape a claim takes while it waits to be asserted by
  nobody. §4 is now a deferral ledger: all five deferrals - including the glossary's synonyms and the
  feature matrix's `supported` mapping rows - carry a disposition, and one stays honestly open (the
  fixture's body measurements are declared constants of this repository, copied from no standard, and stay
  `assumed` until somebody reads one)
- **two rules make the chapter normative**: no clause number, table number or quoted definition of a
  standard appears anywhere in the book unless its status is `read-in-repo` (none is, so none does); and a
  standard is DATA with provenance, never authority - "ISO 8559 says so" cannot satisfy an `unknown`
- **§3 states what the model needs from any standard**, so reading one later has something to satisfy:
  landmark identity, procedure repeatability, designation mapping (a label never supplies a measurement),
  and tolerance semantics - where a standard's tolerance is the physical-acceptance class, the factory's
  number, and is never silently adopted as a numerical or geometric one
- **the verification plan names its real dependency** rather than implying that reading is a formality:
  `.14` names the reviewer and the procurement owner (it is blocked on the director naming humans),
  procurement obtains the texts, and a read changes exactly three things - clause-level citations for
  landmarks and procedures, the fixture's `assumed` measurements, and the size-set designation data. A read
  that contradicts the book is a defect like any other
- **the registry claim is derived**: `run_standards_census.sh` -> "standards census: 6 registered / 6
  designations used / 0 failure(s)", exit=0, with a per-designation list of every file that uses it;
  `run_standards_probes.sh` -> "probes: 6 pass / 0 fail", including an arm that smuggles "per ISO 4915"
  into a chapter and requires the census to name it, and one that turns a row `read-in-repo` with no
  citation behind it
- **the containment ceiling refused the first draft**: the six-column registry table had rows of 410 B
  against a 320 B maxline ceiling and the deferral ledger 398 B. The registry became four columns plus one
  bounded subsection per standard, and the ledger became bounded prose entries - the remedy
  G0-CONTRACT.2 recorded when the same ceiling caught its tolerance table. The chapter now measures 204
  lines / 13 849 B, widest line 187
- **the glossary census caught two prose spans wearing token formatting** before the chapter shipped:
  `blocked` (a task-tree status) and `AAMA` inside a sentence describing the census's match shapes. The fix
  was to de-tokenize the prose, not to exempt it - a backticked span is an identifier a program reads
- D34's immediate half is corrected here: docs/TASK_TREE.md's frontier cell and execution-order line had
  gone stale at `.4` while `.5` and `.6` landed, because COMMIT.md updates that file "only if the frontier
  changes" and nothing derived whether it did. The durable half is PLANNING.5, deferred behind product work
- gates: make gate -> "=== all doctrines green ==="; make probes -> "11 suite(s) green"; make book ->
  exit=0 with 9 spec pages; glossary census -> "275 terms / 8 parts / 144 tokens / 0 failure(s)";
  feature-matrix census -> "105 rows / 29 diagnostics / 0 failure(s)"; containment -> "OK - 17 surfaces,
  15 routes, 70 files measured"; no product code touched
- LIVE_STATUS: 33 defects logged / 29 closed; the G0 frontier moves to .8, ADR-0001

## STITCHCAD-G0-0006 - size-set ownership decided, and a label is not a token (leaf `G0-CONTRACT.6`)

Roadmap §3.4 left one question open at G0 - "whether [the SizeSet] lives in the Design, the Factory
Profile, or a third Order object" - and ontology §2.3 pointed at this chapter for the answer. Both now
exist: `docs/decisions/decision_size-set-ownership.md` and `docs/book/src/spec/size-sets.md` (187 lines /
12 812 bytes).

- **the decision**: a `SizeSet` is its own object with identity and revision; a `Design` REFERENCES it (by
  id plus the revision it was authored against, so re-opening a design pins what it was built with); a
  Factory Profile may override it, and an override is a typed transformation that produces a *resolved*
  size set naming the design's reference, the profile's revision, the transformation and its evidence -
  never an in-place relabeling, so a release package can always say whose sizes it cut
- **quantities stay out**: roadmap §7.5 puts size-run quantities in an Order object, not in the reusable
  design, so a size set carries none and a tech pack that needs them records an unresolved input rather
  than inventing a ratio - a ratio baked into a design would make a commercial change stale-ify approvals
  that geometry never touched
- **the three candidate owners are argued, not listed**: design-only cannot serve a factory's house chart;
  profile-only makes a design unsized until a factory exists, which breaks the headless CLI, the WASM
  viewer and the reference fixture; an Order object owns quantities but must not own sizes, because the
  recipe has to be evaluable with no order in the loop. Precedence follows roadmap §8: a hard restriction
  outranks a factory override, and the conflict is visible rather than merged
- **the chapter is normative about the object**: ten fields with types and requiredness; labels are names
  and `members` is an AUTHORED order that no code path re-sorts (an alphanumeric sort of "XS, S, M, L, XL,
  2XL" puts 2XL first); exactly one base size which SHALL be a member and which is what cumulative and
  incremental breaks are measured from; breaks per adjacent pair so an uneven range is expressible, and a
  set with no breaks serves regeneration while refusing grading with a diagnostic; multi-dimensional
  systems as named axes where a member is a point and an absent combination is absent, never interpolated
- **EN 13402 and ASTM D5585 are named, not quoted**: §8 states what the model must EXPRESS for each of the
  five designation systems and marks the standards' content as cited-from-the-roadmap with
  `G0-CONTRACT.7` as the owner that reads them. Asserting a standard's tables without having read it is
  the exact failure the claim-verification policy exists to stop
- **MTM is not a special case**: a made-to-measure instance is a `custom` set of one whose chart comes from
  body measurements, whose base is its only member and whose breaks are empty - so path 2 refuses and
  path 1 is the only route
- **the glossary census found a convention violation in the new chapter before it shipped**: three example
  size labels were written in backticks, so C1 reported `S`, `M` and `L` as undeclared machine tokens. The
  fix was not an exemption but the rule - a label is prose a factory reads, a token is an identifier a
  program reads, so labels are quoted and §3 of the chapter now says so. Glossary: 270 -> **275 terms**
  (`size system`, `axis`, `resolved size set`, `size-set transformation`, `order object`), index re-derived,
  `275 terms / 8 parts / 144 tokens / 0 failure(s)`
- **the matrix census grew a link rule (M7)** because this slice added cross-chapter links to the matrix:
  every markdown link in a normative chapter must resolve, and a cited clause must exist in its target.
  Its RED arm mutates one link in a copy and requires the refusal -> `probes: 11 pass / 0 fail`
- gates: `make gate` -> "=== all doctrines green ==="; `make book` -> exit=0 with 8 spec pages;
  feature-matrix census -> "105 rows / 29 diagnostics / 0 failure(s)"; containment -> OK, 67 files
  measured; no product code touched

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
