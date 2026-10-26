# SizeSet profile transformations

> **Status:** G1-SLICE.4c.4 implements immutable transformation intent in `sc-measure`.
> Selected profile labels, resolution-origin metadata and state-bearing label/base declarations are
> executable, together with canonical break/chart bindings and an immutable resolution-intent aggregate. G4 owns actual profile lookup,
> constraints, precedence and resolution;
> G3 owns numeric instantiation. This page extends [Size sets §9](size-sets.md#9-ownership-reference-and-override).

## Pin the selected profile explicitly

`SizeSetProfilePin` retains three distinct authored strings: `profile_id`, `schema_version` and
`content_version`. Roadmap §8's example selects profile `acme-cmt-pt`, schema `2.0` and content
`1.3.0`. Updating profile content does not silently change a saved pin or its schema version.

```rust
use sc_measure::{SizeSetProfilePin, SizeSetProfilePinDefinition};
let profile = SizeSetProfilePin::new(SizeSetProfilePinDefinition {
    profile_id: "acme-cmt-pt".to_owned(),
    schema_version: "2.0".to_owned(),
    content_version: "1.3.0".to_owned(),
}).expect("all three fields explicitly supplied");
assert_eq!(profile.definition().schema_version, "2.0");
assert_eq!(profile.definition().content_version, "1.3.0");
```

G1 checks presence only: an empty or whitespace-only field returns `SizeSetProfilePinError`, naming
that field and preserving its refused spelling. The order is profile key, schema version, content
version. Nonblank text retains Unicode, case and spacing exactly. Labels are opaque here; accepting
one proves neither a valid version grammar nor an existing or approved registry entry. G4 validates
those contracts. There is no normalization, version comparison, inferred default or latest-version
selection. `validation()` explicitly returns `ProfileBindingValidation::DeferredToG4`.

This selected pin belongs to a resolution operation. Existing authored `ProfileParameterRef` bindings
remain symbolic and factory/version agnostic until instantiation; this API does not rewrite them.

## Keep the source and resolved target distinct

`SizeSetResolutionOriginDefinition` records:

| Field | Authored meaning |
| --- | --- |
| `source` | Exact original SizeSet identity/revision named by the Design |
| `profile` | Explicit selected profile/schema/content pin |
| `resolved` | Separately allocated target identity/revision |
| `provenance` | Explicit correspondence/provenance record identity |

Pass that definition and an actual `SizeSetInputs::Garment` or `SizeSetInputs::Mtm` view to
`SizeSetResolutionOrigin::new`. It rechecks current supplied inputs, requires the exact source pin,
and refuses a target identity already present among supplied input records or equal to the origin's
provenance identity. Changing the revision does not permit reusing the source object's identity.
The checks cover membership, axes/coordinates, canonical numeric and measurement records, tables,
chart observations or MTM body/Ease records, the chart itself and available breaks.

For example, source `(set A, revision 3)` may record target `(set B, revision 0)` and the exact
selected profile. Target `(set A, revision 4)` refuses with `DuplicateIdentity`; a definition that
claims source `(set A, revision 2)` against revision-3 inputs returns `SourceMismatch` with both
references. The constructor never retargets a stale source or mutates the original membership/chart.
`definition()` borrows the immutable authored metadata; `validate_current(inputs)` rechecks it.

The caller or future command bus supplies the allocated target ID. Local nonaliasing is not proof
of global ID freshness, registry currentness or provenance truth. Those obligations remain with the
allocator, registry and evidence store. An origin can accompany draft chart/break inputs; recording
intent does not certify regeneration or grading readiness. Existing path checks remain separate,
including MTM's original grading refusal.

## Author label and base transformation intent

`SizeSetTransformationDefinition` declares its own identity, exact source SizeSet reference, source
provenance, explicit `SizeSetTransformationKind` and `SizeSetTransformationState`. Its constructor
accepts an actual garment or MTM `SizeSetInputs` view. It checks the source pin and refuses a
transformation ID that aliases a supplied input record or its own source provenance. The immutable
result borrows out its original definition and supports `validate_current(inputs)`.

The state follows the ontology's five-way distinction:

| State | Authored content |
| --- | --- |
| `Known` | Typed payload and nonempty distinct evidence references |
| `Assumed` | Typed payload and the original human assumption reference |
| `Unknown` | Original observation request; no payload |
| `Preference` | Typed payload and original preference provenance |
| `Derived` | Sole formula source; no independent payload |

`authored_value()` borrows the original payload for Known, Assumed and Preference without upgrading
its state. Unknown returns `RequiresObservation` with the transformation/request IDs; Derived returns
`RequiresEvaluation` with the transformation/formula IDs. Nothing substitutes a default or cached
result. Evidence existence, scope, human attribution and truth remain explicitly
`ValueProvenanceValidation::DeferredToDesignAndG4`.

A `LabelMapping` payload is a nonempty vector of `SizeLabelMapping { member, label }`. Member IDs
must exist and occur once. Target labels retain exact Unicode, case and spacing through `SizeLabel`.
Mappings can be partial: other members retain their source labels. Proposed labels must be unique
across mapped and unchanged members. The vector retains its authored order; membership order is
never sorted or rewritten. A simultaneous swap is valid when the final designations are unique.

For original members `(A, "2XL"), (B, "S"), (C, "M")`, mapping `B -> "Small"` is valid. Mapping
`B -> "M"` refuses because unchanged C still has that label. Mapping `C -> "S", B -> "M"` is valid
and retains that authored mapping order. Neither case changes the original membership object.

`BaseSubstitution(member)` names an actual existing member directly. A label or ordinal never
selects the base. Selecting the already-current base remains explicit valid intent, including for
an MTM singleton; it does not make MTM grading available. Missing members refuse. The explicit
kind must agree with the authored payload kind; no operation is inferred from its contents.

For example, after obtaining actual `inputs`:

```rust,ignore
let definition = SizeSetTransformationDefinition {
    id: transformation_id,
    reference: inputs.reference(),
    source: provenance_id,
    kind: SizeSetTransformationKind::BaseSubstitution,
    state: SizeSetTransformationState::Assumed {
        value: SizeSetTransformationValue::BaseSubstitution(existing_member_id),
        assumption: human_assumption_id,
    },
};
let intent = SizeSetTransformation::new(definition, inputs)?;
```

## Bind break scaling to canonical records

`SizeSetTransformationKind::BreakScaling` uses a `SizeBreakScaling { size_break, factor }` payload.
The break identity must exist in the actual available break inventory. Its existing axis, named
POM/dimension, adjacent-member pair and dedicated canonical Length delta remain the source of truth;
there is no label-derived or reconstructed break key.

The factor must be an actual general `ScalarInputDeclaration` of kind `Ratio` in the shared current
numeric inventory supplied through `SizeBreakContext`. It need not be an axis coordinate. A missing
factor returns `MissingFactor`; a supplied record of another role, including a dedicated Length,
returns `FactorRole`. A general scalar of another kind returns `FactorKind` with its actual kind,
even for an Unknown or Derived declaration. Missing available breaks return `MissingBreak`.

```rust,ignore
let binding = SizeBreakScaling { size_break: break_id, factor: ratio_declaration_id };
let records = binding.inputs(inputs)?;
// These are the original canonical records, preserving state/source/provenance.
let original_delta = records.delta;
let original_factor = records.factor;
```

`binding.inputs(inputs)` rechecks current inputs and borrows the actual break, Length delta and
Ratio declaration in `SizeBreakScalingInputs`. The transformation constructor validates this binding
for an authored payload. Calling either API does not multiply values, round a result, fill a missing
break or create an independent state/value copy. A replacement input snapshot is rechecked through
the same lookup, so changing a factor's actual kind cannot preserve an earlier successful binding.

Zero and negative signed factors are retained, as are factors above one. This local structural API
imposes no invented positivity constraint; G4's actual profile domain and hard restrictions decide
whether proposed intent may be executed. All five factor/delta states remain canonical. Reading an
Unknown factor still returns its original observation refusal, and Derived still requires its
original formula. Known transformation evidence does not upgrade either numeric input's state.
The same scaling domain can be declared Unknown or Derived without any binding payload.

## Resolve explicit current chart replacements

`SizeSetChartReplacement::Garment(chart_id)` and `::Mtm(chart_id)` retain an explicit family and
identity. `SizeSetReplacementCharts::Garment(&charts)` or `::Mtm(&charts)` supplies actual borrowed
candidate objects to `SizeSetChartReplacementContext::new(source_inputs, inventory)`. An inventory
must match the source input family; this operation replaces a chart within that family. It does
not infer body/Ease versus garment provenance or convert one input model into the other.

Each candidate is validated as a composite through the source's original chart context, membership,
axes and break inventory. Original canonical observations or body/Ease metadata remain authoritative.
A candidate valid against a different observation inventory can therefore refuse in this context.
Repeated candidate IDs refuse, and another object cannot substitute for the original source chart's
ID. Explicitly selecting the original actual chart is valid intent. Candidate chart IDs also cannot
alias a transformation's own identity.

```rust,ignore
let candidates = [&replacement_chart];
let context = SizeSetChartReplacementContext::new(
    source_inputs, SizeSetReplacementCharts::Garment(&candidates),
)?;
let binding = SizeSetChartReplacement::Garment(replacement_chart.id());
let candidate_inputs = context.resolve(binding)?;
```

`resolve` returns an actual validated `SizeSetInputs` view borrowing the selected chart with the
source's original membership/axes/breaks. Missing IDs and wrong families refuse explicitly. A draft
candidate stays a draft: path completeness remains an explicit query. MTM candidates retain their
original body/Ease sources and always refuse grading, naming the selected candidate chart/member.
Obtaining the candidate view does not retarget or change readiness of the original source view.

An authored `ChartReplacement(binding)` transformation uses the corresponding explicit kind and
one of the five states. Construct it with `SizeSetTransformation::new_with_context` and recheck with
`validate_current_with_context`. These methods retain exact source, declaration and candidate
failures. The source-only methods return `ChartContextRequired` for an authored replacement; a bare
chart ID never certifies its target. Unknown/Derived declarations have no authored binding and
retain their original observation/evaluation refusals. Evidence and numeric states are not upgraded.

## Immutable resolution intent

`SizeSetResolutionIntentDefinition` combines an immutable `origin` and an authored ordered
`transformations` vector. `SizeSetResolutionIntent` retains their exact profile/schema/content pin,
original/resolved references, provenance and every declaration/state. An empty vector records no
declared override. Unresolved declarations remain representable without inventing a payload.

```rust,ignore
let intent = SizeSetResolutionIntent::new_with_context(
    SizeSetResolutionIntentDefinition { origin, transformations }, &context,
)?;
```

Use `new`/`validate_current` with source inputs when no authored chart replacement needs candidates;
use `new_with_context`/`validate_current_with_context` for actual replacement targets. Each declaration
rechecks the original source pin and current bindings. Original typed errors remain in the error
chain, with the failing transformation ID. Duplicate declaration IDs and collisions with resolved
identity/origin provenance refuse. A resolved target cannot alias supplied candidate charts or a
transformation's source record. No source is retargeted and no authored vector is sorted.

The aggregate records locally valid intent. Declaration order is retained, while G4 owns precedence,
conflicts and composition; collecting declarations does not certify their combined effect. Numeric
states stay canonical. These constructors execute no profile, multiplication, chart change or Design
revision; they produce no resolved geometry, storage, approval or release artifact. G3/G4 and
integration owners retain application, equivalence, global freshness, evidence truth and serialization.
