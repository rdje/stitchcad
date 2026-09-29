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
| fresh-evidence check | does THIS commit add its own ticked, evidence-backed acceptance boxes for the code it stages? (project doctrine `FRESH-ACCEPTANCE-EVIDENCE`) | `scripts/check_fresh_acceptance_evidence.sh` (runs inside `make gate`); arms: `--self-test` |
| fresh-evidence probe suite | does the fresh-evidence doctrine refuse stale evidence and still accept a co-staged documentation tree? | `bash docs/tasks/artifacts/fresh_evidence/run_fresh_evidence_probes.sh` → `probes: N pass / M fail` |
| task-acceptance probe suite | does the inherited acceptance gate still hold its shipped properties? | `bash docs/tasks/artifacts/task_acceptance/run_task_acceptance_probes.sh` |
| Rust gate | do fmt, clippy (deny warnings) and the tests pass? | `make check` |
| Knowledge Map | is the derived orientation map in sync with its sources? | `knowledge-map/scripts/check_knowledge_map.sh`; regenerate with `knowledge-map/scripts/gen_knowledge_map.sh > "$(knowledge-map/scripts/gen_knowledge_map.sh --print-map-path)"` |

⚠ Inherited probe suites call `mktemp -d`, which lands on the system volume; pin scratch to the
repository volume with `TMPDIR="$PWD/target/scratch"` (defect D16, leaf `SPINE.10`).

⚠ Probes and checks must **pin the instrument they measure with**. This machine's `PATH` puts GNU
coreutils ahead of BSD userland, so `stat -f %m` (BSD mtime) means "filesystem status of a file named
`%m`" and fails; `awk` is BSD awk 20200816, which lacks the GNU regex extensions `\b` and `{n}` that
`grep -E` here supports. A probe that measures with whatever is first in `PATH` measures the `PATH`.
