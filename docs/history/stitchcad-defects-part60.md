# Sealed archive — D142/D143 original CI reports

Immutable historical segment, sealed by leaf `G1-SLICE.5b.3c.2b.h1.v` on `2026-10-03` (UTC).

- **Sealed identity:** 16 lines, 1448 bytes, `sha256:abef4792ef4b496045e3ebde955e6a55dba1842770306143d7ab5c1decee2ad3`
- **Coverage:** D142/D143 original CI reports and implementation receipts; original payload unchanged from d5dd11f3a5483b4d28f40feb58c86a8b3c148724.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D142** — exact-head9fab7ec Rust CI job111242008842 fails strict Clippy1.99 at namespace.rs:221:
  declarations() has redundant bare must_use though returned Iterator already has that obligation.
  Actual job/log shows double_must_use; tests/WASM skipped; local1.95 strict gate was green.
  Owner G1-SLICE.5b.3c.2b.h1.r1, P0 immediately before any further product work; remove redundant
  annotation without lint waiver, preserve existing iterator behavior, full strict/public contracts.
  Implementation .h1.r1 verified on exact local1.99 (before101/after strict663/56→0); keep open
  until repaired-head runner .h1.v verifies it after D143, with no inferred remote success.

- **D143** — same runner log shows CARGO_HOME points to the runner-home .cargo store during Clippy;
  Rust workflow supplies no checkout-derived store/temp environment. Project dependency store
  defaults to user home, contrary to locality policy even when volumes happen to coincide.
  Owner G1-SLICE.5b.3c.2b.h1.r2, P1 immediately after D142, before required repaired CI .h1.v.
  Use documented toolchain integration and root-derived local stores/temp; require actual runner
  environment/path checks, full local gate/probes and observed exact-head CI before closure.
  Implementation .h1.r2 sets four root-derived stores before public Rustup installation, guards
  effective paths afterward and verifies19 controls/six compiled faults; keep open until .h1.v.
