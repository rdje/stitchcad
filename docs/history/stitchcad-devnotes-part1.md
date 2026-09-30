# Sealed archive — StitchCAD dev notes, the 2026-09-04 … 2026-09-29 lessons

The oldest lessons of `DEV_NOTES.md`, sealed when the live window passed its health target — the
rolling-ledger protocol `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` prescribes, performed by leaf
`G0-CONTRACT.4b` in the same commit as the append that crossed it.

**Immutable.** A correction is a superseding record in the live file, never an edit here (the rule the
changelog's D30 correction established).

- **Coverage:** the four lessons that were oldest when the window rolled — `a gate that scopes evidence to
  a bullet still scopes it to the WRONG leaf` (2026-09-29), `a template's trial must include the first
  commit` (2026-09-04), `a green gate that judges nothing is the class a template must not ship`
  (2026-09-04), and the `bootstrap` entry.
- **Sealed identity:** 66 lines, 5589 bytes, `sha256:d3b94e9a3c799c070bea1efcaee9aa20aed0e4715e966180a8d13b19132016b6` — the segment exactly as it stood
  in `DEV_NOTES.md`.
- **Retrieval:** `sed -n '<rule+2>,$p' docs/history/stitchcad-devnotes-part1.md | shasum -a 256` reproduces
  the digest, and `docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh`'s `DESCRIPTOR` rule runs
  it over every segment in `docs/history/` on every `make probes`.

---

## _(2026-09-29)_ — a gate that scopes evidence to a bullet still scopes it to the WRONG leaf

- The inherited `TASK-ACCEPTANCE` check was distilled to close two leakage holes (a co-staged
  unrelated file supplying the tokens; a token matched anywhere in the file). Both closures hold.
  A third hole sits one level up and is measured, not argued: the check takes the **first** bullet
  matching each label *in the file*, so with two leaves in one file the verdict follows section
  order, not ownership. `HOLE-1` → a change owned by leaf 2 accepted on leaf 1's evidence
  (`exit=0`); `HOLE-2` → an honest, evidenced leaf refused because a future leaf's placeholder sat
  above it (`exit=1`); `HOLE-3` → delete the placeholder, same leaf passes. Probe:
  `docs/tasks/artifacts/task_acceptance/run_multileaf_shadowing_probe.sh` → `probes: 6 pass / 0 fail`.
- The general lesson: **box-scoping is a within-bullet property; attribution is a within-unit
  property.** A gate can satisfy the first and still answer for the wrong unit whenever several
  units share a file. The shipped probe suite tested the file boundary because that is where the
  founding incident happened, and a probe suite bounded by its own incident reads as a proof of the
  wider claim in the header. Promoted to `docs/decisions/decision_acceptance-evidence-per-leaf.md`.
- Corollary for authors, and the reason this is a convention and not just a bug report: placeholders
  are not inert decoration. An unticked box is a *claim about a leaf*, and in a first-match gate it
  is a claim that can silence a genuinely ticked one. Do not pre-create checkboxes for work not
  yet done.
- **Facet 3 arrived on the commit that published the probe.** Staging a `.sh` file made it a code
  commit, so the check judged *every* staged leaf file — including a documentation tree owning no
  code — and refused it: `❌ TASK-ACCEPTANCE … docs/tasks/PLANNING.md — the 'ROOT CAUSE' box is
  ticked but carries no tool-output evidence`, `exit=1`. The box it read (`PLANNING.md:155`) cited
  two census commands and their real listings; recognized signature families inside that bullet:
  `0`. So the refusal was correct under the gate's contract and the evidence was still honest — the
  missing element was a *result token*. Hence two rules: cite invocation + output + **exit status**,
  and never co-stage an unrelated tree with a code change. Also note the asymmetry that shapes the
  local fix: a project-slot check can add refusals but cannot relax a universal one.
- **Measure a gate with the gate's own instrument.** The first cut of the local fix matched evidence
  signatures in awk, and a self-test arm failed on `error[E0432]` / `rc=1`. The tempting reading —
  "the inherited signature list is not portable, log it as a defect" — was wrong, and was killed by
  one command: the universal check tests signatures with `grep -qE`, and both GNU grep 3.12 and BSD
  grep 2.6.0-FreeBSD match `\brc=[0-9]+` and `error\[E[0-9]{4}\]`. Over a 36-line corpus of realistic
  evidence, awk left 12 unmatched and grep left 2 (both bad samples). The defect was in the new
  instrument, not in the gate it was measuring; the candidate defect record was withdrawn, and the
  check now pins the engine choice with a `GREEN-2` arm. Corollary: `\b` and `{n}` are GNU extensions
  that BSD awk lacks, so awk is for structure here and grep is for signatures.
- **Pin the instrument you measure with — same lesson, second instance, one day later.** The
  scaffold-sync probe's "an identical file is not rewritten" control read mtimes with `stat -f %m`
  and got filesystem dumps: this machine's `PATH` puts GNU coreutils ahead of BSD userland, where
  `-f` means *filesystem status* and `%m` becomes a filename operand. The arm printed a verdict about
  data it never read. Fixed by pinning `/usr/bin/stat` and failing loudly when neither the BSD nor the
  GNU form yields a number. General rule, now in `TOOLBOX.md`: a probe that measures with whatever is
  first in `PATH` measures the `PATH`.

## _(2026-09-04)_ — a template's trial must include the first commit

- Every gate was green on the generated project and the first commit still failed: the doctrines judge STAGED
  code, and nothing had been staged until the user tried. Trial the path a user walks, to its end.
- `grep -c` prints `0` and exits 1. `$(grep -c … || echo 0)` therefore yields `0⏎0` — a second line — which
  here started a flush-left line inside a checklist bullet and hid its evidence from the box-scoped extractor.
  Capture the count, then default the empty case; never append a fallback to grep's own output.

## _(2026-09-04)_ — a green gate that judges nothing is the class a template must not ship

- Two of the four doctrine ports in `.2.6` were wrong on first run and their own RED self-test arms said so:
  a `python3 - <<'PY'` detector whose stdin was the heredoc (every arm read 0 rows), and a `grep -c … | grep -qx 0`
  control under `pipefail` (`grep -c` prints 0 and exits 1). A self-test with only GREEN arms would have passed both.
- The neutrality bar is measured, not felt: `grep -ciE 'grammar|parser|…'` over each ported script → 0, after the
  generic uses of "corpus" and "grammar" were re-worded ("tree", "syntax") so the count means what it says.

## _(YYYY-MM-DD)_ — bootstrap

Repo created from the `bedrock` template: durable 4-layer memory, task-tree tracking, the
strict commit workflow, and the mechanical doctrine enforcer are in place and enforced by
git hooks + CI. No project code yet.
