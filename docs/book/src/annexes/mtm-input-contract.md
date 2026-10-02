# Annex: made-to-measure input contract

This is the technical reference for the [made-to-measure introduction](../spec/mtm-input-charts.md).
G1-SLICE.4c.3c implements these structural library contracts in sc-measure. The governing requirements
are [size sets §11](../spec/size-sets.md#11-made-to-measure-is-a-size-set-of-one) and
[regeneration](../spec/instantiation-paths.md). Geometry evaluation, full SizeSet and release are later work.

## A custom member of one

MtmChartDefinition pins the chart identity, SizeSetReference, sole member identity, expected Ease-set
reference targets and chart correspondence provenance. The membership must use Custom designation
intent and contain exactly one member. Its authored base is therefore that member. The chart names
the existing stable identity; a replacement with the same label cannot inherit it. Exact set identity
and revision are checked before current inputs are resolved.

The expected EaseSetDefinition is a reference snapshot: canonical set identity, both named tables and
ordered mapping bindings. It contains no copied numeric value, fit, uncertainty state or mapping
provenance. The authoritative Ease set remains a separate canonical object borrowed through
MtmChartContext. A changed table, mapping token/target/amount or authored mapping order requires an
explicit chart replacement. Reordered context inventory has no effect on identity-based lookup.

The context rejects duplicate and cross-kind identities across membership, members, Ease sets and
their supplied canonical targets. Chart correspondence provenance remains authored content; source
existence, truth, measurement scope and human attribution retain their Design/G4 validation owners.

## Body, Ease and garment are different roles

A selected POM query borrows the current canonical Ease mapping and checks its required body/POM
metadata, both named table memberships and current signed amount/compression permission. Body and
Ease declaration queries borrow separate canonical values, states and sources. The garment query
borrows logical target metadata, without evaluating the target or measuring generated geometry.

For example, these illustrative assumed inputs describe an authored waist correspondence:

| Role | Input | Meaning |
| --- | --- | --- |
| Body | 74 cm waist | Source measurement taken using the referenced body landmarks/procedure |
| Ease | +4 cm | Separate signed mapping amount and fit/correspondence intent |
| Garment | Waist POM metadata | The named target, with garment landmarks/procedure and scalar identity |

Reading the Body input returns 74 cm with its source and assumption; reading Ease returns +4 cm
with its independent state/source. The MTM chart does not return 74 cm as a garment observation,
add the two inputs, produce a geometry measurement or prove that the intended garment is 78 cm.
Formula evaluation and regeneration must establish the result and preserve provenance separately.
Several targets may explicitly share a body measurement or Ease declaration through the canonical
set; shared input does not prove independent physical measurements or quantity equivalence.

Same-id fit, mapping/compression provenance, permission, scalar value/state/source and documented
metadata changes remain visible through the current records. An edited negative amount must have
explicit compression permission, as in every Ease mapping. This structural representation does not
expand the supported v1 garment envelope, which retains its negative-Ease refusal at G3.

Unknown and derived body or Ease inputs remain inspectable drafts. Body numeric queries name the
required body observation/evaluation; Ease numeric queries retain the mapping's unresolved amount
error. Neither invents zero, reuses an earlier value or returns the other role's value as a fallback.
An authored nominal garment scalar is also not an MTM evaluation result.

## Draft validity, complete coverage and path limits

Construction and current validation check every authored current mapping, but an empty or partial
mapping inventory may remain a draft. An unmapped selected POM is a typed refusal. Complete validation
requires nonempty mappings, a wholly valid current Design table, and an Ease correspondence for every
current garment POM in that table. A reduced mapping subset cannot certify itself as complete; adding
a Design POM invalidates earlier coverage until that new correspondence is authored. Body-only or empty
Design data cannot turn an empty MTM chart into a completed input chart.

Selected POM queries validate their own correspondence and required targets. They do not certify
other mappings or full Design coverage. Complete structural coverage can include unresolved inputs;
it does not authorize recipe execution, establish numeric readiness or validate physical fit.

MTM is a regeneration input. The explicit grade-rule-input query always refuses with chart and sole
member identities, including for an empty draft. It certifies no currentness or regeneration readiness.
No breaks are stored or invented in this object; the complete SizeSet must preserve the empty-break/
path-2 refusal when the composite lands. Axes remain independent and await D70's director ruling.

Definitions are cloneable inputs and validated charts are private and immutable. Reconstruction is
explicit: the old chart retains its prior targets and refuses against retargeted current records.
No body-plus-Ease or garment-result numeric operation is offered by this authored-input API.

## Public API vocabulary

| API | Contract |
| --- | --- |
| `MtmChartDefinition` | Authored single-member references, expected Ease-set targets and provenance |
| `MtmChart` | Immutable body/Ease input correspondence with explicit complete-coverage query |
| `MtmChartContext` | Borrowed current membership, canonical Ease sets and their target records |
| `MtmChartError` | Scoped identity, system/member, reference, mapping, input or grading refusal |

Fifteen contracts and two privacy/role compile-fail tests verify borrowing, exact custom-member scope,
current targets/provenance/state, complete POM coverage, unresolved inputs, compression and grading
refusal. Twelve real production mutations must fail regression assertions, including separate Body
and Ease zero fallbacks and an enabled grade-rule path, then restore exact source. Strict native/WASM,
book and focused doctrine checks verify integration; actual instantiation and release remain later work.
