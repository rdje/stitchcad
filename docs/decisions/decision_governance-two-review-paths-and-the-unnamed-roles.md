# Governance: two review paths, roles defined by the decision they may make, and three seats that are empty

- **Type:** `decision`
- **Date:** `2026-09-30` (absolute)
- **Status:** `active` — the model is adopted; the three named humans it needs are still the director's to
  name (his ruling of `2026-09-30` reserved exactly that, and nothing else in this area)
- **Owner / source:** leaf `G0-CONTRACT.14`, drafting the governance model roadmap §11 G0 requires
  ("governance model drafted; project owner named; sewist-vs-programmer review paths defined") from §12, §9,
  §7.8, §10, §13 and §14

answers: "who approves a profile change that alters exported bytes?" · "who may re-freeze a golden file?" · "what happens when a sewist and a programmer disagree?" · "which roles still need a named human?" · "can an agent approve anything?" · "what is the fallback if evaluation seats never arrive?"

## The decision

`docs/book/src/governance.md` is the model. Six of its rules are **project decisions** — they are what the
roadmap's prose commits to once it has to be operated, and each is recorded here so a later reader can tell
the citation from the invention:

1. **Two review paths, and a change lands on exactly one.** Code review judges whether the implementation
   does what the specification says; domain review judges whether the artifact is right for a cutting room.
   Roadmap §12 requires the separation ("domain review is not code review"); the *classification table* that
   assigns each change class to a path is this project's, because an unclassified change goes to whoever
   answers first.
2. **The two-step rule is conjunctive, and the tooling says which half is missing.** A change that alters
   exported bytes needs the domain expert *and* the maintainer. "Partially approved" is not a state this
   product reports, because a state a receiver can misread is worse than a refusal.
3. **A role is the decision it may make, not the person holding it.** Every role in §2 carries the authority
   it needs and whether an agent may hold it, which is what makes the empty seats in §8 assignable rather
   than mysterious: the work is specified, only the name is missing.
4. **Independence gets a criterion.** Roadmap §11 requires the G7 reviewer to be "outside the implementation
   team"; this chapter makes it checkable — not an author of the code, the specification or the evidence
   under review. A reviewer who fails that is a second opinion, which is worth recording and is not an
   independent review.
5. **Classify the question before arguing about it, and make a contested default a profile parameter.** Most
   sewist-versus-programmer conflicts are two correct answers to different questions; the residue is a
   default two cutting rooms genuinely disagree about, and the model already carries both readings as Factory
   Profile parameters (roadmap §8.3). Governance does not need a winner where the product can hold both.
6. **A golden is a release-contract event with two signatures, and never frozen over an `assumed` constant.**
   The maintainer signs the mechanical half (canonical, deterministic, reproducible), the domain expert the
   semantic half (the garment is the one the specification describes). The reference fixture's `assumed`
   constants therefore gate the **first G2 golden** — the concrete cost of an unnamed domain expert, stated
   where a schedule will hit it rather than where it can be missed.

## Why now, and why this shape

- **The room is empty, which is the only time this is cheap.** Roadmap §14's risk row is "community fork over
  governance", mitigated by "governance doc at G0, while the room is empty". A model written after the first
  conflict is a negotiation between parties who already have positions; written before, it is a contract both
  signed. §12's cautionary tale is a real sewing-CAD community that split over exactly this.
- **A fork is a legitimate outcome, so the rulings must carry their evidence.** The core is dual-licensed
  permissive (ADR-0001), which means nobody can be held here by licence. What a fork cannot copy is the
  conformance corpus, the goldens and the profile registry — the evidence that a claim is earned. That is
  why §3 requires every ruling to be a decision record with its dissent, and why this record exists at all:
  governance worth staying for is governance whose reasoning survives the person who decided.
- **The fallback is normative, not a consolation.** Roadmap §14 already rules that an eval-seat slip is
  handled by "partner-run manual test as documented fallback", so §7 states each item's fallback *and what
  it costs in evidence quality* (a partner run is layer-4 evidence with recorded product, version and
  settings — slower, fewer targets, and honest about it). A fallback without a stated cost is how a
  procurement slip becomes a silent downgrade of the release claim.

## What stays open, deliberately

No contributor licence agreement (ADR-0001 records that none is needed to contribute to a dual-licensed
permissive core), no foundation, trademark policy or moderation role, no funding model, and no dispute
process reaching outside the project — §3 ends at the project owner, and a contributor who rejects a ruling
forks. Each is named in §9 of the chapter so that it arrives as a decision with a record rather than as an
emergency.

## The blocked seats, and what each one costs

| Seat | Blocked on | What waits for it |
| --- | --- | --- |
| project owner | the director names one | §3's last escalation step; the G7 reviewer's appointment |
| domain expert (sewing / factory) | the director names one | the domain review path, golden semantics, safety terms, and the fixture's `assumed` constants — so the **first G2 golden** waits |
| procurement owner | the director names one | evaluation seats ("seats take months" — roadmap §11), the physical plotter, the standards texts |

Until a name exists, each affected clause sits in the uncertainty vocabulary's `unknown` state (ontology
§5): not silently defaulted, and blocking what it governs. `G0-CONTRACT.15` records the G0 governance clause
as `met — model drafted, named owner pending`, or as `not met`, and never as met on the strength of the
chapter alone.

Related: [[decision_director-ruling-2026-09-30-four-findings]] · [[decision_adr-0001-license-and-solver]] ·
[[decision_reference-fixture-waistband-straight-folded]] · `docs/book/src/governance.md` ·
`docs/book/src/spec/standards.md` §7 · `docs/tasks/PLANNING.md` (the defect census).
