# Release and approval

> **Status:** normative specification, gate **G0** (roadmap §9, and §8.2's artifact policy matrix, which
> the [glossary](glossary.md) routes here). Implemented from gate G4 — the evidence store, the policy
> matrix and the Profile Editor — and consumed by G7's scoped production declaration.
> [Governance §4](../governance.md) owns who signs a golden file; this chapter owns what a package is
> and what an approval binds to.

A release package is the only thing a factory cuts from, and the only thing an approval can mean. So
this chapter fixes three things that are easy to blur: what the package **contains** (a manifest whose
every field has a source), what an approval **binds to** (the package's identity, and nothing else), and
what a claim **is evidence for** (a scope, never a certificate).

## 1. What a release package is

A package is an **immutable set**: the artifacts, the manifest that describes them, and the evidence
records the manifest cites. Three properties are normative:

- **A package is a candidate until a human approves it.** Generating artifacts produces a candidate with
  an identity; approval is a separate act that binds to that identity (§3).
- **A package is never edited.** A change to any input or artifact produces a *new* candidate with a new
  identity, and the prior approval stale-ifies (§3). There is no in-place amendment, because an amended
  package is a package nobody approved.
- **A package is complete or invalid.** Completeness is checked against the declared construction (§4),
  not against the geometry: a valid file missing a piece is an invalid package.

## 2. The manifest, field by field

Every field carries its source, because a manifest field nobody can trace is an assertion. The
**roadmap §9 field** column quotes the clause this table realises, and the census in §10 compares the two
lists.

| Field | Roadmap §9 field | Carries | Source |
| --- | --- | --- | --- |
| `design_revision` | design revision | the design's entity id and revision counter | ontology §3.1 |
| `inputs` | inputs (measurements, materials) | the measurement table, the ease set and the materials, each at a revision | ontology §2.1, §2.2, §6 |
| `resolved_profile_versions` | resolved profile versions | every Factory Profile consulted, by id and version, plus the resolved value of each parameter used | roadmap §8, governance §3 |
| `sizes_and_pieces` | concrete sizes/pieces | the size set at a revision, and per size the piece list with multiplicity, pairing, fold and material | size sets §2, ontology §4.1 |
| `engine_and_exporter_config` | engine + exporter config | the generator's version, the target token and every axis value, the tessellation bound, the canonicalizer's version | [interchange §1, §10](interchange-dialects.md) |
| `artifact_hashes` | artifact hashes | one digest per artifact file, with its byte length | this chapter §3 |
| `validation_results` | validation results | every check that ran, its verdict, its tolerance class and the acceptance state it earned | §5, units §3 |
| `unresolved_assumptions` | unresolved assumptions + dispositions | every `assumed` and `unknown` value the package depends on, with its state, its owner and its disposition | ontology §5, §8 below |
| `approver_scope_date` | approver + scope + date | the human identity that approved, the scope the approval covers, and the date | §3, §6, §7 |

Two rules keep the manifest honest rather than merely long:

- **No field is optional and no field is prose.** Each is a stable token with a typed value; a field
  whose value is not available is present and unresolved, never absent, because an absent field and an
  empty one are different claims and only one of them is checkable.
- **Nothing derived from a wall clock enters except the approval date**, which is a human act and is
  recorded as one (ontology §7's canonicalization rule).

## 3. Identity, approval and stale-ification

**A package's identity is the digest of its canonical manifest**, which includes every artifact hash, so
the identity changes if any byte of any artifact changes. Approval binds to that identity and to nothing
else — not to a design, not to a style number, not to "the latest".

- **Any input or artifact change creates a new candidate** and stale-ifies the prior approval. A stale
  approval is not deleted: it remains as a record of what was approved, when, by whom and over what
  scope, and it is *void* for the new candidate.
- **Approval is never inherited.** A candidate that differs by one byte from an approved package is
  unapproved, and shipping it is `release_identity_changed`. The rule is deliberately blunt: the
  alternative is a similarity threshold, and a threshold is a guess about which differences a factory
  cares about.
- **The chain is visible.** A candidate names the approvals it stale-ified, so a reviewer can see what
  changed between the package a factory accepted and the one being offered.

## 4. Package completeness

Completeness is checked against the **declared construction**, not against the artifacts' internal
validity: a geometrically valid file missing a piece is an invalid package.

| Checked against | The check |
| --- | --- |
| the piece list | every piece the design declares is present in every artifact that claims to carry it, at its declared multiplicity and pairing |
| the size set | every size the package claims is instantiated, and no size is present that the package does not claim |
| cut instructions | fold edges, mirrored pairs, face/wrong side and label data are complete enough to cut without consulting anything else (ontology §4.1) |
| material assignments | every piece names a material, or the package records the assignment as unresolved and the policy matrix (§8) decides |
| companion files | every artifact the target's convention requires — a `.rul` where the grading carriage is base plus rules, a provenance sidecar where a cell says so, a tech pack where the order asks for one |
| the sewing graph | every piece is accounted for by a span or by a declared non-sewn attachment ([reference skirt §8](reference-skirt.md)) — the account the fixture's own instrument derives |
| notions | every closure the design declares appears in the notions list, with the counts the geometry implies |

The reference fixture states its own completeness obligation in exactly this shape — 5 pieces, 2 notched
side-seam pairs, 1 zipper, 1 hook and bar, and a notions list that matches ([reference skirt
§12](reference-skirt.md)) — which is what makes it the right first package to check.

## 5. The graduated acceptance states

Six states, in this order, each earned by evidence and none implied by another. The order is the
roadmap's; the evidence column is this chapter's.

| State | What it claims | Evidence it requires | Granted by |
| --- | --- | --- | --- |
| `generated` | the artifacts exist and are byte-stable | the generator's deterministic replay: two runs, one byte string | the machine |
| `internally checked` | the model's own invariants hold on it | the check suite's verdicts at their named tolerance classes (units §3), including completeness (§4) | the machine |
| `independently inspected` | somebody who did not build it agrees it is right | a review record naming the reviewer, the artifacts and the findings, with the reviewer meeting [governance §2](../governance.md)'s independence criterion | a human reviewer |
| `target-imported` | the receiver read it back | an import record: the product, its version, the import settings, what was preserved / approximated / omitted / unsupported | a human or a partner run |
| `physically evaluated` | the artifact survives contact with fabric | a physical record: printed, plotted or cut, measured against the canonical geometry at T5 | a human, with the factory or a partner |
| `human-approved` | a person accepts responsibility for this scope | an approval record bound to the package identity, naming the approver, the scope and the date | a human, never an agent (§7) |

Three rules make the ladder a ladder:

- **A state may not be claimed without every state below it.** `release_state_unearned` names the missing
  rung, because a claim that skips one is a claim about evidence nobody has.
- **A state belongs to a package identity.** A new candidate starts at `generated`; states do not carry
  over, for the same reason approvals do not (§3).
- **A state is not a property of a design, a factory or a product.** It is a property of one package, and
  the only generalization available is the scope rule in §6.

## 6. Scope, and how it narrows

**One factory's acceptance is evidence for that envelope and never a general certificate.** Every
acceptance record and every approval names its scope, and a scope is a tuple, not a sentence:

| Scope axis | Example |
| --- | --- |
| garment | the reference skirt; a darted bodice with a set-in sleeve |
| material | a named woven, at a declared weight and shrinkage state |
| target system | a named receiver product and version, with its import settings |
| size range | the sizes actually instantiated and checked |
| artifact set | the target tokens and grading carriages actually exported |

- **A claim's scope is the intersection of the evidence behind it.** Two acceptances over overlapping
  scopes support a claim over the overlap and nothing wider; scope narrows automatically and never
  widens by inference.
- **Widening is a new evidence record**, not an edit: a production claim over a garment, material and
  target system nobody tested is `release_scope_widened`, and G7's supported-envelope statement is built
  from these scopes rather than from an impression of them.
- **A production claim names its scope in the same sentence.** "Validated for woven womenswear skirts on
  `<receiver>` `<version>`" is a claim; "validated" is not.

## 7. Human approval, and what an agent may do

**Approval is a human act.** An agent — including this repository's own AI co-users — may inspect,
propose, commit and generate, and it may assemble the evidence an approval needs; it may not approve.

- The approver field names a human identity. A package whose approver is an agent, a service account or
  a role nobody holds is invalid: `release_approver_not_human`.
- Approval is **deliberate and scoped**: it names the scope it covers (§6), so an approval is never
  broader than the sentence that records it.
- The authority levels themselves — inspect / propose / commit / generate / approve — are command-layer
  concepts specified by the [command layer §7](command-layer.md) and bounded by
  [governance §6](../governance.md); this chapter
  fixes only the release consequence: the top level cannot be held by a machine.
- Where a seat is vacant, an approval does not exist and is not inferred. A package needing the domain
  expert's signature stays unapproved, which is the state [governance §8.1](../governance.md) leaves the
  first G2 golden in.

## 8. The artifact policy matrix

The matrix answers one question: **what does an unresolved value block?** It is consulted for the
`unknown` state only — the other four states have fixed rules, stated below — and its cells are project
decisions, tuned with evidence at G4 as the roadmap directs.

| Unknown affects | PDF preview | DXF/PLT draft | Production release |
| --- | --- | --- | --- |
| a unit or a conversion | `block` | `block` | `block` |
| allowance ownership or width | `badge` | `sidecar` | `block` |
| notch geometry or type | `badge` | `sidecar` | `block` |
| a body measurement or ease | `badge` | `sidecar` | `block` |
| a material property (shrinkage, nap) | `badge` | `badge` | `block` |
| a grade rule or a size designation | `badge` | `sidecar` | `block` |
| a cosmetic label | `badge` | `badge` | `block` |
| a closure or notion count | `badge` | `sidecar` | `block` |

The roadmap publishes this matrix as an *example* of three rows and invites G0 to tune it. The tuning is
recorded rather than silent, so a reader can see which of its rows survived and what they became:

| Roadmap §8.2 row | Realised here as | What the tuning changed |
| --- | --- | --- |
| Units | a unit or a conversion | nothing — `block` in every column, as the example has it |
| Notch geometry | notch geometry or type | the draft column became `sidecar` instead of "default + visible badge", because §8.3 forbids substituting a plausible value for an observation |
| Cosmetic label | a cosmetic label | production became `block` without the "until resolved or human disposition" qualifier, which §8's disposition rule now states once for every row |

The four dispositions are a closed vocabulary:

| Disposition | Meaning |
| --- | --- |
| `permit` | exported with no qualifier — the value is resolved |
| `badge` | exported with a visible badge on the artifact and the assumption carried in the manifest |
| `sidecar` | exported with a provenance sidecar naming the unresolved value and what it would change |
| `block` | not exported until the value is resolved or a recorded human disposition exists |

The rules around the matrix are what make it a contract rather than a table:

- **The other four states are fixed.** `known` is `permit`; `preference` is `permit` with its provenance
  badged; `assumed` is `badge` with the assumption in the manifest; `derived` takes the **strictest**
  disposition of its inputs, so a derived value can never be more exportable than what it was computed
  from.
- **The dependency closure is computed per requested artifact.** An unknown that cannot affect the
  artifact being exported does not block it, and the closure that decided this is recorded with the
  package — an irrelevant unknown blocking an unrelated export is a defect, and so is a relevant one
  not blocking.
- **A disposition is a recorded human decision, not a deletion.** It turns a `block` into a `badge` for a
  named scope, names the human who made it, and leaves the `unknown` in the model. A package whose
  blocked value has no disposition is refused: `release_disposition_missing`.
- **No conservative default exists anywhere in this path** (roadmap §8.3). A cell saying `badge` means
  the artifact carries a visible mark and the manifest carries the gap; it never means a plausible value
  was substituted.

## 9. Diagnostics

| Token | Raised when | Required arguments |
| --- | --- | --- |
| `release_manifest_field_missing` | a manifest field is absent rather than present and unresolved | the field, the package identity |
| `release_identity_changed` | an approval is offered for a candidate whose identity is not the approved one | both identities, the fields that differ |
| `release_incomplete` | a completeness check of §4 fails | the check, what is missing, what declared it |
| `release_state_unearned` | an acceptance state is claimed without a rung below it | the state claimed, the rung missing, the evidence owed |
| `release_scope_widened` | a claim's scope exceeds the intersection of its evidence | the claim, the evidence scopes, the axis that widened |
| `release_approver_not_human` | the approver field is not a human identity | the field's value, the authority level it would need |
| `release_unknown_blocking` | an `unknown` in the dependency closure maps to `block` | the value, its owner, the artifact, the cell |
| `release_disposition_missing` | a blocked value has no recorded human disposition | the value, the artifact, the seat that owes it |

A diagnostic is structured, names what was asked for and what to do instead, and is never accompanied by
a partial package ([envelope §10](feature-matrix.md)).

## 10. Verification status of the claims in this chapter

- **The manifest fields, the six acceptance states and the three artifact classes are the roadmap's**,
  quoted in the tables above and compared against §9 and §8.2 by a tracked producer rather than by
  reading: `docs/tasks/artifacts/release_contract/run_release_contract_census.sh` parses both roadmap
  clauses and refuses a field, a state or an artifact class the chapter does not carry, in either
  direction. `run_release_contract_probes.sh` is its ground truth.
- **The policy matrix's cells, the four dispositions, the diagnostics, the scope axes and every rule in
  §3–§8 are project decisions**, reasoned with the alternative each rejected in
  `docs/decisions/decision_release-package-identity-and-scope.md`. The roadmap publishes the matrix as an
  *example* to be tuned at G0/G4;
  this chapter tunes it, and the tuning is revisited at G4 against evidence from the Profile Editor and
  the factory pilot. Where a cell here is stricter than the roadmap's example — notch geometry in a draft
  export is `sidecar` rather than a badged default — the reason is §8.3: a default substituted for an
  observation is forbidden, so the draft carries the gap beside the artifact instead of inside it.
- **Nothing in this chapter is `assumed` or `unknown` on a domain question.** The matrix's cells are
  engineering dispositions over states the ontology already declares; the domain questions that would
  change them (what a factory actually tolerates) belong to the vacant domain expert seat
  ([governance §8.1](../governance.md)) and arrive at G4 as evidence, not as a guess recorded here.
- **The independence criterion for `independently inspected`, the two-step approval for anything that
  alters exported bytes, and the acting-authority limits are governance's**, cited where they bind. This
  chapter owns the package and the ladder; governance owns who may stand on it.

## 11. What must be true in tests

- A candidate's identity changes when any artifact byte, any input revision or any manifest field
  changes, and does not change when a non-canonical detail (a comment, a key order) does.
- An approval offered for a stale identity is refused naming both identities and the fields that differ;
  a stale approval is still readable as a record.
- Every §4 completeness check fails for the right reason on a deliberately broken package: a missing
  piece, a size nobody claimed, an absent `.rul` where the carriage needs one, a piece with no span and
  no declared attachment, a closure with no notion.
- A state claimed without the rung below it is refused, and the six states are earned in order on the
  reference fixture as it moves through G2's harness.
- A scope widens only by a new evidence record; two overlapping acceptances produce a claim over the
  intersection and refuse the union.
- A package whose approver field is not a human identity is invalid, whatever else it carries.
- The policy matrix is consulted only for `unknown`, the dependency closure is recorded with the
  package, and an irrelevant unknown does not block an unrelated export while a relevant one does.
- A `derived` value takes the strictest disposition of its inputs, so no derivation launders a `block`
  into a `badge`.
