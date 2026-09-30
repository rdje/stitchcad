# Sealed archive — StitchCAD defect census, the closed defects D1–D52

Immutable historical segment, sealed out of the defect census in `docs/tasks/PLANNING.md` under the
remedy defect **D46** prescribes and **D49**'s trigger requires: the census had become that tree's
dominant mass, and a closed defect is exactly the content an archive terminal is for — cited by id,
immutable once fixed, and not what a reader needs in order to know what to do next.

- **Sealed identity:** 624 lines, 53323 bytes, `sha256:1897bde0777d71caecfceb2eef09e7c0666164efc91c322f894ca9ce23b32cc4`
- **Sealed by:** leaf `SPINE.19.1` on `2026-09-30`, at the trigger D49 declares (the live file had
  reached 95 % of its `tasks_collection` byte ceiling).
- **Coverage:** `D1`, `D2`, `D3`, `D4`, `D5`, `D6`, `D7`, `D8`, `D9`, `D10`, `D11`, `D12`, `D13`, `D14`, `D15`, `D20`, `D19`, `D21`, `D25`, `D23`, `D22`, `D47`, `D24`, `D16`, `D17`, `D26`, `D27`, `D28`, `D29`, `D30`, `D31`, `D32`, `D33`, `D36`, `D37`, `D39`, `D41`, `D42`, `D43`, `D44`, `D45`, `D48`, `D50`, `D52`.
- **Order:** the order the census carried them, which is discovery order within a session and not numeric.
- **Retrieval:** this file, `git log -S'<id>' docs/tasks/PLANNING.md` for the revisions before the seal, or
  the id itself — every reference to a sealed defect elsewhere in the tree resolves here.
- **Write policy:** none. A closed defect is immutable; a correction is a superseding note in the live
  census, never an edit here.

---

- **D1** — layer-B index lists a tree that does not exist (`BEDROCK-MAINTENANCE`, plus a
  `MAINTAINING.md` reference) and omits one that does (`BOOTSTRAP`)
- Reproduce: `grep -oE 'tasks/[A-Za-z0-9_-]+\.md' docs/TASK_TREE.md \| sort -u` vs `ls docs/tasks/*.md`
- Impact: a resuming agent is pointed at a missing tree and cannot see the real one
- Owner: `PLANNING.1` (fixed)

- **D2** — layer-A pointer reports `Latest commit: none yet` although the bootstrap commit landed
- Reproduce: `git log --oneline -1` vs `grep -n 'Latest commit' MEMORY.md`
- Impact: the resume pointer misreports state — the exact drift layer A exists to prevent
- Owner: `PLANNING.1` (fixed)

- **D3** — `README.md` still introduces the bedrock template, not StitchCAD
- Reproduce: `head -1 README.md` → `# bedrock — a Rust project discipline-spine template`
- Impact: `COMMIT.md` requires README to carry objective/layout/commands; a visitor reads the wrong
  project
- Owner: `SPINE.1` (**fixed** — the landing page now names StitchCAD; `README-STABILITY: OK — 103/300
  lines, 6063/16384 bytes`)

- **D4** — the mdBook carries template identity (`title = "Project Book"`, `authors = ["<your name>"]`)
  and the template introduction
- Reproduce: `grep -nE 'title\|authors' docs/book/book.toml`
- Impact: the director's only view into the project is unnamed and empty
- Owner: `SPINE.1` (**fixed** — `book.toml` identity set, real introduction, `spec/` part added; `mdbook
  build docs/book` → `exit=0`)

- **D5** — `LIVE_STATUS.md` rows are the template's (`_(your first milestone)_`)
- Reproduce: `grep -n 'Not Started' LIVE_STATUS.md`
- Impact: the authoritative progress tracker says nothing about StitchCAD
- Owner: `PLANNING.1` (fixed)

- **D6** — `CHANGELOG.md` holds only bedrock-scaffold history (6 headings), zero StitchCAD entries
- Reproduce: `grep -c 'bedrock-scaffold' CHANGELOG.md` → 6; `grep -ci stitchcad CHANGELOG.md` → 0
- Impact: layer-D human-readable history has no project entries
- Owner: `PLANNING.1` (fixed)

- **D7** — the Knowledge Map documents no subsystems
- Reproduce: `grep -n 'no subsystems documented' KNOWLEDGE_MAP.md`
- Impact: orientation map is empty for a project with a 50 KB roadmap
- Owner: `SPINE.5` (**symptom fixed by accretion, verified `2026-09-30`** — `grep -c '^- \`'
  KNOWLEDGE_MAP.md` → `6` subsystem entries and `grep -c 'no subsystems documented' KNOWLEDGE_MAP.md` →
  `0`, `rc=1`; the rows were added by `G0-CONTRACT.18`, `SPINE.4.2` and `G0-CONTRACT.1`, not by `SPINE.5`.
  What `SPINE.5` still owes is its own acceptance clause — invoking each listed tool once and recording
  its real output shape — which stays open in that leaf.)

- **D8** — `docs/ARTIFACT_CLEANUP.md` is absent, so the 24 h cleanup cadence has no record
- Reproduce: `ls docs/ARTIFACT_CLEANUP.md` → No such file
- Impact: directive §8 cannot be honoured or audited across sessions
- Owner: `SPINE.2` (**fixed** — first cleanup run: four regenerable paths removed, `target` 2.1 MB → 1.2
  MB, residue census clean, record created)

- **D9** — `TOOLBOX.md` project-toolbox table holds placeholders (`<your-probe>`)
- Reproduce: `grep -n 'your-probe' TOOLBOX.md`
- Impact: tools-first doctrine has no reachable tool list
- Owner: `SPINE.5` (**symptom fixed by accretion, verified `2026-09-30`** — `grep -cE '<your-|<fill'
  TOOLBOX.md` → `0`, `rc=1`, with `18` real instrument rows each carrying an invocation and its output
  shape. As with D7, the leaf's own verification clause is what remains, and it remains in `SPINE.5`.)

- **D10** — the starter crate still prints the bedrock message and is not a roadmap crate
- Reproduce: `grep -n bedrock crates/app/src/main.rs`
- Impact: `cargo run` describes the wrong project; roadmap §4.3 crate layout unrepresented
- Owner: `G1-SLICE.1` (deferred) — **closed early by `G0-CONTRACT.18`**: the roadmap's own G0 CI
  clause requires `sc-core` + `sc-units` to build for `wasm32-unknown-unknown`, so the starter crate
  was retired when those two landed (`git rm crates/app`); `G1-SLICE.1` keeps the rest of the §4.3
  crate layout

- **D11** — the in-repo `README_POLICY.md` is the older neutral body; the director's external reference
  has been revised (authority/provenance, duplication probe, routing-pressure closure, derived caps,
  unconditional check)
- Reproduce: `diff README_POLICY.md <external reference>` → 100+ differing lines
- Impact: directive §14 requires applying source updates; the guard lacks the routing-closure rule
- Owner: `SPINE.3` (**fixed** — revised neutral body copied in behind a fenced StitchCAD adoption note;
  `190` lines / `10535` bytes, `0` donor tokens; the derived caps and destination registry it now
  requires are recorded as owed by `SPINE.4`)

- **D12** — the claim-verification policy (directive §17) is not adopted in-repo
- Reproduce: `ls docs/CLAIM_VERIFICATION.md` → No such file
- Impact: published numbers would have no defined standard for "checked"
- Owner: `SPINE.3` (**fixed** — adopted as `CLAIM_VERIFICATION.md` with all three legs restated in this
  project's terms; `CLAUDE.md` and the README route to it)

- **D13** — live-document containment (directive §18) is only partially adopted: `MEMORY.md`/`README.md`
  caps exist, but the surfaces that are about to grow (`ROADMAP.md` 50 821 B, `CHANGELOG.md` 11 761 B,
  `docs/TASK_TREE.md` with 12 trees) have no inventory, ceilings or ratchet
- Reproduce: `wc -lc ROADMAP.md CHANGELOG.md docs/TASK_TREE.md`
- Impact: bounded pointer, unbounded neighbours — the failure the containment guide exists to prevent
- Owner: `SPINE.4` (**fixed** across `.4.1`–`.4.3`) — doctrine adopted in-repo; 17 surfaces and 15
  routes classified with derived ceilings; enforced by the `LIVE-DOC-SIZE` project doctrine
  (`live-doc-size: OK — 17 surfaces, 15 routes, 41 files measured`, 11 self-test arms, 4 end-to-end
  probes). Residual pressure is owned, not ignored: `SPINE.13` (roadmap navigation) and `SPINE.15`
  (wide-row convention).

- **D14** — the workspace lockfile is untracked, so the first `make check` in a fresh clone leaves the
  tree dirty
- Reproduce: `make check && git status --short` → `?? Cargo.lock`; `git check-ignore -v Cargo.lock` →
  `rc=1` (not ignored)
- Impact: the pivot rule defines handoff-ready as *no untracked files*, and the toolchain itself
  violates it; `.gitignore` states the lockfile is deliberately tracked for reproducible builds
- Owner: `SPINE.6` (fixed)

- **D15** — the inherited `TASK-ACCEPTANCE` gate scans a staged tree file for the FIRST box matching
  each label, so in a multi-leaf file one leaf's evidence satisfies another leaf's code change (false
  GREEN), and an earlier unticked placeholder rejects a leaf carrying real evidence further down the
  same file (false RED). **Facet 3, measured in the wild on this repository's own `STITCHCAD-SPINE-0007`
  attempt:** the check judges *every* staged leaf file, not only the leaf owning the code, so co-staging
  a documentation tree with a code change imposes the evidence-signature requirement on that doc tree's
  boxes — `PLANNING.1`'s census bullets cited two commands and their real output and were still refused
  (`carries no tool-output evidence`, `exit=1`) because a filename listing matches no signature family
- Reproduce: `bash docs/tasks/artifacts/task_acceptance/run_multileaf_shadowing_probe.sh` → `HOLE-1
  exit=0`, `HOLE-2 exit=1`, `HOLE-3 exit=0`, `probes: 6 pass / 0 fail`; mechanism at
  `scripts/check_task_acceptance.sh:106-112`; facet 3 reproduced by `scripts/check_doctrines.sh` with
  the probe staged → `❌ TASK-ACCEPTANCE … docs/tasks/PLANNING.md — the 'ROOT CAUSE' box is ticked but
  carries no tool-output evidence`, `exit=1`
- Impact: every code commit in this repository is judged by this gate, and this project's trees are
  deliberately multi-leaf; the check's own header claims box-scoping closed cross-leaf evidence leakage,
  and its probe suite exercises that leakage only across files
- Owner: `SPINE.7` (measured, published — **done**) and `SPINE.8` (local `FRESH-ACCEPTANCE-EVIDENCE`
  doctrine + the evidence-token convention: cite the invocation, its output **and its exit status**);
  the upstream fix is reported, not patched — the spine is shared code

- **D20** — a **published number has no producer**: the awk-versus-grep signature measurement in
  `SPINE.8`'s record (`12 of 36` unmatched under awk, `2` under grep) was produced by a corpus that
  lived in untracked scratch and was removed by the `SPINE.2` cleanup — a leg-3 (durability) breach of
  the claim-verification standard this repository adopted in the same session
- Reproduce: `grep -rn '12 of 36' --include='*.md' .` → 2 tracked files publish it; `ls
  docs/tasks/artifacts/evidence_signatures` → `No such file or directory`; `ls
  target/doctrine_scratch/evidence_corpus.txt` → `No such file or directory`
- Impact: the number is quoted in two tracked documents and cannot be re-derived by any command, which
  is exactly the "trust me with extra steps" failure the adopted standard exists to stop; found by the
  adoption's own §7 step-3 sweep for untracked producers
- Owner: `SPINE.11` (**fixed** — corpus + runner tracked under
  `docs/tasks/artifacts/evidence_signatures/`; one command reproduces `corpus: 36 lines · grep-
  unmatched: 2 · awk-unmatched: 12`, and the constants are watched so a drift fails the probe instead of
  silently restating the claim)

- **D19** — `make book` writes `docs/book/book/`, which `.gitignore` did not ignore, so building the
  documented book left the tree dirty
- Reproduce: `make book && git status --short` → `?? docs/book/book/`; `git show HEAD~1:.gitignore |
  grep -c 'docs/book/book'` → `0`
- Impact: the pivot rule defines handoff-ready as *no untracked files*, and the documented quick start
  violated it on its last command; a build artifact was one `git add -A` away from being committed
- Owner: `SPINE.1` (**fixed** — `/docs/book/book/` ignored; a build now leaves `git status --short`
  clean)

- **D21** — a table cell in this census carried a raw `|` inside a code span
  (`git show HEAD~1:.gitignore | grep -c …`), so the row's cell count depended on which table rule the
  renderer implements: the GFM spec asks for `\|` even inside a code span, while the inherited arity
  checker treats a code span as protective. A naive splitter (this leaf's own conversion script) read
  the row as 6 cells against a 5-cell header.
  - Reproduce: `git show HEAD~1:docs/tasks/PLANNING.md | grep -c 'gitignore | grep -c'` → `1`, `rc=0`;
    `bash scripts/check_table_arity.sh --all` → `0 arity-defective row(s)`, `exit=0`.
  - Impact: the rightmost column (the defect's owner) is exactly what a renderer following the spec
    drops, so ownership could vanish from the rendered page while every gate stayed green.
  - Owner: `SPINE.4.2` (**fixed** — the census is bounded prose entries now, so no cell contains a
    pipe; the escape-always convention for any future table is `SPINE.15`).

- **D25** — the inherited code-path default misclassifies this project's documentation as code:
  `default_code_re` in `scripts/check_task_acceptance.sh` contains `(^|/)src/`, and the mdBook chapters
  live in `docs/book/src/`, so writing a specification chapter triggered the code-change acceptance gate
  and demanded tool-output-backed boxes for prose.
  - Reproduce: `git ls-files 'docs/book/src/*.md' | wc -l` → `4`, `rc=0`; staging one of them makes
    `scripts/check_fresh_acceptance_evidence.sh` report `a CODE change is staged`, `exit=1`.
  - Impact: a false positive on the most common product action at gate G0 (writing spec chapters), and
    exactly the pressure `.doctrine/README.md` warns about — a gate whose signature does not fit the
    real corpus teaches authors to paste tokens they do not mean.
  - Owner: `SPINE.16` (**fixed** — `.doctrine/code_paths.txt` declares the classification for this
    repository; both acceptance checks read the same seam; the probe suite copies it so it tests the
    real configuration).

- **D23** — the containment registry's `book_collection` row declared its glob as
  `docs/book/src/**/*.md`, which under `git ls-files` pathspec semantics matches **only nested** files
  and silently misses top-level chapters — so two of the three existing book files would have been
  outside every per-part and aggregate bound, in the registry whose whole purpose is to leave no
  surface unbounded.
  - Reproduce: `git ls-files 'docs/book/src/**/*.md'` → `docs/book/src/spec/index.md` (1 file) while
    `git ls-files 'docs/book/src/*.md'` → 3 files, `rc=0` both.
  - Impact: a collection row that under-matches reports green while ungoverning most of the collection;
    the failure is silent because an empty or small expansion is not an error.
  - Owner: `SPINE.4.3` (**fixed** — the row now uses `docs/book/src/*.md`, the registry header records
    the measured glob semantics, and the checker refuses a glob that matches nothing unless the row
    declares `meas_files=0`).

- **D22** — the inherited `scripts/check_table_arity.sh` documents its cell rule as "pipes NOT inside an
  inline code span" (lines 19–22) and self-tests that rule (line 73: *a pipe inside a code span is not a
  separator*), which is not what the GFM spec text says. The gate therefore cannot see the D21 class.
  - Reproduce: `grep -n 'code span' scripts/check_table_arity.sh` → lines `19`, `22`, `73`, `99`, `rc=0`;
    a synthetic row `| x | ` + "`cmd | grep y`" + ` | owner |` under a 3-column header reports
    `0 arity-defective row(s)`.
  - Impact: a renderer-dependent row shape passes the gate that exists to catch it. The divergence is
    **not verified against a real renderer**, so it is logged as a question with an oracle, not as a
    claim that the spine is wrong.
  - Owner: `SPINE.15` (**settled against a real renderer** — the answer is that the renderer splits, so the
    inherited checker's documented rule is wrong for the renderer this repository ships through. Oracle:
    `bash docs/tasks/artifacts/table_render/run_table_render_probes.sh` → `probes: 3 pass / 0 fail`, with the
    rendered rows printed: a 3-column row whose first cell carried `` `x | y` `` came back as
    `A raw pipe in a code span: `x` │ ``y` `` │ `2` — the code span broken open and the rightmost cell
    **dropped**, while the same row with `` `x \| y` `` kept all three cells and rendered a literal pipe. The
    third arm pins the divergence: the inherited checker's own self-test still asserts "a pipe inside a code
    span is not a separator", so the gate under-reports what the renderer does. The convention is adopted in
    `COMMIT.md` (escape every pipe in a table cell; a cell is not a paragraph), the spine file is left
    untouched as NEUTRAL, and the upstream question is reported to the director below as **D47**.)

- **D47** — the table convention is prose, and the only gate that could enforce it is the inherited checker
  whose rule the render oracle just disproved: nothing in this repository mechanically refuses a raw `|`
  inside a table cell's code span, so the next such row ships and loses its rightmost cell silently.
  - Reproduce: `printf '| a | b | c |\n|---|---|---|\n| `x | y` | 2 | 3 |\n' | python3 -c "$PY_SRC"` is not
    reachable from outside the checker, but its documented behaviour is: `bash scripts/check_table_arity.sh
    --self-test` → `arm ok  a pipe inside a code span is not a separator (0)`, i.e. the gate reports **0**
    defects for a row that renders with a dropped cell. `scripts/check_doctrines.project.sh` registers two
    project doctrines, neither of which reads table cells.
  - Impact: the failure is invisible in the source and visible only in the rendered page — a column of a
    contract silently missing from the book the director reads, with every gate green. That is the exact shape
    `TABLE-ARITY-RATCHET` was ported to catch, and its inherited implementation cannot catch this instance.
  - Owner: `SPINE.20` (**fixed**) — `scripts/check_table_code_pipes.sh` is registered as the project doctrine
    `TABLE-CODE-PIPE` and refuses a staged `.md` table row carrying a raw pipe inside a code span, naming the
    file, the line and the escaped form to write instead. Seven `--self-test` arms (raw, escaped, an ordinary
    separator, a fenced quotation, a double-backtick span, prose, an indented row) → `7 arms, 0 failed`; the
    whole tracked book measured `0` violations first, so the gate is absolute rather than a ratchet; the render
    probe stays its ground truth. The inherited `check_table_arity.sh` is untouched (NEUTRAL, re-synced by
    `scripts/update_scaffold.sh`) and the divergence is mirrored in `DOCTRINE_ENFORCEMENT.md` for the upstream
    report, per this repository's standing rule.

- **D24** — a process defect, surfaced by the director rather than by any gate: of this project's first
  17 commits, 15 were spine/governance slices and none was product work. No garment-domain
  specification chapter and no line of `sc-*` code existed after a full session, and five of the ten
  roadmap lanes still had no task-tree, while the spine kept yielding one more defect to chase.
  - Reproduce: `git log --oneline | grep -cE 'leaf (SPINE|PLANNING)'` → `15`;
    `git log --oneline | grep -c 'leaf G0-CONTRACT'` → `0`; `ls docs/book/src/spec/` → `index.md` only;
    `git ls-files 'crates/*'` → the bedrock starter crate alone; and for the unowned lanes:
    `for g in G5 G6 G7 V1 V2; do ls docs/tasks/${g}* >/dev/null 2>&1 || echo "$g unowned"; done`.
  - Impact: real defects were genuinely fixed (a data-loss path in the scaffold updater, an unsound
    acceptance gate, an unbounded live-document set), but governance work is self-rewarding and
    infinitely discoverable, and nothing in the spine asks whether the frontier is still on the
    product. The repository's reason to exist — the G0 semantic contract — stayed at zero.
  - Owner: `PLANNING.3` and `PLANNING.4` (**both closed** — the capture is complete and derived by the census tool; the sequencing rule is `docs/decisions/decision_product-work-takes-the-frontier.md`, wired into `CLAUDE.md`). `PLANNING.3` completes the capture (the five missing trees
    plus a coverage census that makes "every lane is owned" a re-derivable claim instead of an
    intention); `PLANNING.4` records the sequencing rule in the layer-A pointer so the next session
    inherits the priority instead of rediscovering the spine.

- **D16** — the inherited probe suites scratch off-volume: they call `mktemp -d`, which resolves to the
  system volume while the repository lives on another one
- Reproduce: `mktemp -d` → `/var/folders/…/T/tmp.…`; `TMPDIR="$PWD/target/scratch" mktemp -d` →
  `<repo>/target/scratch/tmp.…`; both suites pass either way (`probes: 10 pass / 0 fail`)
- Impact: violates the data-locality rule (project-owned temporary workspaces stay on the repository
  volume), and scratch invisible to the repository volume is scratch the artifact-cleanup census cannot
  see
- Owner: `SPINE.10` (**fixed** — `make probes` pins `TMPDIR` under `target/scratch`; one accepted
  residual recorded in the leaf: the shared check's trap-cleaned `mktemp -d` at commit time)

- **D17** — `scripts/update_scaffold.sh` classifies four files that carry project content as NEUTRAL —
  "safe to overwrite because it never carries project content" — while the template's own instructions
  tell the project to fill them in
- Reproduce: `grep -nE '^  (docs/TASK_TREE\.md|TOOLBOX\.md|README_POLICY\.md|docs/tasks/TEMPLATE\.md)$'
  scripts/update_scaffold.sh` → lines 27, 28, 31, 33 (`count=4`)
- Impact: running the documented "keep the spine current" command would silently replace the Active Task
  Trees index (layer-B navigation), the project toolbox rows and the README-policy adoption note with
  template blanks — a data-loss path wired into the maintenance instructions
- Owner: `SPINE.9` (**fixed** — two declared classes, backup + skip, dirty-tree refusal, `--dry-run`;
  `probes: 7 pass / 0 fail`)

- **D26** — the reference fixture used machine tokens that nothing declared: `docs/book/src/spec/reference-skirt.md`
  named its 17 derived values in prose ("quarter hip", "front dart centre") while §5's recipe and §4's
  own formulas referred to them as tokens (`quarter_hip`, `front_dart_centre`), and four allowance/ease
  tokens (`ease_waist`, `ease_hip`, `sa_cb`, `sa_waist`, `sa_wb_bottom`) appeared in formulas with no
  declaring table at all.
  - Reproduce (before the fix): the tokens the chapter used, against the tokens its tables declared —

    ```bash
    grep -oE '`[^`]+`' docs/book/src/spec/reference-skirt.md | tr -d '`' \
      | grep -oE '[a-z][a-z0-9]*(_[a-z0-9]+)+' | sort -u
    ```

    → `front_dart_centre`, `quarter_hip` and four allowance/ease tokens used by §4 and §5, with no table
    declaring them. From `G0-CONTRACT.1` onward the same population is derived by a tracked census
    rather than eyeballed, and it reports `unaccounted: 0` over the whole specification set.
  - Impact: a token nobody declares has no defined meaning, so the fixture G2 freezes as a golden would
    carry names whose referent a reader has to guess, and no instrument could tell a typo from a value.
  - Owner: `G0-CONTRACT.13b` (**fixed** — §2's ease table, §4's derived table and §7's allowance table
    now lead with the token, §4's formulas are written over tokens only, and the census enforces it).

- **D27** — the reference fixture's waistband is two different garments at once: §4's
  `waistband_cut_width = 2 × wb_width + sa_waist + sa_wb_bottom = 10.0 cm` is the cut width of ONE band
  folded lengthwise, while §6's piece list carries `waistband_outer`, `waistband_inner` and
  `waistband_interfacing` — a faced two-piece band whose pieces would each be cut at 6.0 cm — and §8's
  `waist` span sews only the outer band, so the inner band has no span and §12's count of 6 pieces is
  the faced reading's.
  - Reproduce: `python3 -c "print(2*4.0+1.0+1.0, 4.0+1.0+1.0)"` → `10.0 6.0`, the folded and faced cut
    widths; `grep -c 'waistband' docs/book/src/spec/reference-skirt.md` → the piece list, the span table
    and the formula disagree about how many bands exist. The **standing** reproduce is now the instrument
    that refuses the disagreement: `bash docs/tasks/artifacts/reference_fixture/run_fixture_derivation.sh`
    → `0 mismatch(es)`, `exit=0`, and its `BAND-PAIR` probe arm restores the faced reading and requires a
    refusal naming D27.
  - Impact: the fixture is the subject of every G2 golden, mutation, offset and agent test. Freezing it
    with an unresolved construction freezes a contradiction into the conformance corpus, and the piece
    count (6) is an asserted package-completeness expectation.
  - Owner: `G0-CONTRACT.13d` (**fixed**) — the director ruled on `2026-09-30` that the engineer decides and
    acts on this (see `docs/decisions/decision_director-ruling-2026-09-30-four-findings.md`), with external
    sources read and cited, and with the choice still subject to the domain reviewer `.14` names.
    `G0-CONTRACT.13b` recorded the contradiction **in the chapter** (§6 and §11) so no reader and no golden
    could take a waistband number as settled while it was open. `.13d` chose the **single straight band,
    cut once and folded at its midpoint, plus one interfacing piece fused inside the seam lines**: five
    pieces, not six; `waistband_outer` and `waistband_inner` no longer exist; §4 gained four rows and §4.1
    gained the band's two closure checks; §8 now accounts for every piece by a span or by a declared
    non-sewn attachment. The decisive fact was sourced, not preferred — the drafting references prescribe
    the two-piece cut for a **contoured** band, and this fixture's band is straight at the natural waist.
    Decision, five sources with URLs and the date read, what stays `assumed`, and the re-open condition:
    `docs/decisions/decision_reference-fixture-waistband-straight-folded.md`. The agreement between §4, §6,
    §8 and §12 is now derived by a tracked producer rather than by reading.

- **D28** — the fixture's recipe step 7 sent the reader to the wrong clause: "**allowances** (§6)" where
  allowances are §7 and §6 is the piece list.
  - Reproduce (before the fix): `grep -n 'allowances\*\* (§6)' docs/book/src/spec/reference-skirt.md` → `1`.
  - Impact: small, but it is the class a reader cannot detect — a confident cross-reference to the wrong
    clause, in the chapter the conformance corpus is built from.
  - Owner: `G0-CONTRACT.13b` (**fixed** — §5 step 7 now cites §7; the glossary census's R1 rule resolves
    every canonical-object reference in the glossary against real headings, which is the same class
    instrumented rather than eyeballed).

- **D29** — `CHANGELOG.md`'s live window is not in the newest-first order its own header declares: three
  entries (`G0-0002`, `PLANNING-0004`, `PLANNING-0003`) sit above six that are newer than they are.
  - Reproduce: compare the two orders —
    `git log --format='%s' | grep -oE 'STITCHCAD-[A-Z0-9]+-[0-9]+[a-c]?'` against
    `grep -oE '^## STITCHCAD-[A-Z0-9]+-[0-9]+[a-c]?' CHANGELOG.md`; they disagree from the first row.
  - Impact: a reader takes the top entry for the latest slice, and the rollover rule ("seal the oldest")
    would seal the WRONG end — the ordering defect turns into a data-placement defect the first time the
    window crosses its health target, which the next append does.
  - Owner: `G0-CONTRACT.1` (**fixed** — the append that crossed the rollover milestone performed the
    rollover: the live window is reordered into commit order, the five oldest entries are sealed into
    `docs/history/stitchcad-changelog-part2.md` with their own descriptor, and
    `docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh` now derives the order instead of
    trusting it → `probes: 6 pass / 0 fail`, `ORDER PASS 8 live entries in commit order`).

- **D30** — the sealed archive's descriptor and the live pointer both misstate what part1 holds: each says
  coverage runs "from `STITCHCAD-PLANNING-0001` through `STITCHCAD-SPINE-0004c`", but part1's newest entry
  is `SPINE-0004b` and `SPINE-0004c` is still in the live window.
  - Reproduce: `grep -c 'STITCHCAD-SPINE-0004c' docs/history/stitchcad-changelog-part1.md` → `0` (the id
    appears only in its coverage line), while `grep -n '^## STITCHCAD-SPINE-0004c' CHANGELOG.md` → `324`.
  - Impact: a reader who follows the pointer for slice `0004c` searches an immutable segment that does not
    contain it, and the "slices 1–15" count overlaps the live window by one.
  - Owner: `G0-CONTRACT.1` (**fixed by superseding record**, the only correction an immutable segment
    allows: part2's descriptor and the live pointer both state part1's true coverage
    (`PLANNING-0001` … `SPINE-0004b`) and name the error; part1 is untouched, and the ledger probe's
    `COVERAGE` rule carries it as a declared, reasoned exemption so the check stays green without
    pretending the segment is right).

- **D31** — `CHANGELOG.md`'s header told the reader that "everything below the _Inherited spine history_
  divider is the bedrock scaffold's own changelog", but that divider left the file in `SPINE-0014`: the
  scaffold's changelog is sealed in `docs/history/bedrock-scaffold-changelog.md`.
  - Reproduce (before the fix): `grep -c 'Inherited spine history' CHANGELOG.md` → `1`, while
    `grep -c '^# Inherited spine history' CHANGELOG.md` → `0` — the header named a divider that was not
    there, and the sentence was quoted back by `SPINE-0014`'s own entry.
  - Impact: the ledger's own navigation sentence described a structure the file no longer had, so a
    reader looking for the scaffold's history looked in the wrong file.
  - Owner: `G0-CONTRACT.1` (**fixed** — the header now names the two sealed segments in a table with
    their coverage and identity, points at `bedrock-scaffold-changelog.md` explicitly, and the ledger
    probe's `POINTER` rule keeps the pointer and the segments on disk in agreement both directions).

- **D32** — four features the v1 envelope promises have no gate whose exit criteria prove them, and a
  fifth follows from them: roadmap §3.2 puts a **classic collar** and **trousers** inside the envelope and
  ontology §4.7 specifies **button/buttonhole** and **pocket** objects, yet no gate's exit criteria in
  roadmap §11 mention any of them; a **fly construction** follows trousers. G3's exit names a bodice and a
  set-in sleeve, and its "intermediate complexity note" *permits* a shirt/trousers intermediate without
  promising one.
  - Reproduce: `bash docs/tasks/artifacts/feature_matrix/run_feature_matrix_census.sh` → the `A1` advisory
    listed all five rows with `Proven at` = `unnamed (D32)`, on every run, by design (before `.4b` resolved
    it; A1 now reports `0` and the A3 advisory lists the four `(proposed)` cells); and
    `grep -c 'collar\|trousers\|button\|pocket' ROADMAP.md` against `sed -n '/### G3 /,/### G4 /p'
    ROADMAP.md | grep -ci 'collar\|trousers\|button\|pocket'` → the second count is `1`, **not the `0` this
    entry first recorded** (defect **D41**): a section range also contains G3's complexity note, which says
    "a shirt/trousers intermediate". The count that supports the claim is over G3's exit criteria alone —
    `sed -n '702,708p' ROADMAP.md | grep -ci 'collar\|trousers\|button\|pocket'` → `0` — and the A1
    advisory, which reads the matrix rather than the roadmap, listed all five rows correctly throughout.
  - Impact: gate G7's exit is a "supported-envelope statement with named limitations". An envelope whose
  collar, trousers, buttons and pockets were never tested cannot be declared honestly, and the gap is
  invisible today because a feature matrix did not exist to show it.
  - Owner: `G0-CONTRACT.4b` (**resolved**) — the director ruled on `2026-09-30` that the engineer decides
  and acts on this. The matrix gets a proving gate per row, and the `ROADMAP.md` §11 amendment those gates
  need is prepared as an exact **proposal** in a decision record, because the roadmap is the director's to
  amend (the ruling reserves it). Until he approves it, the record says so and `.15` carries the proposal
  into the G0 exit review. **Landed:** collar, trousers, buttons and pockets are assigned to **G3** (buttons
  also to **G5**, whose tech-pack clause already requires notions) and the fly to **G7**, whose existing exit
  already requires named limitations and so needs no amendment. The four cells that depend on the amendment
  say `(proposed)`; the exact text — one added G3 exit bullet covering every garment §3.2 names, plus the
  rewritten complexity note — is quoted current-vs-proposed with its line numbers in
  `docs/decisions/decision_d32-proving-gates-proposed-roadmap-amendment.md`. Derived, not asserted:
  `bash docs/tasks/artifacts/feature_matrix/run_feature_matrix_census.sh` → `A1 … D32 rows: 0` and the new
  `A3` advisory listing all `4` proposed cells; `run_feature_matrix_probes.sh` → `probes: 12 pass / 0 fail`,
  including an arm that removes the markers and requires the A3 count to fall. The defect closes as a
  *matrix* gap; the roadmap amendment stays pending the director's ruling, kept visible by A3 on every run
  and carried into the G0 exit review by `.15`.

- **D33** — the reference fixture's waist geometry was arithmetically wrong, and its own oracle could not
  see it: §5 step 2 placed the waist side point at `quarter_waist − ss_suppress` = 15.5 cm from CF, while
  §3's `ss_suppress` = 3.0 cm and §4's allocation balance require `quarter_hip − ss_suppress` = 22.5 cm —
  the side seam takes its suppression off the **hip** width, not the waist width.
  - Reproduce: re-derive the finished waist from the drafting steps —
    `python3 -c "print(4*((98.0+4.0)/4 - 3.0 - 4.0), 4*((74.0+0.0)/4 - 3.0 - 4.0))"` → `74.0 46.0`, the
    corrected and the as-written garment waists against a declared `waist_girth + ease_waist` = 74.0 cm;
    and the chapter's own oracle over the same error: `4 × (3.0 + 4.0) = 102.0 − 74.0` → `28.0 = 28.0`,
    which passes either way.
  - Impact: a 28.0 cm error in the finished waist of the one garment every G2 golden file, mutation test,
    offset-pathology case and agent evaluation is built around — and the balance check the chapter
    described as "the fixture's own invariant" is blind to it, because it closes over *declared* quantities
    while the error is in a *constructed* point. Frozen as a golden, the error would have become the
    expected output.
  - Owner: `G0-CONTRACT.13c` (**fixed** — §5 step 2 and step 3 corrected, the two dart-centre formulas
    corrected to the span they meant (7.75 → 11.25 cm), §4 gains a `waist_closure` row as a second oracle,
    §12 makes both checks obligations, §11 records the correction as the superseding record for the sealed
    `STITCHCAD-G0-0013` entry; all 18 derived rows re-derived with 0 mismatches, and the general rule is
    promoted to `docs/decisions/decision_fixture-oracles-derive-the-finished-dimension.md`).

- **D36** — `LIVE_STATUS.md`'s spine row reported the enforcement counts wrong: "13 universal + 2 project
  doctrine gates · 7 probe suites", where the driver registers 12 universal doctrines plus the project
  slot's 2, and `make probes` finds 12 suites.
  - Reproduce: `make gate 2>&1 | grep -c '✅'` → `13` printed lines, the last being the PROJECT-SPECIFIC
    slot; `bash scripts/check_doctrines.project.sh | tail -1` → `PROJECT-SPECIFIC: 2 project doctrine(s)
    green`; `find docs/tasks/artifacts -name 'run_*probe*.sh' | wc -l` → `12`.
  - Impact: the authoritative tracker is what the director reads to know what is enforced, so a stale
    count misstates the gate set. It drifted for the same reason D34 did — a number restated in prose
    whose producer lives elsewhere — which `CLAIM_VERIFICATION.md` §5B already names: a constant that is a
    function of this repository is derived or gated, never remembered.
  - Owner: `G0-CONTRACT.13d` (**fixed in this commit** — the row carries the corrected counts *and* the
    two commands that derive them, so the next reader can falsify it in one line instead of trusting it).
    **Second instance, found and removed by `G0-CONTRACT.4b`:** the same file's `PLANNING` row claimed
    "142 leaves" and its `SPINE` row "17 of 20 leaves done", where the trees now hold `153` leaf ids
    (count them with grep over the leaf-id lines of `docs/tasks/*.md`) and `SPINE` has `23`. Neither count has a
    producer, so both rows now state what their named instrument prints, or name the open leaves instead of
    counting them — the remedy `CLAIM_VERIFICATION.md` §5B prescribes, applied rather than cited.
    **Third instance, found and removed by `G0-CONTRACT.9`:** the same file's defect-census row listed D47 as
    open and counted "46 logged, 40 closed" after `SPINE.20` had closed D47 in `a743d53` — the row is
    hand-kept, which is D38, and the count has no producer, which is this defect. Corrected in that commit's
    lockstep (D47 closed, D48 logged and closed by the same slice, `16` probe suites named by the command
    `find docs/tasks/artifacts -name 'run_*probe*.sh' | wc -l` rather than remembered). Its `PLANNING` row
    claimed "2 evidence siblings" where `run_tree_coverage_census.sh` prints `3 sibling(s)`, and four notes
    cells were past the surface's 220-byte maxline health, so all four were tightened in the same pass — the
    remedy `COMMIT.md`'s table convention prescribes rather than a wider target.

- **D37** — `DEV_NOTES.md` described itself 148 lines into itself: the paragraph saying what the surface is
  ("Detailed technical notes … Newest first") sat *below* every lesson, immediately above the bootstrap
  entry, where the template's own header had been pushed by years of prepends.
  - Reproduce (before the fix): `grep -n 'Detailed technical notes' DEV_NOTES.md` → `149`, against
    `grep -n '^# DEV_NOTES.md' DEV_NOTES.md` → `1`; the description of a newest-first ledger was itself
    below the newest entry.
  - Impact: cosmetic for a reader who scrolls, real for the doctrine that reads it — a prepend-only surface
    whose header lives at the bottom silently loses its contract ("newest first", "this is not the public
    docs") to the append order, and the next author prepends above a description they never saw.
  - Owner: `G0-CONTRACT.13d` (**fixed in this commit** — the paragraph moved to line 3, under the title,
    where a prepend cannot bury it; the bootstrap entry stays last because it is the oldest lesson).

- **D39** — the changelog ledger's verifier parsed work-unit ids with a hardcoded suffix range
  (`STITCHCAD-[A-Za-z0-9]+-[0-9]+[a-c]?`), so the FOURTH sub-slice of any unit was mis-read: this slice's
  own id `STITCHCAD-G0-0013d` became `STITCHCAD-G0-0013`, collided with the sealed entry of that name, and
  the probe refused an honest ledger.
  - Reproduce (before the fix): `bash docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh` →
    `probes: 5 pass / 1 fail`, with `NO-DUP FAIL both live and sealed: STITCHCAD-G0-0013` (a duplicate that
    did not exist) and `ORDER FAIL … STITCHCAD-G0-0008-older-than-STITCHCAD-G0-0013` (comparing the sealed
    entry's commit position, not the new one's); and the parse itself,
    `grep -oE '^## STITCHCAD-[A-Za-z0-9]+-[0-9]+[a-c]?' CHANGELOG.md` → `STITCHCAD-G0-0013` for a heading
    that reads `## STITCHCAD-G0-0013d`.
  - Impact: a false red on the first `d`-suffixed slice of any unit, which is the shape that trains an author
    to bypass a probe — and the same collapse in the other direction is a false GREEN: two live entries
    `-0013d` and `-0013e` parse to one id, so `ORDER` cannot see them being swapped. The verifier the
    containment doctrine's rollover protocol depends on was unsound for a whole family of ids it had never
    been shown, because `a`–`c` happened to cover every sub-slice written until now.
  - Owner: `G0-CONTRACT.13d` (**fixed in this commit** — the shape is `[0-9]+[a-z]?` in all six places, and
    a new GREEN arm `SUFFIX` pins it: a live `d`-suffixed entry beside the sealed unsuffixed id must satisfy
    every rule. The arm is sensitive, measured by reverting the class in a scratch copy → `probes: 5 pass /
    2 fail`, `SUFFIX` and `REAL` both red. After the fix: `probes: 7 pass / 0 fail`.) D29's reproduce lines
    in this census still quote the old `[a-c]?` shape; they are a record of what was run then, and are left
    as written rather than edited into a command nobody ran.

- **D41** — a published measurement did not reproduce: D32's entry cited `0` for
  `sed -n '/### G3 /,/### G4 /p' ROADMAP.md | grep -ci 'collar\|trousers\|button\|pocket'`, and the command
  yields `1`. The same wrong output was quoted in three more places, one of them a sealed changelog segment.
  - Reproduce: run the cited command → `1`; the matching line is G3's complexity note, "a shirt/trousers
    intermediate may be inserted without shame". Two candidate causes, both measured: the pattern without
    `trousers` (`grep -ci 'collar\|button\|pocket'`) → `0`, and G3's **exit bullet alone**
    (`sed -n '702,708p' ROADMAP.md | grep -ci …`) → `0`. So either the pattern lost a word or the range was
    narrower than the one written down; the conclusion — no *exit criterion* names the five features — was
    true either way.
  - Impact: the claim was right and the citation was wrong, which is the more corrosive of the two: a reader
    who re-runs a recorded command and gets a different number stops trusting the numbers that are right
    (`CLAIM_VERIFICATION.md` §4.1, prose must stay true across re-verification). It also hides a real
    measurement trap — **a section range silently includes that section's notes**, so "no exit criterion
    mentions X" must be measured over the exit criteria, not over the gate's heading range. `G0-CONTRACT.15`
    will re-derive every gate clause the same way, so the trap was worth finding now.
  - Owner: `G0-CONTRACT.4b` (**fixed in this commit** — D32's reproduce line, `.4`'s ROOT CAUSE box and the
    live DEV_NOTES lesson all carry the corrected pair of commands and the reason; the sealed
    `stitchcad-changelog-part4.md` citation is immutable, so the `.4b` changelog entry is its superseding
    record, the shape D30 established). No instrument is owed: the fix is a corrected citation plus the
    scoping rule, which `.15` inherits.

- **D42** — `docs/tasks/SPINE.md` is past the split threshold the containment registry sets for a tree file
  and this slice added to it (the `SPINE.19` leaf) without performing the split `G0-CONTRACT.md` just got.
  - Reproduce: `wc -lc docs/tasks/SPINE.md` → `1096 88341`, against a `tasks_collection` per-part health of
    `800` lines / `65 536` B and a byte ceiling of `98 304`; the registry's own remedy for that shape is
    printed in its notes column — "a tree that passes 1000 lines splits its completed-leaf evidence into a
    sibling file under docs/tasks/ before adding more".
  - Impact: none today, it is inside the ceiling. The cost of deferring is that whichever slice crosses
    `98 304` B gets its commit blocked and performs the split under pressure, in a file it was not working
    on — which is precisely what happened to `.4b` (see D42's sibling evidence in that leaf's NO REGRESSION
    box). The remedy is mechanical and now demonstrated once, so there is nothing to gain by waiting.
  - Owner: `SPINE.4.4` (**fixed**), the leaf the frontier reached next — it split SPINE's `17` completed
    checklists into `docs/tasks/SPINE-evidence.md` under the convention `G0-CONTRACT.4b` recorded, taking the
    tree file from `1096` lines / `88 341` B to `552` / `42 028` (inside its `800` / `65 536` health) with the
    moved bytes identical, and left this leaf's own checklist in the tree file so the staged-leaf gate still
    has a box to judge.

- **D43** — a sealed segment's digest depends on a newline convention nothing had written down: sealing
  `stitchcad-changelog-part6.md` with the raw-byte digest made the ledger's `DESCRIPTOR` rule report a
  mismatch that read as content drift in an immutable archive, because the rule hashes `content=$(sed …)`
  followed by `printf '%s\n'`, and bash command substitution strips EVERY trailing newline — so a segment
  whose sealed content ends with a blank line can never reproduce a raw-byte digest.
  - Reproduce: at the measurement, `tail -c 12 docs/history/stitchcad-changelog-part6.md | od -c` ended
    `…touched \n \n` where part5 ended `…touched \n`; the raw-bytes digest of the sealed content was
    `8553accc…` and the rule's was `3148dd0f…`, one newline apart, with the descriptor declaring the former.
  - Impact: the failure mode is a MISLEADING refusal. "content hashes to X, its descriptor declares Y" is the
    most alarming sentence the ledger can print — silent drift in a segment that is supposed to be immutable —
    and an author chasing it could "repair" a sealed file, which is the one edit the archive forbids. It
    arrived with the third rollover of the day, i.e. exactly when the trap is cheapest to fall into.
  - Owner: `SPINE.4.4` (**fixed in this commit** — part6 normalized to end with exactly one newline and its
    descriptor recomputed by the rule's own method, so the live pointer, the descriptor and the verifier
    agree; `DESCRIPTOR` now refuses a segment ending in a blank line BY NAME instead of reporting a digest
    mismatch; the byte contract is written into the probe's header; and a `TRAILING-BLANK` arm pins the named
    refusal → `probes: 9 pass / 0 fail`).

- **D44** — the tree-coverage census enumerated every `docs/tasks/*.md` as a TREE, so the evidence siblings
  the containment registry prescribes (`G0-CONTRACT-evidence.md`, `SPINE-evidence.md`) were reported as two
  lane-less orphans and one lane looked unowned — and nothing noticed, because no gate and no probe suite ran
  that census.
  - Reproduce: `bash docs/tasks/artifacts/planning/run_tree_coverage_census.sh` at `SPINE.4.4` →
    `census: 10 lanes / 15 trees / 1 unowned / 4 orphan(s) / 0 dead link(s)`, `exit=1`, with
    `G0    G0-CONTRACT-evidence   NO — the tree does not name its lane` (population 1 picked the sibling
    because `X-evidence.md` sorts before `X.md`) and `G0-CONTRACT-evidence NO  NO LANE DECLARED` twice more in
    population 2. Two commits — `G0-CONTRACT.4b` and `SPINE.4.4` — shipped with that census red.
  - Impact: two defects in one. The census was wrong (a sibling is not an orphan), and the wrongness was
    invisible because a census whose green nobody re-derives is a claim, not an instrument: `make probes`
    globs `run_*probe*.sh`, so a file named `*_census.sh` is watched by nothing. Every other census in this
    repository has a probe suite; this one did not, and the difference is exactly the two commits it took to
    find out.
  - Owner: the slice applying roadmap v0.3 (**fixed**) — a tree is now recognised STRUCTURALLY by its
    `- Tree ID:` line rather than by its filename, a non-tree file must be linked from a tree or it is
    refused as a stray, the summary reports siblings separately
    (`10 lanes / 13 trees / 2 sibling(s) / 0 unowned / 0 orphan(s) / 0 dead link(s)`), and
    `run_tree_coverage_probes.sh` puts the census under `make probes` with seven arms — REAL, an unlinked
    sibling, a stray file, a lane with no tree, a tree with no lane, a dead index link and a missing roadmap
    → `probes: 7 pass / 0 fail`.

- **D45** — the standards census's S4 rule tested for a citation **anywhere in the registry row**, so an owner
  cell that happened to carry a cross-reference satisfied it: a `read-in-repo` claim with no clause behind it
  would have passed, which is a green verdict on the one status that permits quoting a document.
  - Reproduce: before the fix, `grep -n 'line !~ /§|clause/' docs/tasks/artifacts/standards/run_standards_census.sh`
    → the test ran over the whole matched line; set any row's status to `read-in-repo` and put a section sign in
    its OWNER cell, and S4 reported nothing. Found because rewording the ISO 8559 owner cell to
    `the domain expert — seat vacant (governance §8.1)` made the `BARE-READ` probe arm stop firing —
    `probes: 4 pass / 2 fail` — i.e. the arm broke honestly and named the weakness instead of hiding it.
  - Impact: S4 is the only rule standing between "we read the standard" and "we would like to quote the
    standard", and `standards.md` §1 makes `read-in-repo` the sole status that permits a clause number, a table
    or a quoted definition anywhere in the book. A rule that can be satisfied by an adjacent cell is the
    vacuous-green class this repository has now measured four times (the glossary's R1, the matrix's two
    mis-anchored arms, the coverage census's advisory).
  - Owner: the slice applying roadmap v0.3 (**fixed** — S4 reads the STATUS and ROLE cells only, its refusal
    names them, and `BARE-READ` now pins the scoping because the row it mutates carries a section sign in its
    owner cell → `probes: 6 pass / 0 fail`). The `NO-OWNER` arm was re-anchored on the row rather than on the
    owner's wording at the same time: an arm whose pattern silently matches nothing reports a failure whose
    message blames the tree.

- **D48** — `docs/tasks/G3-GRADING.md` carried two `## Acceptance Checklist` headings, and a children range
  that omitted a leaf the file holds: the boilerplate section stayed in place when `G0-CONTRACT.4c` added a
  second heading above its own boxes, and `Children:` still said `.1` … `.14` after `.15` was created.
  - Reproduce: `grep -c '^## Acceptance Checklist' docs/tasks/G3-GRADING.md` → `2` at `HEAD`, `1` after the
    fix; `grep -n 'Children:' docs/tasks/G3-GRADING.md` → a range ending at `.14`, against
    `grep -c '^- ID: .G3-GRADING\.' docs/tasks/G3-GRADING.md` → `15` leaves at `HEAD` and `16` after this
    slice added one. The population is every tree
    file, enumerated: `for f in docs/tasks/*.md; do n=$(grep -c '^## Acceptance Checklist' "$f");
    [ "$n" -gt 1 ] && echo "$f: $n"; done` → `docs/tasks/G3-GRADING.md: 2`, one file of sixteen.
  - Impact: two sections with one name, so a reader — and any gate that keys on the heading, as
    `scripts/check_task_acceptance.sh` keys on the first matching box — meets the boilerplate where the
    evidence is. That is D15's class one level up: a placeholder shadowing real evidence, by heading rather
    than by box. The stale children range is D34's class inside one file: a hand-kept summary that no
    producer re-derives.
  - Owner: `G0-CONTRACT.9` (**fixed in this commit** — the headings merged into one section, the range
    corrected to `.1` … `.16` with the leaf this slice added). The durable fix for the range is `PLANNING.5`,
    which derives index↔tree agreement; for the heading the remedy is a convention recorded here, because
    `grep -c` over `docs/tasks/*.md` shows no gate counts headings: one `## Acceptance Checklist` per tree
    file, and a leaf's boxes under it.

- **D50** — `KNOWLEDGE_MAP.md` reached 99 % of its byte ceiling while two product slices were landing:
  `wc -lc KNOWLEDGE_MAP.md` → `99 8128` against the `knowledge_map` row's ceiling of `8192`, so the next
  decision record would have blocked a commit on a surface that is generated and holds no unique facts.
  - Reproduce: the `wc -lc` above at `STITCHCAD-G0-0009`, plus
    `bash scripts/check_live_doc_size.sh 2>&1 | grep knowledge_map` → `8128 bytes = 198% of its 4096-byte
    health target`. The cause is measurable: `wc -lc knowledge-map/subsystems.md` → `53 4784`, i.e. the
    hand-curated input had grown prose entries that duplicated the chapters they point at, while the
    generated sections (one line per tree, one per record) are content the map cannot trim.
  - Impact: a blocked commit is a slice that has to make a containment decision under pressure, and the
    pressure lands on whoever happens to add the next record. The generator is NEUTRAL
    (`scripts/update_scaffold.sh` line 108), so its format — which carries each record's filename twice,
    ~128 B per record — cannot be compacted locally.
  - Owner: `G0-CONTRACT.10` (**fixed in this commit** for the pressure that exists: the input trimmed to
    `37` lines / `3 255` B and the map to `84` / `6 733`, which is 82 % of the ceiling). The durable half —
    re-deriving the row's health and ceiling from the generated shape at the named trigger — is `SPINE.5`'s,
    with the convention, the formula and the trigger recorded in
    `docs/decisions/decision_knowledge-map-entries-are-orientation-sized.md`.

- **D52** — three completed leaves' ROOT CAUSE boxes in `docs/tasks/G0-CONTRACT-evidence.md` asserted their
  evidence in prose with no invocation, so the file could not be committed at all once a seal changed which
  box the acceptance gate judges first: `scripts/check_doctrines.sh` →
  `TASK-ACCEPTANCE: docs/tasks/G0-CONTRACT-evidence.md — the 'ROOT CAUSE' box is ticked but carries no
  tool-output evidence INSIDE ITS OWN BULLET`, `exit=1`.
  - Reproduce: the refusal above, at the commit that sealed `G0-CONTRACT.2` … `.4b` out of the sibling. The
    gate judges the FIRST matching box in a staged file, so while `.2`'s checklist led the file its
    well-formed boxes shadowed three defective ones behind it. Enumerated rather than sampled:
    a scan of every `### <leaf>` section in the sibling for the gate's own signature regex reported
    `G0-CONTRACT.14`/ROOT, `G0-CONTRACT.14b`/ROOT and `G0-CONTRACT.19`/ROOT (plus four REPRODUCE boxes,
    which the gate does not hard-require).
  - Impact: a blocked commit is the visible half. The invisible half is that a ticked box citing no command
    is a "trust me" — the same shape as D20, in the one file whose whole purpose is evidence. It survived
    eleven commits because the gate's box-scoping, which is correct, means an earlier leaf's good evidence
    conceals a later leaf's absent evidence.
  - Owner: `G0-CONTRACT.12` (**fixed in this commit** — all three boxes now carry the invocation and its
    real output, run at fix time rather than recalled, and `.19`'s "twelve directories" is corrected to the
    `13` the command prints). The durable half is a convention rather than a new gate: a sealing operation
    changes which box is first, so **re-run the enforcer after moving checklists between files and before
    committing** — which is how this was caught, and is recorded in the sibling's own header note.
