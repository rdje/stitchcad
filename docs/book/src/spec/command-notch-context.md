# Original-frame notch context

Notch-bounded allocation needs the actual semantic landmarks and topology journal,not guessed
parameters or physical notch dimensions. Bounded current-point uniqueness and explicit original-point
workspace admission are implemented. Inverse coordinates and notch quantity contexts remain pending;the [weighted source component](command-seam-splits.md#query-exact-original-weighted-quantities)
is already implemented with focused checks.

| API token | Meaning |
| --- | --- |
| `Param` | Bounded exact position in an edge's own original frame |
| `Rational` | Signed64-bit reference fraction,distinct from a private wide numeric workspace |
| `IdentityLedger` | Actual typed topology journal and reference resolver |
| `UniquePointLimits` | Explicit positive journal/nested input budgets,without defaults |
| `OriginalPointLimits` | Explicit current-input,candidate and integer workspace limits |
| `OriginalPointAdmission` | Borrowed actual current point,original source creation and reserved widths |
| `OriginalPointAdmissionError` | Actual current/source/integer preflight refusal |
| `admit_original_point` | Admit wide inverse workspace before any wide integer arithmetic |
| `IntegerBits` | Conservative operand/result reservation exceeds the selected limit |
| `BoundOverflow` | Journal-derived reservation cannot be represented |
| `UniquePointResolution` | Original held reference and fixed bounded query outcome |
| `UniquePointOutcome` | Current point,required choice,repair,unknown source or admission refusal |
| `PointJournalCollection` | Actual declared-edge or offset-fragment collection |
| `resolve_unique_point` | Admit the whole journal before current uniqueness lookup/mapping |
| `JournalEntries` | Complete journal count exceeds the selected limit |
| `NestedItems` | Actual collection would exceed the aggregate nested budget |
| `CountOverflow` | Aggregate count cannot be represented |
| `SplitChoice` | Required first split choice;branch identities have not been folded further |
| `Repair` | Actual first repair cause with fixed immediate offset candidates |
| `TopologyEdit` | Declared split,merge,reverse,delete or offset mapping |
| `Notch` | Actual semantic anchor with deferred physical representation |
| `SewingLandmark` | Actual registered Notch or TurnPoint |
| `BetweenNotches` | Ease intent naming two semantic notch stops on one side |
| `ResolvedRef` | Current edge/parameter plus direction relative to its holder's frame |
| `Resolution` | Existing current resolution,explicit choice or repair evidence |
| `SplitPoint` | A required reference choice,even when later locations coincide |
| `Unresolved` | Reference repair outcome |
| `Original` | Direction agrees with the reference holder's original frame |
| `Reversed` | Direction opposes the reference holder's original frame |
| `UnitError` | Concrete numeric cause retained by the resolver |
| `Overflow` | Exact value exceeds the relevant stored representation |

## Valid current positions can need wide original coordinates

Split an edge at one half,then author a live notch on its first fragment at1/9223372036854775807.
The actual Notch and structural sewing graph are accepted. Its original coordinate is
1/18446744073709551614,a64-bit denominator that cannot fit the reference fraction's positive63-bit
field. The bounded multiplication correctly refuses that representation. A context must retain
this derived coordinate in a wider exact workspace rather than round,clamp or treat the valid
current notch as corrupt. The source reference representation itself is unchanged.

Even a tiny endpoint matters to quantization. Linear allocation between that original coordinate
and1/4 lies below one half at1/8. A1 µm source therefore allocates0 at that boundary;replacing the
tiny endpoint by zero or performing binary64 subtraction gives a half and allocates1. These are
exact arithmetic example inputs,not measured geometry or an implemented notch quantity verdict.

## Unique current location does not prove unique original coordinate

A native fixture splits at half,reverses the second fragment,then merges the two using equal
positive declared lengths. A newly authored notch at the merged half has a unique current
resolution. Two different original positions reach that location:original half through the explicitly
chosen first split branch,and original end through the reversed branch. The source-direction
witnesses are Original/Reversed;the newer notch's own merged-frame direction is Original.
Directions are relative to different holder frames,so they cannot be compared as absolute point IDs.

The first diagnostic incorrectly compared complete directed references and failed. The corrected
watch compares current edge/parameter and separately asserts all relative direction values. It
preserves their information. The explicit branch choice in the test is a forward witness,not a
repair or an inverse selection made for a consumer. A context must retain all actual origin
candidates and refuse ambiguous allocation. It must also check that the two selected original
notch coordinates are distinct;distinct current IDs/edges alone do not prove a positive original interval.

## Admit derived work before full resolution

Eight split/merge cycles produce256 resolved leaves from17 journal entries. The actual public
logical topology census is25 collection entries and accepts a25-entry limit. All leaves may share
the same current location while the held reference still requires choices. The collection meter
measures its documented input data correctly;it does not bound derived resolution data or CPU time.

New context construction must admit or avoid that expansion before invoking full resolution.
Bound current uniqueness separately,retain required choice/repair/arithmetic causes,and never
collapse a held reference's choices merely because their numeric locations happen to coincide.
Admission includes the actual journal and nested reference populations,candidate storage and wide
integer work. Whole-source/effect integration remains a separate owned stage.

The shared single-edit point calculation is implemented. It uses fixed immediate evidence and at
most two offset candidates;the journal enforces positive nonoverlapping offset intervals. It retains
exact arithmetic,relative direction,split branches and concrete repair causes. The existing full
resolver consumes this step and still follows both branches into its public choice tree. This
shared step supplies both consumers;it does not bound the full resolver.

## Query bounded current uniqueness

Choose both positive limits explicitly. Journal entries include all edits,including unrelated ones.
The nested budget sums every declared-edge and offset-fragment collection across that complete
journal. Fixed split/merge/reverse/delete fields are covered by their journal entries. Admission
finishes before source lookup or point mapping;no registration collection is read or charged.
A refusal retains the original held edge/parameter and actual operation/collection,prior consumed
count,whole next request and selected limit. Equality with a limit is accepted.

The query uses fixed storage and shared exact point steps without constructing the full choice
tree. An unknown source is explicit,while the existing point fold still has its documented
pass-through behavior. A known unique result agrees with that full fold. Actual deletion,offset
gap/shared-boundary and arithmetic causes remain concrete. Offset candidates retain their immediate
meaning and authored order,as in the full fold.

A first split choice stops the query. Its branch IDs refer to the edit's immediate fragments:the
first starts at its end,the second at its start. Neither is presented as a final current candidate.
Stated registration choices are not applied by this reference-only query. Coincident later
locations do not cancel a required reference choice. Relative direction is retained throughout.

```rust
use core::num::NonZeroUsize;
use sc_core::ontology::{DeterministicIdGenerator, IdentityLedger, Param, Rational,
    UniquePointLimits, UniquePointOutcome};
use sc_units::Length;
let mut ledger = IdentityLedger::new();
let mut ids = DeterministicIdGenerator::new();
let edge = ledger.declare_edges(&mut ids, 1)?.remove(0);
let half = Param::new(Rational::new(1, 2)?)?;
let fragments = ledger.split(&mut ids, edge, half)?;
let one = Length::from_micrometres(1)?;
ledger.merge(&mut ids, fragments.first(), one, fragments.second(), one)?;
let entries = NonZeroUsize::new(3).ok_or("positive journal limit")?;
let nested = NonZeroUsize::new(1).ok_or("positive nested limit")?;
let result = ledger.resolve_unique_point(edge, half, UniquePointLimits::new(entries, nested));
assert_eq!(result.reference(), edge);
assert!(matches!(result.outcome(), UniquePointOutcome::SplitChoice { source, .. } if *source == edge));
assert!(result.resolved().is_none());
# Ok::<(), Box<dyn std::error::Error>>(())
```

The answer is an observation of this journal;repeat after edits. This primitive does not certify
whole-range coverage,copy ownership,physical correspondence or original-coordinate uniqueness.
Its input counts do not constitute bus authority,an allocator/RSS limit or a CPU-time certificate.
Native6 plus the14 independent fraction rows cover actual limits/causes/choices/current mapping;
focused checks pass. Original-coordinate inverse,semantic context and paired effects remain owned.

## Admit original-coordinate workspace

Choose the existing current-point input limits,an explicit positive candidate limit and an explicit
positive integer-bit limit. The candidate limit covers all simultaneously retained numeric states,
including intermediate states. No default is supplied. Admission first runs bounded current
uniqueness,then finds the actual creation entry for the named original source,then reserves integer
work. It performs no wide arithmetic and produces no inverse coordinate or candidate collection.
A current refusal retains its full fixed outcome in one diagnostic box;numeric causes remain the
standard error source. Unknown target creation and integer-limit refusals keep both original source
and held reference/parameter. No journal,registration or anchor is modified.

For J actual journal entries,the conservative reduced coordinate-component bound is W =63+127J.
For an offset with current coordinate qn/qd and original bounds fn/fd,tn/td,the complete inverse has:

```text
numerator   = qn*(tn*fd - fn*td) + fn*qd*td
denominator = qd*td*fd
```

The bounds use at most63 bits. Thus the numerator gains at most127 bits and the denominator at
most126. Split adds at most64,positive declared-length merge at most32,and reverse adds none.
The admission reserves 2W+128 bits per integer operand/result. That also covers comparison products
and sequential unreduced offset scratch of at most W+190,because W is at least63. These are
conservative arithmetic bounds,not a measured maximum or an allocator/RSS/CPU certificate.
Six independent exact-integer cases include the last fitting64-bit reservation and its overflowing
successor. A recorded-seed Fraction check exercises960 inverse-offset steps against component and
scratch bounds;the accepted late-fragment64-bit coordinate remains its counterexample to a63-bit
original workspace. Native checks consume the exact reservation fixture.

```rust
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::ontology::{DeterministicIdGenerator, IdentityLedger, OriginalPointLimits,
    Param, Rational, UniquePointLimits};
let mut ledger = IdentityLedger::new();
let mut ids = DeterministicIdGenerator::new();
let edge = ledger.declare_edges(&mut ids, 1)?.remove(0);
let creation = ledger.journal()[0].operation();
ledger.reverse(&mut ids, edge)?;
let one = NonZeroUsize::new(1).ok_or("positive")?;
let two = NonZeroUsize::new(2).ok_or("positive")?;
let limits = OriginalPointLimits::new(UniquePointLimits::new(two, one), one,
    NonZeroU64::new(762).ok_or("positive")?);
let held = Param::new(Rational::new(1, 4)?)?;
let token = ledger.admit_original_point(edge, held, edge, limits)?;
assert_eq!(token.source_creation(), creation);
assert_eq!(token.current().param().as_rational(), Rational::new(3, 4)?);
assert_eq!(token.integer_bit_bound(), 762);
# Ok::<(), Box<dyn std::error::Error>>(())
```

The immutable ledger borrow keeps the token tied to this exact journal. Bounded Debug omits the
journal body. A reversed edge may keep its identity while its current parameter differs from its
creation-frame coordinate;the inverse must stop at the actual creation event. Known source creation
and current uniqueness do not prove that a preimage exists in the named source:two unrelated real
edges can pass this admission. Equal numeric forward witnesses also do not cancel an old held
reference's split choice. Actual inverse candidate production remains the next implementation.
Notch interval/amount arithmetic,whole Design readiness and physical correspondence need their
separate admission/context checks;this inverse workspace does not authorize those later operations.

## Current-reference composition defect

D305 is repaired with focused verification. After a split at1/3,the valid held position
1537228672809129301/4611686018427387902 resolves to the second fragment at1/9223372036854775804.
The temporary offset1/13835058055282163706 needs64 denominator bits,but the complete normalized
result fits the supported63-bit field. Split-second and offset maps now use the existing exact
complete-progress kernel;point and range merge maps share complete arc-length recomposition.
Reduction precedes the return to bounded reference fields. No coordinate is rounded or clamped.

The independent Fraction fixture covers14 cases through both point and range resolution:nine
valid results defeat premature intermediate checks,five genuinely wide final fractions still
produce numeric repairs. Repairs retain the held source and position/range plus the actual
completed-operation cause. Declared merge lengths remain caller-supplied arc-length evidence;
this repair adds no measured geometry or physical correspondence.

```rust
use sc_core::ontology::{DeterministicIdGenerator, IdentityLedger, Param, Rational};
let mut ledger = IdentityLedger::new();
let mut ids = DeterministicIdGenerator::new();
let source = ledger.declare_edges(&mut ids, 1)?[0];
let split = Param::new(Rational::new(1, 3)?)?;
let fragments = ledger.split(&mut ids, source, split)?;
let held = Param::new(Rational::new(1537228672809129301, 4611686018427387902)?)?;
let current = ledger.resolve(source, held).resolved().ok_or("expected exact current position")?;
assert_eq!(current.edge(), fragments.second());
assert_eq!(current.param().as_rational(), Rational::new(1, 9223372036854775804)?);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Selected implementation boundary

Bounded current-point uniqueness is implemented without constructing the full choice tree.
Then project exact coordinates backward through actual journal mappings to the named source edge's
creation frame. Split/offset use exact affine maps,reverse retains relative direction,and merge
joins retain all applicable predecessor candidates. Equal coordinate/direction states may coalesce
for numeric work. This does not rewrite references or cancel current reference repair requirements.

A63-bit current coordinate needs at most127 additional component bits per inverse edit:an offset
uses two63-bit bounds plus an addition;split/reverse/positive-length merge need no more. Thus
63+127J is a conservative component bound for J journal entries. Comparisons and notch interval
arithmetic must reserve their additional products explicitly before allocation. This is selected
arithmetic/workspace design,not an implemented algorithm or allocator/CPU certificate.

Use the actual original source,side,copies,full stops,registry and journal. Preserve named notch
identities and authored stop order while locating their original interval;both named orders are
structurally legal. Unknown,orphaned,ambiguous,outside and numeric outcomes keep concrete source data.
Formula/Profile amounts remain unavailable until their actual resolution. Physical correspondence,
complete Design readiness,command authority and release retain their own checks.

The independent Fraction producer freezes three recovery cases:

```bash
python3 -I -B docs/tasks/artifacts/seam_partition/notch_context_oracle.py --check
python3 -I -B docs/tasks/artifacts/seam_partition/reference_recomposition_oracle.py --check
python3 -I -B docs/tasks/artifacts/seam_partition/projection_workspace_oracle.py --check
```

Focused native range20/recovery4 and selected Clippy/format checks pass. Those checks validate actual
accepted source states,forward witnesses,derived choice growth and the corrected D305 resolution;
they do not claim implemented original-coordinate projection. The exact
pre-context sewing decision and closed D302–D304 ledgers are retained in repository history records
658/659;the current decision keeps standing contracts without a raised ceiling or another map row.
