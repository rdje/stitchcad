# A containment debt baseline is revision-aware: it names the revision it was measured at, and a stale one is refused

- **Type:** `decision`
- **Date:** `2026-09-30` (absolute)
- **Status:** `active` — implemented in `scripts/check_live_doc_size.sh` (the `at=<token>` debt axis) and in
  the `roadmap` row of `.doctrine/live_document_size/surfaces.tsv`; it is the containment adoption note's
  **deferred trigger 3, fired and discharged** rather than left owed
- **Owner / source:** the slice that applied roadmap **v0.3** (the §11 G3 envelope-coverage amendment,
  defect D32) and found that a legitimate revision could not land: the `roadmap` row's transition debt was
  measured at exactly the file's size, so any growth was refused as a widened baseline

answers: "how do I amend ROADMAP.md without the containment gate refusing?" · "what does `at=v0.3` mean in a debt column?" · "may a transition-debt baseline ever grow?" · "when does a document's revision identity matter to a size gate?" · "what happened when containment trigger 3 fired?"

## The decision

A debt baseline may carry a third axis, `at=<token>`, naming the **revision of the file the baselines beside
it were measured at**. The checker reads the file's first line and refuses the row when that line no longer
declares the token:

```
LIVE-DOC-SIZE: roadmap: its debt baseline was measured at revision `v0.2`, which ROADMAP.md no longer
declares (first line: `# StitchCAD — Roadmap v0.3 (functionality-first, deadline-free)`) — a revision
re-bases its own baseline in the same commit, under a recorded authority
```

Three rules follow, and each closes a way this could have been faked:

1. **A baseline still never widens on its own.** `at=` does not disable the size comparison; declaring the
   right revision and growing past the baseline is refused exactly as before. The revision token is a
   *freshness oracle for the stored number*, not a licence.
2. **A revision re-bases in its own commit.** The only way to grow a baselined file is to change its revision
   marker and re-measure the baseline in the same diff — which is the same commit that must carry the
   recorded authority for the change (for the roadmap: its Appendix A disposition log entry). Growth is
   therefore never silent, and a reviewer sees the marker, the numbers and the reason together.
3. **`at=` is only legal on a `kind=file` surface.** A collection has no single first line, so a revision
   token on one is refused rather than ignored — an unread check is the failure this repository keeps
   measuring (the glossary census's R1 rule, the matrix probe's two vacuous arms).

## Why

- **The alternative was a hand-widened baseline.** The doctrine says a baseline never grows, and the checker
  enforced that by comparing numbers — so the honest-looking way to land a legitimate amendment was to edit
  the number, which is precisely the silent widening the rule exists to prevent. A rule whose only compliant
  path is "do not change the document" gets bypassed; the fix is to give the legitimate case a path that is
  *more* visible than the illegitimate one, not to relax the comparison.
- **This is trigger 3 of the containment adoption, fired.** `decision_live-document-containment-proportionate-adoption.md`
  deferred the neutral checker package behind three triggers, the third being "a stored copy of a
  mechanically owned value needs an executed freshness oracle", and the `roadmap` row's own note named
  "a revision-aware baseline+delta adapter … deferred (adoption note trigger 3)". The amendment hit it, and
  the record says whoever hits a trigger owns it. What is adopted is the *contract*, implemented locally in
  the existing awk checker (six lines and one measurement field) — not the 2 100-line neutral interpreter,
  whose other two triggers have not fired.
- **The freshness oracle is executed, not documented.** `CLAIM_VERIFICATION.md` §5B's rule is that a constant
  which is a function of this repository is derived or gated. A baseline measured "at v0.2" is a stored copy
  of the file's size at a moment; without an oracle it goes stale silently the day the file is revised, and
  then it either blocks legitimate work or, worse, is edited by whoever is blocked.

## How to apply

- **Amending a baselined document:** change its revision marker, make the edit, re-measure
  (`wc -lc <file>`), write the new numbers *and* the new `at=` token into the debt column, and cite the
  authority for the change in the row's `notes` — all in one commit. The refusal message names the file and
  the revision it expected, so a missed re-base is diagnosed in one line.
- **Adding a baseline to another surface:** include `at=<token>` whenever the file declares a revision
  identity of its own; omit it for files that have none (a task tree, a status snapshot), where the plain
  `lines=`/`bytes=` comparison is the whole rule.
- **Do not** silence a stale-baseline refusal by deleting the `at=` axis: that converts a freshness oracle
  back into a frozen number, which is the state this record exists to leave.

Measured when adopted: `ROADMAP.md` `919` lines / `50 821` B at v0.2 → `947` / `52 818` B at v0.3 (+28 /
+1 997), the baseline re-based with `at=v0.3`; the checker's `--self-test` grew from `11` to `15` arms
(`GREEN-AT`, `RED-AT-STALE`, `RED-AT-WIDEN`, `RED-AT-KIND`), and
`docs/tasks/artifacts/live_doc_size/run_live_doc_size_probes.sh` gained `REAL-3`, which stales the *real*
registry copy to `at=v0.2` and requires the refusal → `probes: 5 pass / 0 fail`.

Related: [[decision_live-document-containment-proportionate-adoption]] ·
[[decision_d32-proving-gates-proposed-roadmap-amendment]] ·
[[decision_maxline-health-derived-from-the-cell-budget]] · `.doctrine/live_document_size/surfaces.tsv` (the
`roadmap` row) · `ROADMAP.md` Appendix A (the v0.3 disposition entry).
