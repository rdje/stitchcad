# A release package is never edited, an approval binds to a digest, and a scope narrows by itself

- **Type:** `decision`
- **Date:** `2026-09-30` (absolute)
- **Status:** `active`
- **Owner / source:** leaf `G0-CONTRACT.12`, from roadmap §9 (signoff and release contract), §8.2 (the
  artifact policy matrix) and §15.8; the normative text is `docs/book/src/spec/release-contract.md`

answers: "what does an approval bind to?" · "can a released package be amended?" · "why is there no similarity threshold for stale-ification?" · "what does an unresolved unknown block?" · "how wide is a factory's acceptance?" · "may an agent approve a release?"

## The decision

Four rules, each closing a door that would otherwise be opened by convenience:

1. **A package is immutable.** Any change to an input, an artifact or a manifest field produces a *new*
   candidate with a new identity. There is no in-place amendment, because an amended package is a package
   nobody approved.
2. **Approval binds to the digest of the canonical manifest**, which includes every artifact hash — not to
   a design, a style number or "the latest". Stale-ification is therefore mechanical: a candidate whose
   identity differs from the approved one is unapproved, and shipping it is `release_identity_changed`.
3. **A claim's scope is the intersection of the evidence behind it.** Scope narrows automatically and
   never widens by inference; widening is a new evidence record, and a production claim names its scope in
   the same sentence as the claim.
4. **Approval is a human act.** An agent may inspect, propose, commit and generate, and may assemble the
   evidence an approval needs; a detached approval record must name an authenticated human.

## The alternative each rejected

| Rejected | Why |
| --- | --- |
| a similarity threshold for stale-ification ("only geometry changes void an approval") | a threshold is a guess about which differences a factory cares about, made by the party that wants the approval to survive. One byte, one new candidate |
| amending a package in place with a change log | the amendment is then approved by nobody, and the log is the only evidence — which is exactly the shape the manifest exists to replace |
| approval bound to a design revision | a design revision does not determine the artifacts: the profile, the exporter config and the target axes do, and two packages from one revision differ |
| profile-wide or factory-wide "verified" | roadmap §14 already names the risk; per-claim evidence with a scope is the answer, and the scope tuple is what makes it checkable |
| a policy matrix cell that substitutes a plausible value | roadmap §8.3 forbids it. A `badge` means the artifact carries a visible mark and the manifest carries the gap; a `sidecar` means the gap travels beside the artifact. Neither invents a number |
| an agent-held approve authority "for automation" | the release is the act that puts a knife through fabric. The authority levels are the command layer's (`G0-CONTRACT.17`); this record fixes only that the top one cannot be a machine |

## The tuning of roadmap §8.2's example matrix

The roadmap publishes three example rows and invites G0/G4 to tune them. The chapter tunes one and records
the tuning in a table rather than silently: **notch geometry in a draft export is `sidecar`, not "default +
visible badge"**, because a default is a value substituted for an observation and §8.3 forbids it. The
draft still exports — the gap travels beside it, named — so nothing is blocked that need not be, and
nothing is invented that must not be. The other two example rows survive unchanged in substance; the
production column's "until resolved or human disposition" qualifier became §8's disposition rule, stated
once for every row instead of per cell.

## How to apply

- G4 implements the evidence store, the policy matrix and the dependency closure against this chapter, and
  tunes the cells against Profile Editor and pilot evidence — a tuning is a diff to the chapter, not a
  configuration file nobody reads.
- G7's supported-envelope statement is built from the scope tuples of the acceptances behind it, never from
  an impression of them.
- Any new artifact class is a new column with a recorded reason, and the census
  (`docs/tasks/artifacts/release_contract/run_release_contract_census.sh`) refuses a column the roadmap
  does not name until the record that adds it exists.

## What would reverse it

Two conditions, each needing a recorded decision: (1) a receiver or a factory workflow that cannot accept
an immutable package (a system that mutates files in place), which would make the candidate chain a
translation layer rather than the contract; (2) evidence from the G6 pilot that a `sidecar` disposition is
read as a defect report by cutting rooms, which would move those cells to `block` — stricter, never
looser, because loosening is how an unobserved value reaches fabric.

## D239 adopted — detached approval records

Director2026-10-26 selected “Separate immutable approval records (Recommended)”; coordinated adoption
is G1-SLICE.6.2.2.c.2.5.2. Original source/diagnosis is retained whole in
`docs/history/stitchcad-devnotes-part484.md`; full prior proposal/alternative/ruling in
`docs/history/stitchcad-devnotes-part485.md`.

The ninth manifest field is `approval_binding`, a fixed `{"kind":"detached"}` descriptor. It contains
neither actual human/scope/date values nor a changing record list or self digest. The candidate's
identity still hashes the complete immutable manifest, all artifact hashes included.
A separate immutable record carries exact `package_identity`, authenticated human `approver`, reviewed
`scope` and human-act `date`. Approval appends that record; it never edits the candidate. Receipt
transport is outside the content identity. Withdrawal is another record. Record fields cannot confer
human authority; the core must validate actual authenticated identity, grant, scope and subject.

Sealed validation results stay frozen; later acceptance records bind that same identity externally.
No state or approval silently carries to a changed candidate. The existing ladder, scope narrowing,
evidence and unknown-policy obligations remain. No actual package or G0 closure is approved here.

The historical approval_identity_probe.py reproduces why filling metadata after hashing changes
identity; its alternative fixtures are historical, not additional supported protocols. Its SHA256/JSON
choice is diagnostic only. Production hash/date/canonical encodings retain their implementation owners.
