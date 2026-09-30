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
  Status: `done`
  Goal: the deterministic checker `scripts/check_live_doc_size.sh` registered in the project slot —
  fails on an unclassified surface, a missing owner/lifecycle/ceiling, an absolute or off-volume path
  in the data plane, a line/byte/max-line overflow past a ceiling, a ceiling raised without a recorded
  authority, a route to an uncontrolled destination, and a stale derived projection; warns at 80 % of a
  health target. Plus a probe suite with a RED arm per refusal class.
  Acceptance: `make gate` runs it unconditionally (not staged-scoped); every refusal class has an arm
  seen to fire; the healthy tree passes; `TOOLBOX.md` names it; D13 closes.
  Verification: recorded below — `--self-test` 11 arms (1 GREEN, 10 RED), probe suite `4 pass / 0 fail`,
  real tree `OK — 17 surfaces, 15 routes, 41 files measured`, `exit=0`.
  Commit: `STITCHCAD-SPINE-0004c`

- ID: `SPINE.4.4`
  Status: `done`
  Goal: re-derive the `book_collection` **maxline health** for table-shaped reference parts. Measured at
  the ruling: the glossary's widest entry row is `272` B and the feature matrix's `197` B against a `200` B
  health and a `320` B ceiling, so a permanent warning has no defect behind it — the health target was
  derived from the shape of a prose chapter, and a five-column termbase row is a different shape.
  Acceptance: the derivation is written where the registry's other derivations live (the `notes` column of
  the `book_collection` row, or a split row if the checker's glob semantics allow one), citing the measured
  cell budget rather than a round number; the ceiling is unchanged unless a decision record authorises it;
  the check stays green and the probe suite still discriminates; if `.doctrine/` is touched, the
  immediate-push exception is honoured and the observed CI verdict is recorded here.
  Verification: recorded below and in the acceptance checklist — the cell-budget census derives `275` B from
  the termbase shape (`259` B of p95 cells + `16` B of separator) over `36` shapes and `625` data rows, its
  `--self-test` is `7 pass / 0 fail`, the registry carries `275`/`440` with the derivation in its notes, a
  decision record authorises the ceiling, and the split-row option is measured and deferred rather than
  assumed. `.doctrine/` changed, so the immediate push is owed and the CI verdict is recorded below.
  Commit: `STITCHCAD-SPINE-0004d`

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

- ID: `SPINE.16`
  Status: `done`
  Goal: declare `.doctrine/code_paths.txt` so the acceptance gates classify **this** repository's files
  correctly (defect **D25**). The spine's default code-path regex matches any path containing `/src/`,
  and the mdBook chapters live in `docs/book/src/` — so writing a specification chapter was judged a
  CODE change and demanded ticked, tool-output-backed acceptance boxes for prose.
  Acceptance: a specification chapter is no longer classified as code; every genuinely
  behaviour-altering path still is; both acceptance checks read the same seam; the probe suites still
  discriminate; `make gate` green.
  Verification: recorded below — classification census over 7 representative paths, 7 probe suites green.
  Commit: `STITCHCAD-SPINE-0016`

- ID: `SPINE.17`
  Status: `done`
  Goal: make the approved push-cadence exception mechanical — an unpushed commit that touches CI, a
  doctrine check, the `.doctrine/` seams or the hooks owes a push immediately, because layer E4 is the
  un-bypassable backstop and such a change is unverified until a runner executes it.
  Acceptance: `COMMIT.md` states the exception and the deriving command; `make push-due` reports the
  state and exits 1 when a push is owed; the "owed" arm is demonstrated against a revision known to
  contain CI/doctrine changes; the refusal arm (no upstream / bogus base) is demonstrated.
  Verification: recorded below — three arms observed, including the one that caught a defect in the
  trigger list.
  Commit: `STITCHCAD-SPINE-0017`

- ID: `SPINE.18`
  Status: `done`
  Goal: make `SPINE.17`'s trigger set mean what the rule means — "a runner must re-verify this" — by
  deriving the registered doctrine checks from the two registries instead of globbing filenames.
  Acceptance: an unregistered helper no longer reports a false exceptional push; a change to a
  registered check, a workflow, a `.doctrine/` seam or a hook still does; all three arms re-observed.
  Verification: recorded below — three arms re-run against the derived set.
  Commit: `STITCHCAD-SPINE-0018`

- ID: `SPINE.19`
  Status: `pending` (created by `G0-CONTRACT.4b`, which performed the first non-changelog rollover and
  found the archive verifier hardcoded to one ledger — defect **D40**; extended by **D46**, the defect census
  becoming `PLANNING.md`'s dominant mass; deferred behind product work by
  `decision_product-work-takes-the-frontier.md`)
  Goal: make the sealed-archive verifier ledger-agnostic. `docs/history/` now holds two ledgers'
  segments — the changelog's four and the dev notes' one — and only the changelog's have their
  **Coverage** and **pointer** claims checked: `DESCRIPTOR` (digest + declared line count) already runs
  over every segment, while `COVERAGE` reads work-unit ids and `POINTER` compares
  `stitchcad-changelog-part*.md` against `CHANGELOG.md`.
  Acceptance: a segment registry declares, per ledger, its live file, its segment glob and the id shape
  its coverage line carries; `COVERAGE` and `POINTER` are derived from it, so adding a third ledger is a
  registry row and not a code change; a RED arm renames a lesson inside the dev-notes segment's coverage
  list and is caught, and a RED arm deletes `DEV_NOTES.md`'s pointer row and is caught; the existing
  `DESCRIPTOR` generalisation and its arms stay green; **and the third ledger exists** — `PLANNING.md`'s closed
  defects sealed into `docs/history/stitchcad-defects-part1.md` under the same descriptor contract, with the
  live census keeping the open ones and a pointer, so that tree is inside its health target again (D46);
  `TOOLBOX.md`, D40 and D46 updated in the same commit.
  Verification: `pending`
  Commit: `pending`

- ID: `SPINE.4.5`
  Status: `done`
  Goal: make a transition-debt baseline **revision-aware**, so a legitimate revision of a baselined document
  can land without hand-widening the number the doctrine forbids widening. The `roadmap` row's baseline was
  measured at exactly the file's size, so roadmap v0.3 could only be applied by editing the baseline — the
  silent widening the rule exists to prevent. This is the containment adoption note's deferred **trigger 3**
  ("a stored copy of a mechanically owned value needs an executed freshness oracle"), fired by the first
  roadmap amendment and owned by whoever hit it.
  Acceptance: the debt column accepts an `at=<revision>` axis; the checker refuses a row whose revision token
  the file's first line no longer declares; declaring the right revision does NOT license growth past the
  baseline; `at=` on a non-`file` surface is refused rather than ignored; the roadmap's baseline is re-based
  in the same commit as the revision, with the authority cited in the row's notes; the refusal classes fire in
  `--self-test` and against the real registry in the probe suite; a decision record carries the rule.
  Verification: recorded below and in the acceptance checklist — `--self-test` grew `11` → `15` arms
  (`GREEN-AT`, `RED-AT-STALE`, `RED-AT-WIDEN`, `RED-AT-KIND`), the probe suite `4` → `5` arms with `REAL-3`
  staling the real registry to `at=v0.2`, and the roadmap re-based to `lines=947;bytes=52818;at=v0.3`.
  Commit: `STITCHCAD-G0-0004c` (landed in the slice that applied roadmap v0.3, which is the slice that hit the
  trigger)

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
| 8c | `SPINE.4.3` | `done` | declared ceilings are enforced now; D13 closed |
| 8d | `SPINE.4.4` | `done` | the maxline target is derived from the binding table shape's cell budget (275 B), not from prose; the ceiling rises to 440 B by record, and D42's split is performed here |
| 8e | `SPINE.4.5` | `done` | taken inside `G0-CONTRACT.4c`: a debt baseline names the revision it was measured at, and a stale one is refused — containment trigger 3, fired by the first roadmap amendment and discharged locally |
| 9 | `SPINE.5` | `pending` | toolbox rows are honest only once the instruments are in use |
| 10 | `SPINE.13` | `pending` | roadmap navigation + per-section bounds: the `maintained_reference` debt |
| 11 | `SPINE.14` | `done` | taken before `.4.3`: the ledger had to be inside its window before a baseline could be declared honestly |
| 12 | `SPINE.15` | `pending` | settle D22 against a real renderer and adopt the wide-row convention |
| — | `SPINE.17` | `done` | taken out of order: a director-approved rule is recorded when it is made |
| — | `SPINE.18` | `done` | taken immediately after: `.17` shipped a trigger that fired on itself |
| — | `SPINE.19` | `pending` | the archive verifier is ledger-agnostic (D40, found by the first non-changelog rollover). Deferred behind product work: the digest leg already covers every segment, so silent content drift is caught and only the coverage and pointer claims are not |

## Decisions

- `2026-09-30`, leaf `.4.4`: a **maxline health target is a cell budget**, not a prose width and not today's
  widest line: the binding table shape's per-column p95 cells summed, plus `3 × columns + 1` bytes of GFM
  separator. `book_collection` is `275` B on that derivation (`259 + 16`), and its ceiling is `440` B —
  `1.6 ×` health, the ratio every other registry row uses, and above the shape's own worst legitimate row
  (`379` B), which the old `320` B ceiling would have refused. A ceiling rises only by a recorded authority:
  `docs/decisions/decision_maxline-health-derived-from-the-cell-budget.md`, which also records the two
  rejected alternatives with their arithmetic (raising health to silence the warning, and splitting the
  collection into prose and table rows). The producer of every number is
  `docs/tasks/artifacts/live_doc_size/run_cell_budget_census.sh`.
- `2026-09-30`, leaf `.4.4`: a **persistent warning is either a defect or a true statement, and the registry
  says which**. After the re-derivation the book still warns at `272 B = 99% of its 275 B target`; that is
  now true — the termbase is at its budget — and the remedy that would clear it is a table-authoring
  convention, which is `SPINE.15`'s, not a bigger number. No health target at or below the old `320` B
  ceiling could have silenced a `272` B row (it would need `> 340` B), which is a structural property of a
  maximum axis and is recorded so the next reader does not re-derive it as a surprise.

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

Completed leaves' checklists live in [`SPINE-evidence.md`](SPINE-evidence.md), split out at `SPINE.4.4`
(defect D42) under the containment registry's remedy for a tree past 1000 lines. The leaf being landed keeps
its checklist here, because `scripts/check_task_acceptance.sh` judges every staged `docs/tasks/*.md` and
refuses one with no ticked boxes; the next slice moves it across. Neither file carries an unticked
placeholder box (defect D15).

### `SPINE.4.5` — a debt baseline knows which revision it was measured at

- [x] **REPRODUCE / ISSUE** — roadmap v0.3 could not be applied honestly. `wc -lc ROADMAP.md` → `919 50821`
  and the registry's `roadmap` debt read `lines=919;bytes=50821`, so the amendment's `+28` lines / `+1 997` B
  hit `LIVE-DOC-SIZE: roadmap: transition debt WIDENED on lines (947 > baseline 919) — a baseline never grows`
  and `make gate` blocked. The rule is right; what was missing is a legitimate path, and the only one available
  was editing the baseline number — the silent widening the rule exists to prevent.
- [x] **ROOT CAUSE (WHY + WHERE)** — a debt baseline is a **stored copy of a mechanically owned value** (the
  file's own size) with nothing tying it to the moment it was taken, so a legitimate revision and a silent
  widening are indistinguishable to the checker. `grep -n 'debt' .doctrine/live_document_size/surfaces.tsv`
  → one hit per row, `rc=0`, and shows the header's own rule — "the exact measured baseline at adoption …
  A baseline never widens" — while the
  `roadmap` row's note names the missing piece: "a revision-aware baseline+delta adapter is deferred (adoption
  note trigger 3)". `grep -n -A6 'trigger' docs/decisions/decision_live-document-containment-proportionate-adoption.md`
  → trigger 3 is "a stored copy of a mechanically owned value needs an executed freshness oracle", and the
  record says whoever hits it owns it. This slice hit it.
- [x] **ADDRESSED (verified)** — the debt column now accepts `at=<revision>`, executed rather than documented:
  the measure step emits each file's first line as a sixth field, and the evaluator refuses a row whose token
  that line no longer declares. `bash scripts/check_live_doc_size.sh --self-test` →
  `live-doc-size --self-test: 15 arms, 0 failed`, `exit=0`, the four new arms being `GREEN-AT` (a baseline
  measured at the revision the file declares passes), `RED-AT-STALE` (a token the file no longer declares is
  refused), `RED-AT-WIDEN` (declaring the right revision does not license growth past the baseline) and
  `RED-AT-KIND` (a revision token on a collection row is refused rather than ignored). Against the REAL
  registry: `bash docs/tasks/artifacts/live_doc_size/run_live_doc_size_probes.sh` → `probes: 5 pass / 0 fail`,
  where `REAL-3` copies the registry, stales the roadmap row to `at=v0.2` and requires
  `debt baseline was measured at revision `v0.2`, which ROADMAP.md no longer declares`. The roadmap's baseline
  is re-based in the revision's own commit — `.doctrine/live_document_size/surfaces.tsv` now reads
  `lines=947;bytes=52818;at=v0.3` with the authority cited in its notes — and the tree passes:
  `bash scripts/check_live_doc_size.sh` → `live-doc-size: OK — 17 surfaces, 15 routes, 84 files measured`,
  `exit=0`. The rule is recorded in `docs/decisions/decision_revision-aware-containment-baseline.md`, and the
  adoption record's trigger 3 is marked fired and discharged locally rather than left owed.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `13 suite(s) green`, `exit=0`; the pre-existing debt arm still fires (`RED-DEBT`, a widened baseline with no
  `at=` axis, is refused exactly as before, so the new axis added a path and did not relax a rule); the other
  ten self-test arms are unchanged; no row other than `roadmap` carries `at=`, so every other surface is judged
  by the same comparison it always was. No Rust changed; `scripts/check_live_doc_size.sh` is a project-doctrine
  check, so this slice owes the immediate push and the observed CI verdict, recorded in the Verification Log.
- [x] **FIX** — one measurement field (the first line, tabs stripped), one early branch in the debt loop, four
  self-test arms, one probe arm, the header's refusal list, the re-based registry row, the decision record, and
  the adoption record's trigger line. Deliberately NOT done: importing the neutral JSONL checker package that
  trigger 3 nominally points at — the *contract* (an executed freshness oracle for a stored copy) is what the
  trigger asks for, and six lines in the existing awk evaluator discharge it without a 2 100-line interpreter
  in the commit path, which the adoption record's own local parameters forbid.
- [x] **LOCKSTEP** — the registry row and its notes; `TOOLBOX.md`'s containment rows name the new refusal
  class; `docs/decisions/INDEX.md` carries the record and the Knowledge Map was regenerated; `MEMORY.md`,
  `LIVE_STATUS.md`, `CHANGELOG.md` and `DEV_NOTES.md` updated in the commit that lands this leaf
  (`STITCHCAD-G0-0004c`, the slice that hit the trigger). Lesson promotion: **promoted** — the record carries
  an `answers:` line.

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
| `2026-09-29` | `SPINE.4.3` | `check_live_doc_size.sh` on the real tree; `--self-test`; `run_live_doc_size_probes.sh`; `make gate`; `make probes`; `make check` | `OK — 17 surfaces, 15 routes, 41 files measured, 19 warning(s)`, `exit=0`; `11 arms, 0 failed`; `probes: 4 pass / 0 fail`; `=== all doctrines green ===`; `7 suite(s) green`; `test result: ok. 1 passed` |
| `2026-09-30` | `SPINE.4.4` | `run_cell_budget_census.sh` + `--self-test`; `check_live_doc_size.sh` before/after and its `--self-test`; the size probe suite; `make gate`; `make probes` | `36 shapes / 625 data rows / recommended maxline health 275 B`, `exit=0`; `probes: 7 pass / 0 fail`; the warning moved from `272 B = 136% of its 200 B target` to `272 B = 99% of its 275 B target`, `OK — 80 files measured`; `11 arms, 0 failed`; `probes: 4 pass / 0 fail`; all doctrines green; `12 suite(s) green` |
| `2026-09-30` | `SPINE.4.4` (CI verdict, observed after the exceptional push `.doctrine/` owed) | `make check`/`gate`/`probes`; `git push origin main`; the Actions runs API for `head_sha=9b58c47` | `exit=0` all three; `119946b..9b58c47  main -> main` and `git rev-list --count origin/main..HEAD` → `0`; `runs: 2` — **`rust` completed `success`**, **`doctrines` completed `success`** |

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
| `SPINE.4.3` | `STITCHCAD-SPINE-0004c (leaf SPINE.4.3): enforce the containment registry` | D13 closed; 2nd project doctrine, 11 self-test arms + 4 probes |
| `SPINE.16` | `STITCHCAD-SPINE-0016 (leaf SPINE.16): declare the code-path seam` | D25 fixed; prose is no longer judged as code |
| `SPINE.17` | `STITCHCAD-SPINE-0017 (leaf SPINE.17): the push-cadence exception is derived` | `make push-due`; 3 arms observed |
| `SPINE.18` | `STITCHCAD-SPINE-0018 (leaf SPINE.18): derive the push-due trigger set` | false obligation removed; 3 arms re-observed |
| `SPINE.4.4` | `STITCHCAD-SPINE-0004d (leaf SPINE.4.4): the maxline target is derived from the cell budget` | D42 fixed by the evidence split; the ceiling rises by record; `.doctrine/` changed, so the immediate push is owed |
| `SPINE.5`, `SPINE.13`, `SPINE.15`, `SPINE.19` | `pending` | — |

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
- `2026-09-29`: `SPINE.4.3` landed — the containment registry is enforced by a second project doctrine
  (11 self-test arms, a 4-arm end-to-end probe, unconditional in hook and CI). D13 closed; the
  remaining `SPINE` leaves (`.5`, `.13`, `.15`) block no product work and are deferred behind it.
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
- `2026-09-30`: `SPINE.4.4` landed — the `book_collection` maxline health is derived from the binding table
  shape's cell budget (`275` B = `259` B of per-column p95 cells + `16` B of separator) instead of from a prose
  chapter's width (`200` B), with `run_cell_budget_census.sh` as the tracked producer and a decision record
  authorising the ceiling's move to `440` B. The warning survives at `99%` of the new target and now says the
  termbase is *at budget*; clearing it is `SPINE.15`'s table-authoring convention. D42 fixed here: SPINE's
  `17` completed checklists moved to `SPINE-evidence.md`, tree file `1096` → `552` lines.
