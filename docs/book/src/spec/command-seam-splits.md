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
    Ok(binding)
}
```

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
