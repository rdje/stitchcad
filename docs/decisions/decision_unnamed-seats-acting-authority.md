# An empty seat is held acting with hard limits, or it is openly vacant — never silently defaulted

- **Type:** `decision`
- **Date:** `2026-09-30` (absolute)
- **Status:** `active`
- **Owner / source:** the governance leaf `G0-CONTRACT.14` and the slice that acted on the director's second
  instruction of `2026-09-30` ("make the necessary calls and every needed action" for the three findings). The
  first ruling reserved **naming humans** to the director; the second delegated the finding, which changes what
  may be decided about a vacancy and nothing about who may fill it.

answers: "who acts as project owner until one is named?" · "can the engineer review a sewing decision?" · "what is blocked by the missing domain expert?" · "how does a project proceed with an empty seat without faking it?" · "what exactly do I need to name for each role?"

## The decision

Three seats in `docs/book/src/governance.md` need a named human. They are handled in two classes, and the
difference between the classes is the whole decision:

1. **A seat whose authority the director already holds is held *acting*, and the acting is recorded.** The
   project owner's seat (scope, gate exit, the roadmap's revision policy, §3's last escalation step) and the
   procurement owner's seat (with §7's documented fallbacks as the operative path) are his today. Naming them
   changes the *record*, not the decisions — so rulings made in the seat are logged as the seat's, and a
   successor inherits decisions instead of archaeology.
2. **A seat whose authority is competence is *vacant*, and no acting arrangement may imply otherwise.** The
   sewing/factory domain expert supplies knowledge nobody in this repository has. An acting holder therefore
   may **not**: confirm the reference fixture's `assumed` constants; sign the semantic half of a golden file;
   rule a safety-relevant glossary term; or approve a Factory Profile change that alters exported bytes — which
   under §1's conjunctive two-step rule leaves such a change **unapproved**, reported as missing its domain half
   rather than as "partially approved".

Every clause affected by a vacancy sits in the uncertainty vocabulary's `unknown` state (ontology §5), not
`assumed`: an unknown requires observation and may not be exported as if it were known. The concrete
consequence with a schedule attached is that **the first G2 golden is gated on a named domain expert**, because
a golden frozen over an unreviewed `assumed` constant freezes a guess with the confidence of a fact.

## Why

- **The alternative to a bounded acting authority is silent defaulting**, which the whole uncertainty model
  exists to forbid. A seat that is "blocked" with no statement of who acts in it is a seat whose decisions get
  made by whoever is holding the pen, unlabelled — and the reader of the record cannot tell a ruling from an
  assumption.
- **Certification is not delegable to the person who needs the certificate.** The engineer who drafts a fixture
  cannot also be the domain reviewer who confirms its constants; that is the same reason roadmap §11 G7 requires
  an *independent* evidence reviewer, and governance §2 states the criterion (not an author of the code, the
  specification or the evidence under review). Calling an acting holder a reviewer would satisfy the wording of
  the two-step rule and destroy its meaning.
- **Writing the ask makes naming a one-step act.** §8.2 states, per seat, what the person must be able to do,
  what authority the seat carries, and what it costs — so the director's decision is "yes, this person" rather
  than a scoping exercise. An unspecified seat stays empty longer, and the domain seat is the one with a
  dependency chain behind it (fixture constants → first G2 golden → the conformance corpus every later gate
  cites).

## How to apply

- **Acting in a seat:** log the ruling as the seat's, cite §8.1, and do not let the acting period create
  precedent the seat cannot revoke — a ruling recorded as the project owner's is revisable by the project owner.
- **Facing a vacancy:** state it. A gate review, a golden freeze or a profile approval that needs the vacant
  seat is `not met — domain seat vacant (§8.1)`, never `met` on an acting signature. `G0-CONTRACT.15` records
  the G0 governance clause as `met — model drafted; two seats acting, the domain seat vacant` or as `not met`.
- **When a seat is named:** update governance §2's "currently" column and §8's table, close the `unknown` state
  for the clauses it governed, and record the name's provenance (who, competent in what, appointed by whom) in
  this record's place — a named reviewer whose competence nobody recorded is the next vacancy in disguise.
- **Do not** widen the acting authority to cover the vacant seat because a schedule presses. The pressure is
  real and the remedy is §7's fallbacks, which state their cost in evidence quality, not a borrowed signature.

Related: [[decision_governance-two-review-paths-and-the-unnamed-roles]] ·
[[decision_director-ruling-2026-09-30-four-findings]] ·
[[decision_reference-fixture-waistband-straight-folded]] · `docs/book/src/governance.md` §2, §4, §7, §8 ·
`docs/book/src/spec/reference-skirt.md` §11.
