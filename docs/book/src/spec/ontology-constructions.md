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
conserved intake, export readiness or a production approval. Other construction objects are later owned child slices.

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

**Gather (`G1-SLICE.3c.4a.2b`).** A Gather binds a sewing graph, span, selected side and held
physical-copy identity, plus an explicit directed reference and closing-operation id. There is no
independently authored gather intake or second hidden allocation: `intake_source()` borrows the
span's existing DeclaredEase, retaining the raw signed A-minus-B source and selected gathered side.
Uniform, weighted and between-notch allocations therefore stay the canonical graph declaration.

For example, a span declaring A-minus-B = +5 000 µm permits gathering A; −5 000 µm permits gathering
B. Explicit wrong-sign selection is refused, and an authored zero is legal no-op intent. Formula or
profile sources provide no value/default; their resolved sign and actual walking/shortening must be
verified later. Borrowing a declaration is not evidence that material has physically shortened.

`Gather::new` requires the named graph/span/side-copy binding, a current CutPlan copy belonging to
the source Piece, and complete owned attachment/direction intervals with unique endpoints. A copy
change on the span returns `CopyBindingChanged`, even when the new copy shares the same Piece/source
range. A missing graph/span/copy or reassigned Piece is a typed target refusal. A replacement copy
cannot inherit an old gather. The held graph/span/copy ids remain visible in the immutable definition.

`copy()` checks the current plan; `intake_source()` checks the current graph/span/copy binding and
explicit ease sign. Those are independent target queries: the graph query alone does not assert a
copy is still present in a separately changed plan. `attachment_resolution()` and
`direction_resolution()` expose current raw interval/endpoint and directed evidence, including
interior deletions and split choices. A removed middle stays visible despite live endpoints; no
repair selects another span/copy by convention. Global Design validation must check all current
registries and the source graph's landmarks/bindings too; these queries do not grant release readiness.

Every Gather reports `GeometricValidation::DeferredToG2` and `IntakeValidation::DeferredToG2AndG3`.
A present profile ease source reports `ProfileBindingValidation::DeferredToG4`; the descriptor caches
no profile values/states. G2/G3 executes the required typed closing operation, walks actual intervals
and proves realized intake/allocation. Operation/parameter existence/kinds remain `.5`/`.6` and G4
obligations. All four intake kinds now have structural APIs; none certifies physical closing behavior.

| API token | Meaning in the gather implementation |
| --- | --- |
| `GatherDefinition` | Editable graph/span/side/copy binding, direction and operation intent |
| `GatherError` | Typed changed/missing target, wrong sign or invalid reference refusal |
| `GatherIntakeSource` | Borrowed signed canonical ease declaration with selected gathered side |
| `GatherReferenceRole` | Attachment or authored direction field |
| `CopyBindingChanged` | Selected span side no longer names the held physical copy |

**Facing, Lining and Interfacing (`G1-SLICE.3c.4b.1`).** Distinct immutable layer types share
editable `LayerDefinition` input and structural validation. A definition names its served Piece,
explicit `LayerOffsetRelationship` and material assignment. The relationship holds a required recipe
offset-operation id and nonempty directed source ranges in that Piece's frame. The recipe owns offset
dimensions/input states: descriptors do not cache generated contours, defaults or a second output-Piece
material state. Recipe/Design must validate the named operation's existence, kind and dependencies.

For example, an interfacing descriptor can serve the waistband and name an offset operation using
owned construction ranges for its inner half. The fixture's recipe still supplies the 4.0 × 77.0 cm
finished rectangle and non-sewn fused attachment; this descriptor does not generate or prove it.
A facing similarly retains its served Piece and recipe relationship, rather than recovering a facing
from line art. Assigned material holds an entity id whose registry remains a Design check; unresolved
material carries a nonblank reason without a substitute assignment. The shared local reason check
preserves the original Piece invariant too.

Constructors require the supplied Piece to match the held served id. Unknown/foreign source portions,
interval repairs, endpoint choices, empty source lists and exact duplicate held intervals are typed
`LayerError`s. `source_resolutions()` keeps authored input-list order and directed raw range/endpoint
evidence. Merging owned ends around a foreign middle does not make that source owned, even after
reversal. An interior deletion stays visible; editing cloned material/offset input cannot mutate a
validated layer. Actual offset shape, bounded error and physical relationships remain G2 obligations.

**Modelled content and execution scope are separate.** The feature matrix's rule 3 permits a lining
object to be modelled while deferring supported execution. `Lining::new` validates structural content;
`require_in_scope()` refuses requested v1 execution with `LayerEnvelopeError::LiningDeferred`.
Its `diagnostic()` is env_lining, the payload names the served Piece, and `proving_gate()` is G7.
No facing or interfacing is silently substituted. G1-SLICE.6 must apply that refusal before requested
construction execution while allowing inspection to retain modelled content. Facing and Interfacing
pass only this envelope check; every layer still reports `GeometricValidation::DeferredToG2`.
Neither successful structural birth nor an envelope check proves geometry or grants release approval.

| API token | Meaning in the served-layer implementation |
| --- | --- |
| `LayerDefinition` | Editable served Piece, offset relationship and material input |
| `LayerOffsetRelationship` | Recipe offset-operation identity and owned directed source ranges |
| `LayerKind` | Distinct facing, lining or interfacing semantic kind |
| `LayerError` | Typed target, source-range or material-reason refusal |
| `LayerEnvelopeError` | Typed execution-scope refusal independent of structural birth |
| `LiningDeferred` | Modelled lining request refuses with env_lining, served Piece and G7 |
