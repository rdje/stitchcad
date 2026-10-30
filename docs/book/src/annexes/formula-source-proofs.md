# Formula source validation

> **Status:** complete initial-scope expression proofs,actual canonical dependency joins and Derived
> consumer-kind checks and canonical Derived dependencies have focused verification. Bounded cycle
> admission and lazy available-scalar source execution have scoped independent acceptance.
> Actual Design pairing has23 independent scalar cases;full resource/backend/platform/physical proof remains pending.

[Native source-backed recipes](formula-design-integration.md#execute-an-admitted-recipe-with-actual-source-plans)
now combine the admitted graph with real ordered storage/read/error views;actual Design execution re-admits its root.

Start with the [original source owners and normalized collections](formula-recipe-inputs.md#explicit-formula-source-owners).
FormulaNormalizedSources.check_sources(&namespace, limits) checks the complete supplied source
inventory through the existing expression kind factory. Every source,including an unused one,
receives its original proof. All branches and repeated name occurrences participate. The supplied
FormulaNamespace is the existing immutable initial scope;its constructor rejects prior-let seeds.
This API supplies no fictitious statement/header/name or new binding/rounding boundary.

For actual containing Design roots,start with the [admitted document scope](formula-design-integration.md).
Its held-content/current checks and normalization precede its actual-namespace source/recipe kind
factories. These preserve the original static rules below;Design-backed graph/runtime joins follow.

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

## Prepare original canonical Derived dependencies

FormulaCheckedSources.prepare_dependencies(limits) admits the complete prospective canonical
Derived relation before allocating its edge/offset vectors. Every checked name occurrence whose
actual canonical state is Derived becomes a distinct consumer-expression→supplier-source edge.
It uses that state's formula field,never the separate source/provenance field,a parameter id,
canonical text,label or list position. The original private source index selects the exact supplier.

| Public API | Meaning |
| --- | --- |
| `FormulaSourceDependencies` | Complete admitted canonical Derived relation and original source proofs |
| `FormulaSourceDependency` | Original consumer/supplier proofs,name occurrence and canonical adapter |
| `FormulaSourceDependencyLimits` | Explicit complete source,edge and retained-record allowances |
| `FormulaSourceDependencyError` | Actual source-proof owner,selected limits and complete refusal |
| `FormulaSourceDependencyRefusal` | Source/edge/retained quota,overflow or original vector allocation cause |

Every repeated or untaken Derived name use remains a separate edge in original source/dependency
order. All source vertices remain,including unused/constant sources. Known,Assumed,Preference and
Unknown records create no Derived edge;their original values/state/provenance remain untouched.
Reserved and geometry metadata are not invented canonical Derived suppliers. Geometry/operation
dependencies still need their own complete graph. This relation is prospective,not executed reads.

Let E be the complete actual edge population and S the source count. Admission checks S,checked
complete edge/offset arithmetic,E and S+E+1 retained records before either allocation. Each logical
edge record retains original indices;S+1 source offsets include a terminal sentinel. The empty
relation therefore requires one retained record,not zero. These are retained logical populations,
not allocator bytes/capacity,CPU,existing proof storage or future cycle-check scratch.

Only after complete admission are the S+1 offsets and E edges fallibly reserved. Source/body/state/
index data are not cloned. OffsetAllocation and EdgeAllocation preserve their actual TryReserveError
and standard source chain;quotas retain complete measured/selected populations without guessing a
failing source or statement. Failure returns no accepted edge/offset prefix. Policy is not stored
canonical content.

edges() lends all edges in original order and supports reverse iteration. outgoing(source_id)
lends that exact source's grouped edges,including Some(empty) for a constant source and None for
an absent source id. A canonical declaration id does not impersonate an expression entity.
consumer()/supplier() return actual source proofs;name_use() returns the original occurrence and
span;canonical_input() returns its existing actual inventory certificate without reading a value.
Views borrow original source-proof owners and copy only original indices,never canonical state.

For example,source9001 reading a+a from dedicated declaration10 Derived{formula:9002} retains two
separate edges9001→9002 with distinct a spans. A declaration.source of9001 does not redirect either
edge. General Count reads n+n similarly retain both actual scalar adapters and supplier source.
Known zero,Preference zero,Assumed and Unknown uses beside them create no extra edges or fallbacks.
An unused source containing tan(1 deg) remains a vertex without requiring its numeric kernel.

This phase deliberately retains cyclic relations for required cycle admission. For sources9001
reading b and9002 reading a,with actual Derived bindings a→9001 and b→9002,the relation contains
both9001→9002 and9002→9001. An unused self-reference remains present too. Preparing that relation
supplies no acyclic proof,loader formula_cycle context,execution order,actual let/binding/rounding,
currentness or computed result. Separate cycle admission below is required before values;
actual authored binding and full graph/runtime integration retain their own tasks.

Three native contracts check both canonical families,actual pointer/endpoints/spans,repeated and
untaken uses,all four non-Derived states/reserved inputs,unused/constant vertices,complete quotas,
cyclic/self relations and the empty terminal offset. Six existing source-proof contracts remain
green. Three compiler guards reject default limits,private edge mutation and source-proof lifetime
escape. Broader graph/runtime/platform acceptance remains deferred.

## Admit the complete canonical Derived relation as acyclic

FormulaSourceDependencies.admit_acyclic(limits) checks every actual source and original outgoing
edge over the same immutable relation. FormulaSourceCycleLimits selects a complete source allowance
and 2S logical scratch records:one color and one active frame per source. Source quota,checked
scratch arithmetic and complete workspace allowance precede both full fallible reservations.
No default limits exist. An empty relation needs zero cycle scratch;its previously retained offset
sentinel still belongs to dependency preparation.

| Public API | Meaning |
| --- | --- |
| `FormulaAcyclicSourceDependencies` | Complete acyclic canonical Derived relation over actual owners |
| `FormulaSourceCycleLimits` | Explicit source and complete 2S cycle-scratch allowances |
| `FormulaSourceCycleError` | Actual relation,selected limits and original refusal |
| `FormulaSourceCycleRefusal` | Source/scratch/overflow/allocation or concrete original cycle |
| `FormulaSourceCycle` | Actual relation,original active path and closing edge witness |

The walk uses an explicit heap frame stack and white/gray/black colors,with no Rust recursion.
An edge to a gray active ancestor closes a cycle;an edge to an already finished black vertex is
valid sharing,such as a diamond. Every source root participates,including an unused component.
Every repeated or untaken edge participates too. Original source/outgoing occurrence order selects
the first concrete cycle. No topological or recipe execution order is produced.

On success,the proof borrows exactly the admitted dependency owner;dependencies()/limits()/
workspace_records() retain its identity and selected/measured scratch allowance. Both scratch
vectors drop. Private fields prevent substituting a different relation. No source/body/state or
existing edge/offset vector is cloned. The proof cannot outlive the actual relation it certifies.

On failure,Cycle retains the real active-frame vector and actual closing edge index. It allocates
no separate path or graph copy. steps() lends original directed edges from the gray ancestor through
the closing edge;step_count() excludes any earlier traversal prefix. Each step retains actual
consumer/supplier proofs,canonical adapter and original name/span. For0→1→2→3→1,the witness is
1→2→3→1,not the unrelated0→1 prefix. A self-reference has one original closing edge.

Cycle refusal returns no accepted acyclic prefix or repaired graph. ColorAllocation and
FrameAllocation preserve their original TryReserveError inline and in the standard source chain.
Source/workspace refusals retain complete required/allowed populations. Cycle is a component
witness,not an invented persisted-loader formula_cycle token or statement ordinal. The actual
loader must retain its own real context when applying its canonical refusal contract.

These are logical color/frame populations,not allocator bytes/capacity,CPU,existing proof/edge
storage or the full operation/geometry/runtime envelope. Acyclicity covers the canonical Derived
relation prepared here;it proves no whole operation graph,current Design/host namespace,provider
availability,actual ordered let/binding rounding or computed value. No execution result is created.
The actual authored binding and runtime integration remain .derived.3.3.3 and existing owners.

Four native contracts compare actual verdicts and every returned original cycle edge/span/record
against nine curated graphs:empty,constant,chain,diamond,repeated edge,self,pair,unused later cycle
and cycle after a prefix. A separate graphlib.TopologicalSorter producer independently judges the
tracked cases;the native target invokes it as a fixture watcher. Actual false-branch cycles remain
visible,complete quota precedence is checked,and the cycle walk for a512-source chain succeeds
with a64KiB requested thread stack. Fixtures are prepared outside that worker;original endpoints/
spans remain
exactly unchanged. Existing three dependency contracts retain original
edge classification/order and empty-sentinel behavior. Three compiler guards reject default limits,
private relation substitution and a certificate escaping its actual dependency owner.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/source_cycle_reference.py
cargo test -p sc-core --test formula_source_cycle_contract
```

This is focused canonical Derived cycle verification. Full formula language/backend/resource,
load/replay,current Design/geometry/operation integration,platform identity and physical acceptance
remain pending. In particular,a successful cycle guard does not evaluate or round a source.

## Prepare available scalar source execution

FormulaAcyclicSourceDependencies.prepare_scalar_sources(limits) constructs a complete
FormulaScalarSourceGraphPlan through the existing per-expression admission/source factories.
It does not evaluate a source,read a context provider or publish a value. The separate runtime
method below earns source results;actual ordered-recipe/Design integration remains required.

| Public API | Meaning |
| --- | --- |
| `FormulaScalarSourceGraphPlan` | Complete available scalar source plans and actual acyclic owner |
| `FormulaScalarSourceGraphSource` | Actual original source proof and its existing scalar source plan |
| `FormulaSourceGraphExecutionLimits` | Explicit complete source/node/name/workspace/output allowances |
| `FormulaScalarSourceGraphPlanError` | Actual graph/source/input context,selected limits and refusal |
| `FormulaScalarSourceGraphPlanRefusal` | Complete quota,unavailable original node,allocation or factory cause |

For S sources,N actual nodes and D name uses,the prospective runtime workspace allowance is
2S+2N+D:source control records,graph call frames,task/value buffers and original read slots.
A separate output quota must fit at most S actual source results. All unused sources,branches and
repeated names participate. These are logical prospective allowances,not allocated runtime values,
allocator bytes/capacity,numeric backend scratch,CPU or the full containing resource envelope.

Preparation checks source count,checked complete arithmetic,node/name/output/workspace quotas,
then scans every original source for scalar implementation availability. Only after that complete
scan can the outer source vector and per-expression source slots be allocated/prepared. It reuses
the same exhaustive availability catalog as existing scalar execution;there is no second builtin
list to drift. Legacy unused Tan calls report MissingTangentPolicy;geometry gaps remain UnavailableBuiltin/UnavailableSource or
UnavailableRootKind with actual source/node witnesses. They do not become language errors.

Every source then passes its actual existing execution admission and canonical/reserved source
preparation,using the same original proof and canonical inventory. Sources remain in original
order. source()/plan() lend real source proofs/plans;graph()/limits()/node_count()/name_use_count()/
workspace_records()/output_records() retain actual owners and complete selected/measured quotas.
They expose no caller-seeded value,completed source cache or fake statement/header.

Outer and nested read-slot allocation causes remain inline in Allocation/SourceAllocation and the
standard source chain. Nested reservation errors are moved out of their original source-plan
wrapper without allocating another diagnostic box. Original semantic admission/preparation errors
retain their typed borrowed witnesses in compact boxes. No accepted source-plan prefix escapes.

Four native contracts check an actual acyclic two-source relation,original proof/arena/canonical
pointers and unchanged Derived requirements. S2/N6/D1 reserves17 workspace records and2 outputs.
All five quota boundaries,late unused tangent's real call/span and zero empty allowances are
covered. Existing five scalar-plan contracts and three compiler guards remain green. These checks
certify preparation;they do not certify graph execution or full backend/platform integration.

## Execute an actual selected scalar source

FormulaScalarSourceGraphPlan.evaluate_source(source_id,context) consumes the complete actual plan.
The id must exist in the original source registry;canonical declaration ids,parameter ids,text
aliases and nearby positions supply no fallback. MissingSource retains the actual graph,limits
and requested id before runtime allocation. Even an empty graph has no valid entry.

| Public API | Meaning |
| --- | --- |
| `FormulaExecutedScalarSourceGraph` | Actual selected entry and demanded suppliers,with private earned roots |
| `FormulaExecutedScalarSource` | Borrowed completed source,unbound value,ordinal and selected reads |
| `FormulaExecutedDerivedSourceRead` | Actual canonical Derived join and successfully executed supplier proof |
| `FormulaScalarSourceGraphRead` | Original executed name,value and direct/Derived witness |
| `FormulaScalarSourceGraphReadKind` | Existing direct authored/provider witness or actual computed Derived witness |
| `FormulaScalarSourceGraphWorkspace` | Source-control,call-frame,output,task or value population |
| `FormulaScalarSourceGraphExecutionError` | Actual graph,limits,entry and original complete refusal |
| `FormulaScalarSourceGraphExecutionCause` | Missing selection,inline allocator,original scalar or frame cause |

All S source controls,S call frames,S initially empty output slots and every source's N task/N
value buffers reserve fallibly before the first literal,value or provider read. The prepared D
slots retain actual successful name reads. Thus the admitted 2S+2N+D logical workspace and S output
allowance now cover actual runtime buffers. These remain logical populations,not allocator bytes/
capacity,numeric backend scratch,CPU or a full Design/resource certificate.

The iterative walk starts only the selected entry. An unresolved actual canonical Derived name
suspends its consumer with original operands/context intact. Its sole formula identity selects the
actual supplier. Only successful supplier-root completion fills that source's private output slot;
the consumer then resumes that exact name. Repeated reads and multiple consumers reuse this actual
unbound result. No caller callback,value seed,authored-state replacement,source-edge storage round
or Rust recursion is involved. Untaken branches and unused bodies are not evaluated;complete
static/capability/cycle admission still covered them before this lazy runtime phase.

For example,source a containing1 mm/3 and source b containing a*6,used by a+b+a,yield8000/3 µm.
The shared a finishes once with1000/3 µm,b once with2000 µm,and the consumer finishes third. All
three name occurrences retain distinct read records and their actual dedicated/general canonical
joins. Unrelated eps_phys or Unknown sources stay unexecuted. A false Boolean source
cos(0 deg)==0.0 selects the false branch of if(flag,n+n,3/2.0);n stays unused,the result remains
unbound Count3/2 and the condition's cos contribution survives. A direct Count source1/2.0 likewise
returns1/2 until a real authored let requires storage.

plan()/context()/entry() retain the complete original plan,pinned contexts and successful selection.
source(id) returns only a completed source,or None for an absent/unused source. sources() lends
completed sources in original registry order;completion_ordinal() separately records actual
one-based completion order. Each value() is unbound. reads() lends selected name occurrences in
execution order,including repetitions. Direct retains existing authored/tolerance/size witnesses.
Derived retains the real inventory join and exact successfully executed supplier proof. Private
constructors prevent turning a supplied value or metadata annotation into an executed trace.

Any source failure drops the whole private cache/output prefix and returns no accepted partial
result. Evaluation keeps the original source position and scalar error with actual failing node,
canonical observation requirement,quantitative provider or numeric cause. A supplier containing
eps_phys fails at that supplier's original name,not an invented entry statement. Within consumes
its real direct class:an executed cos contribution forbids T1 even if its numeric term cancels to
zero. Allocation failures retain their original inline TryReserveError without another error-box
allocation. The private call-frame guard forbids growth beyond S.

Six implementation contracts cover sharing,mixed canonical families,repeated reads,lazy missing
sources,false conditions,fractional roots,approximation/class propagation,original nested failures,
absent/empty selection and an actual512-source chain on a thread requesting64 KiB stack. Two
independent-reference watchers below bring this native target to eight passing contracts. Whole
recipe/Design/currentness,geometry,load/replay,backend/platform and physical proof remain open.

## Independent initial-source acceptance

The existing independently parsed,table-driven book reference evaluates27 curated source
collections:19 successes and8 original refusals. Its separate recursive demand walk calls the real
expression evaluator for suppliers and caches only actual returned unbound roots. The declared
canonical environment stays unchanged. Actual ReferenceInputRequired requests identify Derived
demand;they do not become formula_unknown or invented language tokens. A storage trap rejects any
attempt to treat a bare source as an authored let. Standard-library graphlib separately admits
all static source/branch dependencies before values. No Rust implementation,native output or
frozen expected file is read by this producer.

The native comparison checks every completed root,scalar kind/exact value/contribution set,
actual completion ordinal and each successful read's name,canonical target,direct/Derived role,
supplier identity and value. Error cases compare the original failing source and language token;
no accepted partial outcome/read trace is substituted. All canonical states/provenance remain
unchanged after success or failure. A second native watcher reruns the producer and compares all
5350 frozen fixture bytes,including the original inputs. Both watchers run in the ordinary native
target;adding,removing or changing a reference row cannot silently escape its frozen fixture.

Cases cover all six scalar kinds,mixed dedicated/general Length,shared/diamond/alias suppliers,
conditional/operand order,lazy unused Unknown/optional providers,false preferences,all five state
labels,explicit direct tolerances and symbolic approximation guards. A Count source
9223372036854775807*2 returns18446744073709551614 unbound;its consumer n/2.0 returns9223372036854775807.
Source lookup must not first attempt i64 let storage. Negative angle sources retain raw turns;
Area and Ratio sources retain exact arithmetic. Unknown-before-division and division-before-supplier
cases verify the actual first failure. Numeric domains and the existing rational width remain in
force. These results do not approve containing Design/recipe integration or platform identity.

The tracked input population is docs/tasks/artifacts/formula_structure/source_execution_cases.tsv.
The producer is docs/tasks/artifacts/formula_structure/scalar_source_execution_oracle.py;the
frozen result is crates/sc-core/tests/fixtures/formula_scalar_source_graph_execution.tsv.

```sh
cargo test -p sc-core --test formula_scalar_source_graph_execution_contract
python3 -I -B docs/tasks/artifacts/formula_structure/scalar_source_execution_oracle.py
```

## Containing Design source ownership

DesignInputsDefinition now requires an explicit owned FormulaSources registry. The actual root
census includes each expression entity,including unused ones,and refuses collisions with Design,
canonical declarations,Pieces,copies,creating operations,parameters,Materials or higher metadata.
Current validation reuses the complete original canonical/source join:every Derived formula
reference must select its real owned expression,without using a provenance field as a substitute.
Unknown/observation and other authored-state references keep their separate roles. Source bodies,
values and graph execution are not fabricated by this ownership check.

The [Design input contract](../spec/design-inputs.md#owned-core-input-composition) documents the
required field,owned typed refusals,original census order and immutable source access. Exact clones,
resource traversal and candidate context comparisons preserve source/index/original/canonical
content. Source92 in the authored Measurement fixture has a real waist-plus40 mm expression,while
source9003 in the structural fixture is distinct from actual operation1000. Both remain canonical
Derived states until real expression execution. Current root ownership/reference checks pass;
Design-backed runtime proof joins,actual ordered-recipe Derived reads and whole integration
acceptance remain G1-SLICE.5e.2.5.derived.3.3.3.4.3–.5. Scoped standalone source execution above
supplies no substitute for those remaining joins.

## Bare roots and actual let storage

A bare source expression is not a let. Its entity id,canonical consumer kind or graph edge supplies
no authored binding name/header. The source graph worker must retain the actual unbound scalar
root and its contribution flags;it must not invoke binding storage at an edge or manufacture a
let from an id. This applies to this bare initial-expression collection. Actual authored statements
continue to use their real statement proof and once-only storage.

The existing expression worker returns1 mm/3 as1000/3 micrometres. A source-reference continuation
multiplying that root by3 must not first round it to333. In contrast,the actual recipe
let a:length=1 mm/3 stores333,and the later let b:length=a+a stores666. Existing explicit round_to
and irrational-call rounding remain their authored/normative operations. Domain/width limits
still apply to every completed expression. Boolean kinds and approximation contributions survive.
Source lookup or a private runtime cache is not an additional rounding boundary.

Initial namespaces reject caller-seeded prior lets. Actual recipe reads consume only real earlier
stored outcomes;source traversal cannot reorder forward lets. Canonical Derived keeps its sole
formula reference and no independently authored/cached result. Supplier values must be earned by
actual source execution,not callbacks or supplied cache values. The durable canonical-input record
is docs/decisions/decision_length-declarations-retain-state-and-provenance.md. The source worker
preserves this boundary with scoped independent source acceptance. The actual recipe/Design bridge
has finite paired scalar acceptance;existing expression/recipe reference fixtures retain their exact outcomes.

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
let deps = checked.prepare_dependencies(FormulaSourceDependencyLimits::new(1,0,2))
    .expect("complete constant-source relation");
assert_eq!(deps.edges().len(), 0);
assert_eq!(deps.outgoing(formula).unwrap().len(), 0);
let acyclic = deps.admit_acyclic(FormulaSourceCycleLimits::new(1,2))
    .expect("complete acyclic canonical Derived relation");
assert!(std::ptr::eq(acyclic.dependencies(), &deps));
let scalar = acyclic.prepare_scalar_sources(FormulaSourceGraphExecutionLimits::new(
    1,nodes,0,2+2*nodes,1)).expect("complete available scalar source preparation");
assert_eq!(scalar.sources().len(), 1); // Preparation has computed no source value.
let executed = scalar.evaluate_source(formula, FormulaScalarExecutionContext::new(
    FormulaToleranceContext::empty(), FormulaSizeContext::empty())).expect("actual selected root");
assert_eq!(executed.entry().value().numeric().unwrap().rational(), FormulaRational::new(1000,1,false)?);
assert_eq!(executed.entry().completion_ordinal(), 1);
assert_eq!(executed.sources().count(), 1);
assert!(canonical.length(EntityId::from_bits(10)).unwrap().authored_value().is_err());
# Ok::<(), Box<dyn std::error::Error>>(())
```

The original declaration still has no authored result;execution returns its scoped unbound value
without changing that canonical state. A kind proof alone is no numeric value,storage
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

## Private evaluator continuation

The shared scalar evaluator now has a private continuation path for an actual unresolved Derived
name. Starting pins the original checked root and immutable tolerance/size contexts. Resuming takes
no replacement root,context or value. A suspension returns the original name node and actual
canonical inventory join,requeues exactly that name and keeps completed operands,pending operators,
calls and the selected conditional's contribution set. It neither restarts the expression nor
records an unresolved occurrence as a successful read. Both buffers retain their full admitted
N reservations;resumption adds no new buffer allocation or source-edge binding round.

A supplier can therefore run between two consumer steps without repeating earlier provider reads.
The private tests execute actual mapped supplier expressions before returning their unbound roots.
For eps_num+min(a*3,a*6),with a supplied by1 mm/3,the result is1001 µm and eps_num is read once.
For if(sin(0 deg)==0.0,min(a*3,2 mm),a),the chosen branch suspends at its real a;the untaken a
stays unread. A supplier containing cos(0 deg)*0 mm retains its cos contribution despite the
zero product,and the condition's sin contribution remains present after resumption.

Only successful completion makes scratch reusable. Any value/provider or one-shot Derived failure
permanently rejects further start/resume on that private machine. Existing expression/recipe
adapters still return their original RequiresEvaluation and missing-provider causes. Callers gain
no public callback,continuation,computed-value constructor or cache seed. The bounded lazy
source worker above uses this private path with scoped independent acceptance. Actual recipe/Design
pairing now has finite scalar acceptance;broader integration remains separately owned.

## Verification and remaining work

Six focused native contracts cover all six scalar kinds and dedicated Length,reversed source-id
order with exact proof/arena pointers,actual Unknown/reserved metadata after dropping the namespace
map,late unused branch errors,untaken equal-owner refusal,shared-source consumer mismatch,separate
quota precedence and empty drafts. Four source-registry contracts retain original index/buffer/
identity behavior,including the independent256-sequence duplicate scan. Compiler guards reject
default limits,private collection mutation and escaped proof nodes. No full verification is claimed.

This certifies sources under the supplied initial namespace,not completeness/currentness of a
containing Design/host namespace. Geometry metadata supplies no current operation/provider proof.
Prior lets cannot be caller-seeded into this initial scope. Actual ordered-statement/source
execution and real binding rounding now pass the available-scalar Design integration checkpoint.
Full geometry/operation dependency and cycle integration,load/replay,providers,whole-Design atomic
publication,full resource/backend bounds and platform/physical verification remain owned by
G1-SLICE.5e.2.5.derived.integration and the existing execution/integration tasks. No cycle or
execution certificate follows from successful name/kind checking alone.

## Explicit tangent policies in source graphs

The actual acyclic proof supplies prepare_scalar_sources_with_tangent_limits(layout,selected).
The layout is the existing complete S/N/D/2S+2N+D/output allowance;selected is an explicit
FormulaTangentLimits for each actual Tan request. Source count,population overflow and all complete
layout quotas still precede capability admission. Every Tan occurrence,including unused roots and untaken
branches,must have a selected policy before per-source scalar plan allocation/binding or values.
The earlier input/namespace/kind/dependency factories remain authoritative and unchanged.

```rust
use sc_core::recipe::*;
fn configured<'a>(graph: &'a FormulaAcyclicSourceDependencies<'a>,
    layout: FormulaSourceGraphExecutionLimits, selected: FormulaTangentLimits)
    -> Result<FormulaScalarSourceGraphPlan<'a>, FormulaScalarSourceGraphPlanError<'a>> {
    graph.prepare_scalar_sources_with_tangent_limits(layout, selected)
}
```

The resulting graph and every prepared expression retain the exact selected policy. The legacy
entry point has no Tan policy and returns MissingTangentPolicy with the actual source index and
call node. Complete population errors still win first. Preparation computes no source value.
Explicit zero budgets configure the family:an unused source or untaken Tan branch performs no
native request and adds no Tan contribution. A selected exact zero/unit requires only156 bits/0
terms. Each executed nontrivial call has its own cumulative native iteration budget.

| Actual source body | Selected bits/terms | Actual outcome |
| --- | --- | --- |
| `tan(0 deg)` | 156/0 | Ratio0 with Tan contribution |
| `tan(30 deg)` | 2073/512 | Ratio577350ppm with Tan contribution |
| `if(1==1,1.0,tan(90 deg))` | 0/0 | Ratio1M;untaken call contributes nothing |
| `tan(90 deg)` | 156/0 | Original exact pole |

The existing lazy source runtime invokes the prepared expression's policy and earns each unbound
supplier root once. A full128-bit Angle literal divided by Count remains its exact fraction in the
cache. A Tan consumer receives that original Angle,including actual upstream cos contributions.
Repeated references reuse only that successfully computed supplier,with original completion
ordinals/read witnesses. No source id,edge or cache adds storage rounding or a canonical value.

Graph preparation/execution errors retain tangent_limits(),including population failures,missing
selection,allocation or an unrelated earlier numerical cause. Nested actual scalar failures retain
the same policy,original supplier index/call/proof and native input/progress/cause. No accepted
cache prefix escapes failure. Logical layout/native quotas still exclude total heap/CPU and the
existing error-witness boxes;allocator causes remain their original inline errors.

A configured Tan supplier can already feed the existing source-backed recipe mode when the
recipe itself has no direct Tan call. Success retains source_plan().tangent_limits();even a
foreign-owner refusal before values retains source_tangent_limits(). Direct recipe-owned Tan
now has its own explicit policy;actual Design selection is also available.

Six focused tests cover whole-quota/missing-policy precedence,unused/untaken calls,an exact
full-width unbound Angle supplier,once-only shared native roots,original late quota failures,
missing selection/unrelated errors,and source-backed recipe success/foreign-owner retention.
Existing12 source tests,nine private execution guards and five relevant compiler/doc examples
pass. Coupled paired acceptance,full resources/backend/platform and production approval remain separate.
