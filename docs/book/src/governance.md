# Governance

> **Status:** governance model, gate **G0** (roadmap §11: "governance model drafted — project owner named;
> sewist-vs-programmer review paths defined"; sources §12, §9, §7.8, §10, §13, §14). Drafted in full. Two of
> the three seats that need a **named human** are held acting by the director and one is openly vacant; §8
> states the acting authority, its hard limits, and the ask per seat, so the gap is one place instead of a
> discovery at each gate.

**Why this is written while the room is empty.** Roadmap §14 carries the risk "community fork over
governance" with the mitigation "governance doc at G0, while the room is empty", and §12 names the
cautionary tale: a sewing-CAD community that split over who decides what a factory needs. A governance
model written after the first conflict is a negotiation between parties; written before, it is a contract
both parties already signed. So this chapter decides *how* a disagreement is settled, *who* is competent to
settle each class of it, and *what happens when nobody is named yet* — and it does not decide anything the
roadmap left open elsewhere.

## 1. Two review paths, because two kinds of correctness exist

Roadmap §12 is explicit: **"Domain review is not code review."** A wrong notch default is a cut-floor
incident, not a failing test, and the reviewer must be a peer of the knowledge rather than of the code.
Every change therefore lands on exactly one path, and the paths do not substitute for each other.

| Change class | Path | Approval | What the approver judges |
| --- | --- | --- | --- |
| Rust, CLI, CI, doctrine gates — no exported byte changes | code review | maintainer | does it do what the spec says |
| a Factory Profile change that alters exported bytes | domain review, two-step | domain expert **and** maintainer | are these bytes right for a cutting room |
| a golden file re-freeze (§4) | golden approval | domain expert **and** maintainer | is the frozen claim still true |
| a glossary term, or any safety-relevant term (roadmap §7.6) | domain review | domain expert | does the word mean what a factory hears |
| a specification chapter that changes a construction claim | domain review | domain expert | is the construction real |
| a specification chapter that changes only wording or structure | code review | maintainer | does it still say what the model does |
| a translation or termbase contribution | the no-code review queue (§7.6, §12) | domain expert for the terms | is the term the one factories use |
| a contributed profile or golden from outside | two-step **plus** provenance (§4) | both, with the record | may this be published at all (§5) |

**The two-step rule.** Where a change alters exported bytes, the domain expert's approval and the
maintainer's approval are both required, and neither implies the other: the expert judges whether a cutting
room can use the bytes, the maintainer judges whether the change is consistent with the model, the release
contract (roadmap §9) and the conformance matrix (roadmap §13). A change with one of the two is unapproved,
and the tooling records which one is missing rather than reporting "partially approved".

## 2. Roles, and the authority each one needs

A role is defined by the decision it may make, not by the person holding it. "Currently" says who holds it
today; §8 lists the ones nobody holds.

| Role | Authority it needs | Agent-eligible? | Currently |
| --- | --- | --- | --- |
| project owner | scope, gate exit, the roadmap's revision policy and disposition log, the last word in §3 | no | director, acting (§8.1) |
| maintainer | merges code, owns CI and the doctrine gates, the release channels | no | the engineer of record |
| domain expert (sewing / factory) | the domain path in §1, golden semantics, safety terms, `assumed` constants | no | **vacant** — no acting holder (§8.1) |
| procurement owner | buys evaluation seats, the physical plotter, standards texts | no | director, acting (§8.1) |
| release approver | the human-only `approve` authority over a package (roadmap §9) | **never** | per package |
| independent evidence reviewer (G7) | signs the evidence pack for a production declaration | no | named at G7 (§8) |
| contributor | opens a change on one of §1's paths | yes, to `propose` | anyone |

Two rules bind the whole table:

- **Approval is human-only.** Roadmap §7.8: `inspect`, `propose`, `commit`, `generate` and `approve` are
  distinct permissions, and "approval is a human-only capability that a graph mutation can never
  manufacture". An agent may prepare a profile change, a golden re-freeze and even the *draft* of a
  governance ruling; it may not approve any of them. Mutating commands carry an actor trace (roadmap §10),
  so a ruling's provenance is a record and not a memory.
- **Independence is a rule, not a courtesy.** The G7 reviewer is "a named human outside the implementation
  team" (roadmap §11). This chapter adds the criterion that makes it checkable: the reviewer must not be an
  author of the code, the specification or the evidence under review. A reviewer who fails that test is a
  second opinion, and a second opinion is worth recording and is not an independent review.

## 3. Sewist-versus-programmer conflict resolution

The conflict roadmap §12 anticipates is not personal; it is structural. A sewist asks what a cutting room
needs and a programmer asks what the model can guarantee, and both answers can be correct.

1. **Classify the question before arguing about it.** A question about what a factory needs is a *domain*
   question and the domain expert rules. A question about what the model can guarantee — determinism, an
   error budget, a tolerance class — is an *engineering* question and the maintainer rules against the
   specification. Most conflicts dissolve here, because the two sides were answering different questions.
2. **A default is a profile parameter, not a verdict.** Where the two rulings genuinely differ about a
   default (a notch encoding, an allowance width, a corner treatment), the answer is the one the product
   already takes: the value becomes a Factory Profile parameter with both readings recorded, because two
   cutting rooms disagree about exactly these things (roadmap §8.3, and the ontology's inclusion policy).
   Governance does not have to pick a winner where the model can carry both.
3. **Escalation is a path, not an appeal to authority.** Reviewer → domain expert and maintainer jointly →
   project owner. Each step is recorded as a decision record with the evidence and the dissent, so the next
   conflict starts from the ruling instead of from the argument. There is no calendar in this project
   (roadmap §11), so the trigger is the review round, not a date: a conflict that survives one round goes up.
4. **A fork is a legitimate outcome, and the shared assets are the point.** The core is dual-licensed
   `MIT OR Apache-2.0` (ADR-0001), so anyone may leave with the code. What a fork cannot copy is the
   conformance corpus, the golden files and the profile registry — the evidence that a claim is earned.
   Governance worth staying for is governance whose rulings carry their evidence, and that is the whole
   reason a ruling is a record here rather than a message.

## 4. Golden-file approval ownership

A golden file is a frozen claim about bytes (roadmap §13: canonicalizer output, no timestamps, stable ids,
fixed float formatting, a cross-platform determinism policy). Freezing one is therefore a release-contract
event, not a test edit.

- **Two signatures, one record.** The maintainer signs the mechanical half — the bytes are canonical,
  deterministic and reproducible from the recorded inputs — and the domain expert signs the semantic half:
  the garment the golden represents is the garment the specification describes. The record names both.
- **No golden over an unreviewed assumption.** The reference fixture carries constants that are `assumed`
  pending domain review (§11 of that chapter), and a golden frozen over them would freeze a guess with the
  same confidence as a fact. The domain expert's confirmation of those constants is a **precondition of the
  first G2 golden**, which is the concrete dependency §8's blocked role creates.
- **Stale-ification applies to goldens too.** Roadmap §9: approval binds to package identity, and any input
  or artifact change creates a new candidate and stale-ifies the prior approval rather than inheriting it.
  A golden whose inputs changed is a new golden; a golden edited in place without a recorded reason is a
  defect, and the conformance suite treats it as one.
- **A contributed golden carries provenance or it is not admissible.** Roadmap §13's layers of truth put
  "import through actual target products with recorded options" at layer 4, so an externally produced golden
  must record the product, its version and the import or export settings that produced it. Without that
  record the file is an opinion about bytes, and the corpus is where opinions go to be checked.

## 5. What is public, and what never is

Roadmap §12 fixes the boundary: **public** — code, reference fixtures, sanitized contributed profiles,
golden files; **never** — customer data, body scans, factory trade secrets. Profiles are private by
default, and contribution is an explicit share action with a preview of exactly what leaves the machine.

Three governance consequences, because the boundary constrains the review paths in §1:

- A reviewer of a contributed profile sees only the sanitized artifact. No review path may require a trade
  secret to operate, or the path is closed to everyone who is not the contributor's employer.
- Where a domain expert genuinely cannot judge without the real file, the review happens on the
  contributor's side or under an agreement recorded in the decision — and the **ruling** is published
  without the data. The reasoning is the shared asset; the file is not.
- Sanitization is a checked step, not a promise: the share preview is the artifact a contributor approves,
  and publishing something the preview did not show is a defect against this chapter.

## 6. Agents under this governance

Roadmap §7.8 and §10 make agents first-class users and second-class authorities, and governance inherits
both halves:

- The five authority levels are command-layer concepts ([the command layer](spec/command-layer.md) §7
  specifies them): an agent may
  inspect, propose, commit within a scope and generate artifacts; approval stays human.
- Imported files are **data, never instructions** — which is a governance rule before it is a security one,
  because a factory's returned DXF is the most plausible carrier of a request that nobody in this project
  authorized.
- Independent checks evaluate artifacts, never the agent grading its own work (roadmap §7.8). The same rule
  is why every claim in this book is derived by a tracked census rather than asserted by its author.

### 6.1 Decisions made under delegation

This project is built by an engineer acting under the director's delegation, so most decisions are both
authored and applied by the same party. That is not a defect to hide and not a licence: it is a case with its
own rules, because self-application is where a governance model quietly stops being one.

1. **Record the author and the applier, and say so when they are the same.** A reader must be able to see that
   one party proposed a change and applied it. The roadmap's Appendix A entry for v0.3 does this: it names the
   proposal's record, the delegation it was made under, and the defect it closes.
2. **The author of a decision may not approve the evidence that decision requires.** Approval belongs to the
   gate's reviewer, who must satisfy §2's independence criterion — not an author of the code, the
   specification or the evidence under review. Where no independent reviewer exists, the claim stays
   **unapproved** and is recorded as unapproved; it is never approved by the party that needed it. This is the
   same rule as a vacant seat (§8.1), applied to a decision instead of a role.
3. **The consequences of a delegated decision are derived, not asserted.** Anything the decision obliges later
   work to prove gets an instrument somebody else can run — a census, a probe, a golden — so the check does not
   rest on the author's honesty or memory. The envelope-coverage criterion v0.3 added is checked against the
   trees by `run_tree_coverage_census.sh`'s clause-versus-leaf table, and its garments are owned by named
   leaves, so a future author cannot satisfy it with prose.
4. **The record states what would reverse it, and who may.** A delegated decision is the principal's decision
   made in their absence, so it carries its own re-open condition. Reversal must not require archaeology.
5. **A delegation to decide is not a delegation to upgrade evidence.** External material read here is labelled
   with its source and date, a claim about a standard stays governed by the standards chapter's closed
   vocabulary, and a decision resting on general practice stays `assumed` with its reviewer named — the rule
   the first delegation of `2026-09-30` stated and this section inherits.

## 7. Procurement, and the fallback when it slips

Each item names what is bought, why a gate needs it, who owns it, and the fallback with its cost in evidence
quality. Roadmap §14 already rules that an eval-seat slip is handled by "a named owner at G0" plus "a
partner-run manual test as documented fallback", so the fallback is normative rather than a consolation.

| Item | Gate that needs it | Owner | Documented fallback, and what it costs |
| --- | --- | --- | --- |
| evaluation seats in a commercial CAD | G2, G6 — import-diff | procurement (§8) | a partner runs it and records product, version and settings: layer-4 evidence, fewer targets |
| a physical plotter | G6 — plotted output | procurement (§8) | the printed scale-square check at G2 plus a partner plot: proves scale, not the device's quirks |
| standards texts (the six registered) | any `read-in-repo` claim | procurement (§8) | none — until a text is read here no clause may be quoted ([standards §1](spec/standards.md)) |
| a pilot factory partner | G6 — the rejection-reason taxonomy | project owner (§8) | none yet; the partner loop is the evidence, so this one cannot be substituted |

## 8. The three seats nobody holds, and who acts in them

The director's ruling of `2026-09-30` delegated the drafting of this chapter and reserved exactly one thing:
**naming humans**. He then delegated the finding itself, which changes what may be *decided* here and nothing
about what may be *certified*: a name is still his, and no engineering decision can manufacture the competence
a domain review requires. So two seats are held **acting**, with limits, one is openly **vacant**, and the ask
per seat is written down so that naming one is a single act rather than a negotiation.

| Role | Blocked since | Held by | What unblocks it |
| --- | --- | --- | --- |
| project owner | G0 exit clause | the director, acting (§8.1) | the director names one |
| domain expert (sewing / factory) | G0, and G2 depends on it | **nobody — vacant, not acting** (§8.1) | the director names one |
| procurement owner | G0 exit clause ("seats take months") | the director, acting, via §7's fallbacks | the director names one |
| independent evidence reviewer | G7 | not needed until G7 | named at G7 by the project owner |

### 8.1 Acting authority, and the one thing it cannot do

- **The project owner's seat is held acting by the director.** Everything that seat does inside this repository
  — scope, gate exit, the roadmap's revision policy and its disposition log, §3's last escalation step — is his
  already, so the arrangement grants no new authority and invents none. What it adds is a record: a ruling made
  in the seat is logged as the project owner's, so whoever is named inherits decisions instead of archaeology.
- **The procurement owner's seat is held acting by the director, and §7's fallbacks are the operative path.**
  No purchase waits on the naming; what waits is *ownership of the search*, which is why each §7 row states its
  fallback and what that fallback costs in evidence quality.
- **The domain expert's seat is vacant, and pretending otherwise is the one thing this chapter must not do.**
  No acting arrangement supplies sewing and factory competence. With the force of a gate, an acting holder may
  **not** confirm the reference fixture's `assumed` constants, may **not** sign the semantic half of a golden
  (§4), may **not** rule a safety-relevant glossary term, and may **not** approve a Factory Profile change that
  alters exported bytes — §1's two-step rule loses its first step, so such a change is unapproved and the
  tooling reports which half is missing rather than "partially approved". The first G2 golden is therefore gated
  on a named expert, and the fixture chapter says so where a plan will hit it.
- **Every affected clause sits in the `unknown` state, not `assumed`** (ontology §5): an unknown requires
  observation and may not be exported as if it were known. `G0-CONTRACT.15` records the G0 governance clause as
  `met — model drafted; two seats acting, the domain seat vacant` or as `not met`, and never as met on the
  strength of this chapter alone.
- **What the vacancy holds up is a list, not an impression.** `bash
  docs/tasks/artifacts/uncertainty/run_uncertainty_census.sh` enumerates every marker the book carries —
  `assumed`, `unknown`, `unverified-with-owner`, `read-external`, `(proposed)`, `vacant` — with the authority
  that resolves each, and **refuses** a blocking marker whose verification-status section names no resolver. So
  the day a name arrives, the work it unblocks is one command away; and an edit that quietly drops an `assumed`
  from a fixture constant changes a count somebody reads.

### 8.2 The ask, per seat

Naming a seat is one act, so each row says what the person must be able to do rather than what the project
hopes for:

- **Domain expert (sewing / factory)** — competence in woven-garment sample-making or production; able to read
  a piece list and a tech pack and to say whether a notch default, an allowance width or a corner treatment
  would be rejected on a cutting floor. Authority: to rule §1's domain path and to sign a golden's semantic
  half. Cost: a handful of review sessions, not a role in daily work. **This is the seat with a schedule behind
  it**, because the fixture's constants gate the first G2 golden.
- **Project owner** — authority to set scope, to close a gate, and to amend the roadmap under its revision
  policy. Held acting today, so naming it changes the record rather than the decisions.
- **Procurement owner** — authority to spend, and patience with lead times: evaluation seats in a commercial
  CAD take months, which is why roadmap §11 asks for the owner at G0 rather than at the gate that needs the
  seats.

## 9. What this chapter deliberately does not decide

Named so that each is a decision with a record when its time comes, rather than an emergency:

- **A contributor licence agreement.** ADR-0001 records that none is needed to contribute to a dual-licensed
  permissive core; reopening that needs the same strength of argument as the ADR's own re-open condition.
- **A foundation, a trademark policy, or a moderation role.** All three are real questions for a community
  that has one, and none is a question this repository can answer honestly today.
- **Paid maintainer funding.** Roadmap §12's governance bullet names funding and procurement *owners*, not
  a model; the model follows whoever funds the work, and pretending otherwise would be a claim without
  evidence.
- **A dispute process involving parties outside the project.** §3 ends at the project owner. What happens if
  a contributor rejects a ruling is a fork (§3, rule 4), not an arbitration.

## 10. Verification status, and where each rule is enforced later

- **Every clause above is cited from the roadmap** — §12 for the two review paths, the public/private
  boundary and the governance-before-community rule; §9 for approval, scope and stale-ification; §7.8 and
  §10 for agent authority and actor trace; §13 for goldens and provenance; §14 for the fork and eval-seat
  risks; §11 for the named-owner requirements. Where this chapter adds something the roadmap does not say —
  the independence criterion in §2, the classification step in §3, the two-signature golden rule in §4 — it
  is a **project decision**, and it is labelled as one rather than dressed as a citation.
- **Nothing here is enforced by prose alone forever.** The mechanical parts land as gates: the actor trace
  and the authority levels at G1 (the command layer §7 specifies the contract), the profile approval path and
  the evidence store at G4, the golden and provenance rules at G2/G6 in `conformance/`, and "governance in
  force" as a G7 exit criterion. A rule in this chapter that no gate ever enforces is a defect to be logged
  like any other, and §15's gate-exit review is where that audit happens.
