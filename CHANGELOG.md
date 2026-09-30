# CHANGELOG.md

Newest first. The StitchCAD sections are this project's history; everything below the
_Inherited spine history_ divider is the bedrock scaffold's own changelog, kept as the
provenance of the discipline spine this repository was generated from.

# Sealed archive — earlier slices

Slices 1–15 of this project's changelog (`STITCHCAD-PLANNING-0001` … `STITCHCAD-SPINE-0004c`)
are sealed in [`docs/history/stitchcad-changelog-part1.md`](docs/history/stitchcad-changelog-part1.md)
— 365 lines, 30452 bytes, `sha256:f4aec75ac7dd1fa5…`, immutable. The live window below holds the most
recent entries; when it passes its health target again, the oldest are sealed the same way.

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

## STITCHCAD-G0-0002 — the numerical contract is normative (leaf `G0-CONTRACT.2`)

**The first product deliverable.** `docs/book/src/spec/units-and-tolerances.md` (291 lines, widest line
114 bytes) is now the chapter every other chapter and every crate quotes when it uses a number.

- **One internal unit:** lengths are `i64` **micrometres** (1 in = `25 400` exactly, 1 mm = `1 000`),
  angles are `i64` **microdegrees** (a full turn = `360 000 000`), with a declared domain tighter than
  the type (|length| ≤ 10⁹ µm, area ≤ 10¹⁸ µm²) so intermediate arithmetic cannot overflow and an
  out-of-domain value is a typed diagnostic rather than a clamp or a wrap.
- **One rounding rule and one conversion rule:** half away from zero, and every conversion is a single
  multiply-then-divide with an exact integer ratio. Chained conversions are forbidden because they round
  more than once and disagree with the direct conversion — a test fails the chained path, it does not
  document the discrepancy.
- **Five tolerance classes instead of an epsilon:** numerical (1 µm, one quantum — anything more is a
  bug), geometric approximation (10 µm internally, 100 µm chordal for polyline-only receivers), format
  quantization (fixed by the format, published with the artifact), importer comparison (declared per
  receiver in the profile), physical acceptance (**the factory's number, never ours**). Every comparison
  names its class; each value carries the requirement it was derived from, so "why 10 µm?" has an answer
  that is not "it passed".
- **Topology is exact, not approximate:** with integer coordinates, orientation, segment intersection,
  point-in-polygon and winding are computed in 128-bit integers with no epsilon, so
  `if (distance < EPSILON)` over integer coordinates is recorded as a defect. Curved-geometry predicates
  use adaptive precision bounded by the geometric class.
- **The offset engine carries a declared error budget** and fails explicitly — naming the edge, the
  achieved deviation and the requested bound — when a cusp, near-tangency or self-intersection puts it
  out of tolerance. It never emits out-of-tolerance geometry and never silently repairs topology; the G2
  pathology corpus is its oracle.
- **Formats whose quantum is not an integer number of µm** (the PDF point at ≈ 352.78 µm) convert once,
  at serialization, and the quantization is published with the artifact — precision never flows back
  into the model.
- Every arithmetic claim was re-derived, not recalled: `1016 × 25 = 25400` (an HPGL plotter unit is
  exactly 25 µm), `25400 ÷ 72 ≈ 352.78`, `10¹⁸` needs `60` bits and `2 × 10¹⁸` needs `61` (so i128
  products of domain-bounded coordinates are exact), `10⁹ µm = 1 km`, `10¹⁸ µm² = 1 km²`. §8 of the
  chapter labels every external claim as exact arithmetic, cited-from-roadmap, or to-be-confirmed at the
  gate that needs it — no format detail is asserted that nobody has read here.
- Recorded as `docs/decisions/decision_numerical-contract-fixed-point.md` (indexed, with an `answers:`
  line) so the *why* — determinism for golden bytes and CLI replay, decidability of topology, symmetry of
  rounding under mirroring, degrees as the domain's own vocabulary — survives separately from the *what*.
- **The containment ceiling caught this chapter before it shipped:** the first draft's tolerance table
  had rows of `311`–`381` bytes against a `book_collection` maxline ceiling of `320`. It became five
  bounded subsections — a better shape for a book, and the reason max-content-line is its own axis.
- Validation: `make book` → `exit=0` with the chapter rendered; `check_live_doc_size.sh` →
  `OK — 17 surfaces, 15 routes, 47 files measured`; `make gate` → `=== all doctrines green ===`;
  `make check` → `test result: ok. 1 passed; 0 failed`.

## STITCHCAD-PLANNING-0004 — product work takes the frontier (leaf `PLANNING.4`)

- Defect D24 closed with a rule, not a resolution: at the ruling,
  `git log --oneline | grep -cE 'leaf (SPINE|PLANNING|BOOTSTRAP)'` → **20** governance slices against
  `grep -cE 'leaf (G[0-7]|V[12])'` → **0** product slices. Every individual slice was defensible, which
  is why the pattern needed a decision record rather than more care.
- `docs/decisions/decision_product-work-takes-the-frontier.md` states the rule (product takes the
  frontier; spine work only when it blocks the next product slice, when a defect can destroy or corrupt
  work now, or when the director asks), keeps directive §15 intact (every defect is still logged and
  owned — logging is not scheduling), and gives the two-command census that reveals the drift. Wired into
  `CLAUDE.md`'s non-negotiables and the layer-C index.
- The `PLANNING` tree is complete: all four leaves done, the roadmap→tree capture derived by
  `run_tree_coverage_census.sh` (`10 lanes / 13 trees / 0 unowned / 0 orphan(s) / 0 dead link(s)`).
- **This entry landed one commit late, and the reason is worth recording:** the script that was supposed
  to insert it used `str.replace()` on a heading it had not verified, so the replacement silently did
  nothing while the script printed success. Found by `grep -n '^## STITCHCAD' CHANGELOG.md` when the next
  entry would not anchor. Anchor edits are asserted from now on.

## STITCHCAD-PLANNING-0003 — the whole roadmap is captured, and the claim is derived (leaf `PLANNING.3`)

- **All ten roadmap lanes now have a tree.** Seeded the five that were missing — `G5-SHELLS` (14 leaves:
  Tauri native + WASM web shells, the six UX panels, the parity table proven by agent E2E, a real native
  UI suite because MCP tests are not UI tests, the OS/browser matrix, the first complete language pack
  with RTL verified, the minimum tech pack), `G6-CONFORMANCE` (10: DXF importer with an honest loss
  report, real import-filter validation with receiver settings recorded, the foreign-DXF semantic diff
  loop, physical plotter and print checks, a factory pilot cycle whose rejection taxonomy feeds
  calibration, the reliability matrix, cross-platform regressions, fuzzing, the conformance lab itself),
  `G7-RELEASE` (7: independent evidence review, the scoped envelope statement, semver/schema policy,
  install-reopen-upgrade-rollback evidence, release channels, governance in force), `V1-ASSEMBLY`
  (7: mesh, ease-aware seam resampling that is never welded 1:1, net-line binding, arrangement surfaces
  and layer index, viewport precision, blinded validation with false-pos/neg rates) and `V2-SIM`
  (6: `sc-sim` out of the default build, XPBD research as progress reports, labelled approximation,
  calibration and observables protocols declared before data collection, an evidence-only exit gate).
  Repository totals: **13 trees, 142 leaves**.
- **The capture claim is now a command, not a sentence:**
  `bash docs/tasks/artifacts/planning/run_tree_coverage_census.sh` →
  `census: 10 lanes / 13 trees / 0 unowned / 0 orphan(s) / 0 dead link(s)`, `exit=0`. It checks both
  directions — every §11 lane has a tree whose metadata names it, and every tree on disk is registered
  in the index with a declared lane — plus dead index links (defect D1's class). The RED state was
  observed before the fix: mid-slice the same census printed `5 orphan(s)` and exited `1`.
- The clause-versus-leaf table it prints is **advisory and says so**: more clause rows than roadmap
  clauses is expected (a tree may split one clause into several leaves, as `G5-SHELLS` does with the
  "full UX spec" list); fewer rows than clauses is the alarm. A classifier guessing at prose meaning
  would be worse than the side-by-side.
- The tool is bash 3.2 compatible (no `mapfile`) because the spine must run on whatever bash a platform
  ships — the same portability lesson as the awk-versus-grep signature measurement.
- Validation: `make gate` → `=== all doctrines green ===`; `make check` → `test result: ok. 1 passed;
  0 failed`; `make probes` → `7 suite(s) green`; `bash -n` clean.

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
## STITCHCAD-SPINE-0004c — declared ceilings become enforced ones (leaf `SPINE.4.3`)

- **`LIVE-DOC-SIZE` is the second project doctrine** (`scripts/check_live_doc_size.sh`, 271 lines,
  registered in `scripts/check_doctrines.project.sh`, so it runs in the hook and in CI unconditionally
  and judges the resulting tree rather than the staged diff). Bash measures — lines, bytes, max content
  line under `LC_ALL=C`, and for collections file count, per-part maxima and aggregates — one awk pass
  evaluates every rule, which is what makes the evaluator testable against a synthetic registry without
  touching the real tree.
- **It refuses** on an unclassified tracked Markdown surface, a malformed registry row (field count,
  unknown lifecycle or kind, missing owner/authority, non-numeric bound, a ceiling below its own health
  target), an absolute or off-volume path in the data plane, a glob that silently matches nothing, a
  line/byte/maxline/file-count/aggregate overflow, a widened transition-debt baseline, and a route whose
  destination is unclassified or contradicts its lifecycle. It warns at 80 % of a health target.
  `--self-test` → `11 arms, 0 failed`, one per refusal class plus a GREEN control.
- **Coverage proven to have teeth, not assumed:** `run_live_doc_size_probes.sh` → `probes: 4 pass / 0 fail`,
  where `REAL-2` deletes the `roadmap` row from a *copy* of the real registry and the check names
  `ROADMAP.md` as an unclassified live surface, and `MISSING` proves an absent data plane refuses with
  `exit=2` instead of reporting green over nothing. On the real tree:
  `live-doc-size: OK — 17 surfaces, 15 routes, 41 files measured, 19 warning(s)`, `exit=0`.
- **Three target corrections, each with its derivation stated rather than fitted to bloat:** `roadmap`
  health set to `-` (a `maintained_reference` aggregate follows legitimate product scope, so a fixed
  target would be dishonest — the debt baseline and ceiling govern); `doctrine_docs` per-part health
  derived from the largest adopted standard (455 lines / 24 573 bytes) plus ~15 % for a local adoption
  note; and one real fix — the widest `LIVE_STATUS.md` row trimmed from `303` to `207` bytes. Warnings
  `21` → `19`, breaches `0`.
- Remaining warnings are owned, not ignored: `decisions_collection` maxline at 153 % of target →
  `SPINE.15`; `tasks_collection` per-part at 107 % → the convention recorded in its registry row;
  `roadmap` navigation → `SPINE.13`. Defect D13 closed.
- Validation: `make gate` → `=== all doctrines green ===`; `make probes` → `7 suite(s) green`;
  `make check` → `test result: ok. 1 passed; 0 failed`.

## STITCHCAD-SPINE-0014 — the changelog becomes a ledger with an archive terminal (leaf `SPINE.14`)

- **The inherited bedrock changelog is sealed out** of `CHANGELOG.md` into
  `docs/history/bedrock-scaffold-changelog.md`, classified `archive_terminal`. It was `158` lines /
  `11 811` bytes of frozen, untrimmable content occupying 30 % of a rolling ledger's window
  (`git show HEAD:CHANGELOG.md | grep -n '^# Inherited spine history'` → line `365` of `522`).
- **Losslessness is proved by hash, not asserted:** the sealed segment and the original segment are both
  `sha256:78f43e0fe24c60f7bb8b0bb159a2751cc37f967659111bd81df7d74b22dbeca7` → `BYTE-IDENTICAL: True`.
  The archive carries its own identity header (lines, bytes, sha256), provenance, retrieval path and a
  no-write policy; `CHANGELOG.md` keeps a pointer.
- **The ledger is now inside its window:** `522`/`42 124` → `371` lines / `30 713` bytes, widest line
  `185` → `118`, against a health target of 400 / 32 768. Its transition-debt row is cleared, and the
  rollover rule for our own entries is recorded in the registry row.
- **This leaf was pulled ahead of `SPINE.4.3` for a measured reason:** the containment checker, run
  before it was wired into the gate, refused the tree with
  `LIVE-DOC-SIZE: changelog: transition debt WIDENED on lines (522 > baseline 487)`, `exit=1`. A debt
  baseline declared while the surface is still growing breaks on the next slice — so the migration had
  to land before the baseline could be honest.
- Retrieval censuses: `grep -c '^## bedrock-scaffold'` → `6` in the archive, `0` in the ledger;
  `grep -c '^## STITCHCAD' CHANGELOG.md` → `15` (ours stayed).
- Validation: `scripts/check_live_doc_size.sh` → `OK — 17 surfaces, 15 routes, 41 files measured`,
  `exit=0`; `make gate` → `=== all doctrines green ===`; `make check` → `test result: ok. 1 passed`.
