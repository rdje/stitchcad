# Sealed archive — complete exact built-in value lesson

Immutable historical segment, sealed by leaf `G1-SLICE.5e.2.3` on `2026-10-26` (UTC).

- **Sealed identity:** 14 lines, 1111 bytes, sha256:ce57219bc082cf232b92427bc4f0039dd50353501b4d06a97bccf6be24322f80
- **Coverage:** whole committed source interval from de0bc8915f577f8378e3de4e52c5fc59c0010026.
- Whole source interval: DEV_NOTES.md bytes 26307–27418, 1111 bytes, sha256:ce57219bc082cf232b92427bc4f0039dd50353501b4d06a97bccf6be24322f80
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

## _(2026-10-25 UTC)_ — exact abs/min/max/clamp numeric built-ins

0251 .5c.2.4 adds total exact abs and checked homogeneous variadic min/max plus inclusive clamp.
Abs preserves canonical zero/full signed magnitudes; symmetric scalar domains/nonnegative Count
make every output admitted. Min/max/clamp select existing exact operands, without rounding or
rescuing invalid completed values. Empty min/max calls retain formula_parse/builtin/count;
first kind mismatches retain builtin/zero-based argument/wanted/actual kind; inverted clamp bounds
retain formula_domain/shared kind/exact low/high. No call evaluation or binding is implied.

One affected native formula_numeric_contract target passes5/0, rc=0, including prior constructor,
binary/unary/comparison cases. New cases cover full magnitudes, all absolute kinds, single and
multiple operands, empty/mixed-kind calls, exact adjacent fractions and all clamp outcomes/bounds.
No broad/platform/mutation pass claimed. Whole0250 task/ledger retained322/223; book/live/frontier
align. Promotion declined: existing exact numeric contract/focused-check policy.
