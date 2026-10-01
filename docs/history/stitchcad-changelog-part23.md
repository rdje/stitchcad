# Sealed archive — StitchCAD persistent identity contract

Immutable historical segment, sealed by leaf `SPINE.21a` on `2026-10-01`.

- **Sealed identity:** 48 lines, 4079 bytes, `sha256:e74210d6710b542f0503eee72b197fa85ca1322520e90ef12d3578704edee8b0`
- **Coverage:** STITCHCAD-G1-0005, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## STITCHCAD-G1-0005 - the persistent-identity contract: a reference is never rewritten, the journal folds (leaf `G1-SLICE.3b`)

G1's second new product code. `sc_core::ontology` now carries the persistent-identity contract the whole
design's reference integrity rests on, implemented against the design decision this slice recorded,
dependency-free and wasm-safe:

- `topology` — the `IdentityLedger`: an append-only journal of typed `TopologyEdit`s (declare, split, merge,
  reverse, delete, offset-fragment). A stored reference is **never rewritten by an edit** — the `(EdgeRef,
  Param)` a consumer holds stays byte-identical, and resolution is a pure fold of the journal, so a replay
  reproduces every fragment identity. Split resolves a reference into the fragment holding its parameter and
  offers BOTH sides at the split point for the consumer to state (`SplitSide`); merge recomputes by
  caller-declared arc length (exact `Rational`, never a rounding); reverse maps `t` to `1 − t` and tells
  directed consumers through `Direction`; delete and offset-fragmentation orphan references into visible
  `RepairTask`s naming the reference, the orphaning edit and the candidate resolutions. No silent
  reassignment: repair state is *derived* (`open_repairs`, `release_readiness`), never stored, so it cannot
  drift from the journal. A design with unresolved references stays inspectable — every query answers
  identically — but is `Blocked` from release, which is §1.1's rule.

Validation: `cargo test -p sc-core` → 62 unit + 8 contract-property + 9 identity-property, all green (the
properties prove split's trichotomy and both-sides split point, merge's arc-length recomputation against a
cross-multiplied oracle, reverse an involution, split↔merge round-trips to the identical reduced rational
with zero drift, a delete orphaning exactly the references resolving onto the victim and nothing else,
offset against a hundredths-grid oracle, the live-edge set equal to the journal replayed, and byte-identical
replay of one script); `make check` clean at clippy `-D warnings`; `make wasm` cross-builds `sc-core`;
`make gate` → `=== all doctrines green ===`. The design is recorded in
`decision_reference-resolution-journal-fold.md`.

Two containment obligations this append discharged atomically, both surfaced as defects:

- **CHANGELOG rolled over** — the prior append (`STITCHCAD-G1-0004`) crossed the live window's 32 768-byte
  health target (31 237 → 33 338) without sealing, which the protocol requires of the crossing commit. The
  rollover milestone is a convention the size checker only *warns* at (rc=0 past a health target; only the
  ceiling fails), so the miss passed every gate — defect **D54**. Sealed `STITCHCAD-G0-0012` +
  `STITCHCAD-G0-0011` into `stitchcad-changelog-part13.md` (75 lines / 7188 bytes, digest reproduced by
  `run_changelog_ledger_probes.sh`).
- **KNOWLEDGE_MAP hit its 8192-byte ceiling** — the pressure `STITCHCAD-G1-0004` flagged ("99% … for the
  next record/tree addition"): this slice's decision record tipped it to 8216. Tightened two subsystem
  entries (the containment doctrine's only local lever) back under → 8187. The structural cause — the
  generated decision-record and task-tree sections grow a line per slice while the sole trim lever is the
  bounded subsystem list — is defect **D53** for the containment owner. The same edit repaired a run-on
  bullet in `knowledge-map/subsystems.md` (two entries shared one line, invisible to the sync gate, which
  checks derivation not source form).

- lockstep: `G1-SLICE.md` (`.3b` done, a fresh evidence-backed acceptance subsection, frontier → `.3c`,
  verification + commit logs, changelog), `lib.rs` status + module table, `ontology/mod.rs` re-exports,
  `knowledge-map/subsystems.md` + regenerated `KNOWLEDGE_MAP.md`, `docs/TASK_TREE.md`, `MEMORY.md`,
  `LIVE_STATUS.md` (G1 → 4 of 18, census → 9 open), `PLANNING.md` (D53, D54), `DEV_NOTES.md` (the lesson +
  its own rollover), `CHANGELOG.md` (this entry + the part13 rollover).
