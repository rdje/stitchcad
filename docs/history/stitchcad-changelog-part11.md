# Sealed archive — StitchCAD changelog, the delegation-and-uncertainty slice

Immutable historical segment, sealed out of `CHANGELOG.md` under the rollover rule recorded in
`.doctrine/live_document_size/surfaces.tsv` (row `changelog`, lifecycle `rolling_ledger`): when the
live window passes its health target, the oldest entries are sealed here and a pointer is left behind.

- **Sealed identity:** 46 lines, 4507 bytes, `sha256:de34382e110556442bdd33320f042f6298c7836ac2eb7323cdd66face52a297d`
- **Sealed by:** leaf `G0-CONTRACT.17` (the append that crossed the rollover milestone performed the
  rollover, as the doctrine requires).
- **Coverage:** `STITCHCAD-G0-0014c`, the oldest live entry, exactly as it stood in `CHANGELOG.md`. No
  slice range is declared, for the reason `stitchcad-changelog-part9.md` records: the ranges in the earlier
  descriptors have no producer and disagree with a derivation from git (defect D51).
- **Retrieval:** this file, or `git log --follow CHANGELOG.md`.
- **Write policy:** none — sealed segments are immutable.

---

## STITCHCAD-G0-0014c - a delegated decision is bounded, and what the project does not know is derived (leaves `G0-CONTRACT.14c`, `G0-CONTRACT.19`)

The director's instruction was to decide and act on the three findings. The first was that the engineer proposed
the roadmap amendment and then applied it; the second was that three seats are empty. Both are now governed
rather than reported.

- **governance §6.1, decisions made under delegation** — five rules for the case this project actually runs in:
  record the author and the applier and say so when they are the same party; the author of a decision may never
  approve the evidence that decision requires (where no independent reviewer exists the claim stays
  **unapproved**, and is recorded as unapproved); the consequences become instruments others can run; the record
  states what would reverse it and who may; and a delegation to decide is not one to upgrade evidence. A
  disclosure decays with the conversation it was made in - a rule is read by whoever acts next
- **the rule bites in three verifiable places**: the roadmap's Appendix A v0.3 entry now states that author and
  applier were the same party (`grep -c 'same one' ROADMAP.md` -> `1`); `G3-GRADING.14`'s acceptance withholds
  the exit review from the criterion's author; and `decision_self-application-under-delegation.md` carries the
  reasoning with the v0.3 instance as its founding measurement. The bound is on CERTIFICATION, not on action -
  waiting for an approver would stall the project, and letting the author certify would make every gate a
  formality
- **the revision-aware baseline caught this slice's own roadmap edit**, which is the mechanism working rather
  than a nuisance: four lines of disclosure grew v0.3 from 947 to 951 lines and `check_live_doc_size.sh`
  refused it (`transition debt WIDENED on lines (951 > baseline 947)`). Handled by the rule - re-based to v0.3's
  final state with the authority cited in the row's notes - and the limit it exposed is recorded rather than
  hidden: `at=<revision>` binds a baseline to a revision MARKER, not to a commit, so the real control is that
  every re-base is visible in a diff beside the content that moved it
- **G0-CONTRACT.19, the uncertainty census**: `bash docs/tasks/artifacts/uncertainty/run_uncertainty_census.sh`
  -> `uncertainty census: 88 markers / 10 files / 0 unowned / 0 failure(s)`, enumerating every marker in the
  book's own vocabulary (`assumed`, `unknown`, `unverified-with-owner`, `read-in-repo`,
  `cited-from-roadmap`, `read-external`, `known`, `derived`, `(proposed)`, `vacant`) per file and per resolving
  authority, and refusing a blocking marker whose verification-status section names no resolver. So the day a
  name arrives, the work it unblocks is one command away, and an edit that quietly drops an `assumed` from a
  fixture constant changes a count somebody reads
- **six probe arms** -> `probes: 6 pass / 0 fail`: an owned-claim control, an unowned synthetic chapter refused
  by name, a chapter added AFTER the census was written refused (the rule is about the population, not today's
  files), a definition left alone (U1 reads verification-status sections, because deciding that any mention is a
  claim would be a classifier guessing at meaning), and an absent book refusing with exit=2
- **building it found two defects in existing instruments, both fixed here**: the glossary census's `resolve()`
  deleted one `/x/../` per gsub pass and so reported `../../governance.md` - a file that exists - as a dead
  reference (it now normalises segment by segment -> `277 terms / 8 parts / 146 tokens / 0 failure(s)`); and
  this census's first authority list counted a bare gate id as an owner, so "frozen as a golden at G2" satisfied
  it and the UNOWNED arm passed for the wrong reason until the list was narrowed to parties that can RESOLVE a
  claim. A census whose own probe arm passes for the wrong reason is the vacuous-green class, fifth instance
- the token census fired a sixth time at authoring time: `vacant` is a status an instrument greps for, so it
  became a glossary term (`vacant seat`, ⚠, owning `vacant`) with the A-Z index re-derived
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `15 suite(s) green`; `make book` ->
  exit=0; containment `OK - 17 surfaces, 15 routes, 89 files measured`; matrix 105 rows, standards 6
  registered, fixture 20 rows / 4 checks / 5 pieces, coverage 13 trees / 3 siblings - all 0 failure(s)
