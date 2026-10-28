# Actual model resource accounting

These component producers borrow actual held content under the
[logical resource contract](command-operation-control.md#logical-dynamic-content-accounting-contract).
The structural and complete Inputs/Sewing/Garment roots compose their actual fields under one
cumulative sink, including actual source content and its retained identity census. Runtime integration and [coupled native checks](#coupled-native-resource-acceptance) are implemented; accounting grants no authority or currentness certificate.

## Coupled native resource acceptance

At resource checkpoint0473, after implementation and D293 repair, these selected native suites passed:

| Actual test target | Passed cases |
| --- | --- |
| sc-measure: design_inputs_contract | 158, including93 actual command-bus cases |
| sc-measure: resource_contract | 33 |
| sc-core: design_resource_contract | 47 |
| sc-core: command_control_contract | 6 |
| sc-core: design_metadata_contract | 8 |

These252 cases cover actual source/model composition, all implemented metadata preview/commit/history
paths, exact limits and first witnesses, aliases/replay, current and prospective states, authority,
cancellation, reserve ownership and source cache stability. Five private unit cases also pass for
model/audit successor forecasts, exact bit/entry overflow chunks and independent reserve cursors.
Eleven bus, four resource and one operation-policy compile-fail doctests preserve privacy/sealing.

The primary native commands are:

```bash
python3 -I -B scripts/local_environment.py -- cargo test -p sc-measure --test design_inputs_contract --test resource_contract
python3 -I -B scripts/local_environment.py -- cargo test -p sc-core --test design_resource_contract --test command_control_contract --test design_metadata_contract
```

This closes the implemented logical-content resource feature's native acceptance. Broader command
acceptance, persistence/reopen, browser execution, raw ingress, numeric/file/decompression/geometry
work, allocator peaks, arbitrary source algorithm time and physical/release proof remain separate.
No whole-project CI or release verdict follows from these selected suites.
The table records that checkpoint; [two subsequent transaction cases](command-operation-control.md#mixed-transaction-checkpoint-acceptance)
pass separately. The commands also include those later cases; current broader bus acceptance stays .6.9.

## Actual canonical-input accounting

charge_canonical_input_resources borrows an actual CanonicalInputs and charges its existing inventory
and Known-state evidence vectors into the supplied sink. It creates no replacement declarations,
looks up no registry, extracts no numeric availability and changes no value/state/provenance/order.

```rust
let mut meter = DesignResourceMeter::new(resource_limits);
charge_canonical_input_resources(&canonical_inputs, &mut meter)?;
let canonical_component = meter.finish()?;
let actual_entries = canonical_component.collection_entries();
```

The order is exact: charge the length inventory's full length, visit lengths in authored order and
charge each Known evidence vector, then do the scalar inventory and its Known evidence vectors.
Static fields are canonical.lengths, canonical.lengths.evidence, canonical.scalars and
canonical.scalars.evidence in Structure scope. Inventory charges have no index; evidence charges
retain the actual declaration owner identity and original zero-based declaration index. Reused evidence identities in different declarations
still occupy entries in both original vectors; accounting does not deduplicate them.

For example, the native actual inventory has five Length declarations covering Known, Assumed,
Unknown, Preference and Derived, plus six Scalar declarations covering those states and a computed
Area. The Known length and Known scalar each hold two references, reusing the same identities.
Demand is15 collection entries (5+2+6+2), zero text bytes and zero unbounded-counter bits.
Maximum14 refuses at canonical.scalars.evidence, index0, consumed13/requested2. Maximum4 refuses
first at the full length inventory (consumed0/requested5), before inspecting nested evidence.
Maximum6 admits those five records then refuses their first two-reference evidence vector.

An exhaustive internal owner view and exhaustive declaration/state patterns require accounting
updates when a new owned field/state is added. Present fixed value payloads must remain Copy; a new
owned dynamic payload requires revisiting the producer. Unknown/Derived states remain authored
states, with no value fallback. Empty actual canonical scope contributes zero entries; an earlier
meter refusal stays the exact original error, even if this producer's error is ignored.

Four actual canonical cases plus the previous six meter cases pass:10 total, with strict selected
lint and formatting. The command-bus source remains unchanged. This is canonical-component proof;
Higher roots, source traversal and runtime admission are implemented below; implemented component accounting is described in the following sections. Finishing this component's meter supplies no
whole-model, source-truth, allocator RSS/time or publication certificate.

## Actual Piece and cut-plan accounting

charge_piece_inventory_resources borrows the actual Piece inventory. It precharges Pieces, then each
Piece's boundary, outer hole list, each hole's edge vector, construction lines and fold references in
schema/authored order. It charges exact unresolved Material-reason bytes, then name/size/fabric/colorway
label bytes. Assigned Material identity contributes no reason text; no registry lookup or physical
property inference occurs. charge_cut_plan_resources charges the actual physical-copy vector.

```rust
let mut meter = DesignResourceMeter::new(resource_limits);
charge_piece_inventory_resources(&pieces, &mut meter)?;
charge_cut_plan_resources(&cut_plan, &mut meter)?;
let represented_components = meter.finish()?;
```

The producers do not rebuild a Piece or copy against a later topology ledger. Actual held references,
assignment/label text and physical-copy order remain unchanged after success/refusal. Exhaustive
private owner/definition/label views and fixed Copy-reference constraints expose new owned fields.
This is content accounting, not current-reference, Material truth, geometry or release validation.

Nested locations now carry optional actual owner identity as well as scope/field/index. For a hole-edge
vector, pieces.holes.edges retains its Piece owner and original hole index. Other Piece member fields
retain owner and original Piece index. Whole inventories/copy vectors have no standalone entity owner.
Canonical evidence locations likewise carry actual declaration identity. No dynamic path string or
copied user label/value enters an error; identities supply location, not authority.

The native fixture holds two Pieces, each with boundary3, two hole vectors containing1 and2 edges,
construction1 and fold1. Piece entries total22, and two physical copies bring the total to24. Each
label is7 UTF-8 bytes; the first unresolved reason ` é ` adds4 bytes, while the second Piece is Assigned.
Text demand is18 bytes, with no trimming/deduplication. Maximum18 entries refuses the second Piece's
second hole vector with consumed18/requested2 and that actual Piece owner/hole index1. Maximum17
text bytes refuses its colorway with consumed16/requested2. Maximum23 entries admits Pieces, then
refuses cut_plan.copies with consumed22/requested2.

After a real split retires a held boundary source, rebirth would refuse that original Piece; accounting
still succeeds without repair, rewriting or changed ledger. Empty actual Piece/copy scopes contribute
zero entries and preserve an earlier meter refusal. Five new actual cases plus prior10 resource cases
pass:15 total, with strict selected lint/formatting and four meter privacy/sealing checks rechecked.
Whole-root/source/runtime admission is implemented; allocator RSS/time certification stays separate. The command bus has not changed its runtime behavior.

## Actual topology accounting

charge_topology_resources borrows the actual IdentityLedger. It charges the journal's complete
length before visiting entries in append order. Each Declared entry charges its held edge vector;
each Offset entry charges its fragment vector. Split, Merge, Reverse and Delete carry fixed Copy
payloads and add no nested collection entries. Their original values remain unchanged.

```rust
let mut meter = DesignResourceMeter::new(resource_limits);
charge_topology_resources(&identity_ledger, &mut meter)?;
let actual_topology = meter.finish()?;
```

After journal payloads, the producer charges the maintained live-edge set and registered-reference
map by their actual stored lengths. It does not replay the journal, recompute an index, resolve a
reference, rebuild repairs, mint an identity or rewrite a saved split choice. Exhaustive ledger/
journal/edit views and Copy index-key/value constraints require an update for new owned payloads.
The internal view's debug output contains counts, not journal values.

Whole fields are topology.journal, topology.live_edges and topology.registrations. Nested fields
are topology.journal.declared.edges and topology.journal.offset.fragments, with actual operation
owner and original zero-based journal index. Registration choices and retired-source references
remain part of the held model; accounting does not require them to resolve or certify integrity.

The native fixture performs Declare, Split, Merge, Reverse, three-fragment Offset and Delete, with
two registered references and a saved split choice. It retains6 journal entries,5 declared edges,
3 offset fragments,6 live-index entries (including an unused live edge) and2 registrations:22 logical
entries, zero text bytes and zero unbounded-counter bits. Maximum5 refuses the full journal6;
maximum10 refuses declaration6+5 at index0; maximum13 refuses offset11+3 at index4 with its actual
operation owner. Maximum19 refuses the live index14+6; maximum21 refuses registrations20+2.

Four new actual topology cases plus prior15 resource cases pass:19 total, with strict selected lint
and formatting. They retain the exact original ledger/choices on success/refusal, and empty actual
scope preserves a prior meter error. This closes Piece/copy/topology component accounting. Higher root composition, source traversal and runtime admission are implemented below; no release, physical, integrity, allocator RSS/time or authority proof follows.

## Actual input metadata accounting

The parameter names below are exact authored test data, not inferred product quantities or defaults.

| Fixture token | Meaning here |
| --- | --- |
| `a` | First parameter's one-byte machine token in the input-metadata fixture |
| `bb` | Second parameter's two-byte machine token in that same fixture |

charge_parameter_resources precharges the actual parameter vector, then charges each held machine
token's exact bytes in authored order. Target references remain fixed metadata; it performs no
canonical target lookup or numeric-availability read. Refusals retain actual parameter owner/index.

charge_recipe_document_resources borrows both retained fields: exact original source, then actual
canonical syntax text. It does not call the parser, normalize literals again, regenerate identity or
evaluate operators. Both logical strings count even when their text could coincide. An empty source
still retains the eight-byte canonical identity `(recipe)`; it contributes8 bytes, not zero.

charge_material_inventory_resources precharges the actual Material vector, then each exact name
and provided composition. None contributes no composition text and supplies no inferred composition.
Property targets and optional shrinkage are fixed Copy metadata, with no duplicate canonical values/
evidence or applied physical scale. It performs no property, source or registry lookup. Locations
retain actual Material owner and authored index. Exhaustive private owner/text/definition views and
Copy guards expose future owned-field obligations, including canonical reference-vector payloads.

```rust
let mut meter = DesignResourceMeter::new(resource_limits);
charge_parameter_resources(&parameters, &mut meter)?;
charge_recipe_document_resources(&recipe_document, &mut meter)?;
charge_material_inventory_resources(&materials, &mut meter)?;
let actual_input_metadata = meter.finish()?;
```

The native fixture has two parameters with tokens `a` and `bb` (3 bytes). The original recipe text
is24 bytes including outer whitespace; its retained canonical identity is36 bytes. Two Materials
have names of3 and4 bytes; the first provided composition ` é ` adds4 bytes and the second is None.
Combined demand is4 collection entries,74 UTF-8 bytes and zero unbounded-counter bits. Actual
Unknown/Derived width/flag/shrinkage references remain unchanged, without inferred property values.

Maximum2 text bytes refuses the second token at consumed1/requested2. Maximum23 refuses recipe.source
at0+24; maximum62 after tokens refuses recipe.canonical at27+36. Maximum6 on Materials refuses the
first composition at3+4; cumulative maximum73 refuses the second Material name at70+4. Collection
maximum3 after parameter entries refuses materials at2+2. Owners/indices distinguish member fields;
recipe strings and whole inventories have no standalone entity owner.

Five new actual cases plus prior19 resource cases pass:24 total, with strict selected lint/formatting.
They preserve canonical inputs, parameter order/targets, both recipe fields and complete Material
intent. Empty parameter/Material drafts plus empty recipe source demand0 entries/8 bytes/0 bits;
a prior meter refusal stays original across all three producers. Whole-root/source and runtime admission are implemented below; this is no binding, recipe evaluation,
Material truth/physical, whole-model or allocator RSS/time certificate.

## Actual sewing accounting

charge_sewing_graph_resources precharges the actual graph's span vector, then each span's weighted
region vector (when Weighted) and stop list in authored/schema order. Uniform and BetweenNotches
carry no region vector. Fixed side, direction and Explicit/Formula/Profile amount metadata stay
unchanged; the producer does not read a symbolic value, look up a copy/landmark, resolve an anchor,
validate current contexts or realize ease/geometry.

charge_sewing_landmark_resources charges the provided actual Notch/TurnPoint registry slice. Root
composition must supply its exact owned vector. Exhaustive private views and fixed Copy-payload
constraints cover each held piece/anchor/id/profile representation; bindings remain symbolic, with
no inferred notch style, sample/production dimensions or encoding. No landmark is reborn or resolved.

```rust
let mut meter = DesignResourceMeter::new(resource_limits);
charge_sewing_graph_resources(&sewing_graph, &mut meter)?;
charge_sewing_landmark_resources(&landmarks, &mut meter)?;
let represented_sewing = meter.finish()?;
```

The native graph has3 spans covering Uniform/Weighted/BetweenNotches and Explicit/Profile/Formula
amounts. Its weighted span holds2 regions, the spans retain1/2/2 stops, and the registry holds two
Notches plus one TurnPoint:13 collection entries, zero text bytes and zero unbounded-counter bits.
Maximum2 refuses sewing.spans (requested3, actual graph owner). Maximum3 refuses the first stop list
at3+1, span owner/index0; maximum5 refuses the weighted vector at4+2, weighted span owner/index1.
After10 graph entries, maximum12 refuses the3-entry sewing.landmarks field. Empty actual graph and
registry contribute zero and cannot clear an earlier meter refusal.

After a real deletion retires a held boundary source, the same graph/landmark accounting still succeeds
without rebirth or resolving the now-orphaned anchors. Exact graph/landmarks/ledger and profile intent
remain unchanged on success/refusal. Four new actual cases plus prior24 resource cases pass:28 total,
with strict selected lint and formatting. The first compile caught a missing public re-export, which
was corrected before these checks passed. Runtime model admission is implemented below; these counts supply no binding, realized ease, geometry, physical,
release, whole-model or allocator RSS/time certificate.

## Actual Pocket, notion and Closure accounting

charge_pocket_inventory_resources charges the actual Pocket vector, then each Pocket's component
vector in original authored order. A component is one held physical-copy/source-Piece pair; source
geometry and copy transformations are not expanded. Position, orientation and Declaration/Profile
opening bindings stay fixed metadata. Exhaustive private owner/definition views and Copy guards
expose future owned-field changes without lookup, rebirth or opening execution.

charge_notion_placement_inventory_resources charges every actual placement, including unused ones.
Its fixed original Piece/copy/anchor/direction payload stays held. charge_closure_inventory_resources
charges the actual Closure vector and each original instance vector. Fixed kind, explicit/formula/
profile zipper length, hook/bar/button size, buttonhole operation and the stored derived Count add no
collection/text. Instance placement ids remain references; no hardware or derived length is realized.

```rust
let mut meter = DesignResourceMeter::new(resource_limits);
charge_pocket_inventory_resources(&pockets, &mut meter)?;
charge_notion_placement_inventory_resources(&notions, &mut meter)?;
charge_closure_inventory_resources(&closures, &mut meter)?;
let represented_composition = meter.finish()?;
```

The native fixture has two Pockets with1/2 components,13 notions (one unused), and five Closures
with2/1/1/1/1 instances:29 collection entries, zero text bytes and zero counter bits. Its zipper
cases retain all three length origins; hook/bar and button/buttonhole retain declaration/profile
sizes and the canonical derivation operation. Authored/reflected physical copies remain distinct.

Maximum4 refuses pockets.components at3+2, actual second Pocket owner/index1. After five Pocket
entries, maximum17 refuses notions at5+13. Maximum22 refuses closures at18+5. Maximum24 refuses
closures.instances at23+2, first Closure/index0; maximum25 refuses the next instances at25+1,
second Closure/index1. Whole Pocket precharge, empty inventories and prior-error preservation are
also covered. Original objects remain unchanged on success and refusal.

A real deletion makes the held notion inventory fail current validation; the same resource walk
still succeeds without rebirth, resolution or repair and preserves the exact objects and ledger.
Four new cases plus prior28 resource cases pass:32 total, with strict selected lint/formatting.
Runtime admission is implemented below. Logical
counts supply no current target binding, opening/closure execution, selected profile value, physical
readiness, whole-model or allocator memory/time certificate.

## Actual layer and Hem accounting

charge_layer_inventory_resources visits Facing, modelled Lining and Interfacing in schema order.
It precharges each actual family vector, then each original layer's offset source vector and exact
UTF-8 unresolved Material reason. Assigned Material ids add no text and trigger no registry lookup.
Locations use layers.facings/linings/interfacings for whole vectors, layers.offset.sources and
layers.material.reason for nested fields, actual layer owner and original within-family index.

The iterator borrows actual private layer content; exhaustive inventory/wrapper/content/definition
views and Copy source guards expose owned-field changes. It does not resolve ranges, regenerate
contours, execute offsets, select Materials or bypass the Lining envelope. Modelled Lining content
counts even when require_in_scope refuses env_lining; the refusal remains unchanged.

charge_hem_inventory_resources precharges the actual Hem vector. Exhaustive held source/definition
views guard fixed edge, Explicit/Formula/Profile depth, Declaration/Profile fold and Turned/Faced
method fields. A Faced Hem keeps one Facing id: its source list and reason are charged once through
the owning layer vector. No Facing lookup, rebirth, depth resolution or folding is performed.

```rust
let mut meter = DesignResourceMeter::new(resource_limits);
charge_layer_inventory_resources(&layers, &mut meter)?;
charge_hem_inventory_resources(&hems, &mut meter)?;
let represented_finishes = meter.finish()?;
```

The native fixture has two Facings (1/2 sources), one Lining (2 sources), one Interfacing (1 source)
and three Hems:13 entries. Reasons ` é `, `猫` and `é` contribute4+3+2=9 UTF-8 bytes, without trimming;
the assigned Facing contributes none. Counter bits are zero. Hems cover explicit zero, formula and
profile depths, both fold origins, and turned/faced methods without duplicating Facing content.

Entry maxima1/4/5/7/8/9/12 retain exact first refusals0+2/3+2/5+1/6+2/8+1/9+1/10+3 across whole
families, nested sources and hems. Nested witnesses keep the actual layer and original family index.
Text maxima3/6/8 refuse0+4/4+3/7+2 at the exact Material reason owner/index. A real deletion makes
current layer validation refuse; accounting the same held layers/Hems succeeds without repair and
leaves objects/ledger unchanged. Empty collections and prior-error preservation also pass.

Five new actual cases plus prior32 resource cases pass:37 total, with strict selected lint/formatting.
Runtime admission is implemented below. These counts grant no
current source/Material binding, Lining execution, generated offset/folding geometry, physical
readiness, whole-model or allocator memory/time certificate.

## Actual construction-family accounting

charge_construction_inventory_resources borrows the actual six-family owner and precharges Dart,
Tuck, Pleat, Gather, Grainline and SeamAllowance vectors in that schema order. Each Tuck/Pleat then
charges its own fold_lines vector before traversal. Original family order and nested fold order remain
unchanged; whole fields have no entity owner, while fold refusals carry actual construction id and
original within-family index. Fields are constructions.darts/tucks/pleats/gathers/grainlines/allowances
and constructions.tucks.fold_lines or constructions.pleats.fold_lines.

Exhaustive private inventory/wrapper/content/definition views and individual Copy-payload guards
cover all held fields. The private named collection view's Debug shows counts only. Dart apex/legs,
explicit/formula/profile intake, closing operations, grain arrow/alignment/optional stripe/plaid and
allowance width/corner/inclusion stay fixed metadata. Gather retains its original graph/span/side/
copy binding; the canonical weighted ease belongs to the SewingGraph and is not recharged through
that reference. No geometric resolution, rebirth, repair, value/operation lookup or execution occurs.

```rust
let mut meter = DesignResourceMeter::new(resource_limits);
charge_construction_inventory_resources(&constructions, &mut meter)?;
charge_sewing_graph_resources(&sewing_graph, &mut meter)?;
let represented_owners = meter.finish()?;
```

The native fixture has3 Darts,2 Tucks with1/2 fold lines,1 Pleat with2 lines,1 Gather,4 Grainlines
and3 allowances:14 constructions plus5 lines=19 entries, zero text bytes/counter bits. It covers all
intake/angle/allowance-width origins, parallel alignment, optional print references and explicit zero.
Its separate graph has one span/two weighted regions; composing actual owners gives22, without
expanding the Gather's reference. This does not realize ease, angles, offsets or conserved intake.

Entry maxima2/4/5/7/8/10/11/15/18 retain exact refusals0+3/3+2/5+1/6+2/8+1/9+2/11+1/12+4/16+3.
The second Tuck's fold failure carries its actual owner/index1; the Pleat carries its owner/index0.
Empty families preserve zero demand and prior errors. A real deleted construction source makes
current Dart validation refuse while held accounting succeeds with exact inventory/graph/ledger
unchanged. Five new cases plus prior37 pass:42 resource cases, strict selected lint and formatting.
The first lint rejected the initial six-slice tuple; a named borrowed view resolved it before acceptance.
Runtime admission is implemented below; no
current binding, execution, geometry/physical, whole-model or allocator memory/time certificate follows.

## Actual structural root and revision bits

charge_design_structure_resources borrows the actual private DesignStructure and exhaustively
matches its definition. It charges the represented revision's exact magnitude bits first, then actual
canonical inputs, Pieces, cut copies and topology in schema order under the same sink. It performs
no revision formatting/advancement, model copy, source/value lookup or current validation.

The private revision view borrows the original BigUint and reads its public magnitude-bit count.
Zero contributes zero bits;1 contributes1,2/3 contribute2,u128::MAX contributes128, and its exact
successor contributes129. This is represented magnitude, not decimal digits, allocation capacity,
a maximum-issued command revision or a fixed machine-width cap. The original value remains unchanged.
The first counter refusal uses design.revision, the actual Design id and no collection index.

```rust
let mut meter = DesignResourceMeter::new(resource_limits);
charge_design_structure_resources(&structure, &mut meter)?;
let represented_structure = meter.finish()?;
```

The native nonempty root holds two canonical declarations,22 Piece entries,2 copies and15 topology
entries:41 collection entries,18 text bytes and129 revision bits. Maximum128 bits refuses0+129 before
later fields; maximum25 entries refuses cut_plan.copies at24+2. Maximum40 entries refuses the actual
live-edge index at34+7; maximum17 bytes retains the second Piece colorway refusal16+2. Each refusal
preserves the original root and exact component location rather than returning partial demand.

Prior charges3 entries/2 UTF-8 bytes/7 bits compose to44/20/136. Maximum135 bits refuses7+129 with
the original Design owner;136 admits the complete structural scope. An actual empty structural draft
has0/0/0 demand, including its zero revision, and cannot clear an earlier meter error.

Five new actual cases plus prior42 pass:47 resource cases; both existing command-context cases also
pass, with strict selected lint and formatting. Native source comparison proves the command-context
delta is only the private readonly bit getter; public successor/display/context behavior stays intact.
This is structural-root accounting; complete higher roots compose below. Runtime model enforcement is implemented. Neither the producer name nor finish supplies a physical, allocator memory/time or
command-admission certificate.

Actual measurement and external SizeSet component examples continue in
[Actual source resource accounting](source-resource-accounting.md).

## Complete actual Inputs, Sewing and Garment roots

charge_design_input_resources borrows every private DesignInputsDefinition field in schema order:
structure, parameters, retained recipe, Materials, source metadata and provided references. Actual
source metadata owns both the higher host and a separately retained vector of DesignHostEntity.
The producer first charges the host through DesignResourceSource, then precharges that exact stored
vector. It does not regenerate a census through the semantic entities callback. Fixed Copy guards
cover each captured identity record and the provided measurement/Ease/SizeSet reference carrier.

charge_design_sewing_resources charges that actual input root, the graph, then the complete retained
landmark registry. charge_design_garment_resources charges sewing, Pockets, physical notions,
Closures, layers, Hems and all six construction families, once in original schema order. Exhaustive
private wrapper/definition views make future retained fields explicit accounting obligations. None
of these producers clones, rebuilds or validates current models, reparses recipes, replays topology,
looks up numeric values or resets/finishes the caller's meter.

```rust
let mut meter = DesignResourceMeter::new(resource_limits);
charge_design_garment_resources(&actual_garment, &mut meter)?;
let represented_model = meter.finish()?;
```

The native complete model uses actual DesignSources with both external set families. Its structure
has6 canonical declarations,2 Piece entries,1 copy and3 topology entries; parameter1, actual host39,
retained metadata identities26, semantic landmark1 and an unused physical notion1 bring the total
to80 entries. Piece reason/labels11 bytes, parameter token1, retained recipe10 and source628 bring
the total to650 UTF-8 bytes. The actual revision is the successor of u128::MAX:129 magnitude bits.
The complete input/sewing roots count78/79 entries respectively; the garment root counts80.

Maximum77 entries refuses sources.entities at52+26, with Inputs scope, actual containing Design id
and no collection index. This catalog owns no new entity id. Maximum78 refuses sewing.landmarks
at78+1; maximum79 refuses notions at79+1. Maximum649 bytes preserves the original source chart's
saved measurement-token refusal641+9, chart id and entry index1. Maximum128 bits preserves the
initial design.revision refusal0+129. None publishes partial demand or changes the original model.
Prior2 entries/2 bytes/7 bits compose to82/652/136 under the same meter, and a prior refusal survives.

After a real edge deletion, current validation refuses the held landmark/notion source. Accounting
still preserves the exact held model and counts80/650/129: the journal gains one entry and live-edge
index loses one. An explicit empty complete draft counts0 entries/8 bytes/0 bits because its retained
canonical recipe is (recipe); a7-byte limit refuses recipe.canonical at0+8. Empty authored source
does not imply empty canonical syntax. Five new actual cases and prior28 source cases pass33, with
strict core-library/source-test lint and formatting. A wrong expected static token-field spelling was
corrected against the actual chart producer before acceptance. Python/OS byte counts independently
confirm the additional11+10+1 bytes; the retained26-record census is read from the actual owner.

This completes implemented model accounting under the source port's honest/exhaustive integration
contract. [Installed current-model admission](command-operation-control.md#installed-current-model-admission)
now follows exact authority/snapshot binding. [Exact prospective Measurement accounting](command-operation-control.md#exact-prospective-measurement-resources)
borrows a replacement in its actual slot before candidate copies. [Selected Material accounting](command-operation-control.md#exact-prospective-selected-material-resources)
borrows resulting Piece reasons before copies. [Notch append accounting](command-operation-control.md#exact-prospective-notch-resources)
charges the actual new registry before cursor consumption/copy. [Publication revision admission](command-operation-control.md#prospective-publication-revision-resources)
uses the independent maximum-issued counter before successor construction. [Retained command
composition](command-resource-accounting.md#actual-current-private-retained-composition) and
[prospective publication](command-retained-publication.md) include actual journal/history content.
The coupled native resource checks above pass. Raw ingress, allocation peaks and host algorithm time
retain separate boundaries; logical demand supplies no dishonest-host security, readiness, physical
or release certificate. Broader command/deeper/full verification remains separately owned.
