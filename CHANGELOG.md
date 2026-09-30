# CHANGELOG.md

Newest first: one section per completed slice, in commit order. Older slices live in sealed, immutable
segments under `docs/history/`, each named below with its identity and retrieval path.

# Sealed archive — earlier slices

| Segment | Coverage | Sealed identity |
| --- | --- | --- |
| [`docs/history/stitchcad-changelog-part1.md`](docs/history/stitchcad-changelog-part1.md) | slices 1–15, `STITCHCAD-PLANNING-0001` … `STITCHCAD-SPINE-0004b` | 365 lines, 30452 bytes, `sha256:f4aec75a…` |
| [`docs/history/stitchcad-changelog-part2.md`](docs/history/stitchcad-changelog-part2.md) | slices 16–20, `STITCHCAD-SPINE-0014` … `STITCHCAD-G0-0002` | 152 lines, 12811 bytes, `sha256:5783ac36d8bc7cee…` |

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

## STITCHCAD-G0-0004 - the release claim gets a boundary, and the boundary is derived (leaf `G0-CONTRACT.4`)

docs/book/src/spec/feature-matrix.md dispositions **105 rows** - 76 supported, 19 rejected, 10 deferred -
each with a reason that cites its source explicitly (`ontology §4.3`, `roadmap §3.2`), the gate whose exit
criteria prove it, and for every refusal the diagnostic token it must produce. §10 declares those 29
tokens with the arguments each carries, because an undeclared token has no defined meaning.

- three rules make the table normative rather than descriptive: **no silent approximation** (a refused
  construction produces its diagnostic and NO geometry - a knit block drafted as woven, a NURBS curve
  flattened without saying so, a pocket drawn as internal lines are all defects); **a supported row names
  its proof** (a gate's exit criteria, not a hope); and **modelled is not supported** (an object nobody
  scheduled a proof for is `deferred` to G7, whose exit is an envelope statement with named limitations)
- the acceptance clause "nothing in the ontology is silently unlisted" is derived, not asserted:
  `run_feature_matrix_census.sh` -> "feature-matrix census: 105 rows / 29 diagnostics / 0 failure(s)",
  exit=0 - all 16 required ontology object clauses cited, 8 of 8 roadmap §1.3 non-goals matched to a
  rejected row with the mapping printed, 5 of 5 §3.2 envelope garments supported and 3 of 3 named refusals
  rejected, every gate cell resolving to a real roadmap §11 gate or to `unnamed (D32)`, and every declared
  diagnostic used by exactly the rows that raise it
- **D32, found by writing the rows**: roadmap §3.2 puts a classic collar and trousers inside the v1
  envelope and ontology §4.7 specifies button and pocket objects, yet no gate's exit criteria mention any
  of them - `sed -n '/### G3 /,/### G4 /p' ROADMAP.md | grep -ci 'collar\|trousers\|button\|pocket'` -> `0`.
  G3's note *permits* a shirt/trousers intermediate without promising one. Rather than borrow a gate, those
  five rows (collar, trousers, buttons, pockets, fly) say `unnamed (D32)` and the census prints them on
  every run as advisory A1, so `G0-CONTRACT.15` must put the assignment to the director instead of the gap
  closing by being forgotten
- the glossary absorbed the 26 terms the matrix introduces (239 -> **265**), index re-derived with
  `--emit-index`; `run_glossary_census.sh` -> "265 terms / 8 parts / 139 tokens / 0 failure(s)", exit=0.
  The machine-token convention paid for itself immediately: the matrix's 29 diagnostic tokens are declared
  by its own §10 table, so the glossary census's coverage rule needed no exemption
- **two of the ten new probe arms passed for the wrong reason, and the census was right** - measured, not
  assumed. One renamed a diagnostic token everywhere, which renames a declaration and its use together and
  changes nothing a census can see; the other de-cited one of three rows citing `ontology §4.4`, leaving
  the clause covered. A RED arm must remove the PROPERTY, not one instance of it, and the rule is now in
  TOOLBOX.md's probe conventions where a probe author reads it. A third bug was mine: swapping a `{2,4}`
  interval for `###+` "for portability" silently dropped every `##`-level heading, so the coverage rule
  required three clauses fewer and still printed 0 failures - which is why a census prints the size of the
  population it judged, not only its breaches
- gates: `make gate` -> "=== all doctrines green ==="; `make probes` -> "10 suite(s) green" (72 arms,
  0 fail); `make check` -> five "test result: ok" lines, no product code touched; `make book` -> exit=0 with
  6 spec pages; containment -> "OK - 17 surfaces, 15 routes, 64 files measured", with the matrix part at
  28 646 B against a 24 576 B per-part health (117%, ceiling 40 960) recorded in the leaf rather than paid
  for in vaguer rows

## STITCHCAD-G0-0001 — the glossary: one meaning per term, one owner per token (leaf `G0-CONTRACT.1`)

**The G0 exit clause "glossary of construction terms" is met, and its completeness is derived rather
than declared.** `docs/book/src/spec/glossary.md` plus eight domain parts carry **239 terms**, each with
a plain-language meaning, the canonical object that specifies it, the synonyms factories and other CADs
use, and the machine token — and a ⚠ on the ones whose mistranslation causes a wrong cut.

- **Partitioned because a termbase is not a chapter.** One file would have been ~70 KB of five-column
  rows against a `book_collection` per-part health of 24 576 bytes, so the glossary is eight parts by
  domain (29–48 lines, 4.3–7.5 KB each) behind an index chapter that carries the rules and the derived
  A–Z list. A term sits next to the terms it is confused with, and no part breaches its ceiling.
- **The machine-token rule is normative** (and is now a layer-C record,
  `decision_machine-tokens-declared-where-used.md`): a token is never rendered raw to a human; one token
  has one meaning, so where four terms are all a `Closure`, one entry owns the token and three write
  `→ Closure`; tokens are ASCII and locale-independent; and where code exists the token is quoted from
  code — `Micrometre`, `MICRODEGREES_PER_DEGREE`, `UnitError::DomainExceeded`, the six `ToleranceClass`
  variants — not invented prettier than the crate.
- **Safety-relevant terms are a shipping rule, not a styling one.** Roadmap §7.6 names notch types and
  the sew/cut line aliases; §8.3 names units and allowance ownership. 22 entries are required to carry
  the ⚠ (72 do), and the census refuses any of the 22 that loses it, plus any entry owning a `type`
  token — so a new notch shape inherits the requirement instead of being forgotten.
- **Its own claims are re-derived:** `bash docs/tasks/artifacts/glossary/run_glossary_census.sh` →
  `glossary census: 239 terms / 8 parts / 138 tokens / 0 failure(s)`, `exit=0`. Ten rules: entry
  structure, parts inventory, term uniqueness, token ownership, cross-references, token shape, every
  canonical reference resolving to a real chapter clause, roadmap clause, ADR or task-tree leaf (155
  references, 0 dead), the safety marks, index↔parts equality both directions, and the coverage claim —
  every machine token the specification uses is owned here, declared by the chapter that uses it, or
  exempted with a written reason (`82` used, `0` unaccounted).
- **The probes found three defects in the census before it found any in the glossary**, which is why
  they exist: `R1` read the *meaning* column instead of the object column, so it checked nothing and
  reported `dead references: 0` vacantly; the `§` clause slice was one byte off, because `§` is two bytes
  and `LC_ALL=C` makes awk count bytes; and `resolve()` stripped `/../` without the parent segment, so
  every `../ontology.md` resolved to a path that does not exist. All three were caught by RED arms
  (`DEAD-CLAUSE` failing to fire), never by reading the code. `probes: 10 pass / 0 fail`.
- **A second instrument came with it**, because the rollover this entry triggers is exactly the
  operation D29/D30 corrupted: `run_changelog_ledger_probes.sh` checks the live window's order against
  commit order, live-vs-sealed uniqueness, each segment's declared `sha256` against its content, each
  descriptor's coverage claim, and the pointer↔segment closure — the five checks the containment
  doctrine's rollover protocol step 6 requires. It verified part1's digest (`365` lines reproduce
  `sha256:f4aec75a…`) and then proved part1's coverage line wrong (D30).
- **Rollover performed by this append:** the live window crossed its health target, so the five oldest
  entries are sealed into `docs/history/stitchcad-changelog-part2.md` with their own descriptor, the
  window is reordered into commit order (D29), the stale "_Inherited spine history_ divider" sentence is
  corrected (D31 — that divider left this file in `SPINE-0014`), and part1's coverage claim is corrected
  by superseding record rather than by editing an immutable segment (D30).
- Validation: `make book` → `exit=0` with all nine glossary pages rendered; `make probes` →
  `9 suite(s) green`; `make gate` → `=== all doctrines green ===`; `make check` → `test result: ok`;
  containment → the widest glossary row is `268` B against a `320` B ceiling (over the `200` B
  prose-shaped health target, recorded in the leaf rather than trimmed away).

## STITCHCAD-G0-0013b — the reference fixture declares its own tokens (leaf `G0-CONTRACT.13b`)

**Found by building the glossary's token census, not by reading.** The fixture every G2 golden, mutation
suite and agent gate will be frozen over used **19 machine tokens that no table declared** — and forcing
each one to a declaration exposed a contradiction in the garment itself.

- **D26, fixed.** §4 named its 17 derived values in prose ("quarter hip", "front dart centre") while §5's
  recipe and §4's own formulas referred to them as `quarter_hip` and `front_dart_centre`; `ease_waist`,
  `ease_hip`, `sa_cb`, `sa_waist` and `sa_wb_bottom` appeared inside formulas with no declaring table at
  all. All three tables now lead with a **Token** column, §7 declares `sa_side`/`sa_cb`/`sa_waist`/
  `sa_hem`/`sa_wb_bottom`, and §4's formulas are written over tokens only — no prose word survives inside
  one, which is what makes them machine-checkable. Measured: tokens used-and-undeclared `19` → `6`, and
  the six remaining are glossary vocabulary (`SeamAllowance`, `assumed`, `close`, `known`, `semi`, `walk`),
  not fixture names. **No derived value changed**: `allocation_balance` still closes `28.0 = 28.0`.
- **D27, logged and owned, not guessed at.** §4's `waistband_cut_width = 2 × wb_width + sa_waist +
  sa_wb_bottom = 10.0 cm` is the cut width of ONE band folded lengthwise; §6's piece list carries
  `waistband_outer`, `waistband_inner` **and** `waistband_interfacing` — a faced two-piece band whose
  pieces would each be cut at `4.0 + 1.0 + 1.0 = 6.0 cm` (`python3 -c "print(2*4.0+1.0+1.0, 4.0+1.0+1.0)"`
  → `10.0 6.0`). §8 compounds it: the `waist` span sews only the outer band, so the inner band has no span
  and §12's count of 6 pieces is the faced reading's. Both are real skirt constructions and they are
  different garments, so choosing is a domain decision: **`G0-CONTRACT.14`** owns it, with the expert it
  names. The contradiction is now recorded in the chapter (§6 and §11) with both readings and their
  arithmetic, so no reader and no golden can take a waistband number as settled while it is open.
- **D28, fixed.** §5 step 7 sent the reader to "allowances (§6)"; allowances are §7 and §6 is the piece
  list. `G0-CONTRACT.1`'s glossary census carries an `R1` rule that resolves every canonical-object
  reference against real headings, which is that class instrumented rather than eyeballed.
- Validation: `make book` → `exit=0`; `make gate` → `=== all doctrines green ===`; the token census over
  the fixture at `HEAD` against the working tree → the numbers above.

## STITCHCAD-SPINE-0018 — the push-due trigger means "CI must re-verify this" (leaf `SPINE.18`)

- `SPINE.17` globbed `scripts/check_*.sh`, so the helper matched its own trigger: committing it reported
  `EXCEPTIONAL PUSH DUE — 1 unpushed file(s) … scripts/check_push_due.sh`, `exit=1`, for a file no CI job
  executes. A standing false obligation is worse than no instrument — the honest response to a warning
  that always fires is to stop reading it.
- The registered checks are now **derived** from the two registries (`registry_checks()` reads the paths
  cited in `scripts/check_doctrines.sh` and `scripts/check_doctrines.project.sh`, plus the drivers):
  17 paths, including both project doctrines, excluding this helper. A newly registered check becomes a
  trigger with no edit here — the property a glob cannot have.
- Arms re-observed: not-owed → `no push due (1 < 400, no CI/doctrine paths touched)`, `exit=0`; owed →
  `EXCEPTIONAL PUSH DUE — 6 unpushed file(s)`, naming the workflow, both project-slot files and the three
  `.doctrine/` seams, `exit=1`; bogus base → `REFUSED`, `exit=2`.
- Validation: `make gate` → `=== all doctrines green ===`; `make check` → `test result: ok. 3 passed;
  0 failed`; `make probes` → `7 suite(s) green`; `bash -n` clean. No push is owed by this slice, so the
  400-commit cadence holds.

## STITCHCAD-SPINE-0017 — the push-cadence exception is derived, not remembered (leaf `SPINE.17`)

- **Director-approved rule change.** The cadence stays 400 commits, with one exception: a commit that
  touches `.github/workflows/`, a `scripts/check_*.sh` doctrine check, the `.doctrine/` seams those
  checks read, or `.githooks/` owes a push immediately — because layer E4 is the un-bypassable backstop
  and such a change is *unverified until a runner executes it*. `COMMIT.md` carries the rule and the
  deriving command; `make push-due` (`scripts/check_push_due.sh`) reports the state and exits 1 when a
  push is owed, listing the triggering files.
- **It is not hypothetical here.** `eb83f01` rewrote `.github/workflows/rust.yml` to add the
  `wasm32-unknown-unknown` target and its smoketest step, and that workflow had never executed anywhere
  until the exceptional push. And the platform risk is measured: this machine's BSD awk lacks `\b` and
  `{n}`, and its `PATH` shadows BSD userland with GNU coreutils — a gate can be green here and behave
  differently on the ubuntu runner.
- **Three arms observed:** not-owed (`0 unpushed commit(s)`, `exit=0`); owed
  (`PUSH_DUE_BASE=051a075 …` → `EXCEPTIONAL PUSH DUE — 6 unpushed file(s) … unverified by CI`, `exit=1`);
  refused (`PUSH_DUE_BASE=nope-not-a-ref` → `exit=2`). The owed arm is testable because the comparison
  base is overridable, so it needs no invented commits.
- **The RED arm caught a defect in the first cut:** the trigger list used the pathspec
  `scripts/check_`, which matches *nothing* — a git pathspec matches whole path components unless it
  carries a wildcard — so every doctrine check script was silently missed. Now `scripts/check_*.sh`, with
  the lesson recorded in the file header. Only the arm that compared against a revision *known* to
  contain those files could reveal it.
- **The exceptional push was made and CI observed, not assumed:** `git push origin main` →
  `051a075..119946b`, and the Actions API for that head sha reports `runs: 2` with **`rust`
  completed `success`** (the first execution of the new WASM smoketest step) and **`doctrines`
  completed `success`**. The verdict is recorded in `G0-CONTRACT.18`'s Verification Log.
- Known over-breadth, stated rather than hidden: the helper matches its own trigger pattern although no
  runner executes it, so landing it owes one further push. Flagged to the director rather than pushed
  unilaterally, since the authorisation was for one exceptional push.
- Validation: `make gate` → `=== all doctrines green ===`; `make check` → `test result: ok. 3 passed;
  0 failed`; `make probes` → `7 suite(s) green`; `bash -n` clean.

## STITCHCAD-G0-0013 — the reference skirt, specified to the millimetre (leaf `G0-CONTRACT.13`)

`docs/book/src/spec/reference-skirt.md` (245 lines) turns the roadmap's prose fixture — "A-line, one
waist dart/side, CB zipper, grain ∥ CB, SA 1 cm sides / 3 cm hem, single notches at side seams" — into a
garment two independent implementers would draft identically. It is the subject of gate G2's CLI replay,
DXF/PDF goldens, printed scale-square check, offset pathology corpus, mutation tests and agent gate.

- **Every number has a source:** 4 body measurements with landmarks and procedures, 2 ease entries with
  fit intent, 9 declared drafting constants, and **17 derived values each shown with its formula** —
  quarter widths, suppression, dart intake and centre, hem width, side-seam slope length, waistband
  length and cut width.
- **The allocation balance closes exactly**, which is the fixture's own internal oracle:
  `4 quadrants × (3.0 side seam + 4.0 dart) = 28.0 cm = garment hip 102.0 − garment waist 74.0`; side
  seams total 12.0 cm and darts 16.0 cm. If a future edit breaks that equality, the recipe is wrong, not
  the check.
- **An interpretation is recorded, not smuggled in:** the roadmap's "one waist dart/side" is realised as
  one dart per *pattern quadrant* (front piece carries two, symmetric about the fold; each back piece
  one), with the arithmetic reason stated — a single dart per body side would need 8.0 cm of intake,
  past the practical single-dart maximum.
- **8 constants are explicitly `assumed`, not `known`:** the five drafting constants (`ss_suppress`,
  `dart_intake`, both dart lengths, `a_line_flare`) and three closure constants need a sewing expert's
  review before the fixture is frozen as a golden at G2, and `G0-CONTRACT.14` owns naming that expert.
  A golden frozen over an unreviewed assumption freezes a guess — so the chapter says which numbers are
  arithmetic and which are judgement.
- It also exercises, by construction: darts with conserved intake, grainlines parallel to CB, variable
  allowances (1 / 1.5 / 3 cm), notches with matched parameters on both sides of a seam, a fold edge
  carrying no allowance, a mirrored pair with L/R labels, the included-vs-excluded allowance policy under
  two profiles, a zero-ease sewing graph (so `walk` must report inside the numerical tolerance class),
  a closure with notions, and recipe replay determinism. A traceability table maps each property to its
  ontology clause and the gate that depends on it, and 8 test obligations close the chapter.
- Validation: `make book` → `exit=0`; `check_live_doc_size.sh` → `OK — 17 surfaces, 15 routes, 51 files
  measured`; `make gate` → `=== all doctrines green ===`; `make check` → 30 tests, 0 failed.

## STITCHCAD-SPINE-0016 — prose is no longer judged as code (leaf `SPINE.16`)

- **Defect D25, and it was blocking product work.** The spine's default code-path regex contains
  `(^|/)src/`, and this project's mdBook chapters live in `docs/book/src/`. Writing a specification
  chapter — the main activity at gate G0 — was therefore classified as a CODE change, and
  `check_fresh_acceptance_evidence.sh` refused the commit demanding tool-output-backed acceptance boxes
  for prose. Taken ahead of the slice it unblocked, per
  `docs/decisions/decision_product-work-takes-the-frontier.md`: a spine slice is legitimate when it
  blocks the product slice about to be taken.
- **`.doctrine/code_paths.txt`** now declares the classification (10 patterns, with the reason in its
  header): crates and `.rs`, `scripts/` and any `.sh`, `Makefile`, `Cargo.toml`/`Cargo.lock`,
  `rust-toolchain.toml`, `.clippy.toml`, CI workflows and `.doctrine/`. Both acceptance checks read the
  same seam, so they cannot disagree. No spine file was edited — the seam is the documented extension
  point, and `.doctrine/README.md` is explicit that a signature family which does not fit the real
  corpus teaches authors to waive it.
- Census over representative paths: `docs` for `docs/book/src/spec/ontology.md`, `README.md` and
  `docs/tasks/SPINE.md`; `CODE` for `crates/sc-units/src/lib.rs`, `scripts/check_live_doc_size.sh`,
  `.doctrine/live_document_size/surfaces.tsv` and `Makefile`.
- The fresh-evidence probe now copies the seam into its throwaway repositories, so it tests the
  configuration this repository actually runs rather than the default it replaced.
- Validation: `make probes` → `7 suite(s) green`; `make gate` → `=== all doctrines green ===`;
  `make check` → `test result: ok. 3 passed; 0 failed`.

## STITCHCAD-G0-0003 — the garment ontology is normative (leaf `G0-CONTRACT.3`)

`docs/book/src/spec/ontology.md` (295 lines, widest line 198 bytes) turns roadmap §3.1's bullet list of
objects into a specification an implementer cannot misread — and that `sc-core`'s skeleton already
points at.

- **Identity first.** ULID entity ids, assigned once and never re-derived from content, plus stable
  *topological* references (`EdgeRef`, `PointRef` with a rational parameter along the edge) instead of
  array indices or tessellation vertices. The persistent-identity contract is a table, not a promise:
  split, merge, reverse, delete and offset-fragmentation each either preserve references or produce a
  **visible repair task**. No silent reassignment, and a design with unresolved references can be saved
  and inspected but not released.
- **Measurement and fit:** `MeasurementTable` entries carry kind (body vs garment POM), landmark,
  procedure, source and state — and a measurement without a landmark or procedure is *rejected*,
  because an unrepeatable measurement cannot be evidence and a factory dispute about "the chest" is a
  dispute about landmarks. `Ease` is a first-class body→garment mapping with fit intent, which is what
  makes regeneration and grade rules reconcilable.
- **The design is a recipe:** parameters, formula graph, ordered operations, revision counter, sewing
  graph, materials, and imported geometry kept as explicit primitives with `origin: imported` and no
  fabricated history. `walk` and `true` are first-class *operations*, not late-added validation checks.
- **Geometry-bearing objects** specified field by field with their invariants: `Piece` (CCW closed
  boundary, holes, internal lines, multiplicity, mirroring, cut-on-fold, face/wrong-side, material,
  layer index, printable label data), `SeamSpan`/`SewingGraph` (oriented, partial and one-to-many
  correspondences with declared ease distribution and stop landmarks), darts/tucks/pleats/gathers with
  conserved intake, `SeamAllowance` as a **per-edge derived object** whose included-vs-generated policy
  is resolved per Factory Profile and never a project-wide boolean, `Notch` whose type, geometry and
  encoding are profile parameters, directed `Grainline` with dual stripe/plaid references, and
  hem/facing/lining/interfacing/closure/pocket as objects rather than drawing conventions.
- **Uncertainty is part of the object:** known / assumed / unknown / preference / derived, with the rule
  that an `unknown` is never silently defaulted — defaults exist only as `preference` values with
  provenance, and the artifact policy matrix decides what an unresolved unknown blocks.
- **Seven test obligations** close the chapter, including identity stability under boundary edits,
  invariants enforced at construction, dart-intake conservation, recipe determinism across platforms,
  and save/load preserving *drafting intent* — two designs with identical contours but different
  recipes remain distinct.
- External claims are labelled, not asserted: the ISO 8559 / ASTM D5219 / EN 13402 / ASTM D5585 roles are
  cited from the roadmap and explicitly owed to the measurement-standards chapter, which owns reading
  the documents.
- The containment checker caught a 238-byte table row against the 200-byte health target for book
  chapters; the verification table became bounded prose and the chapter now sits at 198.
- Validation: `make book` → `exit=0` with the chapter rendered; `check_live_doc_size.sh` →
  `OK — 17 surfaces, 15 routes, 50 files measured`; `make gate` → `=== all doctrines green ===`;
  `make check` → 30 tests, 0 failed; `make probes` → `7 suite(s) green`.

## STITCHCAD-G0-0018 — the first product code: `sc-units` implements the numerical contract (leaf `G0-CONTRACT.18`)

- **`crates/sc-units`** — 1 097 lines of library, 564 lines of tests, **zero dependencies** (so it
  serves the `wasm-viewer` profile and byte-stable golden files): `Length` (i64 micrometres), `Angle`
  (i64 microdegrees, normalized), `Area`, `Ratio` (parts-per-million), `Count`, `Unit` with exact
  integer ratios, the five `ToleranceClass`es, and `UnitError` diagnostics for domain, overflow,
  division-by-zero and non-finite inputs. `#![forbid(unsafe_code)]`; workspace lints deny
  `unwrap_used`/`expect_used`/`panic` so a geometry kernel cannot abort a session on a degenerate input
  (`.clippy.toml` re-allows them in tests, where a test that cannot fail loudly protects nothing).
- **A tolerance cannot be constructed without stating what it was derived from** — `Tolerance::new`
  returns `UnitError::EmptyDerivation` on an empty derivation. That is the mechanical form of "a value
  chosen to make a test pass is not a tolerance" (spec §3).
- **`crates/sc-core`** exists as a documented skeleton: `SCHEMA_VERSION`, the spec path it implements,
  and a table naming which leaf owns each future module (`ontology`, `recipe`, `command`,
  `uncertainty`). It is here because the roadmap's G0 CI clause requires a real WASM build of
  `sc-core` + `sc-units`, not a host `cargo check`.
- **G0 CI shape landed:** `.github/workflows/rust.yml` now installs `wasm32-unknown-unknown` and runs
  fmt → clippy (`-D warnings`) → tests → a real cross-compilation that lists the produced `.rlib`
  files; `make wasm` is the local equivalent. The bedrock starter crate is retired (`git rm
  crates/app`), closing defect D10 ahead of `G1-SLICE.1`.
- **30 tests green** (21 conformance properties with a recorded seed and no dependencies, 5 rounding
  unit tests, 3 skeleton tests, 1 doc-test), each naming the spec clause it discharges: conversion
  round-trips, rounding symmetry, mirror/round commutation, exact ratios, domain and overflow
  diagnostics, division by zero, non-finite rejection, exactness over a million-step chain, angle
  normalization and smallest-turn difference, tolerance inclusivity, class separation, ratio scaling.
- **The tests caught a real API defect before it shipped:** `Ratio` conflated *a percentage* with *a
  multiplier* — `from_percent_rational(2, 100)` returned 200 ppm, self-consistent and a 100× error for
  anyone reading the name as "2 percent". Split into `from_percent(2, 1)` (a percentage value) and
  `from_rational(102, 100)` (the 1.02 a 2 % shrinkage is applied as), each documented with the
  confusion it prevents. Two further failures were wrong expectations on the author's side, corrected
  against the spec: `as_rational_in` returns the *reduced* exact ratio (1 µm = 9/3175 pt, not
  72/25400), and a whole-point round trip is bounded by one internal quantum expressed in points
  (0.00283 pt), not by 1e-6.
- **The containment doctrine's rollover rule fired on this very commit:** `CHANGELOG.md` had reached
  130 % of its health target, so this append performed the declared rollover — slices 1–15 (365 lines /
  30 452 bytes) sealed into `docs/history/stitchcad-changelog-part1.md` with their sha256 recorded,
  a pointer left behind. `LIVE_STATUS.md` and `MEMORY.md` were also trimmed back inside their targets.
- Validation: `cargo fmt --all -- --check` clean; `cargo clippy --all-targets --all-features --
  -D warnings` clean; `cargo test --all` → 30 passed / 0 failed; `make wasm` green; `make gate` →
  `=== all doctrines green ===`; `make probes` → `7 suite(s) green`;
  `check_live_doc_size.sh` → `OK — 17 surfaces, 15 routes, 49 files measured`.
