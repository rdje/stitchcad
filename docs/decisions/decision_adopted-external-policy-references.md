# External policy references are adopted as repository-owned copies, never depended on

- **Type:** `decision`
- **Date:** `2026-09-29` (absolute)
- **Status:** `active`
- **Owner / source:** repo-local workflow, leaf `SPINE.3`; director's session directives §12, §14,
  §17, §18 and §21 (external references are read-only; other repositories are never written to)

answers: "where does the README size policy come from?" · "is CLAIM_VERIFICATION.md ours or borrowed?" · "do we sync policy documents from other repositories?" · "why does README_POLICY.md have an adoption note above the policy?" · "what do we owe after adopting a policy?"

## The fact / decision

1. A policy the director points at outside this repository is **copied in**, and the copy here is
   authoritative. The origin is a read-only source, not an upstream: there is no automatic
   synchronization, and a later revision is adopted only by deliberate local review.
2. The adopted copy keeps the **project-neutral body verbatim** and carries a fenced
   **local adoption note** above it — authority, date, owning leaf, reviewed measurements, current
   enforcement ceilings, routed destinations, and an **adoption frontier** listing what the policy
   obliges that this repository does not yet do, each with an owning leaf.
3. Donor-project values are never copied: not their caps, measurements, surface identifiers, task ids,
   registry paths or conclusions. Those are evidence about the donor, not portable policy.
4. Nothing in the build, the gates or the code may depend on a path outside this repository. Adopted
   text is the only thing that crosses the boundary.

Adopted so far: `README_POLICY.md` (refreshed to the revised neutral body — authority/provenance,
the duplication probe, routing-pressure closure, derived caps, unconditional checking) and
`CLAIM_VERIFICATION.md` (the three legs, the publishing contract, the grading axes, the two
mechanizations). Both carry adoption notes; `CLAUDE.md` routes every agent through the second.

## Why

A rule that lives in another checkout is a rule this repository cannot enforce, cannot version and
cannot survive: the repository may be moved to another volume or another machine, and the session
directive requires every project path to be repository-relative precisely so that moving it breaks
nothing. Copying also removes the ambiguity about which text binds — the adoption note names the
authority, so a reader never has to guess whether the local file or a distant one wins.

The adoption frontier exists because adopting a policy creates obligations, and an obligation nobody
owns is a policy nobody keeps. Recording it in the note (and in the task tree) is what makes the gap
visible: at adoption this repository's README ceilings were still the inherited template defaults,
which the adopted body explicitly forbids copying, and the destination registry its routing-pressure
section requires did not exist.

## How to apply

- To adopt or refresh a policy: copy the neutral body, write the fenced adoption note, measure what
  the note claims, list what the policy obliges that is not yet true, give each item an owning leaf,
  and wire discovery (`CLAUDE.md`, `README.md`) without making a bootstrap file the authority.
- Never write to the source repository, never pin a submodule or a path to it, and never make a gate
  read it. Read it, copy what binds, cite it in prose at most.
- When a spine sync runs, `scripts/update_scaffold.sh` classifies `README_POLICY.md` as
  PROJECT-CONTENT: it is backed up and skipped, because the adoption note is ours. See
  [[decision_scaffold-sync-protects-project-content]].
- A claim published under this standard carries its legs: re-derived by command, falsified against an
  oracle we did not build, and durable (tracked producer). A missing leg is stated, not hidden — and
  the local breach of that rule found during adoption is defect D20, owned by leaf `SPINE.11`.
