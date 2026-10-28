# Quantities for actual captured seam commands

Weighted command quantity views now consume an [admitted actual capture](command-seam-capture.md).
They borrow the real request,target span and complete immutable original source,then admit its
weighted integer workspace. This is a mathematical allocation view before current-world,physical
correspondence,authority and paired effects. The SplitSeamSpan bus handler is still unimplemented.

| API token | Meaning |
| --- | --- |
| `CommandEaseCapture` | Actual admitted materialized source,target intervals and request provenance |
| `CommandWeightedEasePartition` | Borrowed actual weighted source and original target/pair quantities |
| `CommandWeightedEaseError` | Consuming target and original workspace/query refusal |
| `CommandEaseQuantityStage` | Actual failed target,pair,boundary or interval stage |
| `weighted_explicit_ease` | Admit and query complete original weighted target/pair quantities |
| `WeightedEaseLimits` | Explicit original region and integer-width allowances |
| `WeightedEaseAdmission` | Original full density,literal and numeric reservation evidence |
| `WeightedEaseWorkspaceError` | Original intent,value,population or integer reservation refusal |
| `WeightedEaseAmountError` | Actual original edge,closed bounds or numeric query refusal |
| `Workspace` | Original workspace cause plus the consuming target identity |
| `Quantity` | Original query cause plus consuming target and actual stage |
| `Target` | Allocation of the whole requested target interval |
| `FirstPair` | First projected pair in canonical A order |
| `SecondPair` | Second projected pair in canonical A order |
| `Boundary` | Explicit full original-source cumulative query |
| `Interval` | Explicit full original-source interval query |

## Admit the full original numeric source

Call `capture.weighted_explicit_ease(limits)` after source capture. Existing `WeightedEaseLimits`
have explicit positive region/integer budgets and no default. Admission keeps complete original
density and full signed literal;it does not truncate to the requested target. Intent/amount/count/
bit failures retain their existing precedence. Formula/profile sources supply no numeric value.
Source capture and numeric workspace admission are separate:counted metadata is not an integer
workspace allowance or a fresh whole-command resource/authority proof.

```rust
use sc_core::command::{CommandEaseCapture, CommandWeightedEasePartition, CommandWeightedEaseError};
use sc_core::ontology::WeightedEaseLimits;

fn allocate<'a>(actual: &'a CommandEaseCapture<'a>, limits: WeightedEaseLimits)
    -> Result<CommandWeightedEasePartition<'a>, CommandWeightedEaseError>
{
    actual.weighted_explicit_ease(limits)
}
```

The view retains the exact capture/request/span/source borrows. `weighted_admission()` exposes
the original limit/source evidence without a cloned density. Its private fields and lifetime
checks prevent fabricated amounts or escape from the actual source owner. No dependency integer
type,coordinate narrowing,cloned root or duplicated rounding rule is introduced.

## Target and projected pair amounts

`source_parent_amount()` is the full original literal. `parent_amount()` is the actual requested
target's allocation. They can differ:an inherited first half can have target0 while its original
source is1 µm. `first_amount()` and `second_amount()` each difference the original source's rounded
cumulative boundaries for the corresponding projected pair. Pair order follows lower/upper A;
Opposite direction pairs those with upper/lower B. The density side is the actual original declared
side,independent of physical-copy reflection. The target and two allocations conserve the original
quantum relationship without treating a rounded child total as a new source.

For a1 µm source with density1 on the first half and3 on the second half,an A distribution at A
half allocates0 then1. With a B distribution at B2/3,the exact density cumulative fraction is1/2.
The first A-ordered pair receives1 for either Same or Opposite direction. For B/Opposite,the
source query complements the exact fraction before shared half-away rounding;subtracting an
already rounded B prefix from1 would incorrectly give0. Negative sources produce signed mirrors;
zero sources retain the same original frame and admission rules.

The two requested cuts remain descriptive inputs. These examples deliberately do not apply the
Uniform helper's equal-normalized-progress predicate as a weighted physical proof. Actual matching
A/B cuts still require their real length/correspondence context before effects.

### Retained rounding counterexample

With the same original1 µm density,an inherited upper A half has target allocation1. Cutting it
again at original A2/3 yields1 then0:the original cumulative at that cut is exactly the half tie.
An independent declaration of the rounded child1 on its own half-domain would normalize that cut
to a local third and allocate0 first. The actual command view returns1 from the original source;
its full density and original identity remain intact. Empty child stop lists cannot replace that
root's complete metadata or original numeric denominator.

## Query failures and original frame

`cumulative_at(at)` and `amount_in(interval)` query the full original density-side frame/domain,
including positions outside an inherited target. They delegate original edge/closed-bound checks
and exact arithmetic to the admitted source. Query errors preserve the actual target identity,
the concrete stage and original source/frame/bounds/numeric cause in the standard error chain.
For example,a request for child302 can fail a region/bit limit on original301;both identities stay
visible. A wrong-edge auxiliary query retains its actual boundary or interval and stage.

A valid source normalization can need189 bits even though all stored input fields fit `Param`.
The command view does not construct a bounded cut-progress fraction or use binary64. The actual
wide-domain fixture succeeds through original weighted arithmetic while the bounded progress API
keeps its genuine representation refusal. Existing conservative numeric limits bound operands and
results,not allocator internals,CPU or aggregate command costs.

Focused actual direct/retained command tests cover A/B,Same/Opposite,signed/zero values,original
rounding,full-source population/bit refusals,symbolic values,standard causes and wide normalization.
Notch-context command views,actual higher caller/dependency integration and whole paired acceptance
retain their subsequent implementation owners.
