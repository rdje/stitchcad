# Inverse rounding-threshold certificate feasibility

- **Type:** `decision`
- **Date:** `2026-10-26`
- **Status:** `active`; bounded feasibility only, no full population or native inverse admission
- **Owner / source:** engineer proposal, G1-SLICE.5d.1.2.2.2.b.2.a; complete proof .b/.c

answers: "what bounds inverse-function rounding work?" · "does the inverse cohort admit atan or atan2?"

## Decision

Continue the finite-threshold approach with a bounded inverse cohort. The
[forward decision](decision_finite-threshold-feasibility.md) derives45,000,000 first-octant
half-microdegree targets p_k=(2k+1)/(720M), k=0,...,45M-1, M=1,000,000. Sign/quadrant and
reciprocal-slope symmetries cover other non-axis cases only when their exact runtime contracts
are implemented. Axes and principal branches retain separate obligations.

Atan2's private rational slope can have denominator up to Q=(2^128-1)^2,256 bits. Atan's
ppm-ratio slope is covered by that conservative bound. No public128-bit ratio is materialized
by the proposed certificate. Actual geometry selectors retain their separate input contracts.

For each p_k, enclose tan(2*pi*(p_k-delta)) through tan(2*pi*(p_k+delta)). Strict containment
inside a determinant-one rational gap a/b<c/d with b,d<=Q and b+d>Q excludes all admitted
rational slopes from that angle band, by monotonicity and the exact gap identity in the forward
decision. Complete coverage could prove a uniform inverse-answer margin delta in turns.
This does not itself supply a native inverse algorithm or its error/workspace proof.

## Candidate construction

Try delta=2^-600 turns, scale2^640, the existing certified640-bit pi enclosure and64 directed
sin/cos terms. Rational target endpoints convert to radian intervals without floating point.
Their first-octant arguments are below1 radian. The same width invariant as the forward plan
gives term width<=12 scaled units;64 terms plus outward alternating tail fit770 units.
The exact128! exceeds2^640, so each series tail is smaller than one quantum. Divide the sine
enclosure by the strictly positive cosine enclosure outward to bound tangent. A small extension
beyond pi/4 exists only for the rational45-degree negative control, with argument still below1.

These are experimental certificate parameters, not language limits or admitted runtime budgets.
Any failed experimental gap is investigated; it never becomes a formula refusal or substituted
value. Tangent as a production formula function remains a different resource problem.
[NIST DLMF4.19](https://dlmf.nist.gov/4.19) supplies the series and
[DLMF4.21](https://dlmf.nist.gov/4.21) the angle identities, opened2026-10-26.

## Focused evidence — work unit0282

Run python3 -I -B docs/tasks/artifacts/formula_structure/inverse_threshold_feasibility.py.
It passes38 distinct bands, including endpoint/central neighbors and spread indices, rc=0.
The report scope is inverse_feasibility_cohort_only, retaining exact slope/gap endpoints, source
hashes, candidate parameters and observed cohort timing under target/trig_inverse_feasibility/0282/.
Its timing is maintenance observation, not a guaranteed full-population or native duration.

Four distinct algebraic oracles check tangent at turns0,1/24,1/16,1/8:0,2-sqrt(3),sqrt(2)-1,1.
Square-root bounds use768-bit integer sqrt, independent of the series constructor. Two exact
Fraction arctangent series at slopes1/4 and1/2 independently bracket the phase whose tangent
must contain that rational slope. The actual tan45 rational and a deliberately wide band must
refuse gap certification. Thus an always-accepting predicate cannot pass the controls.
Pure import with Path.read_text patched to refuse also passes, rc=0; pi input remains explicit.

Complete45M coverage, its retained cost/source/hash evidence and an independent exact payload
reader remain .b.2.b. Native atan/atan2 folding/branches/error/workspace admission remains .b.2.c,
then implementation .5d.3.3. No native inverse function, full recipe execution or platform claim
is made by this cohort. The existing forward certificate and sin/cos APIs remain unchanged.

## Native chunk prototype — work unit0283

The pure-stdout Rust diagnostic example inverse_threshold_chunk accepts start/count and the
certified pi endpoints on stdin. It emits288-byte rows: two640-bit slope endpoints and four
256-bit rational-gap components. Count is bounded to100,000 rows per child, a diagnostic chunk
contract rather than a language input limit. It does not read/write project files or implement
an inverse formula API. Every exact gap predicate is checked before its row is emitted.

Evaluate tangent once at the target center and widen by16*2^-600 in slope. Every target angle
band lies inside the first octant because2^-600<1/(720M). There tan'=sec^2<=2 and2*pi<8, so
the displacement from center is strictly below16*2^-600. Directed center enclosures plus that
exact padding therefore contain both endpoint images. This replaces duplicate series work with
a justified wider certificate band; it does not change the required angle margin.

Run python3 -I -B docs/tasks/artifacts/formula_structure/native_inverse_probe.py.
It validates the full local source/output/store plan before Cargo dispatch, uses Cargo's actual
reported release artifact, and checks its hash plus source hashes around execution. Its Python
reader independently checks10,035 rows' exact gap predicates.67 prefix/spread cases contain the
separate Python endpoint bands; five actual invalid native inputs refuse before any output.
The native probe and selected strict example lint pass, rc=0. An actual10,000-row call took
765,937,250ns on this run, excluding build and Python reading. This is not a duration guarantee.

The report/prefix/build record stays under target/native_inverse_probe/0283/. No full45M result
or native inverse error/workspace/branch admission is claimed. Full controller/source-locked
checkpoints/coverage reader follow .b.2.b.2; runtime admission remains .b.2.c. Broad verification
stays .5d.4/pre-push. The previous forward certificate and product sin/cos code remain unchanged.

## Resumable controller and frozen provenance — work unit0284

inverse_threshold_certificate.py owns450 bounded100,000-row chunks under
target/inverse_threshold_certificate/0284/. It guards every source/store/output before producers,
captures all build-source bytes, verifies the actual Cargo executable, copies it byte-exact with
its execution permissions, and records hashes. Frozen build inputs remain beside the retained
producer so future unrelated library work cannot overwrite or ambiguously stale this provenance.
Live native source, pi fixture, controller and reader are the critical math/coverage basis and
must still match. Frozen inputs and producer are rechecked before dispatch and publication.
This separates captured build history from dependencies of the current mathematical claim.

An exclusive run lock and owned-child registry prevent concurrent controllers. Children write
unpublished parts; only terminal success plus independent exact row/size/hash checks publishes
a chunk, followed by an atomic manifest. Completion means all450 unique contiguous ranges.
Resume verifies every published chunk. Unpublished final bytes are replaced only by identical
regeneration; differing bytes are preserved and refused. Error cleanup terminates/reaps only the
controller's owned children. After abrupt interruption, inspect recorded processes before removing
its lock; partial chunks may be regenerated, published evidence may not be silently trusted.

Run the controller with --chunks1 --workers1 to bound one invocation; use
verify_inverse_certificate.py --allow-partial for explicitly partial evidence. Without that flag,
the reader refuses any incomplete population. The full controller defaults to at most4 workers;
this is maintenance concurrency, not a formula-language parameter or native inverse admission.

Focused runs produce100k then resume to200k in2 chunks, rc=0; independent partial reading passes.
Default full reading correctly refuses partial coverage. Controls reject3 corrupted rows,
2 bad completion/coverage records and1 stale source hash. A prohibited unrelated-live-source read
does not affect frozen-source verification. Two actual owned test children are terminated/reaped,
rc=0; the controller registry is empty and its lock absent. No full45M result is claimed.

The read-only cleanup inspection also found that generic .bin cleanup accepts actual forward
and inverse certificate chunks as deletion candidates. No deletion occurred. P1 .b.2.b.2.h owns
protection immediately after this controller commit and before full coverage or the next cleanup.
