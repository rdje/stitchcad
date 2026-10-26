# Formula runtime validation evidence

This annex describes the book reference evaluator's runtime controls, separate from the
[static checks](formula-static-validation.md). Product libraries currently expose syntax, input
normalization, canonical identity, [whole recipe kind proofs](formula-checked-recipes.md), exact numeric
values and the canonical length authored-read adapter below. Complete execution remains .5e–.5g work.
The [formula chapter](../spec/formula-language.md) supplies the normative rules. This instrument
verifies scoped numerical examples and diagnostics; it does not certify a physical garment.

## Canonical length authored reads

FormulaLengthInput::from_declaration accepts an actual borrowed FormulaDeclarationSource::LengthInput.
Its Option is None for general scalar metadata even when that metadata claims Length, prior bindings,
reserved names and geometry. This selection reads no numeric value. All five canonical length-input
origins retain their original name, metadata identity, canonical declaration/source and borrowed state.
Call read_authored explicitly to read an authored value:

| Canonical state | Authored-read result |
| --- | --- |
| Known | Exact Length with the original evidence inventory retained |
| Assumed | Exact Length with its actual assumption reference retained |
| Preference | Exact Length with its actual preference provenance retained |
| Unknown | RequiresObservation with the declaration and observation identities |
| Derived | RequiresEvaluation with the declaration and sole formula identity |

A successful FormulaLengthInputValue keeps its input beside the numeric value. Inspect input().state()
to retain the distinction between measured claims, assumptions and preferences. An unavailable read
returns FormulaLengthInputError with the same input and exact canonical reason. Unknown never becomes
zero; derived never becomes an independent authored or cached result. A complete evaluator must follow
the derived dependency and attach the available expression/statement context; this read adapter does
not execute that formula or manufacture a statement ordinal/diagnostic publication.

```rust
use sc_core::{name::MachineToken, ontology::EntityId,
    recipe::{FormulaDeclaration, FormulaInputOrigin, FormulaLengthInput},
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState}};

let name = MachineToken::new("body_width")?;
let record = LengthDeclaration::new(LengthDeclarationDefinition {
    id: EntityId::from_bits(1), source: EntityId::from_bits(2),
    state: LengthState::Unknown { observation: EntityId::from_bits(3) },
})?;
let metadata = FormulaDeclaration::length_input(&name, FormulaInputOrigin::Measurement,
    EntityId::from_bits(4), &record);
let input = FormulaLengthInput::from_declaration(metadata).expect("actual canonical length input");
let unavailable = input.read_authored().expect_err("observation still needed");
assert_eq!(unavailable.input().canonical_declaration().id(), EntityId::from_bits(1));
# Ok::<(), Box<dyn std::error::Error>>(())
```

Validated sc_units::Length converts totally into FormulaNumericValue through `From<Length>`. It imports
the signed canonical micrometre integer exactly, including zero and both scalar limits; no second
input/binding rounding or i64 check is introduced. Reads preserve the source record unchanged.
Readable authored claims still carry DeferredToDesignAndG4 provenance validation. This API grants no
namespace/order/registry/evidence-scope/human/export approval. General scalar, size and optional
context adapters and the complete evaluator remain owned by G1-SLICE.5e.

The focused formula_length_input_contract target passes three tests covering25 origin/state pairs,
the scalar endpoints/zero, and every other declaration-source variant, including a generic Length
claim. Broad integration/platform proof remains deferred.

## General scalar authored reads

FormulaScalarInput::from_declaration takes general Input metadata and the caller's actual
ScalarInputDeclaration. It first checks source type, then the canonical declaration id, then the
actual kind. The order is fixed: a wrong target wins over a wrong annotation. All six declared
kinds and the Parameter/Profile/Material origins are supported; canonical Area remains Derived.
Construction does not read the value, so matching Unknown/Derived declarations remain selectable.

| Adapter refusal | Retained witness |
| --- | --- |
| NotScalarInput | Actual metadata kind/origin; a LengthInput, binding, reserved or geometry source cannot impersonate general Input |
| CanonicalIdentity | Metadata target id and actual supplied canonical id |
| CanonicalKind | Claimed annotation and actual canonical kind |

FormulaScalarInputAdapterError also retains the actual name and candidate record. Debug omits the
customer name; explicit name/Display access retains it. A matched view reconstructs the original
name/origin/metadata id/target id/kind and borrows the original canonical source/state. Metadata id,
canonical declaration id and its source id remain distinct; none is guessed from the name.

Call read_authored to obtain FormulaScalarInputValue with the exact typed canonical scalar beside
its input. Its value is ScalarInputValue: inspect numeric_integer or boolean without treating zero
or false as absence. Signed angle turns and Ratio values remain unchanged. Unknown/Derived return
FormulaScalarInputError with the original input and exact RequiresObservation/RequiresEvaluation
reason; the standard error source retains that same canonical reason. Derived Area has no authored
numeric result. Convert a present numeric payload exactly with
`FormulaNumericValue::try_from(read.value())`, keeping the original read wrapper for source/state.
Boolean conversion refuses; inspect boolean separately. Dependency evaluation, source propagation
and binding publication remain later evaluator responsibilities.

```rust
use sc_core::{name::MachineToken, ontology::EntityId,
    recipe::{FormulaBindingKind, FormulaDeclaration, FormulaScalarInput, FormulaScalarInputOrigin},
    value::{ScalarInputDeclaration, ScalarInputDeclarationDefinition, ScalarInputKind,
        ScalarInputState, ScalarInputValue}};

let name = MachineToken::new("allow_adjustment").expect("machine token");
let record = ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
    id: EntityId::from_bits(1), source: EntityId::from_bits(2), kind: ScalarInputKind::Boolean,
    state: ScalarInputState::Preference {
        value: ScalarInputValue::from_boolean(false), provenance: EntityId::from_bits(3),
    },
}).expect("typed preference");
let metadata = FormulaDeclaration::input(&name, FormulaScalarInputOrigin::Parameter,
    EntityId::from_bits(4), record.id(), FormulaBindingKind::Boolean);
let input = FormulaScalarInput::from_declaration(metadata, &record).expect("matched id/kind");
let read = input.read_authored().expect("present authored false");
assert_eq!(read.value().boolean(), Some(false));
assert!(std::ptr::eq(read.input().canonical_declaration(), &record));
```

This checks the supplied candidate's identity/kind; it does not establish registry membership,
uniqueness, the exposing input record's validity, source truth/evidence scope, execution order or
human/export approval. Provenance remains DeferredToDesignAndG4. A complete runtime lookup must
supply the actual candidate and attach only its available expression/statement context.

The focused scalar adapter target passes five tests, including78 origin/state reads, the independent
six-kind table, complete kind mismatch pairs, identity priority and every other source variant.
The affected length adapter retains three passing tests; selected strict Clippy passes. Whole
runtime context providers/evaluation and broad/platform verification remain later owned work.

## Explicit tolerance contexts

FormulaToleranceContext makes all five reserved tolerance reads explicit. Empty/default contexts
supply eps_num=1µm and eps_geo=10µm from the existing units API. The optional format, importer and
physical slots have no default values. Each provided FormulaToleranceProvider borrows the actual
Tolerance and retains its caller-supplied source-record identity.

FormulaToleranceContext::new checks actual quantitative classes in format/importer/physical order:
FormatQuantization, ImporterComparison and PhysicalAcceptance respectively. A wrong-class provider
returns FormulaToleranceProviderError with the actual name, expected class, source and content.
The100µm geometric chordal sibling cannot impersonate another class. The definition has no fields
for replacing the fixed numerical/internal geometric intrinsics.

```rust
use sc_core::{ontology::EntityId, recipe::{FormulaToleranceContext,
    FormulaToleranceContextDefinition, FormulaToleranceName, FormulaToleranceProvider}};
use sc_units::{Length, Tolerance, ToleranceClass};

let format = Tolerance::new(ToleranceClass::FormatQuantization,
    Length::from_micrometres(25).expect("fixture Length"),
    "explicit fixture writer quantum; not a product default").expect("derivation");
let context = FormulaToleranceContext::new(FormulaToleranceContextDefinition {
    label: Some("explicit_fixture_export"),
    format: Some(FormulaToleranceProvider { source: EntityId::from_bits(1), tolerance: &format }),
    ..FormulaToleranceContextDefinition::default()
}).expect("format class");
assert_eq!(context.read(FormulaToleranceName::Format).expect("provided quantum")
    .value().rational().numerator_magnitude(), 25);
assert_eq!(context.read(FormulaToleranceName::Physical).expect_err("profile absent")
    .diagnostic_code(), "formula_tolerance_unbound");
```

A successful FormulaToleranceValue retains its name, exact completed Length, quantitative class/
limit/derivation and FormulaToleranceValueSource. Intrinsic identifies language content; Provider
retains the original borrowed object/source id. Reading adds no input/binding rounding. A supplied
zero remains a present value. Cloning/editing a definition and validating a new context cannot
mutate the prior one.

Missing optional providers return FormulaToleranceReadError::MissingContext with the actual name
and optional caller label; None and an explicitly supplied empty label remain distinct. The error
reports formula_tolerance_unbound and the language-required Export/Profile category, without
inventing a target name or statement ordinal. Intrinsic unit-construction errors propagate with
their actual source and formula_domain family; no constant is unwrapped or substituted.

Class matching and typed content do not prove registry membership, derivation/evidence truth,
physical suitability or export approval. Source ids/labels are supplied explicitly by the host;
G4/context integration retains those validation obligations. Runtime within/assertion provenance
and size contexts remain separate owners.

The focused formula_tolerance_context_contract target passes five tests covering both intrinsic
values, all eight optional-provider combinations with absent/nonempty/empty labels, all six
quantitative classes against each external slot, refusal priority, explicit zero and immutable
replacement. Selected strict Clippy passes; broad integration/platform proof remains deferred.

## Core size context frames

FormulaSizeName provides Index, Count and IsBaseSize roles over the existing size_index, size_count
and is_base_size reserved names. FormulaReservedName::size_name selects only these three roles;
tolerance names cannot enter this read interface. No new spelling, text value or size label is added.

FormulaSizeContext::new takes a supplied FormulaSizeContextSource and unsigned128-bit index/count.
The source retains set id, exact Count revision, selected member id and explicit base id. Validation
refuses set/member-or-base aliases first, then empty/out-of-range one-based counts, then a singleton
whose selected member differs from its base. This validates supplied structural facts. It does not
prove actual authored membership/order, pin currentness, axes, charts or path readiness. The [membership bridge](../spec/size-membership.md#formula-size-context) now derives the actual
authored source/order/count/base after matching the expected pin. D70's axis decision is unchanged.

```rust
use sc_core::{ontology::EntityId, recipe::{FormulaSizeContext, FormulaSizeContextSource,
    FormulaSizeName, FormulaRational}};
use sc_units::Count;

let source = FormulaSizeContextSource { set: EntityId::from_bits(1), revision: Count::new(0),
    member: EntityId::from_bits(2), base: EntityId::from_bits(3) };
let context = FormulaSizeContext::new(source, 2, 3).expect("supplied structural frame");
let index = context.read(FormulaSizeName::Index).expect("present context");
assert_eq!(index.numeric_value().expect("Count").rational(), FormulaRational::from_signed_integer(2));
assert_eq!(context.read(FormulaSizeName::IsBaseSize).expect("present false").boolean(), Some(false));
```

Each FormulaSizeValue retains its requested role and source. Index/count return exact Count values;
is_base_size returns the actual identity comparison, independent of ordinal or label. False is a
present Boolean; numeric_value is None for Boolean and boolean is None for Count. The empty/default
context instead returns FormulaSizeReadError with the actual requested name and formula_unbound_name.
It supplies no zero/false, guessed member, source/search scope or statement ordinal.

FormulaNumericValue::from_unsigned_count imports every unsigned128-bit integer exactly, including
zero and the maximum magnitude, without rounding or premature signed-i64 checks. A wide size-frame
count stays valid until an actual binding enforces its storage range. This is not a claim that an
arbitrarily large native membership was allocated; the core frame is a host-supplied value boundary.
Replacing a frame leaves prior pin/values unchanged. A membership adapter must derive source, ordinal,
count and base from the actual immutable authored membership and compare its expected pin.

The focused formula_size_context_contract target passes five tests for role closure, absence,
source/Count/false/base behavior, exact invalid witnesses/priority, wide binding refusal and immutable
replacement. Selected strict Clippy passes. Actual authored membership/pin derivation is available through the
[sc-measure bridge](../spec/size-membership.md#formula-size-context); global registry/axis/path and
broad/platform proofs remain their distinct owners.

## Reference runtime assertion controls

An assertion uses an inclusive tolerance: it holds when the absolute difference is at most the
class value in the operands' internal units. The reference preserves the successful five-element
assertion result, including the evaluated operands. A larger difference raises formula_assertion
before any false result can be returned. For example:

```text
assert width_closure: eps_num = 1 cm == 1 cm
assert false_closure: eps_num = 1 cm == 2 cm
```

The first holds. The second raises formula_assertion with false_closure, length values10000 and
20000, and eps_num value1. The diagnostic's owned arguments dictionary contains assertion_name,
left_kind, left_value, right_kind, right_value, tolerance_class and tolerance_value. Values remain
exact rational results, in authored operand order; this instrument does not invent a recipe ordinal
or canonical source identity. These reference fields are not a production typed diagnostic API.

Syntax/name/kind checking still precedes runtime work. Once static checking succeeds, the existing
operand and tolerance evaluation order is preserved: division by zero, unknown values and missing
class context retain their own diagnostic tokens rather than becoming assertion failures. The
book consumer reports the raised token, label, values and class through its existing error path.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/assertion_contract.py --mutations
```

Its [producer plan](formula-documentation-controls.md#assertion-reference-and-copied-consumer-plans) preserves pure calls and guards copied outputs.

The existing structural runner watches this producer.262 independently authored controls cover all
five arithmetic kinds and all five classes, zero/below/at/above thresholds in both operand orders,
signed values where permitted, unchanged caller bindings, static acceptance of false checks and
earlier static/runtime errors. Context-dependent classes use explicit fixture values25/40/100;
these are controls, not new defaults. Canonical integer input bindings divided by ratio2.0 supply
exact half-unit results. The grammar deliberately makes Count*Ratio a Ratio and Count/Ratio a Count;
the controls follow those declared kinds rather than changing the table to fit a fixture.

A copied worked book falsifies waistband_width_closure: the actual consumer returns1 and reports
formula_assertion with length80000 versus40000 at eps_num1. Eight actual compiled predicate/payload
faults must fail body assertions: false guard, inclusive boundary, token, label, operand order/kind,
class and class value. The reference source stays byte-identical during these in-memory faults.
G1-SLICE.5e.3a owns this repair. D122 reference missing-value routing is described below.
D121 reference contribution provenance is verified below. Product evaluation, geometry and API/MCP/release remain pending.

## Missing values by origin

A name's kind can be known when its value is unavailable. Preflight can accept the name and still
leave a runtime read to refuse. The reference now follows contract3's origin rules:

| Missing input | Runtime diagnostic |
| --- | --- |
| A measurement, Ease, design parameter, profile or material value | formula_unknown |
| A geometry, earlier recipe or size value | formula_unbound_name |
| A tolerance value | formula_tolerance_unbound |

For example, body_width declared as an unknown length from measurement can pass the static check
for let doubled:length=body_width*2. Executing it raises formula_unknown. Missing geometry p raises
formula_unbound_name when read; missing eps_fmt raises formula_tolerance_unbound. None receives a
replacement value. The original state is retained for an absent fact, even when a malformed record
claims known but supplies no value; this routing is not approval of that record as canonical data.

### Optional context values

The reference's reserved tuple contains kind, always-visible metadata and a supplied value. The
always flag records the normative availability rule; it does not prevent an optional context from
supplying a value. A None value means unavailable. For example, eps_fmt with length/False/25 is
readable when the fixture supplies25µm. A size context supplying is_base_size=False is readable
and selects a conditional's else branch. False and zero are values, not absence tests.

The reference accepts an optional explicit context label. A missing tolerance diagnostic includes
that label when supplied; no label is guessed for a caller that supplies none. This label is
instrument context, not proof of a real export target, profile, size set or authority grant.

### Diagnostic arguments and malformed declarations

The owned error arguments retain name and declared origin. Unknown-fact errors retain the actual
state; unbound errors retain the reference's closed, sorted origin vocabulary as origins_searched.
A missing undeclared name has no invented origin. Tolerance errors retain explicitly supplied
context. Artifact-blocking policy, object evidence, statement ordinals and canonical expression
context remain product requirements; these reference fields are not their typed production API.

Missing declaration kind/origin fields, non-string or undeclared metadata and invalid record shapes
raise formula_parse rather than leaking KeyError or TypeError. An explicit state requires one of the five authored tokens; malformed state raises formula_parse.
An unknown record carrying a value is also malformed and refused. Unknown lazy geometry is refused
before resolution or cache publication. An absent fact requires an explicit valid authored state. Static namespace checks
still observe only kind/origin. They do not read values or uncertainty states. Existing populated
numerical fixtures without state metadata remain readable; full canonical input adapters are pending.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/origin_value_contract.py --mutations
```

Its [producer plans](formula-documentation-controls.md#origin-and-provenance-consumer-plans) preserve pure calls and guard both copied books before effects.

The structural runner watches1466 independent controls: nine origins/eight kinds/five missing states,
four populated states, zero/false values, all eight reserved names, explicitly supplied contexts,
taken-only missing reads, malformed declarations and populated-state consistency. Lazy point/edge resolution, subsequent cached
reads and upstream failures retain their established behavior. No partial cache is published after
a failed resolution. The reference geometry values are arithmetic fixtures, not real entity references.

Two actual copied worked books pass all21 static statements, then refuse missing size_index and
eps_fmt at runtime with their distinct tokens. Thirteen compiled predicate/payload faults require body
assertion reds; the reference source remains unchanged during those in-memory faults. Existing
namespace1139/thirteen fault controls still trap all numerical/state/geometry access.
G1-SLICE.5e.1a closes D122 origin/context routing and D127 malformed-input exceptions and D128 uncertainty-state bypass. Product
adapters and complete production diagnostic/provenance context retain their separate owners.

## Approximation contributions and admissible classes

### Product source-set primitive

`FormulaApproximationSources` now supplies an immutable, deduplicated set of existing built-ins.
`for_call` introduces the source of sqrt, hypot, sin, cos, tan, atan, atan2, arc_length, dist, dir
or round_to. Other calls introduce no source themselves. `union` merges executed dependencies;
`with_call` then adds the current call. `iter` reports distinct sources in lexical machine-token
order, independently of the signature table's grouping. Copies and merges leave earlier sets intact.

```rust
use sc_core::recipe::{FormulaApproximationSources as Sources, FormulaBuiltin as B,
    FormulaToleranceName as T};

let sources = Sources::for_call(B::Hypot).with_call(B::RoundTo);
let refusal = sources.check_tolerance(T::Numerical).expect_err("T1 cannot admit these sources");
assert_eq!(refusal.tolerance(), T::Numerical);
assert_eq!(refusal.sources(), sources);
assert_eq!(refusal.diagnostic_code(), "formula_domain");
sources.check_tolerance(T::Geometric).expect("class admission only");
```

The helper checks the symbolic class alone. It neither reads a provider nor compares values, so
an admitted class still needs its actual context and threshold. The refusal keeps the requested
class and complete supplied source set. The enclosing evaluator must attach the actual comparison
and statement context. There is no fabricated label or statement ordinal.

The caller must supply the dependencies that actually executed: both arithmetic operands, every
evaluated min/max/clamp candidate, and a conditional's condition plus its selected branch. Point
coordinates require separate sets. Binding and later reads must retain the set even after exact
cancellation or an integer result. These propagation rules remain full evaluator work; this helper
does not execute a conditional, numerical call, binding or geometry query. Four focused native
tests cover every built-in, all five classes, immutable unions and explicit selected-source examples.

### Product scalar operators and bindings

`FormulaEvaluatedScalar` now retains a numeric or Boolean payload beside its approximation sources.
`from_numeric` and `from_boolean` start with no contributions. Attach successful call sources
explicitly with `with_call_source`, and merge additional executed dependencies with `influenced_by`.
Neither operation removes earlier sources or certifies that a call actually ran.

`checked_unary` and `checked_binary` compute the existing exact numeric operators. Unary results
keep their input set; binary results keep both sets. All six bare comparisons return a Boolean
with the same union. They apply no tolerance rule. Boolean arithmetic/comparison has no admitted
signature and refuses with actual kinds. Count negation, domain, width and division refusals retain
their existing numeric errors. Cancellation to zero cannot erase how a value was obtained.

```rust
use sc_core::{name::MachineToken, recipe::{FormulaBinaryOperator as B, FormulaBuiltin as Call,
    FormulaEvaluatedScalar as V, FormulaKind as K, FormulaNumericValue as N, FormulaRational as R,
    FormulaToleranceName as T}};

let value = V::from_numeric(N::new(K::Ratio, R::new(3, 2, false))?)
    .with_call_source(Call::RoundTo); // caller attaches an already-executed dependency
let name = MachineToken::new("stored_ratio")?;
let binding = value.bind(&name)?;
assert_eq!(binding.value().numeric_integer(), Some(2)); // 2ppm, once-rounded from 3/2ppm
let read = binding.read()?;
let equal = read.checked_binary(B::Equal, read)?;
assert_eq!(equal.boolean(), Some(true));
assert!(equal.sources().check_tolerance(T::Numerical).is_err());
# Ok::<(), Box<dyn std::error::Error>>(())
```

`bind` produces a private `FormulaEvaluatedBinding`: numeric values use the existing single binding
round/storage check, while Boolean values remain Boolean. The binding keeps its original source set
and only the stored integer/Boolean. `read` returns that stored payload with every source retained;
the old fraction is absent. Failed binding returns the actual name/kind/rounded storage refusal and
no replacement binding. Earlier immutable bindings remain usable.

Five focused native tests verify numeric/Boolean presence, all ordinary operators, cancellation,
bare-comparison contributions, sourced numeric/Boolean binding reads and actual typed refusals.
This executes value operators, not parsed expressions or statements. Namespace/provider lookup,
call execution, selected-branch control, point-coordinate provenance, assertions and atomic recipe
publication remain the full evaluator's responsibilities.

### Product implemented numeric calls

The sourced scalar value now executes checked_abs, checked_sqrt, checked_hypot, checked_arc_length,
checked_min, checked_max, checked_clamp and checked_round_to through the existing numerical kernels.
Successful calls automatically union every evaluated argument's sources and add their own source
when the contract requires it. Abs/min/max/clamp add no new contribution. Sqrt/hypot/arc_length and
round_to do, including zero, perfect roots and values unchanged by the requested step.

```rust
use sc_core::recipe::{FormulaEvaluatedScalar as V, FormulaKind as K, FormulaNumericValue as N,
    FormulaRational as R, FormulaToleranceName as T};

let area = V::from_numeric(N::new(K::Area, R::from_signed_integer(9))?);
let root = area.checked_sqrt()?;
assert_eq!(root.numeric().expect("numeric root").rational(), R::from_signed_integer(3));
assert!(root.sources().check_tolerance(T::Numerical).is_err()); // perfect root still carries sqrt
# Ok::<(), Box<dyn std::error::Error>>(())
```

Min/max keep every evaluated candidate's sources even when another value wins. Clamp retains both
bounds as well as its principal value. Explicit round_to retains the step's sources. Function result
sources survive ordinary arithmetic, cancellation, binding and later reads through the same scalar
API. These calls operate on supplied values; the caller still evaluates arguments in source order.

Wrong input kinds retain the first actual argument position; homogeneous calls retain the wanted
and actual kind. Empty min/max, negative sqrt, invalid clamp bounds, zero quantization step and
numeric result refusals propagate their typed failures without a substitute value. A failed call
produces no new sourced scalar. Five focused native tests cover all eight implemented call families,
source unions, exact-valued calls through storage, singleton/extremum cases and actual refusals.
Trig/inverse calls, geometry selectors and full AST/provider/conditional/recipe execution remain open.

### Product named-tolerance comparisons

`FormulaEvaluatedScalar::compare_tolerance(right, tolerance)` now compares same-kind numeric
operands at an actual `FormulaToleranceValue` read from the explicit context. The inclusive test is
the exact absolute difference at that class's canonical internal-unit limit. No rounding, modulo,
ppm rescaling or numeric subtraction result is introduced. A comparison can therefore succeed even
when materializing its difference as another expression would exceed that expression's domain or
128-bit component bound. All five numeric kinds and all five formula classes are supported.

```rust
use sc_core::recipe::{FormulaEvaluatedScalar as V, FormulaKind as K, FormulaNumericValue as N,
    FormulaRational as R, FormulaToleranceContext, FormulaToleranceName as T};

let context = FormulaToleranceContext::empty();
let left = V::from_numeric(N::new(K::Length, R::from_signed_integer(100))?);
let right = V::from_numeric(N::new(K::Length, R::from_signed_integer(110))?);
let comparison = left.compare_tolerance(right, context.read(T::Geometric)?)?;
assert!(comparison.holds()); // exactly 10 micrometres: inclusive
assert_eq!(comparison.value().boolean(), Some(true));
assert_eq!(comparison.tolerance().name(), T::Geometric);
# Ok::<(), Box<dyn std::error::Error>>(())
```

The immutable `FormulaToleranceComparison` retains both original sourced values, the exact named
class/limit/derivation/provider/context and its Boolean outcome. `value` returns a Boolean carrying
both operands' contributions, including a false result. Binding/read preserves those sources.
Zero provider limits permit equality; a supplied negative limit accepts no difference. This primitive
does not replace either value with a default or establish provider truth/physical suitability.

Actual kind refusals precede the symbolic class check. T1 with any contribution refuses before the
threshold, even for equal values. `FormulaToleranceComparisonError` retains actual operands and
quantitative context beside the typed dimension or approximation refusal and underlying error source.
It invents no comparison label, expression span or statement ordinal. Optional-context lookup must
already have succeeded; the primitive cannot create the required tolerance value from an absent
provider. A false threshold is a valid comparison outcome here; a named assertion must refuse it.

Five focused native tests cover all kinds/classes, fractional threshold neighbors, both operand
orders, exact full-width witnesses, a difference outside the Length domain, source/class priority,
actual context, Boolean storage and explicit zero/negative providers. The wide witnesses are also
checked independently with Python Fraction arithmetic. Complete within/assertion AST execution,
source-order lookup, statement diagnostics and atomic publication remain later evaluator work.

### Existing reference execution controls

A value retains an immutable set of approximation-producing calls in its executed dependencies.
The set is evidence about how the result was obtained: sin(90 deg) remains a sine contribution
even though its rounded answer equals ratio1.0. Named T1 comparisons require an empty set.
T2 or looser admits those contributions and then applies the usual inclusive threshold.

```text
assert direct: eps_num = sin(90 deg) == 1.0
let rounded: ratio = sin(90 deg)
assert stored: eps_num = rounded == 1.0
within(rounded, 1.0, eps_geo)
```

Both assertions raise formula_domain. The within comparison holds. The owned diagnostic arguments
are comparison (the assertion label or within), tolerance_class and sorted contribution_sources.
These examples retain sin as the source. Choosing eps_num with a larger supplied numeric value
still refuses: admissibility follows the symbolic class. The director's D121 ruling is recorded
in ADR-0003; these instrument fields are separate from the pending production typed diagnostic API.

The reference marks sqrt, hypot, sin, cos, tan, atan, atan2, arc_length, dist and dir, and the
authored round_to quantization operation from contract4.2. Exact rational operations and input or
binding rounding alone add no approximation source. Arithmetic carries all executed inputs,
including cancellation to zero. min/max/clamp retain evaluated candidates/bounds; an evaluated
Boolean condition also contributes to the branch it selects. Bare exact comparisons retain their
existing numeric behavior and carry their inputs' sources into the Boolean result.

Only the taken branch executes. For example, within(if(1 == 1, 1.0, sin(90 deg)), 1.0, eps_num)
holds with no sine contribution. A binding stores and subsequently reads both its integer value
and contribution set; rounding and later reads cannot erase that evidence. The actual book consumer
publishes the returned sources alongside each stored binding.

Lazy point/edge operations retain the sources of their evaluated arguments after successful
resolution. Point coordinates retain separate sets: if x=1 um and y=hypot(3 um,4 um), a comparison
of x against1um may use T1, while y requires T2 or looser. Cached reads preserve the same distinction.
An error in a later point coordinate publishes neither a value nor new provenance. The opaque
point_at and length-only edge fixtures remain arithmetic models, with no curve-accuracy proof.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/provenance_contract.py --mutations
```

Its [producer plans](formula-documentation-controls.md#origin-and-provenance-consumer-plans) also guard the copied mutant script; tracked reference sources stay read-only.

The structural runner watches425 independent controls across all approximation signatures, five
arithmetic kinds/classes, operand orders, named refusals, ordinary operators, stored/Boolean reads,
lazy branches, geometry/cache/coordinate separation, earlier errors and exact controls. A copied
worked book compares equal side_seam_length bindings at T1: all21 statements preflight, then the
actual consumer refuses with allocation_balance, eps_num and hypot.26 compiled actual guard/source/
cache/consumer faults must fail body assertions; the reference source remains byte-identical.
G1-SLICE.5e.3b closes D121's reference scope. Arbitrary-input transcendental rounding, product
execution, real geometry, replay, API/MCP and release correctness remain separate obligations.
