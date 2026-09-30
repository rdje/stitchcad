# A decision made under delegation records that its author applied it, and never approves its own evidence

- **Type:** `decision`
- **Date:** `2026-09-30` (absolute)
- **Status:** `active` — governance §6.1 is its normative text; this record carries the reasoning and the
  founding instance
- **Owner / source:** leaf `G0-CONTRACT.14c`, acting on the director's instruction to decide and act on the
  three findings surfaced at the end of the `SPINE.4.4`/`.4c` session. Finding 1 was that the engineer had
  proposed the `ROADMAP.md` §11 G3 amendment in `G0-CONTRACT.4b` and then applied it in `.4c` under a second
  delegation — a conflict of interest disclosed in a report rather than governed by a rule.

answers: "may the engineer both propose and apply a change?" · "who approves the evidence a delegated decision requires?" · "what stops a self-applied decision from being self-certified?" · "how is a conflict of interest handled when one party holds every seat?" · "what must a delegated decision record carry?"

## The decision

Self-application under delegation is **permitted, disclosed and bounded**. Governance §6.1 states the five
rules; the three that do real work are:

1. **The record names the author and the applier, and says so when they are the same party.** Concealment is
   the failure mode, not the coincidence — a single party holding the pen is how this project is built, and a
   rule that pretended otherwise would be a rule nobody could follow.
2. **The author of a decision may not approve the evidence that decision requires.** Approval belongs to the
   gate's reviewer under governance §2's independence criterion (not an author of the code, the specification
   or the evidence under review). Where no independent reviewer exists, the claim stays **unapproved** and is
   recorded as unapproved — never approved by the party that needed it.
3. **The consequences are derived, not asserted.** Anything a delegated decision obliges later work to prove
   gets an instrument somebody else can run, so the check does not rest on the author's honesty or memory.

## Why

- **The alternative to a rule is a disclosure, and a disclosure decays.** Finding 1 was handled by naming the
  conflict in a session report — which the director would have to remember, and a future session would not see
  at all. A rule in the governance chapter is read by whoever acts next; a sentence in a report is read by
  whoever was there.
- **Every seat being held by one party is this project's actual condition, not an edge case.** Governance §8
  records two seats acting and one vacant, so there is no second party to hand a decision to. A governance model
  that only works with a populated room is a model that does not work here, and the honest response is to bound
  what a single party may do rather than to pretend there are two.
- **The bound that matters is on *certification*, not on *action*.** Waiting for an independent approver before
  acting would stall the project; letting the author certify that their own decision worked would make every
  gate a formality. So the decision proceeds, its consequences become instruments, and the approval that proves
  them waits for a reviewer who is not the author. The first G2 golden is exactly this case: it cannot be
  frozen while the domain seat is vacant (§8.1), and the engineer who drafted the fixture cannot review it.

## The founding instance, and where each rule bites

Roadmap **v0.3**'s envelope-coverage criterion was proposed by the engineer in `G0-CONTRACT.4b` as an exact
amendment marked *proposed*, then applied in `.4c` when the director delegated the finding. Each rule's
discharge is a thing in the tree, not an intention:

- **Disclosure** — the roadmap's Appendix A entry names the source ("the engineer's proposal under the
  director's delegation"), the defect it closes (D32), and the record that carries the argument; this record
  states that the proposer and the applier were the same party.
- **No self-approval** — `G3-GRADING.14`'s exit review must produce per-garment evidence and may not be signed
  by the party that proposed the criterion; the domain half of any golden needs the vacant seat (§8.1).
- **Derived consequences** — `run_tree_coverage_census.sh` compares each gate tree's clause rows against the
  roadmap's own clause count (advisory, printed every run), the criterion's garments are owned by named leaves
  (`G3-GRADING.5`, `.15`), and the matrix's cells are checked by `run_feature_matrix_census.sh`, so "the
  envelope is covered" is a command rather than a claim.
- **Reversibility** — the proposal record keeps its rejection path: if a later revision withdraws the
  criterion, the four matrix cells revert to `unnamed (D32)` and the census's A1 advisory lists them again on
  every run.

## How to apply

- **Acting under delegation:** write the decision record with its author, its applier, its evidence and its
  re-open condition; turn each consequence into an instrument; and leave the approval to a reviewer who is not
  you. If no such reviewer exists, record the claim as unapproved and name the seat that owes it.
- **Reviewing a self-applied decision:** check the three things a disclosure cannot fake — is the instrument
  tracked and runnable, is the approval withheld rather than self-signed, and is the reversal path written down.
- **Do not** treat the rule as a reason to defer action until a human arrives. The point of §6.1 is that a
  single party can decide and act at signoff grade, provided the certification of the outcome does not belong
  to them.

Related: [[decision_governance-two-review-paths-and-the-unnamed-roles]] ·
[[decision_unnamed-seats-acting-authority]] · [[decision_d32-proving-gates-proposed-roadmap-amendment]] ·
[[decision_director-ruling-2026-09-30-four-findings]] · `docs/book/src/governance.md` §2, §6.1, §8.1 ·
`ROADMAP.md` Appendix A (the v0.3 entry).
