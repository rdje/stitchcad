# Execute scalar construction recipes

> **Implemented with focused checks:** G1-SLICE.5e.2.5.1.2.2.2 executes the admitted available
> scalar subset from its actual complete static recipe proof. It evaluates real lets/assertions in
> declaration order,stores each successful binding once and retains actual executed source reads.
> Derived reads now execute through [actual source-backed Design recipes](formula-design-integration.md#execute-the-private-actual-design-plan).
> Explicit-policy direct/source-backed Tan recipes are available below. Geometry,corrupt-load
> validation,full replay/platform identity and complete language/bus/
> physical acceptance retain their separate owners. No release claim follows.

Start with complete [syntax/input normalization](formula-recipe-inputs.md) and
[whole static checking](formula-checked-recipes.md),then admit the real proof and prepare every
source before values. The [expression controller](formula-execution-plan.md#actual-scalar-expression-execution)
and recipe executor share one actual-node machine and existing exact sourced kernels.

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
namespace/dependency/acyclic proofs;it supplies no computed values. Integrated Design policy
selection and independent combined paired acceptance remain separately owned.
