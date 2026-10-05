# Sealed archive — whole new-over7e197a7e933f8effefb333f951fcf356d3c51a5e report/closure; docs/tasks/PLANNING.md authored bytes 79843–80404; rejected original candidate retained in task sibling

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.f.n10.2` on `2026-10-05` (UTC).

- **Sealed identity:** 11 lines, 1238 bytes, `sha256:1c01209fbbb6d3ed7e69a333ec1157709a440c2e8184f63cda9d7110d5c7dffc`
- **Coverage:** whole new-over7e197a7e933f8effefb333f951fcf356d3c51a5e report/closure; docs/tasks/PLANNING.md authored bytes 79843–80404; rejected original candidate retained in task sibling.
- **Retrieval:** content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D190** (P1, active fix G1-SLICE.5b.4c.h2.b.p.o.f.n10.2): new D189 sealing helper retires the live report after equality checks but before archive record_stats admission. Reproduce target/name-read-plan-retention-proof.log: logical maxline bound exceeded; rejected uncommitted part111 report line595B exceeds400. Impact: public archive inventory/retrieval
refuses. Retain complete rejected candidate bytes, supersede with valid records; preflight bounds/full inventory/public reads before live retirement. No committed immutable bytes or ceiling may change.

D190 closes at .n10.2: three uncommitted, unadmitted candidates were preserved as
complete original bytes in the existing retention sibling before physical retirement.
Fresh221/164/112/113 supersede them; no committed record or logical address changed.
Actual record_stats and exactly-one-terminal-newline preflights precede every fresh
write; full inventory plus all four public reads precede live report retirement.
Prior496/all40 immutable identities and all whole source intervals remain required.
The existing400-byte logical bound, sealed ledger terminator and reader stay unchanged.
All500 retrieval and ledger refusal controls are the final closure gate in this leaf.
