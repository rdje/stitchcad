# Executable garment chart observations

G1-SLICE.4c.3a implements one authored garment-chart observation for one size member and one logical
Design POM. It builds on [size membership](size-membership.md) and [measurement tables](measurement-metadata.md).
The normative chart contract is [size sets §5](size-sets.md). Chart assembly and complete coverage,
MTM body inputs, axes, breaks, profile resolution and instantiation remain separate owned slices.

## One member, one named quantity, current records

A SizeChartObservation pins its own identity, a SizeSetReference, a stable member identity, the Design
and chart table identities, two MeasurementBindings and a correspondence provenance record. Both
measurements must be garment POMs. A body measurement is refused in either role; converting body inputs
through Ease belongs to the MTM slice. The provenance reference records authored correspondence; it
does not certify the record, its source truth or physical equivalence.

The Design POM identifies the quantity the recipe names. The chart measurement records the authored
value for this member. Its exact declaration, state, source, entered unit, landmarks and procedure are
borrowed from canonical current metadata. Nothing copies a numeric result into the observation.
The Design input is not a measurement taken from regenerated geometry. G3 must separately generate
that result and compare it to the chart under the declared tolerance.

For example, a Design table names garment waist. Its base-size input is an assumed 74 cm; a chart table
contains an assumed 78 cm waist for member "M". An observation explicitly links those two measurement
identities to that member and a correspondence record. Reading the observation yields the canonical
78 cm declaration with its assumption and source. It neither adds the 4 cm difference to a contour nor
proves that either input measures the same physical waist. Those are construction/equivalence proofs.

An observation for the base member may use the same authored measurement in both roles and the same
table. This does not require two copies of the base scalar. The generated geometry result must still
be a distinct measurement; sharing an authored input cannot establish regeneration equivalence.

## References that refuse substitution

Every selected metadata query checks the observation identity, exact set identity/revision and stable
member before resolving the named table and saved measurement identity. Tokens, kind and scalar
identity must match the saved binding. Both required metadata targets and table membership are checked.
A context rejects duplicate table identities and collisions between set/member/table/measurement records.

Examples:

- Removing member "M" and introducing another member named "M" leaves the observation unresolved.
  The replacement label never restores the old member identity.
- Supplying revision 3 to an observation pinned to revision 2 refuses. Supplying a different set with
  the same labels also refuses. These local checks do not certify the command registry's current Design.
- A new table with identical entries cannot replace a missing named table implicitly.
- Explicitly updating a table after renaming a measurement token does not update an old observation.
  Its saved token still differs; construct a validated observation replacement to retarget it.
- Reordering the supplied inventory changes no lookup. Same-id changes to scalar state/source, metadata
  display name, entered unit or documented procedure remain visible rather than creating stale caches.

A selected-role metadata query validates only that role and member reference. Full current validation
checks both required roles. Declaration and numeric queries also require both roles, so a readable
chart measurement alone cannot hide a missing Design POM. None of these queries certifies unrelated
table entries, complete chart coverage, correspondence truth, geometry or release approval.

## Explicit uncertainty and immutable replacement

Unknown and derived chart drafts can be constructed and inspected. A numeric query returns the exact
required observation or formula evaluation instead of zero, the Design input or another member's value.
Known, assumed and preference values retain their authored state/provenance; structural validation does
not promote them to factual certainty. Physical procedure domains are a separate validation obligation.

Definitions are cloneable inputs; validated observations are private and immutable. An explicit
replacement can update references and provenance without rewriting the old observation. Collection
identity uniqueness, member/POM completeness and permitted sharing follow at .4c.3b; MTM .3c and
break/composite .3d remain owned. No axis model is chosen while D70 is pending.

## Public API vocabulary

| API | Contract |
| --- | --- |
| `SizeChartObservationDefinition` | Authored member, revision, table and two measurement bindings |
| `SizeChartObservation` | Immutable current-reference-validated authored correspondence |
| `SizeChartContext` | Borrowed membership, named tables and canonical metadata records |
| `SizeChartRole` | Logical Design POM or authored garment observation |
| `SizeChartError` | Scoped identity, kind, membership, table, binding or unresolved-value refusal |

Sixteen contracts and a privacy compile-fail test cover borrowing, missing/replaced identities,
revision/domain checks, explicit table/observation retargeting, current metadata and unresolved values.
Eight deliberate production mutations must fail actual regression assertions and restore exact source.
The strict native/WASM gates and warning-free book check the integration; the API is not an app or MCP server.
