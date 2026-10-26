# Size sets

> **Status:** normative specification, gate **G0** (roadmap §3.4, §7.5, §8). Implemented by `sc-measure` at
> gate **G1** (leaf `G1-SLICE.4`) and consumed by both instantiation paths at **G3**. The
> [membership foundation](size-membership.md) is executable; full SizeSet implementation remains in progress. The ownership
> question roadmap §3.4 leaves open is decided here and recorded as
> `docs/decisions/decision_size-set-ownership.md`. Terms are in the [glossary](glossary.md).

A size set is the answer to "which sizes does this design exist in, what does each one measure, and how do
they step from one to the next?" It is a first-class object with its own identity, because two designs can
share one, because a factory may resolve one differently, and because a release package must be able to say
exactly which one it cut.

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

[Garment chart observations](size-chart-observations.md) now implement individual current member/POM
correspondences; [chart collections](size-chart-collections.md) now check exact current Design/member
coverage. The remaining SizeSet contracts are still in progress.

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

Adjacency follows the authored `members` sequence (§3). Axis association, measurement identity and
delta are explicit: no label parsing, chart subtraction, inferred dimension, implicit zero or choice
of first axis supplies missing data. G1-SLICE.4c.3d.1 implements this ruled key; chart/break coverage,
path readiness and full SizeSet composition follow in separately owned children.

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

Five focused break contracts plus the five axes contracts pass, with selected strict lint/formatting
and book build. This establishes individual declarations. Required axis/dimension inventories,
complete adjacent-pair coverage, singleton grading refusal and composite SizeSet/path readiness
remain G1-SLICE.4c.3d.2/.3; one valid declaration does not certify those later contracts.

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

`sc-measure` now provides immutable `SizeAxis` records. `SizeAxisDefinition` carries the explicit
set/revision pin, axis identity, nonblank name, kind and ordered nonempty value list. Names preserve
Unicode, case and spacing. `SizeAxisValueDefinition` assigns each coordinate a stable identity and an
explicit source: an exact `SizeLabel`, a dedicated canonical length declaration, or a general scalar
declaration. Neither labels nor magnitudes choose an axis name, value order or numeric kind.

```rust
use sc_core::ontology::EntityId;
use sc_measure::{SizeAxis, SizeAxisDefinition, SizeAxisKind, SizeAxisSourceContext,
    SizeAxisValueDefinition, SizeAxisValueSource, SizeLabel, SizeSetReference};
use sc_units::Count;
let id = EntityId::from_bits;
let sources = SizeAxisSourceContext::new(&[], &[]).expect("no numeric sources needed");
let coordinate = |n, text: &str| SizeAxisValueDefinition {
    id: id(n), source: SizeAxisValueSource::Label(
        SizeLabel::new(text.to_owned()).expect("nonblank authored value")),
};
let axis = SizeAxis::new(SizeAxisDefinition {
    reference: SizeSetReference { id: id(1), revision: Count::new(0) },
    id: id(10), name: "House size".to_owned(), kind: SizeAxisKind::Label,
    values: vec![coordinate(11, "2XL"), coordinate(12, "S")],
}, &sources).expect("explicit order");
assert_eq!(axis.definition().values[0].id, id(11));
let value = axis.value(id(12), &sources).expect("actual coordinate identity");
assert_eq!(value.axis(), id(10));
```

For numeric axes, use `SizeAxisKind::Scalar(kind)` and an explicit `Length(declaration_id)` or
`Scalar(declaration_id)` source. The context borrows actual canonical declarations; lookup returns
`SizeAxisValueContent::Length(&LengthDeclaration)` or `Scalar(&ScalarInputDeclaration)`. Source,
state and provenance remain on that original record. Unknown/Derived coordinates can be represented
without evaluating them: their original value-read refusal still applies. Boolean false remains a
present Boolean. General scalar kinds include Length, Angle, derived Area, Ratio, Count and Boolean;
this local layer imposes no numeric monotonicity or physical meaning on an authored dimension.

Construction refuses blank names, empty values, conflicting metadata/source identities, exact duplicate
labels, absent sources, wrong canonical-record roles and mismatched scalar kinds. A dedicated-length
reference cannot silently use a general Length scalar record. `validate_current` and `value` recheck all
axis references against the supplied inventory, so a same-id replacement with a different kind refuses.
The authored order remains unchanged. Shared canonical numeric records are borrowed, never cached or
claimed to be independent measurements.

Five focused native contracts pass, with selected strict lint and formatting. These checks establish
individual local axes; they do not certify global registry currentness, evidence truth, complete member
points or a complete SizeSet. The complete axis/member aggregate is implemented below. Charts, breaks, resolution and path
readiness remain separately owned.

### Complete axis/member points — G1-SLICE.4c.2.2

`SizeAxes` now validates the axis portion of a SizeSet against an actual `SizeMembership` and current
canonical sources. Its definition contains the exact set/revision pin, a nonempty authored vector of
`SizeAxis` records and explicit `SizeMemberPoint` records. Every current member needs exactly one
point; each point needs exactly one `SizeAxisCoordinate { axis, value }` for every axis. A coordinate's
value must belong to that particular axis. A singleton also needs at least one authored axis and an
explicit coordinate; no name, ordinal or member label supplies it.

For example, extend the labelled axis from the preceding example with two explicitly mapped members:

```rust
use sc_measure::{SizeAxes, SizeAxesDefinition, SizeAxisCoordinate, SizeMember,
    SizeMemberPoint, SizeMembership, SizeMembershipDefinition, SizeSystem};
// `axis`, `id` and `sources` are the explicit records from the preceding example.
let reference = axis.definition().reference;
let membership = SizeMembership::new(SizeMembershipDefinition {
    reference, system: SizeSystem::Custom, base: id(2),
    members: vec![
        SizeMember { id: id(3), label: SizeLabel::new("House 12".to_owned()).expect("label") },
        SizeMember { id: id(2), label: SizeLabel::new("House 2XL".to_owned()).expect("label") },
    ],
}).expect("authored member order and base");
let point = |member, value| SizeMemberPoint { member: id(member),
    coordinates: vec![SizeAxisCoordinate { axis: id(10), value: id(value) }] };
let axes = SizeAxes::new(SizeAxesDefinition {
    reference, axes: vec![axis], points: vec![point(2, 11), point(3, 12)],
}, &membership, &sources).expect("explicit complete member points");
let view = axes.point(id(2), &membership, &sources).expect("current member");
assert_eq!(view.member().label.as_str(), "House 2XL");
assert_eq!(axes.coordinate(id(2), id(10), &membership, &sources)
    .expect("explicit coordinate").definition().id, id(11));
```

Axis order, value order, member order, point-record order and each point's coordinate order remain
authored and independent. A two-axis catalog with two values per axis can contain just two diagonal
members: the other two combinations stay absent. Unused catalog values are allowed. No Cartesian
expansion, interpolation, label parsing, sorting or automatic base choice occurs.

The aggregate rejects duplicate metadata identities across set/members/axes/values/current sources,
exact duplicate axis names, missing/foreign members, duplicate point records, missing/foreign axes,
duplicate coordinates and values from another axis. Two members cannot designate the same tuple of
axis/value identities, even with reversed coordinate-vector order. This compares authored identities,
not numeric equality of independent canonical inputs. Names differing in case or spacing remain distinct.

`validate_current`, `point` and `coordinate` revalidate the complete aggregate before lookup. The
membership pin is checked first; each axis must match it. A supplied membership replacement contributes
its actual member labels/order/base, while missing member coverage refuses. Point views borrow the
actual member and original coordinate record. Coordinate views borrow the original canonical numeric
content; no value/state/source is copied. Original nested axis/membership errors remain standard error
sources. Matching supplied pins is local validation, not global registry currentness or approval.

Five aggregate contracts plus the five individual-axis contracts pass; selected strict lint, formatting
and book build pass. This closes the authored-axis/member-point contract. Complete charts/breaks,
resolved profile transformations, regeneration/grading and release remain separate work.

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

## 10. What each instantiation path consumes

| Path | From the size set | Consequence of a profile override |
| --- | --- | --- |
| regeneration ([§2](instantiation-paths.md)) | the target member's measurements — from the chart, or from a body for MTM | a replaced chart changes the geometry directly |
| grade rules ([§3](instantiation-paths.md)) | the base size and the breaks | a rescaled break or a substituted base changes every graded size, and the equivalence report with it |

Both paths report against the **resolved** set, which is why the report names it.

## 11. Made-to-measure is a size set of one

[MTM input charts](mtm-input-charts.md) now implement current single-member body/Ease correspondence
and explicit Design-POM coverage. Composite SizeSet and actual regeneration remain later work.

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
