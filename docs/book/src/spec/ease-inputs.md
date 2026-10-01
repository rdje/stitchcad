# Executable body-to-garment Ease intent

G1-SLICE.4b.1/.4b.2 implement immutable individual Ease mappings and per-POM Ease sets in sc-measure.
The normative contract is [ontology §2.2](ontology.md#22-ease). Current mapping/table membership is
checked; formula evaluation, regenerated geometry, physical fit and release approval remain later proofs.
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
An individual mapping alone does not certify table membership; the Ease set below checks it.

Each query resolves the saved measurement identity before comparing the exact token, kind and scalar
identity. Same-token peers never replace missing records. Changes to any saved binding field require
an explicit validated replacement. Same-id scalar values, state and source remain visible; landmark
and procedure revisions are validated in the supplied current context. No copied birth-time approval
or current Design-revision certificate is kept.

The selected-side measurement query validates that side and mapping identity. The declaration and
authored-value queries validate both sides and the current amount. Full current validation checks the
same complete contract. Earlier immutable mappings remain unchanged after replacement.

## Per-POM sets and table membership

An EaseSet has a stable identity, a body-table identity, a garment-table identity and authored ordered
bindings. Both sides may name one mixed MeasurementTable. Each binding carries a set-owned machine
token, canonical Ease identity, expected body/POM bindings and expected amount identity. These fields
pin the mapping's targets; no value, fit, uncertainty state or provenance is copied into the set.

Mapping identities, set tokens and POM identities must each be unique. Several different POMs may
share a body measurement or an amount declaration. Tokens are unique within one set; unrelated sets
can use the same spelling. An empty draft is legal with existing table references and does not assert
complete chart coverage. An unmapped POM is a typed refusal, never zero ease or a default mapping.

The borrowed context rejects duplicate and cross-kind identities across tables, mappings, metadata
and canonical target records before lookup. It does not certify every unrelated supplied object.
Set lookup resolves the saved Ease identity, checks its saved target expectations, requires the body
and POM to belong to their named current tables, then validates the current mapping and amount.
A same-content table with a different identity or a same-POM replacement mapping cannot take over
missing references. Current table binding changes and invalid required metadata remain typed refusals.

Retargeting a same-id mapping's body/POM binding or amount identity requires explicit set replacement.
Current fit, mapping/compression provenance and permission edits remain canonical and visible; current
amount values, state and source are borrowed. A new negative amount or revoked permission is checked
again. Same-id fit edits therefore do not preserve a stale classification in a copied set snapshot.

POM, token and mapping-ID queries validate the selected entry and both table memberships. Full current
validation checks every authored entry. A missing unrelated mapping or unmapped invalid table member
does not block a valid selected query; that query is not whole-table, Design or release certification.
Authored order is retained but lookup identity is independent of supplied inventory order.

For the assumed waist example below, the set names the table(s) containing the Body waist and Garment
waist POM, then binds that mapping with an explicit machine token. Calling the POM query returns the
current canonical mapping; calling its amount query returns the current declaration. Removing waist
from the named garment table refuses even if its metadata still exists elsewhere. Removing the Ease
identity refuses even if a new mapping claims the same waist POM. No peer is selected implicitly.

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
| `EaseBinding` | Set token and stable mapping/body/POM/amount target expectations |
| `EaseSet` | Immutable ordered unique per-POM mapping namespace |
| `EaseSetDefinition` | Set identity, selected tables and authored bindings |
| `EaseSetContext` | Borrowed unambiguous current tables, mappings and metadata |
| `EaseSetError` | Scoped namespace, current mapping or table-membership refusal |

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

Set contracts add per-POM/id/token lookup, three uniqueness rules, shared body/amount sources, mixed
tables, empty drafts, current context collisions, missing/reassigned mappings/tables, current intent/
state edits and selected versus full validation. Ten production guard removals must fail real test
assertions and restore exact source. These local checks do not claim observed remote CI for this slice.

## Structural Ease review at G1

G1-SLICE.4b.3 reviews the individual and set APIs against ontology §2.2:

| Required contract | Executable representation | Verified boundary |
| --- | --- | --- |
| Body source | Saved Body MeasurementBinding | Wrong domain, missing/reassigned metadata and foreign table membership refuse |
| Produced garment POM | Saved Garment MeasurementBinding; unique set POM | Wrong domain, duplicate POM, missing mapping and current table membership checked |
| Signed amount | Distinct current LengthDeclaration | Exact sign retained; negative numeric states and supplied evaluated results need explicit permission |
| Ordered fit intent | FitIntent Close < Semi < Loose | Order verified; current same-id fit edits visible; quantitative chart/fit criteria deferred |
| State | Sole canonical LengthState | Current edits visible; unknown/derived numeric queries refuse without fallback |
| Provenance | Amount source/state records, mapping and compression references | References retained; existence/scope/truth/human attribution deferred to Design/G4 |
| Per-POM relationship | EaseSet bound to current mappings and table identities | POM/id/token lookup, unique targets and explicit replacement; no inferred complete chart coverage |

The Rust contracts and privacy tests prove the structural family. The tracked guard diagnostics
have observed seven individual and ten set guard removals failing assertions, then restored exact
source. Their regression tests run in the standard Rust gate; the destructive mutation diagnostics
must run sequentially before a restored build. The milestone probe suite additionally verifies the
repository/spec instruments, including formula, glossary, uncertainty, feature and release contracts.

This closes .4b's structural scope, not G1-SLICE.4: SizeSet and combined family review remain. Actual
body-plus-ease evaluation/current Design registry belongs to G1 .5/.6; chart reconciliation and
physical fit to G2/G3; scoped evidence and envelope/export policy to G3/G4; independent conformance
and production declaration to G6/G7. Local native/WASM proof is not a remote-CI verdict or permission
to release a garment.
