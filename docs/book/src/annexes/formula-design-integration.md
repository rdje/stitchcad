# Formula documents in actual Design inputs

The implemented entry point is DesignInputs.prepare_formula_scope(explicit_content_limits).
It admits the complete actually held input content,then rechecks current ownership and source
references. The resulting borrowed scope normalizes only this root's actual source collection and
ordered recipe. It does not accept a substitute canonical inventory,source registry,recipe,
namespace,computed values or cache.

Actual Design-backed source and ordered recipe kind checking is also implemented below.
Native dependency/cycle admission can now feed a same-root combined scalar plan below.
Actual integrated scalar execution and real recipe storage remain separate stages.
A normalized expression is syntax with converted literals;it is not a checked kind,numerical
result or stored binding. A kind proof still supplies no cycle or numerical acceptance.
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

## Check against the actual containing namespace

DesignNormalizedFormulaSources.check_sources(explicit_check_limits) builds the namespace through
this actual root's existing formula_namespace() factory. It rechecks current inputs,borrows real
parameter declarations and obtains selected higher-host declarations through DesignFormulaHost.
Reserved names and initial-namespace collisions retain the existing factory rules. No caller can
supply a namespace,metadata seed,foreign canonical inventory or accepted source proof.

The existing complete source checker then enforces its S/N/T/retained-proof allowances and checks
every original source,all canonical dependencies and every Derived consumer kind. Namespace
creation/current checks happen before these proof quotas;namespace allocations/checker scratch and
allocator/CPU bounds remain separate. The wrapper does not copy quota algorithms or static rules.
The complete accepted DesignCheckedFormulaSources borrows its actual Design normalization owner
and owns the existing FormulaCheckedSources. normalized(),scope() and proof() preserve these
original stages. Original source/node/name/canonical/consumer witnesses remain available.

DesignNormalizedFormulaRecipe.check_kinds() independently builds the same actual root's current
initial namespace and invokes the existing whole ordered recipe checker. Prior-let declarations
come only from actual earlier checked statements. No caller-supplied earlier-let metadata or
stored value seeds this stage. The accepted DesignCheckedFormulaRecipe retains the actual
normalization owner and original whole FormulaCheckedRecipe in authored order.

For an actual parameter declared_width bound to canonical Length40,these original statements
pass kinds while Length40 can still require Derived evaluation:

```text
let authored:length=declared_width
let twice:length=authored+authored
```

The first dependency retains the actual parameter8003 and original Length40 reference. Both second
statement occurrences retain the real first let's name,statement1 and original whole/name spans.
No numerical value was read and no let was stored by this static check. Reversing dependency order
with let first:length=later followed by let later:length=1 mm instead refuses statement1 with the
original formula_unbound_name at later;no accepted prefix or reordered recipe escapes.

A source9003 containing declared_width/3 can pass kinds while that same Length40 is Derived from
9003. That self-dependency still needs the later actual cycle stage;static kinds alone accept no
executable graph. An unused source9004 containing missing instead refuses the whole source set at
its original source index1. A source9003 containing1==1 refuses the actual Length consumer with
TargetKind,expected Length/actual Boolean,without inventing a let-header diagnostic.

A generic inspection can borrow both complete proof owners:

```rust
use sc_core::{design::{DesignInputs,DesignResourceLimits,DesignResourceSource,DesignFormulaHost},
    recipe::{FormulaSourceNormalizationLimits,FormulaSourceCheckLimits}};
fn inspect<H:DesignResourceSource+DesignFormulaHost>(root:&DesignInputs<H>,content:DesignResourceLimits,
    syntax:FormulaSourceNormalizationLimits,source_checks:FormulaSourceCheckLimits) {
    let scope=root.prepare_formula_scope(content).unwrap();
    let sources=scope.normalize_sources(syntax).unwrap();
    let checked_sources=sources.check_sources(source_checks).unwrap();
    let recipe=scope.normalize_recipe().unwrap();
    let checked_recipe=recipe.check_kinds().unwrap();
    assert!(std::ptr::eq(checked_sources.scope().root(),checked_recipe.scope().root()));
}
```

The namespace allocation itself need not survive either existing proof factory. Its declarations
borrow original Design metadata;the accepted proofs remain bounded by their actual normalization
and root lifetimes. This needs no self-reference,namespace clone or extra host Clone/Debug bound.
There is no method parameter through which an equal-looking foreign root or supplied namespace can
be substituted during checking. The plan join below checks actual root and exact source-proof identity.

## Join actual proofs and prepare complete scalar plans

DesignCheckedFormulaSources.plan_inputs(recipe,source_execution_limits,recipe_execution_limits)
requires both privately constructed Design proof owners to reference the same actual DesignInputs
object. Equality of ids,revisions,source text or complete model content is insufficient. A failure
keeps both actual proof owners in DesignFormulaPlanInputError. A successful DesignFormulaPlanInputs
retains both owners and both explicitly selected existing policies. It is a provenance join;
quota admission and numerical execution have not happened yet.

Build the native dependency/cycle stages from inputs.sources().proof(),using explicit existing
FormulaSourceDependencyLimits and FormulaSourceCycleLimits. All original sources/edges,complete
quotas and cycle refusals remain those native factories' responsibility. The Design wrapper does
not duplicate their algorithms or manufacture an accepted graph.

inputs.prepare_scalar(&graph) first requires graph.dependencies().checked() to be this exact
selected source proof. Another proof over the same actual root and normalized arenas is refused
as ForeignGraph. This is stricter than comparing only containing root pointers. The supplied
graph remains available in that refusal;no quota or plan factory runs before this identity check.

After graph provenance passes,the order is:

1. The existing whole recipe admission checks complete statements,nodes,outputs and workspace.
2. The existing source graph factory admits complete source/node/name/output/workspace populations
   and all-source scalar capability before its outer/per-source plan preparation.
3. The admitted recipe's existing scalar preparation checks whole capability and joins its actual
   dependencies against this root's actual admitted canonical inventory,including genuine prior lets.

If any phase refuses,no accepted combined plan or source-only prefix escapes. The original native
error remains in RecipeAdmission,Sources or Recipe. DesignFormulaScalarPlanError retains the
actual joined inputs,so both original selected policies remain available even when a later phase
fails. Allocation causes remain inline in their existing native error;the wrapper adds no box.
The original borrowed typed causes remain available through refusal(). Standard error chaining
forwards only the native factories' actual static causes.

A successful DesignFormulaScalarPlan owns the complete native source graph and ordered recipe
plans. inputs(),scope(),sources() and recipe() expose readonly provenance and planning facts.
The recipe plan uses the actual containing canonical inventory and actual earlier checked let
statements. Sources and statements retain original order;no recipe is reordered to satisfy a graph.
Neither plan getter supplies a mutable replacement or an accepted numerical result.

For the original source9003 containing1 mm/3,S=1,N=3,D=0. Its complete source workspace2S+2N+D
is8 and source output allowance is1. For the two-statement authored/twice recipe above,S=2,N=4,
D=3,workspace2N+D=11 and output allowance2. These are conservative native logical reservations,
not physical allocation measurements or stored values. Source workspace7 instead keeps the
original allowed7/required8 refusal. Recipe output1 keeps original allowed1/required2 and refuses
before source preparation,even when the source policy would also fail.

A generic caller can preserve all owners while using the existing graph factories:

```rust
use sc_core::{design::{DesignCheckedFormulaSources,DesignCheckedFormulaRecipe},recipe::{
    FormulaSourceGraphExecutionLimits,FormulaRecipeExecutionLimits,
    FormulaSourceDependencyLimits,FormulaSourceCycleLimits}};
fn prepare<'a,H>(sources:&'a DesignCheckedFormulaSources<'a,H>,
    recipe:&'a DesignCheckedFormulaRecipe<'a,H>,source_limits:FormulaSourceGraphExecutionLimits,
    recipe_limits:FormulaRecipeExecutionLimits,edges:FormulaSourceDependencyLimits,
    cycles:FormulaSourceCycleLimits) {
    let inputs=sources.plan_inputs(recipe,source_limits,recipe_limits).unwrap();
    let dependencies=inputs.sources().proof().prepare_dependencies(edges).unwrap();
    let graph=dependencies.admit_acyclic(cycles).unwrap();
    let plan=inputs.prepare_scalar(&graph).unwrap();
    assert!(std::ptr::eq(plan.recipe().admission().proof(),recipe.proof()));
    assert!(std::ptr::eq(plan.scope().root(),sources.scope().root()));
}
```

An empty actual recipe fits zero statement/node/workspace/output allowances while its required
source9003 still needs complete source admission. An unused tangent source refuses at its actual
original source index;an actual tangent recipe call refuses with its actual statement/node cause.
Preparation never reads a context or value provider,executes a source,stores a let or publishes a
computed result. Canonical Length40 still reports RequiresEvaluation9003 after successful planning.
Runtime scratch/output reservation,namespace/checker/provider/kernel/allocator/CPU bounds and
whole Design authority remain separately owned. Logical quotas on these plans grant no global
resource,physical,platform or production approval certificate.

## Public API and failure ownership

| Public API | Meaning |
| --- | --- |
| `DesignFormulaScope` | Actual admitted input root,explicit content limits/demand and canonical/source join |
| `DesignFormulaScopeError` | Original root/limits and first complete content or current-input refusal |
| `DesignFormulaScopeRefusal` | Resources or Inputs with original typed cause |
| `DesignNormalizedFormulaSources` | Complete actual normalized source arenas and containing scope |
| `DesignNormalizedFormulaRecipe` | Actual ordered normalized recipe and containing scope |
| `DesignCheckedFormulaSources` | Complete original source kinds/canonical/consumer proofs and actual Design owner |
| `DesignFormulaSourceCheckError` | Actual namespace or whole-source failure,normalized Design owner and selected limits |
| `DesignCheckedFormulaRecipe` | Complete original ordered recipe kind proof and actual Design owner |
| `DesignFormulaRecipeCheckError` | Actual namespace or whole-recipe failure and normalized Design owner |
| `DesignFormulaPlanInputs` | Actual same-root source/recipe proof owners and explicit native execution policies |
| `DesignFormulaPlanInputError` | Both actual checked owners when containing root identity differs |
| `DesignFormulaScalarPlan` | Complete native source/ordered recipe scalar plans and actual joined inputs |
| `DesignFormulaScalarPlanError` | Actual joined inputs/policies and original first provenance or factory refusal |
| `DesignFormulaScalarPlanRefusal` | ForeignGraph,RecipeAdmission,Sources or Recipe with actual witnesses |

Accepted scopes,normalizations and checked wrappers have private fields and construction paths.
root(),limits(),demand() and source_inputs() expose immutable facts on the scope. Each normalized
wrapper exposes scope() and normalized() through shared access. Neither wrapper accepts a
caller-created arena or proof. They own their normalization result without self-references;
checking borrows these actual owners. Checked wrappers expose their original proof() and scope().

Scope preparation errors retain the original content/current cause and actual selected root/limits.
The uncommon semantic refusal is boxed to keep the error compact. Source normalization errors
remain the original factory type,including its inline allocation cause;the wrapper introduces no
new allocation-error box. Recipe normalization returns the original document error type.
A failed preparation returns no scope;failed normalization returns no accepted normalized prefix.

Source checking errors expose Namespace or Sources variants with the actual Design normalization
owner. selected limits remain available through limits(),including when namespace creation fails
before proof quotas. Recipe errors expose Namespace or Recipe with the actual Design recipe owner.
Their original borrowed typed causes stay inspectable in those variants. Standard Error::source()
forwards only actual static factory causes,such as an allocation failure or concrete current-host
failure;it does not erase a borrowed semantic cause into an invented static token. No diagnostic
box is added around an original source allocation error. Failed checking returns no accepted prefix.

## Shared earned source runtime before recipe execution

The existing standalone source worker now delegates to a shared private runtime. It reserves all
source controls,call frames,initially empty output slots and every source's task/value buffers
before starting a source or reading any value. Original complete plan limits remain authoritative.
The public one-source API still checks the exact requested source id before runtime allocation and
returns the same original graph,selected entry,context,completed roots and read/error witnesses.

Inside the engine,multiple original source demands can share one runtime. Only successfully
executed original roots enter its private cache. Repeated demands return those actual unbound
roots without re-entering their machines or advancing their completion ordinals. With actual
a=1 mm/3,a later source a*3 returns1000 micrometres and a different source a+a returns2000/3.
The original a remains1000/3 with its original completion ordinal;no source edge stores333 first.
Unused sources remain unexecuted. Reserved control/frame/output buffers remain stable across
these successive demands. Canonical Derived states remain unchanged.

An actual later source failure poisons the private runtime. Even an earlier successfully cached
root cannot then be requested or published as an accepted prefix. The enclosing worker returns
its original typed failure and drops the complete private state. This supplies an internal reuse
boundary;it introduces no caller-seeded cache or new public execution API. The actual combined
Design recipe worker and once-only let storage bridge remain subsequent tasks.

The private recipe read adapter now supports actual Derived suspension against that earned runtime.
An unresolved canonical Derived name returns the original joined input to the existing machine
continuation. Its read slot stays empty until the actual supplier has executed. Resuming the same
machine then records the original canonical join,actual successful source proof and unbound root.
Source-cache access checks the exact canonical inventory;an equal-looking foreign inventory cannot
supply that read. Direct canonical/reserved reads and genuine completed prior-let bindings retain
their existing witnesses. No caller can supply a value or cached root to this adapter.

For an actual declared_width supplied by bare source1 mm/3,the focused adapter contract uses:

```text
let prior:length=1 mm/3
let total:length=prior+declared_width+declared_width
```

The actual first RHS executes and its real let stores333 once. The second operand records that
prior-let read before suspending at the first declared_width. It resumes after the actual source
computes1000/3,reads the same earned root twice and completes its unbound RHS as2999/3. The first
read is not replayed;the source completes once. The canonical Derived state stays unchanged.
This checks the private adapter and continuation;it does not publish a complete combined recipe
result or claim that the second let has been stored by the new worker.

The existing standalone recipe worker explicitly selects its original direct-read mode and still
returns RequiresEvaluation for unresolved Derived inputs. Its public read-kind enum and direct/
prior-let result interfaces remain unchanged. A private read record distinguishes earned Derived
witnesses for the forthcoming integrated worker;it cannot be published as a standalone direct read.
The complete paired worker,public Design execution entry point and independent acceptance remain
subsequent tasks.

The recipe side now stages its original admitted plan,fixed context,N task/value buffers and
complete S outcome reservation in a shared private runtime. Construction starts no operand,read,
provider or binding operation. The existing declaration-order statement loop lives in that same
runtime,including actual RHS storage,prior-let visibility,assertion left/right/header order and
original typed refusals. The standalone public adapter prepares that runtime,executes the original
mode and publishes only its completed whole result. It introduces no second implementation of
storage or assertion semantics. The following paired-worker slice will drive its actual source
suspensions after both runtimes have completed reservation;that combined public operation is not
yet available.

## Focused verification and remaining work

Four actual-root native contracts check exact complete demand,original source/name pointers,
non-Clone/non-Debug hosts,first content refusal before current semantics,later typed source refusal,
complete normalization refusal and empty-recipe containment. Compiler guards reject private root
substitution,root lifetime escape and normalized source mutation. An initial test used private span
fields;it was corrected to the existing public start()/end() accessors before passing.

Six additional actual-root contracts cover real parameter/prior-let metadata and original pointers,
complete late unused-source/consumer-kind errors,forward-let order,actual current namespace failure,
source quota and empty recipe proofs. Generic inspection compiles without host Clone/Debug.
Compiler guards reject private proof replacement,normalization-owner lifetime escape and supplied
namespace arguments. A test fixture initially used true as a Boolean literal;the original
formula_unbound_name and grammar identified it as a name. The corrected1==1 fixture reaches the
intended original consumer-kind mismatch without changing any production checking rule.

Six further actual-root plan contracts check exact complete native populations,original canonical
and prior-let links,equal-content root rejection,same-arena alternate-proof rejection,recipe-before-
source quota order,source workspace and original unavailable-node errors,and empty recipe plans.
Compiler guards reject private paired-owner/recipe-plan replacement and plan lifetime escape;
the generic preparation example compiles without a host Clone/Debug bound.

Kind checking alone grants no cycle acceptance. Native dependency/cycle factories still own that
stage before Design plan preparation. Planning grants no numerical result,provider completeness,
whole runtime resource bound or production approval. Subsequent task-tree leaves own real Derived
recipe reads,once-only let storage and independent integration acceptance. Broad verification
remains deferred until the agreed feature set is complete;the latest full local/hosted verdict
remains scoped to its recorded Git revision.
