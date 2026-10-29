# Admit actual command source capture

The actual seam-cut binding admits complete original source capture before materialization.
An actual owned Design entry also admits complete model resources and current sewing sources first.
It preserves the original request and actual graph member. This is a metadata stage before
weighted/notch command quantity views;current source validation,physical matching,authority and
paired effects retain their own stages. The SplitSeamSpan bus handler remains unimplemented.

| API token | Meaning |
| --- | --- |
| `CommandSeamSplitBinding` | Actual held request/span and fixed projected intervals |
| `CommandSeamSplitSourceAdmission` | Actual owned sewing state,current declaration and capture admission |
| `CommandSeamSplitSourceError` | Original complete model/current/binding/capture refusal |
| `admit_sewing_source` | Admit complete actual DesignSewing sources before capture |
| `DesignResourceLimits` | Explicit complete model entry,text and counter-bit allowances |
| `DesignResourceDemand` | Actual model walk demand before semantic preflight |
| `DesignResourceSource` | Actual higher owner readonly exhaustive accounting contract |
| `charge_design_sewing_resources` | Walk complete represented inputs,graph and landmarks under one meter |
| `Resources` | Original complete model envelope refusal |
| `Current` | Original complete current ownership/source/amount metadata refusal |
| `Binding` | Actual target or original cut frame/interior refusal |
| `Capture` | Original complete metadata capture population refusal |
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
guard the public boundary. [Weighted and original-notch command allocation views](command-seam-quantities.md) now
consume this actual source;actual caller/current/dependency integration and whole paired acceptance remain owned.

## Join the actual owned Design

For the complete owned sewing component,call the real request's
`admit_sewing_source(&state,model_limits,capture_limits)`. The generic source owner must implement
both existing `DesignReferenceHost` and `DesignResourceSource`. The latter's actual exhaustive
readonly accounting contract supplies the source-host walk;there are no caller-supplied totals or
default budgets. The method composes `charge_design_sewing_resources` with a fresh explicit meter
before semantic preflight. It visits the entire represented input root,original graph and all
landmarks,including unused data and retained full source metadata.

```rust
use sc_core::command::{SplitSeamSpan, CommandEaseCaptureLimits,
    CommandSeamSplitSourceAdmission, CommandSeamSplitSourceError};
use sc_core::design::{DesignSewing, DesignReferenceHost, DesignResourceSource, DesignResourceLimits};

fn admit<'a,H: DesignReferenceHost + DesignResourceSource>(request: &'a SplitSeamSpan,
    state: &'a DesignSewing<H>, model: DesignResourceLimits, capture: CommandEaseCaptureLimits)
    -> Result<CommandSeamSplitSourceAdmission<'a,H>, CommandSeamSplitSourceError<H::Error>>
{
    request.admit_sewing_source(state, model, capture)
}
```

The exact order is complete model envelope,complete current source preflight,target lookup,cut
binding,then complete original capture admission. Current preflight uses the existing owned
`amount_declarations()` path:actual owned identities/current root,every semantic landmark,original
graph copies/ranges/stops and actual amount metadata. Unused damage and late declarations cannot
be hidden by selecting a healthy target. A missing target is considered after complete current
sources. Actual A then B frame/interior rules remain unchanged;capture cannot erase those causes.

`CommandSeamSplitSourceError` retains the original concrete error and standard cause at each
stage. For example,an otherwise valid retained Notch source with two original stops needs three
capture items even when its child stop list is empty. Capture limit2 keeps actual target97001,
original97000,OriginalStops,consumed1/requested2/limit2. A wrong A edge refuses before that capture
limit. A complete model entry limit one below the actual demand refuses its final two-landmark
charge before target lookup. Replacing copy96000 with96010 refuses the held graph's original
MissingCopy before target/capture,while its raw held source binding remains inspectable.

### Materialize and calculate quantities

The returned private token borrows the exact `state()`,selected `amount_declaration()` and real
`binding()`. The declaration borrows the same canonical input owner and actual target span.
`model_demand()` and `model_limits()` retain the exact completed walk and chosen envelope. The
token contains no cloned population. Its `capture_admission()` is the already admitted original
source token;call `.capture()` on that token to materialize it. This remains an explicit separate
step,so a source refusal produces no owned capture. Keep that capture alive externally for its
[weighted or original-notch quantity view](command-seam-quantities.md).

Actual cross-crate fixtures join this entry to original weighted quantities on A and B for signed
1 µm sources. The half A cut gives first0;the B2/3 half-density tie gives first±1 after Opposite
complement before rounding. Notch fixtures join the real owned source,canonical retained root,
actual semantic registry/ledger,original context and separate473-bit admission. The retained target
at A3/8 and B5/8 gets first0/second±1 for both sides,while retaining its full original source and
two Notches. The original Design state remains exactly equal after these pure queries.

Actual Formula5001 still lends its existing Length-kind parameter/declaration over canonical inputs.
Actual Profile98010 remains `DeferredToG4`. Both pass current metadata admission,then numeric
notch admission refuses the actual symbolic amount before a1-bit quantity allowance. No known,
unknown or derived input is replaced with a convenient scalar.

### Dependencies before effects

This source token covers the represented immutable sewing component. It grants no command
identity/revision authority,whole garment validation,physical cut matching or prepared effects.
Model logical populations bound the data being validated;they do not certify allocator bytes,
CPU,aggregate runtime snapshots or repeated capture totals. Numeric region/integer allowances
and notch input/projection/candidate allowances remain separate explicit stages.

| Required capability before consuming paired effects | Existing task owner |
| --- | --- |
| Actual declaration-order formula execution and dependency/replay validation | G1-SLICE.5e.2 and G1-SLICE.5e.4 |
| Typed selected Profile values,state/evidence and bounded resolution | G4-PROFILES.1, G4-PROFILES.1.resource-admission and G4-PROFILES.3 |
| Real edge geometry/lengths and physical construction correspondence | G2-2D.1, G1-SLICE.5f.3b and G3-GRADING |
| Semantic stop partition and new span lineage | G1-SLICE.6.5.3.2.3.3 and G1-SLICE.6.5.3.2.3.4 |
| Fresh authority/revision,aggregate operation admission and atomic paired effects | G1-SLICE.6.5.3.2 |

These owners must supply real published interfaces and evidence before effects can consume them.
A formula/profile metadata reference is not a numeric provider,and equal normalized cut positions
are not a physical correspondence check. The SplitSeamSpan bus handler remains unimplemented;
focused source/quantity checks do not close its later combined acceptance.

## Captured caller component checkpoint

After all four capture/weighted/notch/actual-Design caller slices are implemented,the scoped
checkpoint passes13 native seam-command cases,4 actual Design caller cases and20 public seam API
doctests. Existing independent weighted160+50 rows and notch6 bound/302 Fraction-width/380 exact
query controls pass. This closes the captured caller component with scoped checks;no product
kernel is changed by the checkpoint. The enclosing nonuniform component checkpoint also passes
its scoped source/context/query checks. Bounded stop rows/views are implemented;sealed capture join and live lineage follow.
The required full pre-push gate and whole paired stop/lineage/atomic/mutation/physical acceptance
retain their own later owners. Current source/quantity success grants no bus or release approval.
