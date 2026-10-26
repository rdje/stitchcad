# Sealed archive — whole0292 terminal ID exhaustion lesson

Immutable historical segment, sealed by leaf `G1-SLICE.4c.4.cleanup5` on `2026-10-26` (UTC).

- **Sealed identity:** 12 lines, 934 bytes, sha256:d23295b70817657e23cf2dc617ef18f7e28ea9f25de510bd24b33bd5dac2654c
- **Coverage:** whole committed source interval from 35e2650bb0bb20f59f2cf3d461416027682fa46a.
- Whole source interval: DEV_NOTES.md bytes 18400–19334, 934 bytes, sha256:d23295b70817657e23cf2dc617ef18f7e28ea9f25de510bd24b33bd5dac2654c
- **Retrieval:** bytes below the first rule and blank line; original terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

## _(2026-10-26 UTC)_ — terminal ID exhaustion and atomic ledger refusal

0292 .6.2.1 owns D225: from_start(MAX) yielded MAX then0; the public regression failed101.
Infallible allocation plus wrapping_add violated nonreuse. IdGenerator now returns Result;
the optional deterministic counter emits its final value once then permanent Exhausted. All actual
callers handle allocation; six ledger edits propagate original IdError/source before mutation.
Focused3/0 checks terminal seeds, twelve original allocator errors and six precondition priorities
against complete ledger equality. Existing id units10/0, affected all-target compile-only and
selected lib/changed-test strict Clippy/fmt/book pass0. Broader runtime/platform checks deferred.
405/406/407/408/264 retain whole history; G1still5/18, no persisted identity/clock change.
Promotion declined: repair to the existing nonreuse contract; identity decision documents the API.

