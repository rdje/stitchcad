# Original-frame notch context

Notch-bounded allocation needs the actual semantic landmarks and topology journal,not guessed
parameters or physical notch dimensions. This chapter records the recovered requirements before
implementation. Exact current-point/original-coordinate helpers and notch quantity queries remain
pending;the [weighted source component](command-seam-splits.md#query-exact-original-weighted-quantities)
is already implemented with focused checks.

| API token | Meaning |
| --- | --- |
| `Param` | Bounded exact position in an edge's own original frame |
| `Rational` | Signed64-bit reference fraction,distinct from a private wide numeric workspace |
| `IdentityLedger` | Actual typed topology journal and reference resolver |
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

First provide bounded current-point uniqueness without constructing an uncontrolled choice tree.
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
```

Focused native range20/recovery4 and selected Clippy/format checks pass. Those checks validate actual
accepted source states,forward witnesses,derived choice growth and the corrected D305 resolution;
they do not claim implemented original-coordinate projection. The exact
pre-context sewing decision and closed D302–D304 ledgers are retained in repository history records
658/659;the current decision keeps standing contracts without a raised ceiling or another map row.
