# Formula documents in actual Design inputs

The implemented entry point is DesignInputs.prepare_formula_scope(explicit_content_limits).
It admits the complete actually held input content,then rechecks current ownership and source
references. The resulting borrowed scope normalizes only this root's actual source collection and
ordered recipe. It does not accept a substitute canonical inventory,source registry,recipe,
namespace,computed values or cache.

This is the first Design-backed integration stage. Source/recipe kind proofs,dependency/cycle
admission,scalar execution and real recipe storage remain separate stages. A normalized expression
is syntax with converted literals;it is not an accepted numerical result or a stored binding.
See [source validation and execution](formula-source-proofs.md) and
[ordered recipe execution](formula-recipe-execution.md) for the existing component contracts.

## Admit complete held input content first

Supply positive DesignResourceLimits for collection entries,text bytes and unbounded-counter bits.
The existing complete input-resource producer visits this actual root,including its explicit
source vector,private lookup index,original/canonical expression text,recipe,canonical inventory,
structure,Materials and higher metadata. No source or recipe prefix is accepted on excess.

The source host's resource port must honestly and exhaustively charge its immutable held data.
Resource walking is itself a host callback. It precedes current semantic callbacks,normalization
and namespace creation;the API does not promise to avoid every callback. The counters describe
logical held input content. They do not bound parser or namespace scratch,provider data,worker or
kernel allocations,allocator capacity,physical memory or CPU. Sewing/garment roots and complete
Design approval remain separate contracts.

After content succeeds,the actual complete current-input validator checks global identities,
parameter targets,all canonical Derived/source references,Materials,higher metadata and provided
references in its existing order. It preserves uncertainty and authoring data;it computes no value.
The resulting scope retains the exact root reference,selected limits,complete demand and existing
FormulaSourceInputs join. Copying the scope copies these borrowed facts without cloning the model
or host. The host need not implement Clone or Debug.

For example,the structural fixture's revision is the successor of u128::MAX and needs129 bits.
With a one-bit allowance,the original CounterBits refusal keeps maximum1,consumed0,requested129,
Structure scope,Design owner1 and field design.revision. No current semantic callback runs,even
when that host would subsequently refuse. With sufficient content room,a later source-host
refusal instead retains DesignInputsError::Sources and its original DesignMetadataError::Host.

## Normalize only these original documents

The source factory remains explicitly bounded by FormulaSourceNormalizationLimits. Its complete
S/N/retained-record checks happen before reparsing or outer allocation. The Design wrapper calls
that existing factory on the actual joined source owners. For a single actual source9003 containing
1 mm/3,the complete source/node/retained reservation is1/3/7. A source allowance0 refuses required1
and returns the original FormulaSourceNormalizationError,with the actual canonical/source owners
and selected limits. No partially normalized source vector becomes accepted.

normalize_recipe() invokes this root's existing FormulaRecipeDocument.normalized() factory.
Original syntax/literal errors and existing document structural limits remain authoritative.
The normalized statements stay in authored order and their name/span slices borrow the original
recipe text. Even an empty recipe keeps its actual containing Design root through the wrapper.

For an already constructed actual root,an inspection function can normalize both documents:

```rust
use sc_core::design::{DesignInputs,DesignResourceLimits,DesignResourceSource};
use sc_core::recipe::FormulaSourceNormalizationLimits;
fn inspect<H:DesignResourceSource>(root:&DesignInputs<H>,content:DesignResourceLimits,
    source_syntax:FormulaSourceNormalizationLimits) {
    let scope=root.prepare_formula_scope(content).expect("actual complete input admission");
    let sources=scope.normalize_sources(source_syntax).expect("complete source syntax");
    let recipe=scope.normalize_recipe().expect("actual recipe syntax");
    assert!(std::ptr::eq(sources.scope().root(),root));
    assert!(std::ptr::eq(recipe.scope().root(),root));
    assert!(std::ptr::eq(sources.normalized().inputs().inputs(),root.inputs()));
}
```

The constructor and explicit source ownership are described in
[owned Design inputs](../spec/design-inputs.md#actual-formula-source-ownership-checks).
The example deliberately stops at syntax inspection. The actual fixture recipe
let authored:length=unbound_source normalizes successfully while its name remains unbound.
Canonical Length40 still reports RequiresEvaluation for source9003. Neither successful
normalization nor source membership earns an evaluated value or a binding-round operation.
A bare source1 mm/3 retains its eventual unbound1000/3 micrometre result;only an actual recipe let
may store its result under the [existing binding rules](formula-rationals.md).

## Public API and failure ownership

| Public API | Meaning |
| --- | --- |
| `DesignFormulaScope` | Actual admitted input root,explicit content limits/demand and canonical/source join |
| `DesignFormulaScopeError` | Original root/limits and first complete content or current-input refusal |
| `DesignFormulaScopeRefusal` | Resources or Inputs with original typed cause |
| `DesignNormalizedFormulaSources` | Complete actual normalized source arenas and containing scope |
| `DesignNormalizedFormulaRecipe` | Actual ordered normalized recipe and containing scope |

All fields and construction paths are private. root(),limits(),demand() and source_inputs()
expose immutable facts on the scope. Each normalized wrapper exposes scope() and normalized()
through shared access. Neither wrapper accepts a caller-created arena or proof. They own their
normalization result without self-references;later checking must borrow these actual owners.

Scope preparation errors retain the original content/current cause and actual selected root/limits.
The uncommon semantic refusal is boxed to keep the error compact. Source normalization errors
remain the original factory type,including its inline allocation cause;the wrapper introduces no
new allocation-error box. Recipe normalization returns the original document error type.
A failed preparation returns no scope;failed normalization returns no accepted normalized prefix.

## Focused verification and remaining work

Four actual-root native contracts check exact complete demand,original source/name pointers,
non-Clone/non-Debug hosts,first content refusal before current semantics,later typed source refusal,
complete normalization refusal and empty-recipe containment. Compiler guards reject private root
substitution,root lifetime escape and normalized source mutation. An initial test used private span
fields;it was corrected to the existing public start()/end() accessors before passing.

This stage grants no source/recipe kind proof,cycle acceptance,value/provider completeness,
whole runtime resource bound or production approval. Subsequent task-tree leaves own actual-root
namespace and proof stages,resource-admitted source/recipe plans,real Derived recipe reads and
independent integration acceptance. Broad verification remains deferred until the agreed feature
set is complete;the latest full local/hosted verdict remains scoped to its recorded Git revision.
