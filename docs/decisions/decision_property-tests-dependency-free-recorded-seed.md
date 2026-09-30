# Property tests are dependency-free, hand-rolled, and carry a recorded seed

- **Type:** `decision`
- **Date:** `2026-09-30` (absolute)
- **Status:** `active`
- **Owner / source:** repo-local workflow, leaf `G1-SLICE.2` — recording the choice `G0-CONTRACT.18`
  (commit `eb83f01`) made when the first property tests landed in `sc-units`, so that later crates do not
  re-litigate it (the question was left open in `docs/tasks/G1-SLICE.md`'s Open Questions).

answers: "do we use proptest or quickcheck?" · "how are property tests written in sc-units?" · "why is there no test-framework dependency?" · "how is a randomized test made reproducible?" · "what RNG drives the property suite?" · "may a later crate adopt a property-test framework?"

## The fact / decision

Property tests in this repository are **hand-rolled and dependency-free**, driven by a deterministic
xorshift64\* generator with a **recorded seed**, rather than a framework such as `proptest` or
`quickcheck`. `crates/sc-units/tests/property.rs` is the reference: 21 properties, seed
`0x57_49_54_43_48_43_41_44` ("STITCHCAD"), zero dev-dependencies, each test naming the spec clause it
discharges. This is the default for any crate on the dependency-free critical path; a crate off that path
may adopt a framework only by its own recorded decision.

## Why

1. **`sc-units` must stay dependency-free** to serve the `wasm-viewer` runtime profile (roadmap §7.3) and
   byte-deterministic golden files (§2.7). A dev-dependency is still a node in the graph, and a framework's
   shrinker pulls in a tree — a dependency here is a dependency everywhere (the crate's own `Cargo.toml`
   says so).
2. **A recorded seed makes a failure reproducible rather than intermittent** (roadmap §6.3: constrained-random
   exploration in testing, deterministic values in production). A framework's random seed must be captured
   from its output after a failure; here the seed is a constant in the file, so a failing case is replayed by
   re-running, not by transcribing a logged seed.
3. **The same suite must run on every platform the cross-regression matrix covers** (gate G6) without a
   framework's platform assumptions or build requirements.
4. **Each property names the spec clause it discharges** (`§9 conversion round-trip`, `§9 class separation`,
   …). A framework's generated cases obscure which requirement a case is testing; an explicit loop over a
   declared domain keeps the clause visible.

The cost — no automatic shrinking, so a failing random case is reported at the seed's first bad value rather
than a minimized one — is accepted: the domains here (integer micrometres, exact ratios) are small enough
that a hand-written minimal case is added beside the property when one is found.

## How to apply

- For a crate that must stay dependency-free (`sc-units`, and any crate on the `wasm-viewer` critical path),
  hand-roll properties with a recorded seed and a deterministic generator; cite the seed and the spec clause
  per test, as `crates/sc-units/tests/property.rs` does.
- A crate **not** on the dependency-free critical path may adopt `proptest`/`quickcheck` if its shrinker earns
  the dependency, but that is a **recorded decision per crate** (a new record linking this one), never a
  default, and it must not leak into a dependency-free crate's graph.
- The seed lives in the test file as a named constant; a failure quotes it so the case is reproducible. Do not
  seed from a wall-clock or a random source — that reintroduces the intermittency the recorded seed removes.
- Related: [[decision_numerical-contract-fixed-point]] (the contract these properties test).
