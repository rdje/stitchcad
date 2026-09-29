# CHANGELOG.md

Newest first. The StitchCAD sections are this project's history; everything below the
_Inherited spine history_ divider is the bedrock scaffold's own changelog, kept as the
provenance of the discipline spine this repository was generated from.

## STITCHCAD-SPINE-0012a — the first push is made and CI is green (leaf `SPINE.12`, addendum)

- `git push origin main` → `f1dcbe4..051a075  main -> main`; ahead-count after the push:
  `git rev-list --count origin/main..HEAD` → `0`. Eleven commits — the bootstrap, the roadmap→tree
  seeding, and the spine-integrity leaves — are now off this machine.
- **CI observed, not assumed:** both workflows completed `success` for the pushed head
  (`gh run list`, cross-checked against `…/actions/runs?head_sha=051a075…` → `total_count: 2`):
  `doctrines` in 11 s (run 36622373461) and `rust` in 18 s (run 36622373539). The verdict was
  recorded in `SPINE.12`'s Verification Log by this follow-up commit, after observation.
- The 400-commit cadence in `COMMIT.md` is now in force: the next push happens at 400 commits ahead,
  or earlier only if the director asks.

## STITCHCAD-SPINE-0012 — the push cadence is now a written rule (leaf `SPINE.12`)

- **Director's ruling recorded:** 400 commits between pushes. `COMMIT.md` had **zero** mentions of
  pushing (`git show HEAD:COMMIT.md | grep -ciE 'push'` → `0`), so the cadence lived only in
  conversation — exactly the state `MEMORY_ARCHITECTURE.md` calls "not yet saved". The new
  `## Push cadence` section names the threshold, the deriving command
  (`git rev-list --count origin/main..HEAD` — never a hand-carried count), the pre-push full gate
  (`make check`, `make gate`, `make probes`), and the single exception (the director asks).
- **The trade-off is written down, not buried:** §8 of the memory architecture argues for frequent
  pushes because an unpushed commit dies with the machine. A 400-commit cadence trades that away for
  fewer interruptions; the section says so, and notes the mitigation (every memory layer is committed
  per slice, so at most un-pushed commits are at risk).
- The director then authorised a **one-off first push** for this project (10 commits ahead at the
  ruling); the 400-commit cadence resumes once it is made and CI is green. The CI verdict is recorded
  in the leaf, by a follow-up commit, after it is observed.
- A wrong number was caught before it shipped: the first draft of this leaf's evidence claimed
  `grep -ciE 'push'` → `1`; re-running it gave `0`. Also corrected: `grep -c '400' COMMIT.md` → `3`.
- Validation: `make check` → `test result: ok. 1 passed; 0 failed`; `make gate` →
  `=== all doctrines green ===`; `make probes` → `5 suite(s) green`.

## STITCHCAD-SPINE-0002 — the first artifact cleanup, and a cadence a next session can read (leaf `SPINE.2`)

- **Defect D8 fixed.** `docs/ARTIFACT_CLEANUP.md` did not exist, so the 24-hour cleanup cadence had
  no record to read at startup and no way to be audited. It now carries one entry (the latest run, by
  design — history is git's), the list of what is safe to remove, and the list of what is never
  removed (tracked files, the current build cache, any shared global cache outside the repository).
- **Cleanup run, measured:** removed four regenerable, gitignored paths — `target/doctrine_scratch`
  (188 KB of doctrine self-test and debugging scratch), `target/scratch` (probe `TMPDIR`),
  `target/debug/incremental` (896 KB, 9 `.bin` cargo caches) and `docs/book/book` (1 068 KB of mdBook
  output). `target` went 2.1 MB → 1.2 MB. Residue census: all four paths `gone`, `0` stray
  `*.log` / `*.bin` / `.DS_Store` files left.
- **Nothing tracked was touched:** `git ls-files | grep -cE '\.(log|bin|tmp)$|^target/|^docs/book/book/'`
  → `0` before and after; `git status --short` showed no deletion. `target/debug/deps` was deliberately
  kept — current build cache, cheap to keep and slow to rebuild — and the record says so rather than
  leaving the choice implicit.
- Validation after the deletion, against the cleaned tree: `make gate` → `=== all doctrines green ===`;
  `make check` → `test result: ok. 1 passed; 0 failed` (rebuilding exactly what was removed);
  `make book` → `exit=0`; `make probes` → `5 suite(s) green`.

## STITCHCAD-SPINE-0001 — the repository introduces itself as StitchCAD (leaf `SPINE.1`)

- **`README.md` is a StitchCAD landing page** (defect D3): what the product is, the three properties
  that define it (uncertainty is data, headless-first, agent-drivable), the current gate, audience and
  explicit non-goals, the crate map by layer, a verified quick start, a canonical-home table that routes
  changing detail to `ROADMAP.md` / the book / `LIVE_STATUS.md` / `docs/tasks/` / `docs/decisions/`, the
  layout and the license. `scripts/check_readme_stability.sh` →
  `README-STABILITY: OK — README.md is 103/300 lines, 6063/16384 bytes.`, `exit=0`, and zero
  date-stamped lines. Before: `git show HEAD:README.md | head -1` →
  `# bedrock — a Rust project discipline-spine template`, with `grep -c StitchCAD` → `0`.
- **The mdBook is named and has a real front door** (defect D4): `book.toml` title/authors/description
  and repository URL replace `"Project Book"` / `"<your name>"`; the template introduction is replaced
  by one that states the idea, the two commitments, who the book is for, what is true right now (G0 —
  specification, not shipped behavior) and how the book is organised; `docs/book/src/spec/index.md`
  states how to read a normative chapter (SHALL/SHOULD/MAY, status lives elsewhere, claims carry their
  verification) and what each G0 chapter settles; `SUMMARY.md` opens the Specification part.
  `mdbook build docs/book` → `INFO HTML book written to …`, `exit=0`.
- **Defect D19 found and fixed while verifying the quick start:** `make book` writes
  `docs/book/book/`, which `.gitignore` did not ignore, so the last documented command left
  `?? docs/book/book/` in the tree and broke handoff-readiness. Ignored; a build now leaves
  `git status --short` clean.
- Every quick-start command was run, not assumed: `make check` → `test result: ok. 1 passed; 0 failed`;
  `make gate` → `=== all doctrines green ===`; `make probes` → `5 suite(s) green`; `make book` →
  `exit=0`; `git config core.hooksPath` → `.githooks`.

## STITCHCAD-SPINE-0010 — one probe entry point, scratch on the repository volume (leaf `SPINE.10`)

- **Defect D16 fixed.** The inherited probe suites take their scratch from `mktemp -d`, which resolved
  to `/var/folders/…` — the system volume — while the repository lives on `/Volumes/SSD`, and each
  suite builds throwaway **git repositories** there (`grep -ln 'mktemp -d' docs/tasks/artifacts/*/*.sh
  scripts/*.sh` → 3 files). `make probes` now discovers every `run_*probe*.sh` under
  `docs/tasks/artifacts/` and runs it with `TMPDIR="$(CURDIR)/target/scratch"`, derived at run time so
  the repository stays relocatable: `make probes: 5 suite(s) green`, `exit=0` — 37 arms across the
  fresh-evidence (9), scaffold-sync (7), multi-leaf shadowing (6), task-acceptance (10) and
  waiver-routing (5) suites.
- **No inherited file was edited** — the pin is applied by the caller (`git diff --stat HEAD --` over
  the two inherited suites → empty, `rc=0`), which keeps `scripts/update_scaffold.sh` able to sync them.
- **One residual, recorded instead of hidden:** the shared `scripts/check_task_acceptance.sh` still
  takes a trap-cleaned `mktemp -d` scratch at commit time, so that single transient directory lands
  wherever `TMPDIR` points. Pinning it would mean editing shared code or a NEUTRAL hook; it persists
  nothing, so it is accepted in the leaf and reported upstream with the suggestion that the driver
  export a repo-local `TMPDIR`.
- Validation: `make check` → `test result: ok. 1 passed; 0 failed`; `scripts/check_doctrines.sh` →
  `=== all doctrines green ===`, `exit=0`; `git status --short` after the run shows only the intended
  edits.

## STITCHCAD-SPINE-0009 — the scaffold updater can no longer clobber project content (leaf `SPINE.9`)

- **Defect D17 fixed.** `scripts/update_scaffold.sh` carried one list described as "safe to overwrite
  because it never carries project content", and four of its entries are the files the template tells
  a project to fill in — `docs/TASK_TREE.md` (the Active Task Trees index, layer-B navigation),
  `TOOLBOX.md` (the project toolbox), `README_POLICY.md` (the local adoption note),
  `docs/tasks/TEMPLATE.md` — at lines `27`, `28`, `31`, `33`. One run of the documented "keep the
  spine current" command would have replaced the index with template blanks.
- **Two declared classes now**, with a recorded reason per guarded file: NEUTRAL is synced (previous
  copy backed up), PROJECT-CONTENT is backed up, reported `SKIPPED` and left alone unless
  `--force-project-sections` is passed. Added `--dry-run` (reports the classification, writes nothing
  — no syncs, no backups, no file modes), a dirty-tree refusal with `--allow-dirty` as the explicit
  override, and scratch/backup paths under `target/` on the repository volume.
- **Probe suite** `docs/tasks/artifacts/scaffold_sync/run_update_scaffold_probes.sh` →
  `probes: 7 pass / 0 fail`. ARM-2 rebuilds the founding situation (local index rows + a changed
  upstream copy) and asserts the rows survive; ARM-4 proves a dry run leaves the whole-tree checksum
  unchanged; ARM-5 proves the dirty-tree refusal wrote nothing; ARM-6 proves `--force` still backs up.
- **Two probe defects caught by its own arms before the leaf closed:** a reused output variable made
  ARM-5 assert against the wrong run, and `stat -f %m` measured the GNU coreutils `stat` that this
  machine's `PATH` puts ahead of BSD userland — so the arm "measured" filesystem dumps instead of
  mtimes. The arm now pins `/usr/bin/stat` and fails loudly if neither form works; `TOOLBOX.md` carries
  the general rule (pin the instrument you measure with).
- Recorded as `docs/decisions/decision_scaffold-sync-protects-project-content.md` (indexed), which
  also resolves the forward reference from the acceptance-evidence record.
- Validation: `bash -n` clean on both scripts; the three other probe suites unchanged at
  `10/0`, `6/0`, `9/0`; `make check` → `test result: ok. 1 passed; 0 failed`;
  `scripts/check_doctrines.sh` → `=== all doctrines green ===`, `exit=0`.

## STITCHCAD-SPINE-0008 — acceptance evidence must be fresh in the commit that lands the work (leaf `SPINE.8`)

- **New project doctrine `FRESH-ACCEPTANCE-EVIDENCE`** (`scripts/check_fresh_acceptance_evidence.sh`,
  registered in `scripts/check_doctrines.project.sh` — the project slot, never the universal
  driver): a staged CODE change must ADD its own ticked, evidence-backed ROOT CAUSE / ADDRESSED /
  NO REGRESSION bullets in the same commit, in at least one staged leaf. Evidence committed for an
  earlier leaf can no longer answer for a later leaf's change, which is defect D15 facet 1 — the
  inherited first-match-per-file scan cannot see it, and a project check cannot relax a universal one,
  only add to it.
- **Probe suite** `docs/tasks/artifacts/fresh_evidence/run_fresh_evidence_probes.sh` →
  `probes: 9 pass / 0 fail`, including the pairing arm that prints both verdicts for one staged
  state (`RED-1 exit=1` here, `exit=0` from the inherited check) and `CTRL-4`, which pins that a
  co-staged documentation tree is not forced to invent evidence (no fourth false-red class).
  `--self-test` → `8 verdict controls + 6 extractor arms`; ground truth runs on every invocation and
  refuses (`exit=2`) if a control misses; scratch lives under `target/doctrine_scratch` (repo volume).
- **A defect in the first implementation, caught by its own arm:** signature matching written in awk
  left 12 of 36 realistic evidence lines unmatched on this platform (`\b` and `{n}` are GNU
  extensions BSD awk 20200816 lacks) where `grep -qE` — the engine the universal check uses — leaves
  2, both bad samples. The check now uses grep for signatures and awk only for bullet structure, and
  `GREEN-2` pins it. The candidate defect record against the inherited list ("D18, not portable")
  was **withdrawn**: GNU grep 3.12 and BSD grep 2.6.0-FreeBSD both match those families.
- Validation: `scripts/check_doctrines.sh` → `=== all doctrines green ===`, `exit=0` (project slot:
  `PROJECT-SPECIFIC: 1 project doctrine(s) green`); inherited suites untouched at
  `probes: 10 pass / 0 fail` and `probes: 6 pass / 0 fail`; `make check` →
  `test result: ok. 1 passed; 0 failed`; per-commit cost ~0.3 s.

## STITCHCAD-SPINE-0007 — the multi-leaf acceptance-evidence hole is measured, not suspected (leaf `SPINE.7`)

- **New committed probe** `docs/tasks/artifacts/task_acceptance/run_multileaf_shadowing_probe.sh`
  (6 arms, 3 of them controls) builds throwaway repositories and runs the *shipped*
  `scripts/check_task_acceptance.sh` against real staged diffs → `probes: 6 pass / 0 fail`.
- **Defect D15 confirmed in both directions.** `HOLE-1`: a code change owned by leaf `TREE.2` is
  accepted on leaf `TREE.1`'s ticked evidence because the extractor takes the first bullet matching
  each label (`scripts/check_task_acceptance.sh:106-112`) — `exit=0`. `HOLE-2`: an honest,
  evidence-backed leaf is refused because an unticked placeholder for a *future* leaf sits above it
  — `exit=1`; `HOLE-3` deletes the placeholder and the identical leaf passes, which attributes the
  refusal. The shipped suite's cross-leaf control stages the other leaf in a separate *file*, so the
  property its header claims is wider than the property it verifies.
- **Convention promoted to layer C** — `docs/decisions/decision_acceptance-evidence-per-leaf.md`
  (indexed, with an `answers:` line so a question can find it): checkboxes exist only for leaves
  whose work has landed, each in its own `### <leaf-id>` subsection, added in the same commit.
  Deliberately *not* written into `docs/tasks/TEMPLATE.md`, which the scaffold updater treats as
  neutral and would overwrite (D17).
- **Two further defects logged and owned:** D16 — the inherited probe suites `mktemp -d` onto the
  system volume while the repository lives on another (`SPINE.10`); D17 — `update_scaffold.sh`
  lists `docs/TASK_TREE.md`, `TOOLBOX.md`, `README_POLICY.md` and `docs/tasks/TEMPLATE.md` as
  NEUTRAL although the template instructs projects to fill them in, so the documented "keep the
  spine current" command would silently destroy the task-tree index (`SPINE.9`).
- **Facet 3, caught by this very commit.** The first attempt was REFUSED: with the probe (a `.sh`
  file) staged, the check judged every staged leaf file — including a documentation tree owning no
  code — and reported `❌ TASK-ACCEPTANCE … docs/tasks/PLANNING.md — the 'ROOT CAUSE' box is ticked
  but carries no tool-output evidence`, `exit=1`. The box it read (`PLANNING.md:155`) cited two
  census commands and their real listings; recognized signature families inside that bullet: `0`.
  Fixed by adding each command's counts and `rc=0` to the bullets and by adopting two authoring
  rules (invocation + output + exit status; never co-stage an unrelated tree with a code change).
  No gate was weakened and nothing was waived.
- Validation: inherited suite still `probes: 10 pass / 0 fail`; `bash -n` clean; `make check` →
  `test result: ok. 1 passed; 0 failed`; `scripts/check_doctrines.sh` → `=== all doctrines green ===`.

## STITCHCAD-PLANNING-0002 — the engine-stage lanes are owned (leaf `PLANNING.2`)

- **Seeded four delivery trees** from roadmap §11 exit criteria, 58 leaves in total:
  `G1-SLICE` (16 — crate layout, `sc-units`, ontology, `sc-measure`, recipe evaluation, command
  bus, `sc-store`, CSP kernel, `sc-api`/`sc-mcp`, `sc-cli`, three runtime profiles, the browser
  and canvas-hosting spikes, dev shell, license census), `G2-2D` (14 — geometry kernel, offset
  engine + pathology corpus, canonicalizer, DXF, PDF, printed scale square, CLI replay, viewer,
  metamorphic + mutation suites, conformance matrix, agent gate), `G3-GRADING` (14 — closures,
  walk/true, identity under edit, bodice + set-in sleeve, both instantiation paths, `.rul`,
  extreme sizes, grading modes, notch set), `G4-PROFILES` (14 — profile schema v2, typed AST,
  CSP + explained unsat, differential oracle, verification pass, evidence store, policy matrix,
  composition, minimal Profile Editor, HPGL, private/public paths, usability gate).
- **Closed three G0 clauses that had no owner**, found by the same census: §7.6 (one message
  system chosen at G0 + externalization architecture) → `G0-CONTRACT.16`; §4.4 (command-layer
  contract, undo/redo granularity defined at G0) → `.17`; §4.3 + §7.3 (G0 CI: fmt/clippy/
  unit+property/WASM smoketest over `sc-core` + `sc-units`) → `.18`. That last clause also
  corrected this tree's earlier "no code in G0" reading, recorded as a superseding decision.
- **Logged defect D15 with measured evidence:** the inherited `TASK-ACCEPTANCE` gate judges the
  FIRST box matching each label in a staged tree file, so in a multi-leaf file one leaf's evidence
  answers for another leaf's code change (probe ARM-1: `task-acceptance: OK`, `exit=0`) and an
  earlier unticked placeholder rejects a leaf carrying real evidence below it (ARM-2: three
  `box is present but NOT ticked` refusals, `exit=1`). Owned by `SPINE.7` (committed probe +
  authoring convention) and `SPINE.8` (a `FRESH-ACCEPTANCE-EVIDENCE` doctrine in the project
  slot); the upstream fix is reported, not patched — the spine is shared code.
- **Convention adopted:** tree files carry no unticked placeholder acceptance boxes; each
  completed leaf adds a `### <leaf-id>` checklist subsection in the same commit as its work.
- Validation: `make check` → `test result: ok. 1 passed; 0 failed`; `scripts/check_doctrines.sh`
  → 13 checks, `=== all doctrines green ===`, `rc=0`.

## STITCHCAD-SPINE-0006 — the workspace lockfile is tracked (leaf `SPINE.6`)

- A fresh clone's first `make check` left `?? Cargo.lock` in the tree, which breaks the pivot
  rule's definition of handoff-ready while `.gitignore` states the lockfile is deliberately not
  ignored (defect D14). Tracked the cargo-generated file unedited; `git status --short` is now
  empty after `make check`.

## STITCHCAD-PLANNING-0001 — roadmap v0.2 represented as task-trees (leaf `PLANNING.1`)

- **Seeded the lane trees:** `docs/tasks/PLANNING.md` (roadmap → tree mapping),
  `docs/tasks/G0-CONTRACT.md` (gate G0, 15 leaves mapped clause-by-clause to the gate's exit
  criteria), `docs/tasks/SPINE.md` (repository identity, hygiene and adopted policy, 5 leaves).
- **Repaired the layer-B index:** `docs/TASK_TREE.md` listed bedrock's own maintenance tree
  (a link to a file that does not exist) and omitted the `BOOTSTRAP` tree that does. The index
  now registers exactly the trees on disk and states the census rule that keeps it true.
- **Logged 13 startup defects with owners** (`PLANNING.md` → "Defects found at startup"), each
  with a reproduce command: template identity in `README.md`/mdBook/`TOOLBOX.md`/starter crate,
  stale layer-A pointer, bedrock-only changelog, missing cleanup-cadence record, an out-of-date
  README policy copy, two unadopted director policies, and unbounded live-document growth.
- **Refreshed the live docs:** `LIVE_STATUS.md` now carries one row per roadmap lane; `MEMORY.md`
  names the real active tree, next action and order, and derives the latest commit instead of
  carrying a hash its own commit would invalidate.
- Validation: `scripts/check_doctrines.sh` → 13 checks, `=== all doctrines green ===`, `rc=0`.

---

# Inherited spine history (bedrock scaffold — provenance only)

## bedrock-scaffold 0.6.1 — creating a project is foolproof through its first commit

`BEDROCK-MAINTENANCE.2.7`.

- ⛔ **Measured on a fresh clone of 0.6.0:** `bootstrap.sh` left the crate rename — a CODE change — with no owning
  leaf, so the new project's FIRST commit was refused by `TASK-TREE-OWNERSHIP` and `TASK-ACCEPTANCE`. A new user's
  first contact with the discipline was a refusal about a rename the tool made.
- **`bootstrap.sh` now seeds `docs/tasks/BOOTSTRAP.md`** on a fresh de-template: a done leaf that owns the bootstrap,
  its ticked checklist carrying the evidence of that very run (crate-name count before/after, hooks path, the
  enforcer's summary and verdict with `rc=0`), registered in `docs/TASK_TREE.md`, pointed to by `MEMORY.md`; and it
  prints the exact first-commit command as step 0. Idempotent.
- Proven: clone → `bootstrap.sh <name>` → the printed commit → hooks green → `make gate` green → `make check` green,
  with no hand edits. Two defects in the fix were caught by the trial itself (an enforcer run before the map
  existed; a `grep -c` fallback that split a checklist bullet).

## bedrock-scaffold 0.6.0 — four evidence and ratchet doctrines: lessons reach the retrievable layer, routings carry evidence, gap claims carry their census, tables keep their columns

`BEDROCK-MAINTENANCE.2.6`.

- **Added `LESSON-PROMOTION`**: a new dated lesson heading staged in `DEV_NOTES.md` must be promoted (a
  `docs/knowledge/` change or a `docs/decisions/` record gaining `answers:`) or explicitly declined
  (`promotion: declined (<reason>)` in the owning leaf). Pure verdict with 9 controls at import.
- **Added `ROUTING-EVIDENCE`**: a leaf that routes a finding out to another tree carries a `ROUTING EVIDENCE`
  section. Keyed on the semantics of leaving the tree; 5-arm `--self-test`.
- **Added `GAP-CLAIM-CENSUS`**: a leaf that ADDS a "nothing checks X" claim records the census it rests on in
  the same section (or `census: not run (<why>)`). Staged-diff-scoped; `--all` reports the backlog; 10-arm
  `--self-test` pinning the founding active and passive sentences.
- **Added `TABLE-ARITY-RATCHET`** (a fresh minimal implementation): a staged `.md` may not raise the number of
  table rows whose cell count disagrees with their header; code spans and escaped pipes respected; 8-arm
  `--self-test`.
- ⛔ Two defects in the ports were caught by their own RED arms before the gate ran: a heredoc that consumed
  the table detector's stdin (every arm read 0), and a `pipefail` control in lesson promotion.
- All four scripts join the `NEUTRAL` allow-list of `scripts/update_scaffold.sh`. Backlog notes record the
  input-bound principles (`BASELINE-IDENTITY`, `IDENTITY-CARRIER-CURRENCY`, `SCRATCH-SLOT-HEADER`, the full
  `LIVE-DOC-CURRENCY` instrument) for a future seam.

## bedrock-scaffold 0.5.0 — the day-one batch: no agent trailers, a handoff census, no self-reported dates

`BEDROCK-MAINTENANCE.2.5`.

- ⛔ **`COMMIT.md` had the trailer rule backwards.** It told every generated project to *end commit
  messages with the project's co-authorship trailer*; the upstream maintainer ruled the opposite on
  2026-08-22 (a commit message ends with its own last line — no agent/tool attribution trailers,
  harness-agnostic). The rule is rewritten and `.githooks/commit-msg` now refuses the known
  agent-attribution shapes mechanically; a human co-author's `Co-Authored-By:` still passes.
- **Added `scripts/check_no_background_jobs.sh`**, the handoff census: pattern-free (`lsof` over the
  caller's uid — an open handle under the repo, or a command line naming the checkout), run before
  a session ends; deliberately not a commit gate. Named in `CLAUDE.md`'s non-negotiables.
- **Added the `LIVE-DOC-CURRENCY` doctrine** (principle): no tracked `.md` reports its own currency
  (`Last updated:` and kin) — git carries it, a hand-kept date is false the day after. The field is
  deleted from `docs/tasks/TEMPLATE.md` and the maintenance tree; `scripts/check_live_doc_currency.sh`
  is structural over `git ls-files '*.md'` with a 3-arm `--self-test`.
- Both scripts join the `NEUTRAL` allow-list of `scripts/update_scaffold.sh`.
- Part 2 of the same transfer (`LESSON-PROMOTION`, `ROUTING-EVIDENCE`, `GAP-CLAIM-CENSUS`, a fresh
  `TABLE-ARITY-RATCHET`) is classified in the `.2.5` leaf and queued as `.2.6`, paused by the maintainer.

## bedrock-scaffold 0.4.0 — TASK-ACCEPTANCE: a change lands with evidence, not with a claim

`BEDROCK-MAINTENANCE.2.4`.

- **Added the `TASK-ACCEPTANCE` doctrine**: a staged CODE change must be owned by a task-tree leaf
  whose checklist has ROOT CAUSE / ADDRESSED / NO REGRESSION **ticked**, each backed by output from
  a tool that was actually run — **inside that box's own bullet**.
- ⭐⭐ **Box-scoping is the soundness property**, not a nicety. It closes two measured leakage
  holes: a co-staged, unrelated leaf supplying the evidence, and a token matched anywhere in the
  file rather than in the box it backs. `CTRL-1` demonstrates it directly — a whole-file grep
  PASSES the fixture that the shipped check REJECTS.
- **Neutral by seam, not by rename.** Default signatures are universal to any Rust project
  (`error[E1234]`, `could not compile`, `clippy::…`, `test result: ok`, panics, profilers) plus any
  project's build-flow forensics (`git log -S`, `shellcheck`, `bash -n`, `make -n`, `ENOSPC`…).
  Project-specific tooling is declared in `.doctrine/evidence_tokens.txt`, and what counts as a
  code change in `.doctrine/code_paths.txt` — both optional, both defaulted, both documented in
  `.doctrine/README.md`. ⭐ `CTRL-4`/`CTRL-4b` prove the seam is load-bearing: the same leaf passes
  WITH the declaration and fails WITHOUT it.
- ⛔ **Fixed a portability defect the probes caught**: the box extractor used `IGNORECASE`, a gawk
  extension that BSD awk silently ignores — every leaf would have been reported as having no
  checklist. Rewritten with POSIX `tolower()`.
- ⚠️ Honest limit, stated in the check itself: it proves the author cited something re-runnable,
  never that the output is true. The un-fakeable leg is re-running the cited command in CI.
- Probes 9/0; `make gate` 8/8.

## unreleased — the admission test asks about VALUE first, not vocabulary

`BEDROCK-MAINTENANCE.2.3`. Process only; no check changed, so `DOCTRINE_VERSION` is unmoved
(`MAINTAINING.md` and the maintenance tree are maintainer-only, not re-syncable spine files).

- **The admission test is now two ordered questions.** Q1 (primary, about VALUE): *does this
  objectively benefit any present and any future project?* — answered by stating what the check
  prevents using no project's nouns, then asking whether a brand-new project is better off with it
  on day one. Q2 (secondary, a filter): *can it be expressed without domain nouns?*
- ⛔ **Q2 cannot substitute for Q1.** A check can score 0 domain nouns and still encode a workflow
  only one project needs — neutral vocabulary, project-shaped substance. Q2 measures whether a
  thing CAN be neutralized; Q1 asks whether it SHOULD be. Running Q2 first waves impostors through.
- ⭐ **Measured worked example, which changed a verdict.** A "destructive automation must require
  confirmation" check scored well on Q2 and was ranked an easy win; its logic hardcodes a Makefile
  path and a `clean:` recipe, so it really offers *"benefits any project that builds with make"* —
  a conditional. **Rejected as-is.** Meanwhile `ROUTING-EVIDENCE` measures 0 build-system
  references and presumes only the task-tree system this template ships ⇒ promoted to top.
- **The portability seam to look for:** does the check presume anything beyond what bedrock ships?
  If yes, give it a project-declared seam or leave it upstream — never hardcode one project's
  answer and call it neutral.
- ✅ Retroactive audit: all four already-ported items PASS Q1. Nothing retracted.

## bedrock-scaffold 0.3.0 — WAIVER-ROUTING, and the neutrality bar for every future port

`BEDROCK-MAINTENANCE.2.2`.

- **Added the `WAIVER-ROUTING` doctrine** (`scripts/check_waiver_routing.sh`): a task leaf saying a
  gate DOES NOT APPLY must name the leaf that owns fixing the gate. ⭐ An author writing a waiver
  IS the gate reporting a missing capability — the highest-signal defect report a gate can get.
  Deliberately does **not** punish honesty: the waiver stays legal, it just has to name an owner.
- **Chosen by measurement.** All 15 upstream doctrines were classified by domain-dependence of
  their LOGIC (comments stripped). `WAIVER-ROUTING` scored **0** — portable essentially unchanged.
  The ranked remainder is now a frontier in `docs/tasks/BEDROCK-MAINTENANCE.md`, not a wish list.
- ⭐⭐ **The port FIXED a defect rather than inheriting one**: the origin's `printf … | grep -q …
  || continue` returns failure ON SUCCESS past the pipe buffer under `pipefail`, silently SKIPPING
  the file — a **fail-open**. Both sites here read a file instead. Threshold measured, not assumed:
  65,606 B → no SIGPIPE; 131,139 B → SIGPIPE.
- **Wrote down the neutrality bar** (`MAINTAINING.md`): every doctrine here must be objectively
  applicable to ANY project, with a measurable admission test and its honest bound — plus the rule
  that **transfer runs both ways**, after this repo's layer-C check turned out to be stronger than
  the reference deployment's.
- Probes 5/0; `make gate` 7/7; added to the `update_scaffold.sh` NEUTRAL allow-list.

## bedrock-scaffold 0.2.0 — README Stability Policy + a layer-A byte cap

`BEDROCK-MAINTENANCE.2.1`. Transferred from the reference deployment by maintainer order.

- **Added `README_POLICY.md`** (project-neutral, verbatim) — keeps `README.md` a stable landing
  page instead of a changelog/roadmap/catalogue, and states the caps rule.
- **Added the `README-STABILITY` doctrine** (`scripts/check_readme_stability.sh`): a line cap
  AND a byte cap, a dated-line (release-history) tripwire, and a required link back to the
  policy. Non-mutating; REFUSES (exit 2) rather than passing when the README or policy is
  absent. Template defaults 300 lines / 16384 bytes — generous on purpose, because they ship to
  a project whose README is not this one; tighten after your own trim.
- ⛔ **Closed a bypass the spine was itself shipping.** `scripts/check_memory_architecture.sh`
  capped layer-A `MEMORY.md` by LINES only (cap 120, no byte bound), exactly as
  `MEMORY_ARCHITECTURE.md` §9's reference check prescribed — so **every adopting project
  inherited a bound that does not bind.** Measured on a real project running this spine:
  60 lines (passing, exactly at its cap) carrying **138,403 bytes** — 2,306 B/line, one line of
  18,816 B. Now both caps, in the check **and** in the standard (§6 / §9 / §9.1).
  Layer-A caps: **50 lines** (tightened from 120, to match the "≤ ~50 lines" §6 already stated)
  and **7168 bytes**. Both env-overridable.
- Both new files added to the `update_scaffold.sh` NEUTRAL allow-list, so existing projects
  pull them with `scripts/update_scaffold.sh <bedrock-url>`.
- Verified: `make gate` 6/6 green; a 13-line / 19,304-byte fixture is REJECTED by the byte cap
  while being well under the line cap; the **retired** layer-A guard PASSES that same file
  (exit 0) — the change is proven necessary by execution, not by argument.

Changelog-style summary of completed work + its validation (internal continuity surface;
the immutable audit trail proper is `git log` — memory layer D). Newest first.

## _(YYYY-MM-DD)_ — bootstrap

Instantiated from the `bedrock` discipline-spine template. Next: replace `ROADMAP.md` and
seed the first task-tree.
