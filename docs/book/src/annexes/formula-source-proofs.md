# Formula source kind proofs

> **Status:** complete initial-scope expression proofs,actual canonical dependency joins and Derived
> consumer-kind checks are implemented with focused verification. Dependency/cycle admission,
> actual ordered-let binding,full resource/currentness and execution integration remain pending.

Start with the [original source owners and normalized collections](formula-recipe-inputs.md#explicit-formula-source-owners).
FormulaNormalizedSources.check_sources(&namespace, limits) checks the complete supplied source
inventory through the existing expression kind factory. Every source,including an unused one,
receives its original proof. All branches and repeated name occurrences participate. The supplied
FormulaNamespace is the existing immutable initial scope;its constructor rejects prior-let seeds.
This API supplies no fictitious statement/header/name or new binding/rounding boundary.

## Complete admission and validation order

Let S be the source population,N the complete normalized node count and T the complete actual
canonical Derived consumer count. FormulaSourceCheckLimits selects independent S/N/T allowances
and a conservative S+2N retained-proof allowance. The latter covers S source proof records,N node
certificates and at most N name dependencies. It is not an exact live-record or allocator-byte
measurement. No default limits exist;an empty collection/consumer scope fits zero allowances.

Before any outer proof allocation or expression check,the method checks source quota,checked
population arithmetic,node quota,target quota and retained quota. Consumer count includes unused
targets and separate targets sharing one source. Quota failures retain complete actual populations
and original selected limits. Only then is one outer proof vector fallibly reserved.

Validation has three complete phases:

1. Every original expression passes the existing initial-namespace checker in source order.
2. Every actual canonical name dependency joins the actual canonical inventory in source/dependency
   order,including repeated and untaken occurrences. Dedicated Length requires its actual owner;
   general scalar metadata requires its actual identity/kind. Reserved/geometry metadata retains
   its existing kind/source role without consulting a value provider.
3. Every original canonical Derived consumer requires its actual source root kind,in canonical
   dedicated-Length/general-scalar order. Source lookup reuses the original private id index;
   no copied/sorted lookup table or target vector is allocated.

All expression errors precede canonical dependency errors;both precede target-kind mismatch. A
late unused source error therefore refuses the whole collection even if an earlier source has a
consumer-kind mismatch. Any failure drops the private prefix and returns no accepted proof vector.

Existing normalized arenas,namespace creation,per-expression checker scratch,allocator capacity/
bytes,CPU and the full graph/runtime envelope are separate. These retained quotas grant no global
resource or execution certificate. Original source-byte and per-expression syntax limits remain.

## APIs and original witnesses

| Public API | Meaning |
| --- | --- |
| `FormulaCheckedSources` | Complete initial-scope source proofs and actual consumer-kind checks |
| `FormulaCheckedSource` | Actual normalized source plus its existing expression proof |
| `FormulaSourceCheckLimits` | Explicit S/N/T and conservative retained-proof allowances |
| `FormulaSourceCheckError` | Actual normalized/source/input context,selected limits and refusal |
| `FormulaSourceCheckRefusal` | Complete quota,allocation,original expression/canonical or target-kind cause |

sources() borrows original source order;source(id) returns the actual same source proof through
the original private source index. No label,canonical alias or neighboring id supplies a fallback.
Each normalized()/proof() pair pins the original source,arena,local spans and actual factory proof.
The collection exposes normalized(),limits(),target_count() and retained_records() without a
second state or consumer cache. Debug omits source text and authored values.

Expression preserves the original source index and FormulaExpressionCheckError,including its
actual owner/span and canonical language token. CanonicalInput preserves the original source
index,name-use span and FormulaCanonicalInputError with actual metadata/inventory. Semantic
errors are boxed to keep the public error compact. TargetKind keeps the actual Derived target,
original source index,required canonical kind and actual inferred root kind. It is an integration
refusal;it does not manufacture a let-header formula_dimension diagnostic or statement ordinal.

Borrowed semantic causes remain available through refusal();they cannot be erased into the
standard Error::source static return type. Allocation retains its actual TryReserveError inline
and through that chain,without allocating another diagnostic box. Handle local borrowed failures
while their actual normalized/source/canonical owners exist. No source or state is copied to make
an error outlive its context.

## Check an actual Derived consumer

```rust
use sc_core::{ontology::EntityId, recipe::*, value::*};
let formula = EntityId::from_bits(9001);
let expression = FormulaExpressionDocument::new("1 mm".into())?;
let nodes = expression.node_count();
let source_limits = FormulaSourceLimits::new(1, expression.source().len(),
    expression.canonical().as_str().len());
let sources = FormulaSources::new(vec![FormulaSource::new(formula, expression)], source_limits)?;
let input = LengthDeclaration::new(LengthDeclarationDefinition {
    id: EntityId::from_bits(10), source: EntityId::from_bits(20),
    state: LengthState::Derived { formula },
})?;
let canonical = CanonicalInputs::new(vec![input], vec![])?;
let inputs = FormulaSourceInputs::new(&canonical, &sources).expect("actual owners");
let normalized = inputs.normalize_sources(FormulaSourceNormalizationLimits::new(1,nodes,1+2*nodes))
    .expect("complete normalization allowance");
let namespace = FormulaNamespace::new([]).expect("initial scope");
let checked = normalized.check_sources(&namespace, FormulaSourceCheckLimits::new(1,nodes,1,1+2*nodes))
    .expect("complete actual source and consumer kinds");
assert_eq!(checked.source(formula).unwrap().proof().kind(), FormulaKind::Length);
assert_eq!(checked.target_count(), 1);
assert!(canonical.length(EntityId::from_bits(10)).unwrap().authored_value().is_err());
# Ok::<(), Box<dyn std::error::Error>>(())
```

The original declaration still requires evaluation. A kind proof is no numeric value,storage
conversion,provenance propagation or release authority. Existing quantity Formula(id) still
selects DesignParameter metadata;formula9001 remains the distinct source entity in this example.

These source/consumer pairs illustrate the supported kind check,without computing their values:

| Original source | Actual consumer kind | Static source kind |
| --- | --- | --- |
| 1 mm | dedicated or general Length | Length |
| 30 deg | Angle | Angle |
| (1 mm)^2 | Area | Area |
| 0.5 | Ratio | Ratio |
| 2 | Count | Count |
| 1==1 | Boolean | Boolean |

If Length target10 and Count target11 share source9001 containing1 mm,the first kind fits and the
second returns TargetKind with actual scalar position/id11,expected Count,actual Length and the
same actual source. The method never changes the target annotation or creates another source.

An actual Unknown width can pass source-kind checking;its value remains unobserved. An equal
external copy cannot impersonate its inventory owner,even in an untaken branch. eps_phys has a
known Length kind without a supplied physical tolerance. Missing optional providers remain a
later lazy execution concern. Both the namespace map and its allocation may drop after checking;
actual borrowed name/declaration owners must survive. Proof node views cannot outlive their proof.

## Verification and remaining work

Six focused native contracts cover all six scalar kinds and dedicated Length,reversed source-id
order with exact proof/arena pointers,actual Unknown/reserved metadata after dropping the namespace
map,late unused branch errors,untaken equal-owner refusal,shared-source consumer mismatch,separate
quota precedence and empty drafts. Four source-registry contracts retain original index/buffer/
identity behavior,including the independent256-sequence duplicate scan. Compiler guards reject
default limits,private collection mutation and escaped proof nodes. No full verification is claimed.

This certifies sources under the supplied initial namespace,not completeness/currentness of a
containing Design/host namespace. Geometry metadata supplies no current operation/provider proof.
Prior lets cannot be caller-seeded into this initial scope. The actual ordered-statement/source
relationship,binding rounding,complete dependency graph/cycles,load/replay,providers,atomic value
publication,full resource/backend bounds and platform/physical verification remain owned by
G1-SLICE.5e.2.5.derived.3.3 and the existing execution/integration tasks. No cycle or execution
certificate follows from successful name/kind checking.
