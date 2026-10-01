# The supported envelope

> **Status:** normative specification, gate **G0** (roadmap §3.2, §1.3, §11). This chapter bounds the v1
> release claim: what StitchCAD supports, what it refuses with a diagnostic, and what it defers to a named
> later gate. Every term used here is defined in the [glossary](glossary.md); gate **G7** turns this
> chapter into the public supported-envelope statement.

A release claim with no boundary is a claim a factory disproves. This chapter is the boundary, and it is a
table rather than prose for one reason: a row can be checked, and a sentence cannot. Every construction
the product models, every non-goal the roadmap refuses, and every capability a later gate owns appears
here with a **disposition**, a **reason**, the **gate that proves it**, and — where the disposition is not
`supported` — the **diagnostic** the product must produce instead of an approximation.

## 1. The three dispositions

| Disposition | Meaning | What the product must do |
| --- | --- | --- |
| `supported` | inside the v1 envelope; a named gate proves it with a test and a fixture | produce it, and fail loudly if it cannot |
| `rejected` | outside the envelope **by decision** — a non-goal, or a construction v1 refuses | produce the row's diagnostic; never approximate, never silently substitute |
| `deferred` | intended, but owned by a gate or track after the v1 2D release | produce the row's diagnostic naming the gate that owns it |

Three rules govern the table, and they are requirements, not descriptions:

1. **No silent approximation.** A construction outside the envelope SHALL produce its diagnostic. A
   plausible substitute — a knit block drafted as woven, a NURBS curve flattened without saying so, a
   pocket drawn as internal lines — is a defect, because the receiver cannot see the substitution.
2. **A `supported` row names its proof.** "Supported" means a gate's exit criteria include a test over a
   fixture. Where the roadmap names no such gate, the row says `unnamed (D32)` rather than borrowing a
   gate that never agreed to it, and where this chapter asks the roadmap to carry a gate it does not carry
   yet, the cell says `(proposed)` with §9 naming the amendment. Neither shape is a commitment, and the
   census prints both counts on every run: §9 records the five rows that were unnamed and the revision that
   closed them.
3. **Modelled is not the same as supported.** The ontology specifies objects whose v1 proof nobody has
   scheduled. Such a row is `deferred` with gate **G7**, because G7's exit is a supported-envelope
   statement *with named limitations* — that is where an unproved capability is declared, not hidden.

A citation in the reason column is explicit about its source — `ontology §4.3` or `roadmap §3.2` — because
the two documents have overlapping clause numbers and a bare `§4.3` would be a guess. The census in §13
uses those citations to prove the coverage claim in both directions.

## 2. The garment family

Roadmap §3.2 declares one family: woven garments with documented seam operations. Everything in this
chapter is inside that family or explicitly outside it.

| Feature | Disposition | Why | Proven at | Diagnostic |
| --- | --- | --- | --- | --- |
| A-line skirt, waist dart, CB zipper | supported | the [reference fixture](reference-skirt.md) **is** this garment, specified to the millimetre | G2 drafting, export and print; G3 closures | — |
| darted bodice | supported | roadmap §3.2 and the G3 exit criteria both name it | G3 | — |
| set-in sleeve with declared cap ease | supported | roadmap §3.2; G3's exit requires ease to be intentional, not an invariant violation | G3 | — |
| classic collar | supported | roadmap §3.2 names it, and §11 G3's envelope-coverage criterion proves it at `G3-GRADING.15` | G3 | — |
| trousers | supported | roadmap §3.2 names it, and §11 G3's envelope-coverage criterion proves it at `G3-GRADING.5` | G3 | — |
| woven fabrics, stable | supported | roadmap §3.2 bounds the envelope to woven garments; the fixture's material is a stable woven | G2 | — |
| knit or stretch blocks | rejected | roadmap §3.2 names it as unsupported: stretch changes what ease means | G2 | `env_knit_stretch` |
| leather and non-textile sheets | rejected | roadmap §3.2 names it; the offset and allowance model assumes textile behaviour | G2 | `env_material_leather` |
| fully bespoke structures | rejected | roadmap §3.2 names it; a structure with no recipe has no parametric content | G1 | `env_bespoke_structure` |
| a garment outside the declared family | rejected | roadmap §3.2: the envelope is a family, not "anything sewn" | G2 | `env_garment_family` |

## 3. Drafting, recipe and geometry

| Feature | Disposition | Why | Proven at | Diagnostic |
| --- | --- | --- | --- | --- |
| construction recipe as the authoring model | supported | ontology §3.1 and roadmap ADR-0003: geometry is derived, never authoritative | G1 evaluation, G2 replay | — |
| formula language v1 | supported | roadmap ADR-0003 requires it specified at G0 and implemented at G1 | G1 | — |
| stable identity and topological references | supported | ontology §1, ontology §1.1, ontology §10: point and whole-range references survive edits or expose repairs | G3 | — |
| canonical project format, byte-identical saves | supported | ontology §7: derived data is never authoritative and two saves of one state are identical | G1 persistence, G2 save and load | — |
| NURBS-class curves and expressions | deferred | roadmap §4.2 fixes the curve set as line, arc and cubic Bézier — "the AAMA/ASTM diet" | G7 or later | `env_nurbs` |
| geometric sketch constraints (`sc-sketch`) | deferred | roadmap §4.3 marks it OPTIONAL and ADR-0001, not v1, settles its solver | G7 or later | `env_sketch_constraints` |
| slash and spread | supported | ontology §3.2: a drafting operation on the recipe, not a property of a closure | G3 | — |
| mirror and mirrored pairs | supported | ontology §4.1; the fixture's back pieces are a mirrored pair with L/R labels | G2 | — |
| `walk` and `true` as editing operations | supported | ontology §3.2 makes them first-class operations, not validation checks | G3 | — |
| imported geometry without history | supported | ontology §3.1 stores it as explicit primitives, always distinguishable from drafted | G6 import | — |
| robust predicates and exact topology | supported | roadmap §4.2; with integer coordinates there is no epsilon to hide behind | G2 | — |
| offsets within a declared error budget | supported | roadmap §4.2 makes the offset engine a first-class risk with a budget | G2 pathology corpus | — |
| an offset that cannot meet its budget | rejected | roadmap §4.2: exports fail explicitly rather than emit self-intersecting geometry | G2 | `geom_offset_budget` |
| piece holes and internal construction lines | supported | ontology §4.1 requires them, oriented opposite to the boundary | G2 | — |
| cut on fold | supported | ontology §4.1; the fixture's front piece is cut on the CF fold with no allowance there | G2 | — |
| multiplicity, cut quantity and label data | supported | ontology §4.1, ontology §10: labels derive from the plan; physical copies have explicit identities | G1 structure, G2 labels, G5 piece manager | — |
| layer index for 3D ordering | deferred | ontology §4.1 stores it, but assembly is the V1 track's proof | V1 | `env_layer_index_3d` |

## 4. Seams, allowances and marks

| Feature | Disposition | Why | Proven at | Diagnostic |
| --- | --- | --- | --- | --- |
| seam spans, partial and one-to-many | supported | ontology §4.2, ontology §10: copy-addressed partial spans and explicit same-copy rule landed | G3 | — |
| the sewing graph as a first-class object | supported | ontology §4.2, ontology §10: immutable graph addresses physical-copy identities | G2 fixture, G3 | — |
| declared ease distribution along a span | supported | ontology §4.2, ontology §10: explicit distribution intent landed; G3 checks actual cap ease | G3 | — |
| seam allowance per edge, variable widths | supported | ontology §4.4/§10; descriptors implemented at G1; actual variable offsets at G2 | G2 | — |
| the five corner treatments | supported | ontology §4.4 names miter, slant, envelope, trim and step as the vocabulary | G2 | — |
| allowance included in contour, or generated downstream | supported | ontology §4.4/§10; symbolic inclusion binding implemented; target value resolution at G4 | G4 policy, G6 receivers | — |
| notch: single, double, drill | supported | roadmap §15 item 12 makes these three the v1 typed set; ontology §4.5, ontology §10: symbolic semantic anchors landed; no geometry defaults | G3 | — |
| notch: V, I, T, U, castle | deferred | roadmap §15 item 12 stages them to G4 export | G4 | `env_notch_type` |
| notch encoding: coded point or drawn geometry | supported | ontology §4.5, ontology §10: encoding binding is symbolic; both forms remain export targets | G4 | — |
| grainline, directed | supported | ontology §4.6/§10; directed intent implemented at G1, physical parallelism remains G2 | G2 | — |
| bias and off-grain placement | supported | ontology §4.6/§10; explicit or symbolic angle intent implemented; physical bias case at G3 | G3 | — |
| dual grain reference for a stripe or plaid | supported | ontology §4.6/§10; independent stripe/plaid references implemented, placement proof at G3 | G3 | — |
| nap and directional-print layout | deferred | ontology §6 stores the flag; layout consequences belong with the marker work this product refuses | G7 or later | `env_nap_layout` |

## 5. Closures, finishes and notions

| Feature | Disposition | Why | Proven at | Diagnostic |
| --- | --- | --- | --- | --- |
| dart with conserved intake | supported | ontology §4.3/§10; structural dart intent implemented, physical 4.0 cm closure remains G2/G3 | G2, G3 | — |
| tuck and pleat | supported | ontology §4.3/§10; distinct structural intent implemented; executed folds/conservation remain G3 | G3 | — |
| gather | supported | ontology §4.3/§10; structural binding borrows span ease; G3 proves realized gathering | G3 | — |
| hem, turned or faced | supported | ontology §4.7/§10; structural depth/fold/Facing intent implemented; G2 proves the fixture hem | G2 | — |
| facing | supported | ontology §4.7/§10; structural facing intent implemented; physical offset/drafting proof remains G3 | G3 | — |
| interfacing | supported | ontology §4.7/§10; structural interfacing intent implemented; physical fixture offset remains G2 | G2 | — |
| lining | deferred | ontology §4.7/§10; modelled lining implemented; execution refuses env_lining; G7 owns limitation | G7 | `env_lining` |
| zipper, centred | supported | ontology §4.7/§10; structural instances/size origins implemented; G2 proves the 18.0 cm fixture zip | G2 | — |
| hook and bar | supported | ontology §4.7/§10; structural instances/size origins implemented; G2 proves the fixture closure | G2 | — |
| button and buttonhole | supported | ontology §4.7/§10; canonical buttonhole source implemented; G3 proves physical derivation, G5 notions | G3 derivation; G5 notions | — |
| fly construction | deferred | ontology §4.7/§10; execution refuses env_fly with closure/trousers-gap context; G7 owns limitation | G7 | `env_fly` |
| pocket | supported | ontology §4.7 models position, orientation, opening type and composition; §11 G3's coverage criterion requires the trousers to carry one | G3 | — |
| notions matched to geometry | supported | roadmap §7.5 puts notions in the tech pack; roadmap §9 makes completeness a release check | G5, G7 | — |

## 6. Measurements, sizes and instantiation

| Feature | Disposition | Why | Proven at | Diagnostic |
| --- | --- | --- | --- | --- |
| body measurement with landmark and procedure | supported | ontology §2.1 rejects a measurement without both: it could be evidence for nothing | G1 | — |
| garment point of measure | supported | ontology §2.1 keeps a POM a distinct kind from a body measurement | G1 | — |
| ease as a first-class body-to-garment mapping | supported | ontology §2.2; without it the two instantiation paths cannot be reconciled | G3 | — |
| fit intent as an ordered vocabulary | supported | ontology §2.2: `close`, `semi` and `loose` can be compared, filtered and validated | G3 | — |
| negative ease | rejected | ontology §2.2 allows it only where declared, and no v1 envelope garment declares it | G3 | `env_negative_ease` |
| size set: labels, order, base size | supported | ontology §2.3, [size sets §2](size-sets.md) and roadmap §3.4 make it a first-class object | G1 | — |
| measurement-driven regeneration | supported | roadmap §3.3 path 1 and [instantiation paths §2](instantiation-paths.md) — MTM-ready | G3 | — |
| grade-rule instantiation, `.rul` | supported | roadmap §3.3 path 2 and [instantiation paths §3](instantiation-paths.md) | G3 | — |
| incremental and cumulative rule tables | supported | roadmap §3.3 and [instantiation paths §3](instantiation-paths.md), with all three attributes | G3 | — |
| both paths, with the divergence declared | supported | roadmap §3.3: independently drafted sizes are not exactly reconstructible, and the loss is stated | G3 | — |
| extreme sizes checked after reconstruction | supported | roadmap §3.3 and [instantiation paths §7](instantiation-paths.md): on the target system's output | G3 | — |
| EN 13402 and ASTM D5585 mappings | supported | roadmap §3.4 and [size sets §8](size-sets.md); the standards' content is verified by `G0-CONTRACT.7` | G3 | — |
| multi-dimensional and custom charts | supported | roadmap §3.4 and [size sets §7](size-sets.md); a custom chart is a size system of one's own | G3 | — |
| the five uncertainty states on every parameter | supported | ontology §5: a value with no state has no provenance and cannot be exported honestly | G1 model, G4 dashboard | — |
| body-scan MTM | rejected | roadmap §1.3 makes it a v1 non-goal, and roadmap §4.5 keeps scans out of the repository | G7 or later | `env_body_scan` |

## 7. Materials, profiles, artifacts and platforms

| Feature | Disposition | Why | Proven at | Diagnostic |
| --- | --- | --- | --- | --- |
| material with width, nap, pattern and shrinkage | supported | ontology §6: the properties the product actually uses, not a name and a colour | G3 | — |
| shrinkage as an explicit transformation | supported | ontology §6: applied at a declared stage, never folded silently into a coordinate | G4 | — |
| Factory Profiles, versioned and typed | supported | roadmap §8: a profile is what turns a design into one factory's bytes | G4 | — |
| CSP resolution of profile parameters | supported | roadmap §6.1: a deterministic finite-domain solver, native and WASM | G4 | — |
| evidence scoped per claim | supported | roadmap §8: never profile-wide, and superseded evidence is preserved | G4 | — |
| the artifact policy matrix | supported | roadmap §8.2 decides what an unknown blocks, per artifact | G4 | — |
| release package, manifest and scoped approval | supported | roadmap §9: approval binds to package identity and stale-ifies | G4, G7 | — |
| agent authority levels, approval human-only | supported | roadmap §7.8: authorization is enforced in core, not in tool descriptions | G1 onward | — |
| DXF, AAMA named layers | supported | roadmap ADR-0004: a named, separately validated mode | G2 one mode, G6 receivers | — |
| DXF, ASTM numbered layers with SST, PST and BLOCKs | supported | roadmap ADR-0004 requires the metadata blocks, not just the geometry | G2, G6 | — |
| DXF R12 and R13 targets, polyline-only | supported | roadmap ADR-0004: many importers accept nothing else | G2 | — |
| cut-as-1 and sew-as-1 as separate modes | supported | roadmap ADR-0004: the swap is a profile mapping, never an inversion of meaning | G4 | — |
| grading modes: base and rules, embedded, all-contours | supported | roadmap ADR-0004 keeps them separate and validated receiver by receiver | G3 | — |
| a single universal interchange package | rejected | roadmap ADR-0004's disposition log refuses it: modes are separately validated | G2 | `env_universal_package` |
| HPGL/PLT with pen mapping and plotter scale | supported | roadmap §7.5; a known-good real-plotter fixture enters `conformance/` | G3 writer, G4 fixture | — |
| PDF, A0 and tiled A3/A4 with a scale square | supported | roadmap §7.5: a piece that does not fit is tiled or errors, never rescaled | G2 | — |
| SVG output | rejected | roadmap §7.5 demotes it: promoted only when a named consumer exists | G7 or later | `env_svg_output` |
| DXF import with an honest loss report | supported | roadmap §7.5: preserved, approximated, omitted or unsupported | G6 | — |
| tech pack generation | supported | roadmap §7.5 generates it from canonical structured content | G5 | — |
| native shell for Windows, macOS and Linux | supported | roadmap §7.3 and ADR-0002: Tauri chrome over the Rust core | G5 | — |
| browser shell over WASM | supported | roadmap §7.3: a documented capability subset, not a second product | G5 | — |
| headless CLI with deterministic replay | supported | roadmap §4.4; G2's exit is the fixture drafted through the CLI | G2 | — |
| MCP façade, stdio only | supported | roadmap §7.8: stdio until a documented threat model permits more | G1 onward | — |
| egui or iced as the product UI | rejected | roadmap ADR-0002 rules them a dev shell for engine stages only | G5 | `env_dev_shell_ui` |
| domain logic in a TypeScript front-end | rejected | roadmap ADR-0002 bans it: one command bus, three adapters | G1 | `env_ts_domain_logic` |
| assembly visualization: mesh, stitching, arrangement | deferred | roadmap §7.7 and the V1 track, which never blocks a G gate | V1 | `env_assembly_3d` |
| cloth simulation and drape preview | deferred | roadmap §7.7 and the V2 track: uncapped, out of the default build | V2 | `env_simulation` |
| photorealistic or quantitatively validated drape | rejected | roadmap §1.3 and §7.7: no fit claim before physical evidence exists | V2 | `env_fit_claim` |
| RTL locales | supported | roadmap §7.6: mirrored layout, never mirrored geometry | G5 | — |
| a first non-English language pack | deferred | roadmap §7.6 stages shipping translation to G5 with review thresholds | G5 | `env_language_pack` |

## 8. Non-goals (roadmap §1.3)

Each row is a refusal the roadmap recorded so a partner cannot reopen it. The reason column cites the
roadmap's own list; the diagnostic is what a user or an agent receives instead of silence.

| Feature | Disposition | Why | Proven at | Diagnostic |
| --- | --- | --- | --- | --- |
| marker making, nesting, yield, consumption | rejected | roadmap §1.3: factories do this after accepting our file | G2 | `ngo_marker_nesting` |
| costing | rejected | roadmap §1.3 | G2 | `ngo_costing` |
| paper-pattern digitization, raster tracing | rejected | roadmap §1.3; ontology §3.1 keeps an imported contour data, never a recipe | G6 | `ngo_digitizing` |
| direct cutter and CAM drivers | rejected | roadmap §1.3; the product writes files a factory's own system drives | G2 | `ngo_cam_driver` |
| PLM and ERP integration | rejected | roadmap §1.3, and roadmap §1.2 names PLM administrators a later persona | G7 or later | `ngo_plm_erp` |
| spreading and colorway management | rejected | roadmap §1.3 | G7 or later | `ngo_spreading_colorway` |
| print and artwork placement | rejected | roadmap §1.3; a stripe reference constrains placement, artwork is not geometry | G7 or later | `ngo_print_placement` |

## 9. Rows the roadmap's gates did not schedule — and the revision that scheduled them

Rule 2 of §1 forbids borrowing a gate, so five rows carried `unnamed (D32)`: a feature inside the envelope
that no gate's exit criteria proved. Defect **D32** recorded the gap, the director ruled on `2026-09-30` that
the engineer decides it, and the decision was prepared as an exact **proposal** because amending
`ROADMAP.md` was reserved to him
(`docs/decisions/decision_director-ruling-2026-09-30-four-findings.md`). He then delegated the three findings
outright, so the proposal is **applied**: roadmap **v0.3** gives §11 G3 an *envelope coverage* exit criterion
— every garment §3.2 names drafts, grades and exports at that gate or an earlier one — logged in the
roadmap's Appendix A with its source, and the four cells below are committed gates rather than proposals.
The census's A3 advisory still prints any cell marked `(proposed)`, and prints none today.

| Feature | Named by | Proving gate |
| --- | --- | --- |
| classic collar | roadmap §3.2's envelope list | G3 — `G3-GRADING.15` |
| trousers | roadmap §3.2's envelope list | G3 — `G3-GRADING.5` |
| button and buttonhole | ontology §4.7's `Closure` | G3 derivation + G5 notions |
| pocket | ontology §4.7's `Pocket` | G3 — `G3-GRADING.5` |
| fly construction | follows trousers | G7 — already covered |

Why each lands where it does:

- **classic collar → G3.** A stand, a fall and a roll line are construction geometry, and G3 is the gate
  that proves construction; its bodice already supplies the neckline a collar attaches to.
- **trousers → G3.** G3's complexity note used to contemplate a trousers intermediate; v0.3's coverage
  criterion requires one, because a crotch curve, an inseam and a waistband stress the drafting and grading
  engine differently from a skirt, which is the point of the gate.
- **button and buttonhole → G3, with G5 for the notions.** The rule that matters is the derivation — a
  buttonhole's length is computed from its button and never entered twice (ontology §4.7) — and a
  derivation is construction semantics. G5's exit criteria already require a tech pack whose notions list
  is complete enough for a factory quote, so the second half needs no amendment.
- **pocket → G3.** Position, orientation, opening type and piece composition are construction, and the
  trousers above are the envelope garment that carries one.
- **fly construction → G7, and no amendment is needed.** v1 refuses a fly with `env_fly`, and rule 3 of §1
  puts a modelled-but-unproved capability in the gate whose exit is a supported-envelope statement *with
  named limitations* — exactly what the matrix's lining row already does.

The rule the amendment rests on is that **permission is not a criterion**. A note saying an intermediate
garment "may be inserted without shame" scheduled nothing, and an envelope feature no exit criterion named
was a feature G7 would have had to declare untested. So v0.3 closes the *class* rather than the four
instances: every garment roadmap §3.2 names drafts, grades and exports at the gate that owns construction,
and a garment the envelope names with no exit criterion proving it is a gate failure, not a scope note.

## 10. The diagnostic contract

Every `rejected` and `deferred` row names a token. The tokens are declared here with the arguments each
diagnostic must carry, because an undeclared token has no defined meaning — the rule
`docs/decisions/decision_machine-tokens-declared-where-used.md` records. Code allocation, localization and
the typed-argument contract belong to the [internationalization chapter](i18n-architecture.md) (§3 for
identity and arguments, §10 for the inventory) and the [command layer](command-layer.md) (§2 for the
command shape, §9 for its diagnostics); what is fixed
here is each diagnostic's identity and what it must tell the receiver.

Three properties hold for all of them: a diagnostic is **structured** (a stable token plus typed
arguments, never prose an agent must parse); it names **what was asked for** and **what to do instead**;
and it is **never accompanied by geometry** — a refused construction produces no partial output.

| Token | Raised when | Required arguments |
| --- | --- | --- |
| `env_knit_stretch` | a knit or stretch block is drafted as if it were woven | material, piece, the ease the request implies |
| `env_material_leather` | a non-textile sheet material is assigned to a piece | material, piece |
| `env_bespoke_structure` | a structure with no construction recipe is requested | the structure asked for, the nearest supported family |
| `env_garment_family` | a garment outside the declared woven family is requested | requested family, the supported family list |
| `env_nurbs` | a NURBS-class curve or expression is requested | the curve kind asked for, the supported curve set |
| `env_sketch_constraints` | a geometric sketch constraint is requested | the constraint kind, the recipe alternative |
| `env_layer_index_3d` | layer ordering is asked to produce a three-dimensional result | the pieces involved, the V1 track that owns it |
| `geom_offset_budget` | an offset cannot be produced inside the declared error bound | edge, requested width, achieved deviation, the bound |
| `env_notch_type` | a notch type staged past v1 is requested | requested type, the v1 set, the gate that owns it |
| `env_nap_layout` | nap or directional-print layout is requested | material, the pieces affected |
| `env_lining` | a lined garment is requested | the pieces a lining would serve, the gate that owns it |
| `env_negative_ease` | negative ease is requested where nothing declares it | the POM, the ease value, the declaration it lacks |
| `env_fly` | a fly construction is requested | the closure asked for, the trousers gap it depends on |
| `env_body_scan` | a body scan is offered as a measurement source | the scan, the measurement procedure it would replace |
| `env_universal_package` | one artifact is asked to serve every receiver | the receivers named, the modes that exist instead |
| `env_svg_output` | SVG output is requested | the consumer asking, the supported artifact set |
| `env_dev_shell_ui` | the dev shell is asked to be a product UI | the shell, the product shell that owns it |
| `env_ts_domain_logic` | domain logic is placed in a TypeScript front-end | the file, the command that owns the behaviour |
| `env_assembly_3d` | a three-dimensional assembly or stitching result is requested | the sewing graph read, the V1 track that owns it |
| `env_simulation` | cloth simulation or drape is requested | the observable asked for, the V2 track that owns it |
| `env_fit_claim` | a fit, pressure or strain claim is requested | the claim, the physical evidence it lacks |
| `env_language_pack` | a language with no reviewed pack is requested | the locale, the review threshold it has not met |
| `ngo_marker_nesting` | marker making, nesting, yield or consumption is requested | the pieces, the downstream step that owns it |
| `ngo_costing` | costing is requested | the inputs offered, the refusal |
| `ngo_digitizing` | paper-pattern digitization or raster tracing is requested | the source image, the recipe alternative |
| `ngo_cam_driver` | a cutter or CAM driver is requested | the machine named, the artifact the factory drives it with |
| `ngo_plm_erp` | PLM or ERP integration is requested | the system named, the persona that owns it later |
| `ngo_spreading_colorway` | spreading or colorway management is requested | the request, the refusal |
| `ngo_print_placement` | print or artwork placement is requested | the artwork, the stripe reference that is supported |

## 11. What the envelope statement at G7 must carry

Gate G7's exit is "a supported-envelope statement with named limitations". This chapter is its input, and
the statement is a projection of it rather than a rewrite:

- the supported garment family, with the fixture that proves each member;
- every `supported` row whose `Proven at` gate has actually passed, every row still `unnamed (D32)`, and
  every row whose gate is a *proposal* §9 records, because an unratified commitment is a limitation too;
- every `deferred` row with the gate or track that owns it, so a limitation has an owner and not a date;
- every `rejected` row, because a refusal a partner does not know about is a refusal they will test;
- the diagnostic tokens, so a receiver's rejection email can be matched to a row of this table.

## 12. Verification status of the dispositions

- **Every `rejected` row in §8** — *cited from the roadmap*: §1.3 is the authoritative non-goal list and
  each row quotes it. The census derives this direction (rule M4), so a non-goal cannot go unlisted.
- **The garment family in §2** — *cited from the roadmap*: §3.2's parenthetical, plus the three
  constructions §3.2 names as unsupported. The census derives this too (rule M5).
- **The staged notch set, the curve set, the SVG demotion, the dev-shell ruling and the TypeScript ban** —
  *cited from the roadmap* at §15 item 12, §4.2, §7.5 and ADR-0002 respectively.
- **Every `Proven at` gate** — *derived from that gate's own exit criteria* in roadmap §11, not assumed.
  Where no exit criterion mentions the feature, the cell either says `unnamed (D32)` — the rule §1 states,
  and no row says it today — or carries `(proposed)`, with §9 naming the amendment that would cover it. A
  proposed cell is a claim about a *draft* of the roadmap, never about the roadmap as it stands.
- **The diagnostic tokens and their arguments** — *a project decision*, not an external claim. No
  receiver's behaviour is asserted here; the tokens name what this product says when it refuses.
- **Collar, trouser, button and pocket construction** — *ratified v1 commitments as of roadmap v0.3*, and
  unproven until G3 runs: §11 G3's envelope-coverage criterion requires each of them, `G3-GRADING.5` and
  `.15` own the garments, and `.14` fails the gate review if a §3.2 garment has no leaf's evidence behind it.
  Before v0.3 these four rows were the gap D32 recorded. The **fly** needs no ruling: it is `deferred`,
  refused with `env_fly`, and G7's existing exit criteria already require the limitation to be named.

## 13. How the coverage claim is derived

"Nothing in the ontology is silently unlisted" quantifies over three populations — the ontology's clauses,
the roadmap's non-goals and the roadmap's envelope — so it is derived, not asserted:

```bash
bash docs/tasks/artifacts/feature_matrix/run_feature_matrix_census.sh
# → feature-matrix census: <rows> rows / <diagnostics> diagnostics / 0 failure(s)
```

The census enforces the shape (five cells, a disposition from the closed vocabulary, a reason and a gate
on every row); the diagnostic contract (every `rejected` or `deferred` row names a token §10 declares, and
every declared token is used by a row); the ontology coverage (every clause of the ontology chapter that
specifies an object is cited by at least one row); the non-goal coverage (every roadmap §1.3 bullet
appears in a `rejected` row); the envelope coverage (every garment roadmap §3.2 names appears in a
`supported` row); and the honesty of the gate column (a `Proven at` cell is a real gate id, a track, or
`unnamed (D32)` — never blank, never prose).

What it cannot judge is whether a disposition is *right*: whether a collar belongs in v1 is a product
decision, and the census only proves the decision was written down.

## 14. What must be true in tests

- **Every refusal is a diagnostic, not a substitution:** requesting each `rejected` row's construction
  produces its token with its required arguments and **no geometry** — the mutation suite asserts the
  absence of output, not merely the presence of a message.
- **Every deferral names its owner:** a `deferred` refusal names the gate or track, so an agent can tell a
  user what to wait for.
- **The envelope is closed under the ontology:** adding an object clause to the ontology without a matrix
  row citing it fails the census, not a reviewer's memory.
- **A supported row cannot quietly lose its gate:** the census refuses a blank or prose `Proven at` cell.
- **Neither a gap nor a proposal can be forgotten:** the census reports the count of `unnamed (D32)` rows
  (advisory A1) and the count of cells marked `(proposed)` (advisory A3) on every run. Both are zero today,
  because roadmap v0.3 carries the criterion §9 proposed — and both advisories stay in the census, since the
  next unproved envelope feature must surface the same way rather than being discovered at G7.
