# Executable garment constructions at G1

The construction companion to [ontology §4.3/§4.7](ontology.md), alongside the running
[piece, sewing and mark APIs](ontology-implementation.md). This chapter describes structural intent
and its remaining physical obligations. Construction operations, conserved intake, bounded offsets
and released receiver outputs require their later gates.

**Dart (`G1-SLICE.3c.4a.1`).** `Dart::new` accepts an editable `DartDefinition`, the source
Piece and identity ledger. The immutable object holds its id, intake origin, apex, two directed leg
ranges, an explicit directed reference carrying closing/folding direction and a closing-operation id.
There is no independent copy of a formula/profile value or uncertainty state. An explicit intake
holds its authored parameter id plus a nonnegative Length, including a deliberately authored zero;
negative explicit intake is a typed `DartError`. Formula/profile intake declarations supply no value
or default. Recipe/Design/profile registries must validate their existence, kinds and resolved values.

An apex is an EdgeAnchor on owned geometry: the fixture's interior apex can be the endpoint of an
internal dart leg, without pretending it lies on the waist cut edge. Every leg/direction range needs
complete Piece-owned positive intervals and unique endpoints. Two identical held leg intervals are
refused even when traversal directions oppose. Unknown/foreign geometry names the failed role;
owned current endpoints cannot conceal a foreign middle after a merge/reversal. A valid reference
alone does not prove leg straightness, apex coincidence or a physical folding direction.

For example, the skirt's 40 000 µm declared dart intake and two internal directed legs can be carried
alongside an apex on one leg and a closing operation's stable id. G1 preserves those declarations;
it does not claim the operation removed 40 000 µm from the waist. G2/G3 must execute the closure and
compare its result within the numerical tolerance class. Changing the intake or operation builds a
replacement object rather than mutating validated content. An authored zero retains its identity;
it is never substituted for an unread profile declaration.

`apex_resolution()` exposes raw point evidence. Splitting at the apex produces a visible split choice;
no side is selected by convention. `reference_resolutions()` returns directed whole-range evidence in
first-leg/second-leg/direction order, including composed reversal, ordered fragments and lost interiors.
Deleting a leg's middle segment leaves a repair even if both held endpoints still resolve. Neither
query rewrites the stored dart. The closing-operation identity is required content; G1-SLICE.5/.6 must
prove existence, kind and dependencies before design/recipe validation can certify its use.

Every dart reports `GeometricValidation::DeferredToG2` and `IntakeValidation::DeferredToG2AndG3`.
A profile intake binding reports `ProfileBindingValidation::DeferredToG4`; absence of that field means
only that the intake source is not a profile binding. Structural birth is not executed closure,
conserved intake, export readiness or a production approval. Gather and other construction objects are later owned child slices.

| API token | Meaning in the dart implementation |
| --- | --- |
| `DartDefinition` | Editable semantic dart input |
| `DartError` | Typed intake, duplicate-leg, apex or directed-reference refusal |
| `DartReferenceRole` | First leg, second leg or authored direction field |
| `IntakeAmount` | Explicit parameter/value, formula declaration or profile declaration |
| `IntakeValidation` | Physical conservation obligation separate from a declared amount |

**Tuck and Pleat (`G1-SLICE.3c.4a.2a`).** These are distinct immutable semantic types even when
their editable `FoldDefinition` inputs happen to match. Each carries its id, intake source, a
nonempty ordered list of directed fold ranges, explicit directed folding reference and required
closing-operation identity. Their shared structural validator names the failed line/direction field;
empty lists, duplicate held intervals (including opposite traversal) and negative explicit intake
are refused. Every positive interval must be owned and complete with unique endpoints. Distinct
partial ranges on one owned edge are valid authored intent; the list alone proves neither a physical
pleat-count rule nor coincidence/fold shape.

For example, a tuck may name one internal fold range and a direction along an owned construction
line, while a pleat can retain several authored fold ranges in order. Both retain intake parameter
origin and operation identity. An authored zero remains explicit; a formula/profile intake produces
no value or fallback. The operation registry must validate the typed closing operation before use.
G2/G3 executes it and compares removed boundary length with the resolved declared intake.

`references()` lists fold lines in authored order, then the direction. `reference_resolutions()`
keeps directed traversal and raw endpoint/range evidence. A reversed split line visits its fragments
in the opposite order; deleting its middle preserves a repair despite live held endpoints. An
owned-endpoints/foreign-middle merge is refused before and after reversal. Editing a cloned input
cannot mutate either validated semantic object.

Both types report `GeometricValidation::DeferredToG2` and `IntakeValidation::DeferredToG2AndG3`.
Present profile intake reports `ProfileBindingValidation::DeferredToG4`; absence means only no such
field. These are semantic descriptors, not executed folds or production-ready physical constructions.
The physical intake-conservation proof and fold-count/shape checks remain G2/G3 obligations.

| API token | Meaning in the tuck/pleat implementation |
| --- | --- |
| `FoldDefinition` | Editable intake, fold ranges, direction and operation input |
| `FoldError` | Typed intake, empty/duplicate-line or unresolved/foreign-range refusal |
| `FoldReferenceRole` | Semantic fold-list position or authored direction field |
