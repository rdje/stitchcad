# SPINE: project identity, hygiene and adopted policy (repo-local lane)

## Metadata

- Tree ID: `SPINE`
- Status: `active`
- Roadmap lane: none directly — this tree owns the repository's own discipline surfaces so
  the roadmap lanes can execute inside them (session directives §8, §12–§14, §17, §18)
- Created: `2026-09-29`
- Owner: repo-local workflow

## Goal

Make this repository *look and behave like StitchCAD* rather than like the template it was
generated from, and adopt the director's external policy references into repository-owned,
mechanically-enforced form:

- identity surfaces de-templated (README landing page, mdBook identity, live status, toolbox,
  knowledge map);
- artifact-cleanup cadence recorded and honoured;
- README policy refreshed to the revised source, claim-verification policy adopted in-repo;
- live-document size containment adopted for the surfaces that are about to grow;
- every path repository-relative and every project-owned artifact on the repository volume.

## Non-Goals

- Product specification or code (that is `G0-CONTRACT` and the `G1-SLICE`+ lanes).
- Weakening any doctrine gate to make a slice land. A cap is never raised to fit content;
  content is demoted (see `MEMORY_ARCHITECTURE.md` §6, `README_POLICY.md`).
- Writing to any other git repository. External policy sources are read-only references;
  what is adopted is copied into this repository.

## Acceptance Criteria

- No tracked document introduces this project as the bedrock template.
- The mdBook names StitchCAD, builds with `make book`, and has a SUMMARY that the G0 spec
  chapters can grow into.
- `docs/ARTIFACT_CLEANUP.md` exists, carries exactly one entry (the latest), and the cleanup
  it records was actually run with a residue census.
- The adopted policy copies are repository-owned, carry a local adoption note, and are
  enforced by a check in `scripts/check_doctrines.project.sh` (the project slot), not by prose.
- `make gate` and `make check` stay green at every leaf.
- Each completed leaf is committed through `COMMIT.md`.

## Task Tree

- ID: `SPINE`
  Status: `active`
  Goal: the repository's own surfaces are project-shaped, bounded and enforced.
  Children: `.1` … `.10`

- ID: `SPINE.6`
  Status: `done`
  Goal: track the workspace lockfile so a fresh clone's first `make check` leaves the tree
  clean (defect D14). Inserted ahead of `.1`–`.5` because the pivot rule defines
  handoff-ready as *no untracked files*, and the toolchain itself was breaking it.
  Acceptance: `Cargo.lock` is tracked; `make check` from a clean checkout ends with an empty
  `git status --short`; `.gitignore`'s stated policy (lockfile deliberately not ignored) holds.
  Verification: recorded below.
  Commit: `STITCHCAD-SPINE-0006`

- ID: `SPINE.12`
  Status: `done`
  Goal: record the director's push cadence — **400 commits between pushes** — in the workflow document
  that owns it (`COMMIT.md`), with the count derived rather than hand-carried and the trade-off
  against `MEMORY_ARCHITECTURE.md` §8's crash-insurance argument stated instead of hidden.
  Acceptance: `COMMIT.md` carries a Push cadence section naming the threshold, the deriving command,
  the pre-push gate and the recorded trade-off; the current ahead-count is measured; no push happens
  before the threshold unless the director asks.
  Verification: recorded below.
  Commit: `STITCHCAD-SPINE-0012`

- ID: `SPINE.11`
  Status: `done`
  Goal: rebuild the evidence-signature corpus as a **tracked** instrument (defect **D20**) so the
  published awk-versus-grep measurement in `SPINE.8`'s record is re-derivable. Deliverable:
  `docs/tasks/artifacts/evidence_signatures/` holding the corpus and a runner that reports, for each
  line, whether the universal signature list matches it under `grep -qE` (the engine the gate uses)
  and under `awk` (the engine a re-implementation might wrongly choose).
  Acceptance: one command reproduces both published numbers (`12` of `36` unmatched under awk, `2`
  under grep, the two being bad samples rather than dead families); the corpus and runner are
  tracked (`git ls-files` proves it); `make probes` picks the runner up; a `TOOLBOX.md` row names it;
  the leg-3 claim in `CLAIM_VERIFICATION.md`'s adoption note stops being an open breach.
  Verification: recorded below — `corpus: 36 lines · grep-unmatched: 2 · awk-unmatched: 12`,
  `probes: 5 pass / 0 fail`, `make probes: 6 suite(s) green`.
  Commit: `STITCHCAD-SPINE-0011`

- ID: `SPINE.1`
  Status: `done`
  Goal: de-template the identity surfaces — `README.md` as the StitchCAD landing page (within
  the reviewed caps), `docs/book/book.toml` identity, `docs/book/src/introduction.md`, and a
  `SUMMARY.md` skeleton with the `spec/` part that `G0-CONTRACT` fills. Owns defects D3, D4
  and D19.
  Acceptance: `head -1 README.md` names StitchCAD; `mdbook build docs/book` succeeds; the
  README quick start and links are verified by running them; `README-STABILITY` green.
  Verification: recorded below — every quick-start command run, book builds, `README-STABILITY: OK`.
  Commit: `STITCHCAD-SPINE-0001`

- ID: `SPINE.2`
  Status: `done`
  Goal: first artifact cleanup + the cadence record `docs/ARTIFACT_CLEANUP.md` (single latest
  entry: date + one-line summary). Owns defect D8.
  Acceptance: cleanup ran on the repository volume only (`target/`, stray `.log`/`.bin`,
  doctrine scratch dirs); a residue census proves what was removed is gone; `make check` still
  green afterwards; nothing tracked was deleted.
  Verification: recorded below — 4 paths removed, residue census clean, all four make targets green.
  Commit: `STITCHCAD-SPINE-0002`

- ID: `SPINE.3`
  Status: `done`
  Goal: adopt the revised external policy references into repository-owned copies — refresh
  `README_POLICY.md` (authority/provenance note, duplication probe, routing-pressure closure,
  derived caps, unconditional check) and add the claim-verification standard in-repo, wired
  into the bootstrap reading list. Owns defects D11, D12.
  Acceptance: the adopted copies carry a StitchCAD adoption note and no external absolute
  path; the README guard still passes and additionally validates its routed destinations where
  the revised policy requires it; a decision record names what was adopted and what was
  deliberately not (donor-specific values).
  Verification: recorded below — both copies adopted, `README-STABILITY: OK`, `0` donor tokens,
  `0` absolute paths, adoption frontier owned by named leaves.
  Commit: `STITCHCAD-SPINE-0003`
  Note: the revised policy's **routing-pressure closure** (a data-only destination registry with an
  owner, lifecycle class and pressure control per route) is *not* implemented by this leaf — it is
  recorded in the adoption note as owed by `SPINE.4`, together with the derived caps the policy
  requires instead of the inherited template defaults. Adopting a policy creates obligations; the
  frontier is where they are tracked rather than forgotten.

- ID: `SPINE.4`
  Status: `active`
  Goal: adopt live-document size containment for this repository — surface inventory,
  lifecycle class, health target and enforcement ceiling per governed surface
  (`MEMORY.md`, `README.md`, `ROADMAP.md`, `CHANGELOG.md`, `DEV_NOTES.md`, `LIVE_STATUS.md`,
  `docs/TASK_TREE.md`, `docs/tasks/*`, `docs/decisions/*`, `docs/book/src/*`), plus a
  deterministic ratchet check registered in the project doctrine slot. Owns defect D13.
  Acceptance: the checker runs unconditionally in the hook and fails on an unclassified or
  over-ceiling surface; its RED arm is demonstrated (a control that has been seen to fire);
  ceilings are derived from the measured survivor with modest headroom, not copied from a
  donor project; existing pressure is recorded as transition debt with an owner.
  Verification: per sub-leaf below.
  Commit: per sub-leaf
  Children: `.4.1` (doctrine adopted in-repo), `.4.2` (inventory, targets, ceilings, routes),
  `.4.3` (the deterministic checker + probe arms). Split because the adoption guide requires these as
  separately committable slices, and because one slice that both chose the ceilings and enforced them
  would have no independent evidence for either.

- ID: `SPINE.4.1`
  Status: `done`
  Goal: adopt the containment **doctrine** as a repository-owned copy — neutral body verbatim behind
  a fenced StitchCAD adoption note stating authority, independence, milestones, locality,
  serialization, landing-page identity, transition-debt policy, and (because a partial adoption that
  hides its gaps is worse than none) the **deferred** neutral checker package with three named
  triggers that reopen the decision.
  Acceptance: the body is verbatim and donor-free; no absolute path; the note names what is adopted,
  what is deferred and why; `COMMIT.md` routes authors to it; a layer-C record carries the
  proportionality call; `make gate` green.
  Verification: recorded below.
  Commit: `STITCHCAD-SPINE-0004`

- ID: `SPINE.4.2`
  Status: `done`
  Goal: the local data plane — `.doctrine/live_document_size/surfaces.tsv` (one row per governed
  surface: path or glob, lifecycle class, owner, authority, measured lines/bytes/max-line, health
  target, inclusive ceiling, transition-debt baseline) and `routes.tsv` (every destination the README,
  the policy and the guard's failure guidance route to, with its owner, lifecycle class and pressure
  control). Derive the README caps from the trimmed survivor and set them through
  `README_LINE_CAP`/`README_BYTE_CAP` instead of the inherited 300/16 384 defaults.
  Acceptance: every live surface in the repository is classified (the census command is recorded);
  no route ends at an unclassified or unbounded destination; every ceiling is derived from a
  measurement stated in its row; the README caps are recorded as reviewed values in the adoption note;
  nothing is copied from the donor project.
  Verification: recorded below — 17 surface rows, 15 route rows, three surfaces trimmed first.
  Commit: `STITCHCAD-SPINE-0004b`

- ID: `SPINE.4.3`
  Status: `pending`
  Goal: the deterministic checker `scripts/check_live_doc_size.sh` registered in the project slot —
  fails on an unclassified surface, a missing owner/lifecycle/ceiling, an absolute or off-volume path
  in the data plane, a line/byte/max-line overflow past a ceiling, a ceiling raised without a recorded
  authority, a route to an uncontrolled destination, and a stale derived projection; warns at 80 % of a
  health target. Plus a probe suite with a RED arm per refusal class.
  Acceptance: `make gate` runs it unconditionally (not staged-scoped); every refusal class has an arm
  seen to fire; the healthy tree passes; `TOOLBOX.md` names it; D13 closes.
  Verification: `pending`
  Commit: `pending`

- ID: `SPINE.13`
  Status: `pending`
  Goal: give `ROADMAP.md` the navigation a `maintained_reference` requires — a complete section index
  with its own bounds, and per-section pressure limits, so the 919-line document is browsable without
  a full read and its growth is governed rather than merely capped.
  Acceptance: every top-level section is reachable from the index; the index itself is inside its
  declared bounds; the containment registry's `roadmap` debt row is cleared or its baseline lowered;
  no roadmap decision is reopened by the edit (navigation only — the revision policy in the roadmap's
  own header still governs content changes).
  Verification: `pending`
  Commit: `pending`

- ID: `SPINE.14`
  Status: `done`
  Goal: make `CHANGELOG.md` a bounded rolling ledger — seal the inherited bedrock-scaffold segment
  (frozen legacy, not this project's history) into `docs/history/` as an immutable archive terminal
  with recorded identity (lines, bytes, sha256, the revision it was sealed at), leave a one-line
  pointer, and state the rollover rule for our own entries.
  Acceptance: the sealed segment is byte-identical to what was removed (proved by hash before and
  after); `CHANGELOG.md` drops below its health target; the registry's `changelog` debt row is cleared;
  the `history_archive` surface exists and is classified; retrieval is one documented command.
  Verification: recorded below — `BYTE-IDENTICAL: True`, `sha256:78f43e0f…`, ledger 522→371 lines.
  Commit: `STITCHCAD-SPINE-0014`

- ID: `SPINE.15`
  Status: `pending`
  Goal: settle defect **D22** with a real oracle — does a GFM renderer split a table cell on a raw `|`
  inside a code span? — then record the answer and adopt the table-authoring convention for this
  repository (escape pipes inside code spans; keep cells short enough that the maxline axis stays
  visible), and clear the `decisions_collection` maxline debt by reformatting the one wide row.
  Acceptance: the oracle and its output are recorded (a rendered page, not a reading of the spec);
  the convention is written where authors look (`COMMIT.md`); the widest line in the files this project
  owns is inside its health target; the inherited checker is left untouched and the upstream question
  is reported.
  Verification: `pending`
  Commit: `pending`

- ID: `SPINE.5`
  Status: `pending`
  Goal: seed the orientation surfaces — `TOOLBOX.md` project-toolbox rows for the instruments
  that exist (doctrine enforcer, per-check `--self-test` arms, `make check`/`gate`/`book`,
  the knowledge-map generator) and `knowledge-map/subsystems.md` rows for the documentation
  surfaces. Owns defects D7, D9.
  Acceptance: no placeholder token remains in either file; every listed tool is invoked at
  least once and its real output shape recorded; the Knowledge Map regenerates in sync.
  Verification: `pending`
  Commit: `pending`

- ID: `SPINE.7`
  Status: `done`
  Goal: prove and publish defect **D15** — the spine's `TASK-ACCEPTANCE` gate judges the FIRST
  box matching each label in a staged tree file, so in a multi-leaf file one leaf's evidence
  answers for another leaf's code change (false GREEN), and an earlier unticked placeholder
  rejects a leaf that carries real evidence later in the same file (false RED). Deliverable: a
  committed probe (`docs/tasks/artifacts/task_acceptance/run_multileaf_shadowing_probe.sh`,
  a first-class diagnostic tool per `TOOLBOX.md`) with RED/GREEN/CONTROL arms, plus the local
  authoring convention recorded as a layer-C decision.
  Acceptance: the probe runs by one command, prints `probes: N pass / 0 fail`, and its arms
  reproduce both directions; the convention (per-leaf `### <leaf-id>` checklist subsections,
  added in the same commit as the work; no unticked placeholder boxes in tree files) is written
  into `docs/tasks/TEMPLATE.md` and the existing tree files; the finding is recorded with its
  measured evidence and flagged to the director as an upstream spine defect.
  Verification: recorded below — `probes: 6 pass / 0 fail`.
  Commit: `STITCHCAD-SPINE-0007`
  Note: the convention deliberately did **not** go into `docs/tasks/TEMPLATE.md` — that file is in
  `scripts/update_scaffold.sh`'s NEUTRAL sync list, so a project convention written there is
  overwritten by the next spine update (defect D17). It lives in a layer-C decision record instead,
  which is project-owned.

- ID: `SPINE.8`
  Status: `done`
  Goal: close D15 locally with a project-slot doctrine — `FRESH-ACCEPTANCE-EVIDENCE`: a staged
  CODE change must be accompanied, **in the same commit's diff**, by added ticked ROOT CAUSE /
  ADDRESSED / NO REGRESSION bullets carrying tool-output signatures, inside a `### <leaf-id>`
  subsection of a staged `docs/tasks/*.md`. Registered in `scripts/check_doctrines.project.sh`
  (never in the universal driver), with `--self-test` arms including a control seen RED.
  Design constraint, learned from D15 facet 3: a project-slot check can only ADD refusals — it
  cannot relax a universal one — so this leaf also publishes the two authoring rules that keep the
  inherited check honest on mixed commits: (i) every evidence bullet cites the invocation, its
  output **and its exit status** (`rc=0`), because a listing of filenames matches no signature
  family; (ii) a code commit stages the leaf that owns it, and unrelated tree updates go in their
  own commit, because the inherited check judges every staged leaf file.
  Acceptance: previously-committed evidence cannot satisfy a new code commit (the false GREEN is
  closed); an honest leaf whose checklist is added with its work passes regardless of its position
  in the file (the false RED is closed); `make gate` green; a `TOOLBOX.md` row names the check and
  what question it answers; the two authoring rules are in the layer-C record.
  Verification: recorded below — `probes: 9 pass / 0 fail`, `8 verdict controls + 6 extractor arms`.
  Commit: `STITCHCAD-SPINE-0008`

- ID: `SPINE.9`
  Status: `done`
  Goal: make the scaffold updater safe for project content (defect **D17**) —
  `scripts/update_scaffold.sh` lists `docs/TASK_TREE.md`, `TOOLBOX.md`, `README_POLICY.md` and
  `docs/tasks/TEMPLATE.md` as NEUTRAL ("safe to overwrite because it never carries project
  content"), yet the template's own instructions tell the project to fill exactly those in: the
  Active Task Trees index (layer-B navigation), the project toolbox table, the README policy's
  local adoption note. Split the list into truly neutral files and files carrying project
  sections; the latter are backed up, reported and skipped unless an explicit flag is passed.
  Acceptance: a dry run against a stub source proves the guarded files are not overwritten and the
  neutral ones are; the backup lands on the repository volume; a decision record
  (`decision_scaffold-sync-protects-project-content.md`, already cross-linked from the
  acceptance-evidence record) states the rule; `make gate` green.
  Verification: recorded below — `probes: 7 pass / 0 fail`.
  Commit: `STITCHCAD-SPINE-0009`

- ID: `SPINE.10`
  Status: `done`
  Goal: keep project-owned scratch on the repository volume (defect **D16**) — the inherited probe
  suites call `mktemp -d`, which resolves to the system volume (`/var/folders/…` here) while the
  repository lives on another volume; pin scratch with a documented, scripted `TMPDIR` and give the
  suites one entry point (`make probes`) so nobody has to remember the incantation.
  Acceptance: `make probes` runs every probe suite with `TMPDIR` inside `target/`; the pin is
  measured (`TMPDIR=… mktemp -d` prints a repository-volume path); all suites still report
  `probes: N pass / 0 fail`; no inherited file is edited (the pin is applied by the caller).
  Verification: recorded below — `make probes` → `5 suite(s) green`; 37 arms, 0 failures.
  Commit: `STITCHCAD-SPINE-0010`
  Residual, accepted and reported rather than hidden: `scripts/check_task_acceptance.sh` (shared,
  NEUTRAL) still calls `mktemp -d` for a trap-cleaned scratch on every commit, so that one transient
  directory lands wherever `TMPDIR` points. Pinning it would mean editing shared code or a NEUTRAL
  hook; the scratch is removed by its own trap and persists nothing, so the accepted residual is
  recorded here and the upstream suggestion (have the driver export a repo-local `TMPDIR`) is
  reported with D15/D17.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 0 | `SPINE.6` | `done` | taken out of order: it repaired a dirty-tree defect found on the first `make check` |
| 0 | `SPINE.7` | `done` | taken in frontier order: D15 had to be measured before it could be owned |
| 1 | `SPINE.8` | `done` | landed in frontier order: every code commit from here on is judged by it |
| 2 | `SPINE.9` | `done` | landed in frontier order: it protects the layer-B index from the documented maintenance command |
| 3 | `SPINE.10` | `done` | landed in frontier order: one entry point, scratch on this volume |
| 4 | `SPINE.1` | `done` | identity landed: README, book identity, the `spec/` part, and a clean `make book` |
| 5 | `SPINE.2` | `done` | cleanup ran and the cadence now has a record a next session can read |
| 6 | `SPINE.3` | `done` | both policies are repository-owned now, so later slices write under them |
| — | `SPINE.12` | `done` | taken out of order: a director ruling is recorded when it is made, not at the end of the lane |
| 7 | `SPINE.11` | `done` | the published number has a tracked, watched producer again |
| 8 | `SPINE.4` | `active` | split into `.4.1`–`.4.3`; the doctrine is adopted, the data plane and the checker follow |
| 8a | `SPINE.4.1` | `done` | the rules are in-repo, so `.4.2`/`.4.3` choose numbers under a stated contract |
| 8b | `SPINE.4.2` | `done` | the data plane exists and three surfaces were trimmed before their targets were set |
| 8c | `SPINE.4.3` | `pending` | **next** — declared rows are not gated rows; the checker makes them real and closes D13 |
| 9 | `SPINE.5` | `pending` | toolbox rows are honest only once the instruments are in use |
| 10 | `SPINE.13` | `pending` | roadmap navigation + per-section bounds: the `maintained_reference` debt |
| 11 | `SPINE.14` | `done` | taken before `.4.3`: the ledger had to be inside its window before a baseline could be declared honestly |
| 12 | `SPINE.15` | `pending` | settle D22 against a real renderer and adopt the wide-row convention |

## Decisions

- `2026-09-29`: adopted policy is **copied into the repository** and enforced here; the
  external references are read-only and are never a build input (session directives §12
  exception, §21). A repository that must reach another checkout to know its own rules is not
  relocatable.
- `2026-09-29`: project-specific gates go in `scripts/check_doctrines.project.sh`, never in
  the universal driver (`DOCTRINE_ENFORCEMENT.md`).
- `2026-09-29`: a defect found in the **inherited spine** is fixed locally in the project slot and
  reported to the director, never patched into the universal checks — those are shared, portable
  code kept in sync by `scripts/update_scaffold.sh`, and a local edit there would be silently
  overwritten or, worse, silently diverge (defect D15 → `SPINE.7`/`SPINE.8`).
- `2026-09-29`: project conventions do not go into files the scaffold updater treats as neutral
  (`docs/TASK_TREE.md`, `TOOLBOX.md`, `README_POLICY.md`, `docs/tasks/TEMPLATE.md`). They go into
  layer-C records and project-owned files, and the updater grows a guard (defect D17 → `SPINE.9`).
  Content already written into those files stays — it is the guard's job to protect it, not ours to
  evacuate it.

## Routing Evidence

Defect **D15** is a property of `scripts/check_task_acceptance.sh`, which this repository inherits
from the bedrock spine, so the durable fix belongs to the spine's own maintenance and is reported to
the director rather than patched here (session directive §21: other repositories are read-only).

- **Does it reproduce outside this repository's own tree files?** Yes, by construction: the
  first-match-per-file scan is in the shared script, and the probe builds throwaway repositories
  containing none of this project's files. The behaviour is a property of the spine, not of our leaves.
- **What was measured:** two arms of a scratch probe over the shipped check — ARM-1 (leaf A ticked
  above, leaf B unticked, code change owned by B) → `task-acceptance: OK …`, `exit=0`; ARM-2 (same
  file, order reversed) → three `box is present but NOT ticked` refusals, `exit=1`. Same code change,
  opposite verdicts, differing only in the order of two sections in one file.
- **What would make this routing wrong:** if the spine documented one-checklist-per-file as an
  authoring contract, ARM-1 would be an author error rather than a gate gap. The check's own header
  claims the opposite — it states that box-scoping closed "a co-staged unrelated leaf supplying the
  evidence" — and its probe suite (`docs/tasks/artifacts/task_acceptance/`) exercises that leakage
  only across FILES, so the claim is broader than the property it verifies.
- **Local mitigation owned here:** `SPINE.8` (`FRESH-ACCEPTANCE-EVIDENCE` in the project slot).

## Open Questions

- Whether the containment checker (`.4`) should be one script or reuse the existing
  `check_readme_stability.sh` + `check_memory_architecture.sh` with a new registry-driven
  surface check beside them. Decided inside `.4` with the measured inventory in hand.

## Blockers

- None.

## Acceptance Checklist

### `SPINE.6` — track the workspace lockfile (defect D14)

- [x] **ROOT CAUSE (WHY + WHERE)** — the bootstrap commit never ran cargo, so the generated
  lockfile was absent from the index while `.gitignore` deliberately does not ignore it:
  `make check && git status --short` → `?? Cargo.lock`; `git check-ignore -v Cargo.lock` →
  `rc=1` (not ignored); `.gitignore` line: `# Note: Cargo.lock is intentionally NOT ignored —
  binary/application projects should commit it for reproducible builds.`
- [x] **ADDRESSED (verified)** — `git ls-files --error-unmatch Cargo.lock` → `Cargo.lock`,
  `rc=0` (tracked); `git status --short` after `make check` → empty, so a fresh clone that runs
  the documented check stays handoff-ready.
- [x] **NO REGRESSION** — `make check` → `cargo fmt --check` clean, clippy clean,
  `test result: ok. 1 passed; 0 failed`; `scripts/check_doctrines.sh` → `=== all doctrines
  green ===`, `rc=0`.
- [x] **FIX** — `git add Cargo.lock` (no hand edit: the file is cargo-generated).
- [x] **LOCKSTEP** — D14 logged with its reproduce command and owner in `PLANNING.md`; this
  leaf records the fix; `MEMORY.md` unchanged (frontier did not move).

### `SPINE.8` — `FRESH-ACCEPTANCE-EVIDENCE`: evidence must be added by the commit it vouches for

- [x] **ROOT CAUSE (WHY + WHERE)** — `scripts/check_task_acceptance.sh:106-112` collects the first
  bullet matching each label and stops at the next box, so evidence committed for an earlier leaf
  answers for a later leaf's code change. Reproduced end to end by the new probe's pairing arm:
  `bash docs/tasks/artifacts/fresh_evidence/run_fresh_evidence_probes.sh` →
  `✓ RED-1   exit=1  code owned by leaf 2, evidence committed for leaf 1 → REFUSED` followed by
  `↳ the inherited universal check accepts this same staged state (exit=0)` — one staged state, two
  verdicts, which is exactly the gap a project-slot check can close and a universal one cannot see.
- [x] **ADDRESSED (verified)** — before: the project slot was a stub (`grep -c 'PROJECT_DOCTRINES'
  scripts/check_doctrines.project.sh` → `0`) and no check asked whether evidence was fresh; after:
  `scripts/check_fresh_acceptance_evidence.sh` is registered, its probe reports
  `probes: 9 pass / 0 fail` and its self-test `8 verdict controls + 6 extractor arms`, `exit=0`.
  One defect in the first implementation was caught by its own arm: signature matching written in
  awk left **12 of 36** corpus evidence lines unmatched on this platform (`\b`, `{n}` are GNU
  extensions BSD awk 20200816 lacks) where `grep -qE` — the engine the universal check uses —
  leaves **2**, both bad samples; the check now uses grep and pins it with the `GREEN-2` arm.
- [x] **NO REGRESSION** — `scripts/check_doctrines.sh` → `=== all doctrines green ===`, `exit=0`
  (13 checks, project slot now reporting `PROJECT-SPECIFIC: 1 project doctrine(s) green`);
  the inherited suites are untouched and still discriminate:
  `bash docs/tasks/artifacts/task_acceptance/run_task_acceptance_probes.sh` → `probes: 10 pass / 0 fail`,
  `bash docs/tasks/artifacts/task_acceptance/run_multileaf_shadowing_probe.sh` → `probes: 6 pass / 0 fail`;
  `make check` → `test result: ok. 1 passed; 0 failed`; per-commit cost measured at ~0.3 s.
- [x] **FIX** — added `scripts/check_fresh_acceptance_evidence.sh` (pure verdict function + ground
  truth on every invocation, fail-closed signature extraction, scratch on the repository volume);
  replaced the stub body of `scripts/check_doctrines.project.sh` with a project registry that
  mirrors the driver's reporting shape; added the 9-arm probe suite. No universal file edited.
- [x] **LOCKSTEP** — `TOOLBOX.md` names both new instruments; `DEV_NOTES.md` carries the lesson
  (including the withdrawn D18 near-miss); the layer-C record states the two authoring rules this
  check cannot enforce; `PLANNING.md`, `LIVE_STATUS.md`, `MEMORY.md`, `CHANGELOG.md` updated.

### `SPINE.9` — the scaffold updater guards project content (defect D17)

- [x] **ROOT CAUSE (WHY + WHERE)** — `scripts/update_scaffold.sh` at HEAD carried ONE list under the
  comment `23:# The project-NEUTRAL spine — safe to overwrite because it never carries project
  content.`, and four of its entries are exactly the files the template tells a project to fill in:
  `git show HEAD:scripts/update_scaffold.sh | grep -nE '^  (docs/TASK_TREE\.md|TOOLBOX\.md|README_POLICY\.md|docs/tasks/TEMPLATE\.md)$'`
  → `27:  TOOLBOX.md`, `28:  README_POLICY.md`, `31:  docs/TASK_TREE.md`, `33:  docs/tasks/TEMPLATE.md`,
  `count=4`, `rc=0`. `docs/TASK_TREE.md`'s own note instructs a generated project to replace the
  seeded row with its own trees, so one run of the documented "keep the spine current" command would
  have deleted the Active Task Trees index — layer-B navigation — with `git diff` as the only witness.
- [x] **ADDRESSED (verified)** — the updater now declares two classes with a recorded reason per
  guarded file (`awk '/^PROJECT_CONTENT=\(/,/^\)/' scripts/update_scaffold.sh | grep -cE '^\s+\S'` →
  `8`), refuses a dirty tree, and writes nothing under `--dry-run`. Before: no instrument
  existed (`ls docs/tasks/artifacts/scaffold_sync` → absent); after:
  `bash docs/tasks/artifacts/scaffold_sync/run_update_scaffold_probes.sh` →
  `probes: 7 pass / 0 fail`, including `ARM-2 ⭐ the task-tree index survived; SKIPPED reported; a
  backup of it exists`, `ARM-4` (whole-tree checksum unchanged by a dry run), `ARM-5`
  (`exit=1` refused, nothing written; `--allow-dirty` → `exit=0`) and `ARM-6` (`--force` overwrites
  but the backup still holds the project rows).
- [x] **NO REGRESSION** — `bash -n` clean on both changed scripts; the other suites are untouched and
  still green: `run_task_acceptance_probes.sh` → `probes: 10 pass / 0 fail`,
  `run_multileaf_shadowing_probe.sh` → `probes: 6 pass / 0 fail`,
  `run_fresh_evidence_probes.sh` → `probes: 9 pass / 0 fail`; `scripts/check_doctrines.sh` →
  `=== all doctrines green ===`, `exit=0`; `make check` → `test result: ok. 1 passed; 0 failed`.
- [x] **FIX** — rewrote `scripts/update_scaffold.sh` (NEUTRAL vs PROJECT-CONTENT classification with
  per-file reasons, backup-before-write under `target/scaffold_backup/`, `--dry-run`,
  `--force-project-sections`, `--allow-dirty`, dirty-tree refusal, scratch on the repository volume);
  added the 7-arm probe suite. Two probe defects were caught by its own arms before the leaf closed:
  a reused output variable that made ARM-5 assert against the wrong run, and `stat -f %m` measuring
  the GNU coreutils `stat` this machine's `PATH` shadows BSD userland with — the arm now pins
  `/usr/bin/stat` and fails loudly if neither form works.
- [x] **LOCKSTEP** — decision record `decision_scaffold-sync-protects-project-content.md` added and
  indexed (its cross-link from the acceptance-evidence record now resolves); `TOOLBOX.md` names both
  new instruments and the pinned-instrument rule; D17 marked fixed in `PLANNING.md`;
  `LIVE_STATUS.md`, `MEMORY.md`, `CHANGELOG.md` and the derived Knowledge Map updated.

### `SPINE.10` — one entry point for the probes, scratch on the repository volume (defect D16)

- [x] **ROOT CAUSE (WHY + WHERE)** — the inherited suites take their scratch from `mktemp -d`, which
  resolves through `TMPDIR` to the system volume: `mktemp -d` →
  `/var/folders/4h/29gg6nrx2pj9wfjkzc460hlr0000gn/T/tmp.F8p84psWRe` while the repository lives on
  `/Volumes/SSD`, and `grep -ln 'mktemp -d' docs/tasks/artifacts/*/*.sh scripts/*.sh` →
  `run_task_acceptance_probes.sh`, `run_waiver_routing_probes.sh`, `check_task_acceptance.sh`
  (3 files, `rc=0`). Each suite builds throwaway **git repositories** there, so the project's
  diagnostic work happened off-volume and its cleanup census could not see it.
- [x] **ADDRESSED (verified)** — before: no single entry point and no pin
  (`grep -c probes Makefile` at HEAD → `0`); after: `make probes` discovers every
  `run_*probe*.sh` under `docs/tasks/artifacts/` and runs it with
  `TMPDIR="$(CURDIR)/target/scratch"` → `make probes: 5 suite(s) green`, `exit=0`, with
  `9 pass / 0 fail`, `7 pass / 0 fail`, `6 pass / 0 fail`, `10 pass / 0 fail`, `5 pass / 0 fail`
  (37 arms). The pin is measured, not assumed:
  `TMPDIR="$PWD/target/scratch" mktemp -d` →
  `/Volumes/SSD/Documents/github/stitchcad/target/scratch/tmp.2n2Xkkjd4i`.
- [x] **NO REGRESSION** — no inherited suite was edited (`git diff --stat HEAD --
  docs/tasks/artifacts/task_acceptance/run_task_acceptance_probes.sh
  docs/tasks/artifacts/waiver_routing/run_waiver_routing_probes.sh` → empty, `rc=0`); `make check` →
  `test result: ok. 1 passed; 0 failed`; `scripts/check_doctrines.sh` →
  `=== all doctrines green ===`, `exit=0`; `git status --short` after the run shows only the intended
  edits, so nothing leaked into the tree.
- [x] **FIX** — `Makefile`: a `probes` target (find + pinned `TMPDIR`, per-suite pass-through of
  failure, a loud refusal when no suite is found) and a help line; `PROBE_TMPDIR` derived from
  `$(CURDIR)` so the repository stays relocatable. The residual (the shared check's own transient
  `mktemp -d`) is recorded in the leaf rather than patched into NEUTRAL files.
- [x] **LOCKSTEP** — `TOOLBOX.md` names `make probes` as the entry point and keeps the pinned-
  instrument rule; D16 marked fixed-with-residual in `PLANNING.md`; `LIVE_STATUS.md`, `MEMORY.md`,
  `CHANGELOG.md` updated.

### `SPINE.1` — the repository introduces itself as StitchCAD (defects D3, D4, D19)

- [x] **ROOT CAUSE (WHY + WHERE)** — the de-template renamed the crate but left every reader-facing
  surface describing the template: `git show HEAD:README.md | head -1` →
  `# bedrock — a Rust project discipline-spine template`, and
  `git show HEAD:README.md | grep -c StitchCAD` → `0`; `git show HEAD:docs/book/book.toml |
  grep -nE '^(title|authors)'` → `2:title = "Project Book"`, `4:authors = ["<your name>"]`;
  `git show HEAD:docs/book/src/SUMMARY.md` → one chapter. A third defect surfaced while verifying
  the documented commands: `make book` writes `docs/book/book/`, which
  `git show HEAD:.gitignore | grep -c 'docs/book/book'` → `0` did not ignore, so building the book
  left `?? docs/book/book/` in the tree and broke handoff-readiness (D19).
- [x] **ADDRESSED (verified)** — `head -1 README.md` →
  `# StitchCAD — a sewing CAD whose design never lives in a vendor file format`,
  `grep -c StitchCAD README.md` → `2`; `scripts/check_readme_stability.sh` →
  `README-STABILITY: OK — README.md is 103/300 lines, 6063/16384 bytes.`, `exit=0`, with
  `grep -cE '20[0-9]{2}-[0-9]{2}-[0-9]{2}' README.md` → `0` date-stamped lines; the book builds:
  `mdbook build docs/book` → `INFO HTML book written to …/docs/book/book`, `exit=0`; and after the
  `.gitignore` fix the same build leaves the tree clean (`git status --short` lists only the intended
  edits, no `??` entry).
- [x] **NO REGRESSION** — every command the README quick start promises was run, not assumed:
  `make check` → `test result: ok. 1 passed; 0 failed`; `make gate` → `=== all doctrines green ===`;
  `make probes` → `make probes: 5 suite(s) green`; `make book` → `exit=0`;
  `git config core.hooksPath` → `.githooks`.
- [x] **FIX** — rewrote `README.md` as a landing page (purpose, three defining properties, status,
  audience and non-goals, crate map, verified quick start, a canonical-home table that routes
  changing detail away, layout, license); set `docs/book/book.toml` identity and repository URL;
  replaced the template introduction with a real one (the idea, the two commitments, who the book is
  for, what is true now, how it is organised); added `docs/book/src/spec/index.md` — how to read a
  specification chapter, and the table of what each G0 chapter settles; pointed `SUMMARY.md` at the
  new `spec/` part; ignored `/docs/book/book/`.
- [x] **LOCKSTEP** — D3/D4 marked fixed and D19 logged with its reproduce command in `PLANNING.md`;
  `LIVE_STATUS.md`, `MEMORY.md`, `CHANGELOG.md` updated. The derived Knowledge Map regenerates in the
  hook; `knowledge-map/subsystems.md` rows are `SPINE.5`'s leaf, not this one.

### `SPINE.2` — first artifact cleanup and its cadence record (defect D8)

- [x] **ROOT CAUSE (WHY + WHERE)** — `ls docs/ARTIFACT_CLEANUP.md` → `No such file or directory`, so
  the 24-hour cadence had no record to read at startup and no way to be audited; meanwhile
  regenerable artifacts had accumulated: `du -sk target/doctrine_scratch target/scratch
  target/debug/incremental docs/book/book` → `188`, `0`, `896`, `1068` KB, and
  `find . -path ./.git -prune -o -name '*.bin' -print | wc -l` → `9` (all cargo incremental caches
  under `target/debug/incremental/`).
- [x] **ADDRESSED (verified)** — removed exactly those four paths and nothing else: `du -sh target` →
  `2.1M` before, `1.2M` after; `docs/book/book` → `1.1M` before, absent after. Residue census:
  `target/doctrine_scratch`, `target/scratch`, `docs/book/book`, `target/debug/incremental` all report
  `gone`, and the stray census (`find … -name '*.log' -o -name '*.bin' -o -name '.DS_Store'`) → `0`
  remaining. The record `docs/ARTIFACT_CLEANUP.md` now exists with one entry (the latest run), the
  safe / never-remove lists, and the instruction to read it at startup.
- [x] **NO REGRESSION** — nothing tracked was touched:
  `git ls-files | grep -cE '\.(log|bin|tmp)$|^target/|^docs/book/book/'` → `0` before and after, and
  `git status --short` after the cleanup showed no deletion. Every documented command was re-run
  against the cleaned tree: `make gate` → `=== all doctrines green ===`; `make check` →
  `test result: ok. 1 passed; 0 failed` (rebuilding the incremental caches it had just removed);
  `make book` → `INFO HTML book written to …`, `exit=0`; `make probes` → `5 suite(s) green`.
- [x] **FIX** — deleted four regenerable, gitignored paths on the repository volume; created
  `docs/ARTIFACT_CLEANUP.md` as the cadence record with the safe / never-remove policy so the next
  session does not re-derive it. Deliberately kept `target/debug/deps` (current build cache: cheap to
  keep, slow to rebuild) and said so in the record rather than leaving the choice implicit.
- [x] **LOCKSTEP** — D8 marked fixed in `PLANNING.md`; `LIVE_STATUS.md`, `MEMORY.md`, `CHANGELOG.md`
  updated; no book chapter changes (the cleanup touches no user-visible behavior).

### `SPINE.12` — the push cadence is a derived count, not a remembered one

- [x] **ROOT CAUSE (WHY + WHERE)** — the workflow document that ends every slice never mentioned
  pushing at all: `git show HEAD:COMMIT.md | grep -ciE 'push'` → `0` (grep exits `1` on no match), so
  the cadence lived only in the live conversation — the one place `MEMORY_ARCHITECTURE.md` says is not
  yet saved. A first draft of this box claimed `1` and was corrected by re-running the command before
  it shipped, which is the leg-1 discipline the same document demands. Measured state at the ruling:
  `git rev-list --count origin/main..HEAD` → `10`, i.e. ten commits of this project existed only on
  this machine.
- [x] **ADDRESSED (verified)** — `COMMIT.md` now carries a `## Push cadence` section:
  `grep -c 'Push cadence' COMMIT.md` → `1`, naming the 400-commit threshold, the deriving command
  (`git rev-list --count origin/main..HEAD`), the pre-push full gate, the "only when the director asks"
  exception, and the recorded trade-off against §8's crash-insurance argument. `grep -c '400' COMMIT.md`
  → `3` (threshold, deriving command, exception). The cadence itself did not trigger a push:
  `10 < 400`; the director then authorised a one-off first push for this project, recorded in the
  Commit Log below, after which the 400-commit cadence resumes. The push ran immediately after that
  commit (`f1dcbe4..051a075`, ahead-count `0`) and its CI verdict was appended to this leaf's
  Verification Log by a follow-up commit, because a verdict written before it is observed is a guess:
  both workflows completed `success` — `doctrines` in 11 s, `rust` in 18 s.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `scripts/check_doctrines.sh`
  reports 13 checks; `make check` → `test result: ok. 1 passed; 0 failed`. `COMMIT.md` is classified
  PROJECT-CONTENT by `scripts/update_scaffold.sh`, so a spine sync will back it up and skip it rather
  than silently revert this rule.
- [x] **FIX** — added the Push cadence section to `COMMIT.md`; added this leaf; recorded the ruling in
  `LIVE_STATUS.md`, `MEMORY.md` (as the standing next-push condition, not as history) and `CHANGELOG.md`.
- [x] **LOCKSTEP** — no code, no book chapter and no decision record: `COMMIT.md` is the canonical home
  for a commit-workflow rule, and duplicating it into a layer-C record would create a second authority.

### `SPINE.3` — the external policy references are now repository-owned (defects D11, D12)

- [x] **ROOT CAUSE (WHY + WHERE)** — the in-repo policy copy was the older body and the
  claim-verification standard had no in-repo copy at all:
  `git show HEAD:README_POLICY.md | wc -lc` → `71` lines / `2920` bytes with sections
  `Storage location | Content contract | Mechanical growth guard | Adoption checklist`, and
  `git show HEAD:README_POLICY.md | grep -cE '^## (Authority and provenance|Routing pressure closure)'`
  → `0`; `git ls-tree --name-only HEAD | grep -c CLAIM_VERIFICATION` → `0` and
  `git show HEAD:CLAUDE.md | grep -c CLAIM_VERIFICATION` → `0`, so no bootstrap route reached it.
- [x] **ADDRESSED (verified)** — `README_POLICY.md` is now `190` lines / `10535` bytes with `7`
  sections, the two revised ones present
  (`grep -cE '^## (Authority and provenance|Routing pressure closure)' README_POLICY.md` → `2`),
  behind a fenced StitchCAD adoption note; `CLAIM_VERIFICATION.md` is `330` lines / `21793` bytes,
  its neutral body verbatim (`grep -c '^## '` → `9` = 8 body sections + the note; the final line
  matches the source byte-for-byte). Donor leakage and locality checked:
  `grep -ciE 'fsmgen|0024|0038|0040|0041|0044|surfaces\.jsonl|routed_destinations' README_POLICY.md`
  → `0`; `grep -cE '/(Users|home|Volumes)/'` → `0` in both files. Discovery wired:
  `grep -c CLAIM_VERIFICATION CLAUDE.md` → `2`, `README.md` → `1`.
- [x] **NO REGRESSION** — `scripts/check_readme_stability.sh` →
  `README-STABILITY: OK — README.md is 103/300 lines, 6063/16384 bytes.`, `exit=0`; `make gate` →
  `=== all doctrines green ===` (after regenerating the derived Knowledge Map for the new decision
  record, which the pre-commit hook does anyway); `make check` → `test result: ok. 1 passed; 0 failed`.
- [x] **FIX** — copied the revised neutral body into `README_POLICY.md` under a fenced adoption note
  (authority, independence, reviewed measurement, current ceilings, routed destinations, adoption
  frontier); adopted `CLAIM_VERIFICATION.md` with a note that restates all three legs in this
  project's terms (conformance suites and goldens; independent `.rul` engines, real importers, a
  ruler on paper, blinded defective assemblies, a non-shipped SMT oracle; tracked producers under
  `docs/tasks/artifacts/` and `conformance/`) and maps §4's claim tag onto the enforced
  invocation + output + exit-status rule; added `decision_adopted-external-policy-references.md`;
  wired `CLAUDE.md` and the README navigation table.
- [x] **LOCKSTEP** — D11/D12 marked fixed and D20 logged in `PLANNING.md`; the adoption frontier is
  owned by named leaves (`SPINE.4` derived caps + destination registry, `SPINE.11` the tracked
  corpus); `LIVE_STATUS.md`, `MEMORY.md`, `CHANGELOG.md`, `docs/decisions/INDEX.md` and the derived
  Knowledge Map updated in this commit.

### `SPINE.11` — a published number gets its producer back (defect D20)

- [x] **ROOT CAUSE (WHY + WHERE)** — `SPINE.8`'s record publishes a measurement whose instrument was
  untracked scratch. The claim is in two tracked files at HEAD:
  `git show HEAD:docs/tasks/SPINE.md | grep -c '12 of 36'` → `1`, `rc=0`, and
  `git show HEAD:CHANGELOG.md | grep -c '12 of 36'` → `2`, `rc=0`; its producer is in neither:
  `git ls-tree -r --name-only HEAD -- docs/tasks/artifacts/evidence_signatures | wc -l` → `0`, `rc=0`,
  against `git ls-tree -r --name-only HEAD -- docs/tasks/artifacts | wc -l` → `5` tracked instruments,
  and `ls target/doctrine_scratch/evidence_corpus.txt` → `No such file or directory` (removed by the
  `SPINE.2` cleanup, which was correct to remove untracked scratch and had no way to know a published
  claim rested on it). That is a leg-3 breach of `CLAIM_VERIFICATION.md`: the number was re-derivable
  when written and is not now.
- [x] **ADDRESSED (verified)** — the corpus and its runner are tracked beside the other instruments
  (`git ls-files docs/tasks/artifacts/evidence_signatures/` → `evidence_corpus.txt`,
  `run_signature_portability_probe.sh`), and one command reproduces both published numbers:
  `bash docs/tasks/artifacts/evidence_signatures/run_signature_portability_probe.sh` →
  `corpus: 36 lines · grep-unmatched: 2 · awk-unmatched: 12`, `probes: 5 pass / 0 fail`, `exit=0`,
  with the per-line table naming the ten awk-only failures (the `\b` and `{n}` families) and the two
  bad samples. The constants are **watched**, not decorative: the runner refuses and prints the
  re-derive instruction if they drift, which is `CLAIM_VERIFICATION.md` §5B applied to our own claim.
- [x] **NO REGRESSION** — `make probes` → `make probes: 6 suite(s) green` (the new runner is picked
  up by the existing discovery glob, and the five earlier suites still report `9`, `7`, `6`, `10`,
  `5` pass / `0` fail); `bash -n` clean on the new script; `make gate` → `=== all doctrines green ===`;
  `make check` → `test result: ok. 1 passed; 0 failed`.
- [x] **FIX** — added `docs/tasks/artifacts/evidence_signatures/evidence_corpus.txt` (the 36 lines the
  claim is stated over) and `run_signature_portability_probe.sh` (census + 5 arms + watched constants
  + a fail-closed refusal if `DEFAULT_SIG` cannot be read out of the universal check). No tracked
  document's numbers needed changing: the probe confirms them.
- [x] **LOCKSTEP** — D20 marked fixed in `PLANNING.md`; `TOOLBOX.md` names the instrument and the
  question it answers; `CLAIM_VERIFICATION.md`'s adoption-frontier line for `SPINE.11` is now
  discharged; `LIVE_STATUS.md`, `MEMORY.md`, `CHANGELOG.md` updated.
- ⚠ **This commit was refused once, by this repository's own new doctrine, and the refusal was
  correct:** the first draft of the ROOT CAUSE box above cited `grep -rn` and two `ls` failures but no
  recognised result token, so `FRESH-ACCEPTANCE-EVIDENCE` reported `Missing box(es): - ROOT CAUSE
  (WHY + WHERE)`, `exit=1`. The fix was better evidence (counts re-derived against `HEAD` with their
  `rc=`), not a looser gate — the second time in this repository that the rule "citation + output +
  exit status" earned its keep.

### `SPINE.4.1` — the containment doctrine is repository-owned, with its gaps named

- [x] **ROOT CAUSE (WHY + WHERE)** — the repository enforced two size caps (`MEMORY.md` in
  `scripts/check_memory_architecture.sh`, `README.md` in `scripts/check_readme_stability.sh`) with no
  inventory, no lifecycle classes and no ceilings for the surfaces that grow every commit:
  `git ls-tree -r --name-only HEAD | grep -c 'LIVE_DOCUMENT_SIZE_CONTAINMENT'` → `0`, `rc=1`, while the
  policy adopted one leaf earlier requires a routing-destination registry. The pressure is measurable
  and current: `wc -lc ROADMAP.md CHANGELOG.md docs/tasks/SPINE.md` → `919`/`50821`, `460`/`36698`,
  `772`/`60433` — the last of those is this tree file, which is itself evidence that per-leaf evidence
  sections are a scaling term the inventory in `.4.2` must bound.
- [x] **ADDRESSED (verified)** — `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` now exists in-repo:
  `wc -lc LIVE_DOCUMENT_SIZE_CONTAINMENT.md` → `386` lines / `23712` bytes, `grep -c '^## '` → `12`
  sections (the donor's neutral body verbatim at 342 lines, plus a fenced StitchCAD note). The body is
  donor-free and local: `grep -ciE 'fsmgen|nexsim|\bisf\b|ppif'` → `0`,
  `grep -cE '/(Users|home|Volumes)/'` → `0`. The note states the milestones (warn at 80 %, inclusive
  ceiling), locality, the TSV data plane, the landing-page rule, the transition-debt policy, and the
  **deferred** neutral checker package with three named triggers.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0` (13 checks); the new
  file is outside the scaffold sync list, so a spine update cannot revert it:
  `grep -c 'LIVE_DOCUMENT_SIZE_CONTAINMENT' scripts/update_scaffold.sh` → `0`; `make check` →
  `test result: ok. 1 passed; 0 failed`.
- [x] **FIX** — copied the neutral body under a fenced adoption note; wired discovery from `COMMIT.md`
  (where an author updating live docs looks) and cross-linked it from `README_POLICY.md`'s adoption
  note; recorded the proportionality call as
  `docs/decisions/decision_live-document-containment-proportionate-adoption.md`; split `SPINE.4` into
  `.4.1`–`.4.3` so choosing the ceilings and enforcing them are separately verifiable.
- [x] **LOCKSTEP** — `PLANNING.md` (D13 now owned by the three sub-leaves), `LIVE_STATUS.md`,
  `MEMORY.md`, `CHANGELOG.md`, `docs/decisions/INDEX.md` and the derived Knowledge Map updated in this
  commit. `.4.2` owes the registry and the derived README caps; `.4.3` owes the checker.

### `SPINE.4.2` — the containment data plane, and the trims that make its numbers honest

- [x] **ROOT CAUSE (WHY + WHERE)** — the repository enforced two caps and inventoried nothing, so the
  surfaces that grow every commit had no lifecycle, no owner and no ceiling:
  `git ls-tree -r --name-only HEAD | grep -c 'live_document_size'` → `0`, `rc=1`. Measured pressure at
  adoption, on three axes rather than two: `MEMORY.md` `38`/`2633`/maxline `101` (carrying a priority
  queue and a defect roster that layer B already owns), `LIVE_STATUS.md` maxline `1104`,
  `docs/tasks/PLANNING.md` maxline `1758` — a five-column table row, i.e. pressure invisible to a
  line-and-byte cap, which is why the doctrine makes max-content-line a separate axis.
- [x] **ADDRESSED (verified)** — `.doctrine/live_document_size/surfaces.tsv` now classifies **17**
  surfaces (`grep -vcE '^(#|$)' …/surfaces.tsv` → `17`, every row `21` tab-separated fields) and
  `routes.tsv` classifies **15** routes (`8` fields each), with route→surface closure proved by
  enumerating every route's `surface_id` against the registry (no unclassified sink). The three
  pathological surfaces were trimmed BEFORE targets were set, so no target was fitted to bloat:
  `MEMORY.md` → `28`/`1780`/`103`; `LIVE_STATUS.md` maxline → `146`; `PLANNING.md` maxline → `255`
  (census converted from a wide table to 21 bounded entries). Derived README caps recorded:
  **160 lines / 9 216 bytes / 320-byte line** (health 120/7 168), replacing the inherited 300/16 384
  defaults as the binding limit.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make check` →
  `test result: ok. 1 passed; 0 failed`; `scripts/check_memory_architecture.sh` → `exit=0` after the
  pointer trim (28 lines / 1 780 bytes against caps of 50 / 7 168); `make probes` →
  `6 suite(s) green`; the data plane carries no absolute or off-volume path
  (`grep -cE '/(Users|home|Volumes|private)/' .doctrine/live_document_size/*.tsv` → `0`).
- [x] **FIX** — added the two bounded TSV registries; trimmed `MEMORY.md` back to the pointer shape
  (the execution order and defect census are layer B and are named, not restated); rewrote
  `LIVE_STATUS.md` notes cells as short pointers; converted the `PLANNING.md` defect census to bounded
  entries; created leaves `SPINE.13` (roadmap navigation), `SPINE.14` (seal the inherited changelog
  segment into `docs/history/`), `SPINE.15` (settle D22 with a real renderer + the wide-row
  convention); recorded the derived caps in `README_POLICY.md`'s adoption note.
- [x] **LOCKSTEP** — D21 and D22 logged in `PLANNING.md`; `LIVE_STATUS.md`, `MEMORY.md`,
  `CHANGELOG.md` updated. Enforcement is `SPINE.4.3`: until it lands these rows are **declared, not
  gated**, and this leaf says so rather than implying a checker exists.
- ⚠ **Two defects in this leaf's own census, caught by running it:** the first pass reported a missing
  `memory_pointer` row and a missing `R01` route, because the command stripped `^#` lines and then
  `tail -n +2` — and the header itself starts with `#`, so the first data row was eaten twice. The
  instrument was wrong, not the registry. The second was real: the `git_history` row carried `20`
  fields instead of `21`. Both fixed, and both are exactly what `SPINE.4.3`'s checker must refuse
  mechanically instead of by luck.

### `SPINE.14` — the changelog becomes a ledger with an archive terminal

- [x] **ROOT CAUSE (WHY + WHERE)** — `CHANGELOG.md` was a rolling ledger carrying another project's
  frozen history in its live window: `git show HEAD:CHANGELOG.md | grep -n '^# Inherited spine history'`
  → line `365` of `522`, i.e. a `158`-line / `11 811`-byte segment that can never be trimmed occupied
  30 % of the window and 30 % of the bytes (`522`/`42124` measured). The containment checker proved the
  consequence before it was wired into the gate: `LIVE-DOC-SIZE: changelog: transition debt WIDENED on
  lines (522 > baseline 487)`, `exit=1` — a debt baseline declared while the surface was still growing
  is a baseline that breaks on the next slice.
- [x] **ADDRESSED (verified)** — the segment is sealed into
  `docs/history/bedrock-scaffold-changelog.md` (`174` lines / `12 794` bytes including its identity
  header) and replaced by a pointer. Losslessness is proved by hash, not asserted: the sealed segment
  and `git show HEAD:CHANGELOG.md`'s segment are both
  `sha256:78f43e0fe24c60f7bb8b0bb159a2751cc37f967659111bd81df7d74b22dbeca7` → `BYTE-IDENTICAL: True`.
  The ledger is now `371` lines / `30 713` bytes / widest line `118`, inside its health target
  (400 / 32 768), and the debt row is cleared. Retrieval: `grep -c '^## bedrock-scaffold'
  docs/history/bedrock-scaffold-changelog.md` → `6` entries, while `grep -c '^## bedrock-scaffold'
  CHANGELOG.md` → `0` and `grep -c '^## STITCHCAD' CHANGELOG.md` → `15` (ours stayed, theirs moved).
- [x] **NO REGRESSION** — `bash scripts/check_live_doc_size.sh` →
  `live-doc-size: OK — 17 surfaces, 15 routes, 41 files measured`, `exit=0` (the `history_archive`
  collection now matches its sealed file); `make gate` → `=== all doctrines green ===`;
  `make check` → `test result: ok. 1 passed; 0 failed`.
- [x] **FIX** — created `docs/history/` as the archive terminal with the sealed segment carrying its
  own identity (lines, bytes, sha256), provenance, retrieval path and a no-write policy; left a pointer
  in `CHANGELOG.md`; updated the `changelog` and `history_archive` registry rows (measurements, debt
  cleared, sealed identity recorded in the row notes).
- [x] **LOCKSTEP** — the seal is recorded in this leaf, the registry, `LIVE_STATUS.md`, `MEMORY.md` and
  `CHANGELOG.md`'s own pointer; the rollover rule for our own entries (seal the oldest when the window
  passes its health target) is stated in the `changelog` registry row so the next author does not have
  to re-derive it.

Leaves `.4.3`, `.5`, `.13` and `.15` each add their own `### <leaf-id>` subsection here, in the same
commit as their work; this file carries no unticked placeholder boxes (the reason is D15).

### `SPINE.7` — measure and publish defect D15 (multi-leaf acceptance-evidence shadowing)

- [x] **ROOT CAUSE (WHY + WHERE)** — `scripts/check_task_acceptance.sh:106-112`: the extractor runs
  `BEGIN{ inbox=0 }` … `if (inbox) exit` … `if (match(tolower(line), kw)) { inbox=1; print; next }`,
  i.e. it collects the FIRST bullet whose text matches the label and stops at the next box, so a
  file holding several leaves is judged on whichever section comes first — not on the leaf that owns
  the staged change. Measured by the new probe over the shipped check:
  `bash docs/tasks/artifacts/task_acceptance/run_multileaf_shadowing_probe.sh` →
  `✓ HOLE-1  exit=0  ⚠ D15 false GREEN: leaf 2's change accepted on leaf 1's evidence`,
  `✓ HOLE-2  exit=1  ⚠ D15 false RED: an honest, evidenced leaf refused by a placeholder above it`,
  `✓ HOLE-3  exit=0  the same leaf without the placeholder → accepted`, `probes: 6 pass / 0 fail`.
- [x] **ADDRESSED (verified)** — before: `git ls-tree --name-only HEAD docs/tasks/artifacts/task_acceptance/`
  → one suite (`run_task_acceptance_probes.sh`), whose cross-leaf control (`CTRL-2`) stages the
  other leaf in a *separate file*, so the multi-leaf case had no instrument; after: the same command
  lists two suites and the new one prints `probes: 6 pass / 0 fail` with three controls
  (`CTRL-1/2/3` → `exit=0`, `exit=1`, `exit=1`) and three defect arms. The authoring convention that
  keeps verdicts attributable meanwhile is now a retrievable layer-C record:
  `docs/decisions/decision_acceptance-evidence-per-leaf.md` (`answers:` line present, indexed).
- [x] **NO REGRESSION** — the inherited suite is unchanged and still discriminates:
  `TMPDIR="$PWD/target/scratch" bash docs/tasks/artifacts/task_acceptance/run_task_acceptance_probes.sh`
  → `probes: 10 pass / 0 fail`; `bash -n` on the new probe → clean; `make check` →
  `test result: ok. 1 passed; 0 failed`; `scripts/check_doctrines.sh` → `=== all doctrines green ===`,
  `rc=0`.
- [x] **FIX** — added the probe (a first-class diagnostic tool, kept in the repo); recorded the
  convention and its measurement in a decision record; promoted the lesson in `DEV_NOTES.md`;
  registered the probe in `TOOLBOX.md`. The shared check itself is deliberately untouched.
- [x] **LOCKSTEP** — `PLANNING.md` carries D15–D17 with reproduce commands and owners; this leaf,
  `TOOLBOX.md`, `DEV_NOTES.md`, `docs/decisions/INDEX.md`, `LIVE_STATUS.md`, `MEMORY.md`,
  `CHANGELOG.md` and the derived Knowledge Map are updated in this commit.
- ⚠ **Honest note on this very commit:** staging a `.sh` file makes this a CODE change, so the
  inherited gate ran and — per D15 — read `### SPINE.6`'s boxes (the first in this file), not the
  ones above. The boxes for THIS leaf are fresh in this commit's diff (3 added ticked boxes:
  `git diff --cached -U0 -- docs/tasks/SPINE.md | grep -cE '^\+- \[x\] \*\*(ROOT CAUSE|ADDRESSED|NO REGRESSION)'`
  → `3`); `SPINE.8` makes that property mechanical instead of a note.
- ⚠ **This commit was REFUSED once, and the refusal was D15's third facet.** With the probe staged,
  the inherited check judged every staged leaf file, including `docs/tasks/PLANNING.md`, whose
  `PLANNING.1` census bullets cite two commands and their real output but no recognized signature
  token: `scripts/check_doctrines.sh` → `❌ TASK-ACCEPTANCE … docs/tasks/PLANNING.md — the
  'ROOT CAUSE' box is ticked but carries no tool-output evidence`, `exit=1`. Diagnosis
  (tools-first): the box the gate read is at `docs/tasks/PLANNING.md:155`, and counting signature
  families inside that bullet gives `0`. Fix: the two bullets now carry their counts and `rc=0`,
  and the convention (invocation + output + exit status) is recorded for `SPINE.8` to publish.
  Nothing was waived and no gate was weakened.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-29` | tree seeded | `scripts/check_doctrines.sh` | `=== all doctrines green ===`, `rc=0` |
| `2026-09-29` | `SPINE.6` | `make check`; `git ls-files --error-unmatch Cargo.lock`; `git status --short` | `test result: ok. 1 passed`; `rc=0`; empty status |
| `2026-09-29` | `SPINE.7` | `run_multileaf_shadowing_probe.sh`; `run_task_acceptance_probes.sh`; `bash -n`; `make check`; `scripts/check_doctrines.sh` | `probes: 6 pass / 0 fail`; `probes: 10 pass / 0 fail`; syntax clean; `test result: ok. 1 passed`; first attempt `exit=1` (D15 facet 3, diagnosed and fixed), then `rc=0` |
| `2026-09-29` | `SPINE.8` | `run_fresh_evidence_probes.sh`; `check_fresh_acceptance_evidence.sh --self-test`; both inherited suites; `make check`; `scripts/check_doctrines.sh` | `probes: 9 pass / 0 fail`; `8 verdict controls + 6 extractor arms`; `probes: 10 pass / 0 fail`; `probes: 6 pass / 0 fail`; `test result: ok. 1 passed`; `=== all doctrines green ===`, `exit=0` |
| `2026-09-29` | `SPINE.9` | `run_update_scaffold_probes.sh`; `bash -n` ×2; the three other probe suites; `make check`; `scripts/check_doctrines.sh` | `probes: 7 pass / 0 fail`; syntax clean; `10/0`, `6/0`, `9/0`; `test result: ok. 1 passed`; `=== all doctrines green ===`, `exit=0` |
| `2026-09-29` | `SPINE.10` | `make probes`; `TMPDIR=… mktemp -d`; `git diff --stat` on the inherited suites; `make check`; `scripts/check_doctrines.sh` | `5 suite(s) green` (37 arms, 0 fail); repo-volume path; no inherited edit; `test result: ok. 1 passed`; `=== all doctrines green ===`, `exit=0` |
| `2026-09-29` | `SPINE.1` | `check_readme_stability.sh`; `mdbook build docs/book`; `make check`; `make gate`; `make probes`; `git status --short` after the build | `README-STABILITY: OK — 103/300 lines, 6063/16384 bytes`; `exit=0`; `test result: ok. 1 passed`; `=== all doctrines green ===`; `5 suite(s) green`; no `??` entry |
| `2026-09-29` | `SPINE.12` | `git rev-list --count origin/main..HEAD`; `grep -c 'Push cadence' COMMIT.md`; `make gate`; `make check` | `10` (below the 400 threshold); `1`; `=== all doctrines green ===`; `test result: ok. 1 passed` |
| `2026-09-29` | `SPINE.12` (push addendum) | `git push origin main`; `gh run list`; `curl …/actions/runs?head_sha=051a075…` | `f1dcbe4..051a075  main -> main`, ahead `0`; `doctrines` completed **success** 11 s (run 36622373461), `rust` completed **success** 18 s (run 36622373539); API `total_count: 2`, both `conclusion=success` |
| `2026-09-29` | `SPINE.2` | before/after `du`; residue + stray censuses; `git ls-files` artifact census; `make gate`, `make check`, `make book`, `make probes` | `target` 2.1M → 1.2M; 4 paths `gone`; `0` strays; `0` tracked artifacts; all four targets green |
| `2026-09-29` | `SPINE.3` | `wc -lc` on both adopted copies; donor-token and absolute-path greps; `check_readme_stability.sh`; `make gate`; `make check` | policy `190`/`10535`, claim standard `330`/`21793`; `0` donor tokens, `0` absolute paths; `README-STABILITY: OK`; `=== all doctrines green ===`; `test result: ok. 1 passed` |
| `2026-09-29` | `SPINE.11` | `run_signature_portability_probe.sh`; `make probes`; `bash -n`; `make gate`; `make check` | `corpus: 36 lines · grep-unmatched: 2 · awk-unmatched: 12`, `probes: 5 pass / 0 fail`; `6 suite(s) green`; syntax clean; `=== all doctrines green ===`; `test result: ok. 1 passed` |
| `2026-09-29` | `SPINE.4.1` | `wc -lc` and section count on the adopted doctrine; donor-noun and absolute-path greps; `make gate`; `make check` | `386` lines / `23712` bytes, `12` sections; `0` donor nouns, `0` absolute paths; `=== all doctrines green ===`; `test result: ok. 1 passed` |
| `2026-09-29` | `SPINE.4.2` | registry censuses (row + field counts, route→surface closure, path census); `wc -lc` and per-file maxline before/after the trims; `make gate`; `make check`; `make probes` | `17` surfaces × `21` fields, `15` routes × `8` fields, closure holds, `0` absolute paths; maxline `1758`→`255`, `1104`→`146`; `=== all doctrines green ===`; `test result: ok. 1 passed`; `6 suite(s) green` |
| `2026-09-29` | `SPINE.14` | sha256 of the sealed segment vs `git show HEAD:CHANGELOG.md`; `wc -lc` before/after; entry censuses; `check_live_doc_size.sh`; `make gate`; `make check` | `BYTE-IDENTICAL: True`, `sha256:78f43e0f…`; ledger `522`/`42124` → `371`/`30713`; `6` sealed vs `0` in the ledger and `15` of ours; `OK — 17 surfaces, 15 routes, 41 files`; both gates green |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| tree seed | `STITCHCAD-PLANNING-0001 (leaf PLANNING.1)` | created by the seeding leaf |
| `SPINE.6` | `STITCHCAD-SPINE-0006 (leaf SPINE.6): track the workspace lockfile` | fixes D14 |
| `SPINE.7`, `SPINE.8` | `STITCHCAD-PLANNING-0002 (leaf PLANNING.2)` | leaves created to own defect D15 |
| `SPINE.7` | `STITCHCAD-SPINE-0007 (leaf SPINE.7): measure the multi-leaf acceptance-evidence hole` | probe + convention record; D15 published |
| `SPINE.8` | `STITCHCAD-SPINE-0008 (leaf SPINE.8): require acceptance evidence fresh in the commit` | project doctrine + 9-arm probe; D15 facet 1 closed locally |
| `SPINE.9` | `STITCHCAD-SPINE-0009 (leaf SPINE.9): guard project content in the scaffold updater` | D17 fixed; 7-arm probe suite + decision record |
| `SPINE.10` | `STITCHCAD-SPINE-0010 (leaf SPINE.10): one probe entry point, scratch on this volume` | D16 fixed (residual recorded) |
| `SPINE.1` | `STITCHCAD-SPINE-0001 (leaf SPINE.1): the repository introduces itself as StitchCAD` | D3, D4, D19 fixed |
| `SPINE.2` | `STITCHCAD-SPINE-0002 (leaf SPINE.2): first artifact cleanup and its cadence record` | D8 fixed |
| `SPINE.12` | `STITCHCAD-SPINE-0012 (leaf SPINE.12): record the 400-commit push cadence` | director's ruling, in `COMMIT.md`; one-off first push made (`f1dcbe4..051a075`), both CI workflows green |
| `SPINE.12` addendum | `STITCHCAD-SPINE-0012a (leaf SPINE.12): record the observed CI verdict` | push confirmed, cadence resumes at 400 |
| `SPINE.3` | `STITCHCAD-SPINE-0003 (leaf SPINE.3): adopt the external policy references in-repo` | D11, D12 fixed; D20 found by the adoption sweep |
| `SPINE.11` | `STITCHCAD-SPINE-0011 (leaf SPINE.11): give the published signature measurement a tracked producer` | D20 fixed; watched constants |
| `SPINE.4.1` | `STITCHCAD-SPINE-0004 (leaf SPINE.4.1): adopt the live-document containment doctrine` | partial adoption; deferrals named with triggers |
| `SPINE.4.2` | `STITCHCAD-SPINE-0004b (leaf SPINE.4.2): the containment data plane` | 17 surfaces, 15 routes, derived README caps, 3 surfaces trimmed |
| `SPINE.14` | `STITCHCAD-SPINE-0014 (leaf SPINE.14): seal the inherited changelog into docs/history/` | hash-proven, lossless; ledger inside its window |
| `SPINE.4.3`, `SPINE.5`, `SPINE.13`, `SPINE.14`, `SPINE.15` | `pending` | — |

## Changelog

- `2026-09-29`: Tree created by `PLANNING.1`; owns startup defects D3, D4, D7–D9, D11–D13.
- `2026-09-29`: `SPINE.6` added and landed out of order — defect D14 (untracked `Cargo.lock`).
- `2026-09-29`: `SPINE.7`/`SPINE.8` added to own defect D15 (multi-leaf acceptance-evidence
  shadowing in the inherited gate); routing evidence recorded; frontier re-ordered so the
  mitigation lands before the first code leaf (`G0-CONTRACT.18`).
- `2026-09-29`: `SPINE.7` landed — probe committed (6 arms, 3 controls), convention promoted to a
  layer-C record, lesson promoted in `DEV_NOTES.md`, `TOOLBOX.md` rows seeded.
- `2026-09-29`: `SPINE.9`/`SPINE.10` added for defects D17 (scaffold updater treats
  project-content files as neutral) and D16 (inherited probes scratch off-volume).
- `2026-09-29`: `SPINE.14` landed — the inherited bedrock changelog segment is sealed into
  `docs/history/` with hash-proven identity; `CHANGELOG.md` is a bounded ledger inside its window and
  its transition debt is cleared.
- `2026-09-29`: `SPINE.4.2` landed — the containment data plane (17 surfaces, 15 routes, derived
  README caps), three surfaces trimmed before their targets were set, and three new leaves owning the
  remaining debt (`SPINE.13` roadmap navigation, `SPINE.14` changelog archive, `SPINE.15` D22).
- `2026-09-29`: `SPINE.4` split into `.4.1`–`.4.3`; `.4.1` landed — the containment doctrine adopted
  in-repo behind a fenced local note that names the deferred neutral checker package and the three
  triggers that reopen it.
- `2026-09-29`: `SPINE.11` landed — the corpus and runner behind the published `12 of 36` /
  `2 of 36` measurement are tracked and watched, so the claim is re-derivable by one command.
  D20 closed; `make probes` now runs 6 suites.
- `2026-09-29`: `SPINE.3` landed — `README_POLICY.md` refreshed to the revised neutral body behind a
  fenced adoption note, `CLAIM_VERIFICATION.md` adopted with all three legs restated in this
  project's terms, and the adoption rule recorded in layer C. D11/D12 closed; the adoption's own
  sweep found D20 (a published number whose producer was untracked scratch) → `SPINE.11`.
- `2026-09-29`: `SPINE.12` addendum — the authorised first push was made and observed green in CI
  (`doctrines` 11 s, `rust` 18 s, both `conclusion=success`); the 400-commit cadence is now in force.
- `2026-09-29`: `SPINE.12` landed — the director's push cadence (400 commits between pushes) recorded
  in `COMMIT.md` with the deriving command and the trade-off against `MEMORY_ARCHITECTURE.md` §8.
- `2026-09-29`: `SPINE.2` landed — first artifact cleanup (four regenerable paths, `target`
  2.1 MB → 1.2 MB) and the cadence record `docs/ARTIFACT_CLEANUP.md`. D8 closed.
- `2026-09-29`: `SPINE.1` landed — README, mdBook identity, the introduction and the `spec/` part;
  `make book` output ignored (D19). D3 and D4 closed.
- `2026-09-29`: `SPINE.10` landed — `make probes` runs all five suites with `TMPDIR` pinned under
  `target/scratch`; D16 closed with one accepted residual (the shared check's trap-cleaned
  `mktemp -d`), recorded rather than patched into NEUTRAL files.
- `2026-09-29`: `SPINE.9` landed — the scaffold updater classifies files (NEUTRAL synced,
  PROJECT-CONTENT backed up and skipped), refuses a dirty tree, and writes nothing on `--dry-run`;
  7-arm probe suite and a layer-C record. D17 closed.
- `2026-09-29`: `SPINE.8` landed — `FRESH-ACCEPTANCE-EVIDENCE` registered in the project slot with
  a 9-arm probe suite; D15 facet 1 is now closed mechanically. A candidate defect (D18, "the
  inherited signature list is not portable") was measured and **withdrawn**: the gate matches with
  `grep -qE`, where both GNU and BSD grep handle the families; only the first implementation of our
  own check, which used awk, was broken.
