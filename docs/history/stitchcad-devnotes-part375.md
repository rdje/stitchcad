# Sealed archive — whole sqrt numeric kernel lesson

Immutable historical segment, sealed by leaf `G1-SLICE.5d.1.2.2.2.c` on `2026-10-26` (UTC).

- **Sealed identity:** 15 lines, 1120 bytes, sha256:0899bb9e4b4a2fea37bb412fb36e41f524d6f4b33f0162070d0cb91eca58262b
- **Coverage:** whole committed source interval from 6037472180a6ea95568746ff217e98a93ed6130a.
- Whole source interval: DEV_NOTES.md bytes 25791–26911, 1120 bytes, sha256:0899bb9e4b4a2fea37bb412fb36e41f524d6f4b33f0162070d0cb91eca58262b
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

## _(2026-10-25 UTC)_ — correctly rounded sqrt numeric value kernel

0255 .5d.2.1 implements the recorded exact root plan in private roots.rs: integer bisection over
floor(N/D), then exact squared-midpoint comparison. Ratio radicands scale by10^6 for ppm output;
area returns length. Existing signature metadata determines result kinds; actual invalid kinds and
negative inputs have typed witnesses. Output stays a canonical rounded integer without premature
i64 narrowing. No float or new dependency is introduced. Source-bearing call provenance remains
.5e.3, broad/platform proof .5d.4/pre-push; hypot is next.

One focused native root target passes2/0, rc=0, including all15 independent sqrt rows: exact ties,
very close rational neighbors, full128-bit ratio, negative values and actual kind refusals. Hypot's
ten retained rows are not claimed as run. Book/live/API examples align; prior quantity-follow prose
is corrected. Complete0254 task/ledger and four closed structural marks/allowance nodes are retained
327/227/328 with live IDs/Goals/children preserved. Promotion: numerical decision records sqrt scope.

