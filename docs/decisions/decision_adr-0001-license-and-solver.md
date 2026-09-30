# ADR-0001 — license and solver are one decision: permissive core, `slvs` rejected

- **Type:** `decision` (ADR-0001 in `ROADMAP.md` §5)
- **Date:** `2026-09-30` (absolute); the roadmap locked the default at v0.2, this record settles it before
  any solver code exists, as §5 requires
- **Status:** `active`
- **Owner / source:** leaf `G0-CONTRACT.8`, from roadmap §5 ADR-0001, §4.3 (`sc-sketch`), §6.1 (the two
  solvers) and §7.2; the workspace already declares `license = "MIT OR Apache-2.0"`

answers: "what license is StitchCAD?" · "why is the SolveSpace solver not used?" · "can a contributor send a patch without a CLA?" · "what is BSL-1.1 and why is it not permissive?" · "what would reopen this decision?" · "which solver handles geometric sketch constraints?"

## The decision

**The core is permissively licensed — `MIT OR Apache-2.0`, dual-licensed, as the workspace `Cargo.toml`
already declares — and the SolveSpace solver (`slvs`, GPLv3) is therefore rejected.** Local geometric
constraints (`sc-sketch`, optional in every runtime profile) are solved by a **custom kernel or a numeric
least-squares solver written for this repository**, under the same license. License and solver are decided
together, in one record, because neither is decidable alone: the solver choice *is* the license choice.

Three consequences are normative from today:

1. **No GPL-licensed or GPL-incompatible dependency enters any shipped crate.** A re-runnable census of the
   dependency tree is the enforcement, not vigilance; `G1-SLICE.15` already owns it ("every dependency's
   license recorded, copyleft excluded from the shipped tree per ADR-0001"), and until it lands this record
   is the rule a reviewer applies.
2. **Z3 stays where roadmap §6.1 put it:** behind the non-default `csp-z3` feature, as a CI-only
   differential oracle, never compiled into a release artifact and never shipped. Its MIT license would
   permit shipping it; the decision not to is about the ~250k-LOC C++ dependency and the browser profile,
   not about copyright, and this record does not reopen that.
3. **`sc-sketch` is optional, so the custom kernel is a capability question before it is a licensing one.**
   No profile in roadmap §7.3 requires sketch constraints, and the production CSP is `sc-constraints`
   (custom, pure Rust, deterministic). A weak custom kernel delays an optional feature; it does not
   copyleft the product.

## Context

Roadmap §5 frames it as one decision with two options and a default:

- **(a) Permissive core (MIT/Apache/BSL) → `slvs` rejected**, `sc-sketch` uses a small custom kernel or
  numeric least-squares with diagnostics.
- **(b) GPLv3 core → `slvs` allowed**, with the embedding implications for factories noted.
- **BSL-1.1 is a distinct category** — use-restricted, converting to a permissive licence after a delay —
  and the roadmap is explicit that it is never lumped with MIT/Apache.

`slvs` is a mature, battle-tested 2D/3D geometric constraint solver, and the roadmap's own ADR-0002 makes
"custom wgpu, not DOM canvas" a hypothesis rather than an axiom — so the temptation to treat a proven
solver as obviously worth a licence change is real and has to be answered, not waved off.

## Why (a)

- **Linking `slvs` into the core copylefts the whole workspace.** That is not a stylistic cost: it decides
  who may embed StitchCAD. The primary persona is a patternmaker producing a signoff package for a *named
  factory* (roadmap §1.2), and the product's value includes factories and partners embedding it in their own
  tools. A GPLv3 core imposes source-availability obligations on every one of them, which is a commercial
  barrier the project has no reason to accept for an optional feature.
- **The feature `slvs` would buy is the least load-bearing one.** Sketch constraints are "optional local
  annotations, not the authoring model" (ADR-0003): the construction recipe is primary, and the recipe's
  evaluation is a single deterministic pass over an acyclic graph, not a constraint solve. Trading the
  licence of the whole product for an optional annotation layer is a bad exchange rate.
- **The production solver is custom anyway.** Roadmap §6.1 makes `sc-constraints` — a deterministic
  finite-domain CP engine in pure Rust — the production CSP, precisely so that native and WASM profiles
  share one engine and one determinism argument. `slvs` would be the *second* solver, for the *optional*
  one, and would still not remove the custom work.
- **A permissive core is what makes the community library possible.** Roadmap §8 requires sanitized,
  consenting Factory Profile contributions and a calibration loop with partners; §12 asks for a governance
  model with sewist and programmer review paths. Contributors and partners inherit a settled licence, and
  the roadmap's own requirement is that "an external contributor arriving at G1 must inherit a settled
  license, not a default". This record is that settlement.
- **Dual `MIT OR Apache-2.0` rather than one.** Apache-2.0 carries an explicit patent grant, which matters
  for a geometry kernel; MIT maximizes compatibility for embedders. Rust's ecosystem convention is the dual
  licence, and the workspace already declares it.

## The BSL-1.1 distinction, kept explicit

BSL-1.1 is **not** permissive and is **not** chosen. It grants use with a restriction (typically: no
production use by a named class of competitor) that expires into a permissive licence after a delay. Two
reasons it is named here rather than folded into option (a) as the roadmap's shorthand might suggest:

- a use restriction is a commercial term, and adopting one is a different decision from adopting MIT — it
  would need its own review by the project owner, which no G0 leaf has authority to give;
- a downstream embedder cannot treat BSL code as MIT, so mixing the two in one "permissive" category would
  misdescribe the product to exactly the factories §1.2 names.

If the project ever wants BSL, that is a new ADR superseding this one, with the change-delay and the
additional use grant stated.

## Consequences

- **For contributors:** no CLA is required to send a patch; contributions arrive under the dual licence, and
  `cargo` metadata must keep declaring it. A contributor may not add a GPL dependency, and the CI check
  above is what tells them so before a human has to.
- **For `sc-sketch`:** it needs a kernel — a numeric least-squares or Newton-style solver over the typed
  constraint AST, with diagnostics that map an unsolvable or under-determined system back to the fields, in
  the spirit of roadmap §6.2's explained outcomes. That is real work, it is scheduled by whichever gate
  picks up the optional feature, and no gate has: the feature matrix marks `geometric sketch constraints`
  as `deferred` with the diagnostic `env_sketch_constraints`.
- **For determinism:** a least-squares kernel is iterative and floating-point, so it is *not* covered by the
  numerical contract's exactness argument. If it ever lands, its results must be quantized into the internal
  units with a declared tolerance class, and two runs must agree byte-for-byte — a fixed iteration bound and
  a declared convergence test, never "until it looks converged".
- **For the WASM profile:** rejecting `slvs` removes a C++ dependency from the browser build, which is why
  roadmap §6.1's Z3 reasoning and this one point the same way.
- **For the record:** this decision is made before any solver code exists, as §5 requires, so no code has to
  be retro-fitted to a licence chosen after the fact.

## Re-open condition

Exactly one, and it is the roadmap's: **a strong case for `slvs` emerging from the G1 spike.** "Strong" is
stated now so it cannot be argued later:

1. the spike demonstrates a sketch-constraint capability the custom kernel cannot reach within the effort a
   gate has actually scheduled, on a realistic pattern set;
2. the capability is wanted by a named persona or partner, not merely possible; and
3. the licence consequence is accepted by the project owner in writing — which, for a GPLv3 core, means
   accepting the embedding implications for factories.

Absent all three, this record stands. A partial re-open is possible in one direction only: `sc-sketch` may
be licensed differently from the core *if* it stays a separate, optional crate that nothing in the default
build links — the same shape §6.1 uses for Z3. That would be a new record, not an edit of this one.

Related: [[decision_numerical-contract-fixed-point]] · [[decision_size-set-ownership]] ·
`docs/book/src/spec/feature-matrix.md` (the `env_sketch_constraints` deferral) ·
`docs/book/src/spec/glossary/model-and-numbers.md` (the tolerance classes an iterative kernel must join) ·
`ROADMAP.md` §5 ADR-0001, §6.1, §7.2.
