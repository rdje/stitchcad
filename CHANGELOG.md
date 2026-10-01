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
| [`part5.md`](docs/history/stitchcad-changelog-part5.md) | slices 30–31, `STITCHCAD-G0-0005` … `STITCHCAD-G0-0013c` | 82 lines, 7505 bytes, `sha256:18548ff7…` |
| [`part6.md`](docs/history/stitchcad-changelog-part6.md) | slices 32–33, `STITCHCAD-G0-0007` … `STITCHCAD-G0-0006` | 93 lines, 8284 bytes, `sha256:3148dd0f…` |
| [`part7.md`](docs/history/stitchcad-changelog-part7.md) | slices 34–35, `STITCHCAD-G0-0013d` … `STITCHCAD-G0-0008` | 105 lines, 9793 bytes, `sha256:ff62d418…` |
| [`part8.md`](docs/history/stitchcad-changelog-part8.md) | slices 41–42, `STITCHCAD-G0-0014` … `STITCHCAD-G0-0004b` | 106 lines, 9766 bytes, `sha256:2c7ee35a…` |
| [`part9.md`](docs/history/stitchcad-changelog-part9.md) | the two oldest live entries, `STITCHCAD-G0-0004c` and `STITCHCAD-SPINE-0004d` — no slice range, because the earlier ranges have no producer (D51) | 90 lines, 8386 bytes, `sha256:8e4081d4…` |
| [`part10.md`](docs/history/stitchcad-changelog-part10.md) | two spine slices on the table convention, `STITCHCAD-SPINE-0020` and `STITCHCAD-SPINE-0015` | 73 lines, 6652 bytes, `sha256:062ccfa3…` |
| [`part11.md`](docs/history/stitchcad-changelog-part11.md) | the delegation-and-uncertainty slice, `STITCHCAD-G0-0014c` | 46 lines, 4507 bytes, `sha256:de34382e…` |
| [`part12.md`](docs/history/stitchcad-changelog-part12.md) | the dialects and formula-language slices, `STITCHCAD-G0-0010` and `STITCHCAD-G0-0009` | 84 lines, 7767 bytes, `sha256:9c61ba7c…` |
| [`part13.md`](docs/history/stitchcad-changelog-part13.md) | the release-contract and canvas-spike-rule slices, `STITCHCAD-G0-0012` and `STITCHCAD-G0-0011` | 75 lines, 7188 bytes, `sha256:1e52b5c9…` |

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

## STITCHCAD-G1-0005 - the persistent-identity contract: a reference is never rewritten, the journal folds (leaf `G1-SLICE.3b`)

G1's second new product code. `sc_core::ontology` now carries the persistent-identity contract the whole
design's reference integrity rests on, implemented against the design decision this slice recorded,
dependency-free and wasm-safe:

- `topology` — the `IdentityLedger`: an append-only journal of typed `TopologyEdit`s (declare, split, merge,
  reverse, delete, offset-fragment). A stored reference is **never rewritten by an edit** — the `(EdgeRef,
  Param)` a consumer holds stays byte-identical, and resolution is a pure fold of the journal, so a replay
  reproduces every fragment identity. Split resolves a reference into the fragment holding its parameter and
  offers BOTH sides at the split point for the consumer to state (`SplitSide`); merge recomputes by
  caller-declared arc length (exact `Rational`, never a rounding); reverse maps `t` to `1 − t` and tells
  directed consumers through `Direction`; delete and offset-fragmentation orphan references into visible
  `RepairTask`s naming the reference, the orphaning edit and the candidate resolutions. No silent
  reassignment: repair state is *derived* (`open_repairs`, `release_readiness`), never stored, so it cannot
  drift from the journal. A design with unresolved references stays inspectable — every query answers
  identically — but is `Blocked` from release, which is §1.1's rule.

Validation: `cargo test -p sc-core` → 62 unit + 8 contract-property + 9 identity-property, all green (the
properties prove split's trichotomy and both-sides split point, merge's arc-length recomputation against a
cross-multiplied oracle, reverse an involution, split↔merge round-trips to the identical reduced rational
with zero drift, a delete orphaning exactly the references resolving onto the victim and nothing else,
offset against a hundredths-grid oracle, the live-edge set equal to the journal replayed, and byte-identical
replay of one script); `make check` clean at clippy `-D warnings`; `make wasm` cross-builds `sc-core`;
`make gate` → `=== all doctrines green ===`. The design is recorded in
`decision_reference-resolution-journal-fold.md`.

Two containment obligations this append discharged atomically, both surfaced as defects:

- **CHANGELOG rolled over** — the prior append (`STITCHCAD-G1-0004`) crossed the live window's 32 768-byte
  health target (31 237 → 33 338) without sealing, which the protocol requires of the crossing commit. The
  rollover milestone is a convention the size checker only *warns* at (rc=0 past a health target; only the
  ceiling fails), so the miss passed every gate — defect **D54**. Sealed `STITCHCAD-G0-0012` +
  `STITCHCAD-G0-0011` into `stitchcad-changelog-part13.md` (75 lines / 7188 bytes, digest reproduced by
  `run_changelog_ledger_probes.sh`).
- **KNOWLEDGE_MAP hit its 8192-byte ceiling** — the pressure `STITCHCAD-G1-0004` flagged ("99% … for the
  next record/tree addition"): this slice's decision record tipped it to 8216. Tightened two subsystem
  entries (the containment doctrine's only local lever) back under → 8187. The structural cause — the
  generated decision-record and task-tree sections grow a line per slice while the sole trim lever is the
  bounded subsystem list — is defect **D53** for the containment owner. The same edit repaired a run-on
  bullet in `knowledge-map/subsystems.md` (two entries shared one line, invisible to the sync gate, which
  checks derivation not source form).

- lockstep: `G1-SLICE.md` (`.3b` done, a fresh evidence-backed acceptance subsection, frontier → `.3c`,
  verification + commit logs, changelog), `lib.rs` status + module table, `ontology/mod.rs` re-exports,
  `knowledge-map/subsystems.md` + regenerated `KNOWLEDGE_MAP.md`, `docs/TASK_TREE.md`, `MEMORY.md`,
  `LIVE_STATUS.md` (G1 → 4 of 18, census → 9 open), `PLANNING.md` (D53, D54), `DEV_NOTES.md` (the lesson +
  its own rollover), `CHANGELOG.md` (this entry + the part13 rollover).

## STITCHCAD-G1-0004 - the identity layer: determinism is a property of the generator, not the id (leaf `G1-SLICE.3a`)

G1's first new product code. `sc_core::ontology` now carries the identity layer the whole ontology rests on,
implemented against the two design decisions `G1-SLICE.3` recorded:

- `id` — `EntityId`, a hand-rolled dependency-free ULID (128 bits: a 48-bit timestamp and 80 bits of
  randomness, 26-character Crockford base32, lexicographically sortable), and the injected `IdGenerator` trait
  with a `DeterministicIdGenerator`. The design point: a real ULID embeds a wall-clock and randomness, yet
  recipe re-evaluation and CLI replay must be byte-identical and canonical content carries no wall-clock — so
  determinism lives in the *generator*, not the id. Domain code never reads a clock; the composition root
  injects a deterministic generator for replay and a clock-plus-entropy one for production (`G1-SLICE.6`).
- `rational` — `Rational`, a bounded exact rational (`i64` numerator/denominator, `i128` intermediates, reduced
  canonical form). The four operators never round, so a parameter survives unbounded splits and merges with no
  drift; a result past `i64` is a typed `UnitError::Overflow`, never a wrap. Not `sc-units`' ppm `Ratio`, not
  the formula evaluator's bigint.
- `reference` — `EdgeRef`/`PointRef` (the creating operation's `EntityId` plus a persistent `LocalTag`, never
  an array index) and `Param`, a `Rational` constrained to `[0, 1]` at construction.

Validation: `cargo test -p sc-core` → 31 unit + 9 dependency-free recorded-seed properties, all green;
`make check` clean at clippy `-D warnings`; `make wasm` cross-builds `sc-core`; `make gate` → `=== all doctrines
green ===`. Two house conventions re-confirmed: fallible arithmetic is `checked_*` (clippy's
`should_implement_trait`, the precedent `sc-units` set), and a non-`#[test]` helper carries its own targeted
`#[allow(clippy::expect_used)]` because `.clippy.toml`'s `allow-expect-in-tests` does not reach it. The
frontier advances to `.3b` (the persistent-identity contract).

## STITCHCAD-G1-0003 - the ontology leaf is three slices, and its design boundaries are recorded first (leaf `G1-SLICE.3`)

`G1-SLICE.3` named the whole garment ontology as one leaf — identity, the persistent-identity contract, and
nine geometry-bearing object types with their invariants. That is three signoff-quality slices, not one, and
they are strictly ordered (the contract consumes the identity types; the objects consume both). This slice
decomposes `.3` into `.3a` (identity types), `.3b` (the persistent-identity contract) and `.3c` (the object
types), and records the three cross-cutting design boundaries BEFORE any code, so each implementation slice
builds against a fixed design rather than re-deciding it.

The three decisions, each a layer-C record with `answers:` so a later slice asking the question finds it:
`decision_entity-identity-ulid-injected-generator.md` — an `EntityId` is a dependency-free hand-rolled ULID
produced through an injected `IdGenerator`, because `sc-core` builds for wasm and recipe/CLI determinism
forbids a free-function clock; `decision_edge-parameter-bounded-exact-rational.md` — the edge parameter `t` is
a bounded exact rational in `sc-core`'s ontology, not the ppm `Ratio` and not the formula evaluator's
arbitrary-precision rational, with a named promotion trigger to `sc-units` if `sc-geometry` (G2) needs it;
`decision_ontology-invariants-structural-g1-geometric-g2.md` — G1 enforces the structural invariants and hands
CCW winding, simplicity, closure and intake conservation to `G2-2D.1` as a visible `DeferredToG2` state,
never a 2D-correctness claim.

No Rust changes; `make gate` stays `=== all doctrines green ===` and the regenerated Knowledge Map carries the
three new records. The tree is 18 leaves; the frontier advances to `.3a`, G1's first new product code.

## STITCHCAD-G1-0002 - the first property tests set the framework every later crate inherits (leaf `G1-SLICE.2`)

`G1-SLICE.2` (`sc-units`) was the second leaf `G0-CONTRACT.18` (commit `eb83f01`) pre-empted: that commit
landed `sc-units` in full — 1097 lines of library, 564 lines of property tests — as "the first product code",
not the skeleton its own leaf scoped. Like `.1`, the leaf stayed `pending` while its deliverable shipped. This
slice reconciles it: an audit, no new code.

Every acceptance criterion was re-derived by command and pasted into the leaf's `### G1-SLICE.2` checklist:
`cargo test -p sc-units --test property` → `21 passed` (conversion round-trips,
`the_classes_disagree_so_they_are_load_bearing` for class separation, `counts_are_their_own_dimension` and
`non_finite_floats_are_rejected_at_the_boundary` for typed dimension/non-finite errors); `UnitError` is a typed
enum, never a silent coercion; `make wasm` cross-builds the crate; the five tolerance classes are distinct
`ToleranceClass` variants (T1–T5).

The slice also discharges the Open Question `eb83f01` left open — "property-test framework choice, decided in
`.2`" — by recording `decision_property-tests-dependency-free-recorded-seed.md`: dependency-free hand-rolled
properties with a recorded seed are the default on the `wasm-viewer` critical path, and a framework off that
path is a per-crate recorded decision. The record carries `answers:`, which promotes this slice's `DEV_NOTES`
lesson. `make gate` stays `=== all doctrines green ===`. The frontier advances to `.3`, the `sc-core` ontology.

## STITCHCAD-G1-0001 - the frontier pointed at a leaf whose work had already shipped (leaf `G1-SLICE.1`)

The G1 frontier named `G1-SLICE.1` (the workspace crate layout) as the next slice to take, but its every
deliverable had already shipped: `G0-CONTRACT.18` (commit `eb83f01`) retired the bedrock starter crate
"closing defect D10 ahead of `G1-SLICE.1`", created `sc-units` and `sc-core` with the workspace lints
inherited, and landed the G0 CI shape (fmt / clippy / unit+property / a real `wasm32-unknown-unknown` build).
The leaf was left `pending`, so the tree's status disagreed with the workspace — a resuming session pointed at
`.1` would have re-done finished work.

This slice is the reconciliation: it audits `eb83f01` against each of `.1`'s acceptance criteria and records
the closure rather than writing new code. Every criterion was re-derived by command — `cargo metadata
--no-deps` lists exactly `sc-core, sc-units` (the roadmap crates that exist so far; §4.3 grows the rest at
their gates); `git ls-tree HEAD crates/` shows `crates/app` gone and no crate prints the template message;
`make check` is green (21 property tests + doc-test), `make wasm` cross-builds both crates, and
`run_g0_exit_review.sh` reports `G0-17 MET` (CI) and `G0-18 MET` (the wasm build); `KNOWLEDGE_MAP.md` names
both subsystems. `make gate` stays `=== all doctrines green ===`.

The lesson is recorded in `DEV_NOTES.md` and promotion declined there: a leaf's status drifting when a sibling
leaf delivers its work early is an instance of the D34 hand-kept-state class `PLANNING.5` owns, so this slice
fixes the instance and leaves the class to its derivation. The frontier advances to `.2` (`sc-units`), whose
code likewise shipped under `eb83f01` and is reconciled next.

## STITCHCAD-G0-0015 - the gate review is a command, and it reports one clause this repository cannot close (leaf `G0-CONTRACT.15`)

Gate G0 had nineteen obligations, twenty leaves marked done, and no verdict for any of them: the gate's state
existed only as an impression. The leaf's acceptance forbids marking a clause met on prose alone - and a review
WRITTEN as prose is exactly that - so the review parses the roadmap and runs the checks.

- **the review** - `run_g0_exit_review.sh` reads §11's `**Exit:**` bullet, splits it into fragments (plus the
  `Fixture:` bullet), requires every fragment to be dispositioned by a row of `g0_exit_clauses.tsv` and every
  row's key to appear in a fragment, then RUNS each row's check: `G0 EXIT: 18 met / 1 not met / 19 clauses`,
  exit=0, in two seconds. Eleven of the checks are censuses; two are `cargo test -p sc-units` and `make wasm`.
- **the one open clause** - G0-12, evaluation-seat procurement, is `not met` and accepted open by the
  director's ruling of `2026-09-30`, with the cost named rather than hidden: no target system reads our
  artifacts back, so the interchange claims stay `cited-from-roadmap` and G6's receiver validation falls to
  the roadmap's partner-run fallback or does not happen. G0-13 (governance) is met as drafted with its
  qualification printed: the model and both review paths exist, the project owner is the director acting, the
  domain seat is vacant.
- **the closure is unapproved, and that is recorded** - governance §6.1 rule 2 withholds approval of a
  decision's evidence from its author, and the reviewing party authored sixteen of the nineteen deliverables.
  The mitigation §6.1 prescribes is in place: every verdict is a command's exit status, so independence is
  available to whoever reads next instead of being held by anyone now. The roadmap's status line is unchanged,
  which is the honest outcome - line 9 says DRAFT until the exit criteria are met, and one is not.
- **the ruling, recorded when it was made** - `decision_director-ruling-2026-09-30-no-seats-proceed-unapproved.md`
  carries the director's words, the boundary table of what proceeds on engineering evidence alone versus what
  needs a seat and when, and the amendment that the residual dependency is **a measurement, not a
  credential**: knowledge is substitutable by sourced reading (D27 was settled that way), judgement under
  disagreement is substitutable if a synthesized default stays `assumed` and cited, and physical truth is not
  substitutable at all - but it needs a machine, a printer and a ruler, not a hire. `G2-2D.15` was created to
  own that protocol, because a ruling that names a cost without an owner is a wish.
- **the probe suite** - `run_g0_exit_review_probes.sh` -> `10 pass / 0 fail`, including ROADMAP-GROWS (a clause
  added to a COPY of §11 is refused by name), CHECK-FAILS (a failing instrument reads as an unmet clause and
  `GATE FAILS`, not as a broken review), NO-BLOCKER and CONTROL. Two parser bugs were fixed on the way, both
  the session's recurring class: the bullet matcher looked for `**Exit:**` after emphasis had been stripped,
  and one roadmap clause spans two `;`-separated fragments, so a row needed a key list.
- **the tree's own ledger sealed again** - adding the review pushed `G0-CONTRACT.md` to 99 136 bytes against a
  98 304 ceiling, a breach rather than a warning, so four changelog entries went to
  `g0-contract-changelog-part2` (48 lines / 4651 bytes, digest reproduced) and the tree fell to 94 923.
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `22 suite(s) green`; the review re-runs
  every clause's own census green; containment `OK`

## STITCHCAD-G0-0017 - the command layer is a contract, and the roadmap's own lists prove it (leaf `G0-CONTRACT.17`)

§4.4 makes undo/redo granularity a G0 deliverable rather than an implementation detail, and twenty glossary
entries were parked against this leaf under a header note saying "until that chapter lands, the roadmap clause
is cited". The chapter lands, and the two lists the roadmap carries in prose are parsed out of it.

- **the contract** - `docs/book/src/spec/command-layer.md` (264 lines / 18 959 B): seventeen commands in five
  classes, where a class fixes a command's authority, its reversibility and its undo granularity, so an adapter
  cannot re-classify one for convenience; the seven fields of a command's shape; **undo at the atomic group,
  restoring semantics - recipe, entity identities, revision - rather than contours**, because two designs with
  the same contours and different recipes are different designs; evaluations and artifacts discarded rather
  than undone; the history not canonical content, so a reopened project starts fresh at its saved revision.
- **preview/commit, preconditions and idempotency** - a preview needs only `inspect` and a commit re-checks
  the precondition the preview passed, because three front-ends and an agent edit one design; a mutation
  carries the revision it was authored against and an idempotency key, so a stale revision is refused naming
  both and a transport retry is a reported replay rather than a second edit; the audit trail is append-only
  and is not the undo history.
- **authority as a core-enforced permission on a class** - the five levels of §7.8, with `approve` unholdable
  by an agent, which is the release chapter's `release_approver_not_human` seen from the other side. The
  parity table's eight columns, closed cell vocabulary and generation rule are normative while its rows stay
  empty, because rows at G0 would be claims about unwritten adapters - the same reason the canvas spike's
  results file is empty. The undo depth is the one number deliberately not written: declared to exist and to
  be bounded, with its value belonging to the resource bounds G1 measures.
- **the instrument** - `run_command_layer_census.sh` parses roadmap §4.4's backticked command list and §7.8's
  slash-separated levels and compares both with the chapter, in both directions for the levels:
  `17 commands / 5 classes / 5 levels / 0 failure(s)`. Its first run parsed zero levels - the list wraps
  mid-item - and reported five invented ones, so the fix is whitespace normalisation and the lesson is that a
  reader must fail closed when it reads nothing. `run_command_layer_probes.sh` -> `14 pass / 0 fail`, with
  LEVEL-EXTRA and LEVEL-MISSING as separate arms and HUMAN-ONLY refusing a softened rule.
- **decisions** - `docs/decisions/decision_command-layer-contract-and-undo-granularity.md` records ten
  rejected alternatives, including per-command undo, contour-level undo, undoing a generated instance,
  persisting the stack, trusting a preview, retry-by-repeat, per-tool permission lists and a hand-written
  parity table.
- **D49's third trigger discharged** - the tree had reached 94 % of its byte ceiling, so the decisions of
  leaves `.1`-`.4c` were sealed into `g0-contract-decisions-part1` (112 lines / 10 079 bytes, digest
  reproduced, 33 segment verdicts) and `.16`'s checklist moved to the evidence sibling.
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `21 suite(s) green`; `make book` ->
  exit=0; all nine other censuses green; containment `OK - 17 surfaces, 15 routes, 113 files measured`

## STITCHCAD-SPINE-0019a - the closed defects are sealed, so the open ones are what a reader meets (leaf `SPINE.19.1`)

D46 named this remedy and `SPINE.19` owns its durable half, but the deferral's premise - "the file is inside
every ceiling today" - had expired: `PLANNING.md` was at 1133 lines / 93378 bytes, 95 % of its byte ceiling,
and 50292 of those bytes belonged to defects already fixed. The next defect any slice logged would have
blocked a commit, so the seal happened at the trigger D49 declares rather than at the breach.

- **the seal** - 44 closed entries moved to `docs/history/stitchcad-defects-part1.md` (624 lines / 53323
  bytes, `sha256:1897bde0…`) in the census's own order, under the descriptor contract; the live census keeps
  the 7 open defects, a pointer, and the two `grep -c` commands that derive both counts instead of a
  hand-kept pair. `PLANNING.md` is now 515 lines / 40606 bytes, inside its health on both axes.
- **nothing lost, nothing duplicated** - 7 live + 44 sealed = the 51 that were there, and the intersection
  of the two id lists is empty. The standing `DESCRIPTOR` rule reproduces the new segment's digest:
  `run_changelog_ledger_probes.sh` -> `9 pass / 0 fail`, `REAL` at 32 segment verdicts.
- **classifying the census is still a reading, and that is D38 measured again** - three entries (`D7`, `D9`,
  `D15`) record their closure in wording no marker list anticipated, so the open/closed split was made by
  reading each owner line. D38's ask (a status token a script can read) stays open with `PLANNING.5`, and
  `SPINE.19` keeps the durable half: a ledger-agnostic verifier, a segment registry, and an arm that refuses
  a fixed defect left live - because today nothing would notice one.
- **two containment defects found in this slice's own work and fixed before committing** - the first seal
  wrote the descriptor without the content (a missing concatenation), which the `DESCRIPTOR` rule caught as a
  one-byte segment; and a changelog entry built as one unwrapped string came out at 928 bytes against a
  600-byte ceiling, the only hard breach this session produced.
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `20 suite(s) green`; containment
  `OK - 17 surfaces, 15 routes, 111 files measured`; the coverage census still `10 lanes / 13 trees /
  3 sibling(s) / 0 unowned / 0 orphan(s) / 0 dead link(s)`, so a history segment is not mistaken for a task
  tree

## STITCHCAD-SPINE-0021 - the cadence runs, and the residue census proves what it took (leaf `SPINE.21`)

The cleanup cadence had no recurring owner: `SPINE.2` discharged the first run and wrote the record, but a
cadence is an obligation that returns, and this one was 23 hours from firing mid-slice with no leaf to own it.

- **the run** - nine paths removed, each named by the residue census and each found gone: both scratch trees
  (`target/doctrine_scratch`, `target/scratch`, `target/tmp`), both incremental caches
  (`target/debug/incremental`, `target/wasm32-unknown-unknown/debug/incremental`, 57 `.bin` files), the mdBook
  output (`docs/book/book`, 4 120 KB) and three scratch bodies the containment self-tests had left in
  `target/`. `target` went 40 648 KB -> 10 808 KB, so 33 960 KB left the volume counting the book.
- **nothing tracked was touched** - `git ls-files | grep -cE '^(target/|docs/book/book/)'` -> `0` before and
  after, `0` tracked artifact-shaped files, `0` deleted tracked files in `git status --porcelain`, and `0`
  stray `*.log` / `*.bin` / `*.tmp` / `*.orig` / `*.rej` / `.DS_Store` anywhere outside `.git`.
- **the removal is shown to cost rebuild time and nothing else** - `make gate` -> `=== all doctrines green ===`;
  `make check` -> `test result: ok. 1 passed; 0 failed`; `make book` -> regenerated at exactly 4 120 KB;
  `make probes` -> `20 suite(s) green` with `target/scratch` recreated by the Makefile's own rule;
  `make wasm` -> the smoketest green. `target` rebuilt to 13 460 KB.
- **D34's fourth instance removed** - the index's `SPINE` frontier cell still named `.20` as open one commit
  after it landed, while the execution-order paragraph in the SAME file had it right: two hand-kept sentences
  about one lane, drifting against each other, which is the strongest argument yet for deriving the cells.
- `SPINE.20`'s checklist moved to `SPINE-evidence.md`, as the convention requires of the slice after the one
  that landed it, bringing the tree back inside its per-part health (683 lines / 56 833 B); the changelog's own
  rollover follows in this entry (`part10`).

## STITCHCAD-G0-0016 - one message system, and an inventory nothing keeps by hand (leaf `G0-CONTRACT.16`)

Roadmap §7.6 required the choice at G0 and refused "Fluent or ICU" as an answer. The repository had neither the
choice nor the architecture, and seven references across five chapters pointed at this leaf instead of a clause.

- **the choice, on evidence** - **Fluent at both ends**: `fluent-rs` in the Rust core and `fluent.js` in the
  TypeScript chrome over one catalogue format, both Apache-2.0 and unarchived, read on `2026-09-30` from the
  GitHub and crates.io APIs and tabulated in the chapter. ICU4X is the rejected alternative and stays a named
  one - active, Unicode License V3, built for resource-constrained clients, which is this product's browser
  profile - and it loses on the boundary: ICU4X in Rust and the platform's Intl in the browser are two message
  dialects, the failure the clause exists to prevent. The bridge stays in reserve for a measured plural-rule or
  locale-data gap, which is `unverified-with-owner` with G5.
- **the architecture** - the message id IS the diagnostic token, so there is no second numbering to drift;
  arguments are typed and carry units and text is a presentation field; the termbase is a projection of the
  glossary with the ⚠ terms first and no guessed translation; the externalization lint has five exemptions
  each carrying a reason; canonical files are locale-free, so a decimal comma changes nothing stored;
  pseudolocalization runs in CI before any translator exists; RTL mirrors layout and never geometry, as a
  byte-comparison test (`i18n_geometry_mirrored`) rather than a sentence; and three review tiers make `strict`
  and `safety` absolute - so with the domain seat vacant no pack can ship today, stated rather than hidden.
- **the instrument** - `run_i18n_census.sh` derives the inventory from the envelope's §10, the formula
  language's §5.2, the dialects' §11, the release contract's §9 and `crates/sc-units/src/error.rs`:
  `8 families / 64 message ids / 0 failure(s)`. Deriving it caught two errors in the chapter's own first
  draft - the envelope's 29th token `geom_offset_budget` folded into the `env_*` family, and `UnitError`
  counted at four variants when it carries five, because the fifth has no braces and a first grep looked for
  braces. `run_i18n_probes.sh` -> `11 pass / 0 fail`, including CODE-GROWS, which adds a variant to a COPY of
  the crate's source and requires the count to be refused.
- **decisions** - `docs/decisions/decision_i18n-one-message-system-fluent.md` records the choice, the
  comparison table and six rejected alternatives, including percentages as review thresholds ("90 % of the
  safety tier" is one message in ten saying the wrong thing about a cut line).
- **glossary** - a ninth part, `localization.md`, with seven terms; the parts table, SUMMARY and the A-Z index
  updated; six cross-references repointed from this leaf to a clause: `305 terms / 9 parts / 156 tokens /
  0 failure(s)`.
- **D49's trigger fired on the tree file itself** at 91 % of its byte ceiling, so the oldest fourteen
  changelog entries were sealed into `g0-contract-changelog-part1` (84 lines / 7941 bytes, digest reproduced)
  and the tree fell to 81 811 B; the dev-notes ledger rolled over in the same commit (`devnotes-part5`,
  43 lines / 3977 bytes), `run_changelog_ledger_probes.sh` -> `9 pass / 0 fail`.
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `20 suite(s) green`; `make book` ->
  exit=0; all eight other censuses green; containment `OK - 17 surfaces, 15 routes, 105 files measured`

