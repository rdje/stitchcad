# Sealed archive — D172 through D174 original reports

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.n.f14` on `2026-10-04` (UTC).

- **Sealed identity:** 7 lines, 1351 bytes, `sha256:8b8da9efa9a95df59524250c6b6e879cb68e43338fb5a11887ea0a86d97c7a4a`
- **Coverage:** complete pre-repair D172/D173/D174 reports captured in target/d172-original-report.txt, target/d173-original-report.txt and target/d174-original-report.txt; not a committed Git interval.
- **Retrieval:** content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D172** — static coupled driver classifier accepts assertion-labelled non-assertion panics; actual mutation finally restores sources but omits artifact rebuild after a compiler refusal. static-entry-baseline.log both actual-flow reproductions, rc=0; simulated dispatch only/no source writes.
  Own G1-SLICE.5b.4c.h2.b.p.n.f14, P1 now before native proof: match current driver assertion sites, always rebuild restored artifact after Rust faults and verify actual early-refusal controls plus all native modes.

- **D173** — static_review_contract.load reports original infer line520 instead of605 (85-line shell header omitted), and in-memory faults impersonate the original file. static-loader-location-baseline.log actual code objects versus original AST and correct primary loader, rc=0; no source/native writes.
  Own G1-SLICE.5b.4c.h2.b.p.n.f14, P1 now: preserve original positions, mark variants virtual, extend actual loader/traceback/compiled-body controls before commit; semantic catalogs and grammar stay unchanged.

- **D174** — book reference-location count86 is stale; independent original AST gives94 primary/95 fixture definitions. Source history adds eight methods after58bbb6b; draft87 was also unsupported. Own G1-SLICE.5b.4c.h2.b.p.n.f14, P1 now: remove fixed totals, document the dynamically verified catalog before commit.
