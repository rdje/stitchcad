# Sealed archive — G0-CONTRACT tree changelog, the leaves before `.9`

Immutable historical segment, sealed out of the `## Changelog` section of `docs/tasks/G0-CONTRACT.md`
under the remedy defect **D49** names for a tree file's ledger tail, performed by leaf `G0-CONTRACT.15`
when the tree reached 99 136 bytes against a `tasks_collection` per-part ceiling of 98 304 — a breach,
not a warning, so the seal is part of the commit that would otherwise have been refused.

- **Sealed identity:** 48 lines, 4651 bytes, `sha256:00682c0ccf26c3eddb94ac7f7e8526c0cf6f4e6f54d5699b3d6c93e0cae02d60`
- **Coverage:** the four tree changelog entries that were the live window's oldest — `.14c` and `.19`,
  `.4c`, `.14` and `.4b` — newest first, exactly as they stood. There is no overlap with the earlier seal:
  `stitchcad-g0-contract-changelog-part1.md` holds `.13d` and everything before it, so the two segments
  together are the tree's changelog from `.13d` backwards, and the live window keeps `.9` onward.
- **Sealed by:** leaf `G0-CONTRACT.15` on `2026-09-30`, after `…changelog-part1.md`.
- **Retrieval:** the sealed content is everything below the `---` rule and the blank line after it, and it
  ends with exactly one newline — the byte contract `run_changelog_ledger_probes.sh` states after defect
  D43, whose `DESCRIPTOR` rule reproduces this digest on every `make probes`.
- **Write policy:** none — sealed evidence is immutable; a correction is a superseding note in the live
  window.

---

- `2026-09-30`: `.14c` and `.19` landed, acting on the director's instruction to decide and act on the session's
  three findings. `.14c` governs self-application under delegation — governance §6.1's five rules (record the
  author and the applier; the author never approves the evidence their decision requires; consequences become
  instruments; the record states its reversal; a delegation does not upgrade evidence), the roadmap's v0.3
  disposition entry now discloses that one party proposed and applied it, and `G3-GRADING.14` may not be signed
  by the criterion's author. `.19` makes the vacancy's consequences enumerable: the uncertainty census lists
  every marker the book carries with the authority that resolves it and refuses a blocking marker whose status
  section names nobody, with six probe arms — and building it found two defects in existing instruments, the
  glossary's link resolver (one `gsub` could not normalise `../../`) and this census's own first authority list
  (a bare gate id counted as an owner, which made the rule nearly unfailable).

- `2026-09-30`: `.4c` landed — the D32 proposal is ruled approved and applied. Roadmap **v0.3** gives §11 G3
  an *envelope coverage* exit criterion (every garment §3.2 names drafts, grades and exports at that gate or an
  earlier one), logged in Appendix A with its source, and the four matrix cells dropped `(proposed)`. The
  criterion arrives with owners: `G3-GRADING.5` became required and `.15` was created, and `.14`'s exit review
  now fails if a §3.2 garment has no leaf's evidence. Applying it needed a mechanism the data plane did not
  have — the `roadmap` debt baseline was measured at exactly the file's size, so a legitimate revision could
  only land by hand-widening a number — which is `SPINE.4.5`'s revision-aware baseline, and the containment
  adoption note's deferred trigger 3, fired and discharged. `PLANNING.6` fixed the coverage census that the
  evidence siblings had broken (D44) and put it under `make probes`.

- `2026-09-30`: `.14` landed — the governance model is drafted in full: every change class lands on exactly
  one of two review paths, a change that alters exported bytes needs the domain expert **and** the maintainer,
  seven roles are defined by the decision each may make and whether an agent may hold it, a golden is a
  release-contract event with two signatures and is never frozen over an `assumed` constant, a contested
  default becomes a profile parameter rather than a verdict, and every procurement item carries its fallback
  with what the fallback costs in evidence. The three seats that need a named human are in one table for the
  director, and the four questions deliberately left open are named so they arrive as decisions rather than
  emergencies. Six of the chapter's rules are project decisions and are recorded as such in
  `docs/decisions/decision_governance-two-review-paths-and-the-unnamed-roles.md`. The slice also obeys the
  evidence-split convention `.4b` recorded: `.4b`'s checklist moved to `G0-CONTRACT-evidence.md`.

- `2026-09-30`: `.4b` landed — D32 is resolved: classic collar, trousers, button/buttonhole and pocket are
  assigned to **G3** (buttons also to **G5**, whose tech-pack clause already requires notions) and fly
  construction to **G7**, whose exit already requires named limitations. No row says `unnamed (D32)` any
  more — the census's A1 advisory reports `0` — and the four cells that depend on a roadmap change say
  `(proposed)`, which the new A3 advisory prints on every run with a probe arm that requires it to notice a
  removed marker. The exact amendment (one added G3 exit criterion over §3.2's garment list, plus the
  rewritten complexity note) is quoted current-vs-proposed with line numbers in
  `docs/decisions/decision_d32-proving-gates-proposed-roadmap-amendment.md`, marked **proposed**: the
  roadmap is the director's to amend and `.15` puts it to him. The slice also performed the dev-notes
  rollover its own append triggered (four lessons sealed byte-identically, digest verified) and generalized
  the ledger probe's DESCRIPTOR rule to every `docs/history/*.md` segment, with a `DEVNOTES-DIGEST` arm
  pinning the generalization; D40 records the two legs that stay changelog-only, owned by `SPINE.19`.

Older entries — `.13d` and everything before it, back to the tree's creation by `PLANNING.1` — are sealed
in [`docs/history/stitchcad-g0-contract-changelog-part1.md`](../history/stitchcad-g0-contract-changelog-part1.md)
(84 lines, 7941 bytes, `sha256:3a6d091a…`), under the remedy defect D49 names for a tree file's ledger tail.
