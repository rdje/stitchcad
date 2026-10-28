# Quantities for actual captured seam commands

Weighted and original-notch command quantity views consume an [admitted actual capture](command-seam-capture.md).
They borrow the real request,target span and complete immutable original source,then admit its
separate original numeric workspace. Notch quantities also borrow the actual sealed original context. This is a mathematical allocation view before current-world,physical
correspondence,authority and paired effects. The SplitSeamSpan bus handler is still unimplemented.

| API token | Meaning |
| --- | --- |
| `CommandEaseCapture` | Actual admitted materialized source,target intervals and request provenance |
| `CommandWeightedEasePartition` | Borrowed actual weighted source and original target/pair quantities |
| `CommandWeightedEaseError` | Consuming target and original weighted workspace/query refusal |
| `CommandNotchEasePartition` | Borrowed actual capture/context and original notch target/pair quantities |
| `CommandNotchEaseError` | Actual context-source mismatch or original notch workspace/query refusal |
| `notch_explicit_ease` | Bind exact captured source/context and admit original notch target/pair quantities |
| `ContextSource` | Supplied context does not borrow the exact captured original source |
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
[Actual Design source admission](command-seam-capture.md#join-the-actual-owned-design) joins complete
model/current preflight to capture;whole paired acceptance retains its subsequent owner.

## Borrow the actual original-notch context

Construct an [original-notch context](command-notch-sources.md) from this capture's `source()`:
first `admit_notch_context(plan,pieces,full_marks,ledger,context_limits)`,then `validate_sources()`,
then `original_context()`. Supply the real complete inputs and explicit limits. Each staged error
remains its original typed error,including full ambiguous or coincident original projections,
actual candidate/numeric quotas and source/ledger frames. A refused context supplies no successful
view. The command helper accepts only the sealed successful context;it never chooses an origin.

Keep both the capture and the resulting context alive outside the view. Call:

```rust
use sc_core::command::{CommandEaseCapture, CommandNotchEasePartition, CommandNotchEaseError};
use sc_core::ontology::{OriginalNotchContext, NotchEaseLimits};

fn notch_quantities<'a>(capture: &'a CommandEaseCapture<'a>,
    context: &'a OriginalNotchContext<'a>, limits: NotchEaseLimits)
    -> Result<CommandNotchEasePartition<'a>, CommandNotchEaseError<'a>>
{
    capture.notch_explicit_ease(context, limits)
}
```

The helper first requires exact pointer identity between the captured full source and the source
borrowed by the context. Equal IDs and even equal metadata bodies are insufficient:two separately
materialized direct captures own different source objects. Build the context from the actual
consuming capture. Retained captures that share the same canonical original root can share its
context. `ContextSource` retains the actual expected source and supplied context for inspection,
plus the consuming target ID. It precedes numeric admission,even with a tiny quantity allowance.

After that match,the helper uses existing `NotchEaseLimits` and `admit_explicit`,preserving actual
symbolic amount before quantity width refusal. The earlier projection/comparison budget does not
license quantity arithmetic. `notch_admission()` retains the selected allowance,reserved width and
actual context. `original_context()` lends the same externally owned object with real named
Notches,full original projections and immutable ledger evidence. No projection or root is cloned;
no self-reference,dependency integer type or new arithmetic kernel is introduced.

### Original notch quantities and retained targets

The same target/first/second accessors and A pair order apply. Each amount differences existing
original density-side rounded cumulative boundaries. The original progress clamps to the exact
original lower/upper Notches;B/Opposite complements it before shared signed half-away rounding.
For original Notches at1/4 and3/4,a1 µm source cut at the half tie gives first1 and second0 for
both sides and directions. The negative source gives−1 and0;zero retains all source/frame rules.

A retained lower A half from that original1 µm source has target1. Cut it again at original A3/8:
original notch progress is1/4,so its two allocations are0 and1. An independently declared rounded
child Uniform1 on[0,1/2] would normalize3/8 to3/4 and incorrectly spend1 first. The actual view
keeps original301,target302 and both original Notches even when child stop lists are empty.
The retained upper target303 can simultaneously have target0 while its full original source is1;
its context can be shared only because both captures borrow that exact canonical root.

Original coordinates remain wide. A real later split-fragment Notch at1/MAX_I64 can project to
original1/(2*MAX_I64),which has no stored `Param` view. With another original Notch at1/4,the
captured command at original1/8 still computes exact first0/second1 without narrowing. For that
two-entry journal,the existing separate reservation is727 bits;726 refuses with original source,
actual consuming target,component bound317 and selected allowance. The simple one-entry context
reserves473 bits,so472 refuses even when the actual numbers are small.

Auxiliary `cumulative_at` and `amount_in` retain the full original domain,including outside an
inherited target. Wrong-edge/bounds/numeric errors preserve original causes,actual consuming
target and the shared boundary/interval stage. Workspace and query causes remain in the standard
error chain. Matching this source/context proves a pure immutable mathematical query;the context's
local source validation does not become a fresh whole Design,current cut,authority or effect proof.

Focused actual direct/retained command cases cover twelve side/direction/sign combinations,
original child rounding,source sharing and mismatch,symbolic precedence,standard errors,separate
width refusal,valid wide originals and unchanged upstream ambiguous/coincident evidence. Complete
Design source admission now joins these views;physical matching/paired effects and their real
formula/profile/geometry dependencies remain separately owned.
