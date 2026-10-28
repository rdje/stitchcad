# Size sets

> **Status:** normative specification, gate **G0** (roadmap §3.4, §7.5, §8). Implemented by `sc-measure` at
> gate **G1** (leaf `G1-SLICE.4`) and consumed by both instantiation paths at **G3**. The
> [membership foundation](size-membership.md), chart families, axes, signed breaks and current composite input views
> are executable. [Profile pins, origin and transformation intent](size-set-transformations.md) are available;
> break/chart bindings and immutable intent aggregation are executable; SizeSet structural review is complete.
> Combined structural acceptance is reviewed at .4d.2; execution and runtime integration remain later work. The ownership
> question roadmap §3.4 leaves open is decided here and recorded as
> `docs/decisions/decision_size-set-ownership.md`. Terms are in the [glossary](glossary.md).

A size set is the answer to "which sizes does this design exist in, what does each one measure, and how do
they step from one to the next?" It is a first-class object with its own identity, because two designs can
share one, because a factory may resolve one differently, and because a release package must be able to say
exactly which one it cut.

## API and example vocabulary

These names refer to the existing structural interfaces and the explicitly authored examples below.
Method names are qualified by the receiver in their surrounding subsection; local variables are not serialized fields.

| Token | Meaning here |
| --- | --- |
| `GarmentSizeSetInputs` | Borrowed current membership, axes, garment chart and break inventory |
| `MtmSizeSetInputs` | Borrowed current singleton, body/Ease chart and break inventory |
| `SizeSetInputs` | Explicit garment or MTM input view, without inferred family selection |
| `SizeSetInputError` | Original current/composite/path validation refusal |
| `Mtm` | SizeSetInputs::Mtm variant retaining actual MTM inputs |
| `From` | Standard-library conversion trait wrapping an actual validated family view |
| `SizeAxis` | Locally validated authored axis; full contract in the axis chapter |
| `SizeAxisDefinition` | Explicit owning set pin, axis id, name, kind and ordered values |
| `SizeAxisSourceContext` | Borrowed canonical numeric sources used by axes and breaks |
| `SizeAxisValueDefinition` | Authored coordinate identity and explicit source |
| `SizeAxes` | Complete current axes and explicit member points |
| `SizeMemberPoint` | Authored member-to-axis-coordinate identity bindings |
| `SizeBreak` | Locally validated signed adjacent-member/dimension delta declaration |
| `SizeBreakContext` | Actual membership, axes, numeric sources, measurement table and records |
| `SizeBreakDimension` | Explicit required axis/table/measurement binding |
| `SizeBreaks` | Independent authored requirements and available break declarations |
| `MissingDimensions` | SizeBreaksError::MissingDimensions: no explicit grading requirements |
| `validate_current` | Revalidate the selected axis, aggregate or input view against actual supplied records |
| `validate_inputs_for` | Revalidate the composite and require the selected path's structural coverage |
| `require_grade_coverage` | Require multi-member range, explicit dimensions and all adjacent cells |
| `missing` | Query the exact required break cells with no available declaration |
| `authored_value` | Read a break's actual delta declaration, preserving observation/formula refusal |
| `delta` | Canonical signed Length declaration identity or the API borrowing its actual record |
| `dimension` | Exact authored measurement binding or the API borrowing its current measurement |
| `from` | Earlier member id of an authored immediately adjacent break pair |
| `to` | Immediately following member id of that pair |
| `value` | Actual coordinate lookup in the axis example |
| `coordinate` | Authored coordinate helper or aggregate member/axis lookup in the axis example |
| `context` | Actual current SizeBreakContext supplied to the break examples |
| `axis` | Explicit authored axis record or identity, as specified by the receiver/field |
| `id` | Axis-example helper converting explicit integer identities |
| `hip` | Actual named hip measurement record in the break coverage example |
| `waist_step` | Actual authored waist break declaration supplied to the partial inventory |
| `garment_inputs` | Actual validated garment view supplied to the family conversion example |
| `grading` | Example result of requesting GradeRules structural validation; MTM retains its refusal |

## 1. What a size set is, and what it is not

It **is**: an ordered set of size labels, one base size, a chart of points of measure per size, and the
breaks between adjacent sizes, with a declaration of which size-designation system the labels come from.

It is **not** a quantity list. How many of each size to cut is order data, and roadmap §7.5 is explicit that
size-run quantities belong to an Order object rather than to the reusable design. A size set therefore
carries no quantities; a tech pack that needs them records an unresolved input instead of inventing a
ratio, because a ratio baked into a design would make every commercial change a design revision and
stale-ify approvals that geometry never touched (roadmap §9).

## 2. The object

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `size_set_id` | entity id | yes | a ULID, stable across saves and exports |
| `revision` | `Count` | yes | monotonic; a design's reference pins the revision it was authored against |
| `size_system` | enumeration | yes | `en13402`, `astm_d5585`, `alphanumeric`, `numeric` or `custom` (§8) |
| `members` | ordered list of labels | yes | the sizes, in **instantiation order** — not sorted, not derived |
| `base_size` | one label | yes | the base size; SHALL be a member (§4) |
| `chart` | per member, the POMs | yes | values with units, landmarks and states (§5) |
| `breaks` | per axis/POM and adjacent pair | yes for path 2 | the deltas between neighbours (§6) |
| `axes` | named dimensions | yes | at least one authored axis for every complete set (§7) |
| `provenance` | source, state, evidence | yes | where the chart came from, and whether it is `known` |
| `transformation` | record | resolved sets only | what a profile override did, to whom, with what evidence (§9) |

Every value in the object carries an uncertainty state like any other number (ontology §5): a chart nobody
has measured is `assumed` with a named human assumption, a size whose breakpoints a factory has not
confirmed is `unknown` and blocks what the artifact policy matrix says it blocks.

## 3. A label is a name; the order is the sequence

The two are different data and conflating them is a graded-garment defect, so the model keeps them apart
and forbids deriving one from the other:

- **`members` is an explicit ordered list.** The order is authored, not sorted. An alphanumeric sort of
  "XS, S, M, L, XL, 2XL" puts "2XL" first; a numeric sort of "38, 40, 42" is right by accident and wrong the
  moment a set contains "170/88A"; and a custom chart may run in any order its owner chooses.
- **A label is prose, not a token.** Labels are written in quotes in this chapter and are never rendered in
  a machine-token style, because a label is a human-facing name a factory reads and a token is an identifier
  a program reads (`docs/decisions/decision_machine-tokens-declared-where-used.md`).
- **A label never implies a measurement.** Two size sets may both contain a label `12` and mean different
  bodies; the chart is the meaning and the label is the name. A release artifact carries both, so a
  receiver cannot mistake one for the other.
- **Adjacent means adjacent in `members`.** Breaks, grading and "the next size up" are all defined over the
  list order, which is what makes an inserted size an explicit edit rather than a re-sort.

## 4. The base size

Exactly one member is the base. It is the size the design was drafted in, the anchor path 2 grades from, and
the size whose instance a `.rul` table is relative to. Rules:

- the base SHALL be a member of the set — a base outside the set is a different design;
- a profile override that substitutes the base is a **transformation**, recorded with evidence, because it
  changes every graded size rather than one label;
- cumulative breaks are measured from the base and incremental breaks from the previous member
  ([instantiation paths §3](instantiation-paths.md)), so the base is not a display preference: it is what
  the rule table's numbers mean.

## 5. The chart

[Garment chart observations](size-chart-observations.md) implement current member/POM correspondence;
[collections](size-chart-collections.md) check Design/member coverage. Composite inputs join axes/breaks;
[immutable profile intent](size-set-transformations.md) records proposed overrides without applying them.

Per member, the chart carries the garment's points of measure — the same POMs the design's measurement
table names, with landmarks and procedures (ontology §2.1). Two rules make it useful rather than decorative:

- **A chart POM and a regenerated POM are different measurements of the same quantity**, and their
  difference is the equivalence report's subject ([instantiation paths §6](instantiation-paths.md)). The
  chart is what a factory checks the pattern against; the regeneration is what the recipe produced.
- **Every chart value carries its state.** A chart copied from a standard is `assumed` until somebody
  measures a garment; a chart measured on a sample is `known` with evidence scoped to that sample.

## 6. Breaks

A break is the delta between two adjacent members in one dimension — the "4 cm waist jump" of a range.
Breaks belong to the size set rather than to the grade-rule table, because they are a property of the range;
a rule table *implements* them as per-point deltas.

- breaks are lengths in the internal unit, so a break is exact and never rounded twice;
- a break may be zero in a dimension (a range that grows in girth but not in length);
- breaks are declared per adjacent pair, so an uneven range — `+4 cm` then `+5 cm` — is expressible;
- a size set with no breaks can still serve path 1, and SHALL refuse path 2 with a diagnostic naming what
  is missing rather than grading by zero.

### Authored break key — D228 ruling2026-10-26

The director approved an explicit **axis + named POM/dimension + adjacent member pair** key.
An axis can have several measurement dimensions: the same House size step from S to M may carry
waist+4cm and hip+5cm as separate authored breaks. Each delta refers to one canonical signed Length
record, retaining its state, source and provenance rather than copying a chart result. Both body
measurement dimensions and garment POMs retain their explicitly declared domain.

Adjacency follows the authored `members` sequence (§3). Axis, measurement and delta are explicit;
no label parsing, chart subtraction, inferred dimension, implicit zero or first-axis choice supplies
missing data. Individual breaks, coverage and current composite/path checks are implemented below.

### Executable individual breaks — G1-SLICE.4c.3d.1

`sc-measure::SizeBreak` now implements that key over actual current records. Its editable definition
contains a stable break id, exact SizeSet reference, axis id, ordered `from`/`to` member ids, named
dimension-table id, exact `MeasurementBinding` and canonical signed Length delta id. The dimension
binding retains measurement identity, token, body/garment domain and declaration identity. A same-token
peer or same-name axis does not replace that explicitly authored dimension.

Build `SizeBreakContext` from the actual membership, complete axes, shared numeric source inventory,
named measurement table and measurement-record context. Include break delta declarations in the same
`SizeAxisSourceContext` that supplies numeric axis values. Construction validates current axes/table
metadata and rejects conflicting content or record roles across the supplied canonical inventories.
Identical canonical Length content may be shared across them. Metadata identities cannot alias the
set, members, axes, values, table or canonical records. These are local snapshot checks; the caller
still owes actual Design/registry currentness, source truth and approval.

Given those explicit records, a declaration looks like this (ids below are authored fixture identities):

```rust
use sc_measure::{MeasurementBinding, SizeBreak, SizeBreakDefinition};
// `context` contains the actual current records; `waist` is its named measurement.
let definition = SizeBreakDefinition {
    id: break_id, reference: context.membership().reference(),
    axis: house_size_axis_id, from: size_s_id, to: size_m_id,
    dimension_table: design_measurements_id,
    dimension: MeasurementBinding::from(waist), delta: waist_break_declaration_id,
};
let step = SizeBreak::new(definition, &context).expect("explicit current adjacent break");
let source = step.delta(&context).expect("original canonical delta record");
// State/source/provenance stay on `source`; Unknown/Derived need no numeric fallback.
let result = step.authored_value(&context); // may require observation or formula evaluation
```

The same axis/member pair can carry a separate hip binding and delta. For example, waist+4cm and
hip+5cm are independent records. The next adjacent pair can author waist+5cm; no uniform range is
assumed. An explicit zero means zero, and a negative Length remains signed. These are authored
inputs, not proof that the chart differences or generated geometry agree with them.

`delta` returns the original canonical `LengthDeclaration`; `dimension` borrows the actual current
measurement. Known, Assumed, Unknown, Preference and Derived retain their original state/source and
provenance. `authored_value` preserves the original observation/formula refusal as an error source.
There is no independently cached numeric result. A general scalar record cannot silently replace
the dedicated Length declaration role even when its kind is Length.

Every break query rechecks its explicit key against the supplied immutable context. The context must
be reconstructed for replacement records. Pin, axis, table and dimension changes refuse; both member
identities must exist and form an immediate ordered pair in `members`. Reversing or skipping members
refuses, independently of their labels or axis-value order. A same-id canonical delta replacement is
borrowed with its actual current source/state; the earlier immutable snapshot remains unchanged.

Individual declarations retain their focused proof in the task records. Required dimensions, adjacent
coverage and singleton refusals are implemented below, followed by current composite/path inputs.

### Explicit break requirements and coverage — G1-SLICE.4c.3d.2

`SizeBreaks` now keeps two independent authored vectors: required `SizeBreakDimension` entries and
available `SizeBreak` declarations. Each requirement explicitly names an axis, dimension table and
exact measurement binding. Required dimensions never come from whichever break records happen to
be present. Both vectors preserve their original order. A partial or empty draft is representable;
construction validates every supplied reference and declaration without claiming complete coverage.

For a three-member range S/M/L with waist and hip required on its House size axis, four cells are
required: waist S→M, waist M→L, hip S→M and hip M→L. If only waist S→M exists, `missing` returns the
other three exact axis/measurement/from/to identities. Gaps follow requirement order, then authored
member-pair order. An absent declaration stays absent; no numeric zero or chart difference is stored.
Only explicitly required axis/dimension pairs participate, not a Cartesian product of axis catalogs.

```rust
use sc_measure::{SizeBreakDimension, SizeBreaks, SizeBreaksDefinition};
// `context`, axis/table ids, `waist`/`hip` and `waist_step` are actual authored records.
let required = |measurement| SizeBreakDimension {
    axis: house_size_axis_id, dimension_table: design_measurements_id,
    dimension: MeasurementBinding::from(measurement),
};
let breaks = SizeBreaks::new(SizeBreaksDefinition {
    reference: context.membership().reference(),
    dimensions: vec![required(waist), required(hip)], breaks: vec![waist_step],
}, &context).expect("structurally valid partial draft");
let gaps = breaks.missing(&context).expect("actual current gaps");
let coverage = breaks.require_grade_coverage(&context); // names the first missing cell
```

`require_grade_coverage` rechecks the whole supplied inventory before requiring a multi-member range,
nonempty explicit requirements and every required adjacent cell. With no authored dimensions it
returns `MissingDimensions`; it never chooses a POM from available records. A singleton returns
`Singleton { set, member }`, even when the gap list is empty. That preserves MTM's path-2 refusal.
Missing break coverage does not itself forbid path 1; its chart/input contract is separate.

Coverage is structural: Unknown/Derived deltas still name present authored declarations. Requiring
coverage does not evaluate them, fabricate values, certify actual grading or apply an artifact policy.
When a value is needed, the original `SizeBreak` read continues to require observation/evaluation.
A same-id delta replacement contributes its actual current source/state, while a reordered membership
or changed saved binding refuses before gaps are reported; no old completeness result is cached.

Duplicate required axis/dimension pairs, repeated declaration ids, duplicate cells and undeclared
dimensions refuse. Every requirement is checked even without a corresponding break. Original
context/declaration errors remain standard error sources and preserve the failing declaration id.
`declaration(cell, context)` borrows the actual authored declaration after whole-inventory validation;
it refuses an undeclared dimension, invalid member pair or exact missing cell without a fallback.

Current requirements/coverage and singleton refusal have focused contract evidence. Composite inputs
are implemented below; numeric instantiation and geometric equivalence remain G3 work.

### Composite garment SizeSet inputs — G1-SLICE.4c.3d.3.1

`GarmentSizeSetInputs` now composes the actual membership, complete axes, garment chart and required/
available break inventories as borrowed inputs. Its identity/revision comes from that canonical
membership; labels, order, base, coordinates, observations and delta state/source are not copied.
This is a current input view, not a persisted container or an executed instance.

Construct it from the actual `SizeChart`, `SizeChartCollectionContext`, `SizeBreaks` and
`SizeBreakContext`. The contexts must reuse the same membership object, measurement-record context
and logical Design measurement table. Equal ids on independently assembled wrappers do not establish
one shared snapshot. `SizeSetInputError::SharedSnapshot` names the component that must be shared:
Membership, Measurements or DesignTable. Reconstruct a view when supplying replacement snapshots;
matching supplied records remains separate from global Design/registry currentness or approval.

The composite also rejects cross-view identity aliases. A chart or observation cannot reuse an axis,
coordinate, canonical-source or break identity, and other supplied chart tables cannot alias the
break inventory's objects. Each layer's ordinary current-reference checks still apply first; nested
chart/break errors retain the original source evidence. None of these checks infers a missing value.

```rust
use sc_measure::{GarmentSizeSetInputs, SizeInstantiationPath};
// All four arguments are actual current records; both contexts share their canonical inputs.
let inputs = GarmentSizeSetInputs::new(&chart, &chart_context, &breaks, &break_context)
    .expect("one consistent current snapshot");
let membership = inputs.membership(); // original authored members/order/base
let axes = inputs.axes();             // original authored axes/member coordinates
let reference = inputs.reference();   // exact canonical set id/revision
let regeneration = inputs.validate_inputs_for(SizeInstantiationPath::Regeneration);
let grading = inputs.validate_inputs_for(SizeInstantiationPath::GradeRules);
```

| Typed path | Required structural coverage |
| --- | --- |
| `Regeneration` | Complete current garment chart for the actual Design POMs and members |
| `GradeRules` | The same complete chart plus explicit complete break requirements/adjacent cells |

The constructor permits structurally valid draft charts or break inventories. Path checks first
revalidate the complete composite, then require the relevant coverage. A missing break refuses
GradeRules with the original exact cell while leaving Regeneration structurally available. A missing
chart cell refuses both paths. Singletons retain the break layer's explicit grading refusal.

Structural input coverage is not numeric availability. Unknown chart or delta declarations remain
Unknown; Derived values still require formula evaluation. Original records can be read through the
borrowed chart/break APIs and their supplied contexts, with their original observation/formula errors.
The view does not manufacture a value, base instance, grade rules, geometry, equivalence result,
profile decision or release approval. MTM body/Ease composition is implemented below; the
unified entry point is implemented below. Actual instantiation and persisted storage remain separate.

Six composite garment tests plus the five break collection tests pass; selected strict lint,
formatting and book build pass. Tests include actual record borrows, both structural paths, draft gaps,
three mismatched snapshot components, cross-inventory aliases and preserved unavailable numeric input.

Readonly chart_context()/break_context() expose the original contexts for explicit quantity queries;
[owned garment sources](design-inputs.md#owned-garment-sizeset-sources) derive those views temporarily.

### Composite MTM body/Ease inputs — G1-SLICE.4c.3d.3.2

`MtmSizeSetInputs` now composes the actual custom singleton membership, authored axis/member point,
MTM body/Ease chart and break inventory by borrow. The singleton's axis and coordinate must be
explicitly authored before a valid `SizeBreakContext` exists; the MTM view creates neither. Its set
identity/revision, label/base, chart and source records remain the original canonical inputs.

Construct it from an actual `MtmChart`, `MtmChartContext`, `SizeBreaks` and `SizeBreakContext`.
As with garment inputs, both contexts must share the actual membership object, measurement-record
context and logical garment Design table. The body table is separately explicit. The same shared
snapshot/identity validator serves both families: chart, Ease-set, Ease and table identities cannot
alias axes, coordinates, break records or other canonical inputs. Matching ids on separate wrappers
remain insufficient; original chart/break source errors survive in `SizeSetInputError`.

```rust
use sc_measure::{MtmSizeSetInputs, SizeInstantiationPath};
// The contexts contain the same actual singleton and canonical measurement records.
let inputs = MtmSizeSetInputs::new(&chart, &mtm_context, &breaks, &break_context)
    .expect("consistent current body/Ease inputs");
let regeneration = inputs.validate_inputs_for(SizeInstantiationPath::Regeneration);
let grading = inputs.validate_inputs_for(SizeInstantiationPath::GradeRules);
// `grading` retains MtmChartError::GradeRulesUnsupported with the original chart/member ids.
```

Regeneration input validation requires complete current body/Ease mappings for every actual Design
POM, using the MTM chart's existing coverage contract. Partial mappings remain valid drafts and
return their original completeness error when regeneration inputs are requested. GradeRules always
retains the MTM chart's explicit `GradeRulesUnsupported { chart, member }` refusal after current
metadata/snapshot validation, including a draft or a singleton with an empty break/gap list.

No body-plus-Ease arithmetic happens here and no garment numeric result is manufactured. Unknown
body inputs or Derived Ease remain on the original records; their explicit numeric reads still
require observation/evaluation. Structural input coverage does not certify numeric execution, fit,
profile resolution, an evaluated instance or release approval. Reconstruct the view for replacement
snapshots; global currentness is still a separate Design/registry obligation.

Six MTM composite and six garment composite contracts now pass with the five break collection tests.
The fifteen original MTM chart contracts passed after fixture sharing in0307. Selected strict lint/
formatting and book build pass; the unified entry point below retains these input-view boundaries.

### Unified current input family — G1-SLICE.4c.3d.3.3

`SizeSetInputs` is the common typed entry point over validated garment or MTM views. Its explicit
`Garment` and `Mtm` variants retain the original chart family. `From` conversions accept actual
`GarmentSizeSetInputs`/`MtmSizeSetInputs` records; no family is inferred from a label, `custom` system
or singleton count. A custom one-member garment chart remains a garment chart. MTM means its authored
body/Ease correspondence, with the original MTM grading refusal.

```rust
use sc_measure::{SizeSetInputs, SizeInstantiationPath};
// `garment_inputs` was built from the actual shared chart/break contexts above.
let inputs = SizeSetInputs::from(garment_inputs);
let reference = inputs.reference();
let membership = inputs.membership(); // same actual borrowed record
let axes = inputs.axes();
let breaks = inputs.breaks();
let result = inputs.validate_inputs_for(SizeInstantiationPath::Regeneration);
// Match the explicit variant when using a family-specific chart/body/Ease API.
```

The common getters borrow the original membership/axes/breaks. `validate_current` and
`validate_inputs_for` delegate to the selected family's original checks, preserving exact missing
cells, mapping errors, snapshot conflicts and MTM refusal. The enum introduces no wire format,
persisted container, numeric cache or alternative validation path.

The bounded structural review keeps these requirements distinct:

| Requirement | Implemented boundary |
| --- | --- |
| D70 authored axes/member points | Complete current axes, including at least one explicit singleton axis |
| D228 signed breaks | Explicit axis/POM/adjacent pair and canonical state/source |
| Required dimensions and gaps | Independent authored inventory; absence never becomes zero |
| Garment path inputs | Complete current chart; grading also requires explicit break coverage |
| MTM path inputs | Complete body/Ease mappings for regeneration; original grading refusal |
| Current snapshots/identity | Shared actual inputs; cross-view aliases and replaced bindings refuse |
| Family selection | Explicit validated view, never inferred from labels/system/member count |
| Identity/order/base/system | Exact SizeSet pin and authored membership; quantities remain order data |
| Profile intent | Exact selected profile, four domains/five states, immutable source/target origin |

Chart/body/break structural input review is complete. Numeric execution, geometry/equivalence,
profile precedence, storage and release remain separate. Current transformation intent is documented
in [SizeSet profile transformations](size-set-transformations.md). The closed review receipt is
retained whole in `docs/history/stitchcad-devnotes-part456.md` (public history reader).

## 7. Multi-dimensional systems

Director ruling2026-10-26 resolves D70: every complete SizeSet requires at least one authored axis,
including one-dimensional and singleton sets. Axis names and ordered values are explicit, never
inferred from labels or silently defaulted. Membership alone remains a foundation; G1-SLICE.4c.2
implements the axis contract before a complete set can be accepted.

Some systems designate a size by two or more body dimensions rather than one label. The model expresses
that without inventing a second mechanism:

- **`axes`** are named dimensions with their own ordered values — for example height and chest;
- **a member is a point on the axes**, and its label is the designation that point carries;
- **not every grid point need exist.** A set declares the members it has; an absent combination is absent,
  never interpolated silently;
- **breaks name an axis and measurement dimension**, so a range can step in height without stepping in chest;
- a one-dimensional system is the same object with a single axis, which is why the model does not carry two
  representations.

### Executable individual axes — G1-SLICE.4c.2.1

The complete current [individual-axis contract and example](size-axes.md#executable-individual-axes--g1-slice4c21) live in the axis chapter.

### Complete axis/member points — G1-SLICE.4c.2.2

The complete [member-coordinate contract and example](size-axes.md#complete-axismember-points--g1-slice4c22) live alongside it.

## 8. Size systems and what the model must express

| `size_system` | What the model must express | Verification status |
| --- | --- | --- |
| `en13402` | a designation derived from body dimensions, and the mapping from that designation to the chart's POMs | *cited from the roadmap*; the standard's text has not been read here — `G0-CONTRACT.7` owns it |
| `astm_d5585` | a missy-figure size designation and its mapping to the chart | *cited from the roadmap*; same owner |
| `alphanumeric` | labels such as "S", "M", "L", "2XL", with no arithmetic implied by the label | project decision — a label is a name (§3) |
| `numeric` | labels such as "38", "40", "42", still with no arithmetic implied | project decision |
| `custom` | a house or client chart: arbitrary labels, arbitrary order, arbitrary breaks | project decision |

The table is deliberately about what the **model must express**, not about what a standard says. Asserting
the content of EN 13402 or ASTM D5585 without reading them would put an unverified claim into a normative
chapter; `G0-CONTRACT.7` (measurement and POM standards) owns that verification and this chapter cites it.
What is normative here is that a designation system never supplies a measurement by itself: the chart does.

## 9. Ownership, reference and override

Decided in `docs/decisions/decision_size-set-ownership.md`; the operative rules:

1. **A `Design` references a size set** by identity and revision. It does not contain one, so a design is
   sized and evaluable with no factory in the loop — which the headless CLI, the WASM viewer and the
   reference fixture all require.
2. **A Factory Profile may override it**, and an override is a typed transformation with a domain (label
   mapping, break scaling, base substitution, chart replacement), a state and evidence like any other
   profile parameter (roadmap §8).
3. **An override produces a *resolved* size set** — a new object naming the design's reference, the
   profile's revision, the transformation and its evidence. It never mutates the design's reference and
   never re-labels a size in place, so a release package can always say whose sizes it cut.
4. **Precedence follows the profile composition order:** hard restriction > factory override > preference >
   default. A size transformation that a hard restriction forbids is refused with a diagnostic, and a
   conflict is visible in the composition report rather than merged silently.
5. **Artifacts record the resolved set.** A `.rul` table is meaningless without the base size and breaks it
   was built against, so the manifest names the resolved size set's identity and revision.

### Executable Design reference boundary

DesignSizeSetReference is the core id/revision carrier. new(id, revision) retains exact authored
values; id() and revision() expose them without lookup or member storage. Explicit From conversions
with sc-measure SizeSetReference preserve both fields, including revision zero and terminal Count.
Construction/conversion supplies no registry, evidence or path-readiness certificate.

SizeSetInputs::validate_design_reference(expected) requires the actual supplied garment/MTM view's
exact set identity and revision, then reuses its current validator. No label, singleton count, latest
revision or profile selection substitutes for the authored pin. The view is not modified.

| API or variant | Implemented boundary |
| --- | --- |
| `DesignSizeSetReference` | Core external reference; no embedded chart/member list |
| `DesignReferenceMismatch` | SizeSetInputError retains exact expected and actual references |

```rust
use sc_core::{design::DesignSizeSetReference, ontology::EntityId};
use sc_units::Count;
let reference = DesignSizeSetReference::new(EntityId::from_bits(17), Count::new(0));
assert_eq!(reference.revision(), Count::new(0));
```

Admission permits partial chart drafts; validate_inputs_for remains the separate structural coverage
check, and MTM still refuses grade rules. Numeric execution and full Design composition remain later.

## 10. What each instantiation path consumes

| Path | From the size set | Consequence of a profile override |
| --- | --- | --- |
| regeneration ([§2](instantiation-paths.md)) | the target member's measurements — from the chart, or from a body for MTM | a replaced chart changes the geometry directly |
| grade rules ([§3](instantiation-paths.md)) | the base size and the breaks | a rescaled break or a substituted base changes every graded size, and the equivalence report with it |

Both paths report against the **resolved** set, which is why the report names it.

## 11. Made-to-measure is a size set of one

[MTM input charts](mtm-input-charts.md) implement current singleton body/Ease correspondence and
Design-POM coverage. Composite inputs and [profile intent](size-set-transformations.md) are
implemented; persisted ownership, actual profile resolution and regeneration remain later work.

An MTM instance is a `custom` size set with a single member, whose base is that member, whose chart comes
from body measurements taken with landmarks and procedures (ontology §2.1), and whose breaks are empty — so
path 2 is refused and path 1 is the only route. Nothing about this is a special case in the model, which is
the point: a bespoke body and a house chart are the same object with different provenance.

## 12. Verification status of the claims in this chapter

- **The object's fields, the label/order separation, the base-size rules, breaks, axes and the ownership
  resolution** — *project decisions*, made here and recorded in the decision record. They are normative
  because this chapter says so, not because an external document does.
- **EN 13402 and ASTM D5585 exist and are size-designation systems** — *cited from the roadmap* (§3.4,
  §11 G0). Their content is not asserted here; `G0-CONTRACT.7` owns reading them and stating what is
  adopted, and until then a mapping to either system is `unknown` rather than guessed.
- **"Size-run quantities belong to an Order object"** — *cited from the roadmap* (§7.5), and the reason
  this chapter carries no quantities.
- **The precedence order and the override's evidence requirement** — *cited from the roadmap* (§8).

## 13. What must be true in tests

- **Order is authored, not sorted:** a set whose members are "2XL, S, M" keeps that order through save,
  grade and export, and no code path re-sorts it.
- **The base is a member:** constructing a set whose base is not in `members` fails with a diagnostic
  naming the invariant.
- **A label carries no measurement:** two sets with the same labels and different charts produce different
  geometry, and the artifacts say which set they used.
- **Breaks drive path 2 and block nothing else:** a set with no breaks serves regeneration and refuses
  grading with a diagnostic naming the missing breaks.
- **An override never mutates the design:** resolving a profile override produces a new object; the design's
  reference and revision are unchanged, and the resolved set names both.
- **A hard restriction outranks an override:** a transformation a restriction forbids is refused, and the
  conflict appears in the composition report.
- **An MTM set of one refuses grading** and regenerates, with its chart's provenance recorded as body
  measurements rather than a standard.
- **The manifest names the resolved set:** exporting twice with different overrides produces two packages
  whose manifests differ in the size-set identity and revision, not only in geometry.
