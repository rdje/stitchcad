# Sealed archive — StitchCAD changelog, the release-contract and canvas-spike-rule slices

Immutable historical segment, sealed out of `CHANGELOG.md` under the rolling-ledger protocol in
`LIVE_DOCUMENT_SIZE_CONTAINMENT.md`, performed by leaf `G1-SLICE.3b` in the commit whose append
crossed the window's byte health target (400 lines / 32 768 bytes) — a rollover `STITCHCAD-G1-0004`'s
append owed but did not perform (defect D54), discharged here.

- **Sealed identity:** 75 lines, 7188 bytes, `sha256:1e52b5c9b35b7de95360d82be692fd51f780a3a4e3401d505426d9c5a245c471`
- **Coverage:** `STITCHCAD-G0-0012` and `STITCHCAD-G0-0011`, newest first, exactly as they stood in `CHANGELOG.md`. No slice range is declared, for the reason `stitchcad-changelog-part9.md` records (defect D51).
- **Sealed by:** leaf `G1-SLICE.3b` on `2026-10-01`, after `stitchcad-changelog-part12.md`.
- **Retrieval:** the sealed content is everything below the `---` rule and the blank line after it, and it
  ends with exactly one newline — the byte contract `run_changelog_ledger_probes.sh` states after defect
  D43, whose `DESCRIPTOR` rule reproduces this digest on every `make probes`.
- **Write policy:** none — sealed segments are immutable.

---

## STITCHCAD-G0-0012 - the release contract is the roadmap's §9, compared rather than restated (leaf `G0-CONTRACT.12`)

Roadmap §9 was the last G0 clause with a promised chapter and no chapter: the spec index carried "Release and
approval" as an unlinked row and thirteen glossary references pointed at this leaf. §9 is a list - nine
manifest fields, six acceptance states, three artifact classes - and a list realised in prose drifts, so the
chapter ships with an instrument that parses the roadmap and compares.

- **the chapter** - `docs/book/src/spec/release-contract.md` (257 lines / 19 010 B): nine manifest fields
  each with the object it comes from; identity as the digest of the canonical manifest, artifact hashes
  included, so a package is never edited and any change makes a new candidate; seven completeness checks
  against the declared construction, so a geometrically valid file missing a piece is an invalid package; the
  six acceptance states in the roadmap's order with the evidence and the granter each needs, where a state
  may not be claimed without the rung below it and does not survive a new identity; five scope axes with the
  rule that a claim's scope is the INTERSECTION of its evidence, narrowing by itself and widening only by a
  new record; approval as a human act, with a package whose approver is not a human identity invalid; and
  §8.2's policy matrix tuned in a recorded table over a closed four-word disposition vocabulary, consulted
  for the `unknown` state alone, with the dependency closure recorded per artifact and eight diagnostics.
- **the tuning is visible, not silent** - the roadmap publishes the matrix as an example and invites G0 to
  tune it, so the chapter quotes each example row beside what it became: notch geometry in a draft export is
  a provenance `sidecar` instead of "default + visible badge", because a default is a value substituted for
  an observation and §8.3 forbids it. `permit` is deliberately absent from the matrix's cells - a cell that
  simply permitted an unknown would be a cell that invented one - and the census refuses one.
- **the instrument** - `run_release_contract_census.sh` parses roadmap §9 (parenthesis-aware, because one
  field carries a nested comma), §8.2's header and example rows, and the five states ontology §5 declares,
  then compares all three with the chapter in BOTH directions: a field the roadmap names and the chapter
  lacks is a refusal, and so is a field the chapter invents wearing the roadmap's authority.
  `9 manifest fields / 6 states / 8 matrix rows / 0 failure(s)`, with `run_release_contract_probes.sh` ->
  `14 pass / 0 fail` over a swapped ladder, an invented field, a dropped artifact class, an unrecorded
  tuning, an undispositioned state, an undeclared and an unused disposition, and a control.
- **decisions** - `docs/decisions/decision_release-package-identity-and-scope.md` records the four rules and
  six rejected alternatives, including the similarity threshold for stale-ification: a threshold is a guess
  about which differences a factory cares about, made by the party that wants the approval to survive.
- **D49's trigger fired and was discharged in this commit** - the evidence sibling had reached 1000 lines and
  96 % of its byte ceiling, so ten completed checklists (`G0-CONTRACT.2` … `.4b`, 560 lines / 52 573 bytes)
  were sealed into `docs/history/stitchcad-g0-contract-evidence-part1.md` under the descriptor contract and
  the live sibling fell to 449 / 42 065; `run_changelog_ledger_probes.sh` -> `9 pass / 0 fail` reproduces the
  digest. Four glossary terms were added, twelve entries repointed from this leaf to a clause, and one
  misrouting corrected (`spi` belongs to `G5-SHELLS.13`, not here).
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `19 suite(s) green`; `make book` ->
  exit=0; all eight other censuses green; containment `OK - 17 surfaces, 15 routes, 102 files measured`

## STITCHCAD-G0-0011 - the spike's rule is written before its measurement (leaf `G0-CONTRACT.11`)

ADR-0002 was the one ADR whose evidence does not exist yet, and nothing in the repository constrained what a
G1 spike would be allowed to conclude. The roadmap itself warns why that matters: "Custom wgpu, not DOM
canvas" is a hypothesis to test, not an axiom. So the rule was written first.

- **the record** - `docs/decisions/decision_adr-0002-ui-stack-and-canvas-spike-protocol.md` (116 lines):
  the chrome (Tauri + TypeScript/React, Slint as the named fallback, Flutter still rejected), the egui/iced
  dev shell and the TypeScript domain-logic ban are `active`; canvas hosting is `proposed`, in the status
  line rather than a footnote, because a decision whose evidence does not exist yet is still a decision
  *structure*. It carries the three topologies with the hypothesis each tests, a corpus declared before
  anybody measures it (16 pieces, 400 boundary vertices each, 200 pick probes, 200 snap probes, 1 000
  fidelity round trips) with a re-run trigger if a G3 garment exceeds it, seven gates each naming what it
  protects, and six rules: eligibility, correctness outranking speed, scoring inside a declared margin with
  a total tiebreak order, a one-renderer preference where that is free, escalation with a bounded fallback,
  and a verdict a human may overrule only by a recorded decision naming the rows.
- **the data plane and the instrument** - `docs/tasks/artifacts/canvas_spike/` holds the gates, the
  applicability table and the rule parameters as three TSVs, plus a `results.tsv` that is empty on purpose:
  `run_spike_verdict.sh` prints `PENDING` with exit=0 until `G1-SLICE.13` measures, refuses a data set that
  cannot produce a verdict, and otherwise prints the rule that decided each profile. Tightening a threshold
  is therefore a diff a reviewer sees, which the GATES-READ arm pins by changing the verdict.
- **the probe suite** - `run_spike_verdict_probes.sh` -> `13 pass / 0 fail` over twelve synthetic result
  sets whose verdicts are known in advance: a tie broken by memory, the fastest topology losing to
  `snap_exact = no`, a faster native-only winner giving way to one renderer for both profiles (R4), a
  profile with no survivor escalating to the `dev-shell` fallback (R5), an unmeasured gate, an undeclared
  topology, a missing row, a control and a missing plane. Six arms failed first against a correct
  instrument because each omitted a row the applicability table declared - recorded as the slice's lesson.
- **the consumer** - `G1-SLICE.13`'s acceptance now names the instrument by path, the corpus by its
  declared numbers, and requires the hardware, OS versions and corpus script identity in the results file,
  because a verdict is scoped to them.
- **the dev-notes ledger rolled over** in the commit whose append crossed it (`devnotes-part4`, 42 lines /
  3706 bytes, digest reproduced by `run_changelog_ledger_probes.sh` -> `9 pass / 0 fail`).
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `18 suite(s) green`; `make book` ->
  exit=0; all seven book censuses unchanged; containment `OK - 17 surfaces, 15 routes, 99 files measured`
