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
