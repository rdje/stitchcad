# Sealed archive — StitchCAD changelog, slices 21–24

Immutable historical segment, sealed out of `CHANGELOG.md` under the rollover rule recorded in
`.doctrine/live_document_size/surfaces.tsv` (row `changelog`, lifecycle `rolling_ledger`): when the live
window passes its health target, the oldest entries are sealed here and a pointer is left behind.

- **Sealed identity:** 147 lines, 12289 bytes, `sha256:14ad52782deb0728284027673e7e3c1f2dcfc5155518f585572b22e55af03ba6`
- **Sealed by:** leaf `G0-CONTRACT.5` on `2026-09-30` — the append that crossed the rollover milestone
  performed the rollover, as the doctrine requires. The window was already in commit order (defect D29 was
  fixed at the previous rollover), so "seal the oldest" sealed the oldest.
- **Coverage:** slices 21–24 — `STITCHCAD-G0-0013`, `STITCHCAD-SPINE-0016`, `STITCHCAD-G0-0003`,
  `STITCHCAD-G0-0018` (oldest first); the file below holds them newest first, exactly as they stood.
- **Predecessors:** `stitchcad-changelog-part2.md` (slices 16–20) and `stitchcad-changelog-part1.md`
  (slices 1–15). part1's own coverage line overstates its range by one slice; the correction is recorded in
  part2's descriptor and in the live pointer, because a sealed segment is never edited (defect D30).
- **Retrieval:** this file, or `git log --follow CHANGELOG.md`. The sealed content is everything below the
  `---` rule and the blank line after it; `shasum -a 256` over exactly those bytes reproduces the digest
  above, and `bash docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh` proves it together with
  the ordering, uniqueness, coverage and pointer rules.
- **Write policy:** none — sealed segments are immutable.

---

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

---

Older StitchCAD slices are in `stitchcad-changelog-part2.md` and `stitchcad-changelog-part1.md`, and the
bedrock scaffold's own changelog in `bedrock-scaffold-changelog.md`. Newer slices are live in
`CHANGELOG.md`, newest first.
