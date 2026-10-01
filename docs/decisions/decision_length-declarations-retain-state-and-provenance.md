# Length declarations retain authored state and provenance without a numeric unknown fallback

- **Type:** `decision`
- **Date:** `2026-10-01`
- **Status:** `active`
- **Owner / source:** `G1-SLICE.4a.1`; ontology §2.1/§2.2/§5, formula-language §2/§3 and roadmap §2.3.

answers: "where do measurement values and states live?" · "can an unknown measurement become zero?" · "does a declared known state prove scoped evidence?" · "how do measurement inputs avoid a core dependency cycle?"

## The fact / decision

The shared immutable length declaration lives in sc-core and holds identity, source reference and
exactly one authored state. sc-measure consumes this contract and core identity; sc-core does not
acquire a dependency on the higher measurement crate. Recipe evaluation consumes a core input
contract through its host context. Measurement metadata references/borrows the canonical declaration,
without a second numeric/state cache. Ease and size-chart inputs can share the same length contract.

Known holds an authored Length and nonempty distinct evidence references; Assumed holds an authored
Length and a recorded-assumption reference; Unknown holds the observation reference needed to obtain
a value; Preference holds an authored Length and provenance reference; Derived holds a formula
reference without an authored/cached result. Mandatory references are not optional strings. Empty or
duplicate known evidence is structurally refused. A declared known state claims evidence; it does
not independently establish its existence, scope, truth or validity.

The input exposes its authored value only for known/assumed/preference. Unknown and derived return
structured unresolved-source refusals naming declaration and observation/formula, never zero or a
preference substituted for observation. Querying an authored preference is not applying it over an
unknown: these are distinct states, and profile composition remains G4's rule. Explicit signed values
and zero retain their units; measurement-specific domains/physical repeatability require their named
procedure and constraints rather than an invented positivity rule in the general length declaration.

## Boundary and later obligations

- G1-SLICE.4a.2/.3 validates MeasurementTable metadata and current references. State/source truth
  is distinct from structural input and cannot be established by a display label or an id alone.
- G1-SLICE.5/.6 resolves declaration/formula dependencies, rejects missing/kind-mismatched references
  and cycles, and propagates derived input state. A derived reference has no independently editable result.
- G4-PROFILES.6/.7 validates scoped evidence and artifact-specific uncertainty closure/policy; G4
  composition resolves preference precedence. No generic declaration query grants export permission.
- Known/assumed/preference authored values are not immutable evaluation certificates: Design revision
  and current source/registry checks remain necessary before computation, approval and release.

This slice introduces only the length-valued contract required by measurement/ease/chart work. It
does not introduce text-valued formula parameters, new physical measurement vocabulary, standards
mapping data, solver assignment to unknown facts or an artifact policy default.

## Shared machine tokens — G1-SLICE.4a.2a

Measurement tokens and recipe identifiers share a core MachineToken. This avoids a higher-crate
lexical dependency or divergent validators. The grammar's ASCII lower-snake rule is made exact:
a lowercase letter starts the token; letters/digits continue each nonempty segment; one underscore
separates segments. Keywords let/assert/if refuse at token construction. Exact authored bytes remain
unchanged, with no normalization or display-name inference. Built-in reserved names such as eps_geo
are valid references; measurement/recipe namespace binding rejects attempts to redeclare them.
MachineToken is neither a scalar text value nor a localized label. Its private representation needs
validated reconstruction to replace a token; a cloned replacement never mutates earlier metadata.

## Verification

The core contract tests must cover all five states, required provenance, nonempty/distinct known
evidence, explicit signed/zero lengths, structured unknown/derived refusals and immutable replacement.
Independent guard/query mutations must fail real regressions, with restored strict checks and WASM.
The book documents authored-state limits; later consumers retain their owned registry/evidence proofs.
