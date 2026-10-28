# Admit actual command source capture

The actual seam-cut binding now admits complete original source capture before materialization.
It preserves the original request and actual graph member. This is a metadata stage before
weighted/notch command quantity views;current source validation,physical matching,authority and
paired effects retain their own stages. The SplitSeamSpan bus handler remains unimplemented.

| API token | Meaning |
| --- | --- |
| `CommandSeamSplitBinding` | Actual held request/span and fixed projected intervals |
| `CommandEaseCaptureLimits` | Explicit positive per-capture logical item allowance |
| `CommandEaseCaptureAdmission` | Borrowed actual binding and complete admitted source cost |
| `CommandEaseCaptureCollection` | Actual full original variable collection being counted |
| `CommandEaseCaptureError` | Concrete target/source/collection/count/limit refusal |
| `CommandEaseCapture` | Materialized original source and actual target intervals/provenance |
| `admit_ease_capture` | Check complete logical capture cost without cloning collections |
| `capture` | Materialize one previously admitted source snapshot |
| `SeamEaseSource` | Full immutable original domains,declaration,direction and stops |
| `SeamEasePortion` | Actual consuming target's original source intervals |
| `WeightedRegions` | Complete original density-region population |
| `OriginalStops` | Complete original semantic-stop population |
| `Items` | Concrete selected population allowance refusal |
| `CountOverflow` | Checked aggregate count cannot be represented |

## Count before materialization

Supply `CommandEaseCaptureLimits::new(items)` with a positive `NonZeroUsize`;there is no default.
The complete per-capture logical cost is **one original root + all original weighted regions +
all original stops**. Count in that order with checked aggregate additions. Source fields remain
borrowed;admission builds no collection,index,topology choice tree or numeric workspace.

For example,a source with two weighted regions and two semantic stops costs5 items. Limit2
refuses the region collection with consumed1/requested2;limit4 refuses original stops with
consumed3/requested2. Limit5 admits the complete source. A Uniform source with no stops costs1,
including when its amount is a logical profile reference. An eligible metadata capture does not
supply that reference's numeric value.

Errors preserve both the actual requested target and full original source identity. A retained
child302 can refer to source301;the failure keeps both. Its empty child stop list cannot erase the
two original stops,so the same retained source still costs5. Shared immutable backing grants no
logical capture discount. These are population bounds;allocator overhead,byte/CPU totals and
aggregate command/snapshot budgets remain separate. Reusing a Copy admission permits another
capture under the same per-capture bound;each still needs its aggregate operation accounting.

## Materialize the admitted original source

```rust
use sc_core::command::{CommandEaseCapture, CommandEaseCaptureError,
    CommandEaseCaptureLimits, CommandSeamSplitBinding};

fn retain<'a>(actual: &CommandSeamSplitBinding<'a>, limits: CommandEaseCaptureLimits)
    -> Result<CommandEaseCapture<'a>, CommandEaseCaptureError>
{
    let admitted = actual.admit_ease_capture(limits)?;
    Ok(admitted.capture())
}
```

For a declared source,`capture()` owns a clone of its complete declaration and stops once. For a
retained target,it shares the actual canonical original source without copying a rounded child
declaration. The captured portion preserves the consuming target's intervals,which can be smaller
than the full original domains. `binding()` retains the original request and actual target borrows;
`source()` and `portion()` borrow the materialized capture. They cannot escape its lifetime.
Private fields prevent fabricating either successful admission or capture.

Literal,formula and profile amounts remain exact metadata. Capturing a held graph after an edge
deletion preserves that damaged held reference for inspection;it does not refresh currentness or
select a repair. No quantity,semantic-notch context,physical counterpart or prepared effect follows.
The existing [original-source queries](command-seam-splits.md#query-quantities-from-the-retained-original-root)
and [actual notch context](command-notch-sources.md) remain separately admitted operations.

Exhaustive private source/portion/definition views and fixed-field Copy guards cover all captured
populations. A future owning field must be reviewed before it can be hidden behind this admission.
Focused native cases exercise direct signed/symbolic sources,complete quotas,retained root sharing
with empty child lists,and damaged held-source metadata. No-default,privacy and lifetime doctests
guard the public boundary. Weighted/notch command allocation views and whole paired acceptance
retain their subsequent implementation owners.
