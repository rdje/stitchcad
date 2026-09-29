# CHANGELOG.md

Newest first. The StitchCAD sections are this project's history; everything below the
_Inherited spine history_ divider is the bedrock scaffold's own changelog, kept as the
provenance of the discipline spine this repository was generated from.

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

## STITCHCAD-SPINE-0004b — the containment data plane, and the trims that make its numbers honest (leaf `SPINE.4.2`)

- **Two bounded TSV registries** now classify every live surface and every route:
  `.doctrine/live_document_size/surfaces.tsv` — **17** surfaces × **21** fields (path, kind, lifecycle
  class, owner, authority, measured lines/bytes/max-line/file-count, health target, inclusive ceiling,
  transition debt, derivation notes) — and `routes.tsv` — **15** routes × **8** fields (emitter,
  destination, governing surface, reader-vs-author class, lifecycle, pressure control). Route→surface
  closure is enumerated, so no bounded file routes its overflow into an unclassified sink: the failure
  the adopted policy's routing-pressure section exists to stop.
- **Three surfaces were trimmed BEFORE their targets were set**, because a ceiling fitted to today's
  bloat is not a ceiling: `MEMORY.md` went 38 → 28 lines / 2 633 → 1 780 bytes by moving its priority
  queue and defect roster back to layer B, where they already lived; `LIVE_STATUS.md`'s widest line
  went 1 104 → 146 bytes by making notes cells pointers instead of paragraphs; `docs/tasks/PLANNING.md`'s
  widest line went **1 758 → 255 bytes** by converting the defect census from a five-column table to
  bounded per-defect entries. Max-content-line is a separate pressure axis precisely because a wide row
  is invisible to a line-and-byte cap.
- **Derived README caps recorded:** 160 lines / 9 216 bytes / 320-byte line, health 120 / 7 168, from
  the trimmed survivor plus modest headroom — replacing the inherited 300 / 16 384 template defaults as
  the binding limit (the neutral guard still runs as a looser backstop; the stricter binds).
- **Two defects in this leaf's own census, caught by running it:** the first pass reported a missing
  surface row and a missing route because the command stripped `^#` lines and then dropped another —
  and the header itself starts with `#`. The instrument was wrong, not the registry. The second was
  real: the `git_history` row carried 20 fields instead of 21. Both fixed; both are what `SPINE.4.3`
  must refuse mechanically.
- **Three new leaves own the remaining debt** rather than leaving it as prose: `SPINE.13` (roadmap
  navigation index + per-section bounds), `SPINE.14` (seal the inherited bedrock changelog segment into
  `docs/history/` as an immutable archive terminal, with hash-proven identity), `SPINE.15` (settle D22
  against a real GFM renderer and adopt the wide-row convention).
- **Two defects logged:** D21 (a raw `|` inside a code span in a table cell — fixed by the conversion)
  and D22 (the inherited arity checker documents "a pipe inside a code span is not a separator", which
  is not what the GFM spec text says; logged as a question with an oracle, not as a claim).
- Declared rows are **not yet gated**: enforcement is `SPINE.4.3`. Validation of this slice:
  `make gate` → `=== all doctrines green ===`; `make check` → `test result: ok. 1 passed; 0 failed`;
  `make probes` → `6 suite(s) green`; `0` absolute paths in the data plane.

## STITCHCAD-SPINE-0004 — the containment doctrine is in-repo, with its deferrals named (leaf `SPINE.4.1`)

- **`LIVE_DOCUMENT_SIZE_CONTAINMENT.md` adopted** at the repository root: the external doctrine's
  neutral body verbatim (`342` lines) behind a fenced StitchCAD adoption note — `386` lines /
  `23 712` bytes, `12` sections, `0` donor nouns (`grep -ciE 'fsmgen|nexsim|\bisf\b|ppif'` → `0`) and
  `0` absolute paths. The file is not in the scaffold sync list, so a spine update cannot revert it.
- **The adoption is PARTIAL and says so.** Adopted now: the doctrine, the surface inventory with
  lifecycle classes, measured pressure axes, health targets, inclusive ceilings, the routing-destination
  inventory and one deterministic checker in the project slot (`SPINE.4.2`, `SPINE.4.3`). Deferred: the
  neutral JSONL checker package (~2 100-line interpreter plus the archive-descriptor, ledger-manifest,
  derived-state, ceiling-authority and version-retention registries), because this repository has no
  partitioned archive, no rolling ledger and no derived-state copies yet, and because it would put a
  non-Rust interpreter in a Rust workspace's commit path. Three triggers reopen the decision, each owned
  by whoever hits it.
- **Local parameters:** warn at 80 % of a health target; the enforcement ceiling is inclusive (equality
  passes, excess fails); a ceiling rises only by a recorded authority; the data plane is bounded TSV
  parsed with awk, so nothing new enters the hook path. Recorded as
  `docs/decisions/decision_live-document-containment-proportionate-adoption.md`.
- **Why now, measured:** `wc -lc ROADMAP.md CHANGELOG.md docs/tasks/SPINE.md` → `919`/`50821`,
  `460`/`36698`, `772`/`60433`. The last is a task-tree file whose per-leaf evidence sections are a
  scaling term — the exact shape containment exists to bound, with 18 specification chapters still ahead.
- `SPINE.4` was split into `.4.1`–`.4.3` so choosing the ceilings and enforcing them are separately
  verifiable slices; discovery is wired from `COMMIT.md` (where an author updating live docs looks) and
  cross-linked from `README_POLICY.md`'s adoption note.
- Validation: `make gate` → `=== all doctrines green ===`, `exit=0`; `make check` →
  `test result: ok. 1 passed; 0 failed`.

## STITCHCAD-SPINE-0011 — a published number gets its producer back (leaf `SPINE.11`)

- **Defect D20 fixed.** `SPINE.8`'s record publishes "awk left 12 of 36 corpus evidence lines
  unmatched where `grep -qE` leaves 2", but the corpus that produced it lived in untracked scratch and
  the `SPINE.2` cleanup removed it — two tracked documents quoting a number no command could re-derive,
  which is exactly the leg-3 (durability) breach `CLAIM_VERIFICATION.md` §3 names. Found by that
  standard's own adoption sweep (§7 step 3, untracked producers), one leaf after adopting it.
- **The instrument is tracked and watched:** `docs/tasks/artifacts/evidence_signatures/` now holds
  `evidence_corpus.txt` (the 36 lines the claim is stated over) and
  `run_signature_portability_probe.sh`, which reads the signature list out of the universal check
  (never a fork), reports per-line verdicts under both engines, and reproduces the published numbers:
  `corpus: 36 lines · grep-unmatched: 2 · awk-unmatched: 12`, `probes: 5 pass / 0 fail`, `exit=0`.
  The constants are gated: if they drift the probe refuses and prints the instruction to re-derive the
  claim in `SPINE.8` and `CHANGELOG.md` rather than retune the number — §5B applied to our own claim.
- Arms include a control that the census discriminates (a prose line with no tool output matches under
  neither engine) and one that the two grep-unmatched lines are the documented bad samples, so a third
  would mean a real evidence family died.
- `make probes` now discovers six suites (`9`, `7`, `6`, `10`, `5`, `5` arms, all `0 fail`); `TOOLBOX.md`
  names the new instrument and the question it answers.
- **A wrong number caught before shipping, again:** the first draft of this slice's census line claimed
  "20 rows (D1–D20)"; re-running `grep -cE '^\| D[0-9]+ ' docs/tasks/PLANNING.md` gives **19** — there is
  no D18 row, because that candidate was measured and withdrawn before being logged. The census line now
  states the count, the command, and why the numbering skips.
- Validation: `make gate` → `=== all doctrines green ===`; `make check` → `test result: ok. 1 passed;
  0 failed`; `bash -n` clean on the new script.

## STITCHCAD-SPINE-0003 — the external policy references are repository-owned now (leaf `SPINE.3`)

- **`README_POLICY.md` refreshed** to the revised project-neutral body (defect D11): `71` lines /
  `2 920` bytes and four sections → `190` lines / `10 535` bytes and seven, adding *Authority and
  provenance*, the duplication probe before deleting apparent duplication, and *Routing pressure
  closure* (every route needs an owner, lifecycle class and pressure control, followed transitively).
  A fenced StitchCAD adoption note above the body records authority, independence (the origin is a
  read-only source, not an upstream), the reviewed measurement (`README.md` at 103 lines / 6 063 bytes),
  the ceilings currently in force, the routed destinations and the **adoption frontier**.
- **`CLAIM_VERIFICATION.md` adopted** (defect D12): `330` lines / `21 793` bytes, the neutral body
  verbatim, with an adoption note that restates all three legs in this project's terms rather than
  copying examples — re-derive by re-running the producing command; falsify with an oracle we did not
  build (an independent engine re-importing `.rul`, a real importer with recorded settings, a ruler on
  a printed scale square, blinded defective assemblies, a non-shipped SMT oracle); make durable with a
  tracked producer. §4's claim tag maps onto the rule this repository already enforces mechanically:
  invocation + output + exit status, fresh in the commit. `CLAUDE.md` and the README now route to it.
- **Donor values were not copied** — no caps, measurements, surface identifiers, task ids or registry
  paths: `grep -ciE 'fsmgen|0024|0038|0040|0041|0044|surfaces\.jsonl|routed_destinations' README_POLICY.md`
  → `0`, and `grep -cE '/(Users|home|Volumes)/'` → `0` in both files, so nothing here depends on a path
  outside the repository. Recorded as `docs/decisions/decision_adopted-external-policy-references.md`.
- **The adoption's own sweep found a real breach (defect D20, owned by `SPINE.11`):** the published
  `12 of 36` awk-versus-grep signature measurement in `SPINE.8`'s record has no producer left — the
  corpus lived in untracked scratch and the `SPINE.2` cleanup removed it. Two tracked documents quote
  a number no command can re-derive, which is precisely the leg-3 failure the standard exists to stop.
- **Obligations adopted but not yet met are tracked, not hidden:** the revised policy forbids copying
  illustrative caps and requires a destination registry; both are recorded in the adoption note as owed
  by `SPINE.4`, which is why that leaf's scope grew.
- Validation: `scripts/check_readme_stability.sh` → `README-STABILITY: OK — README.md is 103/300
  lines, 6063/16384 bytes.`, `exit=0`; `make gate` → `=== all doctrines green ===`; `make check` →
  `test result: ok. 1 passed; 0 failed`.

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

# Sealed archive — inherited spine history

The bedrock scaffold's own changelog (the provenance of this repository's discipline spine) was sealed
out of this file into
[`docs/history/bedrock-scaffold-changelog.md`](docs/history/bedrock-scaffold-changelog.md) —
158 lines, 11811 bytes, `sha256:78f43e0fe24c60f7…`, immutable.
StitchCAD's own history is everything above this line, newest first.
