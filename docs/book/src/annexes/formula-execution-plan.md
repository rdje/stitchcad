# Joining the checked formula runtime

This records the bounded implementation sequence for G1-SLICE.5e.2. Actual sourced scalar expression
and [scalar recipe execution](formula-recipe-execution.md) are available for the admitted subset. The worker
joins complete static node proofs, real canonical reads and explicit tolerance/size contexts with
existing exact kernels. Supplied values or callback annotations cannot replace an actual expression
proof. Derived/tangent/geometry integration,load/replay and full atomic acceptance remain owned work.

## Existing contracts to reuse

| Boundary | Available contract | Execution must retain |
| --- | --- | --- |
| Normalized expression | Private flat arena, original source spans and structural edges | Exact node owner; no substitute syntax or recursive destruction |
| Checked expression | Actual normalized owner, per-node kinds/roles and ordered sourced name uses | Both branches passed static checks before a value/provider read |
| Checked statement | Exact let/assert operand proofs and actual one-based recipe position | Original name/annotation/class and available canonical identity |
| Checked recipe | Complete immutable statement proofs in authored order | No accepted prefix after a later static error |
| Sourced values | Exact operators, implemented calls, stored reads and comparison outcomes | All executed inputs, condition/taken sources, actual failure data |

The source APIs are crates/sc-core/src/recipe/checked.rs,
crates/sc-core/src/recipe/checked_statement.rs, crates/sc-core/src/recipe/checked_recipe.rs and
crates/sc-core/src/recipe/normalized.rs. The static checker computes intermediate kinds on a heap
stack and now retains [actual per-node certificates](formula-checked-nodes.md) beside the expression,
root kind and dependencies. Runtime code can consume the selected branch's real node proof.

## Next implementation slices

1. **Actual per-node certificates — .5e.2.3, implemented0275.** Each successful node retains its inferred kind and
   direct symbolic tolerance role in the checked owner. Child views must stay pinned to that owner
   and its original spans. Neither a foreign arena nor a supplied index may forge a certificate.
   Name dependencies, wanted-kind alternatives and first static error ordering stay unchanged.
   No value, input state, tolerance provider or geometry query enters this static phase.
2. **Bounded sourced expression execution — .5e.2.4, in progress.** Complete-proof/workspace
   admission, source preparation and actual scalar execution are available below. Full family/recipe
   integration remains open. The worker consumes real node proofs and sources in authored order.
   Use the existing exact value kernels and real declaration-source adapters. A conditional checks
   both complete branch proofs, then evaluates only its condition and selected branch. All evaluated
   call operands contribute, including candidates or bounds whose numeric value is not returned.
   Errors retain the actual failing node, owning expression and available canonical identity.
3. **Whole declaration-order execution — .5e.2.5, in progress.** Whole-proof admission is available
   in the [recipe annex](formula-recipe-execution.md),including actual admitted scalar statements. Every let
   evaluates its RHS before its single numeric binding conversion; only a successful binding becomes
   visible later. Boolean bindings stay Boolean. Assertions use their actual class/name and refuse
   false comparisons. Statement ordinals include assertions and come from the real proof owner.
4. **Atomic acceptance and replay — .5e.4.** A later runtime failure yields no accepted partial
   recipe, geometry or replacement result. Dependency validation must cover persisted corruption and
   actual derived-input edges before execution can recurse. Fresh replay starts at statement one;
   cache equivalence and full accepted-result identity are integration obligations.

These are implementation boundaries within one runtime. They add no language syntax, execution
authority or exception to static-before-value validation. A detached expression scope has no recipe
ordinal; a whole statement supplies its real ordinal. Neither invents unavailable context.

## Complete proof and workspace admission

| Public API | Role |
| --- | --- |
| `FormulaExecutionLimits` | Explicit node and complete logical workspace allowances |
| `FormulaExecutionAdmission` | Borrowed complete proof and prospective workspace reservation |
| `FormulaExecutionAdmissionRefusal` | Exact refused allowance and measured population |
| `FormulaExecutionAdmissionError` | Refusal retaining the actual certified root/source extent |

`FormulaCheckedExpression::admit_execution` accepts explicit `FormulaExecutionLimits` and borrows
the complete successful proof. No raw node index, foreign expression or supplied name count enters
this API. The returned `FormulaExecutionAdmission` retains the actual certified root, original
expression/source spans and the exact ordered dependency slice. The admission cannot outlive its
proof; its fields are private. It performs no allocation, source clone, value read or provider call.

For N certified nodes and D name occurrences, the prospective iterative machine reserves:

| Logical population | Capacity |
| --- | --- |
| Pending task frames | N |
| Evaluated value slots | N |
| Executed-source-read records | D |
| Complete workspace | 2 × N + D |

All branches count before runtime selection. Repeated names count separately: dependencies are
individual source occurrences, not a deduplicated inventory or a trace of reads already performed.
This bounds logical records; it does not certify allocator bytes, CPU, numeric-kernel workspace,
provider resources, complete recipe memory or backend availability. The scalar worker below uses
these capacities. An admitted reservation alone is not an execution/result certificate.

```rust
use sc_core::recipe::{FormulaExecutionLimits, FormulaExpression, FormulaNamespace};

let expression = FormulaExpression::parse("if(is_base_size,1 mm,2 mm)")?
    .normalize_literals()?;
let proof = expression.check_kinds(&FormulaNamespace::new([])?).expect("static fixture");
let limits = FormulaExecutionLimits::new(4, 9);
let admitted = proof.admit_execution(limits).expect("exact limits");
assert_eq!(admitted.node_count(), 4);
assert_eq!(admitted.name_use_count(), 1);
assert_eq!(admitted.workspace_records(), 9);
assert!(std::ptr::eq(admitted.root().expression(), &expression));
assert!(std::ptr::eq(admitted.dependencies(), proof.dependencies()));
# Ok::<(), Box<dyn std::error::Error>>(())
```

The condition's value is absent here: static metadata suffices for admission. Likewise, an actual
Unknown length input can be admitted without observing its value or choosing a branch. Later
execution must still refuse a read of a taken Unknown and avoid reads in an untaken branch.

Node quota is checked first, then checked workspace arithmetic, then workspace quota. A zero
allowance is valid and refuses a nonempty proof; there is no Default. The typed
`FormulaExecutionAdmissionRefusal` distinguishes Nodes, WorkspaceCountOverflow and WorkspaceRecords.
`FormulaExecutionAdmissionError` keeps the actual certified root and original span. These resource
refusals introduce no canonical language diagnostic or invented statement ordinal. A caller cannot
obtain an admission from a failed static check or attach an equal-looking foreign expression.

The four native tests in crates/sc-core/tests/formula_execution_admission_contract.rs cover original
owners/spans, quota order, repeated/untaken Unknown sources and the complete 256-node arena. The
maximum fixture is min with 255 eps_num arguments: N=256, D=255, reservation=767. An allowance of
766 refuses; 767 admits all actual argument certificates. Private-proof arithmetic overflow is
unreachable under today's 256-node bound; its defensive refusal is not claimed as a native fixture.
Privacy/lifetime/no-default documentation checks accompany this API. Whole execution, independent
runtime result oracles and cross-platform identity remain the later owners below.

## Whole checked-recipe admission

[Scalar construction recipes](formula-recipe-execution.md#whole-checked-recipe-admission) now have
complete logical admission,actual whole source preparation and declaration-order execution.
The semantic recipe annex retains the full API contracts and examples.

## Source and provider integration

### Actual canonical inventory joins

| Public API | Role |
| --- | --- |
| `FormulaCanonicalInput` | Sealed metadata-to-inventory join before an authored read |
| `FormulaCanonicalInputAdapter` | Inspect the selected existing dedicated/general adapter |
| `FormulaCanonicalInputRefusal` | Actual missing family, owner or scalar adapter cause |
| `FormulaCanonicalInputError` | Refusal retaining original metadata and supplied inventory |

`FormulaCanonicalInput::resolve(metadata, inputs)` matches the actual declaration source against
the supplied CanonicalInputs. It performs no allocation, clone, numeric read or name-only search.
Dedicated LengthInput metadata must borrow the exact record in that inventory. An independently
owned equal record or an equal inventory clone cannot replace that borrow. General Input metadata
carries a target id/kind instead: resolution selects the actual scalar in the supplied inventory
and reuses the existing scalar identity/kind adapter. A general Length-kind scalar remains general.

```rust
use sc_core::{name::MachineToken, ontology::EntityId, recipe::*, value::*};

let name = MachineToken::new("body_width")?;
let inputs = CanonicalInputs::new(vec![LengthDeclaration::new(LengthDeclarationDefinition {
    id: EntityId::from_bits(1), source: EntityId::from_bits(2),
    state: LengthState::Unknown { observation: EntityId::from_bits(3) },
})?], vec![])?;
let actual = inputs.length(EntityId::from_bits(1)).expect("owned dedicated length");
let metadata = FormulaDeclaration::length_input(&name, FormulaInputOrigin::Measurement,
    EntityId::from_bits(4), actual);
let joined = FormulaCanonicalInput::resolve(metadata, &inputs).expect("actual inventory owner");
assert!(std::ptr::eq(joined.inputs(), &inputs));
if let FormulaCanonicalInputAdapter::Length(input) = joined.adapter() {
    assert!(std::ptr::eq(input.canonical_declaration(), actual));
    assert!(input.read_authored().is_err()); // Still requires the actual observation.
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

Unknown/Derived records match without being evaluated. An explicit later authored read keeps its
actual RequiresObservation/RequiresEvaluation request; present zero and false remain present.
Known/Assumed/Preference provenance is preserved by the existing selected adapter. Inspecting the
adapter enum alone is not an inventory certificate: the private joined wrapper retains membership.

Source family precedes lookup. A missing dedicated target reports MissingLength and the original
scalar index if the id belongs to that other family. MissingScalar similarly retains an actual
dedicated length index. Absence from both families keeps that position absent. A length found by
id still requires exact pointer ownership. Scalar kind mismatch retains the existing borrowed
adapter error and actual canonical candidate; missing target precedes kind checking.

The error retains the original metadata and immutable supplied inventory once. Its explicit
refusal view reconstructs the full witness, including the same typed scalar cause from those
unchanged inputs. Debug omits authored names/content. A nested borrowed cause is inspectable
through refusal; it is not erased into a static standard-error source. Reserved names, geometry
and prior-let sources return NotCanonicalInput, which is an adapter scope refusal, not a formula
language diagnostic. Their actual runtime owners remain separate.

This join proves membership only in the supplied canonical component. It proves no containing
Design/revision, exposing metadata ownership, evidence registry/truth, resource envelope, source
currentness or execution. In particular, identity-only scalar metadata can select a record in an
equal inventory copy; this explicitly establishes membership there, not in another Design.
The five native tests use actual inventories and checked-name declarations, all six scalar kinds
and three origins, dedicated origins, unchanged Unknown/Derived requests, false/zero reads,
equal-copy owner refusals, family positions and existing scalar causes. Whole expression execution,
source-order read traces and real geometry/derived dependencies remain subsequent implementation.

### Prepared scalar source plan

| Public API | Role |
| --- | --- |
| `FormulaScalarExecutionPlan` | Complete actual source binding for the available scalar subset |
| `FormulaScalarExecutionSource` | One original certified name occurrence and its bound source |
| `FormulaScalarExecutionSourceKind` | Actual canonical inventory join or reserved-name role |
| `FormulaScalarExecutionPlanRefusal` | Preparation capability/source/allocation refusal |
| `FormulaScalarExecutionPlanError` | Actual failed node, immutable inventory and original cause |

Consume a complete workspace admission with prepare_scalar(inputs). The prepared plan retains
that proof/reservation and the actual immutable CanonicalInputs. It binds every name occurrence,
including repetitions and both branches, before any value or provider read. Preparing a plan
computes no value; the separate evaluate operation below executes it. Whole-runtime acceptance
remains pending.

```rust
use sc_core::{recipe::*, value::CanonicalInputs};

let inputs = CanonicalInputs::new(vec![], vec![])?;
let expression = FormulaExpression::parse("if(1 mm<2 mm,1 mm,eps_phys)")?
    .normalize_literals()?;
let proof = expression.check_kinds(&FormulaNamespace::new([])?).expect("static fixture");
let plan = proof.admit_execution(FormulaExecutionLimits::new(6, 13))
    .expect("exact allowance").prepare_scalar(&inputs).expect("scalar scope");
assert!(std::ptr::eq(plan.inputs(), &inputs));
assert_eq!(plan.sources().len(), 1);
// eps_phys retains its reserved role; no physical provider is requested during preparation.
# Ok::<(), Box<dyn std::error::Error>>(())
```

The allocation-free capability scan uses private same-proof node views in certified arena order.
It accepts the scalar shapes and available numeric/within kernels. Tangent, geometry sources/
selectors and prior-let sources retain explicit unavailable implementation scope. These preparation
gaps precede source binding and are not formula_unsupported or another language diagnostic. They
cannot produce a canonical computed result. An untaken unsupported family still lies outside this
prepared subset; this is a capability boundary, not whole-language conditional semantics.

After that scan, fallible reservation creates D prospective source records within the admitted
2N+D allowance. Private certificate indices locate each actual name occurrence; no foreign or
caller-supplied index is accepted. Actual inventory joins then run in original dependency order.
sources() is an exact double-ended view in that order, including all repeated/untaken occurrences.
Canonical records are bound through the sealed join above. Reserved entries retain their exact
roles without fetching size/tolerance values or checking optional provider availability.

A missing canonical target is invalid source metadata, even in an untaken branch: preparation
refuses before execution. An existing Unknown/Derived record is valid source metadata and remains
unread. Likewise, an absent optional context supplies no default value and raises no value refusal
here; later execution must read only a taken source. The example above intentionally provides no
physical context. Whole Design/currentness/evidence truth remain their existing owners.

Errors retain the actual failed name/call node and expression owner; allocation errors retain the
actual root and requested D-record population. Canonical join details reconstruct the same cause
from immutable original metadata/inventory. The allocator's original error is separately inspectable
and remains the standard-error source. No partial source plan escapes. No allocator failure is
claimed as a native fixture: D is bounded by the real 256-node proof; reservation is fallible.

Five native tests cover available scalar forms, actual Unknown borrows, original reserved roles,
late untaken missing targets, unavailable capability priority and all 256 nodes/255 name occurrences.
The full fixture retains the admitted 767-record logical bound and all 255 prospective source records.
Privacy/lifetime controls accompany the API. Actual evaluation is described next.

## Prepare every whole-recipe scalar source

The [whole source plan](formula-recipe-execution.md#prepare-every-whole-recipe-scalar-source)
binds actual canonical/reserved/prior-let suppliers before values. Its private runtime prefix
executes real statements;it accepts no caller-supplied results or annotations.

## Actual scalar expression execution

| Public API | Role |
| --- | --- |
| `FormulaScalarExecutionContext` | Explicit immutable tolerance/size slots; no input replacement or implicit defaults |
| `FormulaExecutedScalarExpression` | Actual unbound scalar result, original plan/owner and executed reads |
| `FormulaScalarExecutionRead` | One actual selected name node/value and original source witness |
| `FormulaScalarExecutionReadKind` | Dedicated/general canonical read, quantitative tolerance or supplied size value |
| `FormulaScalarExecutionWorkspace` | N-task or N-value logical machine population |
| `FormulaScalarExecutionFailure` | Original numeric/input/provider/comparison or reservation cause |
| `FormulaScalarExecutionErrorCause` | Original allocator failure or boxed full value failure |
| `FormulaScalarExecutionError` | Actual failing certified node and complete typed cause |

The prepared plan now consumes evaluate(context) and returns a sealed actual scalar result. Its
iterative worker executes the original checked nodes. It does not call a supplied-value callback,
reparse source or accept a supplied branch kind. A conditional evaluates its real condition, then
only the selected actual branch proof. Both complete branches already passed static checking and
source preparation. Root/child spans retain their original expression owner and authored locations.

```rust
use sc_core::{recipe::*, value::CanonicalInputs};

let inputs = CanonicalInputs::new(vec![], vec![])?;
let expression = FormulaExpression::parse("if(1 mm<2 mm,1 mm/3,eps_phys)")?
    .normalize_literals()?;
let proof = expression.check_kinds(&FormulaNamespace::new([])?).expect("static fixture");
let plan = proof.admit_execution(FormulaExecutionLimits::new(256, 768))
    .expect("explicit envelope").prepare_scalar(&inputs).expect("scalar scope");
let result = plan.evaluate(FormulaScalarExecutionContext::new(
    FormulaToleranceContext::empty(), FormulaSizeContext::empty())).expect("taken exact branch");
assert_eq!(result.value().numeric().expect("Length").rational(),
    FormulaRational::new(1000, 3, false)?);
assert_eq!(result.plan().sources().len(), 1); // Prospective eps_phys source.
assert_eq!(result.reads().count(), 0);        // Untaken: no provider read.
assert!(std::ptr::eq(result.root().expression(), &expression));
# Ok::<(), Box<dyn std::error::Error>>(())
```

This result remains the exact unbound fraction 1000/3µm. No let binding/storage rounding has occurred.
Canonical scalar reads import their stored integers exactly, preserving negative raw angle turns,
signed ratios, zero and false. Count expression arithmetic may exceed i64 storage while staying
inside its actual nonnegative/128-bit numeric domain. Binding storage checks remain the statement
executor's responsibility. Unknown requires its actual observation; Derived requires its sole
formula's evaluation. Neither becomes zero, false, a cached value or a new language refusal.

| Executed source | Actual retained witness |
| --- | --- |
| Dedicated length | Original canonical borrow/state, including Known/Assumed/Preference labels |
| General scalar | Actual canonical family/kind/state, including Boolean false and signed values |
| Tolerance | Original once-read class/limit/derivation and provider borrow/source or intrinsic |
| Size | Actual supplied pinned size-role frame/value; derive it through the canonical membership adapter |

reads() contains only name occurrences actually executed, in authored order with repetitions.
The complete prospective sources remain inspectable separately through plan().sources(). The
worker fills existing D-source slots in place; it does not clone records or allocate another read
trace. The result, context, read and failure Debug views omit authored names/value content; explicit
value/source/cause inspection remains available. Actual inventory borrows and provider lifetimes
cannot be escaped through the result.

All available scalar calls dispatch to their existing sourced numeric kernels. Every evaluated
operand contributes, including an unselected min/max candidate or clamp bound. Conditional results
retain the condition's approximation sources plus the taken branch's sources. A within call uses
the original quantitative tolerance read from its actual direct reserved-name operand; it does not
query that provider again or accept a caller's replacement class annotation.

| Expression | Actual result or refusal |
| --- | --- |
| `0.0004 mm+0.0004 mm` | 0µm: each literal rounds once before addition |
| `1 mm/3` | Exact 1000/3µm, without a binding conversion |
| `round_to(3 um,2 um)` | 4µm, retaining the round_to contribution |
| `if(sin(0 deg)==0 pct,1 mm,2 mm)` | 1mm, retaining the condition's sin contribution |
| `min(0 mm,round_to(1 mm,1 um))` | 0mm, retaining the evaluated second operand's round_to source |
| `within(sqrt((2 mm)^2),2 mm,eps_num)` | Actual T1 approximation refusal, retaining the computed values and sqrt source |

Before any value/provider read, both N task/value buffers reserve fallibly. Every push checks its
admitted logical population, so the vectors cannot silently grow past N. This implements the
2N+D machine-buffer reservation; it does not bound allocator bytes, existing kernel temporary
allocations, boxed error witnesses, provider resources or CPU. Allocation failures retain their
original allocator error without requiring another error box. A runtime refusal retains its actual
failed name/operator/call node and complete original borrowed cause; no accepted result/read prefix
escapes and canonical inputs remain unchanged. No statement ordinal or artifact-policy context is
invented. Whole recipe/publication atomicity remains .5e.4 work.

Focused evidence is nine native core tests, one real sc-measure membership integration test and
the execution documentation controls. The integration derives all size values through the actual
SizeMembership::formula_context adapter for three authored members, then verifies taken-only reads,
original set/revision/member/base sources and unchanged membership. The full 256-node fanout reads
all 255 original eps_num occurrences under the exact 767-record reservation; a left-deep 256-node
expression also completes without recursive execution frames. The scoped independent result
checkpoint below follows feature completion. Whole worked-book,
platform identity,tangent/geometry/Derived evaluation and replay keep their
existing .5g/.5e.4/parent owners. This scoped expression result grants no whole-language, containing
Design/currentness, evidence-truth, geometry/physical or release approval.

### Finite independent result checkpoint

Checkpoint0580 compares actual public AST execution with the separately parsed, table-driven book
reference. Its guarded pure-stdout producer is
docs/tasks/artifacts/formula_structure/scalar_execution_oracle.py. It imports no Rust implementation
and reads no expected fixture. The frozen native fixture is
crates/sc-core/tests/fixtures/formula_scalar_execution.tsv;two native watchers compare every actual
row and independently regenerate its complete bytes.

The finite population contains49 rows,including10 intended runtime refusals. It checks exact
unbound values,literal conversion/explicit rounding,Boolean outcomes,all available scalar call
families,executed approximation contributions and original name-read order. Cases retain repeated
reads,condition plus chosen-branch contributions,multiple nested call sources,unselected evaluated
operands,untaken Unknown/absent contexts and actual first-failure families. Both watchers pass;
the2325-byte fixture is independently reproduced exactly. Existing implementation sources are
unchanged from0579. This closes the prepared scalar subset's result checkpoint,not whole-language,
all-input numerical,worked-book,platform,geometry/physical or release verification.

D338 was discovered during this checkpoint and repaired0581:the reference had classified missing
Derived and other non-Unknown external values as formula_unknown. They now raise a separate
reference input-context requirement with no canonical language token or invented formula identity.
Actual Unknown diagnostics and available computed bindings remain intact. Rust already preserves
its sole formula's RequiresEvaluation request. Full Derived-resolution verdicts remain outside this
finite checkpoint until their actual dependency executor exists;the repaired boundary supplies
no substitute value,cache or whole-language acceptance.

Name reads use the actual resolved declaration, not a fresh name-only search that could find another
source. Canonical length/general scalar adapters retain source/state and return observation or
dependency-evaluation requirements. Known/Assumed/Preference values remain labelled beside their
numeric payloads. Derived inputs follow their sole authored formula; they do not become separately
authored or unlabelled cached values. Registry/currentness and evidence truth remain their existing
owners; a readable value is not approval or evidence-scope validation.

Reserved tolerances use the explicit context and its actual quantitative provider. Optional classes
remain absent until supplied. Size values come from actual pinned membership/order/base identity;
axes remain .4c.2 under D70's confirmed minimum-one rule. Static availability metadata never supplies a missing runtime value.

Geometry reads retain real point/edge references and the published operation/selector contracts
under .5f.3. Point coordinates need separate approximation-source sets. Direction normalization is
specific to dir; raw angle arithmetic and inverse principal answers keep their signed values. The
parameter/geometry error budget uses actual edge length, not an inferred bounding-box guarantee.

The controller now uses one shared private actual-node machine, prepared for whole-recipe reuse.
Its source interface is internal to the engine: clients cannot supply AST values or callback
annotations. It reserves task/value scratch fallibly before reads, guards every push against the
actual operand population and leaves both buffers empty after a successful operand. A native test
executes an actual conditional fraction followed by within using the same scratch addresses;
insufficient capacity refuses before any source read. The existing binding3/sourced-value5/controller11
tests, including49 independent reference rows and exact frozen bytes, pass after this refactor.
Actual admitted scalar recipes now consume this shared machine through a private earned binding prefix.

## Dependencies that still prevent full runtime closure

The numerical kernels cover rational arithmetic,quantization,roots,arc_length,certified sin/cos
and [native atan/atan2](formula-trig-certificates.md#native-inverse-values). The inverse kernels
and their admitted all-input plan were completed in .5d.3.3/.5d.3.3.1;source-bearing scalar wrappers
are also available. D337 corrected this plan's stale pre0288 inverse-availability statement.

Tangent remains unimplemented. Its [bounded adaptive resource strategy](formula-trig-certificates.md#selected-bounded-adaptive-tangent-admission)
now has convergent enclosures and explicit caller bit/work admission;the .5d.3.2.2 kernel remains
pending. Resource refusal produces no numerical value or new language diagnostic. This plan
selects no replacement math backend,precision cutoff or caller placeholder. An admitted function with an
unfinished implementation cannot be reclassified as a language refusal or published as a
canonical computed result.

Full-family source/Derived/geometry integration and statement/atomic publication
remain open. Admitted scalar statements are available in the recipe annex. Internal slices must state their supported scope. Public whole-runtime
acceptance remains pending until every required family has its real implementation and evidence.
API/MCP publication, artifact authority, browser execution and production approval retain their
separate roadmap owners.

## Verification during implementation

Each implementation slice gets one focused check of its changed behavior. Node certificates need
actual-owner/span/kind/role and foreign-node controls. Expression execution needs actual checked
branches, taken-only unknowns, operation order and sourced failures. Statement execution needs
once-rounded later reads, real assertion data and actual ordinals. These checks cannot be replaced
by a reference evaluator or a supplied-value callback demonstration.

Whole worked-book population, independent runtime oracles, replay/cache fault controls, geometry
integration and observed cross-platform byte identity remain .5g/.5e.4 milestones. Broad campaigns
and full CI follow the feature-delivery policy and required pre-push gate. Passing a focused slice
does not close those later obligations or the G1 gate.
