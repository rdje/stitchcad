# Executable body-to-garment Ease intent

G1-SLICE.4b.1 implements individual immutable Ease mappings in sc-measure. The normative contract
is [ontology §2.2](ontology.md#22-ease). An Ease set, per-POM lookup and table membership follow in
.4b.2; formula evaluation, regenerated geometry, physical fit and release approval remain later proofs.
This body-to-garment mapping is separate from a sewing span's signed A-minus-B differential.

## Authored fields and current queries

| Field | Meaning |
| --- | --- |
| Identity | Stable mapping identity, distinct from supplied measurements and target records |
| Body binding | Saved measurement id, exact token, Body kind and canonical scalar id |
| Garment binding | Saved measurement id, exact token, Garment kind and canonical scalar id |
| Amount declaration | One canonical signed LengthDeclaration, distinct from both measurement scalars |
| Fit intent | Ordered Close < Semi < Loose, corresponding to `close`, `semi`, `loose` |
| Mapping provenance | Reference owning correspondence and fit intent |
| Compression permission | Explicit Forbidden or Declared with its own provenance reference |

The amount declaration supplies state, source and state-specific provenance. The mapping never copies
its number or uncertainty state. Known, assumed and preference amounts carry authored values; unknown
amounts require observation; derived amounts require evaluation. IDs alone do not establish a source's
existence, truth, applicability or human attribution. Design registries and G4 own that validation.

Construction checks both saved bindings and all current metadata targets using the existing borrowed
MeasurementTableContext. Its inventory has unambiguous measurement and target identities. The body
must be Body and the output must be Garment; same tokens in distinct measurement namespaces are legal.
This individual mapping does not yet certify membership of either measurement in a selected table.

Each query resolves the saved measurement identity before comparing the exact token, kind and scalar
identity. Same-token peers never replace missing records. Changes to any saved binding field require
an explicit validated replacement. Same-id scalar values, state and source remain visible; landmark
and procedure revisions are validated in the supplied current context. No copied birth-time approval
or current Design-revision certificate is kept.

The selected-side measurement query validates that side and mapping identity. The declaration and
authored-value queries validate both sides and the current amount. Full current validation checks the
same complete contract. Earlier immutable mappings remain unchanged after replacement.

## Compression and unresolved inputs

A negative amount means compression and requires an explicit Declared permission for this mapping.
Neither a Close fit class nor a negative source number creates that permission. Zero and positive
amounts need no compression authorization; signed values are retained exactly, without clamping.

Unknown and derived drafts can be saved and inspected without a numeric fallback. Their numeric
query refuses with the canonical observation or formula identity. A later evaluator must apply the
mapping's amount-validation operation to its result, including a negative derived result; this check
alone does not certify evaluation provenance or current inputs.

This structural ability to describe declared compression does **not** expand the v1 envelope:
[the feature matrix](feature-matrix.md) rejects negative ease for its supported garments. G3 must
also enforce that scoped envelope rule. No numerical fit thresholds, material stretch claims or
physical construction defaults are introduced here.

## Illustrative authored mapping

These are **assumed fixture values**, not prescribed body measurements or a fit recommendation:

| Input | Value | State |
| --- | --- | --- |
| Body waist | 74 cm | assumed, with an assumption record |
| Garment waist POM | 78 cm | assumed, with an assumption record |
| Waist ease | +4 cm | assumed, with a separate canonical amount declaration |

The mapping links Body waist to Garment waist, selects Semi intent and Forbidden compression.
Its provenance identifies the correspondence/intent record. Querying the ease returns its authored
+4 cm; it does not compute or independently prove 78 = 74 + 4. Formula execution and regeneration
must later establish that relationship. If the amount changes to −1 cm, current validation refuses
until the mapping is explicitly replaced with a compression declaration, and the envelope still
needs to permit the intended construction. If it becomes unknown, inspection shows the observation
request and the numeric query refuses; the prior +4 cm is never retained as a fallback.

## Public API vocabulary

| API | Contract |
| --- | --- |
| `Ease` | Immutable individual body-to-garment mapping |
| `EaseDefinition` | Authored bindings, amount identity, fit and provenance |
| `FitIntent` | Ordered Close, Semi and Loose classes |
| `EaseSide` | Select Body source or Garment output |
| `CompressionPermission` | Forbidden, or Declared with provenance |
| `EaseError` | Scoped identity, domain, binding, metadata, amount or compression refusal |

The mapping's measurement query borrows selected current metadata. Its declaration query borrows
current amount/state/source; its authored-value query refuses unresolved sources. Amount validation
checks permission for a supplied signed result. Structured errors retain mapping, side and missing
or reassigned identities; metadata/value errors expose their original error source.

The MCP/API façade will expose the same semantic contracts when G1 .6/.9 lands. No executable MCP
server or user-facing application is claimed by this library slice.

## Verification boundary

The tracked individual-Ease tests exercise ordered fit classes, canonical borrowing, required side
kinds, distinct amount identity, current identity/binding/metadata changes, signed permission in all
value-bearing states, unknown/derived refusal, same-token namespaces and reordered current inputs.
The tracked mutation diagnostic removes production guards and requires real assertion failures,
then restores source byte-identically. Rust strict checks and the browser-target build verify the
library integration. These are local structural checks; physical fit and production signoff remain
G2/G3/G4/G6 and independent G7 obligations.
