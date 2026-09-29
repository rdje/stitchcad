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

- ID: `SPINE.1`
  Status: `pending`
  Goal: de-template the identity surfaces — `README.md` as the StitchCAD landing page (within
  the reviewed caps), `docs/book/book.toml` identity, `docs/book/src/introduction.md`, and a
  `SUMMARY.md` skeleton with the `spec/` part that `G0-CONTRACT` fills. Owns defects D3, D4.
  Acceptance: `head -1 README.md` names StitchCAD; `mdbook build docs/book` succeeds; the
  README quick start and links are verified by running them; `README-STABILITY` green.
  Verification: `pending`
  Commit: `pending`

- ID: `SPINE.2`
  Status: `pending`
  Goal: first artifact cleanup + the cadence record `docs/ARTIFACT_CLEANUP.md` (single latest
  entry: date + one-line summary). Owns defect D8.
  Acceptance: cleanup ran on the repository volume only (`target/`, stray `.log`/`.bin`,
  doctrine scratch dirs); a residue census proves what was removed is gone; `make check` still
  green afterwards; nothing tracked was deleted.
  Verification: `pending`
  Commit: `pending`

- ID: `SPINE.3`
  Status: `pending`
  Goal: adopt the revised external policy references into repository-owned copies — refresh
  `README_POLICY.md` (authority/provenance note, duplication probe, routing-pressure closure,
  derived caps, unconditional check) and add the claim-verification standard in-repo, wired
  into the bootstrap reading list. Owns defects D11, D12.
  Acceptance: the adopted copies carry a StitchCAD adoption note and no external absolute
  path; the README guard still passes and additionally validates its routed destinations where
  the revised policy requires it; a decision record names what was adopted and what was
  deliberately not (donor-specific values).
  Verification: `pending`
  Commit: `pending`

- ID: `SPINE.4`
  Status: `pending`
  Goal: adopt live-document size containment for this repository — surface inventory,
  lifecycle class, health target and enforcement ceiling per governed surface
  (`MEMORY.md`, `README.md`, `ROADMAP.md`, `CHANGELOG.md`, `DEV_NOTES.md`, `LIVE_STATUS.md`,
  `docs/TASK_TREE.md`, `docs/tasks/*`, `docs/decisions/*`, `docs/book/src/*`), plus a
  deterministic ratchet check registered in the project doctrine slot. Owns defect D13.
  Acceptance: the checker runs unconditionally in the hook and fails on an unclassified or
  over-ceiling surface; its RED arm is demonstrated (a control that has been seen to fire);
  ceilings are derived from the measured survivor with modest headroom, not copied from a
  donor project; existing pressure is recorded as transition debt with an owner.
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
  Status: `pending`
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
  Verification: `pending`
  Commit: `pending`

- ID: `SPINE.10`
  Status: `pending`
  Goal: keep project-owned scratch on the repository volume (defect **D16**) — the inherited probe
  suites call `mktemp -d`, which resolves to the system volume (`/var/folders/…` here) while the
  repository lives on another volume; pin scratch with a documented, scripted `TMPDIR` and give the
  suites one entry point (`make probes`) so nobody has to remember the incantation.
  Acceptance: `make probes` runs every probe suite with `TMPDIR` inside `target/`; the pin is
  measured (`TMPDIR=… mktemp -d` prints a repository-volume path); all suites still report
  `probes: N pass / 0 fail`; no inherited file is edited (the pin is applied by the caller).
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 0 | `SPINE.6` | `done` | taken out of order: it repaired a dirty-tree defect found on the first `make check` |
| 0 | `SPINE.7` | `done` | taken in frontier order: D15 had to be measured before it could be owned |
| 1 | `SPINE.8` | `done` | landed in frontier order: every code commit from here on is judged by it |
| 2 | `SPINE.9` | `pending` | **next** — the updater can silently destroy the layer-B index; guard it before any scaffold sync is run |
| 3 | `SPINE.10` | `pending` | scratch locality is a standing directive obligation, cheap once `make probes` exists |
| 4 | `SPINE.1` | `pending` | identity before content, so the G0 spec chapters grow into a named book |
| 5 | `SPINE.2` | `pending` | cleanup cadence is a session-directive obligation and cheap |
| 6 | `SPINE.3` | `pending` | policy adoptions bind how every later claim and cap is written |
| 7 | `SPINE.4` | `pending` | containment must exist before 18 spec chapters arrive |
| 8 | `SPINE.5` | `pending` | toolbox rows are honest only once the instruments are in use |

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

Leaves `.1`–`.5`, `.9` and `.10` each add their own `### <leaf-id>` subsection here, in the same
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

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| tree seed | `STITCHCAD-PLANNING-0001 (leaf PLANNING.1)` | created by the seeding leaf |
| `SPINE.6` | `STITCHCAD-SPINE-0006 (leaf SPINE.6): track the workspace lockfile` | fixes D14 |
| `SPINE.7`, `SPINE.8` | `STITCHCAD-PLANNING-0002 (leaf PLANNING.2)` | leaves created to own defect D15 |
| `SPINE.7` | `STITCHCAD-SPINE-0007 (leaf SPINE.7): measure the multi-leaf acceptance-evidence hole` | probe + convention record; D15 published |
| `SPINE.8` | `STITCHCAD-SPINE-0008 (leaf SPINE.8): require acceptance evidence fresh in the commit` | project doctrine + 9-arm probe; D15 facet 1 closed locally |
| `SPINE.1` … `.5`, `.9`, `.10` | `pending` | — |

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
- `2026-09-29`: `SPINE.8` landed — `FRESH-ACCEPTANCE-EVIDENCE` registered in the project slot with
  a 9-arm probe suite; D15 facet 1 is now closed mechanically. A candidate defect (D18, "the
  inherited signature list is not portable") was measured and **withdrawn**: the gate matches with
  `grep -qE`, where both GNU and BSD grep handle the families; only the first implementation of our
  own check, which used awk, was broken.
