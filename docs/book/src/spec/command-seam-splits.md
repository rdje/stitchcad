# Paired seam split source binding

The sc-core SplitSeamSpan input can now bind its original source cuts to the actual held SewingGraph.
This is an implemented native prerequisite for the paired handler. Current copy/range resolution,
physical pairing, ease/stop conservation, lineage, resource admission and atomic effects retain their
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

For nested splits, the planned allocation stays tied to the original source and its domains. It
quantizes cumulative original allocation at each retained boundary and differences those endpoint
values for a child. This preserves the parent quantum sum without repeatedly rounding child totals.
An independent Fraction planning prototype for a 1 µm parent gives rounded-child results
[0,1,0] versus [1,0,0] for two split orders; the same original boundaries give [0,1,0]. The negative
parent preserves signed symmetry. Native cumulative allocation and its regression tests remain
the next owned P1 stage. These are planning prototype outputs.
