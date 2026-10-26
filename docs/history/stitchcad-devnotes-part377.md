# Sealed archive — whole hypot numeric kernel lesson

Immutable historical segment, sealed by leaf `G1-SLICE.5d.3.2.1` on `2026-10-26` (UTC).

- **Sealed identity:** 16 lines, 1209 bytes, sha256:a8190ce4ae49a28d6a7cb0b7c66d97ccad85aebd856f756b754a8f729010702c
- **Coverage:** whole committed source interval from 99d643d0b0154a6d0327b48ac7d1b8d617fdaf81.
- Whole source interval: DEV_NOTES.md bytes 25420–26629, 1209 bytes, sha256:a8190ce4ae49a28d6a7cb0b7c66d97ccad85aebd856f756b754a8f729010702c
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

## _(2026-10-25 UTC)_ — correctly rounded hypot without a materialized area

0256 .5d.2.2 forms the exact signed-length squared sum in private bounded integer workspace and
reuses root bisection/midpoint rounding. No Area value or internal128-bit result limit is imposed.
Admitted length domains bound the root quotient below2^61; the function's rounded Length alone
has scalar validation. Existing signature metadata refuses actual non-Length inputs in operand
order, including zero values; no fabricated dimension/value is introduced.

One affected native root target passes4/0, rc=0, consuming all25 independent rows (15sqrt/10hypot).
Cases include signed/zero/Pythagorean/half outcomes, maximum-length squared sum above Area bound,
rounded scalar excess and full128-bit fraction components with513/512-bit raw workspace. Sqrt
cases and actual bad-kind checks remain green. Source-bearing root-call contributions stay .5e.3;
parent .5d.2 remains open for that integration. Broad/platform/mutation work stays .5d.4/pre-push.
Whole0255 scope/ledger retained329/228; book/live/frontier align. Promotion: numerical decision
records the implemented hypot value kernel alongside its existing algorithm proof.

