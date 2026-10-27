# Canonical measurement length inputs

[Ontology §2.1/§5](ontology.md) requires canonical state/provenance. Core owns declarations;
[sc-measure metadata](measurement-metadata.md), tables, [Ease](ease-inputs.md) and structural SizeSet
composition borrow them. Numeric execution/persistence remain separate. No standard's values or
landmark vocabulary are invented ([standards](standards.md)).

## Authored value and state

`LengthDeclarationDefinition` carries stable declaration id, source-record identity and exactly one
`LengthState`. `LengthDeclaration::new` returns immutable content or a structural refusal:

| State | Authored content | Numeric query |
| --- | --- | --- |
| Known | Length and nonempty distinct evidence-record references | Returns authored value; evidence not independently proved |
| Assumed | Length and recorded human-assumption reference | Returns authored value with its assumption retained |
| Unknown | Required observation-request reference | RequiresObservation; no fallback value |
| Preference | Length and preference-provenance reference | Returns the explicit preference; no substitution over another unknown |
| Derived | Sole formula reference, with no independently entered result | RequiresEvaluation; recipe must compute and propagate input state |

For example, a fixture waist declaration can retain its explicitly assumed 760000 µm and assumption
record. A different unknown waist declaration names an observation request and returns a refusal,
even if a separate preference declaration contains zero. An explicitly authored zero or signed
length remains unchanged; the general declaration imposes no invented body-measurement positivity
rule. Each procedure's domain constraints and actual repeatability are separate validation.
A fractional-inch input converts once through sc-units, then the declaration retains internal Length;
original entered units are retained in the measurement metadata.

`state()` borrows the canonical authored state and `definition()` exposes it without mutation.
Replacing a cloned definition with a new value/state/source cannot alter the prior object. Known
input with no evidence or repeated evidence id is refused. Unknown and derived input have no numeric
field, enforced by compile-fail examples; private validated fields prevent unchecked mutation.
`authored_value()` returns a typed refusal naming declaration and observation/formula when no value
exists. This query does not evaluate a formula, resolve a profile, or authorize export.

## Canonical general scalar content

ScalarInputDeclaration supplies immutable canonical scalar content for the general parameter/profile/
material integration, without a registry lookup or formula execution. Its definition carries a
stable id, source, ScalarInputKind and exactly one ScalarInputState. It extends the same authored
state/provenance contract used by LengthDeclaration; metadata consumers must borrow the canonical
target rather than author an independent copy. Existing measurement/Ease Length records remain
singly owned and retain their dedicated adapter.

| Kind | Authored-value construction | Canonical content |
| --- | --- | --- |
| Length | `From<sc_units::Length>` | Validated signed micrometres, including zero and both scalar limits |
| Angle | from_angle_microdegrees(i64) | Signed raw microdegrees; -720000000 retains two negative turns |
| Ratio | from_ratio_ppm(i64) | Signed ppm; no implicit zero-to-one restriction |
| Count | from_count(i64) | Zero through i64 MAX; negative input returns NegativeCount with its actual value |
| Boolean | from_boolean(bool) | False and true remain Boolean values, distinct from numeric zero/one |
| Area | No authored-value constructor | Only Derived with its sole formula identity; the result must be computed |

The [formula kind contract](formula-language.md#2-values-and-kinds) makes Area a derived product,
never an independently authored input. Accordingly an Area declaration in Known, Assumed, Unknown
or Preference state returns AreaMustBeDerived with its actual state tag. An unknown source needed
for an area calculation remains on its dependency; no separately entered Area substitutes for it.

Known/Assumed/Preference values must match the declared kind. Unknown and Derived have no authored
value field; their kind remains inspectable. Structural validation refuses non-derived Area first,
then a present kind mismatch, then missing/duplicate Known evidence. Known references stay in their
authored order; a duplicate refusal identifies the first repeated identity. All five states retain
the same evidence/assumption/observation/preference/formula obligations described above.

```rust
use sc_core::{ontology::EntityId, value::{ScalarInputDeclaration,
    ScalarInputDeclarationDefinition, ScalarInputKind, ScalarInputState, ScalarInputValue}};

let record = ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
    id: EntityId::from_bits(1), source: EntityId::from_bits(2),
    kind: ScalarInputKind::Boolean,
    state: ScalarInputState::Preference {
        value: ScalarInputValue::from_boolean(false), provenance: EntityId::from_bits(3),
    },
})?;
assert_eq!(record.authored_value()?.boolean(), Some(false));
assert_eq!(record.kind(), ScalarInputKind::Boolean);
# Ok::<(), Box<dyn std::error::Error>>(())
```

Readability does not certify facts; the common boundary below applies to both declaration families.

## Evidence and computation remain distinct

All declarations retain `ValueProvenanceValidation::DeferredToDesignAndG4`: evidence ids do not prove
existence, scope, validity or truth; assumptions need their human actor. Recipe/Design checks source,
declaration and formula identities, kinds and dependencies; G1-SLICE.5 propagates derived state. G4
checks scoped evidence, preference composition and artifact closure/policy. No authored query authorizes
release. Core owns records; sc-measure and formula host contexts borrow them without extra value/state
copies or a reverse crate dependency. Metadata/reference APIs exist; executed evaluation follows.

## Owned canonical inputs

`CanonicalInputs::new(lengths, scalars)` owns both exact declaration families in their authored order.
Owned ids must be unique across both vectors; even identical duplicates refuse with
`CanonicalInputPosition` witnesses for first/repeated family and index. Lengths are checked first.
Source/evidence/formula references stay unchanged and are not counted as declarations owned here.
Empty vectors are legal; no records or numeric defaults are manufactured.

`lengths()`/`scalars()` and `length(id)`/`scalar(id)` borrow canonical objects. A general Length-kind
scalar stays general: querying its id with length(id) returns None, without converting it. For example,
a dedicated length id1 plus general scalar id1 refuses; distinct ids retain all states/provenance.
This component grants no metadata binding, current Design revision, evidence truth or release readiness.

## Owned Design parameters

`sc_core::design::DesignParameters::new(definitions, &inputs)` owns named parameter metadata without
copying canonical values/states. Every definition supplies distinct metadata id, exact MachineToken
and DesignParameterTarget: dedicated Length(id), or general Scalar with declaration id/expected kind.
Family and kind stay pinned; a general Length scalar never substitutes for a dedicated declaration.

Metadata ids/tokens must be unique, ids disjoint from canonical declaration ids, and the eight reserved
input names cannot be rebound. These checks precede target resolution in authored order. Distinct
names may share a canonical target; empty drafts accept. No token, declaration or default is generated.
For example, width/hem_depth can both name dedicated declaration1; width/width refuses even if both
name1. Metadata id1 refuses if canonical declaration1 exists. A missing dedicated target1 cannot be
repaired by a general Length scalar1. Unknown/Derived targets remain legal inspectable content.

parameters()/parameter(id) borrow canonical metadata. A parameter's declaration(&inputs) resolves only
its held target against current records, using the existing formula scalar adapter for exact kinds.
Dedicated metadata retains an actual LengthDeclaration borrow; general metadata retains checked
id/kind for that adapter, without a value cache. Same-id state edits remain current: a Boolean Unknown
replaced by an explicit false Preference reads false with its actual provenance. A same-id Angle
replacement refuses the pinned Boolean kind, and deletion retains the missing original id.

validate_current(&inputs) checks every target and rechecks metadata/canonical id disjointness after
inventory replacement. Shared definitions cannot mutate the registry; detached edits require a new
validated replacement. This parameter registry does not certify other Design entities, cross-domain
name composition, measurement/Ease/profile/material membership, source/evidence truth or readiness.
Declaration source references retain their authored meaning; no equality with the metadata id is inferred.

## Stable machine tokens

Measurement metadata uses core `MachineToken` for exact ASCII lower-snake identifiers, separately from
localized labels and scalar inputs. InvalidSyntax retains malformed input; ReservedKeyword names
let/assert/if. No trim, normalization or automatic rename occurs. For example, waist_girth and
waist_girth_2 are valid; Waist_Girth, waist__girth and Cyrillic lookalikes refuse. See
[formula grammar](formula-language/grammar.md). Replacement cannot mutate an earlier token.

Lexical acceptance does not authorize binding: eps_geo is a valid reference, while
MachineToken.is_reserved_input_name identifies the eight names metadata/recipe namespaces cannot
rebind. Tokens prove no source truth, value or display text.

| API token | Meaning in the length-input implementation |
| --- | --- |
| `LengthState` | Exactly one authored state with required state-specific provenance |
| `LengthDeclarationDefinition` | Editable stable id, source record and canonical length/state input |
| `LengthDeclaration` | Immutable structurally validated input |
| `LengthDeclarationError` | Missing/duplicate known-state evidence inventory |
| `CanonicalInputs` | Immutable owned dedicated-length/general-scalar records |
| `CanonicalInputsError` | Repeated owned identity with both original positions |
| `CanonicalInputPosition` | Dedicated-length or general-scalar family and index |
| `DesignParameters` | Immutable ordered named Design parameter registry |
| `DesignParameterDefinition` | Editable metadata id, token and pinned canonical target |
| `DesignParameterTarget` | Dedicated Length id or general Scalar id/expected kind |
| `DesignParameter` | Immutable metadata with a current-target declaration query |
| `DesignParameterError` | Actual identity/name/target/kind refusal |
| `NoEvidence` | Claimed known value has no evidence references |
| `DuplicateEvidence` | Known inventory repeats an evidence id |
| `LengthValueError` | Numeric value requires observation or evaluation |
| `RequiresObservation` | Declaration and observation request; no numeric fallback |
| `RequiresEvaluation` | Declaration and sole formula source; no authored derived result |
| `ValueProvenanceValidation` | Registry/evidence/uncertainty policy still need their owned proofs |
| `DeferredToDesignAndG4` | Content is inspectable without certifying factual truth or exportability |
| `MachineToken` | Validated exact ASCII lower-snake identifier; not a display label |
| `MachineTokenError` | Invalid spelling or grammar-keyword refusal |
| `InvalidSyntax` | Authored token does not meet the shared identifier grammar |
| `ReservedKeyword` | let/assert/if cannot name an identifier |
