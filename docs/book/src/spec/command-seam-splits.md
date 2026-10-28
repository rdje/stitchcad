# Paired seam split source binding

The sc-core SplitSeamSpan input can now bind its original source cuts to the actual held SewingGraph.
This is an implemented native prerequisite for the paired handler. Current copy/range resolution,
physical pairing,complete ease/stop conservation,lineage,command admission and atomic effects retain their
execution owners. The bus still exposes its three implemented metadata handlers.

## Bind the actual held source

| API token | Meaning |
| --- | --- |
| `SplitSeamSpan` | Original target correspondence and separate exact A/B anchors |
| `bind_source_cuts` | Look up the actual held target and check A then B source-frame/interior bounds |
| `CommandSeamSplitBinding` | Sealed borrowed request/span view and fixed lower/upper intervals for both sides |
| `CommandSeamSplitBindingError` | Original missing-target,source-frame or strict-interior refusal |
| `request` | Borrow the exact original request |
| `span` | Borrow the actual original graph member,including copy/direction/ease/stops |
| `ranges` | Borrow lower then upper intervals for the named side in its unchanged source frame |
| `MissingSpan` | Requested correspondence is absent from this supplied graph |
| `SourceFrame` | Original cut edge differs from the actual named side's held edge |
| `Interior` | Same-frame cut is not strictly inside that side's original range |
| `RangeSplitError` | Original held range and cut,retained as the interior refusal's standard source |
| `paired_intervals` | Two declared interval pairs,ordered by A's lower then upper interval |
| `CommandSeamSplitPair` | Fixed original-parent/copy/range/direction projection,without a child span identity |
| `source_span` | Original parent correspondence identity carried by a projected pair |
| `side` | Original named physical-copy identity and its projected interval |
| `direction` | Original endpoint correspondence carried unchanged by each pair |

Lookups use the original target identity and authored graph order. One side cannot substitute its
anchor for the other. Missing target wins first; side A is checked completely before side B. On each
side, the edge frame is checked before the interval bounds. Both original parameters remain exact.
No nearest point, endpoint clipping, side swapping or implicit frame translation occurs.

For example, a supplied span can retain these partial ranges and unequal authored cuts:

| Side | Original range | Cut | Lower interval | Upper interval |
| --- | --- | --- | --- | --- |
| A | 1/7 through6/7 | 2/3 | 1/7 through2/3 | 2/3 through6/7 |
| B | 1/5 through4/5 | 3/7 | 1/5 through3/7 | 3/7 through4/5 |

Each array follows ascending source bounds. Consumers use the actual original seam direction when
establishing paired outputs. The binding retains the actual span's physical-copy identities, direction,
signed/symbolic ease, weighted regions and semantic stops through its original borrow. It does not
split,round or evaluate that content.

The following compiled API example accepts the caller's actual already-constructed graph:

```rust
use sc_core::{command::{CommandSeamSplitBinding, CommandSeamSplitBindingError, SplitSeamSpan},
    ontology::{SeamSideId, SewingGraph}};
fn inspect_existing<'a>(request: &'a SplitSeamSpan, graph: &'a SewingGraph)
    -> Result<CommandSeamSplitBinding<'a>, CommandSeamSplitBindingError> {
    let binding = request.bind_source_cuts(graph)?;
    let [lower, upper] = binding.ranges(SeamSideId::A);
    assert_eq!((lower.to(), upper.from()), (request.cut(SeamSideId::A).param,
        request.cut(SeamSideId::A).param));
    let [first, second] = binding.paired_intervals();
    assert_eq!(first.source_span(), second.source_span());
    assert_eq!(first.direction(), binding.span().definition().direction);
    Ok(binding)
}
```

## Follow the declared endpoint correspondence

The binding projects two fixed interval pairs in side-A lower/upper order:

| Original direction | First pair | Second pair |
| --- | --- | --- |
| Same | A lower with B lower | A upper with B upper |
| Opposite | A lower with B upper | A upper with B lower |

Each pair retains its own ascending source bounds,original physical-copy identities and original
endpoint direction. Reflection of a physical copy does not rewrite this declaration. Two copies of
one pattern may use the same source edge while remaining distinct physical sides. The pair's parent
identity is provenance metadata;creating new live span identities remains an execution obligation.

Given that binding,a caller can read its fixed projected metadata:

```rust,ignore
let [first, second] = binding.paired_intervals();
assert_eq!(first.source_span(), second.source_span());
let actual_a_copy = first.side(SeamSideId::A).copy;
let original_direction = first.direction();
```

These are interval projections. Actual physical correspondence,child ease,semantic stops,lineage
and current graph publication remain required stages before a paired effect.

## Preserve original refusals

A request naming an absent span returns its exact requested identity before inspecting cuts.
An A cut using B's edge returns SourceFrame for A, even if B also refuses. A valid-frame A endpoint
returns Interior for A before a wrong B edge is considered. With valid A, a B endpoint returns its
own original B interval and parameter. The original RangeSplitError is inspectable through the
standard error source chain; a source-frame mismatch invents no interval error.

## Snapshot evidence and execution boundaries

The binding borrows the exact request and actual held graph member. Fields are private; compiler
controls refuse direct construction and a lifetime escape beyond those original owners. Its default
Debug visits only fixed source metadata,without walking the span's ease/stop vectors.

Replacing a graph with another object of the same span identity requires a fresh binding. A prior
binding retains its original ranges and copy identity. Likewise, after a real topology deletion the
held source-frame binding remains inspectable while the graph's actual current-range validation
refuses. Callers must perform that current validation before any effect; source binding supplies no
cached grant,live registry decision or currentness certificate.

Native tests use actual Pieces,CutPlan,ledger,graph,weighted symbolic ease and a semantic notch. They
check original pointer identity, unequal cuts, exact partial intervals, A/B error precedence and the
original source chain, replacement snapshots and actual deleted-range evidence. Full paired behavior
and bus transaction acceptance remain separate work.

## Conserve the original ease before effects

A paired mutation must check coupled conservation against its original parent. Standalone structural
graph validation checks each supplied span's references and declarations and retains its explicit
`EaseValidation::DeferredToG2AndG3` marker. In the actual native counterexample,a10-micrometre parent
copied into two independently supplied child declarations produces20 total;both that graph and the
independently authored5+5 control pass structural validation. They are fixture inputs,not published
results from a paired handler.

The P1 conserved-ease task owns the parent relation,exact allocation/quantization and original
explicit/formula/profile sources before effects. Weighted and notch-bounded distributions retain
that same obligation;an unresolved source cannot be replaced with a fabricated child value. Semantic
stop ownership and retired-parent/new-child lineage remain separate owned partition stages.

## Uniform explicit cumulative allocation

| API token | Meaning |
| --- | --- |
| `uniform_explicit_ease` | Check the actual uniform explicit source and directed cut progress |
| `CommandUniformEasePartition` | Sealed original request/span borrow with conserved A-ordered amounts |
| `CommandUniformEaseError` | Original helper eligibility,progress,frame or quantity refusal |
| `OriginalSource` | Actual retained source refusal wrapped with the requested target identity |
| `original_source` | Borrow the retained original source;direct declarations return none |
| `parent_amount` | Actual requested span allocation,including original cumulative differences for a portion |
| `cut_progress` | Exact normalized original-root A cut progress |
| `first_amount` | First projected pair's cumulative quantized amount |
| `second_amount` | Target span allocation minus first amount |
| `cumulative_at_a` | Quantized original allocation at an original A-domain position |
| `amount_in_a` | Original cumulative upper amount minus original cumulative lower amount |
| `Weighted` | Actual distribution side requires its separately admitted weighted query |
| `BetweenNotches` | Actual side and original notch identities require their allocation context |
| `Amount` | Actual formula/profile source remains symbolic |
| `Progress` | Original side and boxed RangeProgressError,also the standard error source |
| `CutProgress` | Original ascending A/B fractions conflict with the declared direction |
| `IntervalSource` | Query interval names a different original A edge |
| `Numeric` | Original shared UnitError,also the standard error source |

The native helper reads the actual held span. For a direct declaration,it checks distribution
eligibility,then explicit
amount,then exact A/B normalized progress. Same direction requires equal ascending progress;
Opposite requires A progress to equal one minus ascending B progress. Unequal source parameters
can be valid on unequal partial domains. Stored edge parameters are fractions of their own edge
length by the reference contract;they are not polynomial curve parameters or measured geometry.

For a direct declaration,the helper quantizes its original signed amount × q using the shared
half-away rule. For a retained portion,it differences original-root cumulative boundary amounts.
The second A-ordered amount is the target allocation minus first. Negative,zero and positive differentials
retain their signs and exact quantum sum. Private fields and borrowed lifetimes prevent forging
this view or outliving the actual owners. Debug visits only fixed source identity/progress.

```rust
use sc_core::command::{CommandSeamSplitBinding, CommandUniformEaseError,
    CommandUniformEasePartition};
fn allocate<'a>(binding: &CommandSeamSplitBinding<'a>)
    -> Result<CommandUniformEasePartition<'a>, CommandUniformEaseError> {
    let allocation = binding.uniform_explicit_ease()?;
    assert_eq!(allocation.first_amount().checked_add(allocation.second_amount()),
        Ok(allocation.parent_amount()));
    Ok(allocation)
}
```

For further subinterval queries,retain that original source/domain and difference cumulative
quantized endpoint values. Repartitioning already rounded children is order-dependent:for1 µm,
the independent Fraction diagnostic gives[0,1,0] versus[1,0,0] for two split orders. Native queries
at the same original boundaries give[0,1,0] from either cut view,with negative symmetry. Partial
A[1/7,6/7] boundaries8/21 and13/21 exercise this without silently resetting the original domain.
These are original-root allocation queries;canonical nested child lineage/effects remain owned.

A query checks the original A edge,then lower and upper closed-domain endpoints. Wrong-frame and
outside-domain inputs are refused without clipping;normalization retains the original range/cut
and actual numeric cause through the standard error chain. A replacement graph of the same span
identity needs a fresh view;the old view keeps its original amount and domain.

Independent Fraction fixture rows watch signed,tie,zero,domain-limit and full-width fractions in
both directions. Native tests also cover pointer identity,directed mismatch,actual symbolic/weighted/
notch declarations,normalization overflow and original query refusals. Unsupported helper cases
are eligibility refusals,not invalid-graph diagnoses or fabricated missing-source values.

This stage conserves the original uniform explicit allocation only. Persistent original-source/
child provenance,symbolic and nonuniform integration,semantic stops,live lineage,current validation,
resource admission and atomic paired publication remain required. D301 stays open before effects.

## Retain the original source through repeated interval cuts

| API token | Meaning |
| --- | --- |
| `retain_ease_source` | Capture actual span ease/copies/domains/direction once into an owned source |
| `SeamEaseSource` | Private immutable original declaration and full original side domains |
| `SeamEasePortion` | Private current source intervals sharing that same original source |
| `SeamEasePortionError` | Actual named frame or strict-interior portion refusal |
| `source` | Borrow the unchanged original source from any portion |
| `declaration` | Borrow the original full amount/distribution,not an independent child amount |
| `split_at` | Partition actual portion intervals into A-ordered pairs using original direction |

This native prerequisite snapshots the actual structurally validated span's original correspondence
identity,physical-copy identities,full source ranges,direction and complete ease declaration.
Explicit signed amounts,formula identities,profile references,weighted regions and notch-boundary
identities remain exact. Capturing clones the declaration once,including weighted regions and original semantic stops. Callers
must admit that cost before using capture in a command. Cloning or splitting portions then shares
the same immutable source,without cloning those regions or creating independently authored amounts.

A retained portion can outlive the caller's graph. A later graph carrying the same span identity
needs a new capture;an old formula source does not silently become the replacement profile source.
Debug visits only fixed identity/copy/range metadata. Private source and portion fields prevent
callers from forging the provenance or replacing a source domain.

```rust
use sc_core::ontology::{SeamEasePortion, SeamSpan};
fn retain(actual: &SeamSpan) -> SeamEasePortion {
    let portion = actual.retain_ease_source();
    assert_eq!(portion.source().span(), actual.id());
    assert_eq!(portion.source().declaration(), &actual.definition().ease);
    portion
}
```

Each portion cut checks A completely before B and source frame before strict interior. Refusals
retain the actual portion interval,requested cut and original RangeSplitError source. Same pairs
lower A with lower B;Opposite pairs lower A with upper B. Both side ranges stay ascending in their
original frames,including reflected physical copies. Repeated cuts retain the original full domain.
For the partial domains above,cutting at normalized1/3 then2/3,or2/3 then1/3,produces the same three
source-interval portions;all still share the same original declaration and source allocation.

Portions describe source interval partitions. Their cuts may have unequal normalized progress;
this operation does not evaluate the distribution or establish physical correspondence. It supplies
no child quantity,span identity,stop partition,live lineage or installed graph change. Actual current
validation can still refuse after topology deletion while the retained source stays inspectable.
Canonical graph/current-source/resource integration is implemented below. Original-source quantity
queries for uniform explicit retained portions are implemented below;full paired effects remain
pending under D301.

## Canonical graph-owned ease sources

| API token | Meaning |
| --- | --- |
| `SeamEase` | Authored declaration or retained portion,without independent child amount/distribution |
| `Declared` | Direct original amount/distribution intent |
| `Portion` | Exact retained source intervals used by this consuming span |
| `declared_mut` | Edit a direct declaration in an editable input;retained sources return no mutable view |
| `portion` | Borrow retained allocation context alongside original amount metadata |
| `ease_sources` | Unique original source bodies in first-consuming-span order |
| `SeamEaseBindingIssue` | Actual side,direction or overlapping source-claim evidence |
| `Side` | Supplied side differs from its portion's exact original copy/domain |
| `Direction` | Supplied direction differs from the original source |
| `Overlap` | A previously admitted consumer claims overlapping original-domain material |
| `EasePortionBinding` | Consuming span/source identities plus the original boxed binding issue |
| `EaseSourceConflict` | Same original identity appears with conflicting source content |
| `EaseSource` | Retained original source ownership position or wrapped current source refusal |

An authored span now stores its original declaration with an explicit conversion:

```rust,ignore
let ease: SeamEase = DeclaredEase {
    amount: EaseAmount::Explicit(original_length),
    distribution: EaseDistribution::Uniform,
}.into();
```

A child stores its existing portion with the same conversion,then uses the portion's exact sides
and original direction. It has no separately editable child differential or distribution. Recapturing
an inherited span shares its existing root and portion;it never turns a rounded child total into a
new source. The private graph owns each original source identity once. Equal independent captures
are folded onto one actual shared body after complete validation;conflicting same-id content refuses.
This is canonical ownership,not a discount for coincidentally equal backing data across model snapshots.

Current validation checks the consuming span's side/copy/domain and direction against its portion,
then its actual ranges/stops. Original full source ranges/stops/distribution validate separately.
For example,a between-notches root can retain both original notch dependencies while each child's
stop list is a subset. Missing source landmarks or damaged original material remain visible,with
original source identity and concrete SewingError in the standard error chain. Missing-landmark
inventory includes retained source stops. An old root identity names owned provenance,not a live
replacement command target;binding that retired target still refuses MissingSpan.

Interval claims from one source must be disjoint on each side. Ordered exact interval admission
checks A then B;touching endpoints are legal,positive overlap refuses with both original intervals
and the prior consumer identity. Duplicate portions,predecessor/successor overlap and isolated B-side
overlap are watched natively. Partial coverage remains legal;this does not certify full paired
coverage,physical correspondence or a realized quantity sum.

The Design sewing ownership census visits inputs,graph,landmarks,live spans,then unique original
sources. A source cannot alias a parameter or another actual owner. Amount and Gather views retain
the exact portion mapping alongside the full original declaration. An explicit original1 µm is still
source metadata;it is not a reported child allocation. Formula declarations bind actual current
Length-kind parameters;logical profile references remain deferred as before.

Resource accounting visits spans/direct declarations/stops,then canonical source entries,weighted
regions and original stops in source order. Fixed physical/range/reference fields stay metadata.
A native fixture uses2 spans+1 source+2 regions+2 original stops=7 entries;limit6 refuses the original
source's stop charge with consumed5/requested2. Charging two logical graph snapshots costs14 even
when their immutable backing is shared. Exhaustive private owner/source/portion views retain field
coverage. Command capture/admission,quantity integration,stop/lineage effects and paired acceptance
remain their implementation owners;this change does not make SplitSeamSpan executable on the bus.

## Query quantities from the retained original root

| API token | Meaning |
| --- | --- |
| `uniform_explicit_cumulative_at_a` | Quantized full-source cumulative amount at an original A-domain position |
| `uniform_explicit_amount_in_a` | Difference original cumulative endpoints for an original-frame A interval |
| `uniform_explicit_amount` | Portion allocation after both directed endpoint correspondences are checked |
| `SeamEaseAmountError` | Actual original eligibility,progress,correspondence,frame or numeric refusal |
| `Correspondence` | Actual original A/B endpoints and ascending normalized fractions disagree |

The original retained source and each portion now support uniform explicit quantity queries. All
boundary values use the original literal and original full domain with the single shared rounding
kernel. Portion amounts difference rounded cumulative upper/lower boundaries. Both endpoints must
match the declared correspondence:Same compares ascending fractions;Opposite complements ascending
B and pairs A lower with B upper. Normalization checks A's endpoints completely,then B's,followed
by lower-A then upper-A correspondence. This proves the declared allocation relationship;it does
not measure physical seam geometry or refresh current-world authority.

```rust
use sc_core::ontology::{SeamEaseAmountError, SeamEasePortion};
use sc_units::Length;
fn amount(actual: &SeamEasePortion) -> Result<Length, SeamEaseAmountError> {
    actual.uniform_explicit_amount()
}
```

The existing command allocation view now accepts a retained target. Its parent amount denotes
that target portion's allocation;its optional original source retains the full original literal and
domains. Cut progress and cumulative/interval queries still use the original root A domain. The view
borrows the actual request,target span and original source;private fields and lifetime controls
continue to reject forgery or escape. Temporary interval partitions share immutable source data
and produce no new live identities or graph publication.

For example,an original1 µm spread over thirds yields[0,1,0]. Cutting the first third then the
remaining portion,or the first two thirds then their prefix,produces those same three amounts.
The intermediate target may have amount1,but splitting it never treats that rounded1 as a new
uniform source over its child domain. Native partial-domain portion trees and actual command views
watch both orders and negative symmetry. This is nested allocation evidence;the bus split handler
and its identity/stop/lineage effects remain pending.

The independent Fraction fixture also checks retained sources/portions in both directions,including
signed ties,zero,domain limits and full-width fractions. Original wrong-frame and closed-domain
refusals remain visible even for a zero amount. Complete normalization can genuinely exceed the
bounded rational representation;the original range/position and UnitError remain in its source chain.
Formula/profile amounts and weighted/notch-bounded distributions retain actual eligibility refusals.
No missing value is fabricated. A command wraps those native source/portion causes with the actual
target identity,without replacing their original source identity,parameters or concrete numeric cause.

## Scoped retained-source acceptance

A native acceptance case joins the actual Design owner,canonical sewing graph,amount declarations,
retained command view and graph resource meter for positive and negative original sources. Source
metadata can lend the same original literal to several target views;their portion queries determine
the allocations. The case checks initial[0,±1],then the remainder's[±1,0],without replacing the root.

Its graph-only budget admits two spans and one original source;the lower budget refuses the source
collection charge with its original location. Replacing a real physical-copy identity preserves the
held declaration/quantity evidence while complete current validation refuses the original missing
copy. Quantity availability therefore supplies no current-source or effect authority.

The retained-source component is implemented with focused native/compiler/publication checks.
Nonuniform/notch context,semantic stops,live lineage and paired bus effects retain their owners.
Broader paired acceptance follows implementation;no full CI,physical or release verdict is inferred.

Detailed paired execution tasks now live in the existing canonical execution sibling,
`docs/tasks/G1-SLICE-execution.md`. Original IDs,completion states and proof routes remain live;
the exact pre-move branch is retained with its native Git extent. Nonuniform/context work follows
this scoped retained-source component without expanding the root tracker beyond its bounds.

## Nonuniform arithmetic and context requirements

The admitted weighted-source queries below implement exact allocation. The independent diagnostic integrates the
original declared density with exact fractions. For regions with bounds lᵢ,uᵢ and positive
weights wᵢ, total mass is Σwᵢ(uᵢ−lᵢ); ascending cumulative mass at q is
Σwᵢ max(0,min(q,uᵢ)−lᵢ). Divide by the original total,including every region.
A gap contributes zero; it is not filled by a default density. Stored positions are fractions
of their edge's own length, so original partial domains must be normalized exactly first.

All allocations retain canonical A traversal order. For an A distribution,use its cumulative
fraction. For a B distribution with Same direction,use ascending B cumulative fraction;with
Opposite direction,use one minus that exact fraction. Apply this orientation before quantizing
the original signed source. For a one-micrometre source at a half boundary,the correct first
amount is1; subtracting a rounded B prefix from1 instead gives0. Negative sources obey the
same signed symmetry. Child allocations remain differences of original quantized boundaries.

Fixed128-bit rational intermediates cannot cover all valid declarations. Four quarter regions
with densities1/(9223372036854775807−k),k=2,4,6,8,produce a reduced cumulative denominator
of188 bits at the half boundary. The fraction is below one half: exact allocation of1 µm is0,
while binary64 gives one half and allocates1. The tracked independent diagnostic freezes160 density and50 original-domain
input/result rows and checks Fraction against a separate integer common-denominator construction:

```bash
python3 -I -B docs/tasks/artifacts/seam_partition/nonuniform_ease_oracle.py --check
```

A constructive arithmetic bound follows from the stored63-bit positive fields. Each region mass
has an unreduced numerator/denominator below2^189. For N regions,a common denominator D is below
2^(189N),and total numerator T is below N×2^(189N). A query denominator Q with b bits gives a
cumulative ratio P/(Q×T),with0≤P≤Q×T. Its fields need at most189N+ceil(log₂N)+b bits;
multiplication by the maximum source magnitude needs at most30 more. Direct normalized queries
have b≤63;an exact original-range normalization can need up to189 before reduction. This is a
feasible mathematical construction,not a bound on every temporary of an arbitrary implementation,
CPU time or allocator overhead. Implementation owns explicit checked workspace admission before
allocation and reuse of the one shared rounding policy. The existing bounded progress API keeps
its documented final representation refusals.

Physical pairing requires actual context. For illustration only,original lengths11000/10000 µm,
ease1000 µm and A densities1 then3 on the two halves allocate250 µm at A half. A matching B
boundary is21/40;equal half progress would create500 µm differential. These are arithmetic example
inputs,not measured geometry. A future paired handler must consume actual length/correspondence
evidence;weighted allocation cannot reuse the uniform equal-progress predicate as that proof.

Actual native fixtures admit notch stops authored on later split fragments:local half parameters
correspond to original quarter and three-quarter positions. Both named start/end orders remain
valid;locate the interval between the actual points while preserving both identities and stop order.
Current resolved ranges expose live intervals/direction,without per-portion original subdomains.
The context implementation must retain/recover exact original positions through actual journal
mappings and retain ambiguous/lost-anchor evidence. Profile notch dimensions do not locate stops.
The two focused native recovery tests validate real source admission,existing typed refusals and
later-frame anchors;they do not execute weighted quantities or certify physical correspondence.

## Admit original weighted workspace before allocation

| API token | Meaning |
| --- | --- |
| `WeightedEaseLimits` | Explicit positive region-count and conservative integer-width limits,no default |
| `NonZeroUsize` | Standard Rust positive count used for the region limit |
| `NonZeroU64` | Standard Rust positive integer used for the bit limit |
| `WeightedEaseAdmission` | Sealed borrowed original weighted explicit source and admitted reservation |
| `WeightedEaseWorkspaceError` | Actual eligibility or original-source resource refusal |
| `admit_weighted_explicit` | Check distribution,amount,complete region count,then derived integer width |
| `distribution_side` | Actual original side carrying normalized density |
| `region_count` | Complete original density population,including regions beyond a queried prefix |
| `integer_bit_bound` | Conservative reserved width,not a minimal necessary width |
| `integer_bits` | Caller-selected positive maximum integer width |
| `limits` | Actual selected limits retained by value |
| `Regions` | Complete original count exceeds its selected limit |
| `IntegerBits` | Conservative reservation exceeds its selected bit limit |
| `BoundOverflow` | Checked count-derived reservation cannot be represented |

Admission now exists for an actual retained original weighted explicit source. It borrows the
source and density;it does not clone regions or allocate arbitrary-precision integers. Private
fields and a source-bound lifetime prevent forged or escaping admission. Limits have no Default;
the caller selects both positive values. Debug visits fixed metadata and the region count only.
The source getter returns the actual original source;parent_amount remains its signed literal,
including for queries starting from a retained child's source. Admission itself computes no amount.

```rust
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::ontology::{SeamEaseSource, WeightedEaseLimits};
fn admit(actual: &SeamEaseSource) -> Result<(), Box<dyn std::error::Error>> {
    let limits = WeightedEaseLimits::new(
        NonZeroUsize::new(4).ok_or("positive count required")?,
        NonZeroU64::new(977).ok_or("positive bit limit required")?,
    );
    let admitted = actual.admit_weighted_explicit(limits)?;
    assert!(std::ptr::eq(admitted.source(), actual));
    let _reserved_integer_bits = admitted.integer_bit_bound();
    Ok(())
}
```

The conservative reservation is189N+ceil(log₂N)+219 bits:189 for complete original-domain query
normalization and30 for source multiplication supplement the common mass bound above. The source
multiplier width is derived from the actual Length domain constant. Four regions therefore reserve
977 bits even when their particular reduced result is much smaller. A977-bit limit accepts that
reservation;976 refuses it. This policy intentionally reserves by declared input bounds rather
than first performing expensive arithmetic to discover a smaller answer. Count/bit arithmetic is
checked before any integer workspace allocation. Zero source amounts still check the full source.

Eligibility precedes resource checks. Uniform intent reports its original source;notch intent
retains the actual side/start/end;symbolic weighted amounts retain the actual formula/profile
reference. Region-limit and bit-limit errors retain the original source identity,side,required
amount and selected limit. A child interval never truncates the original denominator population.
These are mathematical input/reservation checks. Allocator overhead,CPU time,whole-command
admission,current source validation and physical correspondence remain their own obligations.
The exact weighted query kernel below is implemented;command/context integration remains pending.
Focused native source2/bound1 and privacy/lifetime/no-default doctests3 pass;broader paired proof
and shared-rounding mutation campaigns remain deferred to their explicit owners.

## Query exact original weighted quantities

| API token | Meaning |
| --- | --- |
| `cumulative_at` | Original weighted cumulative amount at an actual original density-side EdgeAnchor,in canonical A order |
| `amount_in` | Original cumulative endpoint difference for a density-side interval |
| `WeightedEaseAmountError` | Actual source/side/frame/bounds or concrete numeric refusal |
| `SourceFrame` | Boundary uses a different edge from the original density side |
| `IntervalSource` | Interval uses a different original density-side edge |
| `Outside` | Boundary lies outside the closed complete original domain |
| `Numeric` | Original UnitError retained as the standard error source |

An admitted view now computes these quantities from the complete immutable original density and
signed literal. It normalizes source-frame positions exactly in a wide workspace,integrates the
piecewise positive density and divides by the original total mass. Missing regions contribute zero.
No bounded reference Rational or formula128-bit intermediate is inserted into this calculation.
For B/Opposite,it complements the exact fraction before invoking the shared rounding primitive.
These interval query amounts difference original rounded boundaries;no child mass is renormalized.

```rust
use sc_core::ontology::{EdgeAnchor, EdgeRange, WeightedEaseAdmission, WeightedEaseAmountError};
use sc_units::Length;
fn inspect(actual: WeightedEaseAdmission<'_>, at: EdgeAnchor, interval: EdgeRange)
    -> Result<(Length, Length), WeightedEaseAmountError> {
    Ok((actual.cumulative_at(at)?, actual.amount_in(interval)?))
}
```

For a declared1 µm source with density1 on the lower half and3 on the upper half,the three
ascending density-side thirds have cumulative fractions0,1/6,1/2,1. Canonical A-ordered interval
amounts are0,1,0 for an A distribution. For a B/Opposite distribution,the A traversal reverses those
intervals and exact complemented boundaries give1,0,0. A−1 µm source produces their signed mirrors.
Each partition sums to its original source quantum. These are declared allocation examples;
matching physical A/B cuts still requires their actual length/correspondence context.

Frame and closed-domain checks precede integer allocation. Interval queries check the original
edge and both endpoints before computing either cumulative value. Zero ease retains those same
refusals. Errors name the original source and density side;frame/interval/bounds errors preserve
the actual supplied data,while numeric errors forward their concrete UnitError source.

The implementation makes two region passes without a cloned density or retained integer array.
It builds a common denominator,then original total/prefix integer masses. Positive term bounds
follow the admitted construction above;query normalization needs at most189 bits,and source
multiplication adds at most30. Explicit reference/Length field types make a future width change
require re-deriving admission at compile time. This reservation bounds numeric operands/results;
allocator internals and whole-command runtime budgets remain separate.

The independent fixture now has50 original-domain rows,including two valid189-bit progress
fractions on opposite sides of one half that binary64 collapses to one half. The unchanged bounded
reference-progress API refuses these wider results;the admitted weighted query returns the exact
quantity. Native tests consume all160 density rows and50 domain rows,including188-bit cumulative
mass,189-bit normalization,positive/negative/zero/maximal sources,omitted regions and signed ties.
Focused weighted6/oracle1/source docs4 and selected Clippy/fmt pass. Semantic-notch context,actual
paired matching,command effects and full paired acceptance retain their later owners.

### Focused weighted-source component acceptance

An actual owned Design fixture joins original weighted metadata,retained intervals,explicit numeric
admission and source queries for±1 µm. At the half cut,weighted quantities are0 and±1;the actual
Uniform control gives±1 at that first boundary,so a uniform fallback cannot satisfy the check.
Both targets retain the same canonical original source and its two-region density. Graph-only
resource cost is5 entries:2 spans+1 source+2 regions;limit4 refuses that original region charge
with consumed3/requested2. Replacing copy96000 with96010 preserves held quantities,but complete
current validation and fresh amount declarations refuse the original MissingCopy on span97001/A.
Focused actual Design1 and selected lint/format checks pass. The weighted source component closes
with focused verification. Original-notch contexts,paired physical matching,command effects and
broader paired/mutation acceptance remain owned;these source partitions are descriptive intervals.

[Original-frame notch context](command-notch-context.md) records actual late-frame,ambiguous-origin
and derived-work counterexamples plus the immediate D305 reference-recomposition repair prerequisite.
Those requirements precede notch allocation and paired effects;weighted source acceptance above
retains its focused scope.
