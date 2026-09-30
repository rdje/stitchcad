# G0-CONTRACT — acceptance evidence for completed leaves

The evidence half of [`G0-CONTRACT.md`](G0-CONTRACT.md), split out under the containment registry's own
remedy for `tasks_collection`: "a tree that passes 1000 lines splits its completed-leaf evidence into a
sibling file under `docs/tasks/` before adding more" (`.doctrine/live_document_size/surfaces.tsv`).

**The convention, and the gate that forces it.** `scripts/check_task_acceptance.sh` judges EVERY staged
`docs/tasks/*.md` file and refuses one that carries no ticked ROOT CAUSE / ADDRESSED / NO REGRESSION box,
so a tree file may not be emptied of checklists: the leaf being landed keeps its checklist in the tree
file, and the next slice moves it here. Two properties follow, and both are the point of
`docs/decisions/decision_acceptance-evidence-per-leaf.md` — the tree file's FIRST matching box is the
current leaf's, so no stale evidence can shadow it (defect D15's facet 1), and this file holds only
completed, ticked, evidence-backed checklists, never a placeholder (facet 2).

Order is landing order, oldest first, so this file reads the same way the tree's Commit Log does.

### `G0-CONTRACT.2` — the numerical contract is written down

- [x] **ROOT CAUSE (WHY + WHERE)** — the G0 exit list requires a units & tolerance policy, and before
  this leaf it existed only as roadmap prose (§4.2: "Single internal unit: fixed-point micrometers
  (i64) recommended" — note *recommended*, i.e. still a choice): `git ls-tree --name-only HEAD
  docs/book/src/spec/` → `index.md` alone, `rc=0`, and `git ls-files 'crates/*'` → the bedrock starter
  crate, `rc=0`. Two implementers reading §4.2 would have chosen different angle units, different
  rounding and different epsilons, and the divergence would have surfaced as golden-file diffs at G2.
- [x] **ADDRESSED (verified)** — `docs/book/src/spec/units-and-tolerances.md` is now the normative
  chapter (`wc -lc` → `291` lines / `17180` bytes, widest line `114` bytes), linked from `SUMMARY.md`,
  and the book builds:
  `make book` → `INFO HTML book written to …`, `exit=0`, producing `docs/book/book/spec/units-and-tolerances.html`.
  It fixes the open choices: i64 micrometres for length, i64 microdegrees for angle, a declared domain
  tighter than the type, half-away-from-zero rounding, single-step conversions, five tolerance classes
  each with its setter and derivation, exact integer topology predicates, and the offset error budget.
  Every arithmetic claim in it was re-derived rather than recalled: `1016 × 25 = 25400` (so an HPGL
  plotter unit is exactly 25 µm), `25400 ÷ 72 = 352.78` µm per PDF point (not an integer, hence the
  format-quantization class), `10¹⁸` needs `60` bits and `2 × 10¹⁸` needs `61` (so i128 products of
  domain-bounded coordinates are exact), `10⁹ µm = 1 km`, `10¹⁸ µm² = 1 km²`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`;
  `bash scripts/check_live_doc_size.sh` → `live-doc-size: OK — 17 surfaces, 15 routes, 47 files
  measured`, `exit=0`, with the new chapter claimed by `book_collection`; `make check` →
  `test result: ok. 1 passed; 0 failed`; `make book` → `exit=0`.
  The containment ceiling set two slices earlier earned its keep on this chapter: the first draft
  carried five tolerance-class table rows of `311`–`381` bytes against a `book_collection` maxline
  ceiling of `320`, so the widest was over the limit before the chapter ever shipped. The table became
  five bounded subsections (widest line now `114` bytes) — which is also the more readable shape in a
  book, and the reason maximum-content-line is a separate axis from lines and bytes.
- [x] **FIX** — wrote the chapter; recorded the coupled choices as
  `docs/decisions/decision_numerical-contract-fixed-point.md` (indexed, with an `answers:` line) so the
  *why* survives separately from the *what*; labelled every external claim in §8 of the chapter with
  its verification status (exact arithmetic / cited-from-roadmap / to be confirmed at the gate that
  needs it) instead of asserting format details nobody has read here.
- [x] **LOCKSTEP** — `SUMMARY.md` grows the chapter; `LIVE_STATUS.md` moves G0 to In Progress;
  `MEMORY.md` points at the next leaf; `CHANGELOG.md` records the slice; the derived Knowledge Map
  picks up the new decision record.

### `G0-CONTRACT.18` — the first product code: `sc-units` implements the numerical contract

- [x] **ROOT CAUSE (WHY + WHERE)** — the roadmap's G0 CI clause (§4.3 "CI grows with stages (G0:
  fmt/clippy/unit+property/WASM smoketest)", §7.3 "WASM CI at G0 = smoketest that `sc-core` +
  `sc-units` compile to `wasm32-unknown-unknown`") had nothing to build:
  `git ls-tree -r --name-only HEAD | grep -c 'crates/sc-'` → `0`, `rc=1`, and the only crate in the
  workspace was the bedrock starter, whose `main` printed `bedrock: replace this crate with your
  project` (defect D10). A WASM smoketest over zero crates is a green light on nothing.
- [x] **ADDRESSED (verified)** — `crates/sc-units` (1 097 lines of library, 564 lines of tests,
  **zero dependencies** so it serves the `wasm-viewer` profile) now implements the `.2` chapter:
  `Length` (i64 µm), `Angle` (i64 µ°, normalized), `Area`, `Ratio` (ppm), `Count`, `Unit` with exact
  integer ratios, the five `ToleranceClass`es with a **mandatory derivation** field, `UnitError`
  diagnostics for domain/overflow/division-by-zero/non-finite, and `#![forbid(unsafe_code)]`.
  `crates/sc-core` exists as a documented skeleton naming which leaf owns each future module.
  `cargo test --all` → **30 tests, 0 failed** (21 conformance properties + 5 rounding unit tests +
  3 skeleton tests + 1 doc-test); `make wasm` →
  `wasm-viewer smoketest: sc-units + sc-core build for wasm32-unknown-unknown`; the CI workflow gained
  the `wasm32-unknown-unknown` target and a smoketest step that lists the produced `.rlib` files.
- [x] **NO REGRESSION** — `cargo fmt --all -- --check` → clean, `exit=0`;
  `cargo clippy --all-targets --all-features -- -D warnings` → `Finished`, `exit=0`;
  `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` → `7 suite(s) green`. The
  starter crate was retired with `git rm crates/app` (its only content was a template message and a
  `2 + 2` test), so D10 closes here rather than at `G1-SLICE.1`.
- [x] **FIX** — added `crates/sc-units` (7 modules) and `crates/sc-core`; workspace lints tightened
  (`unsafe_code = "forbid"`, `missing_docs`, and `unwrap_used`/`expect_used`/`panic` denied so a
  geometry kernel cannot abort a session on a degenerate input — with `.clippy.toml` re-allowing them
  in tests, where a test that cannot fail loudly protects nothing); `rust-toolchain.toml` pins the
  wasm32 target; `Makefile` gained `make wasm`; the `clippy::all` group needed `priority = -1` to
  coexist with individual lint levels.
- [x] **The tests caught a real API defect before it shipped:** `Ratio` conflated *a percentage* with
  *a multiplier*. `from_percent_rational(2, 100)` returned 200 ppm — self-consistent, and a 100× error
  for anyone reading the name as "2 percent". The constructors are now `from_percent(2, 1)` (a
  percentage value → multiplier 0.02) and `from_rational(102, 100)` (the 1.02 a 2 % shrinkage is
  applied as), each documented with the confusion it prevents. Two further failures were wrong
  expectations on my side, corrected against the spec: `as_rational_in` returns the *reduced* exact
  ratio (1 µm = 9/3175 pt, not 72/25400), and a whole-point round trip is bounded by one internal
  quantum expressed in points (0.00283 pt), not by 1e-6.
- [x] **LOCKSTEP** — `knowledge-map/subsystems.md` now names both crates and the spec directory (the
  map was still reporting "no subsystems documented"); `TOOLBOX.md` gained the `make wasm` row;
  D10 closed in `PLANNING.md`; `LIVE_STATUS.md`, `MEMORY.md`, `CHANGELOG.md` and the derived Knowledge
  Map updated in this commit.

### `G0-CONTRACT.3` — the garment ontology is written down

- [x] **ROOT CAUSE (WHY + WHERE)** — the ontology existed only as roadmap prose: §3.1 lists the objects
  in bullets with no fields, no invariants and no identity rules, so `sc-core`'s skeleton could name the
  module it owes (`grep -c 'G0-CONTRACT.3' crates/sc-core/src/lib.rs` → `1`, `rc=0`) but had nothing to
  implement against. `git ls-tree --name-only HEAD docs/book/src/spec/` → `index.md`,
  `units-and-tolerances.md`, `rc=0`: no ontology chapter, while `.4`, `.5`, `.13`, `G1-SLICE.3` and
  `G1-SLICE.4` all depend on it.
- [x] **ADDRESSED (verified)** — `docs/book/src/spec/ontology.md` now specifies all of it:
  `wc -lc` → `295` lines / `16187` bytes, widest line `198` bytes (inside the `book_collection`
  per-part health of 400 lines / 24 576 bytes / 200 B). It covers ULID entity identity and the stable
  topological reference contract with a per-edit preservation table (split, merge, reverse, delete,
  offset-fragmentation → preserved or a visible repair task, never silent reassignment);
  `MeasurementTable` with landmark and procedure mandatory and body-vs-POM kinds distinct; `Ease` as a
  first-class body→garment mapping with fit intent; `Design` as recipe (formula graph + ordered
  operations + revision) with `walk`/`true` as first-class operations; `Piece`, `SeamSpan`/`SewingGraph`,
  `Dart`/`Tuck`/`Pleat`/`Gather`, `SeamAllowance` as a per-edge derived object with profile-resolved
  inclusion, `Notch`, `Grainline`, hem/facing/lining/interfacing/closure/pocket, materials, the five
  uncertainty states, canonical serialization, and seven named test obligations. Wired into
  `SUMMARY.md` and the spec index; `make book` → `INFO HTML book written to …`, `exit=0`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`;
  `bash scripts/check_live_doc_size.sh` → `live-doc-size: OK — 17 surfaces, 15 routes, 50 files
  measured`, `exit=0`; `make check` → `test result: ok. 3 passed; 0 failed` (30 across the workspace);
  `make probes` → `7 suite(s) green`. No product code changed, so no acceptance evidence was owed
  beyond this record.
- [x] **FIX** — wrote the chapter; converted its verification-status table to bounded prose after the
  containment checker reported a `238`-byte row against a `200`-byte health target (the chapter now sits
  at `198`); linked it from the spec index and `SUMMARY.md`.
- [x] **LOCKSTEP** — `LIVE_STATUS.md`, `MEMORY.md`, `CHANGELOG.md` updated; `knowledge-map/subsystems.md`
  already names `docs/book/src/spec/` as a subsystem, so the derived map needed no edit;
  `sc-core/src/lib.rs`'s module table already points at this leaf as the specifier.

### `G0-CONTRACT.13` — the reference skirt, specified to the millimetre

- [x] **ROOT CAUSE (WHY + WHERE)** — gate G2's exit criteria and the mutation, golden, offset and agent
  suites all need one subject garment, and the roadmap's G0 fixture clause described it in prose only:
  "A-line, one waist dart/side, CB zipper, grain ∥ CB, SA 1 cm sides / 3 cm hem, single notches at side
  seams, cut-on-fold or paired front". Two implementers would have produced two different skirts, and
  every comparison between them would have been meaningless. `git ls-tree --name-only HEAD
  docs/book/src/spec/` → `index.md`, `ontology.md`, `units-and-tolerances.md`, `rc=0`: no fixture.
- [x] **ADDRESSED (verified)** — `docs/book/src/spec/reference-skirt.md` (`wc -lc` → `245` lines /
  `14538` bytes, widest line `118`) specifies the garment completely: 4 body measurements with
  landmarks, 2 ease entries with fit intent, 9 declared drafting constants, **17 derived values each
  with its formula**, a 9-step drafting recipe, 6 pieces with multiplicity/fold/pair/material/layer,
  per-edge allowances with corner treatment and policy, notch and grainline placement, a 4-span sewing
  graph with declared zero ease, the closure, a fixture-property → ontology-clause → dependent-gate
  traceability table, and 8 test obligations. The arithmetic was re-derived rather than asserted, and
  the allocation **balance closes exactly**: `4 × (3.0 + 4.0) = 28.0` = `garment_hip − garment_waist`
  = `102.0 − 74.0`; side seams total `12.0` cm and darts `16.0` cm, summing to the same `28.0`.
- [x] **NO REGRESSION** — `make book` → `INFO HTML book written to …`, `exit=0` with the chapter
  rendered; `bash scripts/check_live_doc_size.sh` → `live-doc-size: OK — 17 surfaces, 15 routes, 51
  files measured`, `exit=0`; `make gate` → `=== all doctrines green ===`, `exit=0`; `make check` →
  `test result: ok. 3 passed; 0 failed` (no product code changed).
- [x] **FIX** — wrote the chapter; **recorded an interpretation rather than silently choosing one**
  (the roadmap's "one waist dart/side" is realised as one dart per quadrant, with the arithmetic reason:
  a single dart per body side would need 8.0 cm of intake); marked the 5 drafting constants and 3
  closure constants as `assumed` pending domain review, with `G0-CONTRACT.14` named as the leaf that
  owns naming the reviewer, so a golden frozen at G2 cannot freeze an unreviewed guess as if it were a
  fact; converted the §11 verification table to bounded prose when the containment checker reported a
  `276`-byte row against the `200`-byte book health target.
- [x] **LOCKSTEP** — chapter linked from `SUMMARY.md` and the spec index; `LIVE_STATUS.md`, `MEMORY.md`,
  `CHANGELOG.md` updated; the fixture's test obligations are the G2 leaves' acceptance inputs, so
  `G2-2D.3`/`.8`/`.11` now have numbers to assert against.

### `G0-CONTRACT.1` — the glossary, and the census that derives its claims

- [x] **ROOT CAUSE (WHY + WHERE)** — the G0 exit clause list requires a "glossary of construction terms"
  and none existed: `git ls-tree --name-only HEAD docs/book/src/spec/` → `index.md`, `ontology.md`,
  `reference-skirt.md`, `units-and-tolerances.md`, `rc=0`, and
  `git ls-tree -r --name-only HEAD docs/book/src/spec/glossary/ | wc -l` → `0`. Three normative chapters
  were already using the vocabulary with nothing binding a term to one meaning — `ontology.md` said
  "Terms used below are defined in the glossary" and pointed at a file that did not exist. The population
  needing definitions is measurable rather than a matter of taste: the census counts `82`
  identifier-shaped machine tokens and `183` bolded spans across the spec chapters. The blocking part was
  worse than absence: the reference fixture used `19` tokens no table declared (D26), so a glossary written
  over that chapter would have defined words the chapter's own formulas contradicted — hence `.13b` first.
- [x] **ADDRESSED (verified)** — `docs/book/src/spec/glossary.md` plus eight domain parts under
  `docs/book/src/spec/glossary/` carry **239 terms**, each with a plain-language meaning, the canonical
  object that specifies it, the synonyms factories and other CADs use, and the machine token; `150` tokens
  are owned, `19` cross-referenced with `→`, `22` entries are required to carry the safety mark and `72`
  do. Every claim the glossary makes about itself is derived, not asserted:
  `bash docs/tasks/artifacts/glossary/run_glossary_census.sh` →
  `glossary census: 239 terms / 8 parts / 138 tokens / 0 failure(s)`, `exit=0`, with `155` canonical-object
  references resolved and `0` dead, `82` tokens used by the spec set and `0` unaccounted, index↔parts
  drift `0`. Its discrimination is proved, not assumed:
  `bash docs/tasks/artifacts/glossary/run_glossary_probes.sh` → `probes: 10 pass / 0 fail` (two GREEN arms
  and eight RED arms that break one property each in a copy and require the census to name it). The book
  builds with all nine glossary pages: `make book` → `INFO HTML book written to …`, `exit=0`,
  `ls docs/book/book/spec/glossary/*.html | wc -l` → `8`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `9 suite(s) green` (`62` arms across nine suites, `0 fail`); `make check` → `test result: ok. 21 passed;
  0 failed` and `ok. 5 passed` and `ok. 3 passed` across the workspace, no product code touched;
  `bash scripts/check_live_doc_size.sh` → `live-doc-size: OK — 17 surfaces, 15 routes, 62 files measured`,
  `exit=0`. Containment is respected per part (largest glossary part `48` lines / `7 471` B against a
  health of `400` / `24 576`; the index chapter `369` lines / `21 052` B) with **one axis deliberately over
  health and inside its ceiling**: the widest table row is `268` B against a `200` B maxline health and a
  `320` B ceiling, because a five-column termbase row is not the shape the prose-derived health target was
  measured from. It is recorded here rather than paid for in vaguer definitions; if a later chapter pushes
  the collection past `320` B the answer is a recorded decision and a table-shaped health target, not
  silent trimming.
- [x] **FIX** — wrote the index chapter (the machine-token rule, the ⚠ policy, the parts table, the
  derived A–Z index, the termbase relationship to `G0-CONTRACT.16`, and the verification status of every
  synonym class) and the eight parts; partitioned rather than filed as one ~70 KB chapter, which would
  have breached the `book_collection` per-part ceiling within two more chapters; made the index derived
  (`--emit-index`) and compared in both directions so it cannot drift; built the census as the tracked
  producer and the probe suite as its ground truth; added
  `docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh` because this slice's changelog append
  crossed the rollover milestone and the doctrine's step 6 requires ordering, uniqueness and retrieval
  checks that nothing executed before.
- [x] **The probes found three defects in the census before it found one in the glossary** — which is the
  argument for RED arms over reading your own code. (1) `R1` read the *meaning* column instead of the
  object column, so it examined no cell at all and printed `dead references: 0` — a green verdict on a
  rule that never ran; only `DEAD-CLAUSE` failing to fire revealed it. (2) The clause slice was one byte
  off: `§` is two bytes and `LC_ALL=C` makes awk count bytes, so every extracted clause carried a stray
  `\xa7` and matched nothing. (3) `resolve()` collapsed `a/b/../c.md` by deleting `/../`, leaving
  `a/b/c.md`, so all `155` references reported dead until the pattern also removed the parent segment.
  All three were caught by probes; none by inspection.
- [x] **The rollover this slice triggered is performed and verified in the same commit** — the live window
  crossed its health target, so the five oldest entries are sealed into
  `docs/history/stitchcad-changelog-part2.md` (`152` lines / `12 811` B / `sha256:5783ac36d8bc7cee…`), the
  window is reordered into commit order (D29), the stale "_Inherited spine history_ divider" sentence is
  corrected (D31), and part1's wrong coverage line is corrected by superseding record rather than by
  editing an immutable segment (D30). Losslessness is proved, not claimed: all `12` pre-existing entries
  have byte-identical bodies across live+sealed, `0` missing, `0` changed, no id both live and sealed; and
  the ledger probe → `probes: 6 pass / 0 fail`, including part1's `365` lines still reproducing
  `sha256:f4aec75ac7dd1fa5…`.
- [x] **LOCKSTEP** — `SUMMARY.md` gains the glossary and its eight parts; `spec/index.md` links the
  Glossary row; `ontology.md`'s forward reference resolves; `TOOLBOX.md` gains the census and its probe
  suite; `knowledge-map/subsystems.md` names the glossary as a subsystem (the derived map picks up the new
  decision record); `docs/decisions/decision_machine-tokens-declared-where-used.md` is the layer-C record
  the `DEV_NOTES.md` lesson is promoted to; D29/D30/D31 closed in `PLANNING.md`; `LIVE_STATUS.md`,
  `MEMORY.md` and `CHANGELOG.md` updated in this commit.

### `G0-CONTRACT.4` — the release claim gets a boundary, and the boundary is derived

- [x] **ROOT CAUSE (WHY + WHERE)** — the G0 exit list requires a "supported/rejected/deferred feature
  matrix" and roadmap §3.2 promises "A supported/rejected/deferred feature matrix is a G0 deliverable",
  but the boundary existed only as prose in two places that do not enumerate: `git ls-tree --name-only
  HEAD docs/book/src/spec/` → `glossary/`, `glossary.md`, `index.md`, `ontology.md`,
  `reference-skirt.md`, `units-and-tolerances.md`, `rc=0` — no matrix. Without one, three populations
  stay unchecked: the ontology's object clauses (a feature nobody dispositioned is a feature a factory
  discovers), roadmap §1.3's non-goals (a refusal a partner does not know about is a refusal they test),
  and roadmap §3.2's envelope. The gap is not hypothetical — writing the rows surfaced **D32**: collar,
  trousers, buttons, pockets and the fly they imply are inside the declared envelope and outside every
  gate's exit criteria, `sed -n '/### G3 /,/### G4 /p' ROADMAP.md | grep -ci 'collar\|trousers\|button\|pocket'`
  → `0`. *(Corrected by `.4b`, defect **D41**: that command yields `1`, because a section range also
  contains G3's complexity note, and the note says "a shirt/trousers intermediate". The claim holds over
  the exit criteria alone — `sed -n '702,708p' ROADMAP.md | grep -ci 'collar\|trousers\|button\|pocket'`
  → `0` — so the conclusion was right and the cited range was wrong.)*
- [x] **ADDRESSED (verified)** — `docs/book/src/spec/feature-matrix.md` (`306` lines / `28 646` bytes,
  widest line `197`) dispositions **105 rows**: `76` supported, `19` rejected, `10` deferred; `29`
  diagnostic tokens are declared in §10 with the arguments each must carry, and §1 fixes the three rules
  (no silent approximation, a supported row names its proof, modelled is not supported). Its acceptance
  clause — "nothing in the ontology is silently unlisted" — is derived, not asserted:
  `bash docs/tasks/artifacts/feature_matrix/run_feature_matrix_census.sh` →
  `feature-matrix census: 105 rows / 29 diagnostics / 0 failure(s)`, `exit=0`, with `16` required ontology
  object clauses all cited, `8` of `8` roadmap §1.3 non-goals matched to a rejected row (the mapping is
  printed), `5` of `5` envelope garments supported and `3` of `3` named refusals rejected, and every gate
  cell resolving to a real roadmap §11 gate or to `unnamed (D32)`. The glossary absorbed the `26` terms
  the matrix introduces (`239` → `265`), re-derived:
  `bash docs/tasks/artifacts/glossary/run_glossary_census.sh` → `265 terms / 8 parts / 139 tokens /
  0 failure(s)`, `exit=0`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `10 suite(s) green` (`72` arms, `0 fail`), including the new
  `run_feature_matrix_probes.sh` → `probes: 10 pass / 0 fail` and the glossary suite still at
  `10 pass / 0 fail`; `make check` → `5` `test result: ok` lines across the workspace, no product code
  touched; `make book` → `INFO HTML book written to …`, `exit=0` with `6` spec pages rendered;
  `bash scripts/check_live_doc_size.sh` → `OK — 17 surfaces, 15 routes, 64 files measured`, `exit=0`.
  Containment is inside every ceiling with two axes over *health* and recorded here rather than trimmed:
  the matrix part is `28 646` B against a `24 576` B per-part health (117 %, ceiling `40 960`), and the
  glossary's widest table row is `268` B against a `200` B health (ceiling `320`) — a five-column
  reference table is not the prose shape those health targets were derived from. `subsystems_input` is at
  `3 131` B against `3 072` (102 %, ceiling `6 144`); its registry row already names `SPINE.5` as the
  owner of re-deriving that target.
- [x] **FIX** — wrote the chapter (dispositions and their three rules, the garment family, five feature
  sections, the non-goal table, the D32 gap table, the diagnostic contract, what G7's statement must
  carry, verification status, the derivation, and the test obligations); added `26` glossary entries and
  regenerated the A–Z index with `--emit-index`; built the census as the tracked producer and its probe
  suite as ground truth; logged D32 with the census as its reproduce command so the gap stays visible on
  every run. The DEV_NOTES lesson's promotion decision, on one line because the gate reads lines:
  promotion: declined (the rule binds probe authors, so it went into TOOLBOX.md's probe conventions, which CLAUDE.md step 3 sends every agent to; a layer-C record would duplicate it).
- [x] **Two probe arms were wrong before the census was right, and the census was right** — measured, not
  assumed. `UNDECLARED-DIAG` renamed `` `ngo_costing` `` everywhere, which renames the §10 declaration and
  its row together and therefore changes nothing the census can see; `UNCITED-ONTOLOGY` de-cited one of the
  three rows citing `ontology §4.4`, leaving the clause covered. Both arms passed for the wrong reason —
  exit `0` where a refusal was owed — until each mutation became the *only* copy of the property. The
  general rule, which is the same one D15's shadowing probe taught: **a RED arm must remove the property,
  not one instance of it.** A third bug was mine and not the arms': replacing a `{2,4}` interval with
  `###+` for portability silently dropped every `##`-level heading, so M3 required three clauses fewer
  and reported green. Only `REAL` plus the citation arm caught it, which is why both exist.
- [x] **LOCKSTEP** — `SUMMARY.md` and `spec/index.md` link the chapter; the glossary grew `26` terms and
  re-derived its index; `TOOLBOX.md` gains the matrix census and its probe suite (and the changelog-ledger
  probes the previous slice added but had not listed); `knowledge-map/subsystems.md` names the matrix as a
  subsystem, so the derived map carries it; D32 logged in `PLANNING.md`; `LIVE_STATUS.md`, `MEMORY.md`,
  `CHANGELOG.md` and `docs/TASK_TREE.md` updated in this commit.

### `G0-CONTRACT.6` — size-set ownership decided, and the commit path stops needing a hand-carried variable

- [x] **ROOT CAUSE (WHY + WHERE)** — roadmap §3.4 leaves the owner of the size set open at G0 ("whether it
  lives in the Design, the Factory Profile, or a third Order object is decided at G0") and ontology §2.3
  forwards to a chapter that did not exist: `git ls-tree --name-only HEAD docs/book/src/spec/` →
  `feature-matrix.md`, `glossary/`, `glossary.md`, `index.md`, `instantiation-paths.md`, `ontology.md`,
  `reference-skirt.md`, `units-and-tolerances.md`, `rc=0`. Two consequences were already live: `.5` consumes
  a base size and breaks it could not name an owner for, and roadmap §7.5's rule that quantities belong to an
  Order object had nothing enforcing it, so a size set could have grown a ratio and coupled every commercial
  change to a design revision. A second, unrelated cause is fixed in the same commit: the ledger probe
  needed `LEDGER_PENDING=<id>` in the environment, so `make probes` — which `COMMIT.md` requires before a
  push — was red during every commit that appended a changelog entry (`make probes` → `Error 1`, measured
  twice this session).
- [x] **ADDRESSED (verified)** — `docs/decisions/decision_size-set-ownership.md` (context, three candidate
  owners argued, six operative rules, consequences, re-open condition, `answers:` line, indexed) and
  `docs/book/src/spec/size-sets.md` (`187` lines / `12 812` B, widest line `216`): ten fields with types and
  requiredness, the label/order separation with the sort that proves it ("XS, S, M, L, XL, 2XL" sorts 2XL
  first), the base-size rules, breaks per adjacent pair, multi-dimensional axes, the five designation
  systems stated as *what the model must express* with EN 13402 and ASTM D5585 content explicitly not
  asserted and owned by `.7`, and MTM as a set of one. Both censuses green:
  `run_glossary_census.sh` → `275 terms / 8 parts / 144 tokens / 0 failure(s)`, `exit=0` (the glossary grew
  by `5` terms and re-derived its index); `run_feature_matrix_census.sh` →
  `105 rows / 29 diagnostics / 0 failure(s)`, `exit=0`. The probe seam is now derived: with no environment
  variable set, `run_changelog_ledger_probes.sh` → `probes: 6 pass / 0 fail` and `make probes` →
  `10 suite(s) green`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `10 suite(s) green` (`73` arms, `0 fail`), including the matrix suite at `probes: 11 pass / 0 fail` after
  gaining the M7 link rule and its `DEAD-LINK` arm; `make book` → `exit=0` with `8` spec pages rendered;
  `bash scripts/check_live_doc_size.sh` → `OK — 17 surfaces, 15 routes, 67 files measured`, `exit=0`, with
  `memory_pointer` back inside its `34`-line health target after trimming and every book part under its
  per-part ceilings; no product code changed, so `make check` is unaffected (`test result: ok` across the
  workspace on the last run that touched Rust).
- [x] **FIX** — wrote the record and the chapter; renamed two size-set fields that would have collided with
  tokens the glossary already owns (`system` → `size_system`, `base` → `base_size`) so one token keeps one
  meaning; gave `provenance` and `chart` their own glossary tokens; added the M7 link rule to the matrix
  census because this slice introduced cross-chapter links into a normative table; replaced the ledger
  probe's hand-carried variable with a derivation from the working tree (the newest live entry is excused
  only when `git diff HEAD` shows this tree adding it, so a mistyped id fails on the next run instead of
  being excused forever).
- [x] **The census caught a convention violation in the chapter before it shipped** — and the fix was the
  rule, not an exemption. Three example size labels were written in backticks, so C1 reported `S`, `M` and
  `L` as machine tokens nothing declared: `glossary census … 3 failure(s)`. A size label is prose a factory
  reads, not an identifier a program reads, so labels are now quoted and §3 of the chapter states the
  distinction normatively. That is the token decision record applying itself to a case its author had not
  thought about, which is the only evidence a convention is real.
- [x] **LOCKSTEP** — `SUMMARY.md` and `spec/index.md` link the chapter; `ontology.md` §2.3's forward
  reference resolves; six matrix rows now cite the chapters that specify them (which is what made M7
  necessary); `docs/decisions/INDEX.md` carries the new record; `TOOLBOX.md`'s matrix row names the link
  rule; the glossary grew `5` terms with its index re-derived; `LIVE_STATUS.md`, `MEMORY.md`,
  `CHANGELOG.md` and `docs/TASK_TREE.md` updated in this commit.

### `G0-CONTRACT.7` — the standards registry, and the discipline that keeps a citation honest

- [x] **ROOT CAUSE (WHY + WHERE)** — the G0 exit clause requires the "measurement/POM standards identified
  (ISO 8559, ASTM D5219, EN 13402, ASTM D5585)" and `git ls-tree --name-only HEAD docs/book/src/spec/` →
  eight entries, none of them a standards chapter, `rc=0`. Three chapters had already routed their
  standards claims to it: `ontology.md` §8 ("the standards' texts have not been read in this repository"),
  `reference-skirt.md` §11 ("the measurement-standards chapter owns reconciling them") and `size-sets.md`
  §8 — three deferrals to a document that did not exist, which is the shape a claim takes when it is
  waiting to be asserted by nobody. Meanwhile six designations were already in use across the book with no
  registry, no status and no owner: `grep -roE 'ISO [0-9]+|ASTM D[0-9]+|EN [0-9]+|AAMA' docs/book/src |
  wc -l` → `33` occurrences in `9` files.
- [x] **ADDRESSED (verified)** — `docs/book/src/spec/standards.md` (`204` lines / `13 849` B, widest line
  `187`) registers all six designations with role, what is adopted, what is deliberately not, a status from
  the closed three-word vocabulary and a named owner; states the two rules that follow (no clause or table
  of a standard appears without `read-in-repo` status, and a standard is data with provenance, never
  authority); gives the four requirements the model places on any standard; dispositions all five
  deferrals in a ledger; and names the verification plan's real dependency (`.14` names the reviewer,
  procurement obtains the texts, and until then no claim can become `read-in-repo`). The registry claim is
  derived: `bash docs/tasks/artifacts/standards/run_standards_census.sh` →
  `standards census: 6 registered / 6 designations used / 0 failure(s)`, `exit=0`, with a per-designation
  list of every file that uses it; `bash docs/tasks/artifacts/standards/run_standards_probes.sh` →
  `probes: 6 pass / 0 fail`, including an arm that smuggles "per ISO 4915" into a chapter and requires the
  census to name it.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `11 suite(s) green`; `make book` → `exit=0` with `9` spec pages rendered;
  `bash scripts/check_live_doc_size.sh` → `OK — 17 surfaces, 15 routes, 69 files measured`, `exit=0`, the
  chapter at `153` lines inside its `400`-line per-part health; the other two censuses still green —
  `run_glossary_census.sh` → `275 terms / 8 parts / 144 tokens / 0 failure(s)` and
  `run_feature_matrix_census.sh` → `105 rows / 29 diagnostics / 0 failure(s)`; `make check` unaffected (no
  Rust touched).
- [x] **FIX** — wrote the chapter and the census; normalized the registry's first cells to the designations'
  base forms (`ASTM D6673`, not `ASTM D6673-10`) because the census matches those keys against the whole
  book; **the containment ceiling refused the first draft** — the six-column registry table had rows of
  `410` B against a `320` B maxline ceiling and the deferral ledger `398` B, so the registry became four
  columns plus one bounded subsection per standard, and the ledger became bounded prose entries, exactly
  the remedy `G0-CONTRACT.2` recorded when the same ceiling caught its tolerance table; taught S3 to read a status token that carries its source in parentheses, which the first cut
  rejected as an invented status; fixed a `grep -c … || printf 0` that printed `0⏎0` — the same trap
  DEV_NOTES recorded on 2026-09-04, caught here by its own output.
- [x] **The glossary census caught two prose spans wearing token formatting before the chapter shipped** —
  `` `blocked` `` (a task-tree status, not a product identifier) and `` `AAMA` `` inside a sentence
  describing the census's match shapes. C1 reported both as machine tokens nothing declared
  (`2 failure(s)`), and the fix was to de-tokenize the prose rather than to exempt it: the convention says a
  backticked span is an identifier a program reads, and these were not. That is the third time the token
  census has corrected a chapter at authoring time rather than in review, which is the evidence that the
  convention is load-bearing rather than decorative.
- [x] **LOCKSTEP** — `SUMMARY.md` and `spec/index.md` link the chapter; `docs/TASK_TREE.md`'s frontier cell
  and execution-order line are corrected (the immediate half of D34, whose durable half is `PLANNING.5`);
  `TOOLBOX.md` gains the standards census and its probe suite; `knowledge-map/subsystems.md` lists the
  chapter; `LIVE_STATUS.md`, `MEMORY.md`, `CHANGELOG.md` and this tree updated in this commit.

### `G0-CONTRACT.13d` — one waistband, and the instrument that keeps it one

- [x] **REPRODUCE / ISSUE** — defect D27: §4 published the cut width of ONE band folded lengthwise
  (`2 × wb_width + sa_waist + sa_wb_bottom` = 10.0 cm) while §6 listed a faced two-piece band
  (`waistband_outer`, `waistband_inner`, `waistband_interfacing`, each piece 6.0 cm) and §8 sewed only the
  outer one, so §12's count of 6 was the rejected reading's. Reproduced by the instrument this leaf lands,
  run over the chapter as committed: `git show HEAD:docs/book/src/spec/reference-skirt.md >
  target/scratch/before.md && FIXTURE_CHAPTER=$PWD/target/scratch/before.md bash
  docs/tasks/artifacts/reference_fixture/run_fixture_derivation.sh` →
  `fixture derivation: 16 derived rows / 2 closure checks / 6 pieces / 7 mismatch(es)`, `exit=1`, with the
  decisive line `B1: §6 describes a faced two-piece band (per-piece cut width 6.0 cm) but
  `waistband_cut_width` publishes 10.0 cm — §4 and §6 are different garments, which is defect D27` and six
  `P1` refusals naming every piece §8 accounted for (none). It lived that way for nine commits:
  `git log --oneline 45e0a46..HEAD | wc -l` → `9`.
- [x] **ROOT CAUSE (WHY + WHERE)** — two causes, and only the second was visible before this leaf.
  (1) *The chapter*: a construction decision was never made. §1 said "cut twice plus interfacing", §4's
  formula described a folded band, §6 listed a faced pair — three cells of three different tables, each
  internally correct, and no arithmetic in any one of them could contradict another. The census that
  existed for this chapter read tokens, not tables: `git ls-files docs/tasks/artifacts | grep -c fixture`
  → `0`, `rc=1` (no tracked instrument over the fixture at `HEAD`), and the arithmetic that found D33 and
  then D27 was typed into leaf prose — `grep -c 'python3 -c' docs/tasks/PLANNING.md` → `2` — which is
  re-runnable by nobody and is the leg-3 breach this repository already committed once as D20.
  (2) *The invariant*: the chapter had no rule a piece must satisfy to be in the piece list, so an inner
  band nobody sewed was a prose gap rather than a refusal.
- [x] **ADDRESSED (verified)** — the fixture is one garment and says so with numbers.
  `bash docs/tasks/artifacts/reference_fixture/run_fixture_derivation.sh` →
  `fixture derivation: 20 derived rows / 4 closure checks / 5 pieces / 0 mismatch(es)`, `exit=0`, against
  `7 mismatch(es)` over the same chapter at `HEAD` (above): §4 gained `waistband_fold_position` (5.0 cm),
  `waistband_finished_length` (77.0 cm), `wb_interfacing_width` (4.0 cm), `wb_interfacing_length` (77.0 cm)
  and the band's two closure checks — `waistband_length_closure` `74.0 = 74.0` and
  `waistband_width_closure` `8.0 = 8.0`, both printed as `CLOSES`; §6 lists five pieces; §8's attachment
  account names all five (`accounted: 5`); §12's count agrees (`published count: 5`); and `B1` reports
  `a single band folded lengthwise · cut width 10.0 cm · agrees with §6's 1 band piece(s)`. The instrument
  is a tracked producer with ground truth: `TMPDIR=$PWD/target/scratch bash
  docs/tasks/artifacts/reference_fixture/run_fixture_probes.sh` → `probes: 9 pass / 0 fail`, `exit=0`.
  The decision is sourced, not preferred — five references read on this machine on `2026-09-30` (reachability
  re-measured first: `curl -sS -m 12 -o /dev/null -w '%{http_code}' https://en.wikipedia.org/wiki/Waistband`
  → `200`), the decisive fact being anicka.design's drafting sequence, which gives the **straight** band one
  rectangle with a fold line and reserves "cut two pieces — one for the outer waistband, and one for the
  inner waistband" to the **curved** band; recorded with every URL, the date read, the `read-external` label,
  the one disagreement between sources (`wb_width`: 2–5 cm versus a 3 cm maximum for a straight band, kept
  `assumed` rather than silently adjusted) and the re-open condition in
  `docs/decisions/decision_reference-fixture-waistband-straight-folded.md`. The three neighbouring censuses
  are still green, which is the check that this did not contradict another chapter:
  `bash docs/tasks/artifacts/glossary/run_glossary_census.sh` → `276 terms / 8 parts / 145 tokens /
  0 failure(s)`, `exit=0`; `bash docs/tasks/artifacts/feature_matrix/run_feature_matrix_census.sh` →
  `105 rows / 29 diagnostics / 0 failure(s)`, `exit=0` (its `interfacing` row still says the fixture carries
  an interfacing piece, and still does); `bash docs/tasks/artifacts/standards/run_standards_census.sh` →
  `6 registered / 6 designations used / 0 failure(s)`, `exit=0`. `make book` → `INFO HTML book written to
  …/docs/book/book`, `exit=0`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `make probes: 12 suite(s) green`, `exit=0` (eleven before this leaf, the twelfth being the new one);
  `bash scripts/check_live_doc_size.sh` → `live-doc-size: OK — 17 surfaces, 15 routes, 74 files measured`,
  `exit=0`; `bash docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh` → `probes: 7 pass / 0 fail`,
  `exit=0`. No Rust changed (`git diff --cached --name-only | grep -c '\.rs$'` → `0`, `rc=1`), so
  `make check` is unaffected and the crate tests were last green at `.18`. The probe suite was shown to
  *discriminate* rather than merely to pass, by neutering one rule at a time in a scratch copy of the tool:
  with `B1` removed, `BAND-PAIR` goes red → `probes: 8 pass / 1 fail`; with `P2`'s comparison removed,
  `COUNT` and `BAND-PAIR` go red → `probes: 7 pass / 2 fail`. Containment: the chapter is `379` lines /
  `26 599` B / widest `161` B against a `book_collection` per-part health of `400` / `24 576` / `200` — one
  axis over health and inside every ceiling (`700` / `40 960` / `320`) — and the decision record is `145`
  lines / `11 303` B / widest `353` B against a `decisions_collection` health of `120` / `8 192` / `320` and
  a ceiling of `200` / `16 384` / `600`. The glossary's derived index grew one line with the new term and is
  now `406` lines against its `400`-line health, which is the growth `G0-CONTRACT.7`'s decision anticipated
  and whose remedy (`--emit-index` into two letter halves at ~500) is already recorded. Nothing was trimmed
  to hit a health target, because the bytes are the sourcing the ruling asked for; every overrun is recorded
  here rather than paid for in vaguer prose, which is the remedy `G0-CONTRACT.1` and `.7` both recorded.
- [x] **FIX** — chose the construction and made the chapter one garment: §1 ("cut once and folded
  lengthwise"), §4 (four new rows, the disputed marker removed), §4.1 (two closure checks → four), §5
  (step 8 rewritten, step 9 added for the interfacing, the mirror step renumbered — nothing referenced
  step 9, checked: `grep -rn 'step [0-9]' docs/book/src/`), §6 (five pieces, the provisional paragraph
  replaced), §7 (`sa_cb` and `sa_waist` name the band edges they also serve, the band's free edge
  described, an allowance-free row for the interfacing), §8 (`waist` span names `waistband`, the attachment
  account added, the ends recorded as an edge finish), §10 (two new exercised properties), §11 (the D27
  bullet is now the resolution with its sources and what stays `assumed`), §12 (count 5, three new
  obligations). Built the instrument and its probe suite; added the glossary entry that owns `fused`.
  Also fixed in this commit, because it refused this slice's own changelog entry and the probe was what
  was wrong (defect **D39**): `run_changelog_ledger_probes.sh` read work-unit ids with `[0-9]+[a-c]?`, so
  `STITCHCAD-G0-0013d` parsed as `STITCHCAD-G0-0013` and collided with the sealed entry of that name. The
  class is `[a-z]?` in all six places and a new GREEN arm `SUFFIX` pins it — measured sensitive by
  reverting the class in a scratch copy → `probes: 5 pass / 2 fail`.
- [x] **The acceptance clause "no piece without a span" is met as *no piece without an account*, and that
  is recorded rather than quietly reinterpreted.** A fused interfacing is sewn by nobody, so the literal
  clause is unsatisfiable for the construction the sources prescribe; the invariant that holds is a span
  *or* a declared non-sewn attachment from a closed list that today contains exactly `fused` (a sewn-in
  interlining is sewn, so it would need a span). Rule `P1` checks that, and its `NO-ACCOUNT` arm removes
  the interfacing's row and requires the refusal.
- [x] **The token census corrected the chapter a fourth time at authoring time, and the fix was again the
  convention rather than an exemption** — `run_glossary_census.sh` reported `4 failure(s)` on the first
  draft: `` `Fold` `` (a column name of this chapter's own table, i.e. prose), `` `fused` `` (a machine
  enum value a program reads — so it became a glossary term with one owner), and the two names of the
  rejected reading, `` `waistband_outer` `` and `` `waistband_inner` `` (identifiers that no longer exist,
  so the history sentence names them in prose and the layer-C record keeps the tokens for anyone tracing
  the defect). After the entry and a re-derived index: `276 terms / 8 parts / 145 tokens / 0 failure(s)`,
  `exit=0`.
- [x] **LOCKSTEP** — D27 closed, D35 and D38 logged, D36, D37 and D39 logged and fixed, D34's recurrence
  recorded in `PLANNING.md`; `docs/TASK_TREE.md`'s three stale frontier cells and its execution-order line
  corrected; `TOOLBOX.md` gains both instruments; `docs/decisions/INDEX.md` carries the new record and the
  derived Knowledge Map was regenerated (`make gate` refused until it was); `LIVE_STATUS.md`, `MEMORY.md`,
  `CHANGELOG.md` and `DEV_NOTES.md` updated in this commit. Lesson promotion: **promoted** — the new
  record carries an `answers:` line, which is what `LESSON-PROMOTION` asks for.

### `G0-CONTRACT.4b` — every envelope feature has a gate that proves it, and the proposal says so

- [x] **REPRODUCE / ISSUE** — defect D32: five rows of the envelope matrix named no gate. Measured over the
  chapter as committed, in a synthetic root built from `HEAD`:
  `git archive HEAD docs/book/src ROADMAP.md | tar -x -C target/scratch/before4b &&
  FEATURE_MATRIX_ROOT=$PWD/target/scratch/before4b bash
  docs/tasks/artifacts/feature_matrix/run_feature_matrix_census.sh` → the A1 advisory lists `classic collar`,
  `trousers`, `button and buttonhole`, `fly construction`, `pocket` and prints `D32 rows: 5`, with the census
  itself `exit=0` — an advisory, so the gate was green the whole time the envelope was unprovable. That is
  the shape A1 exists for, and it is why the count and not the exit code is the evidence.
- [x] **ROOT CAUSE (WHY + WHERE)** — roadmap §3.2 puts a collar and trousers inside the envelope and ontology
  §4.7 models buttons and pockets, while §11's exit criteria name none of them:
  `sed -n '702,708p' ROADMAP.md | grep -ci 'collar\|trousers\|button\|pocket'` → `0`, `rc=1` (G3's exit
  bullet). The whole-section range is *not* the measurement, and re-deriving it proved that the hard way:
  `sed -n '/### G3 /,/### G4 /p' ROADMAP.md | grep -ci 'collar\|trousers\|button\|pocket'` → `1`, `rc=0`,
  the one match being G3's complexity note "a shirt/trousers intermediate may be inserted without shame" —
  permission, not a criterion. D32's entry cited `0` for that command, so the citation did not reproduce
  (**D41**, logged and fixed here); two candidate causes were measured rather than guessed — the pattern
  without `trousers` → `0`, and the exit-bullet range → `0` — so the conclusion was right and the recorded
  range was wrong. The trap generalises and `G0-CONTRACT.15` inherits it: a section range silently includes
  that section's notes, so "no exit criterion mentions X" must be measured over the criteria.
- [x] **ADDRESSED (verified)** — every row names a gate and the census derives the state:
  `bash docs/tasks/artifacts/feature_matrix/run_feature_matrix_census.sh` →
  `feature-matrix census: 105 rows / 29 diagnostics / 0 failure(s)`, `exit=0`, with `A1 … D32 rows: 0`
  (was `5`) and the new `A3` advisory listing all four proposed cells — `classic collar → G3 (proposed §11
  amendment)`, `trousers → G3 (proposed §11 amendment)`, `button and buttonhole → G3 (proposed §11
  amendment); G5 notions`, `pocket → G3 (proposed §11 amendment)` — and printing `proposed cells: 4`.
  `bash docs/tasks/artifacts/feature_matrix/run_feature_matrix_probes.sh` → `probes: 12 pass / 0 fail`,
  `exit=0`, the new `PROPOSAL-VISIBLE` arm reporting `A3 reads the cells: 4 proposed on the real tree, 1 with
  the markers removed`. The amendment is quoted current-and-proposed side by side with its line numbers
  (`ROADMAP.md:701`–`711`) in `docs/decisions/decision_d32-proving-gates-proposed-roadmap-amendment.md`,
  marked **proposed**, indexed, and carrying both the approval and the rejection path. The roadmap itself is
  untouched, which is the reservation the ruling made: `git diff --name-only HEAD -- ROADMAP.md | grep -c .`
  → `0`, `rc=1`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `make probes: 12 suite(s) green`, `exit=0`, the ledger suite now at `probes: 8 pass / 0 fail` after its
  `DESCRIPTOR` rule was generalized from `stitchcad-changelog-part*.md` to every `docs/history/*.md` segment
  (its REAL arm went from `11 verdicts` to `13`, the bedrock and dev-notes segments) with the new
  `DEVNOTES-DIGEST` arm pinning the generalization; `make book` → `INFO HTML book written to …`, `exit=0`;
  the neighbouring censuses are green — `run_glossary_census.sh` → `276 terms / 8 parts / 145 tokens /
  0 failure(s)`, `run_standards_census.sh` → `6 registered / 6 designations used / 0 failure(s)`;
  `bash scripts/check_live_doc_size.sh` → `live-doc-size: OK — 17 surfaces, 15 routes, 74 files measured`,
  `exit=0`, with `dev_notes` back inside its health after the rollover (`150` lines / `13 183` B against
  `200` / `16 384`) and the matrix at `339` lines / `31 807` B / widest `221` B against a `book_collection`
  per-part health of `400` / `24 576` / `200` and a ceiling of `700` / `40 960` / `320` — bytes and maxline
  over health, inside every ceiling, recorded rather than trimmed. No Rust changed.
  **The containment ceiling did refuse this slice, once:** appending the `.4b` checklist put this tree file
  at `1182` lines / `104 182` B against the `tasks_collection` per-part byte ceiling of `98 304`, so
  `check_live_doc_size.sh` reported `tasks_collection: 104182 bytes exceed the byte ceiling 98304` and
  `make gate` blocked. The remedy is the one the registry's own note prescribes — "a tree that passes 1000
  lines splits its completed-leaf evidence into a sibling file under docs/tasks/ before adding more" — and
  not a bigger number: the nine completed checklists moved to `G0-CONTRACT-evidence.md`, leaving this file at
  `729` lines / `62 087` B (inside its `800` / `65 536` health) and the sibling at `494` lines / `45 505` B,
  and the checker returned to `live-doc-size: OK — 17 surfaces, 15 routes, 75 files measured`, `exit=0`.
- [x] **FIX** — the five cells and their reasons; §1's rule 2 gains the `(proposed)` shape; §9 rewritten from
  "rows whose proof nobody has been asked for" into the proposal table plus one bounded reason per feature
  (the long-cell shape the containment ceiling refused twice before, so the table stayed three short columns);
  §11, §12 and §14 updated so no sentence still calls the gap open; census advisory `A3` and its probe arm;
  the decision record; and the dev-notes rollover this slice's own append triggered — the four oldest lessons
  sealed into `docs/history/stitchcad-devnotes-part1.md` (`66` lines / `5 589` B / `sha256:d3b94e9a…`, the
  digest re-derived by the same `sed | shasum` the probe runs) with a pointer table in the live window.
- [x] **The rollover is proved lossless against the committed state, not against memory** —
  `python3` compared the sealed bytes with `git show HEAD:DEV_NOTES.md` from the same heading onward:
  `sealed entries byte-identical to HEAD's tail: True`, live window `5` lessons. And the segment is watched
  from the commit that created it: mutating one byte of it in a scratch copy makes `DESCRIPTOR` name it
  (`DEVNOTES-DIGEST … content hashes to b97da359f4c21d45…, its descriptor declar…`), which is the leg-3
  argument D20 already paid for once. What is *not* watched is the segment's Coverage and pointer claims —
  the ledger probe reads those from `CHANGELOG.md` only — so the gap is logged as **D40** and owned by a new
  leaf `SPINE.19`, not left implicit in a green run.
- [x] **The token census fired a fifth time at authoring time** — `run_glossary_census.sh` reported
  `2 failure(s)` on the first draft: `` `lining` `` (the name of a row of this chapter's own table, i.e.
  prose) and `` `proposed` `` (a status marker written without the parentheses it actually carries, so it
  looked like exactly one ASCII identifier). Both were de-tokenized rather than exempted, and the marker is
  written `(proposed)` everywhere, which is also what the A3 advisory greps for → `276 terms / 0 failure(s)`.
- [x] **LOCKSTEP** — D32 resolved, D40, D41 and D42 logged in `PLANNING.md` (D41 fixed here); `SPINE.19`
  created in `docs/tasks/SPINE.md` to own D40 with its acceptance and its two RED arms named; `TOOLBOX.md`'s
  matrix and ledger rows updated; `docs/TASK_TREE.md`'s frontier cell moved to `.14`; the completed-leaf
  checklists split into `docs/tasks/G0-CONTRACT-evidence.md`; `docs/decisions/INDEX.md`
  carries the new record and the Knowledge Map was regenerated; `LIVE_STATUS.md`, `MEMORY.md`,
  `CHANGELOG.md` (including the superseding record for the sealed `part4` citation D41 found) and
  `DEV_NOTES.md` (a new lesson, the correction, and the rollover) updated in this commit. Lesson promotion:
  **promoted** — the new record carries an `answers:` line.

### `G0-CONTRACT.14` — the governance model exists before the community does

- [x] **REPRODUCE / ISSUE** — the G0 exit clause requires "governance model drafted (project owner named;
  sewist-vs-programmer review paths defined)" and roadmap §12 requires four things nothing here had written:
  a domain-review path that is not code review, golden-file approval ownership, the sewist-vs-programmer
  conflict rule, and named funding/procurement owners. Measured at `HEAD`:
  `git ls-tree --name-only HEAD docs/book/src/` → `SUMMARY.md`, `introduction.md`, `spec`, `rc=0` — no
  governance chapter, while `grep -ci governance ROADMAP.md` → `6` clauses expect one, and `G0-CONTRACT.15`
  cannot review an exit clause that has no deliverable.
- [x] **ROOT CAUSE (WHY + WHERE)** — §12 states the requirement as prose with no owner per rule: "a named
  domain-expert approval path (two-step: expert + maintainer)" says who approves but not which changes take
  that path, and §14's risk row "community fork over governance" is mitigated only by "governance doc at G0,
  while the room is empty" — scheduled, and never written. Three of the four rules also depend on a *person*
  this repository does not have, which is why the leaf stayed blocked until the ruling of `2026-09-30`
  separated the drafting from the naming; the blocking part was that coupling, not the missing names.
- [x] **ADDRESSED (verified)** — `docs/book/src/governance.md` (`wc -lc` → `198` lines / `15 593` B, widest
  line `199` B, inside the `book_collection` per-part health of `400` / `24 576` / `200`) carries ten sections:
  the classification table putting every change class on exactly one of the two review paths; the conjunctive
  two-step rule for anything that alters exported bytes; seven roles each with the authority it needs and
  whether an agent may hold it; the four-step conflict path (classify → make a contested default a profile
  parameter → escalate by review round, not by date → a fork is a legitimate outcome); golden approval with
  two signatures and the no-golden-over-an-`assumed`-constant precondition; the public/never-public boundary
  and its three consequences for the review paths; agents under governance; the procurement table with a
  fallback and its cost in evidence quality per item; **all four empty seats in one table** (§8), which is the
  acceptance clause "flagged to the director in one place, not discovered later"; the four questions
  deliberately left undecided; and a verification-status section separating the roadmap citations from the
  project decisions. `make book` → `INFO HTML book written to …`, `exit=0`, and
  `ls docs/book/book/governance.html` → present, wired in as the book's first non-specification part. The six
  project decisions are recorded separately in
  `docs/decisions/decision_governance-two-review-paths-and-the-unnamed-roles.md` (`84` lines / `6 600` B,
  indexed, carrying an `answers:` line), so a reader can tell the citation from the invention.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`;
  `bash scripts/check_live_doc_size.sh` → `live-doc-size: OK — 17 surfaces, 15 routes, 77 files measured`,
  `exit=0`; the three censuses over the book are green and unchanged by the new chapter —
  `run_glossary_census.sh` → `276 terms / 8 parts / 145 tokens / 0 failure(s)` (so the chapter introduces no
  undeclared machine token), `run_feature_matrix_census.sh` → `105 rows / 29 diagnostics / 0 failure(s)`,
  `run_standards_census.sh` → `6 registered / 6 designations used / 0 failure(s)`; the fixture still re-derives
  at `20 derived rows / 4 closure checks / 5 pieces / 0 mismatch(es)`; `make probes` →
  `make probes: 12 suite(s) green`, `exit=0`. No Rust and no instrument changed, so the suite count is
  unchanged; `git diff --cached --name-only | grep -cE '\.(rs|sh)$'` → `0`, `rc=1` — a documentation-only
  slice, so no instrument and no crate behaviour could regress.
- [x] **FIX** — wrote the chapter and the record; added the book's `# Governance` part to `SUMMARY.md`;
  moved `.4b`'s completed checklist into `G0-CONTRACT-evidence.md` in this same commit, which is the
  convention `.4b` recorded and the first slice to obey it — the leaf being landed keeps its checklist in the
  tree file, because `scripts/check_task_acceptance.sh` judges every staged `docs/tasks/*.md` and refuses one
  with no ticked boxes. The tree file is `744` lines / `64 445` B (inside its `800` / `65 536` health) and the
  evidence sibling holds `10` completed checklists at `576` lines / `53 726` B.
- [x] **The blocked part is stated as a cost with a schedule attached, not as an apology.** The unnamed domain
  expert is not a paperwork gap: that seat gates the reference fixture's `assumed` constants, which gate the
  **first G2 golden** (§4 of the chapter), so the dependency is written where a plan will hit it. Each
  procurement item likewise carries its fallback *and what the fallback costs in evidence quality*, because
  roadmap §14 already made the partner-run manual test normative and an unstated cost is how a slip becomes a
  silent downgrade of the release claim.
- [x] **The rollover this slice's changelog append triggered is performed and verified in the same commit.**
  The live window had crossed its health target (`410` lines / `36 632` B against `400` / `32 768`), so the two
  oldest entries are sealed into `docs/history/stitchcad-changelog-part5.md` (`82` lines / `7 505` B /
  `sha256:18548ff78f8345d1…`) with a pointer row in the live file, which is back inside health at `329` lines /
  `29 295` B. Losslessness is proved against the committed state rather than against memory: the sealed bytes
  are identical to `git show HEAD:CHANGELOG.md` from the same heading onward (`True`), and the standing
  verifier agrees — `run_changelog_ledger_probes.sh` → `probes: 8 pass / 0 fail`, its `DESCRIPTOR` rule
  reproducing all seven sealed segments' digests including the new one.
- [x] **LOCKSTEP** — the leaf's status, the frontier (which now names `SPINE.4.4` as the repository's next
  slice, per the ruling's order), the tree's decisions, blockers, verification and commit logs and its
  changelog; `docs/TASK_TREE.md`'s frontier cell; `docs/decisions/INDEX.md` and the regenerated Knowledge Map;
  `MEMORY.md`, `LIVE_STATUS.md`, `CHANGELOG.md` and `DEV_NOTES.md` (a new lesson) updated in this commit.
  Lesson promotion: **promoted** — the governance record gains an `answers:` line.

### `G0-CONTRACT.4c` — the proposal is ruled on and applied, and the criterion arrives with owners

- [x] **REPRODUCE / ISSUE** — two things were true and neither could stand. (1) The envelope was still
  unproved in the roadmap: `sed -n '/^### G3 /,/^- Domain-complexity/p' ROADMAP.md | grep -ci
  'collar\|trousers\|button\|pocket'` → `1` before this slice, and that one hit was the *note* that permitted
  an intermediate, not a criterion; the matrix said so mechanically —
  `bash docs/tasks/artifacts/feature_matrix/run_feature_matrix_census.sh` → `A3 advisory … proposed cells: 4`.
  (2) Applying the amendment was impossible without cheating: `wc -lc ROADMAP.md` → `919 50821`, exactly the
  `roadmap` row's transition-debt baseline (`lines=919;bytes=50821`), and `check_live_doc_size.sh` refuses a
  widened baseline — so the only way to grow the roadmap was to edit the number, which is the silent widening
  the rule exists to prevent.
- [x] **ROOT CAUSE (WHY + WHERE)** — the first cause was authority, not analysis: the director's ruling of
  `2026-09-30` reserved amending `ROADMAP.md`, and his second instruction delegated the finding outright, so
  the proposal became a decision this repository could execute. The second cause was a missing mechanism: a
  debt baseline is a *stored copy* of a file's size at a moment, and nothing tied it to the file's revision
  identity, so a legitimate revision and a silent widening looked identical to the checker. Measured, because
  the block is the evidence: `bash scripts/check_live_doc_size.sh` over the amended roadmap →
  `LIVE-DOC-SIZE: roadmap: transition debt WIDENED on lines (947 > baseline 919) — a baseline never grows`,
  `exit=1`, against `grep -o 'lines=[0-9]*;bytes=[0-9]*' .doctrine/live_document_size/surfaces.tsv` →
  `lines=919;bytes=50821`, `rc=0`. That is the containment adoption note's deferred trigger 3 — "a stored copy
  of a mechanically owned value needs an executed freshness oracle" — fired by the first roadmap amendment,
  and `SPINE.4.5` discharges it.
- [x] **ADDRESSED (verified)** — roadmap **v0.3** carries the criterion: `grep -n 'envelope coverage'
  ROADMAP.md` → line `712`, `- **Exit (envelope coverage):** every garment §3.2 names drafts, grades and
  exports at this gate or an earlier one …`, and the same census over the section now returns `3` matching
  lines instead of `1`. The revision is marked where the roadmap's own policy requires —
  `grep -n 'v0.3' ROADMAP.md` → the title (line 1), the status block (line 11, naming the source), the
  Appendix A disposition entry (line 926) and the end line (line 945) — and the file measures
  `wc -lc ROADMAP.md` → `947 52818` (+28 lines / +1 997 B), re-based in the registry as
  `lines=947;bytes=52818;at=v0.3`. The matrix is committed rather than provisional:
  `grep -c 'proposed §11 amendment' docs/book/src/spec/feature-matrix.md` → `0`, and
  `run_feature_matrix_census.sh` → `105 rows / 29 diagnostics / 0 failure(s)`, `exit=0`, with `D32 rows: 0`
  and `proposed cells: 0`; `run_feature_matrix_probes.sh` → `probes: 12 pass / 0 fail` after its
  `PROPOSAL-VISIBLE` arm was rebuilt to pin A3 in BOTH directions (0 on the real tree, 1 once a marker is
  injected) instead of asserting a count that no longer exists. The criterion arrives with owners:
  `G3-GRADING.5` is required (trousers + pocket + derived buttonhole), `G3-GRADING.15` is created (the classic
  collar), and `.14`'s exit review fails if a §3.2 garment has no leaf's evidence — so the capture claim still
  derives: `run_tree_coverage_census.sh` → `census: 10 lanes / 13 trees / 2 sibling(s) / 0 unowned /
  0 orphan(s) / 0 dead link(s)`, `exit=0`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `13 suite(s) green` (twelve before this slice; the thirteenth is the tree-coverage suite `PLANNING.6` adds),
  with `run_live_doc_size_probes.sh` → `probes: 5 pass / 0 fail` including the new `REAL-3` arm that stales the
  real registry to `at=v0.2` and requires the refusal, and `check_live_doc_size.sh --self-test` →
  `15 arms, 0 failed` (eleven before `SPINE.4.5`); `bash scripts/check_live_doc_size.sh` →
  `live-doc-size: OK — 17 surfaces, 15 routes, 84 files measured`, `exit=0`, the roadmap row inside its
  re-based debt and its `240` B maxline health (widest `220` B); the book's other censuses are green and
  unchanged — glossary `276 terms / 8 parts / 145 tokens / 0 failure(s)`, standards
  `6 registered / 6 designations used / 0 failure(s)`, fixture `20 derived rows / 4 closure checks / 5 pieces /
  0 mismatch(es)`; `make book` → `exit=0`. No Rust changed.
- [x] **FIX** — amended §11 G3 through the roadmap's revision policy (criterion added, the "intermediate
  complexity" note rewritten so an intermediate can never substitute for a criterion); logged it in Appendix A
  with its source and the defect it closes; re-based the containment baseline with `at=v0.3` and the authority
  cited in the row's notes; dropped `(proposed)` from the four cells and rewrote §1 rule 2, §9, §12 and §14 so
  no sentence still calls the gap open; marked `decision_d32-proving-gates-proposed-roadmap-amendment.md` as
  **applied** (keeping the rejection path, because a future revision may withdraw the criterion); marked the
  containment adoption record's trigger 3 as fired and discharged; gave `G3-GRADING` the two leaves and the
  acceptance-table row; fixed the coverage census's tree enumeration and put it under `make probes` (D44,
  `PLANNING.6`); and added the revision-aware baseline (`SPINE.4.5`).
- [x] **LOCKSTEP** — `MEMORY.md` (v0.3, the amendment no longer pending), `LIVE_STATUS.md`, `CHANGELOG.md`,
  `DEV_NOTES.md`, `docs/TASK_TREE.md` (the coverage claim now cites both the census and its probe suite),
  `TOOLBOX.md` (two new rows), `docs/decisions/INDEX.md` and the regenerated Knowledge Map; D44 logged and
  fixed in `PLANNING.md`; `.15`'s scope narrows because the proposal it was to carry is applied. Lesson
  promotion: **promoted** — `decision_revision-aware-containment-baseline.md` carries an `answers:` line.

### `G0-CONTRACT.14b` — an empty seat is held acting with limits, or it is vacant

- [x] **REPRODUCE / ISSUE** — governance §8 listed three seats as "blocked on the director" and every chapter
  that needed a domain reviewer pointed at a leaf that cannot name one:
  `grep -rn 'G0-CONTRACT.14` names' docs/book/src/spec/` → the standards registry's owner cells and the
  fixture's `assumed`-constant bullets, i.e. three chapters deferring to an event that had no mechanism. The
  director then delegated the finding ("make the necessary calls and every needed action"), which makes the
  vacancy this repository's problem to state precisely rather than to wait on.
- [x] **ROOT CAUSE (WHY + WHERE)** — the chapter was drafted under a ruling that reserved naming humans, so it
  recorded the gap honestly and stopped there: "blocked" is a state, not a plan. What was missing is the
  distinction between two kinds of authority — **office**, which the director already holds and can therefore
  hold *acting*, and **competence**, which nobody in this repository has and no acting arrangement can supply.
  Treating the three seats alike would either stall the project owner's decisions unnecessarily or, worse,
  imply that an engineer can certify a sewing judgement.
- [x] **ADDRESSED (verified)** — `docs/book/src/governance.md` §8 is rewritten as three subsections: the seat
  table now names who holds each seat (`director, acting (§8.1)` twice, `**nobody — vacant, not acting**` for
  the domain expert); §8.1 states the acting authority and four hard prohibitions on it (no confirming the
  fixture's `assumed` constants, no signing a golden's semantic half, no ruling a safety-relevant term, no
  approving a byte-changing Factory Profile — which under §1's conjunctive rule leaves such a change
  unapproved with the missing half reported); §8.2 states the ask per seat so naming one is a single act.
  `wc -lc docs/book/src/governance.md` → `233` lines / `18 465` B, widest line `185` B, inside the
  `book_collection` per-part health of `400` / `24 576` / `200`; `make book` → `INFO HTML book written to …`,
  `exit=0`. The rule is recorded in `docs/decisions/decision_unnamed-seats-acting-authority.md` (indexed,
  `answers:` line) and the deferring chapters now cite the seat's real state:
  `grep -rc 'vacant' docs/book/src/spec/standards.md docs/book/src/spec/reference-skirt.md
  docs/book/src/spec/glossary.md` → `3`, `2`, `1`, and the stale deferral is gone —
  `grep -rn 'G0-CONTRACT.14` names' docs/book/src/spec/ | grep -c .` → `0`, `rc=1`. The standards census still
  resolves every owner cell: `run_standards_census.sh` → `6 registered / 6 designations used / 0 failure(s)`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`;
  `bash scripts/check_live_doc_size.sh` → `live-doc-size: OK — 17 surfaces, 15 routes, 84 files measured`,
  `exit=0`; the book's censuses are green — glossary `276 terms / 8 parts / 145 tokens / 0 failure(s)` (the new
  prose introduces no undeclared machine token), standards `6 registered / 0 failure(s)`, matrix
  `105 rows / 0 failure(s)`, fixture `20 derived rows / 4 closure checks / 5 pieces / 0 mismatch(es)`;
  `make probes` → `13 suite(s) green`. No Rust and no instrument changed in this leaf.
- [x] **FIX** — rewrote §8 and its status block, §2's "currently" column for the three seats, the fixture
  chapter's two reviewer references, the glossary's two, the standards registry's two owner cells and its
  verification-plan step 1 — each cross-chapter link written as `../governance.md`, because a chapter under
  `spec/` reaches the book's top level one directory up and a dead link here would have been the D28 class
  (caught by building the book, not by reading the path); added the decision record and its index row.
- [x] **The first G2 golden stays gated, and that is the point of the leaf.** The tempting move was to record
  the director as acting domain expert so the schedule clears; the honest one is a vacant seat with a named
  consequence, because a golden frozen over an unreviewed `assumed` constant freezes a guess with the
  confidence of a fact, and the fixture chapter says so where a plan will hit it.
- [x] **LOCKSTEP** — the leaf, this checklist and the tree's blockers; `MEMORY.md`'s blocker bullet;
  `LIVE_STATUS.md`; `CHANGELOG.md`; `docs/decisions/INDEX.md` and the regenerated Knowledge Map. Lesson
  promotion: **promoted** — the record carries an `answers:` line.

### `G0-CONTRACT.14c` — a delegated decision is bounded, not merely disclosed

- [x] **REPRODUCE / ISSUE** — finding 1: the engineer proposed the `ROADMAP.md` §11 G3 amendment in `.4b` and
  applied it in `.4c`, and the only thing recording that was a sentence in a session report.
  `git show HEAD:docs/book/src/governance.md | grep -ci 'self-appl\|same party'` → `0`, `rc=1`, against `2`
  after this slice: the governance model covered human seats and agents, and said nothing about the case this
  project actually runs in.
- [x] **ROOT CAUSE (WHY + WHERE)** — governance §8 was written to list *vacancies*, and §6 to bound *agents*;
  neither addresses a party that legitimately holds the pen for both halves of a decision. A disclosure is not a
  control: it decays with the conversation it was made in, while a rule is read by whoever acts next. The
  founding instance is measured in the tree: `git log --oneline -3 -- ROADMAP.md` → `513374c` (the v0.3
  amendment), `5257257` (bootstrap), `f1dcbe4` (initial), `rc=0` — the roadmap's first revision in its life was
  prepared as a proposal in `4b0bb95` and applied in `513374c`, adjacent commits by the same author.
- [x] **ADDRESSED (verified)** — `docs/book/src/governance.md` §6.1 carries five rules: record the author and
  the applier and say so when they are the same; the author of a decision may not approve the evidence that
  decision requires (approval belongs to a reviewer meeting §2's independence criterion, and where none exists
  the claim stays **unapproved**); consequences become instruments others can run; the record states what would
  reverse it and who may; a delegation to decide is not one to upgrade evidence.
  `wc -lc docs/book/src/governance.md` → `265` lines / `21 303` B, inside the `book_collection` per-part health
  of `400` / `24 576`; `make book` → `INFO HTML book written to …`, `exit=0`. The rule bites in three places,
  each verifiable: the roadmap's Appendix A v0.3 entry now states that author and applier were the same party
  (`grep -c 'same one' ROADMAP.md` → `1`, `rc=0`), `G3-GRADING.14`'s acceptance withholds the review from the
  criterion's author, and the reasoning is in
  `docs/decisions/decision_self-application-under-delegation.md` (indexed, `answers:` line).
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; the roadmap grew by four lines
  and the revision-aware baseline refused it, exactly as `SPINE.4.5` designed:
  `LIVE-DOC-SIZE: roadmap: transition debt WIDENED on lines (951 > baseline 947)`, `exit=1` — handled by the
  rule rather than around it, re-basing to v0.3's final state (`lines=951;bytes=53153;at=v0.3`) with the
  authority cited in the row's notes, and recording in the decision record the limit this exposed (`at=` binds a
  baseline to a revision marker, not to a commit). `bash scripts/check_live_doc_size.sh` →
  `live-doc-size: OK — 17 surfaces, 15 routes, 89 files measured`, `exit=0`; `make probes` →
  `15 suite(s) green`; `make book` → `exit=0`; the book's censuses green.
- [x] **FIX** — added §6.1 to the governance chapter, the disclosure to the roadmap's disposition entry, the
  independence requirement to `G3-GRADING.14`'s acceptance, the decision record and its index row; re-based the
  containment baseline under the rule and recorded that rule's limit.
- [x] **LOCKSTEP** — the leaf, this checklist, the frontier and the tree's logs; `MEMORY.md`, `LIVE_STATUS.md`,
  `CHANGELOG.md`, `docs/TASK_TREE.md`, `TOOLBOX.md` and the regenerated Knowledge Map in this commit. Lesson
  promotion: **promoted** — the new record carries an `answers:` line.

### `G0-CONTRACT.19` — what the project does not know is one command away

- [x] **REPRODUCE / ISSUE** — the domain seat is vacant and the book's unverified claims were enumerable only by
  reading it: `git grep -c '\`assumed\`' HEAD -- docs/book/src` → `18` occurrences across `6` files, with
  nothing tying them to the seat that owes them and nothing noticing if one disappeared. The roadmap's core
  property is that uncertainty is data; a datum nobody can list is prose.
- [x] **ROOT CAUSE (WHY + WHERE)** — every chapter states its own verification status, and no instrument reads
  them together, so "what waits on a human" lived in the reader's memory. `ls docs/tasks/artifacts/` at `HEAD`
  → twelve directories, none of them about uncertainty; the population spans nine files and six marker
  spellings, which is exactly the shape that drifts.
- [x] **ADDRESSED (verified)** — `bash docs/tasks/artifacts/uncertainty/run_uncertainty_census.sh` →
  `uncertainty census: 88 markers / 10 files / 0 unowned / 0 failure(s)`, `exit=0`, enumerating every marker in
  the book's own vocabulary (`assumed`, `unknown`, `unverified-with-owner`, `read-in-repo`,
  `cited-from-roadmap`, `read-external`, `known`, `derived`, `(proposed)`, `vacant`) per file and per resolving
  authority, and refusing — rule `U1` — a blocking marker whose verification-status section names no resolver.
  Its discrimination is proved, not assumed:
  `TMPDIR=$PWD/target/scratch bash docs/tasks/artifacts/uncertainty/run_uncertainty_probes.sh` →
  `probes: 6 pass / 0 fail`, with an owned-claim control arm, an unowned synthetic chapter refused by name, a
  chapter added *after* the census was written refused (so the rule is about the population), a definition left
  alone, and an absent book refusing with `exit=2`. Governance §8.1 and the fixture's §11 now point at it, so
  the day a name arrives the work it unblocks is one command away.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `make probes: 15 suite(s) green`, `exit=0`; `make book` → `exit=0`;
  `bash docs/tasks/artifacts/glossary/run_glossary_census.sh` → `277 terms / 8 parts / 146 tokens /
  0 failure(s)` after the new `vacant seat` entry and a re-derived index, and its own suite still
  `probes: 10 pass / 0 fail`; the other censuses unchanged (matrix `105 rows / 0 failure(s)`, standards
  `6 registered / 0 failure(s)`, fixture `20 rows / 4 checks / 5 pieces / 0 mismatch(es)`, coverage
  `13 trees / 3 sibling(s) / 0 orphan(s)`); `bash scripts/check_live_doc_size.sh` → `OK — 17 surfaces,
  15 routes, 89 files measured`, `exit=0`.
- [x] **FIX** — built the census and its probe suite; added the `vacant seat` glossary entry (the token census
  demanded an owner for `vacant`, its sixth catch at authoring time) and re-derived the A–Z index; pointed
  governance §8.1 and the fixture §11 at the census; added both `TOOLBOX.md` rows. **Building it found two
  defects in existing instruments, both fixed here:** the glossary census's `resolve()` deleted one `/x/../` per
  `gsub` pass and so reported `../../governance.md` — a file that exists — as a dead reference (it now
  normalises segment by segment), and this census's first authority list counted a bare gate id as an owner, so
  "frozen as a golden at G2" satisfied it and the `UNOWNED` arm passed for the wrong reason until the list was
  narrowed to parties that can *resolve* a claim.
- [x] **LOCKSTEP** — the leaf, this checklist, the frontier and the tree's logs; `TOOLBOX.md`,
  `LIVE_STATUS.md` (fifteen probe suites, re-derived), `MEMORY.md`, `CHANGELOG.md` and the regenerated
  Knowledge Map in this commit. Lesson promotion: declined (the two instrument defects are recorded in this
  checklist and in the tools' own headers, which is where a probe author will look; no new dated lesson was
  added to `DEV_NOTES.md` this slice).

### `G0-CONTRACT.9` — the formula language exists, and its numbers are computed

- [x] **REPRODUCE / ISSUE** — ADR-0003 had two halves and the repository held neither.
  `git ls-files docs/book/src/spec/ | grep -c formula` → `0`, `rc=1`; `git ls-files docs/decisions/ |
  grep -c adr-0003` → `0`, `rc=1`. Two chapters already pointed at the missing one —
  `git show HEAD:docs/book/src/spec/ontology.md | grep -c 'formula-language'` → `1` ("specified in the
  formula-language chapter") — and the vocabulary had parked the system decision in this leaf:
  `git show HEAD:docs/book/src/spec/glossary/recipe-and-pieces.md | grep -c 'G0-CONTRACT\.9'` → `2`
  (`block (pattern)`, `reference drafting`). Roadmap §5 requires the language "specified HERE, not
  later" and §11's G0 exit clause names it, so the gap was a leaf not yet taken.
- [x] **ROOT CAUSE (WHY + WHERE)** — the requirement is in two places and neither had an artifact behind
  it. `grep -n 'specified HERE' ROADMAP.md` → `349:language is specified HERE, not later: operators,
  units inside expressions,`, `rc=0` — that is §5's ADR-0003, whose second half also requires "a named
  drafting system ships as reference blocks (v1: free drafting + one documented system, decided at G0)"
  (`sed -n '345,353p' ROADMAP.md`). `grep -n 'slides into mechanical' ROADMAP.md` → line `831`, whose
  mitigation cell reads "ADR-0003; formula language specified at G0", `rc=0` — the risk register names the
  failure this gap produces. Ownership was never in doubt:
  leaf `.9` held it and the frontier named it next, so the cause is a leaf not taken, and the constraint
  on *how* to take it is that row: a recipe language specified loosely becomes a solver, and a solver is
  what roadmap §6.2 and ADR-0001 put elsewhere. The chapter's first exclusion is therefore implicit
  solving, and its evaluation model is single-pass with no fixpoint.
- [x] **ADDRESSED (verified)** — `bash docs/tasks/artifacts/formula_language/run_formula_language_census.sh`
  → `formula-language census: 17 bindings / 4 assertions / 13 refusals / 0 mismatch(es)`, `exit=0`, with
  `names shared with the fixture: 12 · disagreements: 0`, all four fixture oracles holding as `assert`
  statements (`280000 = 280000`, `185000 = 185000`, `740000 = 740000`, `80000 = 80000` in internal µm),
  and each of the 13 refusals raising the token its row names. Discrimination is proved, not assumed:
  `TMPDIR=$PWD/target/scratch bash docs/tasks/artifacts/formula_language/run_formula_language_probes.sh`
  → `probes: 15 pass / 0 fail`, including a CONTROL arm that keeps an unrelated prose edit green. Sizes
  are inside the collection's per-part health (`400` lines / `24 576` B / `275` B): `wc -lc` →
  `308 20593`, `247 12690`, `92 7116`, widest line `189` B. `make book` → `INFO HTML book written to …`,
  `exit=0`, with `formula-language.html` and both parts rendered.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `16 suite(s) green`; the neighbouring censuses unchanged: fixture `20 derived rows / 4 closure checks /
  5 pieces / 0 mismatch(es)`, matrix `105 rows / 29 diagnostics / 0 failure(s)`, standards `6 registered /
  6 designations used / 0 failure(s)`, coverage `10 lanes / 13 trees / 3 sibling(s) / 0 unowned`,
  uncertainty `107 markers / 13 files / 0 unowned / 0 failure(s)` (was `88 / 10` — the three new files'
  markers are all owned); `bash scripts/check_live_doc_size.sh` → `OK — 17 surfaces, 15 routes, 90 files
  measured`, `exit=0`, at `31 warning(s)` against the `32` before the slice, because four `LIVE_STATUS.md`
  cells were tightened under D36's third instance and its widest line went `313` B → `213` B inside a
  `220` B health target. Two instrument bugs were found and
  fixed while building the census, both recorded in its header: a code-span scanner that paired backticks
  across newlines read the whole file as one span (a fence's ``` is an odd count) and reported `125`
  undeclared operators that were EBNF nonterminals, and a link resolver that dropped the leading `/` of an
  absolute base and so reported every link in the book dead.
- [x] **FIX** — wrote the three chapter parts; the ADR-0003 record with fourteen language decisions, five
  read candidates and three re-open conditions; the census and its 15-arm probe suite; `G3-GRADING.16` to
  own the blocks; twelve glossary terms plus two updated entries with the A–Z index re-derived; SUMMARY,
  the spec index, the ontology's cross-reference and the fixture's §3 note. **D48 logged and fixed** (a
  duplicate `## Acceptance Checklist` heading in `G3-GRADING.md` — D15's class by heading instead of by box
  — and a children range that said `.14` while `.15` was in the file), and **D36's third instance**
  recorded and removed (`LIVE_STATUS.md` listed D47 open after `SPINE.20` closed it; its probe count is now
  the command's, not a hand-kept number).
- [x] **LOCKSTEP** — the leaf, this checklist, the frontier, the tree's decisions and open questions, its
  three logs; `MEMORY.md`, `LIVE_STATUS.md`, `CHANGELOG.md`, `DEV_NOTES.md`, `docs/TASK_TREE.md`,
  `TOOLBOX.md` (two instrument rows, and the `make probes` row no longer carries a hand-kept suite count),
  `docs/decisions/INDEX.md`, `knowledge-map/subsystems.md` and the regenerated Knowledge Map in this
  commit. Both ledgers rolled over in the commit whose append crossed them (`devnotes-part3` 50 lines /
  4723 B, `changelog-part8` 106 lines / 9766 B), with `run_changelog_ledger_probes.sh` → `9 pass / 0 fail`
  reproducing both digests. Lesson promotion: **promoted** — the new record carries an `answers:` line.
