# SPINE — acceptance evidence for completed leaves

The evidence half of [`SPINE.md`](SPINE.md), split out under the containment registry's remedy for
`tasks_collection` ("a tree that passes 1000 lines splits its completed-leaf evidence into a sibling file
under `docs/tasks/` before adding more") and under the convention `G0-CONTRACT.4b` recorded, which leaf
`SPINE.4.4` performed as defect **D42** required.

**Why the tree file keeps one checklist.** `scripts/check_task_acceptance.sh` judges EVERY staged
`docs/tasks/*.md` file and refuses one that carries no ticked ROOT CAUSE / ADDRESSED / NO REGRESSION box, so
a tree file emptied of checklists would turn an honest slice into a refusal. The leaf being landed therefore
keeps its checklist in the tree file and the next slice moves it here — which also makes the tree file's
FIRST matching box the current leaf's, closing defect D15's facet 1 by structure rather than by care.

Order is landing order, oldest first, as in the tree's Commit Log.

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

### `SPINE.4.3` — declared ceilings become enforced ones (defect D13 closed)

- [x] **ROOT CAUSE (WHY + WHERE)** — the data plane existed and nothing read it:
  `git ls-tree -r --name-only HEAD | grep -c live_document_size` → `2` (both registries) while
  `git ls-tree -r --name-only HEAD | grep -c check_live_doc_size` → `0` and
  `git show HEAD:scripts/check_doctrines.project.sh` registered `1` project doctrine. A ceiling no
  check reads is a wish: 17 surfaces carried owners, targets and ceilings that no gate would defend.
- [x] **ADDRESSED (verified)** — `scripts/check_live_doc_size.sh` (271 lines) now measures the tree
  (lines, bytes, max content line, and for collections file count / per-part / aggregate, all under
  `LC_ALL=C`) and evaluates every rule in one awk pass: `bash scripts/check_live_doc_size.sh` →
  `live-doc-size: OK — 17 surfaces, 15 routes, 41 files measured, 19 warning(s)`, `exit=0`.
  `--self-test` → `11 arms, 0 failed`, one per refusal class (unclassified surface, unknown lifecycle,
  missing owner, ceiling below health, absolute path, unclassified route destination, widened debt
  baseline, silently empty glob, wrong field count, unparseable registry) plus a GREEN control.
  The end-to-end probe adds what a synthetic registry cannot:
  `bash docs/tasks/artifacts/live_doc_size/run_live_doc_size_probes.sh` → `probes: 4 pass / 0 fail`,
  where `REAL-2` deletes the `roadmap` row from a COPY of the real registry and the check names
  `ROADMAP.md` as an unclassified live surface — coverage proven to have teeth, not assumed.
  Registered as the second project doctrine (`grep -cE '^  "[A-Z-]+\|' scripts/check_doctrines.project.sh`
  → `2`), so it runs in the hook and in CI unconditionally.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `7 suite(s) green` (the new suite joins the six); `make check` → `test result: ok. 1 passed;
  0 failed`. Tuning the targets removed two artefacts of my own target-setting rather than real
  pressure — `roadmap` health set to `-` (a `maintained_reference` aggregate follows product scope, so a
  fixed target would be dishonest; the debt baseline and ceiling govern) and `doctrine_docs` per-part
  health derived from the largest adopted standard (455 lines / 24 573 bytes) plus ~15 % for a local
  note — and one real fix: the widest `LIVE_STATUS.md` row trimmed from `303` to `207` bytes.
  Warnings `21` → `19`, breaches `0`.
- [x] **FIX** — added the checker (bash measures, awk evaluates, so the evaluator is testable on a
  synthetic registry without touching the real tree) and the 4-arm probe suite; registered both in the
  project slot and `TOOLBOX.md`; tuned three registry rows and recorded the derivation for each.
- [x] **LOCKSTEP** — D13 marked fixed in `PLANNING.md`; `LIVE_STATUS.md`, `MEMORY.md`, `CHANGELOG.md`
  updated. Remaining warnings are owned, not ignored: `decisions_collection` maxline 153 % →
  `SPINE.15`; `tasks_collection` per-part 107 % → the convention recorded in its registry row;
  `roadmap` navigation → `SPINE.13`.

### `SPINE.16` — the code-path seam, so prose is not judged as code (defect D25)

- [x] **ROOT CAUSE (WHY + WHERE)** — `scripts/check_task_acceptance.sh` builds its code test from
  `default_code_re='(^|/)(crates|src|scripts)/|\.(rs|sh)$|(^|/)Makefile$'` when
  `.doctrine/code_paths.txt` is absent, and `(^|/)src/` matches this project's documentation tree:
  `git ls-files 'docs/book/src/*.md' | wc -l` → `4`, `rc=0`. Staging
  `docs/book/src/spec/reference-skirt.md` therefore made a prose chapter a CODE change, and
  `scripts/check_fresh_acceptance_evidence.sh` refused the commit demanding an ADDRESSED box backed by
  tool output (`exit=1`) — a false positive of exactly the kind `.doctrine/README.md` warns creates
  waivers.
- [x] **ADDRESSED (verified)** — `.doctrine/code_paths.txt` now declares the classification, and the
  census over representative paths reads: `docs  docs/book/src/spec/ontology.md`, `CODE
  crates/sc-units/src/lib.rs`, `CODE  scripts/check_live_doc_size.sh`, `CODE
  .doctrine/live_document_size/surfaces.tsv`, `CODE  Makefile`, `docs  README.md`,
  `docs  docs/tasks/SPINE.md` — prose is prose, behaviour-altering files are code. Re-derive with the
  seam itself: `PAT="$(grep -vE '^[[:space:]]*(#|$)' .doctrine/code_paths.txt | paste -sd'|' -)"` then
  `printf '%s\n' <path> | grep -qE "$PAT"` for each path — `4` CODE and `3` docs over those seven,
  `rc=0`; the file declares `10` patterns (`grep -vcE '^[[:space:]]*(#|$)' .doctrine/code_paths.txt` →
  `10`, `rc=0`). Both acceptance checks consume the same seam, so they cannot disagree.
- [x] **NO REGRESSION** — the fresh-evidence probe now copies the seam into its throwaway repositories
  (so it tests the real configuration), and `make probes` → `7 suite(s) green`, `exit=0`; `make gate` →
  `=== all doctrines green ===`, `exit=0`; `make check` → `test result: ok. 3 passed; 0 failed`.
- [x] **FIX** — added `.doctrine/code_paths.txt` (10 patterns with the reason recorded in its header);
  one line added to the fresh-evidence probe so it exercises the declared seam rather than the default.
  No spine file was edited: the seam is the documented extension point.
- [x] **LOCKSTEP** — D25 logged in `PLANNING.md` with its reproduce command and this owner;
  `LIVE_STATUS.md`, `MEMORY.md`, `CHANGELOG.md` updated. Taken ahead of the product slice it unblocked,
  per `decision_product-work-takes-the-frontier.md`: a spine slice is legitimate when it blocks the
  product slice about to be taken, and this one was refusing it.

### `SPINE.17` — the push-cadence exception, derived rather than remembered

- [x] **ROOT CAUSE (WHY + WHERE)** — `COMMIT.md`'s Push cadence section had exactly one escape ("the
  director asks"), so a change to CI or to a doctrine check could sit unpushed for 400 commits with no
  runner ever executing it: `git show HEAD~1:COMMIT.md | grep -c 'workflow'` → `0`, `rc=1`. This is not
  hypothetical for this repository — `eb83f01` rewrote `.github/workflows/rust.yml` (adding the
  `wasm32-unknown-unknown` target and the smoketest step) and that workflow had never executed anywhere
  until the exceptional push; and the platform risk is measured, not imagined: BSD awk on this machine
  lacks `\b` and `{n}`, and its `PATH` shadows BSD userland with GNU coreutils, so a gate can be green
  here and behave differently on the ubuntu runner.
- [x] **ADDRESSED (verified)** — `scripts/check_push_due.sh` derives the obligation from git and prints
  it: `bash scripts/check_push_due.sh` → `push-due: branch main vs origin/main — 0 unpushed commit(s),
  cadence 400` / `nothing to push.`, `exit=0`. The **owed** arm, against the revision before the
  workflow change: `PUSH_DUE_BASE=051a075 bash scripts/check_push_due.sh` → `EXCEPTIONAL PUSH DUE —
  6 unpushed file(s) under CI/doctrine paths are unverified by CI` listing `.github/workflows/rust.yml`,
  `scripts/check_live_doc_size.sh`, `scripts/check_doctrines.project.sh` and the three `.doctrine/`
  files, `exit=1`. The refusal arm: `PUSH_DUE_BASE=nope-not-a-ref …` → `REFUSED — … does not resolve to
  a revision`, `exit=2`. `make push-due` is the entry point; `COMMIT.md` carries the rule and the
  command; `TOOLBOX.md` names the question it answers.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0` (the new script is a
  helper, not a registered doctrine: it reports an obligation, it does not judge the tree, so it must
  not be able to block a commit); `make check` → `test result: ok. 3 passed; 0 failed`; `make probes` →
  `7 suite(s) green`; `bash -n` clean.
- [x] **FIX** — added the helper (base defaults to the branch upstream and is overridable, which is what
  makes the "owed" arm testable without inventing unpushed commits), the `push-due` target, the
  `COMMIT.md` exception and the `TOOLBOX.md` row. **The RED arm caught a defect in the first cut:** the
  trigger list used the pathspec `scripts/check_`, which matches nothing because a git pathspec matches
  whole path components unless it carries a wildcard — so every doctrine check script was silently
  missed. It is now `scripts/check_*.sh`, and the file header records the lesson.
- [x] **LOCKSTEP** — the observed CI verdict for the workflow change is recorded in
  `G0-CONTRACT.18`'s Verification Log (`rust` and `doctrines` both `completed success` at `119946b`);
  `LIVE_STATUS.md`, `MEMORY.md`, `CHANGELOG.md` updated. Known over-breadth, stated rather than hidden:
  this helper matches its own trigger pattern although no runner executes it, so landing it owes one
  further push — flagged to the director rather than pushed unilaterally, because the authorisation was
  for one exceptional push.

### `SPINE.18` — a trigger that means "CI must re-verify this", not "the filename looks like a check"

- [x] **ROOT CAUSE (WHY + WHERE)** — `SPINE.17`'s trigger list globbed `scripts/check_*.sh`, so the
  helper itself matched: committing it reported `EXCEPTIONAL PUSH DUE — 1 unpushed file(s) …
  scripts/check_push_due.sh`, `exit=1`, for a file no CI job executes. A standing false obligation is
  worse than no instrument, because the honest response to a warning that always fires is to ignore it.
- [x] **ADDRESSED (verified)** — the registered checks are now **derived** from the two registries
  (`registry_checks()` reads the paths cited in `scripts/check_doctrines.sh` and
  `scripts/check_doctrines.project.sh`, plus the drivers themselves): 17 paths, including
  `scripts/check_live_doc_size.sh` and `scripts/check_fresh_acceptance_evidence.sh`, and excluding this
  helper. Re-observed arms: `bash scripts/check_push_due.sh` → `no push due (1 < 400, no CI/doctrine
  paths touched)`, `exit=0`; `PUSH_DUE_BASE=051a075 …` → `EXCEPTIONAL PUSH DUE — 6 unpushed file(s)`
  naming `.github/workflows/rust.yml`, `scripts/check_live_doc_size.sh`,
  `scripts/check_doctrines.project.sh` and the three `.doctrine/` files, `exit=1`;
  `PUSH_DUE_BASE=nope …` → `REFUSED`, `exit=2`. A newly registered check becomes a trigger with no edit
  here, which is the property a glob cannot have.
- [x] **NO REGRESSION** — `bash -n` clean; `make gate` → `=== all doctrines green ===`, `exit=0`;
  `make check` → `test result: ok. 3 passed; 0 failed`; `make probes` → `7 suite(s) green`.
- [x] **FIX** — replaced the glob with `registry_checks()` and recorded in the file header why the
  derivation exists (with the measured false obligation that motivated it).
- [x] **LOCKSTEP** — `COMMIT.md`'s exception is unchanged in substance (it names the paths that owe a
  push, and the helper now derives the check subset); `TOOLBOX.md`, `LIVE_STATUS.md`, `MEMORY.md`,
  `CHANGELOG.md` updated. No push is owed by this slice, so the 400-commit cadence holds.

Leaves `.5`, `.13` and `.15` each add their own `### <leaf-id>` subsection here, in the same
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

### `SPINE.4.4` — the widest-line target is derived from the cell budget, and D42's split is performed

- [x] **REPRODUCE / ISSUE** — the containment check printed a warning no defect stood behind, on every run:
  `bash scripts/check_live_doc_size.sh` → `live-doc-size: WARNING book_collection: widest line 272 B = 136%
  of its 200 B target`, `exit=0`, while the same row's ceiling is `320` B and the widest line belongs to a
  five-column termbase row in `docs/book/src/spec/glossary/measurements-and-fit.md`. Every glossary part
  exceeded the target; nothing could be done about it except trim definitions or ignore the warning.
- [x] **ROOT CAUSE (WHY + WHERE)** — the `200` B target was derived from the shape of a prose chapter, and a
  termbase row is a different shape: `.doctrine/live_document_size/surfaces.tsv`'s own header says per-part
  health is "derived from the SHAPE of its content … never from today's largest file", and this row's number
  predated the book's first reference table. Measured rather than argued —
  `bash docs/tasks/artifacts/live_doc_size/run_cell_budget_census.sh` → the binding shape is
  `Term｜What it means｜Canonical object｜Also called｜Machine token` with `276 data rows · 5 columns ·
  separator overhead 16 B`, per-column p95 cells `21 + 106 + 62 + 50 + 20 = 259` B, and
  `derived row budget: p95-sum 259 B (+16 overhead = 275 B) · max-sum 363 B (+16 = 379 B)`, `exit=0` — a
  `275` B budget against a `200` B target, and a `379` B worst legitimate row against a `320` B ceiling. The
  second candidate cause was measured instead of assumed: a split row cannot carry the fix as the data plane
  stands, because the checker claims a file with `PARTOF[$2] = sid`
  (`grep -n 'PARTOF\[' scripts/check_live_doc_size.sh` → one hit, in the measurements branch, `rc=0`) so the
  LAST matching row wins, and one registry field holds one glob — a narrower row would double-count the same
  maxima unless the collection's glob stopped matching the termbase parts.
- [x] **ADDRESSED (verified)** — the target is now derived, and its producer is tracked:
  `bash docs/tasks/artifacts/live_doc_size/run_cell_budget_census.sh` →
  `cell budget: 36 shapes / 625 data rows measured / recommended maxline health 275 B`, `exit=0`, printing
  every shape's per-column widest and p95 cells, both derived row budgets (`p95-sum 259 B (+16 overhead =
  275 B) · max-sum 363 B (+16 = 379 B)` for the termbase), and the honesty check
  `covers the population's widest actual line (272 B): yes`. Its arithmetic is pinned on a synthetic table
  whose budget is known by construction: `--self-test` → `probes: 7 pass / 0 fail`, `exit=0`. The registry row
  now reads health `275` / ceiling `440` with the derivation, the instrument and the measured widest line in
  its `notes` column, and the check reports the change: `WARNING book_collection: widest line 272 B = 99% of
  its 275 B target`. The ceiling rise is authorised by
  `docs/decisions/decision_maxline-health-derived-from-the-cell-budget.md` (indexed, `answers:` line), which
  also records the two rejected alternatives with their arithmetic — raising health to `379` to silence the
  warning, and the split row — so neither is re-litigated by a reader who has not measured them.
- [x] **NO REGRESSION** — `bash scripts/check_live_doc_size.sh --self-test` → `live-doc-size --self-test: 11
  arms, 0 failed`; `bash docs/tasks/artifacts/live_doc_size/run_live_doc_size_probes.sh` →
  `probes: 4 pass / 0 fail`; `bash scripts/check_live_doc_size.sh` → `live-doc-size: OK — 17 surfaces,
  15 routes, 80 files measured, 32 warning(s)`, `exit=0`, with no breach and the registry still parsing at
  `21` fields per row; `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `make probes: 12 suite(s) green`, `exit=0`; the book's own censuses are untouched and green (glossary
  `276 terms / 0 failure(s)`, matrix `105 rows / 0 failure(s)`, fixture `0 mismatch(es)`). No Rust changed;
  the one staged `.sh` is a new measurement tool, so this checklist is the evidence the code-path seam asks
  for.
- [x] **FIX** — wrote the cell-budget census (a portable file list rather than `mapfile`, because macOS ships
  bash 3.2; cells split honouring code spans and escaped pipes exactly as `check_table_arity.sh` splits them,
  so the two instruments agree about where a cell ends); edited the two numbers and the `notes` column of one
  registry row; wrote the decision record; added the `TOOLBOX.md` row; and performed the evidence split D42
  assigned to this leaf — `17` completed checklists moved byte-identically out of `docs/tasks/SPINE.md`
  (`1096` lines / `88 341` B → `552` lines / `42 028` B, inside its `800` / `65 536` health) into
  `docs/tasks/SPINE-evidence.md`, under the convention `G0-CONTRACT.4b` recorded.
- [x] **`.doctrine/` changed, so the exceptional push is owed and the verdict is recorded, not assumed** —
  `make push-due` names the trigger set this commit touches, the push happens immediately after the commit,
  and the observed CI verdict is written into this leaf's Verification Log row in the commit that follows,
  which is the shape `G0-CONTRACT.18`'s first wasm-smoketest push established.
- [x] **The changelog rollover this slice's append triggered is performed here, and it exposed a trap worth
  more than the rollover (defect D43).** The live window had crossed its byte health (`376` lines /
  `33 364` B against `400` / `32 768`), so slices 32–33 are sealed into
  `docs/history/stitchcad-changelog-part6.md` (`93` lines / `8 284` B / `sha256:3148dd0fb5f63088…`), proved
  byte-identical to `git show HEAD:CHANGELOG.md` rather than to memory, and the window is back inside health
  at `283` lines / `25 246` B. The first digest it declared did not reproduce, and the cause was one newline:
  the sealed content ended with a blank line, which `content=$(sed …)` cannot represent because bash strips
  trailing newlines — so the verifier reported what looks like drift in an immutable archive. Fixed by
  normalizing the segment, recomputing its descriptor by the verifier's own method, teaching `DESCRIPTOR` to
  refuse a trailing blank line BY NAME, writing the byte contract into the probe's header, and pinning it with
  a `TRAILING-BLANK` arm: `run_changelog_ledger_probes.sh` → `probes: 9 pass / 0 fail`, `exit=0`.
- [x] **LOCKSTEP** — D42 closed and D43 logged-and-fixed in `docs/tasks/PLANNING.md`; D38's entry gains the
  second measurement of a hand-kept count being wrong by one (`40` written where `41` ids exist), which is the
  argument for the instrument `PLANNING.5` owes; `TOOLBOX.md` gains the census row;
  `docs/decisions/INDEX.md` carries the record and the Knowledge Map was regenerated (`make gate` refused
  until it was); `docs/TASK_TREE.md`'s frontier cells and execution-order line, `LIVE_STATUS.md`,
  `MEMORY.md` and `CHANGELOG.md` (including the rollover) updated in this commit. Lesson promotion:
  **promoted** — the new record carries an `answers:` line.

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

### `SPINE.15` — the table convention is settled by a rendered page, and both prose-derived targets are re-derived

- [x] **REPRODUCE / ISSUE** — defect D22 was logged as a *question* because nobody had rendered the case: the
  inherited `scripts/check_table_arity.sh` documents its cell rule as "pipes NOT inside an inline code span"
  and self-tests it (`bash scripts/check_table_arity.sh --self-test` → `arm ok  a pipe inside a code span is
  not a separator (0)`, `exit=0`), while `DOCTRINE_ENFORCEMENT.md` warns that GFM "silently DROPS extra
  cells". Two tracked instruments disagreed about what a row means, and the repository's convention depended
  on which was right.
- [x] **ROOT CAUSE (WHY + WHERE)** — the checker was written against a reading of the specification, and no
  oracle in the tree ever rendered a page. Settled by rendering one: a scratch mdBook with a 3-column table
  whose first data cell carries `` `x | y` `` and whose second carries `` `x \| y` ``, built and parsed.
  `mdbook build` → `INFO HTML book written to …`, `exit=0`, and the rendered rows are
  `3 cell(s): A raw pipe in a code span: `x │ y` │ 2` and `3 cell(s): B escaped pipe in a code span: x | y │ 2 │ 3`.
  So the renderer **splits at the raw pipe**, breaks the code span open, shifts the cells and **drops the
  rightmost one** — the checker's rule is wrong for the renderer this book ships through, and the loss is
  silent: no diagnostic, and the source still reads correctly. The first cut of this oracle asserted "2
  cells" from a 2-column page and went red on the 3-column truth, so the arm now asserts the *property* (the
  first cell truncated at the pipe, the last cell not the one written) rather than a count.
- [x] **ADDRESSED (verified)** — the oracle is tracked, so the answer is re-runnable rather than remembered:
  `bash docs/tasks/artifacts/table_render/run_table_render_probes.sh` → `probes: 3 pass / 0 fail`, `exit=0`,
  printing the rendered rows, and its third arm pins the divergence by requiring the inherited checker's own
  self-test to still assert the opposite. The convention is written where authors look — `COMMIT.md`'s
  lockstep list gains **Table authoring**: escape every pipe in a table cell including inside code spans; a
  cell is not a paragraph, so content that outgrows its column becomes a bounded subsection; and a maxline
  target is a shape budget whose 80 % warning means *at budget*. Both remaining prose-derived maxline targets
  are re-derived with the `SPINE.4.4` instrument rather than guessed:
  `CELL_BUDGET_GLOB='docs/decisions/*.md' bash docs/tasks/artifacts/live_doc_size/run_cell_budget_census.sh` →
  `8 shapes / 47 data rows measured / recommended maxline health 382 B`, `exit=0` (binding shape: the index's
  `Record｜Type｜One-line hook` row, 17 rows), and the same over `docs/tasks/*.md` →
  `13 shapes / 259 data rows measured / recommended maxline health 443 B`, `exit=0` (binding shape: the
  verification log's `Date｜Leaf｜Checks｜Result` row, 52 rows). The registry now carries `382` and `443`,
  the `decisions_collection` `maxline=491` debt is **cleared**, and
  `bash scripts/check_live_doc_size.sh` → `live-doc-size: OK — 17 surfaces, 15 routes, 87 files measured`,
  `exit=0`, with `decisions_collection: widest line 353 B = 92% of its 382 B target` and
  `tasks_collection: widest line 443 B = 100% of its 443 B target`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `14 suite(s) green`, `exit=0` (thirteen before this leaf; the fourteenth is the render oracle);
  `bash scripts/check_live_doc_size.sh --self-test` → `15 arms, 0 failed`; the containment probe suite →
  `probes: 5 pass / 0 fail` including `REAL-3`; every book census green and unchanged — glossary
  `276 terms / 8 parts / 145 tokens / 0 failure(s)`, matrix `105 rows / 29 diagnostics / 0 failure(s)`,
  standards `6 registered / 6 designations used / 0 failure(s)`, fixture
  `20 derived rows / 4 closure checks / 5 pieces / 0 mismatch(es)`, coverage
  `10 lanes / 13 trees / 3 sibling(s) / 0 unowned / 0 orphan(s) / 0 dead link(s)`; `make book` → `exit=0`.
  The inherited checker was **not** modified: `git diff --stat HEAD -- scripts/check_table_arity.sh` → empty,
  `rc=0`, because a NEUTRAL spine file is reported upstream and never patched locally.
- [x] **FIX** — built the render oracle as a tracked probe (refusing with `exit=2` when `mdbook` is absent,
  rather than reporting green over a page nobody rendered); wrote the convention into `COMMIT.md`; re-derived
  both targets and cleared the debt; and removed the fat that the old prose-derived targets had been blamed
  for — the `491` B `Class｜Behaviour｜Files` row in
  `decision_scaffold-sync-protects-project-content.md` became two bounded bullets (the house remedy), nine
  index hooks that had grown into three-line summaries are one line each again, and four verification-log
  rows above the derived budget were tightened. The target moved *and* the rows moved: raising a number to fit
  verbosity and trimming rows to fit a guess are both wrong, and this leaf did neither alone.
- [x] **The upstream question is reported, not patched, and the local gap is owned.** D22 closes with the
  renderer's answer recorded; **D47** records that nothing in this repository mechanically refuses the row the
  renderer truncates (the inherited checker reports `0` defects for it), and **`SPINE.20`** owns the
  project-slot check, with the render probe as its ground truth. A convention in `COMMIT.md` is what authors
  read; a gate is what holds when they do not, and this repository's own doctrine says a rule that lives only
  in a doc is a suggestion.
- [x] **LOCKSTEP** — D22 closed and D47 logged in `docs/tasks/PLANNING.md`; `SPINE.20` created here;
  `COMMIT.md` gains the convention; `TOOLBOX.md` gains the render oracle; the containment adoption record's
  trigger 3 was already marked fired by `SPINE.4.5` and its max-axis note is cited by the new convention;
  `LIVE_STATUS.md`, `MEMORY.md`, `CHANGELOG.md` and `DEV_NOTES.md` updated in this commit; the index carries
  `docs/decisions/decision_table-cells-escape-pipes-render-to-settle.md`. Lesson promotion: **promoted** — that
  new record carries the rendered evidence and an `answers:` line, and
  `decision_maxline-health-derived-from-the-cell-budget.md` gains the max-axis consequence this leaf measured.

### `SPINE.20` — the table convention becomes a gate, because a rule in a doc is a suggestion

- [x] **REPRODUCE / ISSUE** — `SPINE.15` settled D22 against a rendered page and left the answer enforced by
  nothing: the convention lived in `COMMIT.md` prose, and the only gate in the tree that reads table cells
  asserts the opposite. Both halves measured — `bash docs/tasks/artifacts/table_render/run_table_render_probes.sh`
  → `probes: 3 pass / 0 fail`, printing `3 cell(s): A raw pipe in a code span: `x|y`|2` for a row written with
  four cells; and `bash scripts/check_table_arity.sh --self-test` → `arm ok  a pipe inside a code span is not a
  separator (0)`, `exit=0`. So a row that loses a column in the built book passes every gate, and
  `grep -c TABLE-CODE-PIPE scripts/check_doctrines.project.sh` at `HEAD` → `0`, `rc=1`.
- [x] **ROOT CAUSE (WHY + WHERE)** — the inherited `scripts/check_table_arity.sh` was written against a reading
  of the specification rather than a renderer, and the assumption is load-bearing in four places:
  `grep -n 'code span' scripts/check_table_arity.sh` → lines `20`, `22`, `73`, `99`, `rc=0`, where `20` states
  the cell rule ("and NOT inside an inline code span") and `73` is the self-test arm that pins it
  (`t 0 "a pipe inside a code span is not a separator"`). It is also NEUTRAL spine code: re-synced by
  `scripts/update_scaffold.sh`, so patching it locally would be silently overwritten or silently diverge (the
  rule `SPINE.7`/`SPINE.8` established for defect D15). A defect in an inherited check is therefore fixed in the
  **project slot**, which can only add refusals — and until this leaf nothing there read table cells:
  `grep -c 'PROJECT_DOCTRINES' scripts/check_doctrines.project.sh` at `HEAD` listed two entries, neither of
  them about tables.
- [x] **ADDRESSED (verified)** — `scripts/check_table_code_pipes.sh` is registered as the project doctrine
  `TABLE-CODE-PIPE` and mirrored in `DOCTRINE_ENFORCEMENT.md`. Its arms:
  `bash scripts/check_table_code_pipes.sh --self-test` → `table-code-pipe --self-test: 7 arms, 0 failed`,
  `exit=0` — a raw pipe refused, an escaped one accepted, an ordinary separator accepted (the arity checker's
  business, not this one's), a quoted row inside a fence accepted (documentation is not a table), a
  double-backtick span refused, prose accepted (no cell to split), and an indented row refused. The gate fires
  in the real hook path, demonstrated rather than assumed: staging a scratch file holding
  `| `x | y` | 2 | 3 |` and running the enforcer → `PROJECT TABLE-CODE-PIPE: BREACH (exit=1)` with
  `docs/_pipe_demo.md:5 — a raw `|` inside a code span splits the cell and the renderer drops the rightmost
  one` and the escaped form to write instead; the file was then unstaged and deleted
  (`git status --short | wc -l` → `8`, the slice's own edits). `bash scripts/check_doctrines.project.sh` →
  `PROJECT-SPECIFIC: 3 project doctrine(s) green`, and `make gate` → `=== all doctrines green ===`, `exit=0`.
- [x] **NO REGRESSION** — the gate is **absolute, not a ratchet**, and that was measured before choosing: a
  scan of every tracked `.md` for the shape it refuses, re-runnable rather than remembered —
  `bash scripts/check_table_code_pipes.sh --all` → `table-code-pipe --all: 88 tracked .md files, 0 offending
  table rows`, `exit=0` — so no existing file blocks and no baseline had to be grandfathered. The `--all` mode
  exists for exactly this claim, on the precedent of `check_gap_claims.sh --all`: a count quoted from an
  untracked scan is a memory, and this repository has already shipped that breach once (defect D20). `make probes` → `14 suite(s) green`,
  `exit=0`, including the render oracle that is this check's ground truth; the inherited checker is untouched —
  `git diff --stat HEAD -- scripts/check_table_arity.sh` → empty, `rc=0`; `make book` → `exit=0`;
  `bash scripts/check_live_doc_size.sh` → `live-doc-size: OK — 17 surfaces, 15 routes, 89 files measured`,
  `exit=0`; every census green (glossary `276 terms`, matrix `105 rows`, standards `6 registered`, fixture
  `20 rows / 4 checks / 5 pieces`, coverage `13 trees / 3 sibling(s)` — all `0 failure(s)`).
- [x] **FIX** — wrote the check (staged-scoped like every sibling doctrine, fence-aware, code-span aware for
  any backtick run per CommonMark, with the escaped form printed in the refusal so the fix is one edit away);
  registered it in the project slot's array and its human-readable mirror; mirrored it in
  `DOCTRINE_ENFORCEMENT.md` with the reason and the divergence from the inherited check; added the `TOOLBOX.md`
  row; re-derived the doctrine and probe counts in `LIVE_STATUS.md` instead of incrementing them by hand
  (`scripts/check_doctrines.sh | grep -c '✅'` → `13` printed rows = 12 universal including the conditionally
  appended `KNOWLEDGE-MAP` plus the project row; `check_doctrines.project.sh` → `3 project doctrine(s)`;
  `find docs/tasks/artifacts -name 'run_*probe*.sh' | wc -l` → `14`); moved `.15`'s checklist to the evidence
  sibling per the convention `G0-CONTRACT.4b` recorded.
- [x] **The upstream report is the other half of the fix.** The inherited check's rule is wrong for the renderer
  this repository ships through, and the honest disposition of a defect in NEUTRAL code is a report, not a local
  patch: the divergence is recorded in `DOCTRINE_ENFORCEMENT.md`'s project table and in
  `docs/decisions/decision_table-cells-escape-pipes-render-to-settle.md`, with the oracle that settles it, so
  the report carries evidence rather than an opinion.
- [x] **LOCKSTEP** — D47 closed in `docs/tasks/PLANNING.md`; this tree's leaf, frontier, verification log,
  commit log and changelog; `DOCTRINE_ENFORCEMENT.md`, `TOOLBOX.md`, `LIVE_STATUS.md`; `MEMORY.md` and
  `CHANGELOG.md` in this commit. Lesson promotion: declined (the rule — a convention needs a gate, and a
  defect in inherited code is fixed in the project slot — is already layer C:
  `decision_table-cells-escape-pipes-render-to-settle.md` and the D15 record; a third copy would be a
  duplicate, and no new dated lesson was added to `DEV_NOTES.md` this slice).

### `SPINE.21` — the cadence runs, and the residue census proves what it took

- [x] **REPRODUCE / ISSUE** — the cadence record was 23 hours old at the start of this run and the volume
  had grown: `du -sk target docs/book/book` → `40648` and `4120`, with `find target -name '*.bin' | wc -l`
  → `57` incremental-cache files. `docs/ARTIFACT_CLEANUP.md`'s latest entry was `SPINE.2`'s run of
  `2026-09-29`, so the session directive's §8 obligation ("more than 24 hours old … run a cleanup during
  this session") was about to fire mid-slice with no leaf owning it.
- [x] **ROOT CAUSE (WHY + WHERE)** — `SPINE.2` discharged the *first* cleanup and wrote the record, but a
  cadence is a recurring obligation and the tree had no recurring leaf for it: `grep -c 'cleanup'
  docs/tasks/SPINE.md` matched only `.2`'s block. So each later run would either be unowned (a change with
  no leaf, which the code-change doctrine forbids) or folded into whatever slice happened to notice the
  date — which is how a hygiene action ends up inside a product commit.
- [x] **ADDRESSED (verified)** — nine paths removed, each named by the residue census and each found gone:
  `target/doctrine_scratch`, `target/scratch`, `target/tmp`, `target/debug/incremental`,
  `target/wasm32-unknown-unknown/debug/incremental`, `docs/book/book`, and three scratch bodies the
  containment self-tests had left in `target/`. Measured: `target` `40 648` KB → `10 808` KB (`33 960` KB
  off the volume, counting the book's `4 120` KB). Nothing tracked was touched:
  `git ls-files | grep -cE '^(target/|docs/book/book/)'` → `0` before and after,
  `git ls-files | grep -cE '\.(log|bin|tmp|orig|rej)$'` → `0`, `git status --porcelain | grep -c '^ D\|^D'`
  → `0`, and `find . -path ./.git -prune -o \( -name '*.log' -o -name '*.bin' -o -name '*.tmp' -o
  -name '*.orig' -o -name '*.rej' -o -name '.DS_Store' \) -print | wc -l` → `0`.
- [x] **NO REGRESSION** — every gate re-run after the removal, which is the point of the exercise:
  `make gate` → `=== all doctrines green ===`; `make check` → `test result: ok. 1 passed; 0 failed`;
  `make book` → `INFO HTML book written to …` with `docs/book/book` regenerated at exactly `4 120` KB;
  `make probes` → `20 suite(s) green` with `target/scratch` recreated by the Makefile's own rule;
  `make wasm` → `wasm-viewer smoketest: sc-units + sc-core build for wasm32-unknown-unknown`. `target`
  rebuilt to `13 460` KB, i.e. the incremental caches returned as the record promises.
- [x] **FIX** — removed the nine paths; overwrote the record's single latest entry with the absolute date
  and time, the byte deltas, the residue census result and the gates re-run; created this leaf so the
  cadence has a recurring owner; corrected `docs/TASK_TREE.md`'s `SPINE` frontier cell, which still named
  `.20` as open one commit after it landed (D34's fourth instance).
- [x] **LOCKSTEP** — this leaf, its frontier row, the tree's three logs; `docs/ARTIFACT_CLEANUP.md`,
  `docs/TASK_TREE.md`, `CHANGELOG.md` and `docs/tasks/PLANNING.md` (D34's recurrence) in this commit.
  `MEMORY.md` and `LIVE_STATUS.md` are unchanged: a cleanup moves no product frontier and closes no area.
  Lesson promotion: declined (no new dated lesson — the run is a cadence discharge, and the reusable rule
  "a recurring obligation needs a recurring leaf" is recorded in this leaf's goal rather than duplicated).

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
