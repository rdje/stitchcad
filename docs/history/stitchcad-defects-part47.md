# Sealed archive — D125 false assertion diagnostic

Immutable historical segment, sealed by leaf `G1-SLICE.5e.3a` on `2026-10-02` (UTC).

- **Sealed identity:** 9 lines, 832 bytes, `sha256:048aec71230c691406ae8e88f808793587a31c4c0392711be194545111db3d7e`
- **Coverage:** D125; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D125** — reference runtime assertion adapter returns false without its required diagnostic.
  - Reproduce: statement('assert false_closure: eps_num = 1 cm == 2 cm', {}) returns
    ('assert', 'false_closure', False); the equal-value control returns True. Baseline exit0.
  - Root: statement computes the holds Boolean and returns it, while the book consumer only
    calls bad on False. Neither raises formula_assertion as contract5.2/9 requires.
  - Impact: general runtime reference cannot prove falsified-assertion diagnostic/payload behavior.
  - Owner: G1-SLICE.5e.3, high priority with D121 before product assertion/tolerance evidence;
    preserve expected valid tuples but raise named error on false, test values/class/name payload.
    Static type/name checking remains valid without computing any assertion verdict.
