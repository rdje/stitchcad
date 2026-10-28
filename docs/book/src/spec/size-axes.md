# Size axes and member coordinates

The existing sc-measure axis/member-point contract extends [Size sets §7](size-sets.md#7-multi-dimensional-systems).
It validates supplied authored records; global currentness, numeric instantiation and physical truth retain their separate owners.

## API and example vocabulary

| Token | Meaning here |
| --- | --- |
| `SizeAxis` | Immutable locally validated authored axis |
| `SizeAxisDefinition` | Editable set pin, identity, name, kind and ordered values |
| `SizeAxisValueDefinition` | Coordinate identity and explicit label or canonical numeric source |
| `SizeAxisSourceContext` | Borrowed actual dedicated Length and general scalar inventories |
| `SizeAxes` | Complete axes and explicit member-coordinate points under one membership |
| `SizeMemberPoint` | One member identity and its explicit axis/value correspondences |
| `validate_current` | Recheck axis or aggregate against the supplied current records |
| `value` | Axis lookup of an actual coordinate identity; also the example point's coordinate number |
| `coordinate` | Example coordinate constructor; aggregate API looks up a member/axis coordinate |
| `axis` | Actual axis record in examples; coordinate views report its identity |
| `id` | Example helper converting an explicitly supplied integer to an entity identity |

## Authored axes and member points

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

Individual-axis checks do not certify global registry currentness or evidence truth. Complete
axis/member points follow below; charts/breaks, typed composite inputs and immutable profile intent
are documented in [Size sets](size-sets.md) and its companion. Actual instantiation stays separate.

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

The authored-axis/member-point contract and current chart/break composites are implemented.
Immutable profile intent records proposed overrides; applying them, regeneration/grading and release
retain G3/G4 and integration owners. Original child proof receipts remain available in the task tree.

The chart/body/break structural family and immutable profile transformation intent are implemented.
Current local snapshots/coverage remain distinct from global currentness, numeric availability,
geometry and physical truth. SizeSet review .4c.5 is complete; combined structural acceptance .4d.2 is reviewed. G3/G4 retain actual
profile constraints, precedence and execution. See [transformation intent](size-set-transformations.md).

