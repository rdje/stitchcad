# Sealed archive — extreme public rounding

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3a` on `2026-10-02`.

- **Sealed identity:** 10 lines, 941 bytes, `sha256:742973ccb7b50e4842b2cee7f3cd716f5fd3b135a0db20eb32e9903767257e77`
- **Coverage:** D78, closed and verified by STITCHCAD-G1-0041; diagnostic description unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D78** — extreme negative rounding magnitude can panic before typed i64 overflow refusal.
  - Diagnostic target: public sc_units::round::div_round_half_away_from_zero(i128::MIN, 1).
    Unsigned quotient 2^127 casts to i128::MIN and is then negated before checked i64 conversion.
    Observed public-interface probe at b681a49: catch_unwind → Err(Any) for MIN/1;
    MAX/1 and MIN/-1 return typed Overflow, i64 MIN/1 and ±1/2 controls succeed, probe rc=0.
    git log --follow round.rs identifies eb83f01 (G0-CONTRACT.18) as the sole introducing commit.
  - Impact: the documented total conversion primitive may abort a session on valid-width input,
    undermining exact literal conversion and production numeric refusal guarantees.
  - Owner/schedule: G1-SLICE.5a.3a, immediate prerequisite; reproduce, repair magnitude/sign checks,
    preserve both i64 endpoints/tie behavior, verify real guard mutations and strict native/WASM.
