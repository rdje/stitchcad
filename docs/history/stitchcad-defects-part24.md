# Sealed archive — domain operation and truthful diagnostics

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3b.3b.1b` on `2026-10-02`.

- **Sealed identity:** 16 lines, 1518 bytes, `sha256:5be587210c53d53c7a889d9e071490c3444ae004fb9562baf2213dafa140084b`
- **Coverage:** original D90/D92 observations and ownership, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D90** — UnitError DomainExceeded omits the failing operation required by the numeric contract.
  - Reproduce: error.rs DomainExceeded contains kind/value/limit only; checked_add/sub reuse
    from_micrometres, and the rendered diagnostic cannot identify the operation that produced it.
  - Impact: failures lack required context for users, command diagnostics and agent-controlled repair.
  - Owner/schedule: G1-SLICE.5a.3b.3b.1b, immediately after D89; retain the actual constructor/caller
    operation through the public error, update all construction/match sites, test typed and display
    context plus actual guards. Reference scalar bounds follow .3b.2; no missing context is certified.

- **D92** — generic domain messages invent a unit-conversion cause and misdescribe lower-bound failures.
  - Reproduce: public DomainExceeded with kind positive range width/value0/limit1 renders “exceeds”
    and claims a value this large means a unit-conversion bug. Negative length failures are rendered
    as exceeding a positive limit too. Core range/topology bridges legitimately reuse this variant.
  - Impact: users and external agents receive an unsupported cause and an inaccurate domain relation.
  - Owner/schedule: G1-SLICE.5a.3b.3b.1b, fix with D90 now: render operation/kind/value/limit using
    neutral outside-domain wording; preserve numeric data and other variants. Test signed and zero
    domain payloads and a compiled actual rendering mutation; do not infer a root cause from magnitude.
