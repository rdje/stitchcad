# Sealed archive — whole0312 label and base intent

Immutable historical segment, sealed by leaf `G1-SLICE.4c.4.3.1` on `2026-10-26` (UTC).

- **Sealed identity:** 19 lines, 1763 bytes, sha256:f3e9e4824f73e8931215dfb6627962b87e0423938aa75cee4c276e8b30f84ece
- **Coverage:** whole committed source interval from 62aea0213aaa682ea350b732aaffcf8a9dbc0cc8.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 30051–31814, 1763 bytes, sha256:f3e9e4824f73e8931215dfb6627962b87e0423938aa75cee4c276e8b30f84ece
- **Retrieval:** bytes below the first rule and blank line; original terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.4c.4.2`
  Status: `done`
  Goal:0312 typed transformation declarations retaining authored state/evidence, label mappings and base substitution.
  Own src/size_set_resolution/transformation.rs and transformation/ helpers, lib/module exports,
  garment/MTM composite tests and tests/support/size_set_transformations.rs; transformation/main book,
  decision and live/task pointers.
  Every declaration pins actual SizeSet inputs and explicit domain; Known evidence is nonempty/distinct,
  Assumed/Preference retain source refs, Unknown/Derived have no authored payload. No state/default coercion.
  Label mappings name existing members, preserve vector order and exact target labels, and reject effective
  label collisions; explicit base must be an actual member. Source inputs are never edited or sorted.
  Own452 whole0311 task,453 whole oldest0288 lesson,284 ledger from214c80a; prove exact Git/hash/public
  before removal. Also correct transformation page's actual §9 anchor. Focused state/domain checks/lint/fmt/book;
  execution, profile constraints, global provenance/registry validation remain G3/G4/integration owners.
  - [x] ROOT CAUSE (WHY + WHERE): SizeSet §9/roadmap8 require typed state/evidence; canonical state API review rc=0.
  - [x] ADDRESSED: label/base/source/evidence state contract; garment13+MTM8 focused tests rc=0.
  - [x] NO REGRESSION: original composite/path cases included; selected strict clippy/fmt/book rc=0.
  Verification: native21/0; strict selected lint/fmt/book rc=0;452/453/284 whole Git/hash/public byte equality rc=0.
  LIVE_STATUS reviewed: G1 In Progress5/18 unchanged; profile execution/approval remain later gates.
  Commit: STITCHCAD-G1-0312. Next .4c.4.3; broad checks remain integration/pre-push.
