# TOOLBOX.md — the tools-first diagnostic doctrine

⛔ **TOOLS-FIRST.** For ANY unknown — a failure, a crash, a hang, a surprising result, a
"why isn't this working" — reach for a diagnostic tool FIRST. Never eyeball the code and
guess a root cause.

## The rule

- A code change cannot land without **tool-backed WHY + WHERE** and a **measured
  before→after** recorded in its task-tree leaf (see the acceptance checklist in
  `DOCTRINE_ENFORCEMENT.md`).
- If no existing tool shows WHY+WHERE, **build one** — a probe, a tracer, a counter, a
  minimal reproduction harness. The diagnostic tool is a first-class deliverable, kept in
  the repo, not a throwaway.
- **ANTI-SPIN TRIPWIRE:** if you have analyzed for ~2 turns without producing NEW tool
  output that pinpoints WHY+WHERE, STOP — run a tool, build one, or escalate. Never loop
  on analysis.

## The 3-step UNKNOWN protocol (adapt the specific tools to your domain)

1. **WIDEN** — dump the full picture: enumerate all cases/states, the broadest inventory,
   so the failing one is visible in context.
2. **NARROW** — probe the specific failing case for its exact verdict + position/state.
3. **PINPOINT** — a scoped trace that names the exact function/rule/line that fails and why.

The point is to convert "it's broken somewhere" into "line X of function Y rejects input Z
because predicate P is false" before writing a single line of fix.

## This project's toolbox

<!-- Fill this in as your project grows. List each diagnostic tool, what question it
answers (WHY / WHERE / how-much), and how to invoke it (binary, flag, env var). The next
agent should be able to reach for the right tool without reading the source. -->

| Tool | Answers | How to invoke |
| --- | --- | --- |
| doctrine enforcer | is the repository committable — do all 13 registered doctrines hold right now? | `scripts/check_doctrines.sh` (same as `make gate`; prints `=== all doctrines green ===`) |
| per-check self-test | does a single doctrine check still discriminate (both arms fire)? | `scripts/check_<name>.sh --self-test` (e.g. `scripts/check_live_doc_currency.sh --self-test`) |
| multi-leaf shadowing probe | which leaf's evidence does `TASK-ACCEPTANCE` actually judge, and does a placeholder block an honest leaf? (defect D15) | `bash docs/tasks/artifacts/task_acceptance/run_multileaf_shadowing_probe.sh` → `probes: N pass / M fail` |
| scaffold updater | which spine files would a sync overwrite, and which are guarded project content? (defect D17) | `scripts/update_scaffold.sh <bedrock-url-or-path> --dry-run` (live run needs a clean tree; `--force-project-sections` to override a guard) |
| scaffold-sync probe suite | does the updater still protect the task-tree index, refuse a dirty tree and write nothing on a dry run? | `bash docs/tasks/artifacts/scaffold_sync/run_update_scaffold_probes.sh` → `probes: N pass / M fail` |
| signature-portability probe | which evidence strings does the acceptance gate actually recognise, and does the answer depend on the regex engine? (the tracked producer behind the published `12 of 36` / `2 of 36` measurement) | `bash docs/tasks/artifacts/evidence_signatures/run_signature_portability_probe.sh` → per-line table, the two counts, `probes: N pass / M fail` |
| live-doc size check | is every tracked live document classified, owned and inside its ceiling — and does every route end at a classified destination? | `scripts/check_live_doc_size.sh` (runs inside `make gate`); refusal classes: `--self-test` |
| live-doc size probe suite | does the containment check still pass on the real tree, and does it NOTICE a surface that lost its registry row? | `bash docs/tasks/artifacts/live_doc_size/run_live_doc_size_probes.sh` → `probes: N pass / M fail` |
| containment data plane | which surfaces and routes are governed, with what targets and ceilings? | `.doctrine/live_document_size/surfaces.tsv` and `routes.tsv` (read the header comments first) |
| fresh-evidence check | does THIS commit add its own ticked, evidence-backed acceptance boxes for the code it stages? (project doctrine `FRESH-ACCEPTANCE-EVIDENCE`) | `scripts/check_fresh_acceptance_evidence.sh` (runs inside `make gate`); arms: `--self-test` |
| fresh-evidence probe suite | does the fresh-evidence doctrine refuse stale evidence and still accept a co-staged documentation tree? | `bash docs/tasks/artifacts/fresh_evidence/run_fresh_evidence_probes.sh` → `probes: N pass / M fail` |
| task-acceptance probe suite | does the inherited acceptance gate still hold its shipped properties? | `bash docs/tasks/artifacts/task_acceptance/run_task_acceptance_probes.sh` |
| Rust gate | do fmt, clippy (deny warnings) and the tests pass? | `make check` |
| WASM smoketest | do the foundation crates really cross-compile for the browser profile (not a host `cargo check`)? | `make wasm` |
| push-due | is an exceptional push owed — did an unpushed commit touch CI, a doctrine check, the `.doctrine/` seams or the hooks? | `make push-due` (exit 1 = a push is due, and it lists the files) |
| glossary census | is the vocabulary sound — one meaning per term, one owner per machine token, every canonical reference resolving to a real clause/leaf, every token the spec set uses declared somewhere? | `bash docs/tasks/artifacts/glossary/run_glossary_census.sh` → `glossary census: N terms / M parts / K tokens / 0 failure(s)`; `--emit-index` regenerates the A–Z index the chapter must carry |
| feature-matrix census | is the envelope complete — every ontology object clause cited by a row, every roadmap non-goal rejected, every envelope garment supported, every refusal naming a declared diagnostic, every gate cell a real gate, every link resolving? | `bash docs/tasks/artifacts/feature_matrix/run_feature_matrix_census.sh` → `feature-matrix census: N rows / M diagnostics / 0 failure(s)`, plus the `A1` list of rows no gate has accepted |
| feature-matrix probe suite | does that census still NOTICE a dropped non-goal, an uncited ontology clause, an undeclared diagnostic, a prose gate? | `bash docs/tasks/artifacts/feature_matrix/run_feature_matrix_probes.sh` → `probes: N pass / M fail` |
| standards census | is any external standard cited anywhere in the book without a registered role, a status from the closed vocabulary and a named owner? | `bash docs/tasks/artifacts/standards/run_standards_census.sh` → `standards census: N registered / M designations used / 0 failure(s)`, plus a per-designation list of where it is used |
| standards probe suite | does that census still NOTICE a smuggled citation, an invented status, an ownerless claim, a bare `read-in-repo`? | `bash docs/tasks/artifacts/standards/run_standards_probes.sh` → `probes: N pass / M fail` |
| changelog-ledger probes | is the changelog a ledger — live window in commit order, nothing both live and sealed, every sealed segment's sha256 and coverage claim true, pointer and segments closed? | `bash docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh` → `probes: N pass / M fail`; `LEDGER_PENDING=<id>` declares the entry this commit carries |
| fixture derivation | does the reference skirt still agree with itself — every §4 formula evaluating to its published number, all four closure checks closing, every piece accounted for by a span or a declared non-sewn attachment, §12's count matching §6's list, and §4's band width describing the same construction as §6's band pieces? (defects D27, D33) | `bash docs/tasks/artifacts/reference_fixture/run_fixture_derivation.sh` → `fixture derivation: N derived rows / M closure checks / K pieces / 0 mismatch(es)`; `FIXTURE_CHAPTER=<path>` points it at another copy |
| fixture derivation probe suite | does that instrument still NOTICE a faced two-piece band, an unaccounted piece, a wrong piece count, an edited formula, an undeclared token, a falsified closure, a changed constant? | `bash docs/tasks/artifacts/reference_fixture/run_fixture_probes.sh` → `probes: N pass / M fail` |
| glossary probe suite | does the census still NOTICE a duplicate token, an invented clause, a lost ⚠, an undeclared token, a drifted index? | `bash docs/tasks/artifacts/glossary/run_glossary_probes.sh` → `probes: N pass / M fail` |
| all probe suites | does every diagnostic probe in the repository still discriminate? (scratch pinned to this volume) | `make probes` → per-suite `probes: N pass / M fail`, then `12 suite(s) green` |
| Knowledge Map | is the derived orientation map in sync with its sources? | `knowledge-map/scripts/check_knowledge_map.sh`; regenerate with `knowledge-map/scripts/gen_knowledge_map.sh > "$(knowledge-map/scripts/gen_knowledge_map.sh --print-map-path)"` |

⚠ Run probe suites through `make probes`, which pins `TMPDIR` to `target/scratch` on the repository
volume: the inherited suites call `mktemp -d`, which otherwise lands on the system volume (defect
D16, leaf `SPINE.10`). The shared `scripts/check_task_acceptance.sh` still takes one trap-cleaned
scratch directory from `mktemp -d` at commit time; that residual is recorded in the leaf, not
patched into shared code.

⚠ **A RED arm must remove the property, not one instance of it.** Measured twice in one session: an arm
that renamed a diagnostic token renamed its declaration and its use together, and an arm that de-cited one
row left two other rows citing the same clause — both passed at `exit=0` where a refusal was owed, so both
reported the census sound while testing nothing. Before trusting a RED arm, ask what else in the tree still
satisfies the rule; if anything does, the arm is a report. Its sibling trap: a "portability" edit to a
regex (`{2,4}` → `###+`) silently stopped matching `##` headings, so a coverage rule required three clauses
fewer and still printed green — only a GREEN arm over the real tree plus a RED arm over one citation caught
it.

⚠ Probes and checks must **pin the instrument they measure with**. This machine's `PATH` puts GNU
coreutils ahead of BSD userland, so `stat -f %m` (BSD mtime) means "filesystem status of a file named
`%m`" and fails; `awk` is BSD awk 20200816, which lacks the GNU regex extensions `\b` and `{n}` that
`grep -E` here supports. A probe that measures with whatever is first in `PATH` measures the `PATH`.
