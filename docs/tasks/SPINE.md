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
  Status: `done`
  Goal: settle defect **D22** with a real oracle — does a GFM renderer split a table cell on a raw `|`
  inside a code span? — then record the answer and adopt the table-authoring convention for this
  repository (escape pipes inside code spans; keep cells short enough that the maxline axis stays
  visible), and clear the `decisions_collection` maxline debt by reformatting the one wide row.
  Acceptance: the oracle and its output are recorded (a rendered page, not a reading of the spec);
  the convention is written where authors look (`COMMIT.md`); the widest line in the files this project
  owns is inside its health target; the inherited checker is left untouched and the upstream question
  is reported.
  Verification: recorded below and in the acceptance checklist — the oracle is a tracked probe
  (`probes: 3 pass / 0 fail`) that renders a page and counts cells: a raw code-span pipe splits the cell and
  the rightmost cell is dropped, an escaped one does not; the convention is in `COMMIT.md`; both remaining
  prose-derived maxline targets are re-derived from their binding shapes (`decisions_collection` `320` →
  `382` B with its `maxline=491` debt cleared, `tasks_collection` `400` → `443` B); the inherited checker is
  untouched and the divergence is reported as **D47**, owned by `SPINE.20`.
  Commit: `STITCHCAD-SPINE-0015`

- ID: `SPINE.20`
  Status: `done` (created by `SPINE.15`, which settled D22 against a renderer and found that no gate in
  this repository enforces the convention the answer implies — defect **D47**; deferred behind product work
  by `decision_product-work-takes-the-frontier.md`)
  Goal: enforce the table convention in the project doctrine slot — refuse a staged `.md` table row carrying
  an unescaped `|` inside a code span, which the renderer splits and whose rightmost cell it drops, while the
  inherited `check_table_arity.sh` reports `0` defects for the same row.
  Acceptance: `scripts/check_table_code_pipes.sh` exists, is registered in
  `scripts/check_doctrines.project.sh` and mirrored in `DOCTRINE_ENFORCEMENT.md`; it carries a `--self-test`
  whose arms include the exact row the render probe renders, a row whose pipes are escaped (accepted), a
  pipe outside a code span (accepted, the arity checker's business) and a row in a fenced code block
  (accepted — a block is not a table); the inherited checker is not modified; `docs/tasks/artifacts/
  table_render/run_table_render_probes.sh` is its ground truth and stays green; the doctrine count in
  `LIVE_STATUS.md` is re-derived, not incremented by hand.
  Verification: recorded below and in the acceptance checklist — `scripts/check_table_code_pipes.sh`
  registered as the project doctrine `TABLE-CODE-PIPE`; `--self-test` → `7 arms, 0 failed`; the whole tracked
  book scanned at `0` violations before the gate was made absolute rather than a ratchet;
  `bash scripts/check_doctrines.project.sh` → `PROJECT-SPECIFIC: 3 project doctrine(s) green`; the counts in
  `LIVE_STATUS.md` re-derived (`scripts/check_doctrines.sh | grep -c '✅'` → `13` printed rows = 12 universal
  including the conditionally appended `KNOWLEDGE-MAP`, plus the project row; `find docs/tasks/artifacts
  -name 'run_*probe*.sh' | wc -l` → `14`).
  Commit: `STITCHCAD-SPINE-0020`

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
  `decision_product-work-takes-the-frontier.md`). **Narrowed by `.19.1`, which sealed the defect census at
  D49's trigger:** what remains is the durable half — a ledger-agnostic verifier, a segment registry, and an
  arm that refuses a fixed defect left live in the census.
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

- ID: `SPINE.19.1`
  Status: `done` (the first of `.19`'s three ledgers, taken at D49's trigger rather than waited for)
  Goal: seal the **defect census's closed entries** into `docs/history/stitchcad-defects-part1.md` under the
  descriptor contract, leaving the live census with the open ones and a pointer — D46's remedy, executed
  because the trigger D49 declares had fired: `wc -lc docs/tasks/PLANNING.md` → `1133 93378`, i.e. 95 % of
  the `tasks_collection` per-part byte ceiling, so the next defect any slice logged would have blocked a
  commit.
  Acceptance: the sealed segment carries its identity (lines, bytes, sha256) and reproduces under
  `run_changelog_ledger_probes.sh`'s `DESCRIPTOR` rule; every open defect stays live with its owner; the
  live section states how to derive both counts rather than asserting them; no entry is lost or duplicated
  across the two files; the tree returns inside its health target; D46 and D38 record what changed.
  Verification: recorded below and in the acceptance checklist — `44` closed entries sealed (`624` lines /
  `53 323` bytes, digest reproduced), `7` open kept, `PLANNING.md` `1133` / `93 378` → `515` / `40 606`,
  and the ledger probes at `9 pass / 0 fail` with `32` segment verdicts.
  Commit: `STITCHCAD-SPINE-0019a`

- ID: `SPINE.19.2`
  Status: `pending`
  Goal: D65 — perform an archive transition before ordinary seals exhaust the 64-file ceiling.
  Acceptance: retained historical segments remain byte-identical and addressable by stable identity;
  descriptors declare complete contents, digests, portable retrieval and retention ownership. Prove
  reconstruction, unique coverage, live/history navigation and pressure independently. Bound retained
  aggregate storage and reader working set; no nested-glob evasion or ceiling increase. Review current
  ledger consumers and adopted archive contract before choosing a safe storage topology.
  Priority: before a required product seal would exceed 64 files; audit the projected transaction
  at each rollover. Do not pivot dirty. Keep .19's
  ledger-agnostic coverage/pointer obligations distinct unless this transition requires them.
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

- ID: `SPINE.21`
  Status: `done` (recurring: the cadence record names the obligation, this leaf owns each run)
  Goal: the **recurring** artifact cleanup — remove only what is regenerable, gitignored and project-owned
  (the doctrine and probe scratch trees, cargo's incremental caches, the mdBook output, stray
  `.log`/`.bin`/`.tmp`/`.orig`/`.rej`/`.DS_Store`), prove by a residue census that what was removed is gone
  and that nothing tracked was touched, re-run the gates so the removal is shown to cost nothing but time,
  and overwrite the record's single latest entry.
  Acceptance: the run is measured before and after; the residue census names every path it checked and
  finds them absent; `git ls-files` proves no tracked file was deleted; `make gate`, `make check`,
  `make book` and `make probes` are green afterwards, the last two regenerating exactly what was removed;
  `docs/ARTIFACT_CLEANUP.md` carries one entry, dated absolutely, and no history (that is git's).
  Verification: recorded below and in the acceptance checklist — `target` went `40 648` KB → `10 808` KB
  and `docs/book/book` `4 120` KB → absent, all nine paths found gone by the residue census, `0` stray
  artifacts, `0` tracked artifact-shaped files before and after, `0` deleted tracked files, and all five
  make targets green afterwards with the book and the incremental caches regenerated.
  Commit: `STITCHCAD-SPINE-0021`

- ID: `SPINE.21a`
  Status: `done`
  Goal: recurring cleanup on 2026-10-01 before the next product slice crosses the 24-hour mark;
  remove only regenerable, ignored project artifacts after proving no tracked input or live job is held.
  Acceptance: census before/after names every removed path, proves residue absent and no tracked
  deletion; release/debug incremental/deps log/bin scan included, built dependencies retained.
  Re-run check/book/wasm/gate/probes after removal, overwrite only latest cleanup entry, preserve
  product frontier .4 and synchronize bounded live/history/book records. Prior cleanup checklist
  may relocate unchanged to the existing evidence sibling; ledger rollovers owned here if needed.
  Verification: 6 roots + 255 strays removed; target 663084 → 328736 KB; residue 0, tracked
  deletions 0; regenerated check/WASM/book, 22 probes, ledger and staged gates green.
  Commit: `STITCHCAD-SPINE-0021a`

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
| 12 | `SPINE.15` | `done` | D22 settled against a rendered page: a raw pipe in a code span splits the cell and the rightmost cell is dropped, so the convention is escape-always; both prose-derived maxline targets re-derived from their shapes |
| — | `SPINE.20` | `done` | taken immediately after `.15` on the director's instruction to act on the findings: the convention is now a gate (`TABLE-CODE-PIPE`), because a rule that lives only in `COMMIT.md` is a suggestion and what it prevents is a silently dropped column in the book the director reads |
| — | `SPINE.17` | `done` | taken out of order: a director-approved rule is recorded when it is made |
| — | `SPINE.18` | `done` | taken immediately after: `.17` shipped a trigger that fired on itself |
| trigger | `SPINE.19.2` | `pending` | D65: archive transition before a required product seal exceeds the 64-file ceiling |
| — | `SPINE.19.1` | `done` | taken at D49's trigger rather than waited for: `PLANNING.md` was at 95 % of its byte ceiling, so the 44 closed defects were sealed and the live census is now the open set |
| — | `SPINE.19` | `pending` | the archive verifier is ledger-agnostic (D40, found by the first non-changelog rollover). Deferred behind product work: the digest leg already covers every segment, so silent content drift is caught and only the coverage and pointer claims are not |
| — | `SPINE.21` | `done` | the cleanup cadence is recurring, and a recurring obligation with no leaf is one somebody rediscovers: taken between two product slices because the 24-hour mark falls inside this one |

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


### `SPINE.21a` — recurring cleanup before the next product slice

- [x] **REPRODUCE / ISSUE** — prior run was 2026-09-30 19:00 UTC; the 24-hour mark falls inside
  the next product slice. `du -sk target docs/book/book` → `663084`, `4832`, `rc=0`;
  release/debug incremental/deps artifact scan → `237` bin/log files, all incremental caches.
- [x] **ROOT CAUSE (WHY + WHERE)** — Cargo/probe/book outputs accumulate as expected; recurring
  .21 owns the obligation. `git ls-files` census → `0` tracked artifact-shaped files, `rc=0`;
  `check_no_background_jobs.sh` → `handoff: OK`, `rc=0`; candidate-tree `.git` scan → none.
  Deleting ignored regenerable outputs can reclaim storage without touching tracked input.
- [x] **FIX** — removed six declared scratch/incremental/book roots and 255 ignored project strays;
  removed roots: target doctrine/scratch/tmp, host + WASM incremental caches and rendered book.
  Checked tracked descendants and symlinks before deletion. Kept built dependencies/shared stores;
  record overwrites its latest entry and product frontier stays G1 .4.
- [x] **ADDRESSED (verified)** — `du -sk target` immediately after deletion → `328736`, `rc=0`:
  `339180` KB reclaimed including book. Independent `find` artifact census → `0`, `rc=0`;
  every planned path absent and `git status --porcelain` → `0` tracked deletions. Release/deps
  bin/log census was `0` each. Rebuild regenerates scratch, incremental caches and rendered book.
- [x] **NO REGRESSION** — `make check` → strict fmt/clippy + `287` tests green; `make wasm`/`book`
  → green, warning-free; full `make probes` → `22 suite(s) green`; ledger → `9 pass / 0 fail`;
  tree census → `0 unowned / 0 orphan(s) / 0 dead link(s)`; staged `make gate` →
  `=== all doctrines green ===`, all `rc=0`. Completed prior cleanup checklist compares unchanged to HEAD.
- [x] **LOCKSTEP** — cleanup record, recurring leaf/logs, bounded book upkeep, changelog rollover,
  and unchanged product resume/frontier agree. LIVE_STATUS still G1 5/18, four structural families,
  8 open / 54 sealed; no area status changes. Memory continues to point to G1 .4.
  promotion: declined (routine cadence discharge; ownership/safe-removal rules already canonical).

### `SPINE.19.1` — the closed defects are sealed, so the open ones are what a reader meets

- [x] **REPRODUCE / ISSUE** — `wc -lc docs/tasks/PLANNING.md` → `1133 93378` against a `tasks_collection`
  per-part health of `800` / `65 536` and a ceiling of `1200` / `98 304`: 95 % of the ceiling, with the
  census section alone at `sed -n '/^## Defects found/,/^## Decisions/p' docs/tasks/PLANNING.md | wc -c` →
  `69 898` bytes at `HEAD`, of which the closed entries were `50 292` (the sum over the 44 entries this
  slice sealed). Every closed defect added to it permanently, and the next one any slice logged would have
  blocked a commit.
- [x] **ROOT CAUSE (WHY + WHERE)** — D46 named it and `SPINE.19` owns the durable fix, but the deferral's
  premise ("the file is inside every ceiling today") had expired: `grep -c '^- \*\*D[0-9]'
  docs/tasks/PLANNING.md` → `51` entries, of which reading each owner line classifies `44` as fixed. The
  classification has to be read rather than parsed, which is D38 — the states are prose — and is the reason
  the seal also fixes D38's harm: with only open entries live, the count a reader needs is the count of rows
  in the file.
- [x] **ADDRESSED (verified)** — `44` closed entries sealed into `docs/history/stitchcad-defects-part1.md`
  (`624` lines / `53 323` bytes / `sha256:1897bde0…`), in the census's own order, with the descriptor
  contract's byte rule honoured: `TMPDIR=$PWD/target/scratch bash
  docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh` → `probes: 9 pass / 0 fail`, `REAL` at
  `32 verdicts, 0 failures`, so the new segment's digest reproduces under the standing `DESCRIPTOR` rule.
  `PLANNING.md` → `515` lines / `40 606` bytes, inside its health on both axes. Nothing lost and nothing in
  two places: `grep -c '^- \*\*D[0-9]'` over the two files → `7` live and `44` sealed, `51` in total, and
  `comm -12` over the two id lists is empty.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`; `make probes` → `20 suite(s) green`;
  `bash scripts/check_live_doc_size.sh` → `OK — 17 surfaces, 15 routes, 111 files measured`, with
  `tasks_collection` falling from `1133` / `93 378` to `991` / `92 650` at the collection maximum and no new
  warning; the tree-coverage census still reports `10 lanes / 13 trees / 3 sibling(s) / 0 unowned /
  0 orphan(s) / 0 dead link(s)`, so the new history segment is not mistaken for a task-tree sibling. One
  defect in this slice's own work was found and fixed before committing: the first seal wrote the descriptor
  without the content (a missing concatenation), which the `DESCRIPTOR` rule caught as a `1`-byte segment —
  the arm that proves the digest leg is load-bearing.
- [x] **FIX** — sealed the closed entries; rewrote the census's intro to say the section holds the OPEN set,
  name the segment with its identity, and give the two commands that derive the counts instead of a
  hand-kept pair; recorded in D46 that its remedy is executed and in D38 that the harm is reduced by
  structure; created this leaf so `.19` keeps the durable half (a ledger-agnostic verifier and a segment
  registry) undiluted.
- [x] **LOCKSTEP** — this leaf and checklist, `SPINE.19`'s status note, the tree's frontier, three logs;
  `docs/tasks/PLANNING.md` (D46, D38, D49), `LIVE_STATUS.md` and `CHANGELOG.md` in this commit.
  `MEMORY.md` and `docs/TASK_TREE.md` are unchanged: no product frontier moved. Lesson promotion:
  declined (no new dated lesson — the reusable rule, "seal at the trigger rather than at the breach", is
  already D49's recorded trigger and this leaf is its second discharge).

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
| `2026-09-30` | `SPINE.4.4` | `run_cell_budget_census.sh` + `--self-test`; `check_live_doc_size.sh` before/after and its `--self-test`; the size probes; `make gate`/`probes` | `36 shapes / 625 data rows / recommended maxline health 275 B`, `exit=0`; `probes: 7 pass / 0 fail`; the warning moved from `272 B = 136% of its 200 B target` to `272 B = 99% of its 275 B target`; `11 arms, 0 failed`; `probes: 4 pass / 0 fail`; all doctrines green |
| `2026-09-30` | `SPINE.4.4` (CI verdict, observed after the exceptional push `.doctrine/` owed) | `make check`/`gate`/`probes`; `git push origin main`; the Actions runs API for `head_sha=9b58c47` | `exit=0` all three; `119946b..9b58c47  main -> main` and `git rev-list --count origin/main..HEAD` → `0`; `runs: 2` — **`rust` completed `success`**, **`doctrines` completed `success`** |

| `2026-09-30` | `SPINE.4.5` (CI verdict, observed after the exceptional push) | `make check`/`gate`/`probes`; `git push origin main`; the Actions runs API for `head_sha=513374c` | `exit=0` all three; `9b58c47..513374c  main -> main`, ahead `0`; `runs: 2` — **`doctrines` completed `success`**, **`rust` completed `success`**, so the revision-aware baseline is verified by a runner and not only here |

| `2026-09-30` | `SPINE.15` | the render oracle; both cell-budget derivations; containment + `--self-test` + its probes; every census; `make gate`/`probes`/`book` | `probes: 3 pass / 0 fail`, the raw-pipe row rendered with its rightmost cell dropped; `382 B` and `443 B` derived; `OK — 88 files`, `15 arms, 0 failed`, `probes: 5 pass / 0 fail`; all censuses `0 failure(s)`; `14 suite(s) green` |

| `2026-09-30` | `SPINE.15` (CI verdict, observed after the exceptional push) | `make check`/`gate`/`probes`; `git push origin main`; the Actions runs API for `head_sha=4e34b90` | `exit=0` all three; `513374c..4e34b90  main -> main`, ahead `0`; `runs: 2` — **`doctrines` `success`**, **`rust` `success`** |

| `2026-09-30` | `SPINE.20` | `check_table_code_pipes.sh --self-test`; a scan of every tracked `.md` for the shape it refuses; `check_doctrines.project.sh`; `make gate`; the render oracle it takes as ground truth | `7 arms, 0 failed`; `0` violations in the tracked book, so the gate is absolute and not a ratchet; `PROJECT-SPECIFIC: 3 project doctrine(s) green`; `=== all doctrines green ===`; `probes: 3 pass / 0 fail` |

| `2026-09-30` | `SPINE.20` (CI verdict, observed after the exceptional push `scripts/` owed) | `make check`/`gate`/`probes`; `git push origin main`; the Actions runs API for `head_sha=a743d53` | `exit=0` all three; `4bd4027..a743d53  main -> main`, ahead `0`; `runs: 2` — **`doctrines` `success`**, **`rust` `success`**, the first runner execution of `TABLE-CODE-PIPE` |
| `2026-09-30` | `SPINE.21` | `du -sk` before and after; the residue census over all nine paths; `git ls-files` and `git status --porcelain`; `find` for strays; `make gate`/`check`/`book`/`probes`/`wasm` | `40 648` KB → `10 808` KB plus the book's `4 120` KB; all nine gone; `0` tracked files touched; `0` strays; five targets green |
| `2026-09-30` | `SPINE.19.1` | the seal's digest under the standing `DESCRIPTOR` rule; both id censuses and their intersection; `wc -lc` before and after; `make gate`/`probes`; containment; the coverage census | `9 pass / 0 fail`, `32` segment verdicts; `7` live + `44` sealed = `51`, intersection `0`; `1133` / `93 378` → `515` / `40 606`; all green |

| `2026-10-01` | `SPINE.21a` | ownership/residue census; check/wasm/book/probes; ledger/tree; staged gate | 6 roots + 255 strays gone, 0 residue/tracked deletion; `287` tests, `22` suites green |

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
| `SPINE.4.5` | `STITCHCAD-G0-0004c (leaf G0-CONTRACT.4c, SPINE.4.5, PLANNING.6, G0-CONTRACT.14b)` | landed in the slice that applied roadmap v0.3 and hit the trigger; CI observed green on `513374c` |
| `SPINE.15` | `STITCHCAD-SPINE-0015 (leaf SPINE.15): the table convention is settled by a rendered page` | D22 closed, D47 logged and `SPINE.20` created; both prose-derived maxline targets re-derived; the decisions debt cleared |
| `SPINE.20` | `STITCHCAD-SPINE-0020 (leaf SPINE.20): the table convention becomes a gate` | D47 closed; `TABLE-CODE-PIPE` registered in the project slot; the inherited arity checker untouched |
| `SPINE.21` | `STITCHCAD-SPINE-0021 (leaf SPINE.21): the cadence runs, and the residue census proves what it took` | 33 960 KB off the volume; nothing tracked touched; all five make targets green afterwards |
| `SPINE.19.1` | `STITCHCAD-SPINE-0019a (leaf SPINE.19.1): the closed defects are sealed, so the open ones are what a reader meets` | D46's remedy executed at D49's trigger; `SPINE.19` keeps the durable half |
| `SPINE.5`, `SPINE.13`, `SPINE.19` | `pending` | — |

| `SPINE.21a` | `STITCHCAD-SPINE-0021a (leaf SPINE.21a)` | recurring cleanup, verified regeneration; product frontier unchanged |

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
- `2026-09-30`: `SPINE.15` landed — D22 is settled by a rendered page rather than by reading a specification:
  a raw `|` inside a table cell's code span **splits the cell** and the renderer drops the rightmost cell
  silently, while `\|` keeps the row intact. The oracle is tracked
  (`docs/tasks/artifacts/table_render/run_table_render_probes.sh`, `probes: 3 pass / 0 fail`) and its third arm
  pins the divergence — the inherited `check_table_arity.sh` still self-tests the opposite, so it under-reports
  and is left untouched as NEUTRAL, with the gap reported as **D47** and owned by a new leaf **`SPINE.20`**.
  The convention is in `COMMIT.md` where authors look. Both remaining prose-derived maxline targets are
  re-derived from their binding shapes with the `SPINE.4.4` instrument (`decisions_collection` `320` → `382` B,
  its `maxline=491` debt cleared; `tasks_collection` `400` → `443` B), and the fat those targets were blamed
  for was removed at the same time: one `491` B row became bounded prose, nine index hooks are one line each
  again, and four verification rows above the derived budget were tightened. `make probes` is now
  `14 suite(s) green`.
- `2026-09-30`: `SPINE.20` landed — the table convention `SPINE.15` wrote into `COMMIT.md` is enforced:
  `TABLE-CODE-PIPE` refuses a staged `.md` table row carrying a raw pipe inside a code span, the shape a
  renderer splits and whose rightmost cell it drops silently. Seven `--self-test` arms (raw, escaped, an
  ordinary separator, a fenced quotation, a double-backtick span, prose, an indented row); the whole tracked
  book measured at `0` violations first, so the gate is absolute rather than a ratchet. The inherited
  `check_table_arity.sh` is untouched — NEUTRAL spine code is reported upstream, never patched locally — and
  D47 closes with the divergence recorded in `DOCTRINE_ENFORCEMENT.md`'s mirror.
- `2026-09-30`: `.21` landed — the artifact-cleanup cadence now has a recurring owner. `SPINE.2`
  discharged the first run and wrote the record, but a cadence is an obligation that returns, and a
  returning obligation with no leaf is one somebody rediscovers mid-slice. This run removed nine paths
  (both scratch trees, both incremental caches, the mdBook output, three self-test scratch bodies),
  took 33 960 KB off the volume, proved by a residue census that each is gone and that no tracked file
  was touched, and re-ran all five make targets so the removal is shown to cost rebuild time and
  nothing else. `SPINE.20`'s checklist moved to the evidence sibling, as the convention requires of the
  slice after the one that landed it, which also brings this tree back inside its per-part health.
- `2026-09-30`: `.19.1` landed — D46's remedy executed at the trigger D49 declares, rather than in the
    commit that would have breached. `PLANNING.md` had reached `1133` lines / `93 378` B, 95 % of its
    byte ceiling, with `50 292` B of that belonging to defects already fixed; `44` closed entries were
    sealed into `docs/history/stitchcad-defects-part1.md` under the descriptor contract and the live
    census kept the `7` open ones, a pointer, and the two commands that derive both counts. The tree is
    now `515` / `40 606`, inside its health on both axes, and the standing `DESCRIPTOR` rule reproduces
    the new segment's digest. Classifying the `51` entries needed a reading of each owner line — three
    of them use wording no marker list anticipated — which is D38 measured again, and is why `.19`'s
    durable half (a status token a script can read, a segment registry, and an arm that refuses a fixed
    defect left live) stays open.

- `2026-10-01`: `.21a` discharges the recurring cleanup before the next product slice crosses the
  24-hour mark. Six regenerable roots + 255 strays removed, residue/tracked-deletion proof clean;
  workflows regenerate successfully. Prior cleanup evidence moves unchanged; product frontier .4.
