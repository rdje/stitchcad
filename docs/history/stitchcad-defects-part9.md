# Sealed archive — StitchCAD defect census, closed D62/D63

Immutable historical segment, sealed by leaf `G1-SLICE.3c.4d.2` on `2026-10-01`.

- **Sealed identity:** 14 lines, 1370 bytes, `sha256:5c7cd18f1313168096842e10a2539b3960cb921ffc89b060dc0e2cde76566d8c`
- **Coverage:** `D62`, `D63`; crate-scoped counts and field-targeted negative evidence.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D62** — LIVE_STATUS says sc-units has 30 tests, copying the original whole-workspace total.
  - Reproduce: `cargo test -p sc-units` reports 5 unit + 21 property + 1 doc, total 27; `git show
    eb83f01:crates/sc-core/src/lib.rs` declares the three smoke tests that complete the original 30.
  - Impact: the live status misattributes verification coverage; no test or unit behavior was lost.
  - Owner/schedule: `G1-SLICE.3c.4d.2`, immediate signoff correction and crate-scoped re-derivation.
  - **Closed:** LIVE_STATUS reports 26 regular + 1 doc test for sc-units; unchanged original
    workspace history remains correctly scoped. Full Rust execution and crate-only rerun agree.
- **D63** — feature-matrix BAD-GATE matches old tuck/pleat explanation, so it mutates no current row.
  - Reproduce: `make probes` reports feature suite `11 pass / 1 fail`; BAD-GATE census exits 0.
  - Impact: the negative proof depends on unrelated prose and blocks full milestone verification.
  - Owner/schedule: `G1-SLICE.3c.4d.2`, immediate row/cell-targeted mutation plus mutation proof;
    retain the actual census predicate and require the named malformed-gate refusal.
  - **Closed:** `run_feature_matrix_probes.sh` → `14 pass / 0 fail`, `rc=0`; independent no-op
    writer mutation → suite red `rc=1`, naming setup refusal, before the restored full 22 suites pass.
