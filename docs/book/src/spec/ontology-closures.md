# Executable closure intent

This companion records G1's structural closure APIs under ontology §4.7. Physical hardware geometry,
placement, fold/seam effects and derived buttonhole lengths remain G2/G3 obligations. Logical
parameter/operation/material registries and target profiles must validate their own declarations.
Centred zipper and hook/bar descriptors now have structural APIs; button/buttonhole derivation
follows in G1-SLICE.3c.4c.2. Structural support alone is not realized hardware geometry.

## Physical notion placements

`NotionPlacement` gives each notion placement a stable semantic identity and names the **physical
cut copy** it attaches to. `NotionPlacementDefinition` carries id, copy id, an `EdgeAnchor` position
and a `DirectedRange` orientation reference in the pattern Piece's source frame. The immutable
placement retains its original source Piece identity to detect copy reassignment. It duplicates no
pattern geometry, reflected coordinates, material state or component size.

For example, two copies of one pattern Piece can carry different hook/bar or zipper-side placement
positions. Each placement keeps its own identity, anchor and orientation while naming its intended
copy. Reordering the CutPlan preserves bindings. Removing that copy refuses `MissingCopy`, even if
another copy of the same Piece remains. Reassigning the original copy to another source Piece is
also a typed refusal; neither operation transfers the notion to an arbitrary peer.

The constructor requires the original copy in the supplied plan, matching source Piece, a live
uniquely owned anchor, and complete owned orientation intervals with unique endpoints. Missing edges,
foreign material and split endpoint choices are refused. Current `validate_current()` checks the
copy/owner again and follows the identity journal for the anchor and directed range. A historical
anchor need not retain its original live edge: splitting and reversing it is valid when its current
resolved point remains uniquely owned. A split-point choice or deletion is `CurrentUnresolved` with
raw point evidence. Birth validation retains the existing live-edge requirement and its original
`BornUnresolved` errors; Notch and TurnPoint behavior remains unchanged.

`anchor_resolution()` and `direction_resolution()` expose independent raw point/range evidence
without choosing or rewriting input. Lost orientation interiors remain visible even when both
original endpoints survive. Complete live endpoints around a foreign merged middle do not establish
ownership. Directed fragment order composes authored traversal with journal reversal; reflected
CutCopy orientation is a separate G2/V1 transform, never an inferred reversal of the source frame.

`validate_current()` checks one supplied current plan/Piece context. Design validation must check all
current registries and every placement before closure execution; raw queries grant no release
readiness. Actual hardware shape, attachment coordinates and reflected geometry are unproved:
every placement reports `GeometricValidation::DeferredToG2`. Counts and sizes belong to the closure
kinds, not an implicit expansion or a default supplied by a placement.

| API token | Meaning in the physical-placement implementation |
| --- | --- |
| `NotionPlacement` | Immutable placement on one stable physical cut copy |
| `NotionPlacementDefinition` | Authored placement identity, copy, anchor and orientation range |
| `NotionPlacementError` | Typed copy/owner/anchor/direction refusal |
| `CurrentUnresolved` | Previously valid held anchor now needs repair or an explicit choice |
| `UnresolvedDirection` | Positive orientation content or an endpoint needs repair/choice |
| `DirectionOutsidePiece` | Current orientation material belongs outside the original owner |
| `EdgeAnchor` | Held source-edge identity and exact parameter for a notion attachment |
| `BornUnresolved` | Birth point requires unique resolution; its existing error contract is retained |
| `MissingCopy` | Original physical-copy target is absent; a peer is never substituted |

## Centred zipper, hook and bar: stable instances and canonical targets

`ClosureDefinition` retains a stable id, explicit `ClosureKind` and a nonempty list of
`ClosureInstance`s. Each instance has a stable identity and two distinct notion-placement ids.
For a centred zipper they are its first/second sides; for hook and bar they are respectively the
hook and the bar. Reordering instances preserves identity and target queries. A `Count` is derived
from the instance list, never independently entered: two instances mean two zippers or two hook/bar
pairs, not a cached number that can disagree with their positions. Duplicate instance ids and reuse
of any component placement within one Closure are typed refusals; Count-domain overflow is explicit.
Geometric coincidence of separately identified placements is still a G2 check, not a G1 assertion.

A centred zipper carries `ClosureLength`: explicit positive Length with its authored parameter id,
or a formula/profile declaration. Zero/negative explicit length is refused; symbolic length has no
fallback and its positive domain is checked by the declaration owner later. For example, the fixture
can retain its authored 18.0 cm zip length and two CB-side physical placements without constructing
zipper geometry. Hook/bar sizes each carry a `NotionSize` logical recipe declaration or profile
binding: no vendor designation, dimensions or uncertainty state is guessed or copied into a default.
The fixture's waistband placement references its extension; this API does not prove that attachment.

`Closure::new` validates all current placement targets against the supplied plan, Piece collection
and ledger. Duplicate registry identities refuse ambiguity rather than selecting a first match.
Missing placement/owner or invalid current copy/anchor/direction evidence names the stable instance,
component role and relevant target. A held placement whose direction loses an interior is refused,
even with both endpoints live. Removing a CutCopy is refused even if both placement ids still exist.

`validate_current()` repeats checks for all instances. `placement()` borrows one canonical current
target by stable instance id and role; its successful result validates only that target, not all
other attachments or release readiness. No placement/material/source definition is duplicated in
the Closure. Definition edits require a replacement object; existing objects remain unchanged.
Profile size fields report `DeferredToG4`; absence only means none was authored. Registry/domain
validation and G2/G3 actual hardware geometry remain explicit obligations; geometry is `DeferredToG2`.

A `ClosureKind::Fly` request refuses with `ClosureEnvelopeError::FlyDeferred` **before geometry or
instance validation**. `ClosureDefinition::require_in_scope()` and construction report env_fly,
requested Closure identity, declared trousers-gap identity and proving gate G7. The editable request
can be inspected; no executable supported Closure is constructed and no zipper is substituted.
G7 owns the limitation, not a promise that physical fly support will appear at that gate.

| API token | Meaning in the zipper/hook-bar implementation |
| --- | --- |
| `ClosureDefinition` | Authored closure kind and stable physical instances |
| `ClosureKind` | Centred zipper, hook/bar, or explicitly refused fly request |
| `ClosureInstance` | Stable physical instance with two canonical placement targets |
| `ClosurePlacementRole` | Deterministic first/second component identity |
| `ClosureLength` | Positive authored zipper length or formula/profile declaration |
| `NotionSize` | Logical recipe or target-profile hardware-size declaration |
| `ClosureError` | Typed scope, domain, count, identity or current target refusal |
| `ClosureEnvelopeError` | Named execution-envelope refusal with request context |
| `FlyDeferred` | env_fly with requested Closure, declared trousers gap and G7 |
| `NoInstances` | Closure requires a nonempty physical-instance list |
| `CountOverflow` | Derived instance cardinality is outside Count's u32 domain |
| `NonPositiveLength` | Explicit zipper length must exceed zero |
| `DuplicateInstance` | One physical instance identity is repeated |
| `ReusedPlacement` | One component placement is used more than once inside a Closure |
| `DuplicatePlacement` | Current notion-placement context has ambiguous identities |
| `MissingInstance` | Stable queried physical instance is absent |
| `MissingPlacement` | Required component-placement target is absent |
| `InvalidPlacement` | Current placement needs repair or fails copy/owner/reference validation |
| `DeferredToG2` | Structural closure carries no proof of actual hardware or attachment geometry |
