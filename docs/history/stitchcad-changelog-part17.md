# Sealed archive — StitchCAD changelog, command-layer contract

Immutable historical segment, sealed by leaf `G1-SLICE.3c.3b` on `2026-10-01`.

- **Sealed identity:** 38 lines, 3626 bytes, `sha256:819f240ac3da2536498586ff883c48caba4056c1974db8cc7b292761f0f9a492`
- **Coverage:** `STITCHCAD-G0-0017`, oldest live entry copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## STITCHCAD-G0-0017 - the command layer is a contract, and the roadmap's own lists prove it (leaf `G0-CONTRACT.17`)

§4.4 makes undo/redo granularity a G0 deliverable rather than an implementation detail, and twenty glossary
entries were parked against this leaf under a header note saying "until that chapter lands, the roadmap clause
is cited". The chapter lands, and the two lists the roadmap carries in prose are parsed out of it.

- **the contract** - `docs/book/src/spec/command-layer.md` (264 lines / 18 959 B): seventeen commands in five
  classes, where a class fixes a command's authority, its reversibility and its undo granularity, so an adapter
  cannot re-classify one for convenience; the seven fields of a command's shape; **undo at the atomic group,
  restoring semantics - recipe, entity identities, revision - rather than contours**, because two designs with
  the same contours and different recipes are different designs; evaluations and artifacts discarded rather
  than undone; the history not canonical content, so a reopened project starts fresh at its saved revision.
- **preview/commit, preconditions and idempotency** - a preview needs only `inspect` and a commit re-checks
  the precondition the preview passed, because three front-ends and an agent edit one design; a mutation
  carries the revision it was authored against and an idempotency key, so a stale revision is refused naming
  both and a transport retry is a reported replay rather than a second edit; the audit trail is append-only
  and is not the undo history.
- **authority as a core-enforced permission on a class** - the five levels of §7.8, with `approve` unholdable
  by an agent, which is the release chapter's `release_approver_not_human` seen from the other side. The
  parity table's eight columns, closed cell vocabulary and generation rule are normative while its rows stay
  empty, because rows at G0 would be claims about unwritten adapters - the same reason the canvas spike's
  results file is empty. The undo depth is the one number deliberately not written: declared to exist and to
  be bounded, with its value belonging to the resource bounds G1 measures.
- **the instrument** - `run_command_layer_census.sh` parses roadmap §4.4's backticked command list and §7.8's
  slash-separated levels and compares both with the chapter, in both directions for the levels:
  `17 commands / 5 classes / 5 levels / 0 failure(s)`. Its first run parsed zero levels - the list wraps
  mid-item - and reported five invented ones, so the fix is whitespace normalisation and the lesson is that a
  reader must fail closed when it reads nothing. `run_command_layer_probes.sh` -> `14 pass / 0 fail`, with
  LEVEL-EXTRA and LEVEL-MISSING as separate arms and HUMAN-ONLY refusing a softened rule.
- **decisions** - `docs/decisions/decision_command-layer-contract-and-undo-granularity.md` records ten
  rejected alternatives, including per-command undo, contour-level undo, undoing a generated instance,
  persisting the stack, trusting a preview, retry-by-repeat, per-tool permission lists and a hand-written
  parity table.
- **D49's third trigger discharged** - the tree had reached 94 % of its byte ceiling, so the decisions of
  leaves `.1`-`.4c` were sealed into `g0-contract-decisions-part1` (112 lines / 10 079 bytes, digest
  reproduced, 33 segment verdicts) and `.16`'s checklist moved to the evidence sibling.
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `21 suite(s) green`; `make book` ->
  exit=0; all nine other censuses green; containment `OK - 17 surfaces, 15 routes, 113 files measured`
