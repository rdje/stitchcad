# Canonical measurement length inputs

[Ontology §2.1/§5](ontology.md) requires measurement state and provenance as content. Measurement,
ease and chart metadata need one canonical source; `G1-SLICE.4a.1` supplies the shared length
contract in sc-core, consumed by [sc-measure metadata](measurement-metadata.md). The measurement
table, ease mapping and SizeSet are subsequent owned slices. No standard's measurement
values or landmark vocabulary are invented here ([standards](standards.md)).

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

## Evidence and computation remain distinct

Every declaration reports `ValueProvenanceValidation::DeferredToDesignAndG4`. Carrying evidence ids
and a declared Known state cannot establish that records exist, cover this quantity, remain valid,
or prove the source true. Assumptions need their recorded human actor. Recipe/Design validates
source/declaration/formula identities, kinds and dependencies; G1-SLICE.5 propagates derived state.
G4 validates scoped evidence, preference composition and artifact-specific uncertainty closure/policy.
A query of authored content grants neither factual certification nor release permission.

The dependency direction is explicit: sc-measure consumes core identities and these inputs; core
does not depend on sc-measure. Formula evaluation will consume its core input contract through the
host context. Consumers borrow canonical declarations instead of keeping independent numeric/state
copies. Structural metadata/reference APIs are available now; executed evaluation follows.

## Stable machine tokens

Measurement metadata carries a core `MachineToken`, separate from its localized name and scalar
input. `MachineToken::new` preserves exact ASCII lower-snake spelling or returns `MachineTokenError`:
`InvalidSyntax` retains malformed input; `ReservedKeyword` names let/assert/if. There is no trimming,
normalization or automatic renaming. For example, waist_girth and waist_girth_2 are valid, while
Waist_Girth, waist__girth and a token containing a Cyrillic lookalike are refused. The exact grammar
lives in [formula syntax §1](formula-language/grammar.md). Private representation prevents unchecked
mutation; a validated replacement leaves the original token unchanged.

The built-in eps_geo is a valid reference token. Lexical acceptance grants no permission to bind a
new measurement to it: MachineToken.is_reserved_input_name classifies the eight built-in names;
measurement metadata refuses their redeclaration. Recipe namespace validation retains the same rule. The token API provides no source truth, measurement value or display text.

| API token | Meaning in the length-input implementation |
| --- | --- |
| `LengthState` | Exactly one authored state with required state-specific provenance |
| `LengthDeclarationDefinition` | Editable stable id, source record and canonical length/state input |
| `LengthDeclaration` | Immutable structurally validated input |
| `LengthDeclarationError` | Missing/duplicate known-state evidence inventory |
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
