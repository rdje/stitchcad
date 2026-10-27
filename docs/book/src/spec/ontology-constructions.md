# Executable garment constructions at G1

See [ontology §4.3/§4.7](ontology.md) and [piece/sewing/mark APIs](ontology-implementation.md).
Current construction source/declaration acceptance passes. Executed constructions, conserved intake,
bounded offsets and receiver release require later gates.

**Dart (`G1-SLICE.3c.4a.1`).** `Dart::new(DartDefinition, Piece, ledger)` retains immutable id,
intake origin, apex, two directed legs, directed closing/folding reference and closing-operation id.
Explicit intake holds its parameter id and nonnegative Length, including zero; negative intake returns
`DartError`. Formula/profile origins hold declarations without values, defaults or copied uncertainty;
registries check existence, kinds and resolved values.

An apex EdgeAnchor may end an owned internal leg. Legs/direction require complete owned positive
intervals and unique endpoints; identical held legs refuse even with opposite traversal. Unknown/
foreign geometry names its role; owned endpoints cannot conceal foreign middles after merge/reversal.
Valid references prove no straightness, apex coincidence or physical folding direction.

A skirt can retain40 000 µm intake, two internal directed legs, an apex on one and a closing-operation
id. G2/G3 executes closure and compares removed boundary length with intake under its tolerance class.
G1-SLICE.5/.6 validates operation existence, kind and dependencies. Replacing intake or operation builds a new object; authored zero never substitutes for
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

**Tuck and Pleat (`G1-SLICE.3c.4a.2a`).** Distinct immutable types share `FoldDefinition`/rules:
id, intake origin, nonempty ordered directed fold ranges, directed folding reference, closing-operation
id. Empty/duplicate intervals refuse, including opposite traversal; each positive interval requires
complete owned coverage and unique endpoints. Distinct partial ranges on one edge remain valid intent.
List length proves no physical count, coincidence or shape. Explicit zero retains provenance, negative
intake refuses, and symbolic intake supplies no value/fallback.

A tuck may hold one internal fold range/owned direction; a pleat several ordered lines. The registry
checks its typed closing operation; G2/G3 executes it and compares removed boundary length with
resolved intake.

`references()` lists authored fold lines, then direction. `reference_resolutions()` retains raw
interval/endpoint evidence and composed traversal. Reversed splits visit fragments in opposite order;
a deleted middle remains a repair despite live endpoints. Foreign interiors refuse before/after
reversal. Editing cloned input cannot change either held object.

**Current folds (`G1-SLICE.6.4.5.4.a`).** Original rules require the held Piece, then all current
lines/direction without rewriting references or choosing endpoints. Later damage retains its original
zero-based role and complete repair evidence even with valid earlier lines and held endpoints.

```rust,ignore
tuck.validate_current(&piece, &ledger)?;
pleat.validate_current(&piece, &ledger)?;
```

Both retain `GeometricValidation::DeferredToG2`, `IntakeValidation::DeferredToG2AndG3` and, with
profile intake, `ProfileBindingValidation::DeferredToG4`. Source queries resolve no intake, executed
fold, physical shape/count or production readiness.

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

**Current Gather joins (`G1-SLICE.6.4.5.4.b`).** `validate_current()` requires the held Piece,
then original graph/span/copy/sign and complete source rules. `current_intake_source()` borrows the
exact graph declaration afterward; unlike the graph-only query, it refuses a plan's lost held copy.
Attachment/direction repairs retain original role/evidence without partial views. The view borrows
all contexts, preventing replacement of its plan, Piece, graph or ledger.

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

**Owned construction families (`G1-SLICE.6.4.5.4.d.1`).** Immutable vectors preserve exact
objects/authored order: Dart/Tuck/Pleat/Gather/Grainline/SeamAllowance, without id sorting or kind
merging. `new` refuses the first repeated owned id with both positions. Piece/parameter/profile/
operation references remain uses; Pocket/notion/Closure/layer/Hem inventories stay separate.

```rust,ignore
let sources = ConstructionInventory::new(darts, tucks, pleats, gathers, grains, allowances)?;
for (position, actual) in sources.entries() {
    inspect(position.kind, position.index, actual.id(), actual.piece());
}
```

Getters/entries borrow actual objects; Debug shows counts/kind only. Empty families/held repairs
remain. Ownership alone checks no current sources.

`validate_current(pieces, plan, graph, landmarks, ledger)` first rechecks the whole graph, including
CutPlan quantities/orientations, unique Piece context, all spans, stops and distributions, even with
no constructions. It then validates every held Piece and complete source in family/authored order.
An unused bad graph span blocks a healthy Dart; a damaged later construction blocks the whole check.
Missing owners never retarget. Failures retain actual id, original family/index and concrete cause
with raw repair evidence. `Error::source()` reaches original family errors, including Dart AnchorError;
Graph → SewingError → CutPlanError also retains its exact standard chain (D273). No partial result,
cached success, rewritten reference or inferred endpoint choice escapes.

```rust,ignore
sources.validate_current(&pieces, &plan, &graph, &landmarks, &ledger)?;
```

Declaration roles, global Design ids and physical/release readiness remain separate.

| API token | Meaning |
| --- | --- |
| `ConstructionInventory` | Exact six-family ownership |
| `ConstructionKind` | Distinct semantic family |
| `ConstructionInventoryPosition` | Family and zero-based authored index |
| `ConstructionRef` | Typed borrow of an actual stored object |
| `ConstructionInventoryError` | Duplicate locations, Graph, missing Piece or concrete family cause |

### Intake and closing declarations

`inputs.construction_intake_declarations(&sources)?` validates the whole current root, even empty,
then borrows Intake/ClosingOperation per Dart/Tuck/Pleat in authored family order. Views retain the
actual object, original position and single canonical owner.

```rust,ignore
for view in inputs.construction_intake_declarations(&sources)? {
    inspect(view.position(), view.role(), view.source());
}
```

Explicit intake borrows its parameter id/nonnegative Length, including zero. Formula intake requires
actual Design parameter/current Length metadata, dedicated or general; Unknown/Derived stay unread.
Bare numeric ids and Count refuse with actual construction/position/role and no partial views.
ProfileIntake stays DeferredToG4; None means no profile field. ClosingOperation stays logical despite
numeric-id matches. Debug omits ids/values. Operation existence/kind/dependencies, current garment
sources, global ids, resolved intake/conservation and release proof remain separate. Gather ease
stays on its span.

| API token | Meaning |
| --- | --- |
| `DesignConstructionDeclaration` | Actual object/position/canonical owner/field |
| `DesignConstructionDeclarationRole` | Intake/closing, grain alignment, allowance width/inclusion |
| `DesignConstructionDeclarationSource` | Original typed role source; formula metadata or inline/logical intent |
| `DesignConstructionDeclarationError` | Inputs cause or contextual missing/wrong-kind parameter |

### Grain and allowance declarations

```rust,ignore
let views = inputs.grain_allowance_declarations(&sources)?;
```

Whole-root preflight and actual object/position/canonical borrows apply, even empty. GrainAlignment roles precede allowances; each allowance yields AllowanceWidth then
AllowanceInclusion. FormulaAngle requires actual Angle parameter metadata; FormulaWidth requires
Length (dedicated/general). Bare canonical ids and wrong kinds refuse without partial views;
Unknown/Derived remain unread.

ParallelAlignment borrows the directed axis without inventing an angle. ExplicitAngle preserves its
Angle; ProfileAngle stays logical. ExplicitWidth borrows original parameter/Length, including zero;
ProfileWidth and required AllowanceInclusion retain their exact Profile refs, even with numeric-id
matches. Profile roles stay DeferredToG4. Axis/stripe/plaid/corner intent stays on the object; no current-source,
resolved width-domain, alignment/offset, policy or release proof follows.

**Facing, Lining and Interfacing (`G1-SLICE.3c.4b.1`).** Distinct immutable types share
`LayerDefinition`: served Piece, `LayerOffsetRelationship` (recipe offset-operation id/nonempty owned
directed ranges), material assignment. Recipe owns dimensions/input states and checks operation
existence/kind/dependencies with Design; descriptors cache no contours/defaults/output-Piece material.

For example, waistband interfacing names an offset operation over owned inner-half construction
ranges; the recipe supplies the 4.0 × 77.0 cm finished rectangle/non-sewn fused attachment, without
proof/generation by this descriptor. Facing likewise retains Piece/recipe relationship, never inferred
line art. Assigned Material ids need Design registry checks; unresolved Material needs a nonblank
reason, without substitution. The shared reason check preserves the original Piece invariant.

Constructors require the held Piece; unknown/foreign portions, interval repairs, endpoint choices,
empty sources/duplicate intervals return `LayerError`. `source_resolutions()` preserves authored order/
directed raw evidence: merged owned ends cannot own foreign middles, even reversed; deletions remain
visible. Cloned material/offset edits cannot mutate layers. Shape/bounded-error/physical proof stays G2.

`validate_current(served, ledger)` repeats birth rules/order for original owner, material reason and
all complete sources. Historical ranges may resolve after edits; same-id Piece replacements cannot
own removed content. Raw choices/interior repairs stay explicit; current validity grants no scope.

**Modelled content differs from execution scope** (feature matrix rule3). `Lining::new` checks
structure; `require_in_scope()` refuses v1 with `LayerEnvelopeError::LiningDeferred`, `diagnostic()`
env_lining, served-Piece payload and `proving_gate()` G7. G1-SLICE.6 enforces this before execution;
inspection stays available. Facing/Interfacing pass scope without substituting. Every layer retains
`GeometricValidation::DeferredToG2`; birth/scope prove no geometry/release approval.

| API token | Meaning in the served-layer implementation |
| --- | --- |
| `LayerDefinition` | Editable served Piece, offset relationship and material input |
| `LayerOffsetRelationship` | Recipe offset-operation identity and owned directed source ranges |
| `LayerKind` | Distinct facing, lining or interfacing semantic kind |
| `LayerError` | Typed target, source-range or material-reason refusal |
| `LayerEnvelopeError` | Typed execution-scope refusal independent of structural birth |
| `LiningDeferred` | Modelled lining request refuses with env_lining, served Piece and G7 |

## Owned layer families and complete current sources

`LayerInventory::new(facings, linings, interfacings)` owns exact objects in authored family order.
The census visits Facing/Lining/Interfacing; duplicate ids within/across families retain both original
kind/position pairs. Empty families are draft scope, without whole Design/assembly completeness.

`facings()`/`linings()`/`interfacings()` lend original typed slices; `entries()` keeps family/order/
positions through LayerRef. `facing(id)` cannot select same-id Lining/Interfacing. Damaged objects stay
inspectable without current certification; Debug shows counts/kind, no material reasons/customer sources.

`validate_current(pieces, ledger)` rejects duplicate Piece ids, then checks each original served Piece/
complete source list. Missing owners never retarget: unused later Interfacing with a deleted middle
refuses despite live endpoints/earlier healthy Facing, retaining layer id/family/position and concrete
LayerError/raw evidence. Contexts stay borrowed/uncached; refs are never rewritten or implicitly chosen.

Modelled Lining can pass current source validation. Requested execution must check scope first:

```rust
layers.require_in_scope()?;
layers.validate_current(&current_pieces, &ledger)?;
```

The first Lining retains env_lining, served Piece and G7 even with damaged geometry; inspection stays
available without substitution. Scope proves no sources/geometry. [Material metadata](material-inputs.md#layer-assignments),
global ids, recipe operations and physical proof remain separate.

| API token | Layer collection meaning |
| --- | --- |
| `LayerInventory` | Private ordered Facing/Lining/Interfacing families and complete current sources |
| `LayerInventoryPosition` | Semantic kind and zero-based authored family index |
| `LayerRef` | Shared original distinct layer type, without a validation certificate |
| `LayerInventoryError` | Duplicate ids/Pieces, missing targets, original layer/Material/envelope causes |

### Borrowed offset intent

`inputs.layer_offset_declarations(&layers)?` checks the whole current root before borrowing original
layer/offset in Facing/Lining/Interfacing order; `canonical_inputs()` retains that owner. Operation ids
stay logical despite numeric matches; source order/direction stay authored. Empty layers yield no views.
Lining stays inspectable: apply `view.layer().require_in_scope()?` before execution. Operations/current
sources/assignments/global ids/physical proof remain separate.

| API token | Meaning |
| --- | --- |
| `DesignLayerOffsetDeclaration` | Original layer, canonical owner and unexecuted relationship |

## Hem: depth, fold binding and current Facing composition

`Hem` retains stable id, whole owned finish edge, `HemDepth`/`HemFoldType`/`HemMethod`. Depth retains
explicit parameter/value, formula or logical target-profile declaration. Authored zero stays explicit;
negative values return `HemError::NegativeDepth`; symbolic bindings supply no fallback. Required fold
type names a recipe/Design or target-profile declaration whose owner retains domain/uncertainty/value.
G1 invents no physical fold vocabulary from line art.

For example, the reference skirt's 3.0 cm hem intent can retain its authored depth parameter and fold
binding on the finish edge. This descriptor performs no turning, truing or offset. The fixture's
**envelope corner** belongs to SeamAllowance corner treatment, not an implicit Hem fold type.
`HemMethod::Turned` and `HemMethod::Faced { facing: id }` separately state the finishing method.
A faced hem requires an existing Facing serving the same Piece: there is no generated substitute,
second material assignment or copied Facing source list in the Hem.

`Hem::new` checks complete owned finish coverage/unique endpoints, then the held Facing identity,
served Piece and current layer sources. `facing()` borrows that target after repeating those checks.
Removed/replacement ids, reassigned Pieces or invalid sources refuse; a Facing's lost middle refuses
despite live endpoints. Same-id replacements use current authored material/source content.

`edge_resolution()` retains fragment directions/raw interior repairs without rewriting. Independent
Facing success certifies no finish edge, physical fold or release readiness. Recipe/Design checks
declaration existence/kind/domain/state and current composition registries before execution. Present
profile fields report `DeferredToG4`; absence means only no profile field. Geometry/executed finishing
remain G2/G3; every Hem retains `GeometricValidation::DeferredToG2`.

`Hem::validate_current(piece, ledger, facing)` shares birth rules: original owner, depth, whole finish,
current Facing. It borrows the actual Facing validator without cloned definitions. Healthy Facing/
finish endpoints cannot hide a deleted interior; held input stays intact.

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

HemError's InvalidFacing exposes the original LayerError/raw evidence through `Error::source()`.
Validation and stored intent are unchanged; range evidence is data, not a nested error.

## Owned Hems and complete current Facing joins

`HemInventory::new(hems)` owns exact authored order and rejects duplicate ids with both original
positions. `hems()` and `hem(id)` borrow original objects, including held damage after edits; raw
presence supplies no current certificate. An explicit empty collection is draft scope.

`validate_current(layers, pieces, ledger)` checks every layer's served/source scope, including unused
layers, then each Hem's original Piece, complete finish edge and exact Facing through its validator.
Duplicate Pieces refuse even without Hems; missing owners never retarget. A later lost finish interior
refuses despite live endpoints/earlier Hems, retaining original position/id/HemError and repair data.

Joins select actual Facings: removed100 never transfers to101 or Interfacing100; same-id reassignment
also refuses. Unused damaged layers block healthy/empty Hem scope with the original LayerInventoryError/
LayerError chain. Objects/contexts are never rewritten or cached; Debug shows Hem count only.

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

Explicit depth borrows parameter id/nonnegative Length, preserving zero without another canonical
value or invented metadata. Formula depth requires actual DesignParameter/current Length declaration;
dedicated/general Length both work. Bare canonical ids return MissingFormulaParameter; Count returns
FormulaKind with actual Hem/id/kind. Unknown/Derived remain unread/unchanged, without evaluation or
resolved nonnegativity proof.

Fold type borrows HemFoldType; numeric-id matches never convert logical intent to scalar. Its
recipe/Profile owner supplies domain/selection/state. Profile depth/fold stays DeferredToG4;
profile_validation() None means no Profile field, without approval. Debug omits ids/values.

For example, explicit zero, Derived length and Unknown general Length retain distinct sources.
Profile folds stay logical despite numeric-id matches. No fold choice/depth fallback/finishing is
computed. Current finish/Facing/layer sources, Materials, global ids and G2/G3/G4 proof remain
separate; current-root errors retain concrete standard causes.

| API token | Hem declaration meaning |
| --- | --- |
| `DesignHemDeclaration` | Borrowed actual Hem, canonical owner, role and original source |
| `DesignHemDeclarationRole` | Depth then FoldType for each original Hem |
| `DesignHemDeclarationSource` | ExplicitDepth, FormulaDepth, ProfileDepth or logical FoldType |
| `DesignHemDeclarationError` | Current Inputs cause or actual missing/wrong-kind formula parameter |

## Pocket placement and physical composition

`PocketDefinition` retains served physical copy/source Piece, position anchor, owned directed
orientation, required logical opening and nonempty components. Each `PocketPieceRef` pairs stable
copy/expected Piece ids. A pocket on `front-left` may use `bag-left`/`bag-right` from one “bag” pattern:
distinct physical objects share pattern geometry. The served copy may explicitly be a component;
G1 infers no construction support or recipe-cycle correctness.

`Pocket::new` requires actual copies/unique Piece ids, unique component copies, a live owned anchor
and complete owned orientation with unique ends. `validate_current()` repeats target/placement checks:
missing copies never select peers; same-copy Piece reassignment refuses. Historical split/reversal
may resolve, but choices/deletions stay visible. Live ends cannot hide foreign/lost middles.
Direction remains separate from reflected geometry (G2/V1).

PocketError positions expose original AnchorError/pointer/repair data through `Error::source()`
without dynamic Box wrapping. RangeResolution stays data; placement/definitions/opening deferrals
are unchanged.

`component(copy, plan, pieces)` borrows current Piece labels/material/raw contour evidence. Reordering
components preserves targets; same-id replacement exposes current metadata without rewriting intent.
Target/placement success proves no component contours: Design must inspect every `range_resolutions()`
and refuse lost interiors before execution/release. G2 must still validate geometry.

`PocketOpening::Declaration(operation)`/`Profile(parameter)` retain logical origins without opening
vocabulary/defaults. Every Pocket has `PocketOpeningValidation::DeferredToG3`; binding proves no
execution scope. Recipe/Design checks declaration/domain/state/dependencies; G3 executes construction
after named envelope checks. Profile openings add `Some(ProfileBindingValidation::DeferredToG4)`;
`None` means no profile field. Geometry independently retains `GeometricValidation::DeferredToG2`.

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

### Original Pocket opening declarations

```rust,ignore
let openings = inputs.pocket_opening_declarations(&pockets)?;
```

The whole current input root is checked before views, even empty scope. Each view borrows authored
index, actual Pocket/opening and canonical owner. Logical declaration/Profile ids retain original
G3/G4 deferrals despite numeric matches. Debug shows origin only; placement/components, declaration
domains/dependencies, physical opening and release remain separate.

| API token | Meaning |
| --- | --- |
| `DesignPocketOpeningDeclaration` | Actual Pocket/opening/index/canonical borrow with original deferrals |

## Owned Pocket collection and current component intervals

PocketInventory::new(pockets) owns exact authored order; duplicate ids retain first/repeated positions
without sorting/selection/repair. pockets()/pocket(id) borrow originals. Raw presence grants no current
certificate; empty scope grants no whole Design/assembly completeness.

```rust
let pockets = PocketInventory::new(vec![left_pocket, right_pocket])?;
pockets.validate_current(&current_plan, &current_pieces, &ledger)?;
```

The query revalidates complete CutPlan/Pieces, each Pocket's targets/placement, then full component
boundary/hole/construction intervals. Birth or component lookup cannot substitute: cut-two changing
to cut-four refuses despite surviving copy refs, retaining original CutPlanError.

Split a bag edge into thirds and delete its middle: placement/both endpoints may still resolve.
ComponentRange retains actual Pocket/component copy/source Piece/edge position and live/repair/live
evidence, also for holes/construction lines. Original traversal stays separate from forward-frame
portions. Saved endpoint choice requires exact source-Piece/edge/parameter registration; peers cannot
choose for it. The canonical resolver applies choices without changing raw endpoints. An orphaned
chosen branch blocks with its choice-aware answer/raw intervals. Deleted intervals are never sampled
away or reattached; the unchanged Pocket remains inspectable.

Every Pocket is checked; later failure retains authored position without an accepted prefix.
Wrappers preserve PocketError/AnchorError chains; component range payloads remain data.

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

Layer/Hem components still require complete Design acceptance.
