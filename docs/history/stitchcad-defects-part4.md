# Sealed archive — StitchCAD defect census, closed D58

Immutable historical segment, sealed by leaf `G1-SLICE.3c.3a` on `2026-10-01`.

- **Sealed identity:** 16 lines, 1458 bytes, `sha256:4406d117e8f6b812a0a4f04b602d0d82b3a560b4ec5a0291bafe9b4fdd1b571b`
- **Coverage:** `D58`; discovery record and closure evidence.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D58** — `G4-PROFILES.7`'s Goal forbids a conservative default for an unknown, but its
  Acceptance still asks for "notch geometry → default + sidecar". The normative release §8 tuned
  that draft case to `sidecar` and explicitly forbids substituting a plausible value. Found while
  defining symbolic notch bindings, `2026-10-01`.
  - Reproduce: `sed -n '121,129p' docs/tasks/G4-PROFILES.md` versus
    `rg -n 'notch geometry|No conservative default' docs/book/src/spec/release-contract.md`:
    the same leaf's Goal and Acceptance disagree, and the latter contradicts the canonical matrix.
  - Impact: following the pending leaf verbatim would invent physical notch geometry for an unknown.
  - Owner/schedule: **`G1-SLICE.3c.3a`, now**, aligns the dependent acceptance with release §8 while
    landing explicitly unresolved bindings. This changes the test contract, not the release policy;
    `G4-PROFILES.7` still owns its implementation and complete policy-matrix tests.
  - **Closed:** `G1-SLICE.3c.3a` aligns the acceptance with the normative matrix: badge preview,
    sidecar draft, block production; no defaults. The notch object carries symbolic declarations
    and always reports `DeferredToG4`. `run_release_contract_census.sh` → `8 matrix rows /
    0 failure(s)`, `rc=0`; `cargo test -p sc-core --test notch_contract` → `7 passed`, `rc=0`.
    G4 retains ownership of actual binding validation and artifact-policy enforcement.
