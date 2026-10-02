# SizeSet is referenced by the Design and overridable per Factory Profile, with a recorded transformation

- **Type:** `decision`
- **Date:** `2026-09-30` (absolute)
- **Status:** `active`
- **Owner / source:** leaf `G0-CONTRACT.6`, resolving the question roadmap §3.4 leaves open at G0 and
  ontology §2.3 refers here ("Ownership … is decided in the size-sets chapter")

answers: "who owns a size set?" · "can a factory change my sizes?" · "where do size-run quantities live?" · "why is a size set referenced rather than embedded in a design?" · "what must a profile record to override sizes?" · "how does made-to-measure fit a size set?" · "can label order choose the base?" · "can a size revision wrap?" · "may chart and Design share an authored base input?"

## The fact / decision

1. **The `SizeSet` is a first-class object with its own identity and revision.** It is neither a list of
   strings inside a `Design` nor a bundle of parameters inside a Factory Profile.
2. **A `Design` references a size set; it does not contain one.** The reference is by entity id plus the
   revision it was authored against, so re-opening a design pins the size set it was built with (the same
   rule roadmap §4.5 states for profile versions).
3. **A Factory Profile may override the size set, and an override is a recorded transformation.** The
   override produces a *resolved* size set — a new object with provenance naming the design's reference,
   the profile's revision, the transformation applied and its evidence. It never mutates the design's
   reference, and it never silently re-labels a size.
4. **Size-run quantities are not part of a size set.** How many of each size to cut is order data, and
   roadmap §7.5 is explicit that size-run quantities belong to an Order object rather than to the reusable
   design. v1 does not model an Order object; a size set therefore carries no quantities, and a tech pack
   that needs them says so as an unresolved input rather than inventing a ratio.
5. **The base size belongs to the size set**, is exactly one of its members, and is the size the design was
   drafted in and the anchor path 2 grades from.
6. **Made-to-measure is a size set of one.** A custom chart with a single member, its own base size and its
   measurements taken from a body rather than from a standard, is the same object with different
   provenance — not a separate mechanism.

## Context

Three candidate owners were on the table, and each has a real argument:

- **The Design.** Sizes are a design decision: a graded range is part of what a patternmaker authors, and
  the recipe must be evaluable for every member. Headless use — CLI, agent, WASM viewer — needs a sized
  design with no factory in the loop at all.
- **The Factory Profile.** Factories have house charts, and two factories receiving the same design often
  want different labels, different breaks or a different base. Roadmap §8 makes the profile the place
  where "the design meets one cutting room".
- **A third Order object.** Quantities, delivery and a customer's chart are order-scoped, and roadmap §7.5
  already puts quantities there.

The tension is real: a size set owned only by the design cannot serve a factory's house chart, and one
owned only by the profile makes a design unsized until a factory is chosen — which breaks the headless
promise, the V1 assembly view and the reference fixture, none of which have a factory.

## Why this resolution

- **Referencing keeps both properties.** The design is sized and evaluable on its own (the fixture is), and
  a profile can still resolve it differently per factory. Containment is by reference, so two designs can
  share one size set and a size set can be revised without rewriting designs.
- **An override that produces a new resolved object preserves evidence.** Roadmap §8's whole premise is
  that a parameter's provenance is part of its value: if a profile re-labelled sizes in place, a release
  package could not say whose sizes it cut. A resolved size set names the design's revision, the profile's
  revision and the transformation, so a factory dispute has a document.
- **Precedence is already decided and this follows it.** Roadmap §8 composes hard restriction > factory
  override > preference > default. A size-set override is a factory override, so it loses to a hard
  restriction (for example a safety limit on how far a size may be scaled) and it must be visible in the
  conflict report rather than merged silently.
- **Excluding quantities keeps the design reusable.** A ratio baked into a design would make every size-run
  change a design revision, stale-ifying approvals for a commercial decision that changed no geometry
  (roadmap §9's approval binding).

## Consequences

- **For the model:** `SizeSet` gains identity, revision, member labels, an explicit order, one base size, a
  size-system enumeration, per-size charts, per-dimension breaks, and multi-dimensional axes. A `Design`
  holds a reference, not a list. A resolved size set carries a transformation record.
- **For profiles:** an override is a typed parameter with a domain (label mapping, break scaling, base-size
  substitution, chart replacement) and it carries state and evidence like any other parameter — so an
  `unknown` chart blocks what the artifact policy matrix says it blocks, and a `preference` chart is
  exportable and marked as one.
- **For the two instantiation paths:** path 1 (regeneration) consumes a member's measurements, which come
  from the chart or from a body; path 2 (grade rules) consumes the base size and the breaks. A profile
  override that changes breaks therefore changes path 2's output *and* the equivalence report, which is why
  the report names the resolved size set it compared.
- **For interchange:** a `.rul` table is meaningless without the breaks and the base size it was built
  against, so the artifact records the resolved size set's identity and revision in the manifest.
- **For agents:** relabelling a size is a mutation of a resolved size set, not of a string; an agent with
  `propose` authority can produce the transformation, and only a human approval makes it a release input.
- **Re-open condition:** if a customer-specific chart workflow becomes a v1 requirement (an Order object
  with per-order charts and quantities), the Order becomes a *third* referencer of size sets — it does not
  take ownership away from the Design, because the recipe must still be evaluable headlessly.

Related: [[decision_numerical-contract-fixed-point]] · [[decision_machine-tokens-declared-where-used]] ·
`docs/book/src/spec/size-sets.md` (the normative chapter) · `docs/book/src/spec/instantiation-paths.md`
§2–§3 (what each path consumes) · `docs/tasks/G0-CONTRACT.md` leaf `.6`.

## Membership foundation — G1-SLICE.4c.1

Stable member identities and exact nonblank human labels are distinct from authored vector order.
The base names one existing member identity; labels have no numeric, machine-token or quantity
semantics. Empty/duplicate membership or absent base refuses. A custom member of one establishes only
membership, not MTM body-chart provenance. Pinned SizeSetReference uses Count revision; a successor
retains identity and refuses overflow. Currentness/authorized monotonic transitions remain registry/
command obligations. Axes/chart/break/resolution contracts remain separate; D70's axes cardinality
conflict requires a ruling before .4c.2, and no axis representation is selected by this foundation.

## Chart observations — G1-SLICE.4c.3a

An observation pins member/set revision, named tables and two garment measurement bindings. The
Design role is a logical authored input, not a regenerated result: base inputs may serve both roles
without duplicating their scalar. Current metadata/state/source is borrowed; retargeting requires
explicit replacement. Correspondence provenance does not prove physical equivalence; G3/G4 own that
proof. Chart completeness, MTM body/Ease inputs, breaks and resolution remain separate owned slices.
