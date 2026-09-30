# Sealed archive — G0-CONTRACT decisions, the leaves before `.9`

Immutable historical segment, sealed out of the `## Decisions` section of `docs/tasks/G0-CONTRACT.md`
under the remedy defect **D49** names for a tree file's ledger tail, performed by leaf `G0-CONTRACT.17`
at the trigger D49 declares: the tree had reached 94 % of its `tasks_collection` byte ceiling before
this slice added its own decisions.

- **Sealed identity:** 112 lines, 10079 bytes, `sha256:8d22b367857e8c08efbf184e338c402884dcf92b36200a51ce120b39fd8f2045`
- **Coverage:** the dated decision entries of leaves `.1`, `.4`, `.6`, `.7`, `.13d`, `.4b`, `.14` and
  `.4c`, plus the two `2026-09-29` entries that opened the section, exactly as they stood. Each is a
  layer-B record of why the tree did what it did; the cross-cutting ones also live in `docs/decisions/`,
  which is where a later session looks first.
- **Sealed by:** leaf `G0-CONTRACT.17` on `2026-09-30`.
- **Retrieval:** the sealed content is everything below the `---` rule and the blank line after it, and it
  ends with exactly one newline — the byte contract `run_changelog_ledger_probes.sh` states after defect
  D43, whose `DESCRIPTOR` rule reproduces this digest on every `make probes`.
- **Write policy:** none — sealed rationale is immutable; a superseding decision is a new entry in the
  live section.

---

- `2026-09-29`: normative G0 specifications live in the mdBook under `docs/book/src/spec/`
  (the director reviews the book); ADR-style *decisions* live in `docs/decisions/` (memory
  layer C) and are summarised — never duplicated — in the book. Realises roadmap §4.3's
  `docs/adr/` as `docs/decisions/` deliberately.
- `2026-09-29`, **superseding the same day's earlier "no product code in G0" reading**: G0 owns
  exactly two skeleton crates and the G0 CI workflow (`.18`), because §4.3 and §7.3 state the G0
  CI shape and the G0 WASM smoketest in terms of `sc-core` + `sc-units` compiling. The earlier
  reading took "crates appear when their stage starts" as an absolute and would have left two
  roadmap clauses unowned. Consequence: `.18` is a CODE leaf and carries the full acceptance
  checklist with tool output; every other G0 leaf remains documentation-only.

- `2026-09-30`, leaf `.1`: the glossary is **partitioned into eight domain parts behind one index
  chapter**, not one file. Reason, measured: a single file would have been ~70 KB of five-column rows
  against a `book_collection` per-part health of 24 576 bytes and a ceiling of 40 960 — a termbase is not
  the shape of a prose chapter, and one file would have breached its ceiling within two more chapters.
  Consequences: the A–Z index is **derived** from the parts and compared against them by the census in
  both directions; a new term goes in the part whose domain it belongs to and in no other; every part
  carries the same five columns, so the termbase extraction `G0-CONTRACT.16` owes has one shape to read.
- `2026-09-30`, leaf `.1`: a **machine token has exactly one owner**, and an entry that shares another's
  token writes `→ token`. Recorded as `docs/decisions/decision_machine-tokens-declared-where-used.md`
  with the measurement behind it, because the rule binds every later chapter and every crate.

- `2026-09-30`, leaf `.6`: a **size label is prose, not a token**, so labels are written in quotes in the
  specification and never in a machine-token style. The glossary census found the violation (three example
  labels in backticks were reported as undeclared tokens) before it became a convention in the chapters
  that follow.
- `2026-09-30`, leaf `.4`: a feature matrix row whose proof no gate has accepted says **`unnamed (D32)`**
  rather than borrowing a gate. Assigning a gate would invent a commitment on that gate's behalf, and a
  silently borrowed gate is how an envelope claim becomes untestable. The census prints those rows on
  every run (advisory `A1`), so the gap is closed by a decision at `.15`, not by being forgotten.
- `2026-09-30`, leaf `.4`: a citation in the matrix's reason column repeats its source per clause
  (`ontology §1, ontology §1.1`, never `ontology §1 and §1.1`), because the census reads citations
  mechanically and a bare `§1.1` after a comma is ambiguous between two documents with overlapping clause
  numbers.

- `2026-09-30`, leaf `.7`: the glossary's derived A–Z index is the reason `glossary.md` grows one line per
  term (`405` lines now, against a `400`-line per-part health and a `700`-line ceiling). The remedy is
  already mechanical and is recorded here so it is not rediscovered as a surprise: when the index passes
  ~500 lines it splits by letter range into two derived halves, because `--emit-index` generates whatever
  the census compares. Owner: the next leaf that adds a batch of terms.

- `2026-09-30`, leaf `.13d`: the fixture's waistband is **one straight band, cut once and folded at its
  midpoint**, plus one interfacing piece cut at the band's finished dimensions and fused — five pieces, not
  the faced reading's six. The choice is sourced, not preferred: the drafting references prescribe the
  two-piece cut for a **contoured** band, and this fixture's band is straight at the natural waist.
  Recorded as `docs/decisions/decision_reference-fixture-waistband-straight-folded.md` with its five
  sources, their URLs and the date read, the one disagreement between them (`wb_width`), and the re-open
  condition.
- `2026-09-30`, leaf `.13d`: **every piece is accounted for — by a span or by a declared non-sewn
  attachment from a closed list**, which holds `fused` alone. "Every piece has a span" is false for a fused
  interfacing, and the false invariant is what let D27's inner band pass for nine commits; stating the
  invariant so a fused piece is *declared* rather than *missing* is what made it checkable. The list is
  closed on purpose: a sewn-in interlining is sewn, so it would need a span.
- `2026-09-30`, leaf `.13d`: an external source that is not a standard is labelled **`read-external`** with
  its URL and the date it was read, and it never upgrades a claim about a standard — that vocabulary stays
  closed in `docs/book/src/spec/standards.md` §1. A source read and *not* used is recorded too, so the next
  session does not re-read it.
- `2026-09-30`, leaf `.13d`: the fixture's arithmetic has a **tracked producer**,
  `docs/tasks/artifacts/reference_fixture/run_fixture_derivation.sh`, with `run_fixture_probes.sh` as its
  ground truth. Until this leaf the numbers that found D33 and D27 were `python3 -c` strings inside task
  leaves — re-runnable by nobody, the leg-3 breach this repository had already committed once as D20.

- `2026-09-30`, leaf `.4b`: **permission is not a criterion.** G3's note that an intermediate garment "may
  be inserted without shame" schedules nothing, so the proposed amendment adds one exit criterion over
  roadmap §3.2's whole garment list — closing the class rather than the four instances — and names the
  failure mode: a garment the envelope names and no exit criterion proves is a gate failure, not a scope
  note. Recorded in `docs/decisions/decision_d32-proving-gates-proposed-roadmap-amendment.md`.
- `2026-09-30`, leaf `.4b`: a gate this chapter *proposes* is written `(proposed)` in the cell, and the
  census prints every such cell on every run (advisory `A3`). A provisional commitment that reads like a
  settled one is the D32 gap wearing better clothes, so the proposal is as visible as the gap it replaces —
  and a probe arm removes the markers and requires the printed count to fall, because an advisory that reads
  nothing prints the same number either way.
- `2026-09-30`, leaf `.4b`: a **deferred** row names the gate that declares the limitation (G7, per rule 3
  of the matrix's §1), not a gate that proves the feature. That is why the fly needs no amendment while the
  four supported rows do, and why the `lining` row was already right.
- `2026-09-30`, leaf `.4b`: the dev-notes rollover is performed in the commit whose append crossed the
  window's health target, and the archive verifier's `DESCRIPTOR` rule is generalized to every
  `docs/history/*.md` segment in the same commit — a sealed segment whose digest nothing watches is a
  "trust me" with a hash beside it. The Coverage and pointer legs stay changelog-only, which is logged as
  D40 and owned by `SPINE.19` rather than left implicit.

- `2026-09-30`, leaf `.4b`: **a tree past 1000 lines splits its completed-leaf evidence into a sibling
  file**, which is the containment registry's own remedy and not an invention of this slice — and the leaf
  being landed keeps its checklist in the tree file, because `scripts/check_task_acceptance.sh` judges every
  staged `docs/tasks/*.md` and refuses one with no ticked boxes. Measured, not assumed: emptying the tree
  file of checklists would have turned an honest slice into a `TASK-ACCEPTANCE` refusal. Consequence: the
  tree file's first matching box is now always the current leaf's, which is D15's facet 1 closed by
  structure rather than by care. `G0-CONTRACT-evidence.md` is the sibling; `SPINE.md` owes the same split
  (defect **D42**, owned by `SPINE.4.4`).

- `2026-09-30`, leaf `.14`: **a role is the decision it may make, not the person holding it.** Every role in
  the governance chapter carries the authority it needs and whether an agent may hold it, which is what makes
  an empty seat assignable instead of mysterious: the work is specified and only the name is missing. The
  three empty seats are in ONE table (§8) rather than discovered at the gate that needs each of them.
- `2026-09-30`, leaf `.14`: **a contested default becomes a profile parameter, not a verdict.** Where a sewist
  and a programmer genuinely disagree about a default, the model already carries both readings (roadmap §8.3),
  so governance records the dissent instead of picking a winner — and §3 classifies the question first,
  because most such conflicts are two correct answers to different questions.
- `2026-09-30`, leaf `.14`: **a procurement fallback states what it costs in evidence quality.** Roadmap §14
  already makes the partner-run manual test the documented fallback for an eval-seat slip; §7 of the chapter
  adds the cost of each fallback (a partner run is layer-4 evidence: slower, fewer targets), because a
  fallback without a stated cost is how a procurement slip silently downgrades the release claim.

- `2026-09-30`, leaf `.4c`: **a proposal is a state, not a resting place.** `.4b` marked four cells
  `(proposed)` because amending the roadmap was reserved; the moment the reservation was lifted, the honest
  action was to apply it through the roadmap's own revision machinery — version marker, Appendix A disposition
  entry, containment baseline re-based in the same commit — and not to leave a ratified decision wearing a
  provisional label. The `(proposed)` mechanism and the A3 advisory stay in place for the next one.
- `2026-09-30`, leaf `.4c`: **an exit criterion must arrive with owners.** A roadmap clause no leaf owns is
  the D32 defect one level up, so the same commit that added G3's envelope-coverage criterion made
  `G3-GRADING.5` required (trousers with a pocket and a derived buttonhole), created `G3-GRADING.15` (the
  classic collar) and made `.14`'s exit review fail if a §3.2 garment has no leaf's evidence behind it.
