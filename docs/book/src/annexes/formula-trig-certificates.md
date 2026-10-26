# Annex: investigating bounded trig rounding certificates

Sin/cos/tan and atan/atan2 are specified but remain unimplemented. The following is a research
result, not an available formula operation or an admitted runtime precision. Its owner is
G1-SLICE.5d.1.2.2.2; full execution also retains the other dependencies in the
[execution plan](formula-execution-plan.md).

## Why output thresholds might help

Sine and cosine produce ratios rounded to integer parts per million. Their positive rounding
midpoints are (2j+1)/2,000,000 for j from0 through999,999: one million thresholds. For example,
the first midpoint is0.0000005 and the last is0.9999995. Symmetry covers negative results.

An admitted rational angle has at most128-bit numerator/denominator magnitudes in microdegrees.
Writing it in turns gives a denominator bounded by Q=360,000,000*(2^128-1), a157-bit integer.
Integer-microdegree period/quadrant/octant reductions preserve this bound. This is an input
denominator bound, not a runtime precision choice.

For each output midpoint r, enclose both inverse values asin(r-delta)/(2*pi) and
asin(r+delta)/(2*pi). If the entire interval lies strictly between a/b and c/d where
bc-ad=1, both denominators are at most Q, and b+d>Q, then it contains no admitted rational phase.
Indeed any strictly intermediate rational p/q has
q=d*(bp-aq)+b*(cq-dp)>=b+d>Q. Both integer factors are positive.
This would prove every true admitted sine stays more than delta away from that midpoint.

Once every threshold passes, a separately justified runtime sine enclosure narrower than delta
could round unambiguously. Neither the candidate interval constructor nor a small sample closes
that complete argument by itself.

## What the feasibility cohort establishes

The tracked Python producer uses directed512-bit integer intervals, the existing certified640-bit
pi enclosure, integer square roots, half-angle identities and an alternating arctangent series.
The candidate value band is delta=2^-384. These are experimental certificate parameters.
The independent oracle uses exact Fraction arithmetic and a different, positive arcsine series.
The series are documented by [NIST DLMF4.24](https://dlmf.nist.gov/4.24).

Run from the repository root:

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/finite_threshold_feasibility.py
```

Work unit0277 certified38 sampled forward bands. Controls cover three exact inverse values,
four independent series enclosures, exhaustive rational checks at32 small denominator bounds,
and two required refusals. One refusal deliberately places the real sin30-degree value1/2
inside a band; another uses a band wide enough to contain admitted inputs.
The report explicitly says feasibility_cohort_only and retains exact endpoints and source hashes
under target/trig_threshold_feasibility/0277/. It does not claim the full million thresholds.

## Remaining admission work

Complete forward coverage must retain exact predicates, source identity and measured cost.
A native kernel still needs its own complete error, termination and workspace argument, followed
by focused integration evidence. A failed experimental gap cannot become a new language input
restriction or a silently substituted numeric answer.

Inverse angle rounding has45 million half-microdegree thresholds in its first octant after
sign/quadrant/reciprocal symmetry. Exact atan2 slopes can require256-bit denominators in private
workspace; a planned tangent-of-angle-band certificate must account for that bound. This initial
forward cohort implements neither that generator nor its complete coverage. Axes, poles and
signed principal branches still need their existing exact rules. Tangent's much larger output
threshold population retains a separate practical resource obligation.
