# Sealed archive — StitchCAD dev notes, two more 2026-09-30 lessons

Immutable historical segment, sealed out of `DEV_NOTES.md` under the rolling-ledger protocol in
`LIVE_DOCUMENT_SIZE_CONTAINMENT.md`, performed by leaf `G0-CONTRACT.11` in the commit whose append
crossed the window's byte health target (200 lines / 16 384 bytes).

- **Sealed identity:** 42 lines, 3706 bytes, `sha256:c2ac579105a1eb9efd8fa040ddc08ac0d55d2c8749f5ee07d2a6487d31c4cb18`
- **Coverage:** the lessons `a blocked leaf splits into what can be drafted and what must be named` and
  `permission is not a criterion, and a proposal must be as visible as the gap it replaces`, both dated
  2026-09-30, oldest last, exactly as they stood.
- **Sealed by:** leaf `G0-CONTRACT.11` on `2026-09-30`, after `part3` (sealed by `G0-CONTRACT.9`).
- **Retrieval:** the sealed content is everything below the `---` rule and the blank line after it, and it
  ends with exactly one newline — the byte contract `run_changelog_ledger_probes.sh` states after defect
  D43, whose `DESCRIPTOR` rule reproduces this digest on every `make probes`.

---

## _(2026-09-30)_ — a blocked leaf splits into what can be drafted and what must be named

- `.14` sat blocked for a session because it needed three named humans. The director's ruling separated the
  two halves — draft everything, name nobody — and the whole chapter turned out to be draftable: a role is
  the **decision it may make**, not the person holding it, so each seat could be specified with its authority,
  its agent-eligibility and what waits for it. What was blocking was the *coupling* of the two halves, not the
  missing names. General shape: before calling a leaf blocked, ask which of its clauses actually needs the
  missing input; usually one does and the rest were waiting beside it.
- **Governance written while the room is empty is a contract; written after the first conflict it is a
  negotiation.** Roadmap §14's mitigation is literally "governance doc at G0, while the room is empty", and
  the reason shows up in the drafting: every rule that had to be invented (which changes take the domain
  path, what makes a reviewer independent, who signs a golden) is cheap to state now and expensive to state
  after somebody has a position. Promoted to
  `docs/decisions/decision_governance-two-review-paths-and-the-unnamed-roles.md`.
- **A fallback that does not state its cost is how a slip becomes a silent downgrade.** The eval-seat fallback
  (a partner runs the test) is roadmap-mandated, so the only honest addition was what it costs: layer-4
  evidence with recorded product, version and settings — slower, fewer targets. Same shape as a claim with a
  missing leg: name the leg, do not hide it.
- **The empty seat with a schedule behind it is the one to name loudly.** The domain expert gates the fixture's
  `assumed` constants, which gate the first G2 golden, so the chapter says so where a plan will hit it rather
  than where a reader will sympathise.

## _(2026-09-30)_ — permission is not a criterion, and a proposal must be as visible as the gap it replaces

- D32's five rows were not a documentation gap but a **roadmap** gap: §3.2 promises a collar and trousers,
  ontology §4.7 models buttons and pockets, and no gate's exit criteria proved any of them. G3's note that
  an intermediate "may be inserted without shame" is a licence, and a licence proves nothing at a gate
  review. The amendment this slice proposes closes the *class* — every garment §3.2 names drafts, grades
  and exports — rather than naming four features, so the next envelope addition inherits a proof
  requirement instead of needing its own amendment. Promoted to
  `docs/decisions/decision_d32-proving-gates-proposed-roadmap-amendment.md`.
- **A provisional commitment that reads like a settled one is the same defect in better clothes.** So the
  cells say `(proposed)`, the census gained an A3 advisory printing every proposed cell on every run, and a
  probe arm removes the markers and requires the printed count to fall — an advisory that reads nothing
  prints the same number either way, which is the vacuous-green shape the glossary census shipped once as
  its R1 bug.
- **The engineer prepares the amendment; the director applies it.** The ruling reserved `ROADMAP.md`, so the
  record quotes the current text with its line numbers beside the proposed text, states what happens on
  approval and on rejection, and the matrix acts consistently with a *proposal*. A reservation is not a
  blocker: the work lands, and the one thing that is not the engineer's to do is named rather than done.
- The token census fired a fifth time at authoring time (`` `lining` `` and `` `proposed` `` wearing token
  formatting in prose): both were de-tokenized, not exempted.
