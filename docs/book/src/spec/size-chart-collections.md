# Executable garment chart collections

G1-SLICE.4c.3b implements an immutable SizeChart over canonical [garment observations](size-chart-observations.md).
The normative contract is [size sets §5](size-sets.md). Each authored cell names a stable member and
logical Design POM. [MTM body/Ease input charts](mtm-input-charts.md) are now executable. Axes, breaks, profile resolution
and actual instantiation remain owned by later slices; a structurally complete garment chart is not yet a complete SizeSet.

## Targets and canonical observations

A SizeChartDefinition pins the chart identity, SizeSetReference and named Design table, with an
explicit ordered POM target list and an authored observation inventory. POM targets must be garment
bindings with unique identities and tokens. Observation identities and member/POM cells must be unique.
Two individually valid observations for the same cell are still an ambiguous chart and are refused.

SizeChartBindings capture expected observation, member, revision, table and measurement targets.
SizeChartCollectionContext borrows the canonical observations and their current membership/table/
measurement context. The collection rejects aliases and compares saved targets before using a current
observation. Correspondence provenance, value, uncertainty and source remain in their canonical records.
Changing same-id provenance or scalar state/source is visible; retargeting needs explicit reconstruction.

The Design table may mix body inputs and garment POMs. Only its garment POMs are chart targets. An
observation that names another Design table is refused even if that table has identical entries.
A missing observation cannot be replaced by a peer for the same member/POM. Labels, positions and
matching table contents never substitute for the saved identities.

## Draft validity and complete coverage

Construction and current validation check every authored target and observation, while allowing
incomplete drafts. A draft can have no observations, or a partial target inventory. Even an empty draft
must retain a valid chart identity, exact membership reference and named Design table. It is inspectable,
but this does not establish a complete chart or readiness to instantiate.

Complete validation adds three requirements:

1. The authored POM target inventory is nonempty. An empty or body-only Design table does not turn an
   empty chart into a completed garment chart.
2. The entire current named Design table is valid, and every garment POM it declares appears among the
   chart's targets. An omitted POM cannot be hidden by reducing the chart's own target list. Body inputs
   need valid metadata in this full-table check, but do not require garment chart cells.
3. Every current member has exactly one authored observation for every target POM. Duplicate cells are
   already refused at construction; missing cells name the chart, member and POM without interpolation.

For example, a Design names waist and hip, and membership order is "M, S". Declaring target order
"hip, waist" produces these required cells, regardless of observation inventory order:

| Member | Hip observation | Waist observation |
| --- | --- | --- |
| M | required | required |
| S | required | required |

If M's waist observation is missing, the chart can remain a draft; complete validation names that exact
cell. Removing waist from the target list instead names the omitted Design POM. Adding skirt length to
the current Design table invalidates earlier complete coverage until the chart explicitly adds the
new target and observations for both members. No old coverage result is cached.

Unknown and derived values can satisfy structural coverage. Their numeric queries still require the
named observation or formula evaluation. Complete structure does not prove source truth, physical
quantity correspondence, chart-to-geometry equivalence, path readiness, global revision currentness
or release eligibility. Those proofs retain their G3/G4/G6/G7 owners.

## Ordered queries and explicit sharing

Member queries borrow the current authored membership sequence. Row queries return observations in
the chart's explicit POM order, not the order of canonical inventory, entity identifiers or labels.
A row query requires all its declared cells but does not certify other rows or full Design coverage.
A selected cell query validates that member/POM and its required current observation targets; declaration
and numeric queries borrow the canonical cell input without conversion or a numeric cache.

Two members may explicitly reference the same canonical measurement input through distinct member-
pinned observations with their own correspondence provenance. This records an authored assumption or
measurement scope; it does not assert two physical garments were measured independently. The same
canonical observation may also be explicitly referenced by different chart collections. Repeating an
observation identity inside one chart is refused, and a missing cell never borrows a neighbour's value.

A validated chart is private and immutable. To revise targets, rows or references, clone the definition
and construct a validated replacement. The prior chart retains its exact saved expectations and can
still refuse against a retargeted current inventory; reconstruction does not rewrite its history.

## Public API vocabulary

| API | Contract |
| --- | --- |
| `SizeChartDefinition` | Authored chart identity, membership, Design table, ordered targets and cells |
| `SizeChart` | Immutable draft with explicit current and complete-coverage queries |
| `SizeChartBinding` | Saved canonical observation/reference targets without cached values or provenance |
| `SizeChartCollectionContext` | Borrowed canonical observations and current metadata context |
| `SizeChartCollectionError` | Typed identity, domain, current-target or coverage refusal |

Eighteen contracts/privacy and fourteen deliberate production mutations check coverage/currentness,
including a deliberately reduced target inventory, valid duplicate cells, table expansion and an
introduced numeric zero fallback. Exact source restoration, strict native/WASM checks, a warning-free
book and focused doctrine checks verify this integration. No app, command bus or MCP server is claimed.
