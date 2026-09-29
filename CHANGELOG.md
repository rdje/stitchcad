# CHANGELOG.md

Newest first. The StitchCAD sections are this project's history; everything below the
_Inherited spine history_ divider is the bedrock scaffold's own changelog, kept as the
provenance of the discipline spine this repository was generated from.

# Sealed archive — earlier slices

Slices 1–15 of this project's changelog (`STITCHCAD-PLANNING-0001` … `STITCHCAD-SPINE-0004c`)
are sealed in [`docs/history/stitchcad-changelog-part1.md`](docs/history/stitchcad-changelog-part1.md)
— 365 lines, 30452 bytes, `sha256:f4aec75ac7dd1fa5…`, immutable. The live window below holds the most
recent entries; when it passes its health target again, the oldest are sealed the same way.

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
