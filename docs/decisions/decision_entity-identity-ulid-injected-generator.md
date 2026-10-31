# Entity identity is a dependency-free ULID with an injected generator

- **Type:** `decision`
- **Date:** `2026-09-30` (absolute)
- **Status:** `active`
- **Owner / source:** repo-local workflow, leaf `G1-SLICE.3a` — the design input for the ontology's identity
  layer (`docs/book/src/spec/ontology.md` §1, glossary `entity id` → `ULID`).

answers: "how is an entity id generated?" · "is ULID a dependency?" · "how are ids kept deterministic for replay?" · "does sc-core depend on a ULID crate?" · "where may wall-clock and randomness enter the domain?" · "why is the id generator injected?"

## The fact / decision

Every semantic entity carries an `EntityId`: a **ULID** — 128 bits, a 48-bit millisecond timestamp and 80
bits of randomness, Crockford base32, lexicographically sortable — **hand-rolled dependency-free in
`sc-core`** and displayed as its 26-character canonical form. Ids are produced through an **injected
`IdGenerator`**: production injects a clock-plus-entropy generator; tests and deterministic replay (the CLI,
recipe re-evaluation) inject a deterministic one. An id, once assigned, is persisted and **never re-derived
from content** (ontology §1).

## Why

1. **`sc-core` builds for `wasm32-unknown-unknown`** (the G0 CI smoketest, roadmap §7.3). A ULID crate would
   pull `rand`, a time source and `getrandom` into the wasm graph; the value itself — 128 bits plus Crockford
   base32 — is about a hundred dependency-free lines, consistent with `sc-units`' zero-dependency precedent.
2. **Determinism is a gate property, and the canonical format forbids wall-clock content.** Recipe
   re-evaluation must be byte-identical across runs and platforms (ontology §9, roadmap §6.1), CLI replay must
   be byte-identical (`G1-SLICE.10`), and serialization carries no wall-clock values (ontology §7). A real
   ULID embeds a timestamp and randomness, so id *generation* cannot be a free function in domain code: it is
   injected, and a deterministic generator makes replay reproduce the same ids. Injecting the generator
   isolates the clock and the entropy source — the wasm-hostile, determinism-hostile part — behind one trait.
3. **ULID is the specified scheme** (ontology §1; glossary `entity id` → `ULID`), and its lexicographic
   sortability gives creation-order stability without a separate sequence.

## How to apply

- `sc-core` defines `EntityId` as a 128-bit value with ULID layout and Crockford base32 display,
  dependency-free. Domain code never calls a clock or an RNG directly.
- Creation goes through an `IdGenerator` supplied by the caller — the command bus in production (`G1-SLICE.6`,
  the only mutation path), a deterministic generator in tests and replay. The production generator is the sole
  place wall-clock and randomness enter, and its output is an id, not canonical content.
- A persisted id is never regenerated; identity is stable across saves, grades and exports (ontology §1).
- Related: [[decision_property-tests-dependency-free-recorded-seed]] (the same dependency-free discipline),
  [[decision_ontology-invariants-structural-g1-geometric-g2]] (the leaf this enables).

## Exhaustion repair0292

D225 reproduced from_start(u128::MAX) returning MAX then0 because wrapping_add advanced an
infallible counter. The public IdGenerator allocation boundary now returns Result<EntityId, IdError>.
Deterministic allocation keeps an optional next value, issues MAX once, then returns Exhausted
permanently. No identity is fabricated, repeated or silently moved to another sequence.
All actual callers handle the result. Six allocating ledger edits propagate the original IdError
as LedgerError::IdGeneration, including the standard error chain, before any ledger mutation.
Their existing argument/precondition priority is retained. new() still starts at1; explicit seed0
remains a valid separate deterministic sequence. This corrects implementation to the original
nonreuse contract and introduces no clock, entropy dependency or persisted format change.

answers: "how can a creation preview preserve the identity allocator?"

0423 G1-SLICE.6.5.2.b.3 installs an explicit private reserve supplied by the trusted application
configuration, filled from its injected generator. No request names a new identity. Strict original
generator order and complete garment ownership disjointness are checked at bus construction.
Preview staging owns only an iterator position over immutable reserved ids; it cannot call an external
provider or alias another mutable cursor through Clone. Dropping/exhausting it leaves the reserve
unchanged, including its last128-bit identity. AddNotch preview proves this on actual canonical owners.
Empty reserves refuse with original IdError::Exhausted; no silent refill or substitute sequence.
Production host integration and persistent global nonreuse across buses/restarts remain later owners.
Atomic publication must advance model/reserve together; undo must never rewind issued identities.

## Canonical output-role identity0696

answers: "can size evaluation retire a canonical output role?" · "does undo rewind output-role issuance?"

Persistent output roles keep explicit LocalTags under their canonical creating operation,with stable
local machine keys. A canonical catalogue records all issued key/tag associations;current active
membership is a separate subset. Tags or keys cannot be retargeted to another declared role or
reconstructed from coordinates,array rank or the current fragment count.

An authoring edit or recorded-history restoration may change active membership of the same declared
roles. It must preserve the issued definition history instead of replacing it with an older snapshot
that forgets later-issued tags. This applies the existing private-reserve/maximum-issued revision
separation to local roles:undo restores semantics without making a previously issued identity fresh.
Actual undo authority and canonical publication still belong to the command bus.

An evaluated candidate manifest is readonly presence data against that catalogue. A role absent from
one size/body evaluation is not thereby retired;another instance may still contain it. Evaluation
cannot silently mutate canonical membership or mint replacement role identities. Unknown or
ambiguous roles require visible unresolved/reconciliation handling through the appropriate authority.

Machine keys are declarations,not geometric classifiers. Native catalogue/transition/presence
admission will check metadata and authentic owners only. G2/G3 must establish actual geometric role
continuity and source-to-derived correspondence;equal keys,intervals or coordinates are not that
evidence. Canonical operation/current-Design integration remains .5f.2,publication .6 and storage .7.
The native leaves are .tags.continuity.catalog/.transition/.presence;none closes a geometry gate.

## Shared edge-role metadata0703

answers: "can slash/spread reuse offset role issuance and membership contracts?"

EdgeOutputRoleCatalog and its definition/active/transition types provide one shared edge-role
metadata implementation for offset,slash/spread and mirror-copy declarations. Existing OffsetRole names
remain aliases of those same Rust types;there is no second role history or conversion. Native
slash and mirror association check complete catalogue counts/key bytes before matching their actual operation.
Generic diagnostic names replace offset-specific labels;typed variants and field data remain intact.

This sharing introduces no PointRef role namespace,geometric classifier,current operation-kind
certificate or global authority. Actual recipe/Design checks,issued-history completeness,G2/G3
role matching and bus publication retain their existing owners. Empty/inactive catalogues remain
metadata states rather than executable output manifests.

## Distinct point catalogue metadata0724

answers: "can raw output-role metadata be shared without mixing point and edge membership proofs?"

PointOutputRoleCatalog and its admitted/member views are distinct types. Private composition
reuses the existing bounded metadata algorithm through its published API. Raw definition,quota
and refusal records may be shared aliases;they carry no membership proof. Point views expose
only their actual point owner,never the private admitted edge component. No public conversion
makes a point membership usable as an edge membership. Equal tag/key values do not change that.

This admits declared point metadata only,without PointRef generation,source lookup,current
namespace,issued-history completeness or geometry. Point history/mirror binding/source-output
attribution remain separate native steps;actual registry/frame/authority/publication stay owned.
