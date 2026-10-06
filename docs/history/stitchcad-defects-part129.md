# Sealed archive — whole authored D207 report and verified closure

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.a07` on `2026-10-06` (UTC).

- **Sealed identity:** 7 lines, 639 bytes, sha256:d041ce5b587d4103084a4ced277cfd8b1b9f91e39147e8e31aa59594e6e5a4de
- **Coverage:** whole newly authored D207 report, absent from source HEAD 4d3ec9f7fd6636d3edd968d2245074e6274e8592.
- **Retrieval:** content below the first rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later live record.

---

- **D207** — P1; owner `G1-SLICE.5b.4c.h2.b.p.o.a07`; fixed before0206 commit.
  New uncommitted full-output restoration plan could refuse a late log directory and leave
  safe reference bytes mutated. Actual guarded mirror red rc2; project source remained exact.
  Fix: fresh source-only mutable plan before both restores; unsafe logs still refuse rc2.
  Watched actual restoration assertion red/green passes3.9/3.14; real nine mutations restore
  exactly on both. Logs: target/input-plan-{control,real}{,39}.log and restore-{red,green}.log.
  Whole authored report absent from source HEAD4d3ec9f; no grammar/Rust change; closed in129.
