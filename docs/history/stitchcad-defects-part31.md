# Sealed archive — D99 complete binding replay

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3b.3c.1` on `2026-10-02`.

- **Sealed identity:** 11 lines, 985 bytes, `sha256:fe05a4f02fc54364e34c8b33b577154b907b404bc9269864530181c90cd95dd6`
- **Coverage:** D99; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D99** — actual example replay rejects valid Area and Boolean bindings.
  - Reproduce: append `replay_area: area = 2 mm * 2 mm` (0.04 cm²) and
    `replay_boolean: boolean = 1 == 1` (true) to a copied examples §2 table; run the published
    census with FORMULA_BOOK pointing to that repository-local copy. Both rows refuse, rc=1,
    saying §2 disallows these kinds. Actual statement/stored already returns area:4000000 and
    boolean:1; formula §2 declares both bindable. Log: target/binding-replay-before.log.
  - Root/impact: L2 hardcodes four kinds and its numeric display has no Area/Boolean path;
    valid worked recipes cannot be checked, blocking complete signed-angle equality replay.
    Tiny-area controls also reproduce Decimal str emitting 1E-8 against fixed-decimal Value cells.
  - Owner/schedule: `G1-SLICE.5a.3b.3c.1`, P1 now; normative kind acceptance, cm²/true/false
    presentation, independent actual copied-book controls and compiled mutation reds.
