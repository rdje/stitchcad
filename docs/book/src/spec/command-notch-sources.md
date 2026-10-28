# Actual notch source context

Complete actual input admission,bounded local validation and original-notch coordinate context are
implemented with focused checks. They build on the [point and interval primitives](command-notch-context.md).
Separate explicit quantity arithmetic admission and exact cumulative/interval queries are implemented
with focused actual Design source/query checks. [Captured command views](command-seam-quantities.md)
consume the actual sealed context;higher caller integration and paired effects remain owned.

| API token | Meaning |
| --- | --- |
| `NotchContextLimits` | Explicit complete dependency and later per-query point/range limits |
| `NotchSourceAdmission` | Borrowed actual source/plan/Pieces/registry/ledger after input admission |
| `NotchSourceAdmissionError` | Fixed original source identity and actual admission refusal |
| `NotchSourceAdmissionIssue` | Original intent or complete journal/dependency population refusal |
| `NotchSourceCollection` | Actual source,copy,Piece edge/hole/fold or landmark collection |
| `admit_notch_context` | Admit complete actual source dependencies before indexes or derived work |
| `OriginalNotchContext` | Actual named Notches and distinct in-domain original interval |
| `OriginalNotchContextError` | Source/named IDs and complete original-context refusal evidence |
| `OriginalNotchContextIssue` | Actual admission/projection/frame/ambiguity/bounds/coincidence cause |
| `original_context` | Bind actual locally validated source to exact original notch coordinates |
| `NotchEaseLimits` | Explicit positive quantity integer width allowance |
| `NotchEaseAdmission` | Borrowed sealed context and original signed literal after separate admission |
| `NotchEaseWorkspaceError` | Actual symbolic amount or conservative quantity reservation refusal |
| `admit_explicit` | Admit quantity arithmetic over the actual sealed original-notch context |
| `NotchEaseAmountError` | Actual original frame,bounds,context or concrete quantity cause |
| `cumulative_at` | Quantized original notch-bounded cumulative amount in canonical A order |
| `amount_in` | Difference original quantized boundaries for a distribution-side interval |
| `NotchSourceValidation` | Borrowed local structural proof after actual current-source checks |
| `NotchSourceValidationError` | Original source identity and concrete bounded validation refusal |
| `NotchSourceValidationIssue` | Actual structural,point,coverage,quota or derived-bound cause |
| `NotchSourceRange` | Source-side or authored Piece-edge query location |
| `validate_sources` | Validate local current source structure using admitted actual inputs |
| `SeamEaseSource` | Actual retained original declaration,domains,copies and full stops |
| `CutPlan` | Actual complete physical-copy plan |
| `Piece` | Actual immutable source Piece with held interval references |
| `SewingLandmark` | Actual Notch/Turn registry entry |
| `IdentityLedger` | Actual typed topology journal |
| `OriginalPointLimits` | Explicit current-input,candidate and integer limits per inverse query |
| `BetweenNotches` | Original side and named notch identities,without supplied coordinates |

## Admit complete actual source dependencies

Original-notch context input admission is implemented. Supply the actual retained original source,
complete physical-copy plan,Piece registry,semantic Notch/Turn registry and topology ledger. The
returned token borrows those exact objects,without a copied declaration,synthetic graph or supplied
notch coordinates. It retains the declaration's actual side and named start/end IDs in named order;
input admission does not reorder them by coordinate or alter the original authored stop list.

Choose positive dependency-item,original-point and range-portion limits explicitly. The point
limits include complete journal/nested input and per-point candidate/integer budgets. The range
limit is the peak portion budget per later coverage query. These respective query limits are
retained for local validation and later coordinate context;this input-only operation performs no
projection or range query,
reserves no derived numeric result and does not certify an aggregate allocator or CPU-time bound.

Eligibility comes first. Uniform intent returns its actual source identity;weighted intent also
retains its actual density side and complete region count,without cloning the density. For an
actual BetweenNotches source,complete journal/nested admission precedes the dependency census.
Then charge every original source stop,every plan copy and every supplied Piece. For each Piece,
charge its whole boundary,hole-loop collection,each hole's whole edge list,construction lines and
fold declarations in schema order. Finally charge the complete supplied semantic registry,
including unused Notch and Turn entries. Filtering down to the two named notches would omit actual
source dependencies. Each collection is admitted before traversal;quota equality is accepted.

A refusal retains the original source span,actual collection and source/Piece owner,authored Piece
or hole index,prior consumed count,whole next request and selected limit. Journal refusals retain
the original operation and nested collection. There is no partial index or truncated registry.
Private exhaustive source/plan/Piece/mark views and Copy guards expose future owned-field changes.
Piece label and Material reason text stays borrowed without scanning or cloning;its bytes are not
part of this dependency-item census. No formula is evaluated and no physical profile value is
supplied. Explicit,formula and profile amounts all preserve their actual source declarations.

```rust
use core::num::NonZeroUsize;
use sc_core::ontology::{CutPlan, IdentityLedger, NotchContextLimits, NotchSourceAdmission,
    NotchSourceAdmissionError, OriginalPointLimits, Piece, SeamEaseSource, SewingLandmark};
fn borrow_actual<'a>(source: &'a SeamEaseSource, plan: &'a CutPlan, pieces: &'a [Piece],
    marks: &'a [SewingLandmark], ledger: &'a IdentityLedger,
    dependency_items: NonZeroUsize, point_limits: OriginalPointLimits,
    range_portions: NonZeroUsize)
    -> Result<NotchSourceAdmission<'a>, NotchSourceAdmissionError>
{
    let limits = NotchContextLimits::new(dependency_items, point_limits, range_portions);
    source.admit_notch_context(plan, pieces, marks, ledger, limits)
}
```

Admission success proves complete input populations,not ownership,currentness,unique original notch
positions or quantity availability. The immutable borrows prevent changing the supplied sources
while the token remains in use. Bounded Debug reports fixed identities,limits and population counts
without traversing label text,Notch definitions or the journal body. Current graph validation can
still refuse a deleted source edge while this input token preserves the actual held source;the
Uniform-only amount helper still reports its original BetweenNotches context requirement.

Focused native checks use actual validated graphs,original sources,Pieces,CutPlans,Notch/Turn
objects and ledgers. They cover symbolic declarations,named versus authored stop order,complete
unused registry/Piece/copy dependencies,hole/fold boundaries,journal refusal precedence and damaged
held-source preservation. No-default and immutable-borrow compile-fail checks cover the public
boundary. The bounded local source validation below follows this input step. Original-coordinate context
construction follows local source validation;neither its success nor quantity availability follows from input admission.

## Validate actual local source context

Call validate_sources on the admitted actual input token. The returned proof borrows the same
immutable source,plan,Pieces,registry and ledger. It validates local structural currentness;original
notch coordinates,quantity availability,whole Design and physical readiness remain separate.

Before building any index or querying ranges,derive and check complete workspace cardinalities.
For N actual admitted dependency items and B selected peak portions per range query,reserve N*B
ownership-cache intervals and a conservative3N+1 metadata-row envelope. The latter covers public
CutPlan validation's temporary fixed-copy arrays and local identity/Piece/mark/stop indexes. Check
both products/additions;unrepresentable reservations return DerivedBoundOverflow before allocation.
Every ownership-cache append and local index insertion checks its bound before growing storage.
The proof records these bounds and actual validation cache/index counts;the transient collections
are discarded before returning. These are logical populations,not a measured allocator/RSS/CPU or
whole-bus certificate;the envelope is conservative.

Check the current CutPlan against the actual complete Piece registry and retain its original typed
cause. Check local source/Piece/copy/landmark identity uniqueness. Resolve each Piece's held edges
with bounded positive-interval coverage,retaining surviving intervals for ownership queries. Then
check every semantic registry landmark,including unused entries:its actual Piece must exist,its
actual anchor needs bounded unique current resolution,and that point must belong to its Piece's
surviving intervals. No new semantic object is constructed to replace an old held reference.

For each original side,require the actual physical copy,unique lower then upper current endpoints,
complete positive-interval coverage and full ownership by that copy's actual Piece. Check A before
B. Same-copy positive interiors cannot overlap. Every original stop must retain actual side/Piece/
current-interval membership,even when child stop lists are empty. Named ease boundaries must still
be semantic Notches and have distinct current locations. Named IDs and original authored stop order
remain unchanged. Explicit,formula and profile declarations are preserved without evaluating ease.

This proof has no supplied graph identity and creates none. Other graph members,global Design/root
ownership,current formula metadata and physical correspondence use their actual separate validators.
Surviving Piece intervals supply local membership;repairs elsewhere remain visible without becoming
a whole-Piece topology certificate or an automatic local rejection. Current Design sewing validation
continues to check its complete actual root,all landmarks,full graph and amount metadata.

A refusal retains the original source and concrete evidence. Current point/endpoint failures keep
the fixed bounded outcome,including first split branches,actual repair and held edge/parameter.
Positive-interval failures keep complete bounded coverage evidence;quota errors retain the actual
source side or flattened authored Piece-edge index and original held range. Structural copy/stop/
kind causes remain the existing typed sewing causes. Numeric point and internal-range causes remain
available through the standard error source. No partial source proof,nearest-point choice,legacy
endpoint tree or reference rewrite is produced.

```rust
use sc_core::ontology::{NotchSourceAdmission, NotchSourceValidation, NotchSourceValidationError};
fn validate_actual<'a>(admitted: NotchSourceAdmission<'a>)
    -> Result<NotchSourceValidation<'a>, NotchSourceValidationError>
{
    admitted.validate_sources()
}
```

Focused actual-model checks cover symbolic declarations,copy replacement,same-ID Piece binding,
local identity collisions,unused deleted anchors,full original stops despite child subsets,and
current Notch-kind replacement. They preserve the first endpoint split choice through eight cycles.
A three-slot query refuses before constructing three portions plus its active input;explicit four
slots reach the intended deleted-interior evidence. A valid surviving subdomain remains usable.

Another actual source has unique current endpoints and newly born valid current Notches,but an
internal retained range boundary maps outside the stored reference fraction width. Local validation
retains that full numeric repair and original operation rather than accepting endpoint evidence as
complete coverage. An independent Fraction control confirms the internal normalized half requires
a wider stored component. Current-anchor numeric causes remain distinct. Source success still does
not prove unique original coordinates:the coordinate context below checks their full evidence.

## Construct the actual original-notch interval

Call original_context on the local source-validation proof. That proof now retains the actual named
Notch references from its validated registry. The context borrows those same objects and source
inputs;it accepts no separate coordinates,synthetic registry or guessed geometry. It exposes named
start/end projections and mathematical lower/upper projections separately. A source named401 then400
can have lower400 and upper401 without changing either named IDs or authored stop order.

Let C be the explicit original-point candidate limit. Check C+1 cardinality before any wide
projection:one named result is retained only after it has exactly one candidate,while the other
query may retain at most C states. An overflowing reservation refuses before projection. Admit both
actual named anchors against the actual original distribution-side edge before wide work,using the
same immutable ledger and explicit original-point limits. This includes the actual side-B source
when the distribution is on B;Same/Opposite correspondence does not silently change its numeric axis.

Project each actual Notch anchor into the actual source edge's creation frame. Require matching
immutable ledger,source edge and actual creation entry before comparing values. Equal IDs/journal
contents on a separate ledger snapshot do not supply that frame binding. Require one complete
coordinate/direction candidate per named Notch;an ambiguous result retains every candidate and its
relative mapping direction. No candidate is selected by proximity or an absolute-direction filter.

Compare exact originals against the actual full original side bounds,inclusively,without Param or
u128 conversion. Both named originals must be inside that domain and mathematically distinct.
Distinct current edges/IDs do not prove a positive original interval. If a split's first-end and
second-start Notches both project to the same original half,construction refuses and retains both
actual projections. Valid wide originals survive optional bounded-view refusals.

```rust
use sc_core::ontology::{NotchSourceValidation, OriginalNotchContext, OriginalNotchContextError};
fn inspect_original<'a>(validated: NotchSourceValidation<'a>)
    -> Result<OriginalNotchContext<'a>, OriginalNotchContextError<'a>>
{
    let context = validated.original_context()?;
    let (named_start, named_end) = context.notches();
    assert_eq!((named_start.id(), named_end.id()),
        context.validation().admission().notch_ids());
    // lower()/upper() keep full projection frame evidence;they do not rename start/end.
    assert!(context.lower().unique().is_some());
    assert!(context.upper().unique().is_some());
    Ok(context)
}
```

Focused actual-model tests cover named/authored order,symbolic source declarations,closed original
bounds,actual side-B frames,64-bit valid original coordinates,ambiguous reversed-merge witnesses,
coincident originals on distinct current edges,and eight aligned split/merge cycles. C=2 yields the
expected quarter/half interval in that cycle case;C=1 retains the actual inverse-branch quota cause.
Integer admission and combined candidate overflow also keep their actual original causes.

Failures keep original source/side/named IDs. Admission/projection/numeric causes remain available
through standard error sources. Completed ambiguous,coincident,frame or bounds refusals retain the
actual bounded projections and ledger borrows,so diagnostic lifetimes are explicit. No partial
context,implicit reference choice or source/Notch mutation escapes. The private comparison ports
also refuse a separate equal-content ledger or wrong source frame before numeric work.

This context supplies exact original positions. Separate admitted quantity queries below consume
those positions. Formula/profile values,current whole Design readiness,physical matching and paired
effects retain their subsequent owners. The existing Uniform-only helper still reports BetweenNotches;
constructing this context does not default an amount or extend that old helper implicitly.

## Actual Design integration

Focused component acceptance joins `DesignSewing::amount_declarations()` to the actual retained
source and context. Two retained child views borrow one canonical full source,even with empty child
stop lists. Its named Notches remain start401/end400 in the earlier example;authored stop order and
mathematical lower/upper order remain separate. The actual integration fixture uses IDs98001/98000
and covers both A/B distribution sides and literal/formula/profile amount metadata. Its complete
dependency count is10;with per-range peak2,the interval bound is20 and metadata index bound31.

A later copy replacement is refused against the actual original source before a context is accepted.
After splitting the source edge at half,new Notches on its first fragment at1/MAX and1/2 recover
original1/(2MAX) and1/4. The first remains exact despite `Param` refusal. The retained child's held
endpoint at original half still requires a split choice,so complete current sewing validation refuses
that child while the full original source context succeeds. Context construction changes neither
the held endpoint answer nor source/formula metadata. This is source-context component acceptance;
captured command views have their own focused checks;whole paired/mutation/physical proof remains owned.

## Admit original-notch quantity arithmetic

Call `context.admit_explicit(NotchEaseLimits::new(bits))` with an explicit positive `NonZeroU64`.
There is no default. The returned `NotchEaseAdmission` borrows the sealed context and retains the
full original signed literal,selected limits and conservative integer bound. It copies no projection
or source collection and allocates no query integer. Formula/profile amounts refuse with their
actual declaration before checking the quantity allowance,including when that allowance is1 bit.
Earlier inverse/comparison admission does not license this separate arithmetic construction.

For mathematical lower L=ln/ld and upper U=un/ud,an interior original query x=xn/xd has fraction
`((xn*ld−ln*xd)*ud)/((un*ld−ln*ud)*xd)`. Coordinates have admitted component boundW and stored
query fields have at most63 bits. Each ratio field needs at most2W+63 bits;clamping outside the
notch interval and exact complementation cannot increase this. Multiplying by the maximum original
Length magnitude adds30 bits. The separate conservative reservation is therefore **2W+93** bits.
Bound arithmetic is checked in u64 before quantity allocation. The actual source with one journal
entry hasW190 and reserves473 bits;with two entries,W317 reserves727. A limit one below either
reservation refuses even when the actual coordinates happen to be small. These are conservative
operand/result bounds,not allocator/RSS/CPU certificates or fresh whole Design authority.

The independent producer checks six bound/overflow rows and302 Fraction/raw-integer width controls,
including64/188-bit originals. Native checks consume those rows and admit the actual later-frame
64-bit original without narrowing it. Privacy/lifetime and no-default doctests guard the public
boundary. The query implementation below consumes that separate admission:

```bash
python3 -I -B docs/tasks/artifacts/seam_partition/notch_quantity_oracle.py --check
```

## Query exact original-notch quantities

Use `admitted.cumulative_at(at)` with an `EdgeAnchor` in the actual full original distribution-side
edge/domain. Ascending progress is0 at or before the mathematical lower Notch and1 at or after
the upper Notch;inside,use the exact original ratio above. Named start/end IDs remain unchanged.
For B/Opposite,complement the exact fraction **before** applying the shared signed half-away rule.
The full original literal remains the source on every call,including retained child queries.

For original Notches at1/4 and3/4 and a1 µm source,the implemented cumulative results are:

| Original boundary | A or B/Same | B/Opposite |
| --- | --- | --- |
| `0` | `0 µm` | `1 µm` |
| `1/4` | `0 µm` | `1 µm` |
| `1/2` | `1 µm` | `1 µm` |
| `3/4` | `1 µm` | `0 µm` |
| `1` | `1 µm` | `0 µm` |

Negative sources produce signed mirrors. Subtracting an already rounded B prefix would give0 at
the half tie instead of1,so it cannot supply the complemented result. `amount_in(interval)` checks
the actual original edge and **both** closed-domain endpoints before computing either cumulative
value. It differences those original rounded values in canonical A order,including on Opposite B;
it never renormalizes a child or treats a rounded child amount as the source. Partition amounts sum
to the original literal,including negative,zero and maximal supported magnitudes.

Frame/domain checks happen before query integer allocation and remain active for zero ease.
`NotchEaseAmountError` preserves actual source/side/anchor or interval/bounds,private common-frame
preconditions,or the concrete numeric cause as its standard error source. The admission retains
the sealed actual context and complete original projection evidence. A private closed semantic
port transports only actual projections,`Param` and `Length`;no dependency integer API escapes.
The kernel guards integer operations under the separately admitted width and uses the shared
rounding policy. Width admission does not certify dependency internals,allocator or CPU costs.

The actual native ledger/source/context queries consume380 independently frozen Fraction rows.
These cover A/B,Same/Opposite,reversed current frames,clamping,source bounds and five signed/zero/
maximal literals. After an offset to original half,a later Notch at local1/MAX gives original
1/(2MAX);with upper1/4,the query at1/8 is below the half quantum and returns0,while binary64
would round1. Another actual partial offset preserves188-bit originals and a valid below-half
query;even u128 views refuse,but the quantity stays exact. Full Piece coverage retains surrounding
repair portions:the interior-offset fixture needs an explicit peak4 for its3 portions plus active
input. This local source success grants no whole Piece or Design certificate.

Private actual-projection controls preserve concrete quota operations and refuse a separate
equal-content ledger snapshot before numeric work. Focused quantity checks pass. The actual Design
source/query acceptance below is separate from captured command view checks,nonuniform component acceptance
and whole paired/mutation/physical verification.

## Actual Design quantity acceptance

The focused actual Design fixture joins complete current amount declarations,two retained children,
one canonical full original source and the actual semantic Notches. For either distribution side,
with original Notches at1/4 and3/4 and signed source±2 µm,a descriptive A cut at1/4 and Opposite B
cut at3/4 yields child amounts0 and±2 µm. The actual Uniform counterfactual spends±1 at that first
boundary. Both child views continue to lend the full original±2 source;their empty stop lists do
not erase the original two stops,and their exact amounts sum to that original total.

After an actual copy replacement,fresh Design amount declarations refuse the child's MissingCopy;
fresh local source validation refuses the full original source's MissingCopy. The earlier sealed
immutable source/ledger context can still answer its mathematical quantities. This preserves the
boundary between a snapshot query and current Design readiness. Currently valid formula/profile
metadata and exact original coordinates still refuse numeric admission with the actual symbolic
source,even ahead of a1-bit allowance. Neither a value nor physical correspondence is fabricated.

Five focused native cases include the new signed/symbolic quantity acceptance and directly affected
source-context fixtures. This accepts the source/query component with focused checks;convenience
captured command views have focused checks;whole nonuniform and paired/mutation/physical proof remain owned.

After implementation,the scoped original-notch component checkpoint passes all34 public Notch
cases and4 private original-point cases. Independent projection,width,comparison,recovery and
quantity oracles pass,including380 exact quantity rows. Together with the actual Design5 checks,
this closes the original-notch source/query component. Higher caller integration,physical matching,
paired effects and broader acceptance remain explicit subsequent work.

The canonical decision is docs/decisions/decision_sewing-spans-address-copies-and-permit-disjoint-self-seams.md.
Its whole pre-0512 source is logical record docs/history/stitchcad-devnotes-part670.md,available
through the [public archive reader](../annexes/archive-reader.md). It retains prior prospective
wording and all historical evidence exactly. Focused implementation and source-context integration
checks above are distinct from higher caller integration and whole paired/mutation/physical verification.
