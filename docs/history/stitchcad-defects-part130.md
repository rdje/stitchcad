# Sealed archive — D208 candidate metadata repair

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.a10` on `2026-10-06` (UTC).

- **Sealed identity:** 9 lines, 991 bytes, sha256:3baaee69557a96bb9ffca31af5b8080e45c3d469e2edbaca90a9d966af7e70a2
- **Coverage:** whole authored D208 report; absent at 117e1657e7f813663ba90df51b13a2b42a352ff3.
- **Retrieval:** content below the first rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later live record.

---

### Defect D208 — uncommitted archive candidate title mismatch

- Owner: G1-SLICE.5b.4c.h2.b.p.o.a10; priority P1, fixed before recording window10.
- Reproduce: window_contract.py on the initial candidate raises catalog title identity mismatch.
- Cause: the task-local capture copied the literal window9 human title while assigning window10 identities.
- Impact: source55 and prior9 identities remain exact, but the human catalog label is misleading and the focused archive gate refuses.
- Fix: validate a corrected candidate through the actual catalog_title function in an owned fixture before replacing uncommitted metadata; nine committed windows untouched.
- Verification: original candidate assertion red/corrected fixture and root positives; public prove-source55 and all594 logical bytes exact, rc=0, target/assert-plan-title-proof.log.
- Provenance: authored report absent at 117e1657e7f813663ba90df51b13a2b42a352ff3; title correction receipt retains both full-catalog SHA values.
