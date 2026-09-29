# Live-document containment is adopted in proportion to the repository, with deferrals named

- **Type:** `decision`
- **Date:** `2026-09-29` (absolute)
- **Status:** `active`
- **Owner / source:** repo-local workflow, leaf `SPINE.4.1`; adopts the director's external
  containment doctrine and its adoption guide (session directive §18)

answers: "do we use the live-document size containment doctrine?" · "why is there no JSONL registry and no neutral size checker yet?" · "when do we adopt the full containment package?" · "what stops CHANGELOG.md or a task tree from growing without bound?" · "who sets the size ceilings and how?"

## The fact / decision

1. `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` is adopted at the repository root: the neutral body verbatim,
   behind a fenced StitchCAD adoption note. This copy is authoritative; the external reference is
   read-only and is not an upstream.
2. **The adoption is partial, and the note says so.** Adopted now: the doctrine text, the surface
   inventory with lifecycle classes, measured pressure axes, health targets, inclusive enforcement
   ceilings, the routing-destination inventory, and one deterministic checker in the project doctrine
   slot (`SPINE.4.2`, `SPINE.4.3`).
3. **Deferred:** the neutral JSONL-registry checker package (a ~2 100-line interpreter plus the
   archive-descriptor, ledger-manifest, derived-state, ceiling-increase-authority and
   version-retention registries). Three triggers reopen the decision, and whoever hits one owns it:
   - a surface reaches its ceiling and cannot be trimmed → adopt the atomic partition / rollover /
     archive protocol and the archive-descriptor contract;
   - the inventory exceeds roughly 24 surfaces, or the awk checker becomes the limiting factor →
     evaluate the neutral checker and migrate the two TSV registries to its JSONL data plane;
   - a stored copy of a mechanically owned value needs an executed freshness oracle → adopt the
     derived-state contract family.
4. **Local parameters:** warn at 80 % of a health target; the enforcement ceiling is inclusive
   (equality passes, excess fails); a ceiling rises only by a recorded authority (a decision record
   plus the registry row citing it), never to land content. The data plane is bounded **TSV** parsed
   with awk, so no new interpreter enters the commit path.

## Why

Containment exists to stop a bounded file from routing its overflow into an unbounded neighbour — the
measured upstream failure was a status file that reached 1 547 057 bytes, 94.7 % of it dated changelog,
after a README cap displaced the pressure instead of removing it. This repository has that shape of
risk in front of it: 18 specification chapters, a changelog entry per slice, and task-tree files that
grow one evidence section per leaf (`docs/tasks/SPINE.md` was already 772 lines / 60 433 bytes at
adoption).

It does not yet have the shape of problem the full package solves: no partitioned archive, no rolling
ledger with sealed ranges, no derived-state copies needing freshness oracles, no ceiling-increase
history. Adopting a 2 100-line interpreter and eight registries to govern eleven documents would put a
large unreviewed dependency in the commit path of a project whose entire product surface is still a
specification — and would add a non-Rust interpreter to a Rust workspace's hooks.

The proportionality call is recorded rather than left implicit because "we adopted the doctrine" and
"we adopted part of it" are different claims, and a reader who assumes the full package is enforcing
something would trust a gate that does not exist. Naming the deferrals with triggers converts a
silent gap into tracked work.

## How to apply

- Before growing a live document, find its row in `.doctrine/live_document_size/surfaces.tsv`
  (`SPINE.4.2`): lifecycle class, owner, health target, ceiling. No row means the surface is
  unclassified, and the checker refuses.
- A ceiling increase needs a decision record and a registry row citing it. Trimming the content is
  almost always the right answer instead; `MEMORY_ARCHITECTURE.md` §6 and `README_POLICY.md` both say
  never to raise a cap to fit content.
- When a trigger fires, adopt the corresponding part of the package and update the adoption note in
  the same commit — the note is the record of what is and is not enforced.
- Related: [[decision_adopted-external-policy-references]] (how external policy is adopted at all),
  [[decision_scaffold-sync-protects-project-content]] (why this file cannot be clobbered by a sync).
