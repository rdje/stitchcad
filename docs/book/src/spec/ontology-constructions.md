# Executable garment constructions at G1

The construction companion to [ontology §4.3/§4.7](ontology.md), alongside the running
[piece, sewing and mark APIs](ontology-implementation.md). This chapter describes structural intent
and its remaining physical obligations. Construction operations, conserved intake, bounded offsets
and released receiver outputs require their later gates.

**Dart (`G1-SLICE.3c.4a.1`).** `Dart::new` takes a `DartDefinition`, source Piece and identity
ledger. The immutable result retains id, intake origin, apex, two directed legs, directed closing/
folding reference and closing-operation id. Explicit intake holds its authored parameter id and a
nonnegative Length, including authored zero. Negative intake is a typed `DartError`; formula/profile
origins hold declarations without values, defaults or copied uncertainty. Their registries validate
existence, kinds and resolved values.

An apex is an EdgeAnchor on owned geometry: it may be the endpoint of an internal leg rather than
the cut boundary. Every leg/direction needs complete owned positive intervals and unique endpoints.
Identical held legs refuse even with opposite traversal. Unknown/foreign geometry names its role;
owned endpoints cannot conceal a foreign middle after merge/reversal. Valid references prove no
straightness, apex coincidence or physical folding direction.

For example, a skirt can retain40 000 µm intake, two internal directed legs, an apex on one leg and
a closing-operation id. G2/G3 executes closure and compares removed boundary length with declared
intake within the numerical tolerance class. G1-SLICE.5/.6 validates operation existence, kind and
dependencies. Replacing intake or operation builds a new object; authored zero never substitutes for
an unread declaration.

`apex_resolution()` returns raw point evidence; splitting at the apex leaves a visible split choice.
`reference_resolutions()` returns whole-range evidence in first-leg/second-leg/direction order,
including composed reversal, ordered fragments and lost interiors. Deleting a leg's middle remains
a repair despite live endpoints. Neither query chooses a branch or rewrites held content.

**Current Dart validation (`G1-SLICE.6.4.5.4.a`).** The original Piece identity is required; another
Piece with identical geometry returns WrongPiece with both ids. Birth requires a live, uniquely
resolved apex. Current validation follows an already born-valid apex through historical splits and
reversals, checks current owned scope, then every complete directed source in the original order.
A split-point apex remains CurrentUnresolved; complete leg coverage cannot select it. Apex failures
expose the original AnchorError through `std::error::Error::source()`.

```rust,ignore
// After journal edits, inspect or repair a refusal; held declarations remain unchanged.
dart.validate_current(&piece, &ledger)?;
```

Darts report `GeometricValidation::DeferredToG2` and `IntakeValidation::DeferredToG2AndG3`.
Profile intake reports `ProfileBindingValidation::DeferredToG4`; None means only no such field.
Current source validation grants no executed closure, conservation, export readiness or approval.

| API token | Dart meaning |
| --- | --- |
| `DartDefinition` | Editable semantic dart input |
| `DartError` | Owner, intake, duplicate-leg, apex or directed-reference refusal |
| `DartReferenceRole` | First leg, second leg or authored direction |
| `IntakeAmount` | Explicit parameter/value, formula declaration or profile declaration |
| `IntakeValidation` | Physical conservation obligation separate from declared intake |

**Tuck and Pleat (`G1-SLICE.3c.4a.2a`).** Distinct immutable types share `FoldDefinition` and
structural rules. Each retains id, intake origin, nonempty ordered directed fold ranges, explicit
directed folding reference and closing-operation id. Empty/duplicate held intervals refuse, including
opposite traversal. Every positive interval must be complete and owned with unique endpoints.
Distinct partial ranges on one edge are valid intent; list length proves no physical pleat count,
coincidence or fold shape. Explicit zero retains provenance; negative intake refuses; symbolic intake
supplies no value or fallback.

A tuck may name one internal fold range and an owned direction; a pleat may retain several lines in
order. The operation registry checks its required typed closing operation. G2/G3 executes it and
compares removed boundary length with resolved declared intake.

`references()` lists authored fold lines, then direction. `reference_resolutions()` retains raw
interval/endpoint evidence and composed traversal. Reversed splits visit fragments in opposite order;
a deleted middle remains a repair despite live endpoints. Foreign interiors refuse before/after
reversal. Editing cloned input cannot change either held object.

**Current folds (`G1-SLICE.6.4.5.4.a`).** Both use the original structural rules, first requiring
the held Piece identity. They check all current lines and direction without rewriting references or
choosing endpoints. A later damaged line returns its original zero-based role and complete repair
evidence even when earlier lines and both held endpoints remain valid.

```rust,ignore
tuck.validate_current(&piece, &ledger)?;
pleat.validate_current(&piece, &ledger)?;
```

Both retain `GeometricValidation::DeferredToG2`, `IntakeValidation::DeferredToG2AndG3` and
`ProfileBindingValidation::DeferredToG4` for a present profile intake. No source query resolves intake,
executes a fold, establishes physical shape/count or certifies production readiness.

| API token | Tuck/Pleat meaning |
| --- | --- |
| `FoldDefinition` | Editable intake, directed folds, direction and operation |
| `FoldError` | Owner, intake, empty/duplicate-line or unresolved/foreign-source refusal |
| `FoldReferenceRole` | Original fold-list index or authored direction |

**Gather (`G1-SLICE.3c.4a.2b`).** Immutable `GatherDefinition` content binds graph, span,
selected side, held physical-copy id, directed gathering reference and closing-operation id.
`intake_source()` borrows the span's existing DeclaredEase, including its Uniform, weighted or
between-notch allocation and signed A-minus-B origin. Gather stores no second intake/allocation,
formula/profile value or uncertainty state.

A-minus-B = +5 000 µm permits gathering A; −5 000 µm permits B. Wrong-sign explicit selection refuses;
authored zero is legal no-op intent. Formula/profile origins supply no value/default. Resolved sign,
walking and actual shortening remain later proof obligations.

`Gather::new` checks graph/span/side-copy binding, a current CutPlan copy belonging to the named
Piece, and complete owned attachment/direction intervals with unique endpoints. Changing a span to
another copy returns CopyBindingChanged even with identical Piece/range. Missing targets or a
reassigned copy refuse; another copy never inherits held Gather identity.

Independent `copy()` validates the current plan. `intake_source()` validates graph/span/copy binding
and explicit sign, without checking a separately changed plan. `attachment_resolution()` and
`direction_resolution()` retain raw interval/endpoint evidence and composed traversal. Lost interiors
remain repairs despite live endpoints; no query chooses another span/copy or rewrites held content.

**Current Gather joins (`G1-SLICE.6.4.5.4.b`).** `validate_current()` requires the original Piece
id, then the original graph/span/copy/sign and complete source rules. `current_intake_source()`
returns the exact graph declaration only after those selected current checks pass. A graph-only
intake query can still succeed after a plan loses the held copy; this joined query refuses it.
Attachment or direction repairs return the actual role and complete evidence, without a partial view.
All supplied contexts stay borrowed while the result is used, preventing replacement of the plan,
Piece, graph or ledger under that view.

Use the actual selected Piece from the current inventory. Validate whole graph spans, stops,
landmarks and allocation bindings separately; the selected Gather query cannot certify them:

```rust,ignore
graph.validate_current(&plan, &pieces, &landmarks, &ledger)?;
let intake = gather.current_intake_source(&piece, &plan, &graph, &ledger)?;
```

Gather retains `GeometricValidation::DeferredToG2`, `IntakeValidation::DeferredToG2AndG3` and
`ProfileBindingValidation::DeferredToG4` for present profile ease. G2/G3 executes the required typed
closing operation and proves realized intake/allocation. Operation/parameter existence/kinds remain
.5/.6/G4 obligations. No current source query grants whole Design, physical or release readiness.

| API token | Gather meaning |
| --- | --- |
| `GatherDefinition` | Editable graph/span/side/copy, direction and operation binding |
| `GatherError` | Current owner/target/sign or complete source refusal |
| `GatherIntakeSource` | Borrowed signed canonical ease declaration and selected side |
| `GatherReferenceRole` | Attachment or authored direction |

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

Each layer's `validate_current(served, ledger)` rechecks its original served owner, material reason
and every complete owned source through the same rules/order as construction. Historical ranges
may resolve after edits; a same-id Piece replacement cannot retain ownership of removed content.
Raw endpoint choices and interior repairs remain explicit. Current validity grants no execution scope.

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

## Owned layer families and complete current sources

`LayerInventory::new(facings, linings, interfacings)` owns the exact original objects in each
authored family order. Its id census visits Facing, Lining, then Interfacing. Repeated ids within or
across families refuse with both original kind/position pairs.
Explicit empty families are draft scope and make no whole Design or assembly completeness claim.

`facings()`, `linings()` and `interfacings()` borrow original typed slices. `entries()` visits those
same families/orders with typed LayerRef borrows and original positions. `facing(id)` selects only an
actual Facing; a same-id Lining or Interfacing cannot substitute. These are inspection queries,
including held damage after edits, and supply no current source certificate. Debug prints only
family counts or the borrowed kind, without material reasons or customer source content.

`validate_current(pieces, ledger)` first rejects duplicate Piece identities, then checks every layer's
original served Piece and complete source list through its existing current validator. Missing owners
never retarget to another Piece. For example, an unused later Interfacing whose middle interval was
deleted refuses even when its endpoints and an earlier Facing remain valid. The refusal retains
that layer's identity, original family/position and concrete LayerError with raw evidence. Current
contexts are borrowed, never cached, and source references are never rewritten or implicitly chosen.

Modelled Lining can pass current source validation. Requested execution must check scope first:

```rust
layers.require_in_scope()?;
layers.validate_current(&current_pieces, &ledger)?;
```

The first Lining produces the original env_lining cause, served Piece and G7 even if its geometry
also needs repair. The modelled object remains available for inspection; no supported layer is
substituted. Passing scope certifies neither sources nor geometry. Material metadata is checked
[separately](material-inputs.md#layer-assignments); global ids, recipe operations and physical proof remain separate.

| API token | Layer collection meaning |
| --- | --- |
| `LayerInventory` | Private ordered Facing/Lining/Interfacing families and complete current sources |
| `LayerInventoryPosition` | Original semantic kind and zero-based authored position in that family |
| `LayerRef` | Shared original distinct layer type, without a validation certificate |
| `LayerInventoryError` | Duplicate ids/Pieces, missing targets, original layer/Material or named envelope causes |

### Borrowed offset intent

`inputs.layer_offset_declarations(&layers)?` checks the whole current root, then borrows each
original layer and offset relationship in Facing/Lining/Interfacing order. `canonical_inputs()`
retains the actual root owner. Operation ids matching numeric inputs remain operation intent;
source order/direction remain authored. Empty layers yield no views. Modelled Lining stays
inspectable; apply `view.layer().require_in_scope()?` before execution. Recipe operation checks,
current sources, assignments, global ids and physical proof remain separate.

| API token | Meaning |
| --- | --- |
| `DesignLayerOffsetDeclaration` | Original layer, canonical owner and unexecuted relationship |

## Hem: depth, fold binding and current Facing composition

`Hem` retains a stable identity, whole owned finish edge, `HemDepth`, `HemFoldType` and `HemMethod`.
Depth is an explicit parameter/value, formula declaration or logical target-profile declaration.
Explicit values are nonnegative: a deliberately authored zero is retained, while negative depth is a
`HemError::NegativeDepth`. Symbolic bindings supply no zero or other fallback. Fold type is required
and names a recipe/Design declaration or target-profile declaration; that owner retains its domain,
uncertainty and selected value. G1 invents no physical fold vocabulary from drawn lines.

For example, the reference skirt's 3.0 cm hem intent can retain its authored depth parameter and fold
binding on the finish edge. This descriptor performs no turning, truing or offset. The fixture's
**envelope corner** belongs to SeamAllowance corner treatment, not an implicit Hem fold type.
`HemMethod::Turned` and `HemMethod::Faced { facing: id }` separately state the finishing method.
A faced hem requires an existing Facing serving the same Piece: there is no generated substitute,
second material assignment or copied Facing source list in the Hem.

`Hem::new` checks complete owned finish-edge coverage and unique endpoints. It checks the held Facing
identity, served Piece and its current sources through the layer validator. `facing()` borrows that
canonical current target after repeating those checks. A removed target, replacement id, reassigned
served Piece or invalid source returns a typed refusal. In particular, an old Facing whose source
loses its middle interval is refused even if both original endpoints remain live. Same-id replacement
content is inspected as current content, preserving its authored material/source intent.

`edge_resolution()` preserves current fragment directions and raw interior repairs without rewriting
the Hem. Facing and edge queries are independent: a valid Facing target does not certify the Hem's
finish edge, physical fold or release readiness. Recipe/Design validates every declaration's existence,
kind/domain/state and all current composition registries before execution. Profile fields report
`DeferredToG4`; absence means only no profile field was authored. Actual fold geometry and executed
finishing remain G2/G3, and every Hem reports `GeometricValidation::DeferredToG2`.

`Hem::validate_current(piece, ledger, facing)` checks the original owner, depth domain, whole finish
edge and original current Facing target in that order. Construction shares these rules. Facing
validation borrows the actual object's current validator without cloning its definition. A healthy
Facing and two live finish endpoints cannot conceal a deleted finish interior; held input stays intact.

| API token | Meaning in the Hem implementation |
| --- | --- |
| `HemDefinition` | Editable finish edge, depth/fold origins and finishing method |
| `HemDepth` | Explicit authored nonnegative depth or formula/profile declaration |
| `HemFoldType` | Required logical recipe or target-profile fold-type binding |
| `HemMethod` | Turned finish or faced finish bound to a stable Facing identity |
| `HemError` | Typed depth, finish-edge and current composition-target refusal |
| `NegativeDepth` | Explicit authored depth is below zero |
| `MissingFacing` | Held faced-method target is absent |
| `WrongFacing` | Provided target differs from the held stable Facing identity |
| `FacingOutsidePiece` | Current target serves a different Piece |
| `InvalidFacing` | Current target's source/material content fails layer validation |

HemError exposes its original InvalidFacing LayerError through std::error::Error::source().
Chained diagnostic tools retain that same concrete error and its raw source-repair evidence.
This changes no facing validation or stored intent; range evidence remains data, not a nested error.

## Owned Hems and complete current Facing joins

`HemInventory::new(hems)` owns exact authored order and rejects duplicate ids with both original
positions. `hems()` and `hem(id)` borrow original objects, including held damage after edits; raw
presence supplies no current certificate. An explicit empty collection is draft scope.

`validate_current(layers, pieces, ledger)` first checks every supplied layer's current served/source
scope, including unused layers, then each Hem's original Piece, complete finish edge and exact Facing
through the existing Hem validator. Duplicate Pieces refuse even with no Hems. Missing owners never
retarget to another Piece. A lost finish interior in a later Hem refuses even when both endpoints and
an earlier Hem remain valid; the original position/id/concrete HemError and repair data are retained.

Facing joins select only actual Facings. A removed Facing100 never transfers to surviving101, and
Interfacing100 cannot stand in for it. A same-id Facing now serving another Piece also refuses.
An unused damaged layer blocks the complete query even if all Hem targets are healthy or no Hems
exist; its original LayerInventoryError/LayerError chain remains reachable. Objects and contexts are
never rewritten or cached. Debug reports the Hem count without source content.

For requested execution, apply layers.require_in_scope() first. Material assignment metadata remains
its separate query; depth/fold declarations, global Design ids, physical folding and G2/G3/G4 proof
remain separate. Current structural success grants no assembly or release readiness.

| API token | Hem collection meaning |
| --- | --- |
| `HemInventory` | Private ordered actual Hems and complete current layer/finish/Facing checks |
| `HemInventoryError` | Duplicate ids, original Layers cause, missing Piece or contextual Hem cause |

## Borrow actual Hem depth and fold declarations

`DesignInputs::hem_declarations(hems)` first validates the complete current input root, then borrows
depth and fold-type roles for every actual Hem in authored order. Each view retains its original
Hem and the root's single canonical input owner. No partial view collection escapes a refusal.

Explicit depth borrows the original parameter id and nonnegative Length intent: authored zero is
retained without creating a second canonical value or inventing parameter metadata. Formula depth
must name an actual DesignParameter, then its current canonical Length-kind declaration. Dedicated
lengths and general Length scalars both work. A bare canonical id refuses MissingFormulaParameter;
a Count parameter refuses FormulaKind with the actual Hem/id/kind. Unknown and Derived values stay
unread and unchanged; this query neither evaluates depth nor proves a resolved nonnegative result.

Fold type borrows the original HemFoldType. A logical declaration is not converted into a scalar
because its id matches a numeric input; its recipe/Profile owner supplies domain, selection and state.
Profile depth/fold fields remain DeferredToG4. profile_validation() returning None means only that
role has no Profile field, without readiness approval. Default Debug omits authored ids/values.

For example, an explicit zero-depth intent, a formula over a Derived length and an Unknown general
Length expose different original sources. A profile-backed fold remains logical even when its id
matches a stored Length. No physical fold choice, depth fallback or computed finishing is supplied.
Current finish/Facing/layer sources, Material assignments, global garment ids and G2/G3/G4 physical
proof remain separate. Original current-root errors keep their concrete standard cause chain.

| API token | Hem declaration meaning |
| --- | --- |
| `DesignHemDeclaration` | Borrowed actual Hem, canonical owner, role and original source |
| `DesignHemDeclarationRole` | Depth then FoldType for each original Hem |
| `DesignHemDeclarationSource` | ExplicitDepth, FormulaDepth, ProfileDepth or logical FoldType |
| `DesignHemDeclarationError` | Current Inputs cause or actual missing/wrong-kind formula parameter |

## Pocket placement and physical composition

`PocketDefinition` binds a served physical copy and its source Piece, a held position anchor,
owned directed orientation, required logical opening and nonempty component references. Each
`PocketPieceRef` pairs a stable copy id with the expected source Piece. For example, a pocket on
copy `front-left` can name components `bag-left` and `bag-right`, both sourced from the same
pattern “bag”. The components remain distinct physical objects; their pattern geometry is shared.
The served copy may itself appear explicitly as a component. G1 does not infer construction
support or recipe-cycle correctness from that relationship.

`Pocket::new` requires current copies/Pieces, unambiguous Piece ids, unique component copy ids,
a live owned birth anchor and complete owned orientation intervals with uniquely resolved ends.
`validate_current()` repeats target and placement checks. Removing a copy does not select its
peer; assigning the same copy id to a different source Piece fails its explicit source guard.
Historical anchors may resolve after split/reversal, while a split choice or deletion remains a
visible refusal. Orientation cannot hide foreign or deleted middle geometry behind valid ends.
Authored direction is separate from the copy's reflected geometry, which remains G2/V1 work.

PocketError position refusals expose the original concrete AnchorError through the standard
std::error::Error::source chain. Its pointer and raw repair data are retained, without boxing the
cause behind another dynamic Box type. RangeResolution remains inspectable data, not an invented
error cause. Placement behavior, definitions and physical opening deferrals are unchanged.

`component(copy, plan, pieces)` borrows the current canonical Piece, including its labels/material
and raw contour repair evidence. Reordering components does not change the queried target.
Replacing a same-id Piece changes the newly observed metadata without rewriting old Pocket intent.
This target query and Pocket placement validation do **not** certify component contours: Design
must inspect every component's `range_resolutions()` and reject lost interiors before execution
or release. Complete contour evidence still requires G2 geometry validation.

`PocketOpening::Declaration(operation)` and `PocketOpening::Profile(parameter)` preserve logical
origins without inventing an opening vocabulary or default. Every Pocket reports
`PocketOpeningValidation::DeferredToG3`; carrying a binding does not establish supported execution
scope. Recipe/Design resolves its declaration, domain/state and dependencies; G3 executes pocket
construction and consumes named envelope diagnostics before execution. A profile opening also
reports `Some(ProfileBindingValidation::DeferredToG4)`. `None` means only no profile opening was
authored. Placement/contour geometry reports `GeometricValidation::DeferredToG2` independently.

| API token | Meaning in the Pocket implementation |
| --- | --- |
| `PocketDefinition` | Editable served target, placement, opening and explicit composition |
| `PocketPieceRef` | Stable physical-copy id plus expected source Piece id |
| `PocketOpening` | Required recipe/Design or profile opening declaration |
| `PocketOpeningValidation` | Opening resolution and executable scope remain G3 |
| `None` | No profile opening field was authored; no resolved-value proof |
| `PocketError` | Typed target, composition, anchor and orientation refusal |
| `NoComponents` | No authored composition references |
| `DuplicateComponent` | Repeated physical component copy, regardless of claimed source |
| `WrongCopyPiece` | Current copy source differs from its held expected source |
| `MissingComponent` | Query names no authored component copy |
| `UnresolvedOrientation` | Raw incomplete interval or endpoint-choice evidence |
| `OrientationOutsidePiece` | Some current orientation interval is foreign to the served Piece |

## Owned Pocket collection and current component intervals

PocketInventory::new(pockets) owns exact authored Pocket objects in order. Duplicate identities
refuse with original first/repeated positions; nothing is sorted, selected or repaired. pockets() and
pocket(id) return shared original objects. Raw presence supplies no current certificate, and an
explicitly empty collection supplies no whole Design or assembly completeness claim.

```rust
let pockets = PocketInventory::new(vec![left_pocket, right_pocket])?;
pockets.validate_current(&current_plan, &current_pieces, &ledger)?;
```

The current query first revalidates the complete CutPlan against actual current Pieces, then each
Pocket's targets/placement, then every authored component's full held intervals: boundary, holes and
construction lines. Constructor success and individual component lookup cannot replace these checks.
A plan built for cut-two cannot remain current after the same-id Pieces change to cut-four, even if
each Pocket reference still finds its old copy. The original CutPlanError is retained as its cause.

For example, split a bag component edge into thirds and delete the middle. Pocket placement and
both component endpoints can still resolve. The inventory returns ComponentRange for the actual
Pocket/component copy/source Piece/edge position with live/repair/live evidence. This applies to
holes and construction lines too, retaining original traversal separately from forward-frame range
portions. Only an exact source-Piece/edge/parameter registration supplies a saved endpoint choice;
another consumer cannot choose for the Piece. The canonical ledger resolver applies that choice,
while the raw range endpoints remain unchanged. A chosen branch that later becomes orphaned blocks
with its choice-aware endpoint answer alongside the raw interval evidence. No deleted interval is
sampled away or reattached. The unchanged Pocket remains inspectable.

Every Pocket is checked, including a later one after earlier success. Pocket wrappers preserve the
concrete original PocketError and its AnchorError chain; component range payloads remain data rather
than invented Error causes. All failures retain authored positions without an accepted prefix.

| API or variant | Meaning |
| --- | --- |
| `PocketInventory` | Private ordered actual Pocket collection with current component-reference checks |
| `PocketInventoryError` | Duplicate id, current plan/Pocket cause or full component range evidence |
| `ComponentRange` | Original component interval or endpoint repair/choice with actual locations |

This is current reference coverage, not geometric contour validity. Logical opening declaration
kind/domain/state/dependencies still belong to recipe/Design validation; opening execution is G3,
profile resolution G4 and physical geometry G2. Existing Pocket deferral queries stay unchanged.
Global identity joins with the rest of Design remain separate. See
[the public source](../../../../crates/sc-core/src/ontology/pocket_inventory.rs).

These layer/Hem APIs are component contracts; complete Design ownership and coupled acceptance remain pending.
