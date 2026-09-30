# Director's ruling of 2026-09-30 (second): no domain expert and no independent reviewer exist, and the project proceeds with claims marked unapproved

- **Type:** `decision` (a director ruling, recorded when it is made — the precedent is `SPINE.12`)
- **Date:** `2026-09-30` (absolute)
- **Status:** `active`
- **Owner / source:** the project owner and director, in response to the findings surfaced at the end of the
  slice sequence `G0-CONTRACT.9`–`.17`. His words: *"honestly I am not in contact any sewing or factory
  domain expert and so far there is no independent reviewer. I do not think, I could be wrong, that these
  are blocking points, in that the project can move on as long as we do not need their intervention, right?"*
  The engineer's answer, which this record adopts with its boundary stated: **yes — the seats do not block
  progress, they block certification.**

answers: "can the project proceed without a sewing expert?" · "can an AI agent substitute for domain knowledge?" · "what does the vacant domain seat actually block?" · "who approves the G0 exit review?" · "what happens to receiver validation with no evaluation seat?" · "when does the missing seat become a schedule problem?"

## The ruling

1. **No sewing/factory domain expert exists and none is being sought now.** The seat stays **vacant** (not
   acting — competence cannot be held acting, per `decision_unnamed-seats-acting-authority.md`), and every
   clause that needs it stays `unknown` or `assumed` with the seat named as its resolver.
2. **No independent reviewer exists.** Where governance §6.1 requires independence, the claim is recorded as
   **unapproved** rather than blocked, and the mitigation §6.1 already prescribes applies: the consequences
   of a decision become instruments anybody can run, so independence is *available* to whoever comes next
   instead of being held by anybody now.
3. **The project proceeds.** Gates G1 through G5 are engineering-evidence gates and none of them needs
   either seat. Work is not paused, deferred or re-sequenced around the vacancies.
4. **Evaluation-seat procurement is not started and has no searcher.** The consequence is accepted rather
   than hidden: no target system reads our artifacts back, so the interchange contract's claims remain
   `cited-from-roadmap`, and G6's receiver validation falls to the roadmap's documented fallback — a
   partner-run manual test — or does not happen. That is a decision with a cost, not an open hope.

## The boundary: what proceeds, and what cannot happen without a seat

| Proceeds on engineering evidence alone | Cannot happen, and when it would be needed |
| --- | --- |
| G1: the executable slice — three profiles, the command bus, persistence, one CSP constraint, the canvas spike | a golden's **semantic** half: bytes may be frozen as regression artifacts (they detect change) but never as correctness artifacts (they do not prove a garment fits a body) — G2 onward |
| G2: geometry, offsets, the DXF and PDF writers, the print check, mutation and property tests | the reference block set's transcription review (`G3-GRADING.16`); obtaining the source is only a purchase, which the director can do alone |
| G3: construction, closures, both instantiation paths, grading and `.rul` | a shipped language pack's `safety` and `strict` tiers ([i18n §9](../book/src/spec/i18n-architecture.md)) — G5 |
| G4: profiles, the CSP, the evidence store, the policy matrix | the fixture's five `assumed` drafting constants becoming `known` — needed before a physical toile means anything |
| G5: the shells, the UX, English-only | any **production claim to a factory**, which is G7 and is the point where this ruling's cost becomes visible to a customer |

Two clarifications the ruling depends on:

- **"Independent reviewer" and "domain expert" are different vacancies.** For an *engineering* clause,
  independence means "not the author": a fresh session, another engineer, or the director reading the book
  and re-running the cited instruments all satisfy it. Only *domain* clauses need sewing competence. The
  self-application problem is therefore procedural and cheap; the domain problem is real and distant.
- **Nothing here launders a gap into a pass.** A clause whose evidence needs a seat is recorded `not met`
  with the seat named, or `met (unapproved)` where the engineering evidence is complete and only the
  approval is missing. The uncertainty census still enumerates every marker with its resolver, so the
  population waiting on a name stays one command away.

## How to apply

- `G0-CONTRACT.15` records the G0 exit review under this ruling: the procurement clause `not met` with the
  fallback named, the governance clause met as drafted with the vacancy disclosed, and the gate's **closure
  unapproved** because its reviewing party authored most of what it reviews (governance §6.1 rule 2).
- Any leaf that would otherwise wait for a seat proceeds and marks the affected value `assumed` or
  `unknown` with this record cited, so the reason is a ruling and not an oversight.
- A future session that finds this record must **not** treat the vacancies as a blocker. If it believes a
  seat is now on the critical path, the test is the table above: name the gate and the deliverable that
  cannot proceed, and escalate that, not the vacancy.
- If a domain expert or an independent reviewer ever appears, the population they unblock is derived by
  `docs/tasks/artifacts/uncertainty/run_uncertainty_census.sh`, and the goldens, termbase entries and
  fixture constants marked `assumed` are the first things they sign.

## Amendment, same day: the residual dependency is a measurement, not a credential

The director's follow-up — *"we have access to almost the entire world knowledge with internet and can
process all the information with AI agents, so I think we can go very far and do almost everything without
ever needing an expert, except maybe in some very rare cases"* — is adopted, with the boundary stated
precisely, because the word "expert" bundles three different things:

1. **Knowledge — substitutable, and an agent is arguably better than one human.** Drafting systems, grading
   conventions, seam construction, fabric behaviour and interchange conventions are extensively documented,
   and sourced synthesis beats recall. The evidence is in this repository: defect D27 (two waistbands in one
   fixture for nine commits) was settled by reading five sources with URLs and dates, not by asking a person.
2. **Judgement when sources disagree — substitutable under one discipline.** An agent may enumerate the
   disagreement, choose a reasoned default and record both readings. The failure mode is a confident
   synthesis of a convention that is *plausible* and wrong in the tradition a given factory works in, so the
   rule is unchanged: a synthesized default stays `assumed` and cited, and never becomes `known`.
3. **Physical truth — not substitutable, and this is not an AI limit.** "This pattern produces a wearable
   skirt" is a claim about cloth, and the only oracle is cloth. Roadmap §13 already puts physical checks at
   the top of its layers of truth, and V2's exit forbids fit claims before physical evidence.

The consequence is that the missing dependency is **cheaper than a seat**: the first toile of the reference
skirt needs a machine, a printer, a ruler and an afternoon, and it falsifies or confirms all five of the
fixture's `assumed` drafting constants at once. `G2-2D.15` (created by `G0-CONTRACT.15`) owns that protocol,
written so that whoever sews it — the director, a partner, a sample room, a hired sewer — produces evidence
this project can ingest without interpreting it themselves.

A second consequence: **at G6 the factory is the expert.** The pilot loop's rejection-reason taxonomy is
domain review from the party with the strongest incentive to give it honestly, so the project needs to
*reach a factory*, which is a business step the director owns, rather than to recruit a sewing expert.

What remains genuinely rare, and none of it on a critical path before G5: signing a physical-evaluation
record (any competent sewer, no credential required), and reviewing ⚠ termbase entries before a non-English
pack ships (a native-speaking sewer).

## What would reverse it

The director naming either seat, or a partner factory agreeing to a manual validation run, which converts
clause 4's accepted cost into evidence. Until then this ruling stands and the project moves.
