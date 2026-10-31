# Execute scalar construction recipes

> **Implemented with focused checks:** G1-SLICE.5e.2.5.1.2.2.2 executes the admitted available
> scalar subset from its actual complete static recipe proof. It evaluates real lets/assertions in
> declaration order,stores each successful binding once and retains actual executed source reads.
> Derived reads now execute through [actual source-backed Design recipes](formula-design-integration.md#execute-the-private-actual-design-plan).
> Explicit-policy direct/source-backed Tan recipes and fresh direct/paired/Design replay are available
> below. Geometry,corrupt-load validation,full replay/platform identity and complete language/bus/
> physical acceptance retain their separate owners. No release claim follows.

Start with complete [syntax/input normalization](formula-recipe-inputs.md) and
[whole static checking](formula-checked-recipes.md),then admit the real proof and prepare every
source before values. The [expression controller](formula-execution-plan.md#actual-scalar-expression-execution)
and recipe executor share one actual-node machine and existing exact sourced kernels.

## Replay a prepared scalar recipe

`FormulaScalarRecipePlan::replay(context)` starts a complete fresh execution at statement one.
It borrows the prepared plan,so the same plan can be retried and a plan retained by an accepted
result can be replayed while that earlier result stays available. Every attempt repeats whole
admission and source preparation with the original checked proof,actual canonical inventory,
logical limits and optional tangent policy. It allocates new runtime state and copies no prior
binding,read slot or cached value. The supplied context belongs to this attempt.

```rust
use sc_core::{recipe::*,value::CanonicalInputs};
let inputs=CanonicalInputs::new(vec![],vec![])?;
let recipe=FormulaRecipe::parse("let a:length=1 um/3\nlet b:length=a+1 um")?
    .normalize_literals()?;
let proof=recipe.check_kinds(FormulaNamespace::new([])?).expect("whole proof");
let plan=proof.admit_execution(FormulaRecipeExecutionLimits::new(2,6,13,2))
    .expect("explicit limits").prepare_scalar(&inputs).expect("whole sources");
let context=FormulaScalarExecutionContext::new(FormulaToleranceContext::empty(),
    FormulaSizeContext::empty());
let first=plan.replay(context).expect("first run");
let second=first.plan().replay(context).expect("fresh replay");
assert_eq!(first.statements().len(),2);
assert_eq!(second.reads().count(),1);
# Ok::<(),Box<dyn std::error::Error>>(())
```

Here `a` stores the rounded value zero micrometres,and `b` reads that stored zero and stores one
micrometre on every attempt. Replay does not reuse the earlier fractional RHS as a binding.
Changing a physical tolerance context can select another branch;only that attempt's executed
dependencies appear in its read trace. For example,`if(eps_phys<5 um,a,9 um)` reads `a` only when
the supplied physical tolerance is below five micrometres.

`FormulaScalarRecipeReplayError` preserves the original `Admission`,`Preparation` or `Execution`
cause. A late assertion failure returns an error with its actual statement and quantitative
comparison. It exposes no accepted statement/read prefix and leaves the original plan,canonical
inputs and any earlier accepted result unchanged. Replaying an empty recipe accepts zero outputs
with zero allowances. Tangent replay preserves the explicitly selected per-call limits;zero limits
still refuse an executed tangent call. Derived replay uses the paired entry below.
This focused implementation does not define persisted
result bytes,corrupt-load validation,geometry replacement or cross-platform result identity.

## Replay with actual Derived sources

`FormulaScalarRecipePlan::replay_with_sources(&sources,context)` borrows both prepared plans and
returns a fresh `FormulaExecutedScalarSourceRecipe`. Plans retained by an earlier paired result
can supply the same operation:

```rust
use sc_core::recipe::*;
fn replay<'a>(first:&FormulaExecutedScalarSourceRecipe<'a>,
    context:FormulaScalarExecutionContext<'a>)
    ->Result<FormulaExecutedScalarSourceRecipe<'a>,FormulaScalarSourceRecipeReplayError<'a>> {
    first.recipe_plan().replay_with_sources(first.source_plan(),context)
}
```

The exact canonical input owner must match before either fresh preparation. Equal cloned input
content is insufficient. Replay then repeats whole recipe admission and source preparation with
their original proof/graph owners,complete logical quotas and independently selected recipe/source
tangent policies. Both runtimes use the supplied context. Each attempt starts at statement one
with empty source completion slots and fresh read records. Within that attempt,repeated supplier
demands still share the once-completed unbound source root;only actual lets store a rounded binding.

For a Derived length `width=1 mm/3`,a Derived general Length `triple=width*3` still yields1000
micrometres. Repeated replay keeps the original unbound width1000/3 and restarts its completion
ordinal at one. If a new context selects a branch that reads neither source,the new result has no
completed sources;the earlier result keeps its original two completed sources. A subsequent attempt
that demands them recomputes them and restarts their ordinals. Nothing seeds the new cache.

`FormulaScalarSourceRecipeReplayError` distinguishes `RecipeAdmission`,`RecipePreparation`,
`SourcePreparation` and the original `Execution` cause. That last variant also retains the early
foreign-owner refusal with both actual inventories,proof/graph and selected policies. A later
supplier or assertion failure exposes no accepted source/statement prefix and leaves both borrowed
plans and earlier accepted results intact. A zero tangent policy in the recipe can fail its own
call while the supplier policy remains sufficient;the reverse preserves the real requesting recipe
statement and supplier failure. No policy is silently substituted for the other.

Focused tests use actual Design-owned sources,original fractional roots,changed-context empty
cache,late assertion failure,foreign-owner refusal and both independent tangent policies. This API
returns native formula execution;fresh containing-Design admission/currentness,geometry replacement,
storage/load validation and complete persisted result identity remain separate obligations.

## Replay a containing Design

`DesignFormulaScalarPlan::replay(context)` borrows an actual prepared Design plan.
`DesignExecutedFormulaRecipe::replay(context)` borrows an earlier accepted Design result.
Both return a fresh `DesignExecutedFormulaRecipe` after complete held-input/current admission,
then fresh native paired replay. Neither consumes the earlier plan or result.

```rust
use sc_core::{design::*,recipe::FormulaScalarExecutionContext};
fn again<'a,H:DesignResourceSource>(first:&DesignExecutedFormulaRecipe<'a,H>,
    context:FormulaScalarExecutionContext<'a>)
    ->Result<DesignExecutedFormulaRecipe<'a,H>,DesignFormulaReplayError<'a,H>> {
    first.replay(context)
}
```

Every attempt recharges the actual containing root's complete held input content under the
original model limits and rechecks its current host references. Only success reaches native
preparation or values. The original source/recipe proof join,graph and independent native policies
remain pinned. Replay uses fresh source completion slots and read records,with the same unbound
source/once-stored binding distinction as native paired replay. The model and host are borrowed;
the host needs neither Clone nor Debug.

An earlier accepted input scope does not authorize a later attempt. If the host now refuses a
current reference,replay returns `DesignFormulaReplayError::Admission` with the actual root,
model limits and original typed cause. Its `scope()` is absent. The earlier accepted result remains
available. If that host later accepts again,a new replay repeats admission and can succeed.

After fresh admission,a native preparation or execution failure returns `DesignFormulaReplayError::Replay`.
Its `inputs()` retains the original joined proof owners and selected policies;`scope()` exposes the
new accepted logical input scope. The nested native replay error preserves its original stage,
statement/requester/supplier/provider context. For example,a named physical-tolerance assertion can
succeed with a supplied provider,then refuse atomically when replay receives a context without
that provider. The earlier accepted statements remain intact. Empty recipes also obtain fresh
containing admission while producing no source or statement outputs.

These APIs add fresh logical input/current checks to native replay. They do not publish a Design
replacement or define storage bytes,geometry integration,whole allocator/CPU bounds or production
approval. Full accepted-result identity and corrupt-load integration remain owned work.

## Canonical stored-binding output

Accepted direct and paired scalar results expose `canonical_bindings(max_bytes)`,returning an owned
`FormulaCanonicalBindings`. A containing Design result uses
`result.execution().canonical_bindings(max_bytes)`. The output compares stored bindings across
fresh runs;it is derived data and does not become authoritative project content.

The exact ASCII contract is `(bindings (bind name kind:value (sources ...))...)`,with one space
between parts,no padding or terminal newline. Bindings retain declaration order and actual names.
The empty output is `(bindings)`,also for an accepted recipe containing only assertions. Every
binding has its actual stored kind and signed decimal integer or lowercase `true`/`false`.

| Kind | Stored payload |
| --- | --- |
| length | signed integer micrometres |
| angle | signed integer microdegrees,including complete turns |
| area | integer square micrometres |
| ratio | signed integer parts per million |
| count | nonnegative integer |
| boolean | `true` or `false`,with no numeric coercion |

Every `(sources ...)` lists the stored binding's distinct approximation-producing calls in the
existing lexical machine-token order. An exact binding has `(sources)`. Contributions survive a
false Boolean or numerical cancellation;untaken branches contribute nothing. These source bytes
distinguish exact ratio1.0 from `sin(90 deg)`,even though both store1000000 parts per million.

```text
let a:length=1 um/3
let b:length=a+1 um
```

After complete accepted execution,the exact bytes are:

```text
(bindings (bind a length:0 (sources)) (bind b length:1 (sources)))
```

The serializer reads the actual once-stored bindings. It does not serialize the fractional RHS,
round again,normalize an angle direction or evaluate a provider. Replaying that recipe from
statement one produces identical binding bytes. Both native direct and paired modes use the same
serializer. For example,an unbound Derived width1000/3 multiplied by three stores1000 and emits
`length:1000`,while its unbound source root remains outside these bytes.

The caller selects the complete output-byte allowance explicitly. The serializer first measures
all framing,names,values and source tokens without allocating an output. An insufficient allowance
returns `FormulaCanonicalBindingsError::Bytes { allowed,required }` with the complete requirement.
An exact allowance succeeds. One fallible output reservation follows admission;the writer refuses
to exceed that measured extent. Byte population overflow and original allocation errors remain
separate typed causes. No partial owned output escapes and the accepted result stays unchanged.
This bounds logical output bytes,not allocator capacity or whole runtime memory.

The private constructor accepts only complete accepted results. `as_str()` explicitly reveals
customer-bearing bytes;Debug reports only their count. `into_string()` transfers independent owned
bytes that can outlive the source,proof and execution. Its Rust type is distinct from canonical
expression,statement and recipe identities. Raw bytes define neither a hash framing scheme nor a
reader/project envelope. Assertions,RHS expressions,read traces,source caches,policies and containing
Design approval are outside this identity. Recipes with the same stored bindings can therefore
have equal binding output while differing in those other facts. Full accepted-result identity,
geometry and corrupt-load integration remain separately owned.

## Whole checked-recipe admission

| Public API | Role |
| --- | --- |
| `FormulaRecipeExecutionLimits` | Explicit statement, node, workspace and output allowances |
| `FormulaRecipeExecutionAdmission` | Allocation-free reservation pinned to the complete actual recipe proof |
| `FormulaRecipeExecutionAdmissionRefusal` | Complete quota/population refusal, separate from language diagnostics |
| `FormulaRecipeExecutionAdmissionError` | Actual whole proof, selected limits and original resource refusal |

`FormulaCheckedRecipe::admit_execution` is available at G1-SLICE.5e.2.5.1.1. It accepts only explicit
limits and borrows the existing successful whole static proof. The caller cannot substitute counts,
a statement prefix, values, ordinals or another recipe. The admission retains proof(), limits(),
statement_count(), binding_count(), assertion_count(), node_count(), name_use_count(),
workspace_records() and output_records(). The actual immutable statements and dependencies remain
available through proof(); their source owners and genuine one-based ordinals include assertions.

For the complete recipe, S is its statement population, N is the sum of all operand nodes and D is
every dependency occurrence. Both assertion operands, repeated names and untaken branches count.
Each assertion's original symbolic tolerance header adds one source record to D, without inventing
an expression node. Reserve N task frames, N value slots and D source records: workspace is 2N+D.
Separately reserve S outcome records, one per let or assertion. These are conservative logical
reservations; sequential workspace reuse may use less. They do not certify allocator bytes, kernel
or provider resources, CPU, backend availability, input currentness or accepted execution.

```rust
use sc_core::recipe::*;
let recipe = FormulaRecipe::parse("let n:count=1\nassert check:eps_num=n==n")?
    .normalize_literals()?;
let proof = recipe.check_kinds(FormulaNamespace::new([])?).expect("complete static proof");
let limits = FormulaRecipeExecutionLimits::new(2, 3, 9, 2);
let admitted = proof.admit_execution(limits).expect("exact complete allowance");
assert_eq!((admitted.binding_count(), admitted.assertion_count()), (1, 1));
assert_eq!((admitted.node_count(), admitted.name_use_count()), (3, 3));
assert_eq!((admitted.workspace_records(), admitted.output_records()), (9, 2));
assert!(std::ptr::eq(admitted.proof(), &proof));
# Ok::<(), Box<dyn std::error::Error>>(())
```

Refusal order is complete statements, checked population/workspace arithmetic, nodes, outputs,
then workspace. For that example, limits (1,0,0,0) refuse Statements with actual 2; (2,2,0,0) refuse
Nodes with actual 3; (2,3,0,1) refuse OutputRecords with required 2; (2,3,8,2) refuse WorkspaceRecords
with required 9. Zero allowances admit an empty recipe. None widens the fixed language bounds.
Aggregate refusals retain the actual whole proof and limits, without inventing a failed statement
ordinal or canonical formula diagnostic. Missing runtime context and numerical failures remain
unevaluated; a later static error prevents the complete proof needed to call admission at all.

Five native tests cover actual Unknown/source owners and global spans, quota priority/exact limits,
empty and maximum4096-statement recipes, static-before-runtime order and four independent reference
parser rows. The largest assertion-only fixture has8192 nodes,4096 header uses,20480 workspace and
4096 outputs; no tolerance is read. Three compiler-negative examples enforce explicit limits,
private populations and proof lifetime; the positive example executes. The existing independent
reference parser supplies finite source/population rows through
`docs/tasks/artifacts/formula_structure/recipe_execution_admission_oracle.py`; it reads no Rust or
native output and evaluates no values. The execution below consumes this admission. Missing dependency families,full whole-result
acceptance and replay remain owned integration work.

## Prepare every whole-recipe scalar source

| Public API | Role |
| --- | --- |
| `FormulaScalarRecipePlan` | Complete read-free whole scalar preparation over actual admission and inputs |
| `FormulaScalarRecipeSource` | One original consumer occurrence and prospective supplier |
| `FormulaScalarRecipeSourceKind` | Actual canonical join, reserved role or earlier checked let |
| `FormulaScalarRecipePlanRefusal` | Unavailable capability, original canonical cause or source allocation refusal |
| `FormulaScalarRecipePlanError` | Actual whole owner/inventory and available failed statement, operand or dependency |

Whole admission's prepare_scalar(inputs) is available at .5e.2.5.1.2.1. It first scans every statement
in authored order, binding RHS or assertion left then right, with each expression's actual certified
arena order. This uses the same exhaustive builtin-availability catalog as detached scalar
preparation. Legacy Tan without an explicit selected policy reports MissingTangentPolicy;
geometry remains an implementation gap. Neither becomes a formula-language diagnostic. An unavailable later operand precedes an earlier missing canonical target, because the
whole capability scan finishes before source allocation or inventory joins. Capability scanning is
separate from runtime source-order evaluation; no values or providers are read.

After capability acceptance, one D-source vector reserves fallibly. Every original whole dependency
then joins in its existing order: assertion header before left/right operand uses; repeats and
untaken branches remain distinct. Canonical sources reuse the real inventory adapter, preserving
actual borrowed owner and original typed failures. Reserved sources retain their exact role,
including a missing optional physical tolerance header. No fallback value or provider is supplied.

A PriorLet source points to the actual earlier FormulaCheckedStatement in this complete proof.
The private join verifies the supplier ordinal, let role, name, original spans and kind against its
original occurrence. Interleaved assertions count toward ordinals but supply no scalar. This is a
checked metadata relationship, not a completed expression, stored integer, executed prefix or
accepted binding. Neither caller values nor a standalone binding conversion enter preparation.

```rust
use sc_core::{recipe::*, value::CanonicalInputs};
let inputs = CanonicalInputs::new(vec![], vec![])?;
let recipe = FormulaRecipe::parse("let n:count=1\nassert check:eps_phys=n==n")?
    .normalize_literals()?;
let proof = recipe.check_kinds(FormulaNamespace::new([])?).expect("whole static proof");
let plan = proof.admit_execution(FormulaRecipeExecutionLimits::new(2, 3, 9, 2))
    .expect("exact allowance").prepare_scalar(&inputs).expect("sources, no values");
assert_eq!(plan.sources().len(), 3);
assert!(matches!(plan.sources().first().expect("header").kind(),
    FormulaScalarRecipeSourceKind::Reserved(_)));
assert!(std::ptr::eq(plan.admission().proof(), &proof));
# Ok::<(), Box<dyn std::error::Error>>(())
```

The plan exposes admission(), inputs() and the complete sources() slice. Each source exposes its
original dependency() and kind(). Private fields prevent substitution. On a capability refusal,
error.statement(), expression_part() and node() retain the actual operand owner and node. A canonical
join failure instead exposes dependency() with genuine consumer ordinal, role, span and declaration,
and statement() returns that actual consumer. refusal() reconstructs the same original typed
canonical failure from unchanged source/inventory; allocation_error() retains the original allocator
cause. Allocation has no fabricated statement/operand. Errors expose no partial plan or source list.

Five native whole-plan tests cover actual Unknown/Derived records, repeated and untaken occurrences,
real prior suppliers at ordinals1 and3, late capability priority, exact canonical refusal context,
empty and4096-assertion header populations. The five existing detached-plan tests still pass after
sharing the capability catalog. Two compiler negatives enforce private input ownership and borrowed
inventory lifetime; the positive example executes. Actual execution follows below. Full acceptance,
Derived evaluation,missing families,replay and platform identity retain their implementation owners.

## Execute actual declarations

| Public API | Role |
| --- | --- |
| `FormulaExecutedScalarRecipe` | Complete successful admitted scalar execution,privately constructed |
| `FormulaExecutedScalarRecipeStatement` | One actual checked statement and its earned successful outcome |
| `FormulaScalarRecipeOutcome` | Exact RHS plus stored binding,or true original assertion comparison |
| `FormulaScalarRecipeRead` | One actually executed dependency and optional actual name node |
| `FormulaScalarRecipeReadKind` | Original canonical/provider read or real earlier stored binding witness |
| `FormulaScalarRecipeWorkspace` | Tasks,values or complete statement outcomes |
| `FormulaScalarRecipeError` | Actual complete proof,available refused statement and original cause |
| `FormulaScalarRecipeErrorCause` | Inline allocation refusal or boxed original semantic failure |
| `FormulaScalarRecipeFailure` | Actual operand,storage,tolerance,comparison or false assertion failure |

Call plan.evaluate(explicit_context) after whole preparation. It consumes the plan and reserves N
task frames,N value slots and S statement outcomes before any read. The plan's existing D records
retain actual successful reads in place;no second trace vector or copied input state is allocated.
Scratch reuses its admitted capacity between original operands. These are logical bounds,not an
allocator-byte,CPU,numeric-kernel,provider,currentness or evidence certificate.

The runtime keeps its completed outcomes private. A let evaluates its actual checked RHS first,
then uses the shared private storage conversion once. Only success appends the binding. Later uses
must join the exact earlier checked supplier to its real completed ordinal,including interleaved
assertions,and read that stored value. No caller can seed,advance or substitute the prefix. Numeric
reads use the rounded integer;Boolean false stays present as Boolean. Original approximation
contributions survive storage,conditional reads and cancellation. No replacement MachineToken or
name string is allocated;the original checked statement retains the actual source name.

An assertion evaluates its left operand,then right,then reads its original tolerance header once.
It reuses the actual quantitative comparison,including T1 approximation refusal. False refuses
with formula_assertion and the original comparison;the error's actual statement supplies its name
and ordinal. Success retains that original true comparison. A header is metadata and has no AST
node;no synthetic expression or node is manufactured to read it.

```rust
use sc_core::{recipe::*, value::CanonicalInputs};
let inputs = CanonicalInputs::new(vec![], vec![])?;
let recipe = FormulaRecipe::parse(
    "let a:length=1 mm/3\nlet b:length=a+a\nassert check:eps_num=b==666 um"
)?.normalize_literals()?;
let proof = recipe.check_kinds(FormulaNamespace::new([])?).expect("whole static proof");
let result = proof.admit_execution(FormulaRecipeExecutionLimits::new(3, 8, 20, 3))
    .expect("exact complete allowance").prepare_scalar(&inputs).expect("actual sources")
    .evaluate(FormulaScalarExecutionContext::new(FormulaToleranceContext::empty(),
        FormulaSizeContext::empty())).expect("all actual statements succeed");
let FormulaScalarRecipeOutcome::Binding { expression, binding } =
    result.statements().first().expect("real first let").outcome() else { unreachable!() };
assert_eq!(expression.numeric().expect("Length").rational(), FormulaRational::new(1000, 3, false)?);
assert_eq!(binding.value().numeric_integer(), Some(333));
assert_eq!(result.reads().map(|r| r.dependency().declaration().name()).collect::<Vec<_>>(),
    ["a", "a", "b", "eps_num"]);
# Ok::<(), Box<dyn std::error::Error>>(())
```

Here b reads333 twice and stores666;the pre-binding1000/3 fraction is inspection data,not the value
later names use. An exact Count of2*i64::MAX remains valid during expression arithmetic but refuses
its actual let storage,retaining Count,0/i64::MAX bounds and the full rounded measured witness.
A taken Unknown or Derived canonical read retains the original observation or sole formula request;
an untaken occurrence remains unread after its canonical source was joined. No default is supplied.

## Inspect actual outcomes,reads and failure

A successful result exposes plan(),statements() and reads(). Completed wrappers expose statement()
and outcome();their fields/constructors are private. The original statement supplies real names,
spans,canonical identity and ordinals. Binding outcomes expose the exact RHS and actual stored
binding;assertion outcomes expose only true original comparisons. Supplied outcome values cannot
construct or alter an executed recipe.

Reads stream in actual execution order. Within an operand,only selected names appear in source
order. For assertions,the left/right occurrence blocks stream before the header,matching evaluation.
This reorders existing block views without allocating or sorting another trace. Each read exposes
dependency(),source(),value() and node(). The node is Some only for a real operand name;the actual
assertion header returns None. PriorLet source witnesses retain the real supplier and the actual
stored binding copied from this execution's private prefix,including zero/false and sources.
Default result/read/error Debug omits authored names and values;explicit getters expose them.

Any runtime failure returns FormulaScalarRecipeError and no accepted result or successful prefix.
proof() retains the complete static owner,including the suffix;statement() returns the genuine
refused statement. cause() distinguishes pre-read allocation from original semantic failures.
Expression failures retain binding/left/right role and the original shared-machine node/cause.
Binding failures retain actual kind and exact storage rule;the statement supplies its real name.
ToleranceRead preserves the missing provider request;Comparison preserves actual operands/class/
provider and approximation refusal;Assertion preserves the false original comparison. Aggregate
allocation has no invented statement or expression. Its original TryReserveError remains inline,
so reporting an allocation failure requires no new diagnostic allocation.

Seven focused native tests cover actual fractions/stored zero/false,earned prior suppliers,
interleaved assertions,approximation propagation,Unknown/Derived requests,operand-before-header
failure order,false comparison values,full Count storage witnesses and empty/max4096 outcomes.
Actual within and assertion reads retain the same physical provider and explicit context label;
unselected optional format stays unread. The11 existing expression tests,including49 independent
scalar rows/frozen bytes,still pass. Five scoped documentation tests cover the actual example and
private proof/input/result boundaries. The finite whole-recipe independent checkpoint below closes .5e.2.5.1.2.2.acceptance. Full missing-
family/Derived/load/replay/platform/worked-example completion remains .5e.2.4.integration/.5e.4/.5g.

## Independent whole scalar checkpoint

The focused actual recipe target passes nine tests after adding two independent-oracle watchers.
It compares18 complete recipe rows:11 successes and7 intended refusals. The4167-byte frozen fixture
contains original source,each successful outcome's exact unbound RHS/stored value/contribution set,
true assertion operands/class,and actual executed read values/sources in order. Error rows retain
actual ordinal,name and diagnostic,with no accepted partial trace. A second watcher reruns the
independent producer and requires equality of every fixture byte.

`docs/tasks/artifacts/formula_structure/scalar_recipe_execution_oracle.py` loads the existing
independent reference parser and table-driven evaluator. It preflights the complete source,then
traces original value_of_name and stored calls while invoking their unchanged implementations.
Prior lets enter its runtime environment only from real returned stored results. It reads no Rust,
native result or frozen fixture. The native watcher independently constructs the actual whole
proof/admission/source plan and runs the real recipe executor for each source. Production code is
unchanged by this checkpoint.

Rows cover once-only fractions,zero/false,raw signed turns,half-quantum Count storage,real earlier
lets across assertions,conditional laziness,round_to/min/clamp sources,sin/atan/arc_length sources,
within read order,false assertions,wide Count storage,T1 refusal,division,missing tolerance and
negative roots. An initial authored Count-times-Ratio example correctly failed static checking:
the existing contract returns Ratio. The intended half-Count case uses Count-divided-by-Ratio;
the existing independent signature table and actual reference inference agree. No type rule changed.

```bash
python3 -I -B scripts/local_environment.py -- cargo test -p sc-core --test formula_scalar_recipe_execution_contract -- --nocapture
python3 -I -B docs/tasks/artifacts/formula_structure/scalar_recipe_execution_oracle.py
```

This finite checkpoint closes actual admission/preparation/private-prefix work for the implemented
scalar subset. It has no external canonical-input rows;the separate seven native controls retain
actual Unknown/Derived requests and physical-provider identity. Derived dependency execution and
remaining capability families remain open. Full language/worked-example/platform/load/replay/
physical verification is not inferred from these18 rows. Next .5e.2.5.derived recovers actual
canonical formula/document/input dependency contracts before selecting bounded Derived integration.

## Explicit tangent policies in ordered recipes

FormulaRecipeExecutionAdmission::prepare_scalar_with_tangent_limits selects explicit per-call
limits for actual recipe-owned Tan nodes. The complete whole-proof capability scan still precedes
D-source allocation and canonical joins,including later statements and untaken operands. The legacy
entry point reports MissingTangentPolicy with the original statement,operand role and call node.
No source/provider value,let storage or partial statement result is computed during preparation.

```rust
use sc_core::{recipe::*, value::CanonicalInputs};
let inputs = CanonicalInputs::new(vec![], vec![])?;
let recipe = FormulaRecipe::parse(
    "let first:ratio=tan(30 deg)/1000000\nlet second:ratio=first+first\nassert exact:eps_geo=second==0.000002")?
    .normalize_literals()?;
let proof = recipe.check_kinds(FormulaNamespace::new([])?).expect("actual whole proof");
let selected = FormulaTangentLimits::new(2073, 512);
let plan = proof.admit_execution(FormulaRecipeExecutionLimits::new(3,256,768,3))
    .expect("complete allowance").prepare_scalar_with_tangent_limits(&inputs,selected)
    .expect("actual selected policy");
let result = plan.evaluate(FormulaScalarExecutionContext::new(
    FormulaToleranceContext::empty(),FormulaSizeContext::empty())).expect("actual ordered recipe");
assert_eq!(result.plan().tangent_limits(),Some(selected));
assert_eq!(result.statements().len(),3);
# Ok::<(),Box<dyn std::error::Error>>(())
```

Tan30 returns577350ppm before the explicit division. The first RHS is therefore11547/20000ppm,
which binds once to1ppm. The second let reads that actual stored1ppm twice and produces2ppm.
Replaying the first unbound fraction would instead produce the wrong stored answer. Both real
bindings retain the successful Tan contribution;source lookup adds no other rounding boundary.

The ordered runtime uses the same selected policy for every direct call and across real Derived
suspension/resumption. Each actual call has its own native cumulative budget. Exact zero/unit
calls retain Tan even through stored false conditions and selected branches. T1 refuses actual
Tan dependencies after storage;T2 still needs its real quantitative context. Assertion operands
execute left then right,then the original header. A pole or resource refusal keeps its actual
operand role and wins before later operands/header lookup. No accepted statement prefix escapes.

Recipe plans,preparation errors and execution errors expose tangent_limits(),including source/
allocation errors,an earlier unrelated division and a successful native value's later binding
refusal. Native failures retain the original sourced input,selected limits,precision/work/cause
and the actual recipe call node. A finite113-bit Ratio may succeed numerically and still fail
only at the original signed storage boundary. Mathematical width and resource failures stay distinct.

Source-backed recipes retain two independently selected scopes. A Tan node authored in the recipe
uses recipe_plan().tangent_limits();a Tan node inside a supplier uses source_plan().tangent_limits().
The source graph may provide a full-width unbound Angle with limits0/0 when it executes no Tan,
while its recipe consumer uses8219/3584 to certify the actual near-midpoint Tan. A larger supplier
policy cannot enlarge a smaller recipe policy,and a larger recipe policy cannot enlarge a supplier's.
Both scopes survive every paired error through source_tangent_limits() and recipe_tangent_limits(),
including foreign-owner refusal before any runtime allocation or value. Context/owner equality,
private earned caches,once-only storage and atomic complete outcomes retain their existing contracts.

Eight focused recipe checks and six shared source checks exercise actual factories and kernels:
whole admission priority,false storage/T1/T2,once-only fractions,assertion failure order,later
native resources,wide native success/storage refusal,real source suspension with differing policies,
and paired source/foreign-owner failures. The common fixture builds only real canonical/source/
namespace/dependency/acyclic proofs;it supplies no computed values. Actual Design policy
selection and independent combined24-row tangent acceptance are available.
